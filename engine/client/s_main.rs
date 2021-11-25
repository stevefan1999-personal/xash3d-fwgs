#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type movie_state_s;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn SNDDMA_Init() -> qboolean;
    #[no_mangle]
    fn SNDDMA_Shutdown();
    #[no_mangle]
    fn SNDDMA_BeginPainting();
    #[no_mangle]
    fn SNDDMA_Submit();
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_RegisterVariable(var: *mut convar_t);
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_Null_f();
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_AddCommand(cmd_name: *const libc::c_char, function: xcommand_t,
                      cmd_desc: *const libc::c_char);
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
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Host_IsQuakeCompatible() -> qboolean;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn AVI_GetAudioInfo(Avi: *mut movie_state_t, snd_info: *mut wavdata_t)
     -> qboolean;
    #[no_mangle]
    fn AVI_GetAudioChunk(Avi: *mut movie_state_t,
                         audiodata: *mut libc::c_char, offset: libc::c_int,
                         length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn AVI_TimeToSoundPosition(Avi: *mut movie_state_t, time: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn CL_IsInGame() -> qboolean;
    #[no_mangle]
    fn CL_IsInMenu() -> qboolean;
    #[no_mangle]
    fn Con_NXPrintf(info: *mut con_nprint_t, fmt: *const libc::c_char,
                    _: ...);
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn S_StreamBackgroundTrack();
    #[no_mangle]
    fn S_StreamSoundTrack();
    #[no_mangle]
    fn S_SetSampleStart(pChan: *mut channel_t, pSource: *mut wavdata_t,
                        newPosition: libc::c_int);
    #[no_mangle]
    fn SND_InitMouth(entnum: libc::c_int, entchannel: libc::c_int);
    #[no_mangle]
    fn VOX_SetChanVol(ch: *mut channel_t);
    #[no_mangle]
    fn VectorNormalizeLength2(v: *const vec_t, out: *mut vec_t)
     -> libc::c_float;
    #[no_mangle]
    fn S_LoadSound(sfx: *mut sfx_t) -> *mut wavdata_t;
    #[no_mangle]
    fn S_SkipSoundChar(pch: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn VOX_LoadSound(pchan: *mut channel_t, psz: *const libc::c_char);
    #[no_mangle]
    fn S_TestSoundChar(pch: *const libc::c_char, c: libc::c_char) -> qboolean;
    #[no_mangle]
    fn MIX_PaintChannels(endtime: libc::c_int);
    #[no_mangle]
    static mut idsp_room: libc::c_int;
    #[no_mangle]
    fn S_GetSfxByHandle(handle: sound_t) -> *mut sfx_t;
    #[no_mangle]
    fn S_RegisterSound(name: *const libc::c_char) -> sound_t;
    #[no_mangle]
    fn VOX_FreeWord(pchan: *mut channel_s);
    #[no_mangle]
    fn VOX_LoadWord(pchan: *mut channel_s);
    #[no_mangle]
    fn S_PrintBackgroundTrackState();
    #[no_mangle]
    fn S_SoundList_f();
    #[no_mangle]
    fn MIX_FreeAllPaintbuffers();
    #[no_mangle]
    fn SX_Free();
    #[no_mangle]
    fn VOX_Shutdown();
    #[no_mangle]
    fn S_FreeSounds();
    #[no_mangle]
    fn S_FindName(name: *const libc::c_char, pfInCache: *mut libc::c_int)
     -> *mut sfx_t;
    #[no_mangle]
    fn S_FadeMusicVolume(fadePercent: libc::c_float);
    #[no_mangle]
    fn MIX_InitAllPaintbuffers();
    #[no_mangle]
    fn SX_Init();
    #[no_mangle]
    fn S_InitScaletable();
    #[no_mangle]
    fn SND_CloseMouth(ch: *mut channel_t);
    #[no_mangle]
    fn DSP_ClearState();
    #[no_mangle]
    fn S_InitSounds();
    #[no_mangle]
    fn MIX_ClearAllPaintBuffers(SampleCount: libc::c_int,
                                clearFilters: qboolean);
    #[no_mangle]
    fn VOX_Init();
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn SimpleSpline(value: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn S_StartBackgroundTrack(intro: *const libc::c_char,
                              loop_0: *const libc::c_char,
                              position: libc::c_int, fullpath: qboolean);
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    fn CL_GetEntitySpatialization(ch: *mut channel_s) -> qboolean;
    #[no_mangle]
    fn Mod_PointInLeaf(p: *const vec_t, node: *mut mnode_t) -> *mut mleaf_t;
    #[no_mangle]
    fn CL_GetMovieSpatialization(ch: *mut rawchan_s) -> qboolean;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type sound_t = libc::c_int;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string = [libc::c_char; 256];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundlist_t {
    pub name: [libc::c_char; 64],
    pub entnum: libc::c_short,
    pub origin: vec3_t,
    pub volume: libc::c_float,
    pub attenuation: libc::c_float,
    pub looping: qboolean,
    pub channel: byte,
    pub pitch: byte,
    pub wordIndex: byte,
    pub samplePos: libc::c_double,
    pub forcedEnd: libc::c_double,
}
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavdata_t {
    pub rate: word,
    pub width: byte,
    pub channels: byte,
    pub loopStart: libc::c_int,
    pub samples: libc::c_int,
    pub type_0: uint,
    pub flags: uint,
    pub buffer: *mut byte,
    pub size: size_t,
}
pub type movie_state_t = movie_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundfade_t {
    pub initial_percent: libc::c_float,
    pub percent: libc::c_float,
    pub starttime: libc::c_float,
    pub fadeouttime: libc::c_float,
    pub holdtime: libc::c_float,
    pub fadeintime: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub format: snd_format_t,
    pub samples: libc::c_int,
    pub samplepos: libc::c_int,
    pub buffer: *mut byte,
    pub initialized: qboolean,
}
pub type snd_format_t = snd_format_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_format_s {
    pub speed: libc::c_uint,
    pub width: libc::c_uint,
    pub channels: libc::c_uint,
}
pub type rawchan_t = rawchan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawchan_s {
    pub entnum: libc::c_int,
    pub master_vol: libc::c_int,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub dist_mult: libc::c_float,
    pub origin: vec3_t,
    pub s_rawend: uint,
    pub sound_info: wavdata_t,
    pub oldtime: libc::c_float,
    pub max_samples: size_t,
    pub rawsamples: [portable_samplepair_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
}
pub type channel_t = channel_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_s {
    pub name: [libc::c_char; 16],
    pub sfx: *mut sfx_t,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub origin: vec3_t,
    pub dist_mult: libc::c_float,
    pub master_vol: libc::c_int,
    pub isSentence: qboolean,
    pub basePitch: libc::c_int,
    pub pitch: libc::c_float,
    pub use_loop: qboolean,
    pub staticsound: qboolean,
    pub localsound: qboolean,
    pub pMixer: mixer_t,
    pub wordIndex: libc::c_int,
    pub currentWord: *mut mixer_t,
    pub words: [voxword_t; 64],
}
pub type voxword_t = voxword_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct voxword_s {
    pub volume: libc::c_int,
    pub pitch: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub cbtrim: libc::c_int,
    pub fKeepCached: libc::c_int,
    pub samplefrac: libc::c_int,
    pub timecompress: libc::c_int,
    pub sfx: *mut sfx_t,
}
pub type sfx_t = sfx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub cache: *mut wavdata_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub hashNext: *mut sfx_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixer_t {
    pub sample: libc::c_double,
    pub pData: *mut wavdata_t,
    pub forcedEndSample: libc::c_double,
    pub finished: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listener_t {
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub entnum: libc::c_int,
    pub waterlevel: libc::c_int,
    pub frametime: libc::c_float,
    pub active: qboolean,
    pub inmenu: qboolean,
    pub paused: qboolean,
    pub streaming: qboolean,
    pub stream_paused: qboolean,
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[no_mangle]
pub static mut dma: dma_t =
    dma_t{format: snd_format_t{speed: 0, width: 0, channels: 0,},
          samples: 0,
          samplepos: 0,
          buffer: 0 as *const byte as *mut byte,
          initialized: false_0,};
#[no_mangle]
pub static mut sndpool: poolhandle_t = 0;
static mut soundfade: soundfade_t =
    soundfade_t{initial_percent: 0.,
                percent: 0.,
                starttime: 0.,
                fadeouttime: 0.,
                holdtime: 0.,
                fadeintime: 0.,};
#[no_mangle]
pub static mut channels: [channel_t; 320] =
    [channel_t{name: [0; 16],
               sfx: 0 as *const sfx_t as *mut sfx_t,
               leftvol: 0,
               rightvol: 0,
               entnum: 0,
               entchannel: 0,
               origin: [0.; 3],
               dist_mult: 0.,
               master_vol: 0,
               isSentence: false_0,
               basePitch: 0,
               pitch: 0.,
               use_loop: false_0,
               staticsound: false_0,
               localsound: false_0,
               pMixer:
                   mixer_t{sample: 0.,
                           pData: 0 as *const wavdata_t as *mut wavdata_t,
                           forcedEndSample: 0.,
                           finished: false_0,},
               wordIndex: 0,
               currentWord: 0 as *const mixer_t as *mut mixer_t,
               words:
                   [voxword_t{volume: 0,
                              pitch: 0,
                              start: 0,
                              end: 0,
                              cbtrim: 0,
                              fKeepCached: 0,
                              samplefrac: 0,
                              timecompress: 0,
                              sfx: 0 as *const sfx_t as *mut sfx_t,}; 64],};
        320];
#[no_mangle]
pub static mut ambient_sfx: [sound_t; 4] = [0; 4];
#[no_mangle]
pub static mut raw_channels: [*mut rawchan_t; 16] =
    [0 as *const rawchan_t as *mut rawchan_t; 16];
#[no_mangle]
pub static mut snd_ambient: qboolean = false_0;
#[no_mangle]
pub static mut snd_fade_sequence: qboolean = false_0;
#[no_mangle]
pub static mut s_listener: listener_t =
    listener_t{origin: [0.; 3],
               velocity: [0.; 3],
               forward: [0.; 3],
               right: [0.; 3],
               up: [0.; 3],
               entnum: 0,
               waterlevel: 0,
               frametime: 0.,
               active: false_0,
               inmenu: false_0,
               paused: false_0,
               streaming: false_0,
               stream_paused: false_0,};
#[no_mangle]
pub static mut total_channels: libc::c_int = 0;
#[no_mangle]
pub static mut soundtime: libc::c_int = 0;
// sample PAIRS
#[no_mangle]
pub static mut paintedtime: libc::c_int = 0;
// sample PAIRS
static mut s_volume: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"volume\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0.7\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"sound volume\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut s_musicvolume: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"MP3Volume\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1.0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"background music volume\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
static mut s_mixahead: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"_snd_mixahead\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0.12\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how much sound to mix ahead of time\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
static mut s_show: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"s_show\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"show playing sounds\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut s_lerping: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"s_lerping\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"apply interpolation to sound output\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
static mut s_ambient_level: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"ambient_level\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0.3\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"volume of environment noises (water and wind)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
static mut s_ambient_fade: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"ambient_fade\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1000\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"rate of volume fading when client is moving\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
static mut s_combine_sounds: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"s_combine_sounds\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"combine channels with same sounds\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut snd_mute_losefocus: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"snd_mute_losefocus\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"silence the audio when game window loses focus\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut s_test: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"s_test\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"engine developer cvar for quick testing new features\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut s_samplecount: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"s_samplecount\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"sample count (0 for default value)\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut s_warn_late_precache: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"s_warn_late_precache\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"warn about late precached sounds on client-side\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
/*
=============================================================================

		SOUNDS PROCESSING

=============================================================================
*/
/*
=================
S_GetMasterVolume
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_GetMasterVolume() -> libc::c_float {
    let mut scale: libc::c_float = 1.0f32;
    if s_listener.inmenu as u64 == 0 &&
           soundfade.percent != 0 as libc::c_int as libc::c_float {
        scale =
            if soundfade.percent / 100.0f32 >= 0.0f32 {
                if soundfade.percent / 100.0f32 < 1.0f32 {
                    (soundfade.percent) / 100.0f32
                } else { 1.0f32 }
            } else { 0.0f32 };
        scale = 1.0f32 - scale
    }
    return s_volume.value * scale;
}
/*
=================
S_FadeClientVolume
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FadeClientVolume(mut fadePercent: libc::c_float,
                                            mut fadeOutSeconds: libc::c_float,
                                            mut holdTime: libc::c_float,
                                            mut fadeInSeconds:
                                                libc::c_float) {
    soundfade.starttime =
        cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    soundfade.initial_percent = fadePercent;
    soundfade.fadeouttime = fadeOutSeconds;
    soundfade.holdtime = holdTime;
    soundfade.fadeintime = fadeInSeconds;
}
/*
=================
S_IsClient
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_IsClient(mut entnum: libc::c_int) -> qboolean {
    return (entnum == s_listener.entnum) as libc::c_int as qboolean;
}
// free channel so that it may be allocated by the
// next request to play a sound.  If sound is a
// word in a sentence, release the sentence.
// Works for static, dynamic, sentence and stream sounds
/*
=================
S_FreeChannel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FreeChannel(mut ch: *mut channel_t) {
    (*ch).sfx = 0 as *mut sfx_t;
    (*ch).name[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*ch).use_loop = false_0;
    (*ch).isSentence = false_0;
    // clear mixer
    memset(&mut (*ch).pMixer as *mut mixer_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mixer_t>() as libc::c_ulong);
    SND_CloseMouth(ch);
}
/*
=================
S_UpdateSoundFade
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_UpdateSoundFade() {
    let mut f: libc::c_float = 0.;
    let mut totaltime: libc::c_float = 0.;
    let mut elapsed: libc::c_float = 0.;
    // determine current fade value.
	// assume no fading remains
    soundfade.percent = 0 as libc::c_int as libc::c_float;
    totaltime =
        soundfade.fadeouttime + soundfade.fadeintime + soundfade.holdtime;
    elapsed =
        (cl.mtime[0 as libc::c_int as usize] -
             soundfade.starttime as libc::c_double) as libc::c_float;
    // clock wrapped or reset (BUG) or we've gone far enough
    if elapsed < 0.0f32 || elapsed >= totaltime || totaltime <= 0.0f32 {
        return
    }
    // We are in the fade time, so determine amount of fade.
    if soundfade.fadeouttime > 0.0f32 && elapsed < soundfade.fadeouttime {
        // ramp up
        f = elapsed / soundfade.fadeouttime
    } else if elapsed <= soundfade.fadeouttime + soundfade.holdtime {
        // Inside the hold time
        // stay
        f = 1.0f32
    } else {
        // ramp down
        f =
            (elapsed - (soundfade.fadeouttime + soundfade.holdtime)) /
                soundfade.fadeintime;
        f = 1.0f32 - f
        // backward interpolated...
    }
    // spline it.
    f = SimpleSpline(f);
    f =
        if f >= 0.0f32 {
            if f < 1.0f32 { f } else { 1.0f32 }
        } else { 0.0f32 };
    soundfade.percent = soundfade.initial_percent * f;
    if snd_fade_sequence as u64 != 0 { S_FadeMusicVolume(soundfade.percent); }
    if snd_fade_sequence as libc::c_uint != 0 && soundfade.percent == 100.0f32
       {
        S_StopAllSounds(false_0);
        S_StopBackgroundTrack();
        snd_fade_sequence = false_0
    };
}
/*
=================
SND_FStreamIsPlaying

Select a channel from the dynamic channel allocation area.  For the given entity,
override any other sound playing on the same channel (see code comments below for
exceptions).
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SND_FStreamIsPlaying(mut sfx: *mut sfx_t)
 -> qboolean {
    let mut ch_idx: libc::c_int = 0;
    ch_idx = 4 as libc::c_int;
    while ch_idx < 60 as libc::c_int + 4 as libc::c_int {
        if channels[ch_idx as usize].sfx == sfx { return true_0 }
        ch_idx += 1
    }
    return false_0;
}
/*
=================
SND_PickDynamicChannel

Select a channel from the dynamic channel allocation area.  For the given entity,
override any other sound playing on the same channel (see code comments below for
exceptions).
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SND_PickDynamicChannel(mut entnum: libc::c_int,
                                                mut channel: libc::c_int,
                                                mut sfx: *mut sfx_t,
                                                mut ignore: *mut qboolean)
 -> *mut channel_t {
    let mut ch_idx: libc::c_int = 0;
    let mut first_to_die: libc::c_int = 0;
    let mut life_left: libc::c_int = 0;
    let mut timeleft: libc::c_int = 0;
    // check for replacement sound, or find the best one to replace
    first_to_die = -(1 as libc::c_int);
    life_left = 0x7fffffff as libc::c_int;
    if !ignore.is_null() { *ignore = false_0 }
    if channel == 5 as libc::c_int &&
           SND_FStreamIsPlaying(sfx) as libc::c_uint != 0 {
        if !ignore.is_null() { *ignore = true_0 }
        return 0 as *mut channel_t
    }
    ch_idx = 4 as libc::c_int;
    while ch_idx < 60 as libc::c_int + 4 as libc::c_int {
        let mut ch: *mut channel_t =
            &mut *channels.as_mut_ptr().offset(ch_idx as isize) as
                *mut channel_t;
        // Never override a streaming sound that is currently playing or
		// voice over IP data that is playing or any sound on CHAN_VOICE( acting )
        if !(!(*ch).sfx.is_null() && (*ch).entchannel == 5 as libc::c_int) {
            if channel != 0 as libc::c_int && (*ch).entnum == entnum &&
                   ((*ch).entchannel == channel ||
                        channel == -(1 as libc::c_int)) {
                // always override sound from same entity
                first_to_die = ch_idx;
                break ;
            } else if !(!(*ch).sfx.is_null() &&
                            S_IsClient((*ch).entnum) as libc::c_uint != 0 &&
                            S_IsClient(entnum) as u64 == 0) {
                // don't let monster sounds override player sounds
                // try to pick the sound with the least amount of data left to play
                timeleft = 0 as libc::c_int;
                if !(*ch).sfx.is_null() {
                    timeleft = 1 as libc::c_int
                    // ch->end - paintedtime
                }
                if timeleft < life_left {
                    life_left = timeleft;
                    first_to_die = ch_idx
                }
            }
        }
        ch_idx += 1
    }
    if first_to_die == -(1 as libc::c_int) { return 0 as *mut channel_t }
    if !channels[first_to_die as usize].sfx.is_null() {
        // don't restart looping sounds for the same entity
        let mut sc: *mut wavdata_t =
            (*channels[first_to_die as usize].sfx).cache;
        if !sc.is_null() && (*sc).loopStart != -(1 as libc::c_int) {
            let mut ch_0: *mut channel_t =
                &mut *channels.as_mut_ptr().offset(first_to_die as isize) as
                    *mut channel_t;
            if (*ch_0).entnum == entnum && (*ch_0).entchannel == channel &&
                   (*ch_0).sfx == sfx {
                if !ignore.is_null() { *ignore = true_0 }
                // same looping sound, same ent, same channel, don't restart the sound
                return 0 as *mut channel_t
            }
        }
        // be sure and release previous channel if sentence.
        S_FreeChannel(&mut *channels.as_mut_ptr().offset(first_to_die as
                                                             isize));
    }
    return &mut *channels.as_mut_ptr().offset(first_to_die as isize) as
               *mut channel_t;
}
/*
=====================
SND_PickStaticChannel

Pick an empty channel from the static sound area, or allocate a new
channel.  Only fails if we're at max_channels (128!!!) or if
we're trying to allocate a channel for a stream sound that is
already playing.
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn SND_PickStaticChannel(mut pos: *const vec_t,
                                               mut sfx: *mut sfx_t)
 -> *mut channel_t {
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut i: libc::c_int = 0;
    // check for replacement sound, or find the best one to replace
    i = 60 as libc::c_int + 4 as libc::c_int;
    while i < total_channels {
        if channels[i as usize].sfx.is_null() { break ; }
        if *pos.offset(0 as libc::c_int as isize) ==
               channels[i as usize].origin[0 as libc::c_int as usize] &&
               *pos.offset(1 as libc::c_int as isize) ==
                   channels[i as usize].origin[1 as libc::c_int as usize] &&
               *pos.offset(2 as libc::c_int as isize) ==
                   channels[i as usize].origin[2 as libc::c_int as usize] &&
               channels[i as usize].sfx == sfx {
            break ;
        }
        i += 1
    }
    if i < total_channels {
        // reuse an empty static sound channel
        ch = &mut *channels.as_mut_ptr().offset(i as isize) as *mut channel_t
    } else {
        // no empty slots, alloc a new static sound channel
        if total_channels ==
               256 as libc::c_int + (60 as libc::c_int + 4 as libc::c_int) {
            Con_DPrintf(b"^1Error:^7 S_PickStaticChannel: no free channels\n\x00"
                            as *const u8 as *const libc::c_char);
            return 0 as *mut channel_t
        }
        // get a channel for the static sound
        ch =
            &mut *channels.as_mut_ptr().offset(total_channels as isize) as
                *mut channel_t;
        total_channels += 1
    }
    return ch;
}
/*
=================
S_AlterChannel

search through all channels for a channel that matches this
soundsource, entchannel and sfx, and perform alteration on channel
as indicated by 'flags' parameter. If shut down request and
sfx contains a sentence name, shut off the sentence.
returns TRUE if sound was altered,
returns FALSE if sound was not found (sound is not playing)
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_AlterChannel(mut entnum: libc::c_int,
                                        mut channel: libc::c_int,
                                        mut sfx: *mut sfx_t,
                                        mut vol: libc::c_int,
                                        mut pitch: libc::c_int,
                                        mut flags: libc::c_int)
 -> libc::c_int {
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut i: libc::c_int = 0;
    if S_TestSoundChar((*sfx).name.as_mut_ptr(), '!' as i32 as libc::c_char)
           as u64 != 0 {
        // This is a sentence name.
		// For sentences: assume that the entity is only playing one sentence
		// at a time, so we can just shut off
		// any channel that has ch->isSentence >= 0 and matches the entnum.
        i = 4 as libc::c_int;
        ch = channels.as_mut_ptr().offset(4 as libc::c_int as isize);
        while i < total_channels {
            if (*ch).entnum == entnum && (*ch).entchannel == channel &&
                   !(*ch).sfx.is_null() &&
                   (*ch).isSentence as libc::c_uint != 0 {
                if flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                    (*ch).basePitch = pitch
                }
                if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                    (*ch).master_vol = vol
                }
                if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                    S_FreeChannel(ch);
                }
                return true_0 as libc::c_int
            }
            i += 1;
            ch = ch.offset(1)
        }
        // channel not found
        return false_0 as libc::c_int
    }
    // regular sound or streaming sound
    i = 4 as libc::c_int;
    ch = channels.as_mut_ptr().offset(4 as libc::c_int as isize);
    while i < total_channels {
        if (*ch).entnum == entnum && (*ch).entchannel == channel &&
               (*ch).sfx == sfx {
            if flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                (*ch).basePitch = pitch
            }
            if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                (*ch).master_vol = vol
            }
            if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                S_FreeChannel(ch);
            }
            return true_0 as libc::c_int
        }
        i += 1;
        ch = ch.offset(1)
    }
    return false_0 as libc::c_int;
}
/*
=================
S_SpatializeChannel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SpatializeChannel(mut left_vol: *mut libc::c_int,
                                             mut right_vol: *mut libc::c_int,
                                             mut master_vol: libc::c_int,
                                             mut gain: libc::c_float,
                                             mut dot: libc::c_float,
                                             mut dist: libc::c_float) {
    let mut lscale: libc::c_float = 0.;
    let mut rscale: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    rscale = 1.0f32 + dot;
    lscale = 1.0f32 - dot;
    // add in distance effect
    scale = (1.0f32 - dist) * rscale;
    *right_vol = (master_vol as libc::c_float * scale) as libc::c_int;
    scale = (1.0f32 - dist) * lscale;
    *left_vol = (master_vol as libc::c_float * scale) as libc::c_int;
    *right_vol =
        if *right_vol >= 0 as libc::c_int {
            if *right_vol < 255 as libc::c_int {
                *right_vol
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    *left_vol =
        if *left_vol >= 0 as libc::c_int {
            if *left_vol < 255 as libc::c_int {
                *left_vol
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
}
/*
=================
SND_Spatialize
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SND_Spatialize(mut ch: *mut channel_t) {
    let mut source_vec: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut dot: libc::c_float = 0.;
    let mut gain: libc::c_float = 1.0f32;
    let mut looping: qboolean = false_0;
    let mut pSource: *mut wavdata_t = 0 as *mut wavdata_t;
    // anything coming from the view entity will allways be full volume
    if S_IsClient((*ch).entnum) as u64 != 0 {
        (*ch).leftvol = (*ch).master_vol;
        (*ch).rightvol = (*ch).master_vol;
        return
    }
    pSource = (*(*ch).sfx).cache;
    if (*ch).use_loop as libc::c_uint != 0 && !pSource.is_null() &&
           (*pSource).loopStart != -(1 as libc::c_int) {
        looping = true_0
    }
    if (*ch).staticsound as u64 == 0 {
        if CL_GetEntitySpatialization(ch) as u64 == 0 {
            // origin is null and entity not exist on client
            (*ch).rightvol = 0 as libc::c_int;
            (*ch).leftvol = (*ch).rightvol;
            return
        }
    }
    // source_vec is vector from listener to sound source
	// player sounds come from 1' in front of player
    source_vec[0 as libc::c_int as usize] =
        (*ch).origin[0 as libc::c_int as usize] -
            s_listener.origin[0 as libc::c_int as usize];
    source_vec[1 as libc::c_int as usize] =
        (*ch).origin[1 as libc::c_int as usize] -
            s_listener.origin[1 as libc::c_int as usize];
    source_vec[2 as libc::c_int as usize] =
        (*ch).origin[2 as libc::c_int as usize] -
            s_listener.origin[2 as libc::c_int as usize];
    // normalize source_vec and get distance from listener to source
    dist =
        VectorNormalizeLength2(source_vec.as_mut_ptr() as *const vec_t,
                               source_vec.as_mut_ptr());
    dot =
        s_listener.right[0 as libc::c_int as usize] *
            source_vec[0 as libc::c_int as usize] +
            s_listener.right[1 as libc::c_int as usize] *
                source_vec[1 as libc::c_int as usize] +
            s_listener.right[2 as libc::c_int as usize] *
                source_vec[2 as libc::c_int as usize];
    // don't pan sounds with no attenuation
    if (*ch).dist_mult <= 0.0f32 { dot = 0.0f32 }
    // fill out channel volumes for single location
    S_SpatializeChannel(&mut (*ch).leftvol, &mut (*ch).rightvol,
                        (*ch).master_vol, gain, dot, dist * (*ch).dist_mult);
    // if playing a word, set volume
    VOX_SetChanVol(ch);
}
/*
====================
S_StartSound

Start a sound effect for the given entity on the given channel (ie; voice, weapon etc).
Try to grab a channel out of the 8 dynamic spots available.
Currently used for looping sounds, streaming sounds, sentences, and regular entity sounds.
NOTE: volume is 0.0 - 1.0 and attenuation is 0.0 - 1.0 when passed in.
Pitch changes playback pitch of wave by % above or below 100.  Ignored if pitch == 100

NOTE: it's not a good idea to play looping sounds through StartDynamicSound, because
if the looping sound starts out of range, or is bumped from the buffer by another sound
it will never be restarted.  Use StartStaticSound (pass CHAN_STATIC to EMIT_SOUND or
SV_StartSound.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StartSound(mut pos: *const vec_t,
                                      mut ent: libc::c_int,
                                      mut chan: libc::c_int,
                                      mut handle: sound_t,
                                      mut fvol: libc::c_float,
                                      mut attn: libc::c_float,
                                      mut pitch: libc::c_int,
                                      mut flags: libc::c_int) {
    let mut pSource: *mut wavdata_t = 0 as *mut wavdata_t; // Invasion issues
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut target_chan: *mut channel_t = 0 as *mut channel_t;
    let mut check: *mut channel_t = 0 as *mut channel_t;
    let mut vol: libc::c_int = 0;
    let mut ch_idx: libc::c_int = 0;
    let mut bIgnore: qboolean = false_0;
    if dma.initialized as u64 == 0 { return }
    sfx = S_GetSfxByHandle(handle);
    if sfx.is_null() { return }
    vol =
        if fvol * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (fvol * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (fvol) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as libc::c_int;
    if pitch <= 1 as libc::c_int { pitch = 100 as libc::c_int }
    if flags &
           ((1 as libc::c_int) << 5 as libc::c_int |
                (1 as libc::c_int) << 6 as libc::c_int |
                (1 as libc::c_int) << 7 as libc::c_int) != 0 {
        if S_AlterChannel(ent, chan, sfx, vol, pitch, flags) != 0 { return }
        if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 { return }
        // fall through - if we're not trying to stop the sound,
		// and we didn't find it (it's not playing), go ahead and start it up
    }
    if pos.is_null() { pos = refState.vieworg.as_mut_ptr() as *const vec_t }
    if chan == 5 as libc::c_int {
        flags = flags | (1 as libc::c_int) << 10 as libc::c_int
    }
    // pick a channel to play on
    if chan == 6 as libc::c_int {
        target_chan = SND_PickStaticChannel(pos, sfx)
    } else {
        target_chan = SND_PickDynamicChannel(ent, chan, sfx, &mut bIgnore)
    }
    if target_chan.is_null() {
        if bIgnore as u64 == 0 {
            Con_DPrintf(b"^1Error:^7 dropped sound \"sound/%s\"\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*sfx).name.as_mut_ptr());
        }
        return
    }
    // spatialize
    memset(target_chan as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<channel_t>() as libc::c_ulong);
    (*target_chan).origin[0 as libc::c_int as usize] =
        *pos.offset(0 as libc::c_int as isize);
    (*target_chan).origin[1 as libc::c_int as usize] =
        *pos.offset(1 as libc::c_int as isize);
    (*target_chan).origin[2 as libc::c_int as usize] =
        *pos.offset(2 as libc::c_int as isize);
    (*target_chan).staticsound =
        if ent == 0 as libc::c_int {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*target_chan).use_loop =
        if flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    (*target_chan).localsound =
        if flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*target_chan).dist_mult = attn / 1000.0f32;
    (*target_chan).master_vol = vol;
    (*target_chan).entnum = ent;
    (*target_chan).entchannel = chan;
    (*target_chan).basePitch = pitch;
    (*target_chan).isSentence = false_0;
    (*target_chan).sfx = sfx;
    pSource = 0 as *mut wavdata_t;
    if S_TestSoundChar((*sfx).name.as_mut_ptr(), '!' as i32 as libc::c_char)
           as u64 != 0 {
        // this is a sentence
		// link all words and load the first word
		// NOTE: sentence names stored in the cache lookup are
		// prepended with a '!'.  Sentence names stored in the
		// sentence file do not have a leading '!'.
        VOX_LoadSound(target_chan, S_SkipSoundChar((*sfx).name.as_mut_ptr()));
        Q_strncpy((*target_chan).name.as_mut_ptr(), (*sfx).name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 16]>() as
                      libc::c_ulong);
        sfx = (*target_chan).sfx;
        if !sfx.is_null() { pSource = (*sfx).cache }
    } else {
        // regular or streamed sound fx
        pSource = S_LoadSound(sfx);
        (*target_chan).name[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    if pSource.is_null() { S_FreeChannel(target_chan); return }
    SND_Spatialize(target_chan);
    // If a client can't hear a sound when they FIRST receive the StartSound message,
	// the client will never be able to hear that sound. This is so that out of
	// range sounds don't fill the playback buffer. For streaming sounds, we bypass this optimization.
    if (*target_chan).leftvol == 0 && (*target_chan).rightvol == 0 {
        // looping sounds don't use this optimization because they should stick around until they're killed.
        if (*sfx).cache.is_null() ||
               (*(*sfx).cache).loopStart == -(1 as libc::c_int) {
            // if this is a streaming sound, play the whole thing.
            if chan != 5 as libc::c_int {
                S_FreeChannel(target_chan);
                return
                // not audible at all
            }
        }
    }
    // Init client entity mouth movement vars
    SND_InitMouth(ent, chan);
    ch_idx = 4 as libc::c_int;
    check = channels.as_mut_ptr().offset(4 as libc::c_int as isize);
    while ch_idx < 60 as libc::c_int + 4 as libc::c_int {
        if !(check == target_chan) {
            if (*check).sfx == sfx && (*check).pMixer.sample == 0. {
                // skip up to 0.1 seconds of audio
                let mut skip: libc::c_int =
                    COM_RandomLong(0 as libc::c_int,
                                   (0.1f32 *
                                        (*(*(*check).sfx).cache).rate as
                                            libc::c_int as libc::c_float) as
                                       libc::c_long as libc::c_int);
                S_SetSampleStart(check, (*sfx).cache, skip);
                break ;
            }
        }
        ch_idx += 1;
        check = check.offset(1)
    };
}
/*
====================
S_RestoreSound

Restore a sound effect for the given entity on the given channel
====================
*/
#[no_mangle]
pub unsafe extern "C" fn S_RestoreSound(mut pos: *const vec_t,
                                        mut ent: libc::c_int,
                                        mut chan: libc::c_int,
                                        mut handle: sound_t,
                                        mut fvol: libc::c_float,
                                        mut attn: libc::c_float,
                                        mut pitch: libc::c_int,
                                        mut flags: libc::c_int,
                                        mut sample: libc::c_double,
                                        mut end: libc::c_double,
                                        mut wordIndex: libc::c_int) {
    let mut pSource: *mut wavdata_t = 0 as *mut wavdata_t; // Invasion issues
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut target_chan: *mut channel_t = 0 as *mut channel_t;
    let mut bIgnore: qboolean = false_0;
    let mut vol: libc::c_int = 0;
    if dma.initialized as u64 == 0 { return }
    sfx = S_GetSfxByHandle(handle);
    if sfx.is_null() { return }
    vol =
        if fvol * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (fvol * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (fvol) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as libc::c_int;
    if pitch <= 1 as libc::c_int { pitch = 100 as libc::c_int }
    // pick a channel to play on
    if chan == 6 as libc::c_int {
        target_chan = SND_PickStaticChannel(pos, sfx)
    } else {
        target_chan = SND_PickDynamicChannel(ent, chan, sfx, &mut bIgnore)
    }
    if target_chan.is_null() {
        if bIgnore as u64 == 0 {
            Con_DPrintf(b"^1Error:^7 dropped sound \"sound/%s\"\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*sfx).name.as_mut_ptr());
        }
        return
    }
    // spatialize
    memset(target_chan as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<channel_t>() as libc::c_ulong);
    (*target_chan).origin[0 as libc::c_int as usize] =
        *pos.offset(0 as libc::c_int as isize);
    (*target_chan).origin[1 as libc::c_int as usize] =
        *pos.offset(1 as libc::c_int as isize);
    (*target_chan).origin[2 as libc::c_int as usize] =
        *pos.offset(2 as libc::c_int as isize);
    (*target_chan).staticsound =
        if ent == 0 as libc::c_int {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*target_chan).use_loop =
        if flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    (*target_chan).localsound =
        if flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*target_chan).dist_mult = attn / 1000.0f32;
    (*target_chan).master_vol = vol;
    (*target_chan).entnum = ent;
    (*target_chan).entchannel = chan;
    (*target_chan).basePitch = pitch;
    (*target_chan).isSentence = false_0;
    (*target_chan).sfx = sfx;
    pSource = 0 as *mut wavdata_t;
    if S_TestSoundChar((*sfx).name.as_mut_ptr(), '!' as i32 as libc::c_char)
           as u64 != 0 {
        // this is a sentence
		// link all words and load the first word
		// NOTE: sentence names stored in the cache lookup are
		// prepended with a '!'.  Sentence names stored in the
		// sentence file do not have a leading '!'.
        VOX_LoadSound(target_chan, S_SkipSoundChar((*sfx).name.as_mut_ptr()));
        Q_strncpy((*target_chan).name.as_mut_ptr(), (*sfx).name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 16]>() as
                      libc::c_ulong);
        // not a first word in sentence!
        if wordIndex != 0 as libc::c_int {
            VOX_FreeWord(target_chan); // release first loaded word
            (*target_chan).wordIndex = wordIndex; // restore current word
            VOX_LoadWord(target_chan);
            if !(*target_chan).currentWord.is_null() {
                (*target_chan).sfx =
                    (*target_chan).words[(*target_chan).wordIndex as
                                             usize].sfx;
                sfx = (*target_chan).sfx;
                pSource = (*sfx).cache
            }
        } else {
            sfx = (*target_chan).sfx;
            if !sfx.is_null() { pSource = (*sfx).cache }
        }
    } else {
        // regular or streamed sound fx
        pSource = S_LoadSound(sfx);
        (*target_chan).name[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    if pSource.is_null() { S_FreeChannel(target_chan); return }
    SND_Spatialize(target_chan);
    // NOTE: first spatialization may be failed because listener position is invalid at this time
	// so we should keep all sounds an actual and waiting for player spawn.
    // apply the sample offests
    (*target_chan).pMixer.sample = sample;
    (*target_chan).pMixer.forcedEndSample = end;
    // Init client entity mouth movement vars
    SND_InitMouth(ent, chan);
}
/*
=================
S_AmbientSound

Start playback of a sound, loaded into the static portion of the channel array.
Currently, this should be used for looping ambient sounds, looping sounds
that should not be interrupted until complete, non-creature sentences,
and one-shot ambient streaming sounds.  Can also play 'regular' sounds one-shot,
in case designers want to trigger regular game sounds.
Pitch changes playback pitch of wave by % above or below 100.  Ignored if pitch == 100

NOTE: volume is 0.0 - 1.0 and attenuation is 0.0 - 1.0 when passed in.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_AmbientSound(mut pos: *const vec_t,
                                        mut ent: libc::c_int,
                                        mut handle: sound_t,
                                        mut fvol: libc::c_float,
                                        mut attn: libc::c_float,
                                        mut pitch: libc::c_int,
                                        mut flags: libc::c_int) {
    let mut ch: *mut channel_t = 0 as *mut channel_t; // Invasion issues
    let mut pSource: *mut wavdata_t = 0 as *mut wavdata_t;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut vol: libc::c_int = 0;
    let mut fvox: libc::c_int = 0 as libc::c_int;
    if dma.initialized as u64 == 0 { return }
    sfx = S_GetSfxByHandle(handle);
    if sfx.is_null() { return }
    vol =
        if fvol * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (fvol * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (fvol) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as libc::c_int;
    if pitch <= 1 as libc::c_int { pitch = 100 as libc::c_int }
    if flags &
           ((1 as libc::c_int) << 5 as libc::c_int |
                (1 as libc::c_int) << 6 as libc::c_int |
                (1 as libc::c_int) << 7 as libc::c_int) != 0 {
        if S_AlterChannel(ent, 6 as libc::c_int, sfx, vol, pitch, flags) != 0
           {
            return
        }
        if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 { return }
    }
    // pick a channel to play on from the static area
    ch = SND_PickStaticChannel(pos, sfx);
    if ch.is_null() { return }
    (*ch).origin[0 as libc::c_int as usize] =
        *pos.offset(0 as libc::c_int as isize);
    (*ch).origin[1 as libc::c_int as usize] =
        *pos.offset(1 as libc::c_int as isize);
    (*ch).origin[2 as libc::c_int as usize] =
        *pos.offset(2 as libc::c_int as isize);
    (*ch).entnum = ent;
    CL_GetEntitySpatialization(ch);
    if S_TestSoundChar((*sfx).name.as_mut_ptr(), '!' as i32 as libc::c_char)
           as u64 != 0 {
        // this is a sentence. link words to play in sequence.
		// NOTE: sentence names stored in the cache lookup are
		// prepended with a '!'.  Sentence names stored in the
		// sentence file do not have a leading '!'.
        // link all words and load the first word
        VOX_LoadSound(ch, S_SkipSoundChar((*sfx).name.as_mut_ptr()));
        Q_strncpy((*ch).name.as_mut_ptr(), (*sfx).name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 16]>() as
                      libc::c_ulong);
        sfx = (*ch).sfx;
        if !sfx.is_null() { pSource = (*sfx).cache }
        fvox = 1 as libc::c_int
    } else {
        // load regular or stream sound
        pSource = S_LoadSound(sfx);
        (*ch).sfx = sfx;
        (*ch).isSentence = false_0;
        (*ch).name[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    if pSource.is_null() { S_FreeChannel(ch); return }
    // never update positions if source entity is 0
    (*ch).staticsound =
        if ent == 0 as libc::c_int {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*ch).use_loop =
        if flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    (*ch).localsound =
        if flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*ch).master_vol = vol;
    (*ch).dist_mult = attn / 1000.0f32;
    (*ch).entchannel = 6 as libc::c_int;
    (*ch).basePitch = pitch;
    SND_Spatialize(ch);
}
/*
==================
S_StartLocalSound
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StartLocalSound(mut name: *const libc::c_char,
                                           mut volume: libc::c_float,
                                           mut reliable: qboolean) {
    let mut sfxHandle: sound_t = 0;
    let mut flags: libc::c_int =
        (1 as libc::c_int) << 9 as libc::c_int |
            (1 as libc::c_int) << 10 as libc::c_int;
    let mut channel: libc::c_int = 0 as libc::c_int;
    if reliable as u64 != 0 { channel = 6 as libc::c_int }
    if dma.initialized as u64 == 0 { return }
    sfxHandle = S_RegisterSound(name);
    S_StartSound(0 as *const vec_t, s_listener.entnum, channel, sfxHandle,
                 volume, 0 as libc::c_int as libc::c_float,
                 100 as libc::c_int, flags);
}
/*
==================
S_GetCurrentStaticSounds

grab all static sounds playing at current channel
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_GetCurrentStaticSounds(mut pout: *mut soundlist_t,
                                                  mut size: libc::c_int)
 -> libc::c_int {
    let mut sounds_left: libc::c_int = size;
    let mut i: libc::c_int = 0;
    if dma.initialized as u64 == 0 { return 0 as libc::c_int }
    i = 60 as libc::c_int + 4 as libc::c_int;
    while i < total_channels && sounds_left != 0 {
        if channels[i as usize].entchannel == 6 as libc::c_int &&
               !channels[i as usize].sfx.is_null() &&
               (*channels[i as usize].sfx).name[0 as libc::c_int as usize] as
                   libc::c_int != 0 {
            if channels[i as usize].isSentence as libc::c_uint != 0 &&
                   channels[i as usize].name[0 as libc::c_int as usize] as
                       libc::c_int != 0 {
                Q_strncpy((*pout).name.as_mut_ptr(),
                          channels[i as usize].name.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            } else {
                Q_strncpy((*pout).name.as_mut_ptr(),
                          (*channels[i as usize].sfx).name.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            }
            (*pout).entnum = channels[i as usize].entnum as libc::c_short;
            (*pout).origin[0 as libc::c_int as usize] =
                channels[i as usize].origin[0 as libc::c_int as usize];
            (*pout).origin[1 as libc::c_int as usize] =
                channels[i as usize].origin[1 as libc::c_int as usize];
            (*pout).origin[2 as libc::c_int as usize] =
                channels[i as usize].origin[2 as libc::c_int as usize];
            (*pout).volume =
                channels[i as usize].master_vol as libc::c_float / 255.0f32;
            (*pout).attenuation = channels[i as usize].dist_mult * 1000.0f32;
            (*pout).looping =
                (channels[i as usize].use_loop as libc::c_uint != 0 &&
                     (*(*channels[i as usize].sfx).cache).loopStart !=
                         -(1 as libc::c_int)) as libc::c_int as qboolean;
            (*pout).pitch = channels[i as usize].basePitch as byte;
            (*pout).channel = channels[i as usize].entchannel as byte;
            (*pout).wordIndex = channels[i as usize].wordIndex as byte;
            (*pout).samplePos = channels[i as usize].pMixer.sample;
            (*pout).forcedEnd = channels[i as usize].pMixer.forcedEndSample;
            sounds_left -= 1;
            pout = pout.offset(1)
        }
        i += 1
    }
    return size - sounds_left;
}
/*
==================
S_GetCurrentStaticSounds

grab all static sounds playing at current channel
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_GetCurrentDynamicSounds(mut pout: *mut soundlist_t,
                                                   mut size: libc::c_int)
 -> libc::c_int {
    let mut sounds_left: libc::c_int = size; // don't serialize default sounds
    let mut i: libc::c_int =
        0; // never serialize static looped sounds. It will be restoring in game code
    let mut looped: libc::c_int = 0;
    if dma.initialized as u64 == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int + (60 as libc::c_int + 4 as libc::c_int) &&
              sounds_left != 0 {
        if !(channels[i as usize].sfx.is_null() ||
                 (*channels[i as usize].sfx).name[0 as libc::c_int as usize]
                     == 0 ||
                 Q_strnicmp((*channels[i as usize].sfx).name.as_mut_ptr(),
                            b"*default\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                     0) {
            looped =
                (channels[i as usize].use_loop as libc::c_uint != 0 &&
                     (*(*channels[i as usize].sfx).cache).loopStart !=
                         -(1 as libc::c_int)) as libc::c_int;
            if !(channels[i as usize].entchannel == 6 as libc::c_int &&
                     looped != 0 && Host_IsQuakeCompatible() as u64 == 0) {
                if channels[i as usize].isSentence as libc::c_uint != 0 &&
                       channels[i as usize].name[0 as libc::c_int as usize] as
                           libc::c_int != 0 {
                    Q_strncpy((*pout).name.as_mut_ptr(),
                              channels[i as usize].name.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong);
                } else {
                    Q_strncpy((*pout).name.as_mut_ptr(),
                              (*channels[i as usize].sfx).name.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong);
                }
                (*pout).entnum =
                    if channels[i as usize].entnum < 0 as libc::c_int {
                        0 as libc::c_int
                    } else { channels[i as usize].entnum } as libc::c_short;
                (*pout).origin[0 as libc::c_int as usize] =
                    channels[i as usize].origin[0 as libc::c_int as usize];
                (*pout).origin[1 as libc::c_int as usize] =
                    channels[i as usize].origin[1 as libc::c_int as usize];
                (*pout).origin[2 as libc::c_int as usize] =
                    channels[i as usize].origin[2 as libc::c_int as usize];
                (*pout).volume =
                    channels[i as usize].master_vol as libc::c_float /
                        255.0f32;
                (*pout).attenuation =
                    channels[i as usize].dist_mult * 1000.0f32;
                (*pout).pitch = channels[i as usize].basePitch as byte;
                (*pout).channel = channels[i as usize].entchannel as byte;
                (*pout).wordIndex = channels[i as usize].wordIndex as byte;
                (*pout).samplePos = channels[i as usize].pMixer.sample;
                (*pout).forcedEnd =
                    channels[i as usize].pMixer.forcedEndSample;
                (*pout).looping = looped as qboolean;
                sounds_left -= 1;
                pout = pout.offset(1)
            }
        }
        i += 1
    }
    return size - sounds_left;
}
/*
===================
S_InitAmbientChannels
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_InitAmbientChannels() {
    let mut ambient_channel: libc::c_int = 0;
    let mut chan: *mut channel_t = 0 as *mut channel_t;
    ambient_channel = 0 as libc::c_int;
    while ambient_channel < 4 as libc::c_int {
        chan =
            &mut *channels.as_mut_ptr().offset(ambient_channel as isize) as
                *mut channel_t;
        (*chan).staticsound = true_0;
        (*chan).use_loop = true_0;
        (*chan).entchannel = 6 as libc::c_int;
        (*chan).dist_mult = 0 as libc::c_int as libc::c_float / 1000.0f32;
        (*chan).basePitch = 100 as libc::c_int;
        ambient_channel += 1
    };
}
/*
===================
S_UpdateAmbientSounds
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_UpdateAmbientSounds() {
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut vol: libc::c_float = 0.;
    let mut ambient_channel: libc::c_int = 0;
    let mut chan: *mut channel_t = 0 as *mut channel_t;
    if snd_ambient as u64 == 0 { return }
    // calc ambient sound levels
    if cl.worldmodel.is_null() { return }
    leaf =
        Mod_PointInLeaf(s_listener.origin.as_mut_ptr() as *const vec_t,
                        (*cl.worldmodel).nodes);
    if leaf.is_null() || s_ambient_level.value == 0. {
        ambient_channel = 0 as libc::c_int;
        while ambient_channel < 4 as libc::c_int {
            channels[ambient_channel as usize].sfx = 0 as *mut sfx_t;
            ambient_channel += 1
        }
        return
    }
    ambient_channel = 0 as libc::c_int;
    while ambient_channel < 4 as libc::c_int {
        chan =
            &mut *channels.as_mut_ptr().offset(ambient_channel as isize) as
                *mut channel_t;
        (*chan).sfx = S_GetSfxByHandle(ambient_sfx[ambient_channel as usize]);
        // ambient is unused
        if (*chan).sfx.is_null() {
            (*chan).rightvol = 0 as libc::c_int;
            (*chan).leftvol = 0 as libc::c_int
        } else {
            vol =
                s_ambient_level.value *
                    (*leaf).ambient_sound_level[ambient_channel as usize] as
                        libc::c_int as libc::c_float;
            if vol < 0 as libc::c_int as libc::c_float {
                vol = 0 as libc::c_int as libc::c_float
            }
            // don't adjust volume too fast
            if ((*chan).master_vol as libc::c_float) < vol {
                (*chan).master_vol =
                    ((*chan).master_vol as libc::c_float +
                         s_listener.frametime * s_ambient_fade.value) as
                        libc::c_int;
                if (*chan).master_vol as libc::c_float > vol {
                    (*chan).master_vol = vol as libc::c_int
                }
            } else if (*chan).master_vol as libc::c_float > vol {
                (*chan).master_vol =
                    ((*chan).master_vol as libc::c_float -
                         s_listener.frametime * s_ambient_fade.value) as
                        libc::c_int;
                if ((*chan).master_vol as libc::c_float) < vol {
                    (*chan).master_vol = vol as libc::c_int
                }
            }
            (*chan).rightvol = (*chan).master_vol;
            (*chan).leftvol = (*chan).rightvol
        }
        ambient_channel += 1
    };
}
/*
=============================================================================

		SOUND STREAM RAW SAMPLES

=============================================================================
*/
/*
===================
S_FindRawChannel
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FindRawChannel(mut entnum: libc::c_int,
                                          mut create: qboolean)
 -> *mut rawchan_t {
    let mut i: libc::c_int = 0; // world is unused
    let mut free: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut best_time: libc::c_int = 0;
    let mut raw_samples: size_t = 0 as libc::c_int as size_t;
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    if entnum == 0 { return 0 as *mut rawchan_t }
    // check for replacement sound, or find the best one to replace
    best_time = 0x7fffffff as libc::c_int;
    free = -(1 as libc::c_int);
    best = free;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        ch = raw_channels[i as usize];
        if free < 0 as libc::c_int && ch.is_null() {
            free = i
        } else if !ch.is_null() {
            let mut time: libc::c_int = 0;
            // exact match
            if (*ch).entnum == entnum { return ch } // no free slots
            time =
                (*ch).s_rawend.wrapping_sub(paintedtime as libc::c_uint) as
                    libc::c_int;
            if time < best_time { best = i; best_time = time }
        }
        i += 1
    }
    if create as u64 == 0 { return 0 as *mut rawchan_t }
    if free >= 0 as libc::c_int { best = free }
    if best < 0 as libc::c_int { return 0 as *mut rawchan_t }
    if raw_channels[best as usize].is_null() {
        raw_samples = 8192 as libc::c_int as size_t;
        raw_channels[best as usize] =
            _Mem_Alloc(sndpool,
                       (::std::mem::size_of::<rawchan_t>() as
                            libc::c_ulong).wrapping_add((::std::mem::size_of::<portable_samplepair_t>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(raw_samples.wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))),
                       true_0,
                       b"../engine/client/s_main.c\x00" as *const u8 as
                           *const libc::c_char, 1046 as libc::c_int) as
                *mut rawchan_t
    }
    ch = raw_channels[best as usize];
    (*ch).max_samples = raw_samples;
    (*ch).entnum = entnum;
    ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                               0 as libc::c_int as uint);
    return ch;
}
/*
===================
S_RawSamplesStereo
===================
*/
unsafe extern "C" fn S_RawSamplesStereo(mut rawsamples:
                                            *mut portable_samplepair_t,
                                        mut rawend: uint,
                                        mut max_samples: uint,
                                        mut samples: uint, mut rate: uint,
                                        mut width: word, mut channels_0: word,
                                        mut data: *const byte) -> uint {
    let mut fracstep: uint = 0;
    let mut samplefrac: uint = 0;
    let mut src: uint = 0;
    let mut dst: uint = 0;
    if rawend < paintedtime as libc::c_uint { rawend = paintedtime as uint }
    fracstep =
        (rate as libc::c_double / 44100 as libc::c_int as libc::c_double *
             ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_double) as
            uint;
    samplefrac = 0 as libc::c_int as uint;
    if width as libc::c_int == 2 as libc::c_int {
        let mut in_0: *const libc::c_short = data as *const libc::c_short;
        if channels_0 as libc::c_int == 2 as libc::c_int {
            src = 0 as libc::c_int as uint;
            while src < samples {
                let fresh0 = rawend;
                rawend = rawend.wrapping_add(1);
                dst =
                    fresh0 &
                        max_samples.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint);
                (*rawsamples.offset(dst as isize)).left =
                    *in_0.offset(src.wrapping_mul(2 as libc::c_int as
                                                      libc::c_uint).wrapping_add(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                     as isize) as libc::c_int;
                (*rawsamples.offset(dst as isize)).right =
                    *in_0.offset(src.wrapping_mul(2 as libc::c_int as
                                                      libc::c_uint).wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                     as isize) as libc::c_int;
                samplefrac =
                    (samplefrac as libc::c_uint).wrapping_add(fracstep) as
                        uint as uint;
                src = samplefrac >> 14 as libc::c_int
            }
        } else {
            src = 0 as libc::c_int as uint;
            while src < samples {
                let fresh1 = rawend;
                rawend = rawend.wrapping_add(1);
                dst =
                    fresh1 &
                        max_samples.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint);
                (*rawsamples.offset(dst as isize)).left =
                    *in_0.offset(src as isize) as libc::c_int;
                (*rawsamples.offset(dst as isize)).right =
                    *in_0.offset(src as isize) as libc::c_int;
                samplefrac =
                    (samplefrac as libc::c_uint).wrapping_add(fracstep) as
                        uint as uint;
                src = samplefrac >> 14 as libc::c_int
            }
        }
    } else if channels_0 as libc::c_int == 2 as libc::c_int {
        let mut in_1: *const libc::c_char = data as *const libc::c_char;
        src = 0 as libc::c_int as uint;
        while src < samples {
            let fresh2 = rawend;
            rawend = rawend.wrapping_add(1);
            dst =
                fresh2 &
                    max_samples.wrapping_sub(1 as libc::c_int as
                                                 libc::c_uint);
            (*rawsamples.offset(dst as isize)).left =
                (*in_1.offset(src.wrapping_mul(2 as libc::c_int as
                                                   libc::c_uint).wrapping_add(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                  as isize) as libc::c_int) <<
                    8 as libc::c_int;
            (*rawsamples.offset(dst as isize)).right =
                (*in_1.offset(src.wrapping_mul(2 as libc::c_int as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                  as isize) as libc::c_int) <<
                    8 as libc::c_int;
            samplefrac =
                (samplefrac as libc::c_uint).wrapping_add(fracstep) as uint as
                    uint;
            src = samplefrac >> 14 as libc::c_int
        }
    } else {
        src = 0 as libc::c_int as uint;
        while src < samples {
            let fresh3 = rawend;
            rawend = rawend.wrapping_add(1);
            dst =
                fresh3 &
                    max_samples.wrapping_sub(1 as libc::c_int as
                                                 libc::c_uint);
            (*rawsamples.offset(dst as isize)).left =
                (*data.offset(src as isize) as libc::c_int -
                     128 as libc::c_int) << 8 as libc::c_int;
            (*rawsamples.offset(dst as isize)).right =
                (*data.offset(src as isize) as libc::c_int -
                     128 as libc::c_int) << 8 as libc::c_int;
            samplefrac =
                (samplefrac as libc::c_uint).wrapping_add(fracstep) as uint as
                    uint;
            src = samplefrac >> 14 as libc::c_int
        }
    }
    return rawend;
}
/*
===================
S_RawEntSamples
===================
*/
unsafe extern "C" fn S_RawEntSamples(mut entnum: libc::c_int,
                                     mut samples: uint, mut rate: uint,
                                     mut width: word, mut channels_0: word,
                                     mut data: *const byte,
                                     mut snd_vol: libc::c_int) {
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    if snd_vol < 0 as libc::c_int { snd_vol = 0 as libc::c_int }
    ch = S_FindRawChannel(entnum, true_0);
    if ch.is_null() { return }
    (*ch).master_vol = snd_vol;
    (*ch).dist_mult = 0 as libc::c_int as libc::c_float / 1000.0f32;
    ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                               S_RawSamplesStereo((*ch).rawsamples.as_mut_ptr(),
                                                  (*ch).s_rawend,
                                                  (*ch).max_samples as uint,
                                                  samples, rate, width,
                                                  channels_0, data));
    (*ch).rightvol = snd_vol;
    (*ch).leftvol = (*ch).rightvol;
}
/*
===================
S_RawSamples
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_RawSamples(mut samples: uint, mut rate: uint,
                                      mut width: word, mut channels_0: word,
                                      mut data: *const byte,
                                      mut entnum: libc::c_int) {
    let mut snd_vol: libc::c_int =
        128 as libc::c_int; // bg track or movie track
    if entnum < 0 as libc::c_int {
        snd_vol = 256 as libc::c_int
    } // fixup negative values
    if snd_vol < 0 as libc::c_int { snd_vol = 0 as libc::c_int }
    S_RawEntSamples(entnum, samples, rate, width, channels_0, data, snd_vol);
}
/*
===================
S_PositionedRawSamples
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StreamAviSamples(mut Avi: *mut libc::c_void,
                                            mut entnum: libc::c_int,
                                            mut fvol: libc::c_float,
                                            mut attn: libc::c_float,
                                            mut synctime: libc::c_float) {
    let mut bufferSamples: libc::c_int = 0;
    let mut fileSamples: libc::c_int = 0;
    let mut raw: [byte; 8192] = [0; 8192];
    let mut duration: libc::c_float = 0.0f32;
    let mut r: libc::c_int = 0;
    let mut fileBytes: libc::c_int = 0;
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    if dma.initialized as u64 == 0 || s_listener.paused as libc::c_uint != 0
           || CL_IsInGame() as u64 == 0 {
        return
    }
    if entnum < 0 as libc::c_int || entnum >= (*SI.GameInfo).max_edicts {
        return
    }
    ch = S_FindRawChannel(entnum, true_0);
    if ch.is_null() { return }
    if (*ch).sound_info.rate as libc::c_int == 0 as libc::c_int {
        if AVI_GetAudioInfo(Avi as *mut movie_state_t, &mut (*ch).sound_info)
               as u64 == 0 {
            return
        }
        // no audiotrack
    }
    (*ch).master_vol =
        if fvol * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (fvol * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (fvol) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as libc::c_int;
    (*ch).dist_mult = attn / 1000.0f32;
    // see how many samples should be copied into the raw buffer
    if (*ch).s_rawend < soundtime as libc::c_uint {
        ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                                   soundtime as uint)
    }
    // position is changed, synchronization is lost etc
    if __tg_fabs((*ch).oldtime - synctime) > s_mixahead.value {
        (*ch).sound_info.loopStart =
            AVI_TimeToSoundPosition(Avi as *mut movie_state_t,
                                    (synctime *
                                         1000 as libc::c_int as libc::c_float)
                                        as libc::c_int)
    } // keep actual time
    (*ch).oldtime = synctime;
    while ((*ch).s_rawend as libc::c_ulong) <
              (soundtime as libc::c_ulong).wrapping_add((*ch).max_samples) {
        let mut info: *mut wavdata_t = &mut (*ch).sound_info;
        bufferSamples =
            (*ch).max_samples.wrapping_sub((*ch).s_rawend.wrapping_sub(soundtime
                                                                           as
                                                                           libc::c_uint)
                                               as libc::c_ulong) as
                libc::c_int;
        // no more samples for this frame
        // decide how much data needs to be read from the file
        fileSamples =
            (bufferSamples as libc::c_float *
                 ((*info).rate as libc::c_float /
                      44100 as libc::c_int as libc::c_float)) as
                libc::c_int; // no more samples need
        if fileSamples <= 1 as libc::c_int { return }
        // our max buffer size
        fileBytes =
            fileSamples *
                ((*info).width as libc::c_int *
                     (*info).channels as libc::c_int);
        if fileBytes as libc::c_ulong >
               ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong {
            fileBytes =
                ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as
                    libc::c_int;
            fileSamples =
                fileBytes /
                    ((*info).width as libc::c_int *
                         (*info).channels as libc::c_int)
        }
        // read audio stream
        r =
            AVI_GetAudioChunk(Avi as *mut movie_state_t,
                              raw.as_mut_ptr() as *mut libc::c_char,
                              (*info).loopStart,
                              fileBytes); // advance play position
        (*info).loopStart += r;
        if r < fileBytes {
            fileBytes = r;
            fileSamples =
                r /
                    ((*info).width as libc::c_int *
                         (*info).channels as libc::c_int)
        }
        if !(r > 0 as libc::c_int) { break ; }
        ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                                   S_RawSamplesStereo((*ch).rawsamples.as_mut_ptr(),
                                                      (*ch).s_rawend,
                                                      (*ch).max_samples as
                                                          uint,
                                                      fileSamples as uint,
                                                      (*info).rate as uint,
                                                      (*info).width as word,
                                                      (*info).channels as
                                                          word,
                                                      raw.as_mut_ptr()))
    };
}
// add to raw buffer
/*
===================
S_GetRawSamplesLength
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_GetRawSamplesLength(mut entnum: libc::c_int)
 -> uint {
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    ch = S_FindRawChannel(entnum, false_0);
    if ch.is_null() { return 0 as libc::c_int as uint }
    return if (*ch).s_rawend <= paintedtime as libc::c_uint {
               0 as libc::c_int as libc::c_float
           } else {
               ((*ch).s_rawend.wrapping_sub(paintedtime as libc::c_uint) as
                    libc::c_float) *
                   (1000.0f64 / 44100 as libc::c_int as libc::c_double) as
                       libc::c_float
           } as uint;
}
/*
===================
S_ClearRawChannel
===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_ClearRawChannel(mut entnum: libc::c_int) {
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    ch = S_FindRawChannel(entnum, false_0);
    if ch.is_null() { return }
    ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                               0 as libc::c_int as uint);
}
/*
===================
S_FreeIdleRawChannels

Free raw channel that have been idling for too long.
===================
*/
unsafe extern "C" fn S_FreeIdleRawChannels() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut ch: *mut rawchan_t = raw_channels[i as usize];
        if !ch.is_null() {
            if !((*ch).s_rawend >= paintedtime as libc::c_uint) {
                if (paintedtime as
                        libc::c_uint).wrapping_sub((*ch).s_rawend).wrapping_div(44100
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                       >= 10 as libc::c_int as libc::c_uint {
                    raw_channels[i as usize] = 0 as *mut rawchan_t;
                    _Mem_Free(ch as *mut libc::c_void,
                              b"../engine/client/s_main.c\x00" as *const u8 as
                                  *const libc::c_char, 1292 as libc::c_int);
                }
            }
        }
        i += 1
    };
}
/*
===================
S_ClearRawChannels
===================
*/
unsafe extern "C" fn S_ClearRawChannels() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut ch: *mut rawchan_t = raw_channels[i as usize];
        if !ch.is_null() {
            ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                                       0 as libc::c_int as uint);
            (*ch).oldtime = -(1 as libc::c_int) as libc::c_float
        }
        i += 1
    };
}
/*
===================
S_SpatializeRawChannels
===================
*/
unsafe extern "C" fn S_SpatializeRawChannels() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut ch: *mut rawchan_t = raw_channels[i as usize];
        let mut source_vec: vec3_t = [0.; 3];
        let mut dist: libc::c_float = 0.;
        let mut dot: libc::c_float = 0.;
        if !ch.is_null() {
            if (*ch).s_rawend < paintedtime as libc::c_uint {
                (*ch).rightvol = 0 as libc::c_int;
                (*ch).leftvol = (*ch).rightvol
            } else if S_IsClient((*ch).entnum) as u64 == 0 &&
                          (*ch).dist_mult != 0. &&
                          (*ch).entnum >= 0 as libc::c_int &&
                          (*ch).entnum < (*SI.GameInfo).max_edicts {
                if CL_GetMovieSpatialization(ch) as u64 == 0 {
                    // spatialization
                    // origin is null and entity not exist on client
                    (*ch).rightvol = 0 as libc::c_int;
                    (*ch).leftvol = (*ch).rightvol
                } else {
                    source_vec[0 as libc::c_int as usize] =
                        (*ch).origin[0 as libc::c_int as usize] -
                            s_listener.origin[0 as libc::c_int as usize];
                    source_vec[1 as libc::c_int as usize] =
                        (*ch).origin[1 as libc::c_int as usize] -
                            s_listener.origin[1 as libc::c_int as usize];
                    source_vec[2 as libc::c_int as usize] =
                        (*ch).origin[2 as libc::c_int as usize] -
                            s_listener.origin[2 as libc::c_int as usize];
                    // normalize source_vec and get distance from listener to source
                    dist =
                        VectorNormalizeLength2(source_vec.as_mut_ptr() as
                                                   *const vec_t,
                                               source_vec.as_mut_ptr());
                    dot =
                        s_listener.right[0 as libc::c_int as usize] *
                            source_vec[0 as libc::c_int as usize] +
                            s_listener.right[1 as libc::c_int as usize] *
                                source_vec[1 as libc::c_int as usize] +
                            s_listener.right[2 as libc::c_int as usize] *
                                source_vec[2 as libc::c_int as usize];
                    // don't pan sounds with no attenuation
                    if (*ch).dist_mult <= 0.0f32 { dot = 0.0f32 }
                    // fill out channel volumes for single location
                    S_SpatializeChannel(&mut (*ch).leftvol,
                                        &mut (*ch).rightvol, (*ch).master_vol,
                                        1.0f32, dot, dist * (*ch).dist_mult);
                }
            } else {
                (*ch).rightvol = (*ch).master_vol;
                (*ch).leftvol = (*ch).rightvol
            }
        }
        i += 1
    };
}
/*
===================
S_FreeRawChannels
===================
*/
unsafe extern "C" fn S_FreeRawChannels() {
    let mut i: libc::c_int = 0;
    // free raw samples
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if !raw_channels[i as usize].is_null() {
            _Mem_Free(raw_channels[i as usize] as *mut libc::c_void,
                      b"../engine/client/s_main.c\x00" as *const u8 as
                          *const libc::c_char, 1382 as libc::c_int);
        }
        i += 1
    }
    memset(raw_channels.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut rawchan_t; 16]>() as libc::c_ulong);
}
//=============================================================================
/*
==================
S_ClearBuffer
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_ClearBuffer() {
    S_ClearRawChannels();
    SNDDMA_BeginPainting();
    if !dma.buffer.is_null() {
        memset(dma.buffer as *mut libc::c_void, 0 as libc::c_int,
               (dma.samples * 2 as libc::c_int) as libc::c_ulong);
    }
    SNDDMA_Submit();
    MIX_ClearAllPaintBuffers(1024 as libc::c_int, true_0);
}
/*
==================
S_StopSound

stop all sounds for entity on a channel.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StopSound(mut entnum: libc::c_int,
                                     mut channel: libc::c_int,
                                     mut soundname: *const libc::c_char) {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if dma.initialized as u64 == 0 { return }
    sfx = S_FindName(soundname, 0 as *mut libc::c_int);
    S_AlterChannel(entnum, channel, sfx, 0 as libc::c_int, 0 as libc::c_int,
                   (1 as libc::c_int) << 5 as libc::c_int);
}
/*
==================
S_StopAllSounds
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StopAllSounds(mut ambient: qboolean) {
    let mut i: libc::c_int = 0; // no statics
    if dma.initialized as u64 == 0 { return }
    total_channels = 60 as libc::c_int + 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int + (60 as libc::c_int + 4 as libc::c_int) {
        if !channels[i as usize].sfx.is_null() {
            S_FreeChannel(&mut *channels.as_mut_ptr().offset(i as isize));
        }
        i += 1
    }
    DSP_ClearState();
    // clear all the channels
    memset(channels.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[channel_t; 320]>() as libc::c_ulong);
    // restart the ambient sounds
    if ambient as u64 != 0 { S_InitAmbientChannels(); }
    S_ClearBuffer();
    // clear any remaining soundfade
    memset(&mut soundfade as *mut soundfade_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<soundfade_t>() as libc::c_ulong);
}
/*
==============
S_GetSoundtime

update global soundtime

(was part of platform code)
===============
*/
unsafe extern "C" fn S_GetSoundtime() -> libc::c_int {
    static mut buffers: libc::c_int = 0;
    static mut oldsamplepos: libc::c_int = 0;
    let mut samplepos: libc::c_int = 0;
    let mut fullsamples: libc::c_int = 0;
    fullsamples = dma.samples / 2 as libc::c_int;
    // it is possible to miscount buffers
	// if it has wrapped twice between
	// calls to S_Update.  Oh well.
    samplepos = dma.samplepos; // buffer wrapped
    if samplepos < oldsamplepos {
        buffers += 1;
        if paintedtime > 0x40000000 as libc::c_int {
            // time to chop things off to avoid 32 bit limits
            buffers = 0 as libc::c_int;
            paintedtime = fullsamples;
            S_StopAllSounds(true_0);
        }
    }
    oldsamplepos = samplepos;
    return buffers * fullsamples + samplepos / 2 as libc::c_int;
}
//=============================================================================
#[no_mangle]
pub unsafe extern "C" fn S_UpdateChannels() {
    let mut endtime: uint = 0;
    let mut samps: libc::c_int = 0;
    SNDDMA_BeginPainting();
    if dma.buffer.is_null() { return }
    // updates DMA time
    soundtime = S_GetSoundtime();
    // soundtime - total samples that have been played out to hardware at dmaspeed
	// paintedtime - total samples that have been mixed at speed
	// endtime - target for samples in mixahead buffer at speed
    endtime =
        (soundtime as libc::c_float +
             s_mixahead.value * 44100 as libc::c_int as libc::c_float) as
            uint;
    samps = dma.samples >> 1 as libc::c_int;
    if endtime.wrapping_sub(soundtime as libc::c_uint) as libc::c_int > samps
       {
        endtime = (soundtime + samps) as uint
    }
    if endtime.wrapping_sub(paintedtime as libc::c_uint) &
           0x3 as libc::c_int as libc::c_uint != 0 {
        // the difference between endtime and painted time should align on
		// boundaries of 4 samples. this is important when upsampling from 11khz -> 44khz.
        endtime =
            (endtime as
                 libc::c_uint).wrapping_sub(endtime.wrapping_sub(paintedtime
                                                                     as
                                                                     libc::c_uint)
                                                &
                                                0x3 as libc::c_int as
                                                    libc::c_uint) as uint as
                uint
    }
    MIX_PaintChannels(endtime as libc::c_int);
    SNDDMA_Submit();
}
/*
=================
S_ExtraUpdate

Don't let sound skip if going slow
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_ExtraUpdate() {
    if dma.initialized as u64 == 0 { return }
    S_UpdateChannels();
}
/*
============
S_UpdateFrame

update listener position
============
*/
#[no_mangle]
pub unsafe extern "C" fn S_UpdateFrame(mut rvp: *mut ref_viewpass_s) {
    if (*rvp).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 ||
           (*rvp).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        return
    }
    s_listener.origin[0 as libc::c_int as usize] =
        (*rvp).vieworigin[0 as libc::c_int as usize];
    s_listener.origin[1 as libc::c_int as usize] =
        (*rvp).vieworigin[1 as libc::c_int as usize];
    s_listener.origin[2 as libc::c_int as usize] =
        (*rvp).vieworigin[2 as libc::c_int as usize];
    AngleVectors((*rvp).viewangles.as_mut_ptr() as *const vec_t,
                 s_listener.forward.as_mut_ptr(),
                 s_listener.right.as_mut_ptr(), s_listener.up.as_mut_ptr());
    s_listener.entnum = (*rvp).viewentity;
    // can be camera entity too
}
/*
============
SND_UpdateSound

Called once each time through the main loop
============
*/
#[no_mangle]
pub unsafe extern "C" fn SND_UpdateSound() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut combine: *mut channel_t = 0 as *mut channel_t;
    let mut info: con_nprint_t =
        con_nprint_t{index: 0, time_to_live: 0., color: [0.; 3],};
    if dma.initialized as u64 == 0 { return }
    // if the loading plaque is up, clear everything
	// out to make sure we aren't looping a dirty
	// dma buffer while loading
	// update any client side sound fade
    S_UpdateSoundFade();
    // release raw-channels that no longer used more than 10 secs
    S_FreeIdleRawChannels();
    s_listener.velocity[0 as libc::c_int as usize] =
        cl.simvel[0 as libc::c_int as usize];
    s_listener.velocity[1 as libc::c_int as usize] =
        cl.simvel[1 as libc::c_int as usize];
    s_listener.velocity[2 as libc::c_int as usize] =
        cl.simvel[2 as libc::c_int as usize];
    s_listener.frametime = (cl.time - cl.oldtime) as libc::c_float;
    s_listener.waterlevel = cl.local.waterlevel;
    s_listener.active = CL_IsInGame();
    s_listener.inmenu = CL_IsInMenu();
    s_listener.paused = cl.paused;
    // update general area ambient sound sources
    S_UpdateAmbientSounds();
    combine = 0 as *mut channel_t;
    // update spatialization for static and dynamic sounds
    i = 4 as libc::c_int; // respatialize channel
    ch = channels.as_mut_ptr().offset(4 as libc::c_int as isize);
    while i < total_channels {
        if !(*ch).sfx.is_null() {
            SND_Spatialize(ch);
            if !((*ch).leftvol == 0 && (*ch).rightvol == 0) {
                // try to combine static sounds with a previous channel of the same
		// sound effect so we don't mix five torches every frame
		// g-cont: perfomance option, probably kill stereo effect in most cases
                if i >= 60 as libc::c_int + 4 as libc::c_int &&
                       s_combine_sounds.value != 0. {
                    // see if it can just use the last one
                    if !combine.is_null() && (*combine).sfx == (*ch).sfx {
                        (*combine).leftvol += (*ch).leftvol;
                        (*combine).rightvol += (*ch).rightvol;
                        (*ch).rightvol = 0 as libc::c_int;
                        (*ch).leftvol = (*ch).rightvol
                    } else {
                        // search for one
                        combine =
                            channels.as_mut_ptr().offset((60 as libc::c_int +
                                                              4 as
                                                                  libc::c_int)
                                                             as isize);
                        j = 60 as libc::c_int + 4 as libc::c_int;
                        while j < i {
                            if (*combine).sfx == (*ch).sfx { break ; }
                            j += 1;
                            combine = combine.offset(1)
                        }
                        if j == total_channels {
                            combine = 0 as *mut channel_t
                        } else if combine != ch {
                            (*combine).leftvol += (*ch).leftvol;
                            (*combine).rightvol += (*ch).rightvol;
                            (*ch).rightvol = 0 as libc::c_int;
                            (*ch).leftvol = (*ch).rightvol
                        }
                    }
                }
            }
        }
        i += 1;
        ch = ch.offset(1)
    }
    S_SpatializeRawChannels();
    // debugging output
    if s_show.value != 0.0f32 {
        info.color[0 as libc::c_int as usize] = 1.0f32;
        info.color[1 as libc::c_int as usize] = 0.6f32;
        info.color[2 as libc::c_int as usize] = 0.0f32;
        info.time_to_live = 0.5f32;
        i = 0 as libc::c_int;
        total = 1 as libc::c_int;
        ch = channels.as_mut_ptr();
        while i < 256 as libc::c_int + (60 as libc::c_int + 4 as libc::c_int)
              {
            if !(*ch).sfx.is_null() &&
                   ((*ch).leftvol != 0 || (*ch).rightvol != 0) {
                info.index = total;
                Con_NXPrintf(&mut info as *mut con_nprint_t,
                             b"chan %i, pos (%.f %.f %.f) ent %i, lv%3i rv%3i %s\n\x00"
                                 as *const u8 as *const libc::c_char, i,
                             (*ch).origin[0 as libc::c_int as usize] as
                                 libc::c_double,
                             (*ch).origin[1 as libc::c_int as usize] as
                                 libc::c_double,
                             (*ch).origin[2 as libc::c_int as usize] as
                                 libc::c_double, (*ch).entnum, (*ch).leftvol,
                             (*ch).rightvol, (*(*ch).sfx).name.as_mut_ptr());
                total += 1
            }
            i += 1;
            ch = ch.offset(1)
        }
        info.color[0 as libc::c_int as usize] = 1.0f32;
        info.color[1 as libc::c_int as usize] = 1.0f32;
        info.color[2 as libc::c_int as usize] = 1.0f32;
        info.index = 0 as libc::c_int;
        Con_NXPrintf(&mut info as *mut con_nprint_t,
                     b"room_type: %i (%s) ----(%i)---- painted: %i\n\x00" as
                         *const u8 as *const libc::c_char, idsp_room,
                     Cvar_VariableString(b"dsp_coeff_table\x00" as *const u8
                                             as *const libc::c_char),
                     total - 1 as libc::c_int, paintedtime);
    }
    S_StreamBackgroundTrack();
    S_StreamSoundTrack();
    // mix some sound
    S_UpdateChannels();
}
/*
===============================================================================

console functions

===============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Play_f() {
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Usage: play <soundfile>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    S_StartLocalSound(Cmd_Argv(1 as libc::c_int), 1.0f64 as libc::c_float,
                      false_0);
}
#[no_mangle]
pub unsafe extern "C" fn S_Play2_f() {
    let mut i: libc::c_int = 1 as libc::c_int;
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Usage: play2 <soundfile>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    while i < Cmd_Argc() {
        S_StartLocalSound(Cmd_Argv(i), 1.0f64 as libc::c_float, true_0);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_PlayVol_f() {
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Usage: playvol <soundfile volume>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    S_StartLocalSound(Cmd_Argv(1 as libc::c_int),
                      Q_atof(Cmd_Argv(2 as libc::c_int)), false_0);
}
unsafe extern "C" fn S_Say(mut name: *const libc::c_char,
                           mut reliable: qboolean) {
    let mut sentence: [libc::c_char; 1024] = [0; 1024];
    // predefined vox sentence
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 {
        S_StartLocalSound(name, 1.0f32, reliable);
        return
    }
    Q_snprintf(sentence.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
               b"!#%s\x00" as *const u8 as *const libc::c_char, name);
    S_StartLocalSound(sentence.as_mut_ptr(), 1.0f32, reliable);
}
#[no_mangle]
pub unsafe extern "C" fn S_Say_f() {
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Usage: speak <vox sentence>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    S_Say(Cmd_Argv(1 as libc::c_int), false_0);
}
#[no_mangle]
pub unsafe extern "C" fn S_SayReliable_f() {
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Usage: spk <vox sentence>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    S_Say(Cmd_Argv(1 as libc::c_int), true_0);
}
/*
=================
S_Music_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Music_f() {
    let mut c: libc::c_int = Cmd_Argc();
    // run background track
    if c == 1 as libc::c_int {
        // blank name stopped last track
        S_StopBackgroundTrack();
    } else if c == 2 as libc::c_int {
        let mut intro: string = [0; 256];
        let mut main_0: string = [0; 256];
        let mut track: string = [0; 256];
        let mut ext: [*const libc::c_char; 2] =
            [b"mp3\x00" as *const u8 as *const libc::c_char,
             b"wav\x00" as *const u8 as *const libc::c_char];
        let mut i: libc::c_int = 0;
        Q_strncpy(track.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        Q_snprintf(intro.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s_intro\x00" as *const u8 as *const libc::c_char,
                   Cmd_Argv(1 as libc::c_int));
        Q_snprintf(main_0.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s_main\x00" as *const u8 as *const libc::c_char,
                   Cmd_Argv(1 as libc::c_int));
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut intro_path: *const libc::c_char =
                va(b"media/%s.%s\x00" as *const u8 as *const libc::c_char,
                   intro.as_mut_ptr(), ext[i as usize]);
            let mut main_path: *const libc::c_char =
                va(b"media/%s.%s\x00" as *const u8 as *const libc::c_char,
                   main_0.as_mut_ptr(), ext[i as usize]);
            if FS_FileExists(intro_path, false_0 as libc::c_int) != 0 &&
                   FS_FileExists(main_path, false_0 as libc::c_int) != 0 {
                // combined track with introduction and main loop theme
                S_StartBackgroundTrack(intro.as_mut_ptr(),
                                       main_0.as_mut_ptr(), 0 as libc::c_int,
                                       false_0);
                break ;
            } else if FS_FileExists(va(b"media/%s.%s\x00" as *const u8 as
                                           *const libc::c_char,
                                       track.as_mut_ptr(), ext[i as usize]),
                                    false_0 as libc::c_int) != 0 {
                // single non-looped theme
                S_StartBackgroundTrack(track.as_mut_ptr(),
                                       0 as *const libc::c_char,
                                       0 as libc::c_int, false_0);
                break ;
            } else { i += 1 }
        }
    } else if c == 3 as libc::c_int {
        S_StartBackgroundTrack(Cmd_Argv(1 as libc::c_int),
                               Cmd_Argv(2 as libc::c_int), 0 as libc::c_int,
                               false_0);
    } else if c == 4 as libc::c_int &&
                  Q_atoi(Cmd_Argv(3 as libc::c_int)) != 0 as libc::c_int {
        // restore command for singleplayer: all arguments are valid
        S_StartBackgroundTrack(Cmd_Argv(1 as libc::c_int),
                               Cmd_Argv(2 as libc::c_int),
                               Q_atoi(Cmd_Argv(3 as libc::c_int)), false_0);
    } else {
        Con_Printf(b"Usage: music <musicfile> [loopfile]\n\x00" as *const u8
                       as *const libc::c_char);
    };
}
/*
=================
S_StopSound_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StopSound_f() { S_StopAllSounds(true_0); }
/*
=================
S_SoundFade_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SoundFade_f() {
    let mut c: libc::c_int = Cmd_Argc();
    let mut fadeTime: libc::c_float = 5.0f32;
    if c == 2 as libc::c_int {
        fadeTime =
            if atof(Cmd_Argv(1 as libc::c_int)) >= 1.0f32 as libc::c_double {
                if atof(Cmd_Argv(1 as libc::c_int)) <
                       60.0f32 as libc::c_double {
                    atof(Cmd_Argv(1 as libc::c_int))
                } else { 60.0f32 as libc::c_double }
            } else { 1.0f32 as libc::c_double } as libc::c_float
    }
    S_FadeClientVolume(100.0f32, fadeTime, 1.0f32, 0.0f32);
    snd_fade_sequence = true_0;
}
/*
=================
S_SoundInfo_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SoundInfo_f() {
    Con_Printf(b"Audio: DirectSound\n\x00" as *const u8 as
                   *const libc::c_char);
    Con_Printf(b"%5d channel(s)\n\x00" as *const u8 as *const libc::c_char,
               2 as libc::c_int);
    Con_Printf(b"%5d samples\n\x00" as *const u8 as *const libc::c_char,
               dma.samples);
    Con_Printf(b"%5d bits/sample\n\x00" as *const u8 as *const libc::c_char,
               16 as libc::c_int);
    Con_Printf(b"%5d bytes/sec\n\x00" as *const u8 as *const libc::c_char,
               44100 as libc::c_int);
    Con_Printf(b"%5d total_channels\n\x00" as *const u8 as
                   *const libc::c_char, total_channels);
    S_PrintBackgroundTrackState();
}
/*
================
S_Init
================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Init() -> qboolean {
    if Sys_CheckParm(b"-nosound\x00" as *const u8 as *const libc::c_char) != 0
       {
        Con_Printf(b"Audio: Disabled\n\x00" as *const u8 as
                       *const libc::c_char); // nehahra stuff
        return false_0
    }
    Cvar_RegisterVariable(&mut s_volume);
    Cvar_RegisterVariable(&mut s_musicvolume);
    Cvar_RegisterVariable(&mut s_mixahead);
    Cvar_RegisterVariable(&mut s_show);
    Cvar_RegisterVariable(&mut s_lerping);
    Cvar_RegisterVariable(&mut s_ambient_level);
    Cvar_RegisterVariable(&mut s_ambient_fade);
    Cvar_RegisterVariable(&mut s_combine_sounds);
    Cvar_RegisterVariable(&mut snd_mute_losefocus);
    Cvar_RegisterVariable(&mut s_test);
    Cvar_RegisterVariable(&mut s_samplecount);
    Cvar_RegisterVariable(&mut s_warn_late_precache);
    Cmd_AddCommand(b"play\x00" as *const u8 as *const libc::c_char,
                   Some(S_Play_f as unsafe extern "C" fn() -> ()),
                   b"playing a specified sound file\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"play2\x00" as *const u8 as *const libc::c_char,
                   Some(S_Play2_f as unsafe extern "C" fn() -> ()),
                   b"playing a group of specified sound files\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"playvol\x00" as *const u8 as *const libc::c_char,
                   Some(S_PlayVol_f as unsafe extern "C" fn() -> ()),
                   b"playing a specified sound file with specified volume\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"stopsound\x00" as *const u8 as *const libc::c_char,
                   Some(S_StopSound_f as unsafe extern "C" fn() -> ()),
                   b"stop all sounds\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"music\x00" as *const u8 as *const libc::c_char,
                   Some(S_Music_f as unsafe extern "C" fn() -> ()),
                   b"starting a background track\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"soundlist\x00" as *const u8 as *const libc::c_char,
                   Some(S_SoundList_f as unsafe extern "C" fn() -> ()),
                   b"display loaded sounds\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"s_info\x00" as *const u8 as *const libc::c_char,
                   Some(S_SoundInfo_f as unsafe extern "C" fn() -> ()),
                   b"print sound system information\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"s_fade\x00" as *const u8 as *const libc::c_char,
                   Some(S_SoundFade_f as unsafe extern "C" fn() -> ()),
                   b"fade all sounds then stop all\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"+voicerecord\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Null_f as unsafe extern "C" fn() -> ()),
                   b"start voice recording (non-implemented)\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"-voicerecord\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Null_f as unsafe extern "C" fn() -> ()),
                   b"stop voice recording (non-implemented)\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"spk\x00" as *const u8 as *const libc::c_char,
                   Some(S_SayReliable_f as unsafe extern "C" fn() -> ()),
                   b"reliable play a specified sententce\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"speak\x00" as *const u8 as *const libc::c_char,
                   Some(S_Say_f as unsafe extern "C" fn() -> ()),
                   b"playing a specified sententce\x00" as *const u8 as
                       *const libc::c_char);
    if SNDDMA_Init() as u64 == 0 {
        Con_Printf(b"Audio: sound system can\'t be initialized\n\x00" as
                       *const u8 as *const libc::c_char);
        return false_0
    }
    sndpool =
        _Mem_AllocPool(b"Sound Zone\x00" as *const u8 as *const libc::c_char,
                       b"../engine/client/s_main.c\x00" as *const u8 as
                           *const libc::c_char, 1905 as libc::c_int);
    soundtime = 0 as libc::c_int;
    paintedtime = 0 as libc::c_int;
    // clear ambient sounds
    memset(ambient_sfx.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[sound_t; 4]>() as libc::c_ulong);
    MIX_InitAllPaintbuffers();
    SX_Init();
    S_InitScaletable();
    S_StopAllSounds(true_0);
    S_InitSounds();
    VOX_Init();
    return true_0;
}
// =======================================================================
// Shutdown sound engine
// =======================================================================
#[no_mangle]
pub unsafe extern "C" fn S_Shutdown() {
    if dma.initialized as u64 == 0 { return }
    Cmd_RemoveCommand(b"play\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"playvol\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"stopsound\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"music\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"soundlist\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"s_info\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"s_fade\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+voicerecord\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"-voicerecord\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"speak\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"spk\x00" as *const u8 as *const libc::c_char);
    S_StopAllSounds(false_0);
    S_FreeRawChannels();
    S_FreeSounds();
    VOX_Shutdown();
    SX_Free();
    SNDDMA_Shutdown();
    MIX_FreeAllPaintbuffers();
    _Mem_FreePool(&mut sndpool,
                  b"../engine/client/s_main.c\x00" as *const u8 as
                      *const libc::c_char, 1949 as libc::c_int);
}
