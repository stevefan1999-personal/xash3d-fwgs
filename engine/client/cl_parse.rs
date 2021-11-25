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
    pub type mip_s;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn S_StopAllSounds(ambient: qboolean);
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn UI_SetActiveMenu(fActive: qboolean);
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Con_Print(txt: *const libc::c_char);
    #[no_mangle]
    fn SCR_BeginLoadingPlaque(is_background: qboolean);
    #[no_mangle]
    fn CL_Drop();
    #[no_mangle]
    fn CL_Disconnect();
    #[no_mangle]
    fn CL_ProcessFile(successfully_received: qboolean,
                      filename: *const libc::c_char);
    #[no_mangle]
    fn CL_IsPlaybackDemo() -> qboolean;
    #[no_mangle]
    fn CL_ServerCommand(reliable: qboolean, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn HPAK_GetDataPointer(filename: *const libc::c_char,
                           pRes: *mut resource_s, buffer: *mut *mut byte,
                           size: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn CSCR_LoadDefaultCVars(scriptfilename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn COM_SizeofResourceList(pList: *mut resource_t, ri: *mut resourceinfo_t)
     -> libc::c_int;
    #[no_mangle]
    fn COM_CreateCustomization(pHead: *mut customization_t,
                               pRes: *mut resource_t, playernum: libc::c_int,
                               flags: libc::c_int,
                               pCust: *mut *mut customization_t,
                               nLumps: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn COM_ClearCustomizationList(pHead: *mut customization_t,
                                  bCleanDecals: qboolean);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
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
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_IsSafeFileToDownload(filename: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_Reset(var_name: *const libc::c_char);
    #[no_mangle]
    fn CRC32_Init(pulCRC: *mut dword);
    #[no_mangle]
    fn CRC32_Final(pulCRC: dword) -> dword;
    #[no_mangle]
    fn MD5_Print(hash: *mut byte) -> *mut libc::c_char;
    #[no_mangle]
    static mut cl_allow_levelshots: *mut convar_t;
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    fn NET_StringToAdr(string: *const libc::c_char, adr: *mut netadr_t)
     -> qboolean;
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn Host_Credits();
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_AbortCurrentFrame();
    #[no_mangle]
    fn Host_IsQuakeCompatible() -> qboolean;
    #[no_mangle]
    fn HTTP_AddCustomServer(url: *const libc::c_char);
    #[no_mangle]
    fn HTTP_AddDownload(path: *const libc::c_char, size: libc::c_int,
                        process: qboolean);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_AddFilteredText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_Execute();
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
    fn CRC32_File(crcvalue: *mut dword, filename: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MD5_HashFile(digest: *mut byte, pszFileName: *const libc::c_char,
                    seed: *mut uint) -> qboolean;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FreeImage(pack: *mut rgbdata_t);
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut world: world_static_t;
    #[no_mangle]
    fn Mod_FreeModel(mod_0: *mut model_t);
    #[no_mangle]
    fn Mod_LoadWorld(name: *const libc::c_char, preload: qboolean)
     -> *mut model_t;
    #[no_mangle]
    fn Mod_ForName(name: *const libc::c_char, crash: qboolean,
                   trackCRC: qboolean) -> *mut model_t;
    #[no_mangle]
    fn Mod_ValidateCRC(name: *const libc::c_char, crc: CRC32_t) -> qboolean;
    #[no_mangle]
    fn Mod_NeedCRC(name: *const libc::c_char, needCRC: qboolean);
    #[no_mangle]
    fn Mod_FreeUnused();
    #[no_mangle]
    fn Mod_GetStudioBounds(name: *const libc::c_char, mins: *mut vec_t,
                           maxs: *mut vec_t) -> qboolean;
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_WriteOneBit(sb: *mut sizebuf_t, nValue: libc::c_int);
    #[no_mangle]
    fn MSG_WriteUBitLong(sb: *mut sizebuf_t, curData: uint,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteShort(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteLong(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteBytes(sb: *mut sizebuf_t, pBuf: *const libc::c_void,
                      nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_ReadOneBit(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadBitAngle(sb: *mut sizebuf_t, numbits: libc::c_int)
     -> libc::c_float;
    #[no_mangle]
    fn MSG_ReadSBitLong(sb: *mut sizebuf_t, numbits: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadUBitLong(sb: *mut sizebuf_t, numbits: libc::c_int) -> uint;
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
    fn MSG_ReadBytes(sb: *mut sizebuf_t, pOut: *mut libc::c_void,
                     nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    static mut r_decals: *mut convar_t;
    #[no_mangle]
    fn R_UpdateRefState();
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    static mut gameui: gameui_static_t;
    #[no_mangle]
    static mut mp_decals: convar_t;
    #[no_mangle]
    static mut cl_allow_download: convar_t;
    #[no_mangle]
    static mut cl_download_ingame: convar_t;
    #[no_mangle]
    static mut cl_levelshot_name: *mut convar_t;
    #[no_mangle]
    static mut cl_clockreset: *mut convar_t;
    #[no_mangle]
    static mut cl_trace_messages: *mut convar_t;
    #[no_mangle]
    fn CL_SetLightstyle(style: libc::c_int, s: *const libc::c_char,
                        f: libc::c_float);
    #[no_mangle]
    fn CL_CheckFile(msg: *mut sizebuf_t, pResource: *mut resource_t)
     -> qboolean;
    #[no_mangle]
    fn CL_AddToResourceList(pResource: *mut resource_t,
                            pList: *mut resource_t);
    #[no_mangle]
    fn CL_RemoveFromResourceList(pResource: *mut resource_t);
    #[no_mangle]
    fn CL_MoveToOnHandList(pResource: *mut resource_t);
    #[no_mangle]
    fn CL_Parse_Debug(enable: qboolean);
    #[no_mangle]
    fn CL_Parse_RecordCommand(cmd: libc::c_int, startoffset: libc::c_int);
    #[no_mangle]
    fn CL_ResetFrame(frame: *mut frame_t);
    #[no_mangle]
    fn CL_PrecacheResources() -> qboolean;
    #[no_mangle]
    fn CL_SetupOverviewParams();
    #[no_mangle]
    fn CL_SignonReply();
    #[no_mangle]
    fn CL_ClearState();
    #[no_mangle]
    fn CL_WriteDemoMessage(startup: qboolean, start: libc::c_int,
                           msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_ParseEvent(msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_ParseReliableEvent(msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_SetEventIndex(szEvName: *const libc::c_char, ev_index: libc::c_int);
    #[no_mangle]
    fn Con_ClearNotify();
    #[no_mangle]
    fn CL_ClearWorld();
    #[no_mangle]
    fn CL_LoadClientSprites();
    #[no_mangle]
    fn CL_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn CL_AddClientResources();
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn CL_LinkUserMessage(pszName: *mut libc::c_char, svc_num: libc::c_int,
                          iSize: libc::c_int);
    #[no_mangle]
    fn CL_ParseFinaleCutscene(msg: *mut sizebuf_t, level: libc::c_int);
    #[no_mangle]
    fn CL_InitEdicts();
    #[no_mangle]
    fn CL_CenterPrint(text: *const libc::c_char, y: libc::c_float);
    #[no_mangle]
    fn SCR_EndLoadingPlaque();
    #[no_mangle]
    fn S_FadeClientVolume(fadePercent: libc::c_float,
                          fadeOutSeconds: libc::c_float,
                          holdTime: libc::c_float,
                          fadeInSeconds: libc::c_float);
    #[no_mangle]
    fn CL_ParsePacketEntities(msg: *mut sizebuf_t, delta: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn CL_FireCustomDecal(textureIndex: libc::c_int, entityIndex: libc::c_int,
                          modelIndex: libc::c_int, pos: *mut libc::c_float,
                          flags: libc::c_int, scale: libc::c_float);
    #[no_mangle]
    fn S_StartBackgroundTrack(intro: *const libc::c_char,
                              loop_0: *const libc::c_char,
                              position: libc::c_int, fullpath: qboolean);
    #[no_mangle]
    fn CL_ParseTempEntity(msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_IsPlayerIndex(idx: libc::c_int) -> qboolean;
    #[no_mangle]
    fn R_AddEfrags(ent: *mut cl_entity_t);
    #[no_mangle]
    fn CL_ResetLatchedVars(ent: *mut cl_entity_t, full_reset: qboolean);
    #[no_mangle]
    fn S_RestoreSound(pos: *const vec_t, ent: libc::c_int, chan: libc::c_int,
                      handle: sound_t, fvol: libc::c_float,
                      attn: libc::c_float, pitch: libc::c_int,
                      flags: libc::c_int, sample: libc::c_double,
                      end: libc::c_double, wordIndex: libc::c_int);
    #[no_mangle]
    fn S_RegisterSound(sample: *const libc::c_char) -> sound_t;
    #[no_mangle]
    fn Key_SetKeyDest(key_dest: libc::c_int);
    #[no_mangle]
    fn Con_FixedFont() -> qboolean;
    #[no_mangle]
    fn S_StartSound(pos: *const vec_t, ent: libc::c_int, chan: libc::c_int,
                    sfx: sound_t, vol: libc::c_float, attn: libc::c_float,
                    pitch: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn S_AmbientSound(pos: *const vec_t, ent: libc::c_int, handle: sound_t,
                      fvol: libc::c_float, attn: libc::c_float,
                      pitch: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn Netchan_CreateFragments(chan: *mut netchan_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn Netchan_FragSend(chan: *mut netchan_t);
    #[no_mangle]
    fn Delta_InitClient();
    #[no_mangle]
    fn Delta_ParseTableField(msg: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_ReadDeltaMovevars(msg: *mut sizebuf_t, from: *mut movevars_s,
                             to: *mut movevars_s);
    #[no_mangle]
    fn MSG_ReadClientData(msg: *mut sizebuf_t, from: *mut clientdata_s,
                          to: *mut clientdata_s, timebase: libc::c_double);
    #[no_mangle]
    fn MSG_ReadWeaponData(msg: *mut sizebuf_t, from: *mut weapon_data_s,
                          to: *mut weapon_data_s, timebase: libc::c_double);
    #[no_mangle]
    fn MSG_ReadDeltaEntity(msg: *mut sizebuf_t, from: *mut entity_state_s,
                           to: *mut entity_state_s, num: libc::c_int,
                           type_0: libc::c_int, timebase: libc::c_double)
     -> qboolean;
    #[no_mangle]
    fn R_AllocParticle(callback:
                           Option<unsafe extern "C" fn(_: *mut particle_s,
                                                       _: libc::c_float)
                                      -> ()>) -> *mut particle_s;
    #[no_mangle]
    fn R_RunParticleEffect(org: *const vec_t, dir: *const vec_t,
                           color: libc::c_int, count: libc::c_int);
    #[no_mangle]
    fn CL_DecalIndex(id: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn IN_MouseRestorePos();
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
pub type C2RustUnnamed = libc::c_uint;
pub const DEV_EXTENDED: C2RustUnnamed = 2;
pub const DEV_NORMAL: C2RustUnnamed = 1;
pub const DEV_NONE: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed_0 = 5;
pub const kRenderTransAlpha: C2RustUnnamed_0 = 4;
pub const kRenderGlow: C2RustUnnamed_0 = 3;
pub const kRenderTransTexture: C2RustUnnamed_0 = 2;
pub const kRenderTransColor: C2RustUnnamed_0 = 1;
pub const kRenderNormal: C2RustUnnamed_0 = 0;
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
pub struct _resourceinfo_t {
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resourceinfo_s {
    pub info: [_resourceinfo_t; 8],
}
pub type resourceinfo_t = resourceinfo_s;
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
pub type event_args_t = event_args_s;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const force_model_specifybounds: C2RustUnnamed_1 = 2;
pub const force_model_samebounds: C2RustUnnamed_1 = 1;
pub const force_exactfile: C2RustUnnamed_1 = 0;
pub type CRC32_t = libc::c_uint;
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
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DEMO_QUAKE1: C2RustUnnamed_2 = 2;
pub const DEMO_XASH3D: C2RustUnnamed_2 = 1;
pub const DEMO_INACTIVE: C2RustUnnamed_2 = 0;
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
pub struct world_static_s {
    pub loading: qboolean,
    pub flags: libc::c_int,
    pub message: [libc::c_char; 2048],
    pub compiler: [libc::c_char; 256],
    pub generator: [libc::c_char; 256],
    pub hull_models: *mut hull_model_t,
    pub num_hull_models: libc::c_int,
    pub deluxedata: *mut color24,
    pub shadowdata: *mut byte,
    pub visbytes: size_t,
    pub fatbytes: size_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub size: vec3_t,
    pub recursion_level: libc::c_int,
    pub max_recursion: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hull_model_t {
    pub polys: hullnode_t,
    pub num_polys: uint,
}
pub type hullnode_t = hullnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hullnode_s {
    pub next: *mut hullnode_s,
    pub prev: *mut hullnode_s,
}
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
pub type world_static_t = world_static_s;
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
pub const DELTA_STATIC: C2RustUnnamed_4 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub rescount: libc::c_int,
    pub restype: [libc::c_int; 2048],
    pub resnames: [[libc::c_char; 64]; 2048],
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const DELTA_PLAYER: C2RustUnnamed_4 = 1;
pub const DELTA_ENTITY: C2RustUnnamed_4 = 0;
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_double) -> libc::c_double {
    return fabs(__x);
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
/*
cl_parse.c - parse a message received from the server
Copyright (C) 2008 Uncle Mike

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
pub static mut CL_UPDATE_BACKUP: libc::c_int = 16 as libc::c_int;
/*
===============
CL_UserMsgStub

Default stub for missed callbacks
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UserMsgStub(mut pszName: *const libc::c_char,
                                        mut iSize: libc::c_int,
                                        mut pbuf: *mut libc::c_void)
 -> libc::c_int {
    return 1 as libc::c_int;
}
/*
==================
CL_ParseViewEntity

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseViewEntity(mut msg: *mut sizebuf_t) {
    cl.viewentity = MSG_ReadWord(msg);
    // check entity bounds in case we want
	// to use this directly in clgame.entities[] array
    cl.viewentity =
        if cl.viewentity >= 0 as libc::c_int {
            if cl.viewentity < clgame.maxEntities - 1 as libc::c_int {
                cl.viewentity
            } else { (clgame.maxEntities) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
}
/*
==================
CL_ParseSoundPacket

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseSoundPacket(mut msg: *mut sizebuf_t) {
    let mut pos: vec3_t = [0.; 3];
    let mut chan: libc::c_int = 0;
    let mut sound: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut attn: libc::c_float = 0.;
    let mut flags: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut entnum: libc::c_int = 0;
    let mut handle: sound_t = 0 as libc::c_int;
    flags = MSG_ReadUBitLong(msg, 14 as libc::c_int) as libc::c_int;
    sound = MSG_ReadUBitLong(msg, 11 as libc::c_int) as libc::c_int;
    chan = MSG_ReadUBitLong(msg, 4 as libc::c_int) as libc::c_int;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        volume = MSG_ReadByte(msg) as libc::c_float / 255.0f32
    } else { volume = 1.0f64 as libc::c_float }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        attn = MSG_ReadByte(msg) as libc::c_float / 64.0f32
    } else { attn = 0 as libc::c_int as libc::c_float }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        pitch = MSG_ReadByte(msg)
    } else { pitch = 100 as libc::c_int }
    // entity reletive
    entnum = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
    // positioned in space
    MSG_ReadVec3Coord(msg, pos.as_mut_ptr()); // see precached sound
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        let mut sentenceName: [libc::c_char; 32] = [0; 32]; // too early
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            Q_snprintf(sentenceName.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"!#%i\x00" as *const u8 as *const libc::c_char,
                       sound + ((1 as libc::c_int) << 11 as libc::c_int));
        } else {
            Q_snprintf(sentenceName.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"!%i\x00" as *const u8 as *const libc::c_char, sound);
        }
        handle = S_RegisterSound(sentenceName.as_mut_ptr())
    } else { handle = cl.sound_index[sound as usize] as sound_t }
    if cl.audio_prepped as u64 == 0 { return }
    // g-cont. sound and ambient sound have only difference with channel
    if chan == 6 as libc::c_int {
        S_AmbientSound(pos.as_mut_ptr() as *const vec_t, entnum, handle,
                       volume, attn, pitch, flags);
    } else {
        S_StartSound(pos.as_mut_ptr() as *const vec_t, entnum, chan, handle,
                     volume, attn, pitch, flags);
    };
}
/*
==================
CL_ParseRestoreSoundPacket

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseRestoreSoundPacket(mut msg: *mut sizebuf_t) {
    let mut pos: vec3_t = [0.; 3];
    let mut chan: libc::c_int = 0;
    let mut sound: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut attn: libc::c_float = 0.;
    let mut flags: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut entnum: libc::c_int = 0;
    let mut samplePos: libc::c_double = 0.;
    let mut forcedEnd: libc::c_double = 0.;
    let mut wordIndex: libc::c_int = 0;
    let mut handle: sound_t = 0 as libc::c_int;
    flags = MSG_ReadUBitLong(msg, 14 as libc::c_int) as libc::c_int;
    sound = MSG_ReadUBitLong(msg, 11 as libc::c_int) as libc::c_int;
    chan = MSG_ReadUBitLong(msg, 4 as libc::c_int) as libc::c_int;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        volume = MSG_ReadByte(msg) as libc::c_float / 255.0f32
    } else { volume = 1.0f64 as libc::c_float }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        attn = MSG_ReadByte(msg) as libc::c_float / 64.0f32
    } else { attn = 0 as libc::c_int as libc::c_float }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        pitch = MSG_ReadByte(msg)
    } else { pitch = 100 as libc::c_int }
    // entity reletive
    entnum = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
    // positioned in space
    MSG_ReadVec3Coord(msg, pos.as_mut_ptr()); // see precached sound
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        let mut sentenceName: [libc::c_char; 32] = [0; 32];
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            Q_snprintf(sentenceName.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"!%i\x00" as *const u8 as *const libc::c_char,
                       sound + ((1 as libc::c_int) << 11 as libc::c_int));
        } else {
            Q_snprintf(sentenceName.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"!%i\x00" as *const u8 as *const libc::c_char, sound);
        }
        handle = S_RegisterSound(sentenceName.as_mut_ptr())
    } else { handle = cl.sound_index[sound as usize] as sound_t }
    wordIndex = MSG_ReadByte(msg);
    // 16 bytes here
    MSG_ReadBytes(msg,
                  &mut samplePos as *mut libc::c_double as *mut libc::c_void,
                  ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as
                      libc::c_int); // too early
    MSG_ReadBytes(msg,
                  &mut forcedEnd as *mut libc::c_double as *mut libc::c_void,
                  ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as
                      libc::c_int);
    if cl.audio_prepped as u64 == 0 { return }
    S_RestoreSound(pos.as_mut_ptr() as *const vec_t, entnum, chan, handle,
                   volume, attn, pitch, flags, samplePos, forcedEnd,
                   wordIndex);
}
/*
==================
CL_ParseServerTime

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseServerTime(mut msg: *mut sizebuf_t) {
    let mut dt: libc::c_double = 0.; // don't mess the time
    cl.mtime[1 as libc::c_int as usize] = cl.mtime[0 as libc::c_int as usize];
    cl.mtime[0 as libc::c_int as usize] =
        MSG_ReadFloat(msg) as libc::c_double;
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int { return }
    if cl.maxclients == 1 as libc::c_int {
        cl.time = cl.mtime[0 as libc::c_int as usize]
    }
    dt = cl.time - cl.mtime[0 as libc::c_int as usize];
    if __tg_fabs(dt) > (*cl_clockreset).value as libc::c_double {
        // 0.1 by default
        cl.time = cl.mtime[0 as libc::c_int as usize];
        cl.timedelta = 0.0f32
    } else if dt != 0.0f64 { cl.timedelta = dt as libc::c_float }
    if cl.oldtime > cl.time { cl.oldtime = cl.time };
}
/*
==================
CL_ParseSignon

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseSignon(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = MSG_ReadByte(msg);
    if i <= cls.signon {
        Con_Reportf(b"^1Error:^7 received signon %i when at %i\n\x00" as
                        *const u8 as *const libc::c_char, i, cls.signon);
        CL_Disconnect();
        return
    }
    cls.signon = i;
    CL_SignonReply();
}
/*
==================
CL_ParseMovevars

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseMovevars(mut msg: *mut sizebuf_t) {
    Delta_InitClient(); // finalize client delta's
    MSG_ReadDeltaMovevars(msg, &mut clgame.oldmovevars, &mut clgame.movevars);
    // water alpha is not allowed
    if world.flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
           == 0 {
        clgame.movevars.wateralpha = 1.0f32
    }
    // update sky if changed
    if Q_strncmp(clgame.oldmovevars.skyName.as_mut_ptr(),
                 clgame.movevars.skyName.as_mut_ptr(), 99999 as libc::c_int)
           != 0 && cl.video_prepped as libc::c_uint != 0 {
        ref_0.dllFuncs.R_SetupSky.expect("non-null function pointer")(clgame.movevars.skyName.as_mut_ptr());
    }
    memcpy(&mut clgame.oldmovevars as *mut movevars_t as *mut libc::c_void,
           &mut clgame.movevars as *mut movevars_t as *const libc::c_void,
           ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
    (*clgame.entities).curstate.scale = clgame.movevars.waveHeight;
    // keep features an actual!
    clgame.movevars.features = host.features as libc::c_int;
    clgame.oldmovevars.features = clgame.movevars.features;
}
/*
==================
CL_ParseParticles

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseParticles(mut msg: *mut sizebuf_t) {
    let mut org: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut life: libc::c_float = 0.;
    MSG_ReadVec3Coord(msg, org.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        dir[i as usize] = MSG_ReadChar(msg) as libc::c_float * 0.0625f32;
        i += 1
    }
    count = MSG_ReadByte(msg);
    color = MSG_ReadByte(msg);
    if count == 255 as libc::c_int { count = 1024 as libc::c_int }
    life = MSG_ReadByte(msg) as libc::c_float * 0.125f32;
    if life != 0.0f32 && count == 1 as libc::c_int {
        let mut p: *mut particle_t = 0 as *mut particle_t;
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die += life;
        (*p).color = color as libc::c_short;
        (*p).type_0 = pt_static;
        (*p).org[0 as libc::c_int as usize] = org[0 as libc::c_int as usize];
        (*p).org[1 as libc::c_int as usize] = org[1 as libc::c_int as usize];
        (*p).org[2 as libc::c_int as usize] = org[2 as libc::c_int as usize];
        (*p).vel[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize];
        (*p).vel[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize];
        (*p).vel[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize]
    } else {
        R_RunParticleEffect(org.as_mut_ptr() as *const vec_t,
                            dir.as_mut_ptr() as *const vec_t, color, count);
    };
}
/*
==================
CL_ParseStaticEntity

static client entity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseStaticEntity(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut from: entity_state_t =
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
                       vuser4: [0.; 3],};
    let mut to: entity_state_t =
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
                       vuser4: [0.; 3],};
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    memset(&mut from as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    newnum = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
    MSG_ReadDeltaEntity(msg, &mut from, &mut to, 0 as libc::c_int,
                        DELTA_STATIC as libc::c_int,
                        cl.mtime[0 as libc::c_int as usize]);
    i = clgame.numStatics;
    if i >= 3096 as libc::c_int {
        Con_Printf(b"^1Error:^7 MAX_STATIC_ENTITIES limit exceeded!\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    ent = &mut *clgame.static_entities.offset(i as isize) as *mut cl_entity_t;
    clgame.numStatics += 1;
    // all states are same
    (*ent).prevstate = to; // static entities doesn't has the numbers
    (*ent).curstate = (*ent).prevstate;
    (*ent).baseline = (*ent).curstate;
    (*ent).index = 0 as libc::c_int;
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
    (*ent).model = CL_ModelHandle(to.modelindex);
    (*ent).curstate.framerate = 1.0f32;
    CL_ResetLatchedVars(ent, true_0);
    if (*ent).curstate.rendermode == kRenderNormal as libc::c_int &&
           !(*ent).model.is_null() {
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
==================
CL_WeaponAnim

Set new weapon animation
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WeaponAnim(mut iAnim: libc::c_int,
                                       mut body: libc::c_int) {
    let mut view: *mut cl_entity_t = &mut clgame.viewent;
    cl.local.weaponstarttime = 0.0f32;
    cl.local.weaponsequence = iAnim;
    (*view).curstate.framerate = 1.0f32;
    (*view).curstate.body = body;
    // g-cont. for GlowShell testing
}
/*
==================
CL_ParseStaticDecal

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseStaticDecal(mut msg: *mut sizebuf_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut decalIndex: libc::c_int = 0;
    let mut entityIndex: libc::c_int = 0;
    let mut modelIndex: libc::c_int = 0;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut scale: libc::c_float = 0.;
    let mut flags: libc::c_int = 0;
    MSG_ReadVec3Coord(msg, origin.as_mut_ptr());
    decalIndex = MSG_ReadWord(msg);
    entityIndex = MSG_ReadShort(msg);
    if entityIndex > 0 as libc::c_int {
        modelIndex = MSG_ReadWord(msg)
    } else { modelIndex = 0 as libc::c_int }
    flags = MSG_ReadByte(msg);
    scale = MSG_ReadWord(msg) as libc::c_float / 4096.0f32;
    CL_FireCustomDecal(CL_DecalIndex(decalIndex), entityIndex, modelIndex,
                       origin.as_mut_ptr(), flags, scale);
}
/*
==================
CL_ParseSoundFade

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseSoundFade(mut msg: *mut sizebuf_t) {
    let mut fadePercent: libc::c_float = 0.;
    let mut fadeOutSeconds: libc::c_float = 0.;
    let mut holdTime: libc::c_float = 0.;
    let mut fadeInSeconds: libc::c_float = 0.;
    fadePercent = MSG_ReadByte(msg) as libc::c_float;
    holdTime = MSG_ReadByte(msg) as libc::c_float;
    fadeOutSeconds = MSG_ReadByte(msg) as libc::c_float;
    fadeInSeconds = MSG_ReadByte(msg) as libc::c_float;
    S_FadeClientVolume(fadePercent, fadeOutSeconds, holdTime, fadeInSeconds);
}
/*
==================
CL_RequestMissingResources

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RequestMissingResources() -> qboolean {
    let mut p: *mut resource_t = 0 as *mut resource_t;
    if cls.dl.doneregistering as u64 == 0 &&
           (cls.dl.custom as libc::c_uint != 0 ||
                cls.state as libc::c_uint ==
                    ca_validate as libc::c_int as libc::c_uint) {
        p = cl.resourcesneeded.pNext;
        if p == &mut cl.resourcesneeded as *mut resource_t {
            cls.dl.doneregistering = true_0;
            host.downloadcount = 0 as libc::c_int;
            cls.dl.custom = false_0
        } else if (*p).ucFlags as libc::c_int &
                      (1 as libc::c_int) << 1 as libc::c_int == 0 {
            CL_MoveToOnHandList(cl.resourcesneeded.pNext);
            return true_0
        }
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CL_BatchResourceRequest(mut initialize: qboolean) {
    let mut data: [byte; 131072] = [0; 131072];
    let mut p: *mut resource_t = 0 as *mut resource_t;
    let mut n: *mut resource_t = 0 as *mut resource_t;
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    MSG_InitExt(&mut msg,
                b"Resource Batch\x00" as *const u8 as *const libc::c_char,
                data.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // client resources is not precached by server
    if initialize as u64 != 0 { CL_AddClientResources(); }
    p = cl.resourcesneeded.pNext;
    while !p.is_null() && p != &mut cl.resourcesneeded as *mut resource_t {
        n = (*p).pNext;
        if (*p).ucFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int == 0 {
            CL_MoveToOnHandList(p);
        } else if cls.state as libc::c_uint ==
                      ca_active as libc::c_int as libc::c_uint &&
                      cl_download_ingame.value == 0. {
            Con_Printf(b"skipping in game download of %s\n\x00" as *const u8
                           as *const libc::c_char,
                       (*p).szFileName.as_mut_ptr());
            CL_MoveToOnHandList(p);
        } else {
            match (*p).type_0 as libc::c_uint {
                0 => {
                    if !(CL_CheckFile(&mut msg, p) as u64 == 0) {
                        CL_MoveToOnHandList(p);
                    }
                }
                1 => { CL_MoveToOnHandList(p); }
                2 => {
                    if !(CL_CheckFile(&mut msg, p) as u64 == 0) {
                        CL_MoveToOnHandList(p);
                    }
                }
                3 => {
                    if HPAK_GetDataPointer(b"custom.hpk\x00" as *const u8 as
                                               *const libc::c_char, p,
                                           0 as *mut *mut byte,
                                           0 as *mut libc::c_int) as u64 == 0
                       {
                        if (*p).ucFlags as libc::c_int &
                               (1 as libc::c_int) << 3 as libc::c_int == 0 {
                            MSG_WriteCmdExt(&mut msg, 3 as libc::c_int,
                                            NS_CLIENT,
                                            0 as *const libc::c_char);
                            MSG_WriteString(&mut msg,
                                            va(b"dlfile !MD5%s\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               MD5_Print((*p).rgucMD5_hash.as_mut_ptr())));
                            (*p).ucFlags =
                                ((*p).ucFlags as libc::c_int |
                                     (1 as libc::c_int) << 3 as libc::c_int)
                                    as libc::c_uchar
                        }
                    } else { CL_MoveToOnHandList(p); }
                }
                4 => {
                    if COM_IsSafeFileToDownload((*p).szFileName.as_mut_ptr())
                           as u64 == 0 {
                        CL_RemoveFromResourceList(p);
                        _Mem_Free(p as *mut libc::c_void,
                                  b"../engine/client/cl_parse.c\x00" as
                                      *const u8 as *const libc::c_char,
                                  516 as libc::c_int);
                    } else if !(CL_CheckFile(&mut msg, p) as u64 == 0) {
                        CL_MoveToOnHandList(p);
                    }
                }
                5 => {
                    if !(CL_CheckFile(&mut msg, p) as u64 == 0) {
                        CL_MoveToOnHandList(p);
                    }
                }
                6 => {
                    if 0 as libc::c_int == 0 {
                        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8
                                      as *const libc::c_char,
                                  b"../engine/client/cl_parse.c\x00" as
                                      *const u8 as *const libc::c_char,
                                  529 as libc::c_int);
                    }
                }
                _ => { }
            }
        }
        p = n
    }
    if cls.state as libc::c_uint !=
           ca_disconnected as libc::c_int as libc::c_uint {
        if cl.downloadUrl[0 as libc::c_int as usize] == 0 &&
               MSG_GetNumBytesWritten(&mut msg) == 0 &&
               CL_PrecacheResources() as libc::c_uint != 0 {
            CL_RegisterResources(&mut msg);
        }
        if cl.downloadUrl[0 as libc::c_int as usize] as libc::c_int != 0 &&
               host.downloadcount == 0 as libc::c_int &&
               CL_PrecacheResources() as libc::c_uint != 0 {
            CL_RegisterResources(&mut msg);
        }
        Netchan_CreateFragments(&mut cls.netchan, &mut msg);
        Netchan_FragSend(&mut cls.netchan);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_EstimateNeededResources() -> libc::c_int {
    let mut p: *mut resource_t = 0 as *mut resource_t;
    let mut nTotalSize: libc::c_int = 0 as libc::c_int;
    p = cl.resourcesneeded.pNext;
    while p != &mut cl.resourcesneeded as *mut resource_t {
        match (*p).type_0 as libc::c_uint {
            0 => {
                if (*p).szFileName[0 as libc::c_int as usize] as libc::c_int
                       != '*' as i32 &&
                       FS_FileExists(va(b"sound/%s\x00" as *const u8 as
                                            *const libc::c_char,
                                        (*p).szFileName.as_mut_ptr()),
                                     false_0 as libc::c_int) == 0 {
                    (*p).ucFlags =
                        ((*p).ucFlags as libc::c_int |
                             (1 as libc::c_int) << 1 as libc::c_int) as
                            libc::c_uchar;
                    nTotalSize += (*p).nDownloadSize
                }
            }
            2 => {
                if (*p).szFileName[0 as libc::c_int as usize] as libc::c_int
                       != '*' as i32 &&
                       FS_FileExists((*p).szFileName.as_mut_ptr(),
                                     false_0 as libc::c_int) == 0 {
                    (*p).ucFlags =
                        ((*p).ucFlags as libc::c_int |
                             (1 as libc::c_int) << 1 as libc::c_int) as
                            libc::c_uchar;
                    nTotalSize += (*p).nDownloadSize
                }
            }
            1 | 4 | 5 => {
                if FS_FileExists((*p).szFileName.as_mut_ptr(),
                                 false_0 as libc::c_int) == 0 {
                    (*p).ucFlags =
                        ((*p).ucFlags as libc::c_int |
                             (1 as libc::c_int) << 1 as libc::c_int) as
                            libc::c_uchar;
                    nTotalSize += (*p).nDownloadSize
                }
            }
            3 => {
                if (*p).ucFlags as libc::c_int &
                       (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    (*p).ucFlags =
                        ((*p).ucFlags as libc::c_int |
                             (1 as libc::c_int) << 1 as libc::c_int) as
                            libc::c_uchar;
                    nTotalSize += (*p).nDownloadSize
                }
            }
            6 => {
                if 0 as libc::c_int == 0 {
                    Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../engine/client/cl_parse.c\x00" as *const u8
                                  as *const libc::c_char, 590 as libc::c_int);
                }
            }
            _ => { }
        }
        p = (*p).pNext
    }
    return nTotalSize;
}
#[no_mangle]
pub unsafe extern "C" fn CL_StartResourceDownloading(mut pszMessage:
                                                         *const libc::c_char,
                                                     mut bCustom: qboolean) {
    let mut ri: resourceinfo_t =
        resourceinfo_t{info: [_resourceinfo_t{size: 0,}; 8],};
    if if pszMessage.is_null() || *pszMessage == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        Con_DPrintf(b"%s\x00" as *const u8 as *const libc::c_char,
                    pszMessage);
    }
    cls.dl.nTotalSize =
        COM_SizeofResourceList(&mut cl.resourcesneeded, &mut ri);
    cls.dl.nTotalToTransfer = CL_EstimateNeededResources();
    if bCustom as u64 != 0 {
        cls.dl.custom = true_0
    } else { cls.state = ca_validate; cls.dl.custom = false_0 }
    cls.dl.doneregistering = false_0;
    cls.dl.fLastStatusUpdate = host.realtime as libc::c_float;
    cls.dl.nRemainingToTransfer = cls.dl.nTotalToTransfer;
    memset(cls.dl.rgStats.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[downloadtime_t; 8]>() as libc::c_ulong);
    cls.dl.nCurStat = 0 as libc::c_int;
    CL_BatchResourceRequest((bCustom as u64 == 0) as libc::c_int as qboolean);
}
#[no_mangle]
pub unsafe extern "C" fn CL_PlayerHasCustomization(mut nPlayerNum:
                                                       libc::c_int,
                                                   mut type_0: resourcetype_t)
 -> *mut customization_t {
    let mut pList: *mut customization_t = 0 as *mut customization_t;
    pList = cl.players[nPlayerNum as usize].customdata.pNext;
    while !pList.is_null() {
        if (*pList).resource.type_0 as libc::c_uint == type_0 as libc::c_uint
           {
            return pList
        }
        pList = (*pList).pNext
    }
    return 0 as *mut customization_t;
}
#[no_mangle]
pub unsafe extern "C" fn CL_RemoveCustomization(mut nPlayerNum: libc::c_int,
                                                mut pRemove:
                                                    *mut customization_t) {
    let mut pList: *mut customization_t = 0 as *mut customization_t;
    let mut pNext: *mut customization_t = 0 as *mut customization_t;
    pList = cl.players[nPlayerNum as usize].customdata.pNext;
    while !pList.is_null() {
        pNext = (*pList).pNext;
        if pRemove != pList {
            pList = pNext
        } else {
            if (*pList).bInUse as libc::c_uint != 0 &&
                   !(*pList).pBuffer.is_null() {
                _Mem_Free((*pList).pBuffer,
                          b"../engine/client/cl_parse.c\x00" as *const u8 as
                              *const libc::c_char, 652 as libc::c_int);
            }
            if (*pList).bInUse as libc::c_uint != 0 &&
                   !(*pList).pInfo.is_null() {
                if (*pList).resource.type_0 as libc::c_uint ==
                       t_decal as libc::c_int as libc::c_uint {
                    if cls.state as libc::c_uint ==
                           ca_active as libc::c_int as libc::c_uint {
                        ref_0.dllFuncs.R_DecalRemoveAll.expect("non-null function pointer")((*pList).nUserData1);
                    }
                    FS_FreeImage((*pList).pInfo as *mut rgbdata_t);
                }
            }
            cl.players[nPlayerNum as usize].customdata.pNext = pNext;
            _Mem_Free(pList as *mut libc::c_void,
                      b"../engine/client/cl_parse.c\x00" as *const u8 as
                          *const libc::c_char, 665 as libc::c_int);
            break ;
        }
    };
}
/*
==================
CL_ParseCustomization

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseCustomization(mut msg: *mut sizebuf_t) {
    let mut pExistingCustomization: *mut customization_t =
        0 as *mut customization_t;
    let mut pList: *mut customization_t = 0 as *mut customization_t;
    let mut bFound: qboolean = false_0;
    let mut pRes: *mut resource_t = 0 as *mut resource_t;
    let mut i: libc::c_int = 0;
    i = MSG_ReadByte(msg);
    if i >= (1 as libc::c_int) << 5 as libc::c_int {
        Host_Error(b"Bogus player index during customization parsing.\n\x00"
                       as *const u8 as *const libc::c_char);
    }
    pRes =
        _Mem_Alloc(cls.mempool,
                   ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/client/cl_parse.c\x00" as *const u8 as
                       *const libc::c_char, 688 as libc::c_int) as
            *mut resource_t;
    (*pRes).type_0 = MSG_ReadByte(msg) as resourcetype_t;
    Q_strncpy((*pRes).szFileName.as_mut_ptr(),
              MSG_ReadStringExt(msg, false_0),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*pRes).nIndex = MSG_ReadShort(msg);
    (*pRes).nDownloadSize = MSG_ReadLong(msg);
    (*pRes).ucFlags =
        (MSG_ReadByte(msg) & !((1 as libc::c_int) << 1 as libc::c_int)) as
            libc::c_uchar;
    (*pRes).pPrev = 0 as *mut resource_s;
    (*pRes).pNext = (*pRes).pPrev;
    if (*pRes).ucFlags as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int
           != 0 {
        MSG_ReadBytes(msg,
                      (*pRes).rgucMD5_hash.as_mut_ptr() as *mut libc::c_void,
                      16 as libc::c_int);
    }
    (*pRes).playernum = i as libc::c_uchar;
    if cl_allow_download.value == 0. {
        Con_DPrintf(b"Refusing new resource, cl_allow_download set to 0\n\x00"
                        as *const u8 as *const libc::c_char);
        _Mem_Free(pRes as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 704 as libc::c_int);
        return
    }
    if cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint
           && cl_download_ingame.value == 0. {
        Con_DPrintf(b"Refusing new resource, cl_download_ingame set to 0\n\x00"
                        as *const u8 as *const libc::c_char);
        _Mem_Free(pRes as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 711 as libc::c_int);
        return
    }
    pExistingCustomization = CL_PlayerHasCustomization(i, (*pRes).type_0);
    if !pExistingCustomization.is_null() {
        CL_RemoveCustomization(i, pExistingCustomization);
    }
    bFound = false_0;
    pList = cl.players[(*pRes).playernum as usize].customdata.pNext;
    while !pList.is_null() {
        if memcmp((*pList).resource.rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void,
                  (*pRes).rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
                  16 as libc::c_int as libc::c_ulong) == 0 {
            bFound = true_0;
            break ;
        } else { pList = (*pList).pNext }
    }
    if HPAK_GetDataPointer(b"custom.hpk\x00" as *const u8 as
                               *const libc::c_char, pRes, 0 as *mut *mut byte,
                           0 as *mut libc::c_int) as u64 != 0 {
        let mut bError: qboolean = false_0;
        if bFound as u64 == 0 {
            pList =
                &mut (*cl.players.as_mut_ptr().offset((*pRes).playernum as
                                                          isize)).customdata;
            if COM_CreateCustomization(pList, pRes,
                                       (*pRes).playernum as libc::c_int,
                                       (1 as libc::c_int) << 0 as libc::c_int,
                                       0 as *mut *mut customization_t,
                                       0 as *mut libc::c_int) as u64 == 0 {
                bError = true_0
            }
        } else {
            Con_DPrintf(b"Duplicate resource ignored for local client\n\x00"
                            as *const u8 as *const libc::c_char);
        }
        if bError as u64 != 0 {
            Con_DPrintf(b"Error loading customization\n\x00" as *const u8 as
                            *const libc::c_char);
        }
        _Mem_Free(pRes as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 747 as libc::c_int);
    } else {
        (*pRes).ucFlags =
            ((*pRes).ucFlags as libc::c_int |
                 (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
        CL_AddToResourceList(pRes, &mut cl.resourcesneeded);
        Con_Printf(b"Requesting %s from server\n\x00" as *const u8 as
                       *const libc::c_char, (*pRes).szFileName.as_mut_ptr());
        CL_StartResourceDownloading(b"Custom resource propagation...\n\x00" as
                                        *const u8 as *const libc::c_char,
                                    true_0);
    };
}
/*
==================
CL_ParseResourceRequest

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseResourceRequest(mut msg: *mut sizebuf_t) {
    let mut buffer: [byte; 131072] = [0; 131072];
    let mut i: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    let mut nStartIndex: libc::c_int = 0;
    let mut sbuf: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    MSG_InitExt(&mut sbuf,
                b"ResourceBlock\x00" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    arg = MSG_ReadLong(msg);
    nStartIndex = MSG_ReadLong(msg);
    if cl.servercount != arg { return }
    if nStartIndex < 0 as libc::c_int && nStartIndex > cl.num_resources {
        return
    }
    MSG_WriteCmdExt(&mut sbuf, 5 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    MSG_WriteShort(&mut sbuf, cl.num_resources);
    i = nStartIndex;
    while i < cl.num_resources {
        MSG_WriteString(&mut sbuf,
                        cl.resourcelist[i as usize].szFileName.as_mut_ptr());
        MSG_WriteByte(&mut sbuf,
                      cl.resourcelist[i as usize].type_0 as libc::c_int);
        MSG_WriteShort(&mut sbuf, cl.resourcelist[i as usize].nIndex);
        MSG_WriteLong(&mut sbuf, cl.resourcelist[i as usize].nDownloadSize);
        MSG_WriteByte(&mut sbuf,
                      cl.resourcelist[i as usize].ucFlags as libc::c_int);
        if cl.resourcelist[i as usize].ucFlags as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int != 0 {
            MSG_WriteBytes(&mut sbuf,
                           cl.resourcelist[i as
                                               usize].rgucMD5_hash.as_mut_ptr()
                               as *const libc::c_void, 16 as libc::c_int);
        }
        i += 1
    }
    if MSG_GetNumBytesWritten(&mut sbuf) > 0 as libc::c_int {
        Netchan_CreateFragments(&mut cls.netchan, &mut sbuf);
        Netchan_FragSend(&mut cls.netchan);
    };
}
/*
==================
CL_CreateCustomizationList

loading custom decal for self
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CreateCustomizationList() {
    let mut pResource: *mut resource_t = 0 as *mut resource_t;
    let mut pPlayer: *mut player_info_t = 0 as *mut player_info_t;
    let mut i: libc::c_int = 0;
    pPlayer =
        &mut *cl.players.as_mut_ptr().offset(cl.playernum as isize) as
            *mut player_info_t;
    (*pPlayer).customdata.pNext = 0 as *mut customization_s;
    i = 0 as libc::c_int;
    while i < cl.num_resources {
        pResource =
            &mut *cl.resourcelist.as_mut_ptr().offset(i as isize) as
                *mut resource_t;
        if COM_CreateCustomization(&mut (*pPlayer).customdata, pResource,
                                   cl.playernum, 0 as libc::c_int,
                                   0 as *mut *mut customization_t,
                                   0 as *mut libc::c_int) as u64 == 0 {
            Con_Printf(b"problem with client customization %s, ignoring...\x00"
                           as *const u8 as *const libc::c_char,
                       (*pResource).szFileName.as_mut_ptr());
        }
        i += 1
    };
}
/*
==================
CL_ParseFileTransferFailed

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseFileTransferFailed(mut msg: *mut sizebuf_t) {
    let mut name: *const libc::c_char = MSG_ReadStringExt(msg, false_0);
    if cls.demoplayback == 0 { CL_ProcessFile(false_0, name); };
}
/*
=====================================================================

  SERVER CONNECTING MESSAGES

=====================================================================
*/
/*
==================
CL_ParseServerData
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseServerData(mut msg: *mut sizebuf_t) {
    let mut gamefolder: [libc::c_char; 64] = [0; 64]; // server is changed
    let mut background: qboolean = false_0;
    let mut i: libc::c_int = 0;
    Con_Reportf(b"Serverdata packet received.\n\x00" as *const u8 as
                    *const libc::c_char);
    cls.timestart = Sys_DoubleTime();
    cls.demowaiting = false_0;
    // wipe the client_t struct
    if cls.changelevel as u64 == 0 && cls.changedemo as u64 == 0 {
        CL_ClearState();
    }
    cls.state = ca_connected;
    // parse protocol version number
    i = MSG_ReadLong(msg);
    if i != 49 as libc::c_int {
        Host_Error(b"Server use invalid protocol (%i should be %i)\n\x00" as
                       *const u8 as *const libc::c_char, i,
                   49 as libc::c_int);
    }
    cl.servercount = MSG_ReadLong(msg);
    cl.checksum = MSG_ReadLong(msg) as uint;
    cl.playernum = MSG_ReadByte(msg);
    cl.maxclients = MSG_ReadByte(msg);
    clgame.maxEntities = MSG_ReadWord(msg);
    clgame.maxEntities =
        if clgame.maxEntities >= 64 as libc::c_int {
            if clgame.maxEntities < (1 as libc::c_int) << 13 as libc::c_int {
                clgame.maxEntities
            } else { ((1 as libc::c_int)) << 13 as libc::c_int }
        } else { 64 as libc::c_int };
    clgame.maxModels = MSG_ReadWord(msg);
    Q_strncpy(clgame.mapname.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    Q_strncpy(clgame.maptitle.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    background = MSG_ReadOneBit(msg) as qboolean;
    Q_strncpy(gamefolder.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              64 as libc::c_int as size_t);
    host.features = MSG_ReadLong(msg) as uint;
    // receive the player hulls
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int * 3 as libc::c_int {
        host.player_mins[(i / 3 as libc::c_int) as
                             usize][(i % 3 as libc::c_int) as usize] =
            MSG_ReadChar(msg) as vec_t;
        host.player_maxs[(i / 3 as libc::c_int) as
                             usize][(i % 3 as libc::c_int) as usize] =
            MSG_ReadChar(msg) as vec_t;
        i += 1
    }
    if clgame.maxModels > 1024 as libc::c_int {
        Con_Printf(b"^3Warning:^7 server model limit is above client model limit %i > %i\n\x00"
                       as *const u8 as *const libc::c_char, clgame.maxModels,
                   1024 as libc::c_int);
    }
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
    // set the background state
    if cls.demoplayback != 0 && cls.demonum != -(1 as libc::c_int) {
        // re-init mouse
        host.mouse_visible = false_0;
        cl.background = true_0
    } else { cl.background = background }
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
    if cls.changelevel as u64 == 0 {
        // continue playing if we are changing level
        S_StopBackgroundTrack();
    }
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
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        COM_ClearCustomizationList(&mut (*cl.players.as_mut_ptr().offset(i as
                                                                             isize)).customdata,
                                   true_0);
        i += 1
    }
    CL_CreateCustomizationList();
    // request resources from server
    CL_ServerCommand(true_0,
                     b"sendres %i\n\x00" as *const u8 as *const libc::c_char,
                     cl.servercount);
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
}
/*
===================
CL_ParseClientData
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseClientData(mut msg: *mut sizebuf_t) {
    let mut parsecounttime: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut command_ack: libc::c_int = 0;
    let mut from_cd: *mut clientdata_t = 0 as *mut clientdata_t;
    let mut to_cd: *mut clientdata_t = 0 as *mut clientdata_t;
    let mut from_wd: *mut weapon_data_t = 0 as *mut weapon_data_t;
    let mut to_wd: *mut weapon_data_t = 0 as *mut weapon_data_t;
    let mut nullwd: [weapon_data_t; 64] =
        [weapon_data_t{m_iId: 0,
                       m_iClip: 0,
                       m_flNextPrimaryAttack: 0.,
                       m_flNextSecondaryAttack: 0.,
                       m_flTimeWeaponIdle: 0.,
                       m_fInReload: 0,
                       m_fInSpecialReload: 0,
                       m_flNextReload: 0.,
                       m_flPumpTime: 0.,
                       m_fReloadTime: 0.,
                       m_fAimedDamage: 0.,
                       m_fNextAimBonus: 0.,
                       m_fInZoom: 0,
                       m_iWeaponState: 0,
                       iuser1: 0,
                       iuser2: 0,
                       iuser3: 0,
                       iuser4: 0,
                       fuser1: 0.,
                       fuser2: 0.,
                       fuser3: 0.,
                       fuser4: 0.,}; 64];
    let mut nullcd: clientdata_t =
        clientdata_t{origin: [0.; 3],
                     velocity: [0.; 3],
                     viewmodel: 0,
                     punchangle: [0.; 3],
                     flags: 0,
                     waterlevel: 0,
                     watertype: 0,
                     view_ofs: [0.; 3],
                     health: 0.,
                     bInDuck: 0,
                     weapons: 0,
                     flTimeStepSound: 0,
                     flDuckTime: 0,
                     flSwimTime: 0,
                     waterjumptime: 0,
                     maxspeed: 0.,
                     fov: 0.,
                     weaponanim: 0,
                     m_iId: 0,
                     ammo_shells: 0,
                     ammo_nails: 0,
                     ammo_cells: 0,
                     ammo_rockets: 0,
                     m_flNextAttack: 0.,
                     tfstate: 0,
                     pushmsec: 0,
                     deadflag: 0,
                     physinfo: [0; 256],
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
                     vuser4: [0.; 3],};
    let mut frame: *mut frame_t = 0 as *mut frame_t;
    let mut idx: libc::c_int = 0;
    // This is the last movement that the server ack'd
    command_ack = cls.netchan.incoming_acknowledged as libc::c_int;
    // this is the frame update that this message corresponds to
    i = cls.netchan.incoming_sequence as libc::c_int;
    // did we drop some frames?
    if i > cl.last_incoming_sequence + 1 as libc::c_int {
        // mark as dropped
        j =
            cl.last_incoming_sequence +
                1 as libc::c_int; // ack'd incoming messages.
        while j < i {
            if cl.frames[(j & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                             usize].receivedtime >= 0.0f64 {
                cl.frames[(j & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                              usize].receivedtime =
                    -1.0f32 as libc::c_double; // index into window.
                cl.frames[(j & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                              usize].latency =
                    0 as libc::c_int as libc::c_double
            } // frame at index.
            j += 1
        }
    } // mark network received time
    cl.parsecount = i; // time now that we are parsing.
    cl.parsecountmod = cl.parsecount & CL_UPDATE_BACKUP - 1 as libc::c_int;
    frame =
        &mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as isize) as
            *mut frame_t;
    (*frame).time = cl.mtime[0 as libc::c_int as usize];
    (*frame).receivedtime = host.realtime;
    memset(&mut (*frame).graphdata as *mut netbandwidthgraph_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<netbandwidthgraph_t>() as libc::c_ulong);
    // send time for that frame.
    parsecounttime =
        cl.commands[(command_ack & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                        usize].senttime as libc::c_float;
    // current time that we got a response to the command packet.
    cl.commands[(command_ack & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                    usize].receivedtime = host.realtime;
    if cl.last_command_ack != -(1 as libc::c_int) {
        let mut last_predicted: libc::c_int = 0;
        let mut pcd: *mut clientdata_t = 0 as *mut clientdata_t;
        let mut ppcd: *mut clientdata_t = 0 as *mut clientdata_t;
        let mut ps: *mut entity_state_t = 0 as *mut entity_state_t;
        let mut pps: *mut entity_state_t = 0 as *mut entity_state_t;
        let mut wd: *mut weapon_data_t = 0 as *mut weapon_data_t;
        let mut pwd: *mut weapon_data_t = 0 as *mut weapon_data_t;
        if cls.spectator as u64 == 0 {
            last_predicted =
                cl.last_incoming_sequence +
                    (command_ack - cl.last_command_ack) &
                    CL_UPDATE_BACKUP - 1 as libc::c_int;
            pps =
                &mut (*cl.predicted_frames.as_mut_ptr().offset(last_predicted
                                                                   as
                                                                   isize)).playerstate;
            pwd =
                cl.predicted_frames[last_predicted as
                                        usize].weapondata.as_mut_ptr();
            ppcd =
                &mut (*cl.predicted_frames.as_mut_ptr().offset(last_predicted
                                                                   as
                                                                   isize)).client;
            ps =
                &mut *(*frame).playerstate.as_mut_ptr().offset(cl.playernum as
                                                                   isize) as
                    *mut entity_state_t;
            wd = (*frame).weapondata.as_mut_ptr();
            pcd = &mut (*frame).clientdata
        } else {
            ps = &mut cls.spectator_state.playerstate;
            pps = &mut cls.spectator_state.playerstate;
            pcd = &mut cls.spectator_state.client;
            ppcd = &mut cls.spectator_state.client;
            wd = cls.spectator_state.weapondata.as_mut_ptr();
            pwd = cls.spectator_state.weapondata.as_mut_ptr()
        }
        clgame.dllFuncs.pfnTxferPredictionData.expect("non-null function pointer")(ps,
                                                                                   pps,
                                                                                   pcd,
                                                                                   ppcd,
                                                                                   wd,
                                                                                   pwd);
    }
    // do this after all packets read for this frame?
    cl.last_command_ack = cls.netchan.incoming_acknowledged as libc::c_int;
    cl.last_incoming_sequence = cls.netchan.incoming_sequence as libc::c_int;
    if cls.demoplayback == 0 {
        // calculate latency of this frame.
		// sent time is set when usercmd is sent to server in CL_Move
		// this is the # of seconds the round trip took.
        let mut latency: libc::c_float =
            (host.realtime - parsecounttime as libc::c_double) as
                libc::c_float;
        // fill into frame latency
        (*frame).latency = latency as libc::c_double;
        // negative latency makes no sense.  Huge latency is a problem.
        if latency >= 0.0f32 && latency <= 2.0f32 {
            // drift the average latency towards the observed latency
			// if round trip was fastest so far, just use that for latency value
			// otherwise, move in 1 ms steps toward observed channel latency.
            if latency < cls.latency {
                cls.latency = latency
            } else { cls.latency += 0.001f32 }
            // drift up, so corrections are needed
        }
    } else { (*frame).latency = 0.0f32 as libc::c_double }
    // clientdata for spectators ends here
    if cls.spectator as u64 != 0 {
        cl.local.health = 1 as libc::c_int;
        return
    }
    to_cd = &mut (*frame).clientdata;
    to_wd = (*frame).weapondata.as_mut_ptr();
    // clear to old value before delta parsing
    if MSG_ReadOneBit(msg) != 0 {
        let mut delta_sequence: libc::c_int = MSG_ReadByte(msg);
        from_cd =
            &mut (*cl.frames.as_mut_ptr().offset((delta_sequence &
                                                      CL_UPDATE_BACKUP -
                                                          1 as libc::c_int) as
                                                     isize)).clientdata;
        from_wd =
            cl.frames[(delta_sequence & CL_UPDATE_BACKUP - 1 as libc::c_int)
                          as usize].weapondata.as_mut_ptr()
    } else {
        memset(&mut nullcd as *mut clientdata_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<clientdata_t>() as libc::c_ulong);
        memset(nullwd.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[weapon_data_t; 64]>() as libc::c_ulong);
        from_cd = &mut nullcd;
        from_wd = nullwd.as_mut_ptr()
    }
    MSG_ReadClientData(msg, from_cd, to_cd,
                       cl.mtime[0 as libc::c_int as usize]);
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        // check for end of weapondata (and clientdata_t message)
        if MSG_ReadOneBit(msg) == 0 { break ; }
        // read the weapon idx
        idx =
            MSG_ReadUBitLong(msg,
                             if cls.legacymode as libc::c_uint != 0 {
                                 5 as libc::c_int
                             } else { 6 as libc::c_int }) as libc::c_int;
        MSG_ReadWeaponData(msg, &mut *from_wd.offset(idx as isize),
                           &mut *to_wd.offset(idx as isize),
                           cl.mtime[0 as libc::c_int as usize]);
        i += 1
    }
    // make a local copy of physinfo
    Q_strncpy(cls.physinfo.as_mut_ptr(),
              (*frame).clientdata.physinfo.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    cl.local.maxspeed = (*frame).clientdata.maxspeed;
    cl.local.pushmsec = (*frame).clientdata.pushmsec;
    cl.local.weapons = (*frame).clientdata.weapons;
    cl.local.health = (*frame).clientdata.health as libc::c_int;
}
/*
==================
CL_ParseBaseline
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseBaseline(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0; // finalize client delta's
    let mut newnum: libc::c_int = 0; // end of baselines
    let mut nullstate: entity_state_t =
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
                       vuser4: [0.; 3],};
    let mut player: qboolean = false_0;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    Delta_InitClient();
    memset(&mut nullstate as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    loop  {
        newnum = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
        if newnum ==
               ((1 as libc::c_int) << 13 as libc::c_int) - 1 as libc::c_int {
            break ;
        }
        player = CL_IsPlayerIndex(newnum);
        if newnum >= clgame.maxEntities {
            Host_Error(b"CL_AllocEdict: no free edicts\n\x00" as *const u8 as
                           *const libc::c_char);
        }
        ent = CL_EDICT_NUM(newnum);
        memset(&mut (*ent).prevstate as *mut entity_state_t as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
        (*ent).index = newnum;
        MSG_ReadDeltaEntity(msg, &mut (*ent).prevstate, &mut (*ent).baseline,
                            newnum, player as libc::c_int,
                            1.0f32 as libc::c_double);
    }
    cl.instanced_baseline_count =
        MSG_ReadUBitLong(msg, 6 as libc::c_int) as libc::c_int;
    i = 0 as libc::c_int;
    while i < cl.instanced_baseline_count {
        newnum = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
        MSG_ReadDeltaEntity(msg, &mut nullstate,
                            &mut *cl.instanced_baseline.as_mut_ptr().offset(i
                                                                                as
                                                                                isize),
                            newnum, false_0 as libc::c_int,
                            1.0f32 as libc::c_double);
        i += 1
    };
}
/*
================
CL_ParseLightStyle
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseLightStyle(mut msg: *mut sizebuf_t) {
    let mut style: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut f: libc::c_float = 0.;
    style = MSG_ReadByte(msg);
    s = MSG_ReadStringExt(msg, false_0);
    f = MSG_ReadFloat(msg);
    CL_SetLightstyle(style, s, f);
}
/*
================
CL_ParseSetAngle

set the view angle to this absolute value
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseSetAngle(mut msg: *mut sizebuf_t) {
    cl.viewangles[0 as libc::c_int as usize] =
        MSG_ReadBitAngle(msg, 16 as libc::c_int);
    cl.viewangles[1 as libc::c_int as usize] =
        MSG_ReadBitAngle(msg, 16 as libc::c_int);
    cl.viewangles[2 as libc::c_int as usize] =
        MSG_ReadBitAngle(msg, 16 as libc::c_int);
}
/*
================
CL_ParseAddAngle

add the view angle yaw
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseAddAngle(mut msg: *mut sizebuf_t) {
    let mut a: *mut pred_viewangle_t = 0 as *mut pred_viewangle_t;
    let mut delta_yaw: libc::c_float = 0.;
    delta_yaw = MSG_ReadBitAngle(msg, 16 as libc::c_int);
    // update running counter
    cl.addangletotal += delta_yaw;
    // select entry into circular buffer
    cl.angle_position =
        cl.angle_position + 1 as libc::c_int &
            16 as libc::c_int - 1 as libc::c_int;
    a =
        &mut *cl.predicted_angle.as_mut_ptr().offset(cl.angle_position as
                                                         isize) as
            *mut pred_viewangle_t;
    // record update
    (*a).starttime = cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    (*a).total = cl.addangletotal;
}
/*
================
CL_ParseCrosshairAngle

offset crosshair angles
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseCrosshairAngle(mut msg: *mut sizebuf_t) {
    cl.crosshairangle[0 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * 0.2f32;
    cl.crosshairangle[1 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * 0.2f32;
    cl.crosshairangle[2 as libc::c_int as usize] = 0.0f32;
    // not used for screen space
}
/*
================
CL_ParseRestore

reading decals, etc.
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseRestore(mut msg: *mut sizebuf_t) {
    let mut filename: string = [0; 256];
    let mut i: libc::c_int = 0;
    let mut mapCount: libc::c_int = 0;
    let mut pMapName: *mut libc::c_char = 0 as *mut libc::c_char;
    // mapname.HL2
    Q_strncpy(filename.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    mapCount = MSG_ReadByte(msg);
    // g-cont. acutally in Xash3D this does nothing.
	// decals already restored on a server, and correctly transferred through levels
	// but i'm leave this message for backward compatibility
    i = 0 as libc::c_int;
    while i < mapCount {
        pMapName = MSG_ReadStringExt(msg, false_0);
        Con_Printf(b"Loading decals from %s\n\x00" as *const u8 as
                       *const libc::c_char, pMapName);
        i += 1
    };
}
/*
================
CL_RegisterUserMessage

register new user message or update existing
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RegisterUserMessage(mut msg: *mut sizebuf_t) {
    let mut pszName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut svc_num: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    svc_num = MSG_ReadByte(msg);
    if cls.legacymode as u64 != 0 {
        size = MSG_ReadByte(msg);
        bits = 8 as libc::c_int
    } else { size = MSG_ReadWord(msg); bits = 16 as libc::c_int }
    pszName = MSG_ReadStringExt(msg, false_0);
    // important stuff
    if size as libc::c_uint ==
           ((1 as libc::c_uint) <<
                bits).wrapping_sub(1 as libc::c_int as libc::c_uint) {
        size = -(1 as libc::c_int)
    }
    svc_num =
        if svc_num >= 0 as libc::c_int {
            if svc_num < 255 as libc::c_int {
                svc_num
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    CL_LinkUserMessage(pszName, svc_num, size);
}
/*
================
CL_RegisterUserMessage

register new user message or update existing
================
*/
/*
void CL_LegacyRegisterUserMessage( sizebuf_t *msg )
{
	char	*pszName;
	int	svc_num, size;

	svc_num = MSG_ReadByte( msg );
	size = MSG_ReadByte( msg );
	pszName = MSG_ReadString( msg );

	// important stuff
	if( size == 0xFF ) size = -1;
	svc_num = bound( 0, svc_num, 255 );

	CL_LinkUserMessage( pszName, svc_num, size );
}
*/
/*
================
CL_UpdateUserinfo

collect userinfo from all players
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateUserinfo(mut msg: *mut sizebuf_t) {
    let mut slot: libc::c_int = 0; // unique user ID
    let mut id: libc::c_int = 0;
    let mut active: qboolean = false_0;
    let mut player: *mut player_info_t = 0 as *mut player_info_t;
    slot = MSG_ReadUBitLong(msg, 5 as libc::c_int) as libc::c_int;
    if slot >= (1 as libc::c_int) << 5 as libc::c_int {
        Host_Error(b"CL_ParseServerMessage: svc_updateuserinfo >= MAX_CLIENTS\n\x00"
                       as *const u8 as *const libc::c_char);
    }
    id = MSG_ReadLong(msg);
    player =
        &mut *cl.players.as_mut_ptr().offset(slot as isize) as
            *mut player_info_t;
    active =
        if MSG_ReadOneBit(msg) != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    if active as u64 != 0 {
        Q_strncpy((*player).userinfo.as_mut_ptr(),
                  MSG_ReadStringExt(msg, false_0),
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
        Q_strncpy((*player).name.as_mut_ptr(),
                  Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                   b"name\x00" as *const u8 as
                                       *const libc::c_char),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong);
        Q_strncpy((*player).model.as_mut_ptr(),
                  Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                   b"model\x00" as *const u8 as
                                       *const libc::c_char),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        (*player).topcolor =
            Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                    b"topcolor\x00" as *const u8 as
                                        *const libc::c_char));
        (*player).bottomcolor =
            Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                    b"bottomcolor\x00" as *const u8 as
                                        *const libc::c_char));
        (*player).spectator =
            Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                    b"*hltv\x00" as *const u8 as
                                        *const libc::c_char));
        MSG_ReadBytes(msg,
                      (*player).hashedcdkey.as_mut_ptr() as *mut libc::c_void,
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong as libc::c_int);
        if slot == cl.playernum {
            memcpy(&mut gameui.playerinfo as *mut player_info_t as
                       *mut libc::c_void, player as *const libc::c_void,
                   ::std::mem::size_of::<player_info_t>() as libc::c_ulong);
        }
    } else {
        memset(player as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<player_info_t>() as libc::c_ulong);
    };
}
/*
==============
CL_ParseResource

downloading and precache resource in-game
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseResource(mut msg: *mut sizebuf_t) {
    let mut pResource: *mut resource_t = 0 as *mut resource_t;
    pResource =
        _Mem_Alloc(cls.mempool,
                   ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/client/cl_parse.c\x00" as *const u8 as
                       *const libc::c_char, 1397 as libc::c_int) as
            *mut resource_t;
    (*pResource).type_0 =
        MSG_ReadUBitLong(msg, 4 as libc::c_int) as resourcetype_t;
    Q_strncpy((*pResource).szFileName.as_mut_ptr(),
              MSG_ReadStringExt(msg, false_0),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*pResource).nIndex =
        MSG_ReadUBitLong(msg, 12 as libc::c_int) as libc::c_int;
    (*pResource).nDownloadSize = MSG_ReadSBitLong(msg, 24 as libc::c_int);
    (*pResource).ucFlags =
        (MSG_ReadUBitLong(msg, 3 as libc::c_int) &
             !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint) as
            libc::c_uchar;
    if (*pResource).ucFlags as libc::c_int &
           (1 as libc::c_int) << 2 as libc::c_int != 0 {
        MSG_ReadBytes(msg,
                      (*pResource).rgucMD5_hash.as_mut_ptr() as
                          *mut libc::c_void,
                      ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                          libc::c_ulong as libc::c_int);
    }
    if MSG_ReadOneBit(msg) != 0 {
        MSG_ReadBytes(msg,
                      (*pResource).rguc_reserved.as_mut_ptr() as
                          *mut libc::c_void,
                      ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                          libc::c_ulong as libc::c_int);
    }
    if (*pResource).type_0 as libc::c_uint ==
           t_sound as libc::c_int as libc::c_uint &&
           (*pResource).nIndex > (1 as libc::c_int) << 11 as libc::c_int {
        _Mem_Free(pResource as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 1413 as libc::c_int);
        Host_Error(b"bad sound index\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    if (*pResource).type_0 as libc::c_uint ==
           t_model as libc::c_int as libc::c_uint &&
           (*pResource).nIndex > 1024 as libc::c_int {
        _Mem_Free(pResource as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 1419 as libc::c_int);
        Host_Error(b"bad model index\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    if (*pResource).type_0 as libc::c_uint ==
           t_eventscript as libc::c_int as libc::c_uint &&
           (*pResource).nIndex > (1 as libc::c_int) << 10 as libc::c_int {
        _Mem_Free(pResource as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 1425 as libc::c_int);
        Host_Error(b"bad event index\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    if (*pResource).type_0 as libc::c_uint ==
           t_generic as libc::c_int as libc::c_uint &&
           (*pResource).nIndex > (1 as libc::c_int) << 10 as libc::c_int {
        _Mem_Free(pResource as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 1431 as libc::c_int);
        Host_Error(b"bad file index\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    if (*pResource).type_0 as libc::c_uint ==
           t_decal as libc::c_int as libc::c_uint &&
           (*pResource).nIndex > 512 as libc::c_int {
        _Mem_Free(pResource as *mut libc::c_void,
                  b"../engine/client/cl_parse.c\x00" as *const u8 as
                      *const libc::c_char, 1437 as libc::c_int);
        Host_Error(b"bad decal index\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    CL_AddToResourceList(pResource, &mut cl.resourcesneeded);
}
/*
================
CL_UpdateUserPings

collect pings and packet lossage from clients
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateUserPings(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0; // end of message
    let mut slot: libc::c_int = 0; // g-cont. especially swapped
    let mut player: *mut player_info_t = 0 as *mut player_info_t;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        if MSG_ReadOneBit(msg) == 0 { break ; }
        slot = MSG_ReadUBitLong(msg, 5 as libc::c_int) as libc::c_int;
        if slot >= (1 as libc::c_int) << 5 as libc::c_int {
            Host_Error(b"CL_ParseServerMessage: svc_pings > MAX_CLIENTS\n\x00"
                           as *const u8 as *const libc::c_char);
        }
        player =
            &mut *cl.players.as_mut_ptr().offset(slot as isize) as
                *mut player_info_t;
        (*player).ping =
            MSG_ReadUBitLong(msg, 12 as libc::c_int) as libc::c_int;
        (*player).packet_loss =
            MSG_ReadUBitLong(msg, 7 as libc::c_int) as libc::c_int;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_SendConsistencyInfo(mut msg: *mut sizebuf_t) {
    let mut user_changed_diskfile: qboolean = false_0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut filename: string = [0; 256];
    let mut crcFile: CRC32_t = 0;
    let mut md5: [byte; 16] = [0; 16];
    let mut pc: *mut consistency_t = 0 as *mut consistency_t;
    let mut i: libc::c_int = 0;
    if cl.need_force_consistency_response as u64 == 0 { return }
    cl.need_force_consistency_response = false_0;
    MSG_WriteCmdExt(msg, 7 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < cl.num_consistency {
        pc =
            &mut *cl.consistency_list.as_mut_ptr().offset(i as isize) as
                *mut consistency_t;
        user_changed_diskfile = false_0;
        MSG_WriteOneBit(msg, 1 as libc::c_int);
        MSG_WriteUBitLong(msg, (*pc).orig_index as uint, 12 as libc::c_int);
        if (*pc).issound as u64 != 0 {
            Q_snprintf(filename.as_mut_ptr(),
                       ::std::mem::size_of::<string>() as libc::c_ulong,
                       b"sound/%s\x00" as *const u8 as *const libc::c_char,
                       (*pc).filename);
        } else {
            Q_strncpy(filename.as_mut_ptr(), (*pc).filename,
                      ::std::mem::size_of::<string>() as libc::c_ulong);
        }
        if !Q_strstr(filename.as_mut_ptr(),
                     b"models/\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            CRC32_Init(&mut crcFile);
            CRC32_File(&mut crcFile, filename.as_mut_ptr());
            crcFile = CRC32_Final(crcFile);
            user_changed_diskfile =
                (Mod_ValidateCRC(filename.as_mut_ptr(), crcFile) as u64 == 0)
                    as libc::c_int as qboolean
        }
        match (*pc).check_type {
            0 => {
                MD5_HashFile(md5.as_mut_ptr(), filename.as_mut_ptr(),
                             0 as *mut uint);
                (*pc).value = *(md5.as_mut_ptr() as *mut libc::c_int);
                if user_changed_diskfile as u64 != 0 {
                    MSG_WriteUBitLong(msg, 0 as libc::c_int as uint,
                                      32 as libc::c_int);
                } else {
                    MSG_WriteUBitLong(msg, (*pc).value as uint,
                                      32 as libc::c_int);
                }
            }
            1 | 2 => {
                if Mod_GetStudioBounds(filename.as_mut_ptr(),
                                       mins.as_mut_ptr(), maxs.as_mut_ptr())
                       as u64 == 0 {
                    Host_Error(b"unable to find %s\n\x00" as *const u8 as
                                   *const libc::c_char,
                               filename.as_mut_ptr());
                }
                if user_changed_diskfile as u64 != 0 {
                    ClearBounds(maxs.as_mut_ptr(), mins.as_mut_ptr());
                }
                MSG_WriteBytes(msg, mins.as_mut_ptr() as *const libc::c_void,
                               12 as libc::c_int);
                MSG_WriteBytes(msg, maxs.as_mut_ptr() as *const libc::c_void,
                               12 as libc::c_int);
            }
            _ => {
                Host_Error(b"Unknown consistency type %i\n\x00" as *const u8
                               as *const libc::c_char, (*pc).check_type);
            }
        }
        i += 1
    }
    MSG_WriteOneBit(msg, 0 as libc::c_int);
}
/*
==================
CL_RegisterResources

Clean up and move to next part of sequence.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RegisterResources(mut msg: *mut sizebuf_t) {
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    if cls.dl.custom as libc::c_uint != 0 ||
           cls.signon == 2 as libc::c_int &&
               cls.state as libc::c_uint ==
                   ca_active as libc::c_int as libc::c_uint {
        cls.dl.custom = false_0;
        return
    }
    if cls.demoplayback == 0 { CL_SendConsistencyInfo(msg); }
    // All done precaching.
    cl.worldmodel = CL_ModelHandle(1 as libc::c_int); // get world pointer
    if !cl.worldmodel.is_null() && cl.maxclients > 0 as libc::c_int {
        if clgame.entities.is_null() {
            Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/client/cl_parse.c\x00" as *const u8 as
                          *const libc::c_char, 1561 as libc::c_int);
        }
        (*clgame.entities).model = cl.worldmodel;
        if cl.video_prepped as u64 == 0 && cl.audio_prepped as u64 == 0 {
            Con_Printf(b"Setting up renderer...\n\x00" as *const u8 as
                           *const libc::c_char);
            // load tempent sprites (glowshell, muzzleflashes etc)
            CL_LoadClientSprites();
            // invalidate all decal indexes
            memset(cl.decal_index.as_mut_ptr() as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<[libc::c_short; 512]>() as
                       libc::c_ulong);
            cl.video_prepped = true_0;
            cl.audio_prepped = true_0;
            CL_ClearWorld();
            // update the ref state.
            R_UpdateRefState();
            // tell rendering system we have a new set of models.
            ref_0.dllFuncs.R_NewMap.expect("non-null function pointer")();
            CL_SetupOverviewParams();
            // release unused SpriteTextures
            i = 1 as libc::c_int; // clear any lines of console text
            mod_0 = clgame.sprites.as_mut_ptr();
            while i < 256 as libc::c_int {
                if (*mod_0).needload as libc::c_uint ==
                       0 as libc::c_int as libc::c_uint &&
                       (if (*mod_0).name.as_mut_ptr().is_null() ||
                               *(*mod_0).name.as_mut_ptr() == 0 {
                            0 as libc::c_int
                        } else { 1 as libc::c_int }) != 0 {
                    Mod_FreeModel(mod_0);
                }
                i += 1;
                mod_0 = mod_0.offset(1)
            }
            Mod_FreeUnused();
            if host_developer.value <=
                   DEV_NONE as libc::c_int as libc::c_float {
                Con_ClearNotify();
            }
            // done with all resources, issue prespawn command.
			// Include server count in case server disconnects and changes level during d/l
            MSG_WriteCmdExt(msg, 3 as libc::c_int, NS_CLIENT,
                            0 as *const libc::c_char);
            MSG_WriteString(msg,
                            va(b"spawn %i\x00" as *const u8 as
                                   *const libc::c_char, cl.servercount));
        }
    } else {
        Con_Printf(b"^1Error:^7 client world model is NULL\n\x00" as *const u8
                       as *const libc::c_char);
        CL_Disconnect();
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_ParseConsistencyInfo(mut msg: *mut sizebuf_t) {
    let mut lastcheck: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut isdelta: libc::c_int = 0;
    let mut pResource: *mut resource_t = 0 as *mut resource_t;
    let mut skip_crc_change: *mut resource_t = 0 as *mut resource_t;
    let mut skip: libc::c_int = 0;
    let mut pc: *mut consistency_t = 0 as *mut consistency_t;
    let mut nullbuffer: [byte; 32] = [0; 32];
    memset(nullbuffer.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           32 as libc::c_int as libc::c_ulong);
    cl.need_force_consistency_response = MSG_ReadOneBit(msg) as qboolean;
    pResource = cl.resourcesneeded.pNext;
    if cl.need_force_consistency_response as u64 == 0 { return }
    skip_crc_change = 0 as *mut resource_t;
    lastcheck = 0 as libc::c_int;
    while MSG_ReadOneBit(msg) != 0 {
        isdelta = MSG_ReadOneBit(msg);
        if isdelta != 0 {
            delta =
                MSG_ReadUBitLong(msg,
                                 5 as
                                     libc::c_int).wrapping_add(lastcheck as
                                                                   libc::c_uint)
                    as libc::c_int
        } else {
            delta = MSG_ReadUBitLong(msg, 12 as libc::c_int) as libc::c_int
        }
        skip = delta - lastcheck;
        i = 0 as libc::c_int;
        while i < skip {
            if pResource != skip_crc_change &&
                   !Q_strstr((*pResource).szFileName.as_mut_ptr(),
                             b"models/\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
                Mod_NeedCRC((*pResource).szFileName.as_mut_ptr(), false_0);
            }
            pResource = (*pResource).pNext;
            i += 1
        }
        if cl.num_consistency >= 1024 as libc::c_int {
            Host_Error(b"CL_CheckConsistency: MAX_MODELS limit exceeded (%d)\n\x00"
                           as *const u8 as *const libc::c_char,
                       1024 as libc::c_int);
        }
        pc =
            &mut *cl.consistency_list.as_mut_ptr().offset(cl.num_consistency
                                                              as isize) as
                *mut consistency_t;
        cl.num_consistency += 1;
        memset(pc as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<consistency_t>() as libc::c_ulong);
        (*pc).filename = (*pResource).szFileName.as_mut_ptr();
        (*pc).issound =
            ((*pResource).type_0 as libc::c_uint ==
                 t_sound as libc::c_int as libc::c_uint) as libc::c_int as
                qboolean;
        (*pc).orig_index = delta;
        (*pc).value = 0 as libc::c_int;
        if (*pResource).type_0 as libc::c_uint ==
               t_model as libc::c_int as libc::c_uint &&
               memcmp(nullbuffer.as_mut_ptr() as *const libc::c_void,
                      (*pResource).rguc_reserved.as_mut_ptr() as
                          *const libc::c_void,
                      32 as libc::c_int as libc::c_ulong) != 0 {
            (*pc).check_type =
                (*pResource).rguc_reserved[0 as libc::c_int as usize] as
                    libc::c_int
        }
        skip_crc_change = pResource;
        lastcheck = delta
    };
}
/*
==============
CL_ParseResourceList

==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseResourceList(mut msg: *mut sizebuf_t) {
    let mut pResource: *mut resource_t = 0 as *mut resource_t;
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    total = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
    i = 0 as libc::c_int;
    while i < total {
        pResource =
            _Mem_Alloc(cls.mempool,
                       ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/client/cl_parse.c\x00" as *const u8 as
                           *const libc::c_char, 1685 as libc::c_int) as
                *mut resource_t;
        (*pResource).type_0 =
            MSG_ReadUBitLong(msg, 4 as libc::c_int) as resourcetype_t;
        Q_strncpy((*pResource).szFileName.as_mut_ptr(),
                  MSG_ReadStringExt(msg, false_0),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        (*pResource).nIndex =
            MSG_ReadUBitLong(msg, 12 as libc::c_int) as libc::c_int;
        (*pResource).nDownloadSize = MSG_ReadSBitLong(msg, 24 as libc::c_int);
        (*pResource).ucFlags =
            (MSG_ReadUBitLong(msg, 3 as libc::c_int) &
                 !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint) as
                libc::c_uchar;
        if (*pResource).ucFlags as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int != 0 {
            MSG_ReadBytes(msg,
                          (*pResource).rgucMD5_hash.as_mut_ptr() as
                              *mut libc::c_void,
                          ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                              libc::c_ulong as libc::c_int);
        }
        if MSG_ReadOneBit(msg) != 0 {
            MSG_ReadBytes(msg,
                          (*pResource).rguc_reserved.as_mut_ptr() as
                              *mut libc::c_void,
                          ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                              libc::c_ulong as libc::c_int);
        }
        CL_AddToResourceList(pResource, &mut cl.resourcesneeded);
        i += 1
    }
    CL_ParseConsistencyInfo(msg);
    CL_StartResourceDownloading(b"Verifying and downloading resources...\n\x00"
                                    as *const u8 as *const libc::c_char,
                                false_0);
}
/*
==================
CL_ParseVoiceInit

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseVoiceInit(mut msg: *mut sizebuf_t) {
    // TODO: ???
}
/*
==================
CL_ParseVoiceData

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseVoiceData(mut msg: *mut sizebuf_t) {
    // TODO: ???
}
/*
==================
CL_ParseResLocation

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseResLocation(mut msg: *mut sizebuf_t) {
    let mut data: *mut libc::c_char = MSG_ReadStringExt(msg, false_0);
    let mut token: [libc::c_char; 256] = [0; 256];
    if Q_strlen(data) > 256 as libc::c_int as libc::c_ulong {
        Con_Printf(b"^1Error:^7 Resource location too long!\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    loop  {
        data =
            _COM_ParseFileSafe(data, token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 256]>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if data.is_null() { break ; }
        Con_Reportf(b"Adding %s as download location\n\x00" as *const u8 as
                        *const libc::c_char, token.as_mut_ptr());
        if cl.downloadUrl[0 as libc::c_int as usize] == 0 {
            Q_strncpy(cl.downloadUrl.as_mut_ptr(), token.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong);
        }
        HTTP_AddCustomServer(token.as_mut_ptr());
    };
}
/*
==============
CL_ParseHLTV

spectator message (hltv)
sended from game.dll
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseHLTV(mut msg: *mut sizebuf_t) {
    match MSG_ReadByte(msg) {
        0 => { cl.proxy_redirect = true_0; cls.spectator = true_0 }
        1 => {
            MSG_ReadLong(msg);
            MSG_ReadShort(msg);
            MSG_ReadWord(msg);
            MSG_ReadLong(msg);
            MSG_ReadLong(msg);
            MSG_ReadWord(msg);
        }
        2 => {
            cls.signon = 2 as libc::c_int;
            NET_StringToAdr(MSG_ReadStringExt(msg, false_0),
                            &mut cls.hltv_listen_address);
            //		NET_JoinGroup( cls.netchan.sock, cls.hltv_listen_address );
            SCR_EndLoadingPlaque();
        }
        _ => { }
    };
}
/*
==============
CL_ParseDirector

spectator message (director)
sended from game.dll
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseDirector(mut msg: *mut sizebuf_t) {
    let mut iSize: libc::c_int = MSG_ReadByte(msg);
    let mut pbuf: [byte; 256] = [0; 256];
    // parse user message into buffer
    MSG_ReadBytes(msg, pbuf.as_mut_ptr() as *mut libc::c_void, iSize);
    clgame.dllFuncs.pfnDirectorMessage.expect("non-null function pointer")(iSize,
                                                                           pbuf.as_mut_ptr()
                                                                               as
                                                                               *mut libc::c_void);
}
/*
==============
CL_ParseScreenShake

Set screen shake
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseScreenShake(mut msg: *mut sizebuf_t) {
    clgame.shake.amplitude =
        MSG_ReadShort(msg) as word as libc::c_float *
            (1.0f32 /
                 ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_float);
    clgame.shake.duration =
        MSG_ReadShort(msg) as word as libc::c_float *
            (1.0f32 /
                 ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_float);
    clgame.shake.frequency =
        MSG_ReadShort(msg) as word as libc::c_float *
            (1.0f32 /
                 ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_float);
    clgame.shake.time =
        (cl.time +
             (if clgame.shake.duration > 0.01f32 {
                  clgame.shake.duration
              } else { 0.01f32 }) as libc::c_double) as libc::c_float;
    clgame.shake.next_shake = 0.0f32;
    // apply immediately
}
/*
==============
CL_ParseScreenFade

Set screen fade
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseScreenFade(mut msg: *mut sizebuf_t) {
    let mut duration: libc::c_float = 0.;
    let mut holdTime: libc::c_float = 0.;
    let mut sf: *mut screenfade_t = &mut clgame.fade;
    let mut flScale: libc::c_float = 0.;
    duration = MSG_ReadWord(msg) as libc::c_float;
    holdTime = MSG_ReadWord(msg) as libc::c_float;
    (*sf).fadeFlags = MSG_ReadShort(msg);
    flScale =
        if (*sf).fadeFlags & 0x8 as libc::c_int != 0 {
            (1.0f32) / 256.0f32
        } else { (1.0f32) / 4096.0f32 };
    (*sf).fader = MSG_ReadByte(msg) as byte;
    (*sf).fadeg = MSG_ReadByte(msg) as byte;
    (*sf).fadeb = MSG_ReadByte(msg) as byte;
    (*sf).fadealpha = MSG_ReadByte(msg) as byte;
    (*sf).fadeSpeed = 0.0f32;
    (*sf).fadeEnd = duration * flScale;
    (*sf).fadeReset = holdTime * flScale;
    // calc fade speed
    if duration > 0 as libc::c_int as libc::c_float {
        if (*sf).fadeFlags & 0x1 as libc::c_int != 0 {
            if (*sf).fadeEnd != 0. {
                (*sf).fadeSpeed =
                    -((*sf).fadealpha as libc::c_float) / (*sf).fadeEnd
            }
            (*sf).fadeEnd =
                ((*sf).fadeEnd as libc::c_double + cl.time) as libc::c_float;
            (*sf).fadeReset += (*sf).fadeEnd
        } else {
            if (*sf).fadeEnd != 0. {
                (*sf).fadeSpeed =
                    (*sf).fadealpha as libc::c_float / (*sf).fadeEnd
            }
            (*sf).fadeReset =
                ((*sf).fadeReset as libc::c_double + cl.time) as
                    libc::c_float;
            (*sf).fadeEnd += (*sf).fadeReset
        }
    };
}
/*
==============
CL_ParseCvarValue

Find the client cvar value
and sent it back to the server
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseCvarValue(mut msg: *mut sizebuf_t,
                                           ext: qboolean) {
    let mut cvarName: *const libc::c_char = 0 as *const libc::c_char;
    let mut response: *const libc::c_char = 0 as *const libc::c_char;
    let mut cvar: *mut convar_t = 0 as *mut convar_t;
    let mut requestID: libc::c_int = 0;
    if ext as u64 != 0 { requestID = MSG_ReadLong(msg) }
    cvarName = MSG_ReadStringExt(msg, false_0);
    cvar = Cvar_FindVarExt(cvarName, 0 as libc::c_int);
    if !cvar.is_null() {
        if (*cvar).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            response =
                b"CVAR is privileged\x00" as *const u8 as *const libc::c_char
        } else if (*cvar).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
         {
            response =
                b"CVAR is server-only\x00" as *const u8 as *const libc::c_char
        } else if (*cvar).flags & (1 as libc::c_int) << 5 as libc::c_int != 0
         {
            response =
                b"CVAR is protected\x00" as *const u8 as *const libc::c_char
        } else { response = (*cvar).string }
    } else {
        response = b"Bad CVAR request\x00" as *const u8 as *const libc::c_char
    }
    if ext as u64 != 0 {
        MSG_WriteCmdExt(&mut cls.netchan.message, 10 as libc::c_int,
                        NS_CLIENT, 0 as *const libc::c_char);
        MSG_WriteLong(&mut cls.netchan.message, requestID);
        MSG_WriteString(&mut cls.netchan.message, cvarName);
    } else {
        MSG_WriteCmdExt(&mut cls.netchan.message, 9 as libc::c_int, NS_CLIENT,
                        0 as *const libc::c_char);
    }
    MSG_WriteString(&mut cls.netchan.message, response);
}
/*
==============
CL_ParseExec

Exec map/class specific configs
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseExec(mut msg: *mut sizebuf_t) {
    let mut is_class: qboolean = false_0;
    let mut class_idx: libc::c_int = 0;
    let mut mapname: string = [0; 256];
    let mut class_cfgs: [*const libc::c_char; 12] =
        [b"\x00" as *const u8 as *const libc::c_char,
         b"exec scout.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec sniper.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec soldier.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec demoman.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec medic.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec hwguy.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec pyro.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec spy.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"exec engineer.cfg\n\x00" as *const u8 as *const libc::c_char,
         b"\x00" as *const u8 as *const libc::c_char,
         b"exec civilian.cfg\n\x00" as *const u8 as *const libc::c_char];
    is_class = MSG_ReadByte(msg) as qboolean;
    if is_class as u64 != 0 {
        class_idx = MSG_ReadByte(msg);
        if class_idx >= 0 as libc::c_int && class_idx <= 11 as libc::c_int &&
               Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                          b"tfc\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
            Cbuf_AddText(class_cfgs[class_idx as usize]);
        }
    } else if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                         b"tfc\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
        Cbuf_AddText(b"exec mapdefault.cfg\n\x00" as *const u8 as
                         *const libc::c_char);
        COM_FileBase(clgame.mapname.as_mut_ptr(), mapname.as_mut_ptr());
        if if mapname.as_mut_ptr().is_null() || *mapname.as_mut_ptr() == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } != 0 {
            Cbuf_AddText(va(b"exec %s.cfg\n\x00" as *const u8 as
                                *const libc::c_char, mapname.as_mut_ptr()));
        }
    };
}
/*
==============
CL_DispatchUserMessage

Dispatch user message by engine request
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DispatchUserMessage(mut pszName:
                                                    *const libc::c_char,
                                                mut iSize: libc::c_int,
                                                mut pbuf: *mut libc::c_void)
 -> qboolean {
    let mut i: libc::c_int = 0;
    if if pszName.is_null() || *pszName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    i = 0 as libc::c_int;
    while i < 197 as libc::c_int {
        // search for user message
        if Q_strncmp(clgame.msg[i as usize].name.as_mut_ptr(), pszName,
                     99999 as libc::c_int) == 0 {
            break ;
        }
        i += 1
    }
    if i == 197 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 UserMsg: bad message %s\n\x00" as *const u8
                        as *const libc::c_char, pszName);
        return false_0
    }
    if clgame.msg[i as usize].func.is_some() {
        clgame.msg[i as
                       usize].func.expect("non-null function pointer")(pszName,
                                                                       iSize,
                                                                       pbuf);
    } else {
        Con_DPrintf(b"^1Error:^7 UserMsg: No pfn %s %d\n\x00" as *const u8 as
                        *const libc::c_char,
                    clgame.msg[i as usize].name.as_mut_ptr(),
                    clgame.msg[i as usize].number);
        clgame.msg[i as usize].func =
            Some(CL_UserMsgStub as
                     unsafe extern "C" fn(_: *const libc::c_char,
                                          _: libc::c_int,
                                          _: *mut libc::c_void)
                         -> libc::c_int)
        // throw warning only once
    }
    return true_0;
}
/*
==============
CL_ParseUserMessage

handles all user messages
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseUserMessage(mut msg: *mut sizebuf_t,
                                             mut svc_num: libc::c_int) {
    let mut pbuf: [byte; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut iSize: libc::c_int = 0;
    // NOTE: any user message is really parse at engine, not in client.dll
    if svc_num <= 59 as libc::c_int ||
           svc_num > 197 as libc::c_int + 59 as libc::c_int {
        // out or range
        Host_Error(b"CL_ParseUserMessage: illegible server message %d\n\x00"
                       as *const u8 as *const libc::c_char, svc_num);
        return
    }
    i = 0 as libc::c_int;
    while i < 197 as libc::c_int {
        // search for user message
        if clgame.msg[i as usize].number == svc_num { break ; }
        i += 1
    }
    if i == 197 as libc::c_int {
        // probably unregistered
        Host_Error(b"CL_ParseUserMessage: illegible server message %d\n\x00"
                       as *const u8 as *const libc::c_char, svc_num);
    }
    // NOTE: some user messages handled into engine
    if Q_strncmp(clgame.msg[i as usize].name.as_mut_ptr(),
                 b"ScreenShake\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        CL_ParseScreenShake(msg);
        return
    } else {
        if Q_strncmp(clgame.msg[i as usize].name.as_mut_ptr(),
                     b"ScreenFade\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            CL_ParseScreenFade(msg);
            return
        }
    }
    iSize = clgame.msg[i as usize].size;
    // message with variable sizes receive an actual size as first byte
    if iSize == -(1 as libc::c_int) {
        if cls.legacymode as u64 != 0 {
            iSize = MSG_ReadByte(msg)
        } else { iSize = MSG_ReadWord(msg) }
    }
    if iSize >= 2048 as libc::c_int {
        Con_Printf(b"WTF??? %d %d\n\x00" as *const u8 as *const libc::c_char,
                   i, svc_num);
        return
    }
    // parse user message into buffer
    MSG_ReadBytes(msg, pbuf.as_mut_ptr() as *mut libc::c_void, iSize);
    if (*cl_trace_messages).value != 0. {
        Con_Reportf(b"^3USERMSG %s SIZE %i SVC_NUM %i\n\x00" as *const u8 as
                        *const libc::c_char,
                    clgame.msg[i as usize].name.as_mut_ptr(), iSize,
                    clgame.msg[i as usize].number);
    }
    if clgame.msg[i as usize].func.is_some() {
        clgame.msg[i as
                       usize].func.expect("non-null function pointer")(clgame.msg[i
                                                                                      as
                                                                                      usize].name.as_mut_ptr(),
                                                                       iSize,
                                                                       pbuf.as_mut_ptr()
                                                                           as
                                                                           *mut libc::c_void);
        // run final credits for Half-Life because hl1 doesn't have call END_SECTION
        if Q_strnicmp(clgame.msg[i as usize].name.as_mut_ptr(),
                      b"HudText\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 &&
               Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                          b"valve\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
            // it's a end, so we should run credits
            if Q_strncmp(pbuf.as_mut_ptr() as *mut libc::c_char,
                         b"END3\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
                Host_Credits();
            }
        }
    } else {
        Con_DPrintf(b"^1Error:^7 UserMsg: No pfn %s %d\n\x00" as *const u8 as
                        *const libc::c_char,
                    clgame.msg[i as usize].name.as_mut_ptr(),
                    clgame.msg[i as usize].number);
        clgame.msg[i as usize].func =
            Some(CL_UserMsgStub as
                     unsafe extern "C" fn(_: *const libc::c_char,
                                          _: libc::c_int,
                                          _: *mut libc::c_void)
                         -> libc::c_int)
        // throw warning only once
    };
}
/*
=====================================================================

ACTION MESSAGES

=====================================================================
*/
/*
=====================
CL_ParseServerMessage

dispatch messages
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseServerMessage(mut msg: *mut sizebuf_t,
                                               mut normal_message: qboolean) {
    let mut bufStart: size_t = 0; // updates each frame
    let mut playerbytes: size_t = 0; // begin parsing
    let mut cmd: libc::c_int = 0;
    let mut param1: libc::c_int = 0;
    let mut param2: libc::c_int = 0;
    let mut old_background: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    cls.starting_count = MSG_GetNumBytesWritten(msg);
    CL_Parse_Debug(true_0);
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
        // record command for debugging spew on parse problem
        CL_Parse_RecordCommand(cmd, bufStart as libc::c_int);
        // other commands
        match cmd {
            0 => {
                Host_Error(b"svc_bad\n\x00" as *const u8 as
                               *const libc::c_char);
            }
            1 => { }
            2 => { CL_Drop(); Host_AbortCurrentFrame(); }
            3 => {
                CL_ParseEvent(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.event =
                    (cl.frames[cl.parsecountmod as usize].graphdata.event as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            4 => {
                old_background = cl.background as libc::c_int;
                if MSG_ReadOneBit(msg) != 0 {
                    cls.changelevel = true_0;
                    S_StopAllSounds(true_0);
                    Con_Printf(b"Server changing, reconnecting\n\x00" as
                                   *const u8 as *const libc::c_char);
                    if cls.demoplayback != 0 {
                        SCR_BeginLoadingPlaque(cl.background);
                        cls.changedemo = true_0
                    }
                    CL_ClearState();
                    CL_InitEdicts();
                    // re-arrange edicts
                } else {
                    Con_Printf(b"Server disconnected, reconnecting\n\x00" as
                                   *const u8 as *const libc::c_char);
                }
                if cls.demoplayback != 0 {
                    cl.background =
                        if cls.demonum != -(1 as libc::c_int) {
                            true_0 as libc::c_int
                        } else { false_0 as libc::c_int } as qboolean;
                    cls.state = ca_connected
                } else {
                    // g-cont. local client skip the challenge
                    if SV_Active() as u64 != 0 {
                        cls.state = ca_disconnected
                    } else { cls.state = ca_connecting }
                    cl.background = old_background as qboolean;
                    cls.connect_time =
                        -(99999 as libc::c_int) as libc::c_double
                }
            }
            5 => { CL_ParseViewEntity(msg); }
            6 => {
                CL_ParseSoundPacket(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.sound =
                    (cl.frames[cl.parsecountmod as usize].graphdata.sound as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            7 => { CL_ParseServerTime(msg); }
            8 => {
                Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                           MSG_ReadStringExt(msg, false_0));
            }
            9 => {
                s = MSG_ReadStringExt(msg, false_0);
                // disable Cry Of Fear antisave protection
                if !(Q_strnicmp(s,
                                b"disconnect\x00" as *const u8 as
                                    *const libc::c_char, 10 as libc::c_int) ==
                         0 && cls.signon != 2 as libc::c_int) {
                    Cbuf_AddFilteredText(s); // too early
                }
            }
            10 => {
                CL_ParseSetAngle(msg); // make sure any stuffed commands are done
            }
            11 => {
                Cbuf_Execute(); // tracknum
                CL_ParseServerData(msg); // loopnum
            }
            12 => {
                CL_ParseLightStyle(msg); // iAnim
            }
            13 => {
                CL_UpdateUserinfo(msg); // body
            }
            14 => {
                Delta_ParseTableField(msg); // done
            }
            15 => {
                CL_ParseClientData(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.client =
                    (cl.frames[cl.parsecountmod as usize].graphdata.client as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            16 => { CL_ParseResource(msg); }
            17 => { CL_UpdateUserPings(msg); }
            18 => { CL_ParseParticles(msg); }
            19 => {
                CL_ParseRestoreSoundPacket(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.sound =
                    (cl.frames[cl.parsecountmod as usize].graphdata.sound as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            20 => { CL_ParseStaticEntity(msg); }
            21 => {
                CL_ParseReliableEvent(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.event =
                    (cl.frames[cl.parsecountmod as usize].graphdata.event as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            22 => { CL_ParseBaseline(msg); }
            23 => {
                CL_ParseTempEntity(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.tentities =
                    (cl.frames[cl.parsecountmod as usize].graphdata.tentities
                         as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            24 => {
                cl.paused =
                    (MSG_ReadOneBit(msg) != 0 as libc::c_int) as libc::c_int
                        as qboolean
            }
            25 => { CL_ParseSignon(msg); }
            26 => {
                CL_CenterPrint(MSG_ReadStringExt(msg, false_0), 0.25f32);
            }
            30 => { cl.intermission = 1 as libc::c_int }
            31 => { CL_ParseFinaleCutscene(msg, 2 as libc::c_int); }
            32 => {
                param1 = MSG_ReadByte(msg);
                param1 =
                    if param1 >= 1 as libc::c_int {
                        if param1 < 32 as libc::c_int {
                            param1
                        } else { 32 as libc::c_int }
                    } else { 1 as libc::c_int };
                param2 = MSG_ReadByte(msg);
                param2 =
                    if param2 >= 1 as libc::c_int {
                        if param2 < 32 as libc::c_int {
                            param2
                        } else { 32 as libc::c_int }
                    } else { 1 as libc::c_int };
                S_StartBackgroundTrack(clgame.cdtracks[(param1 -
                                                            1 as libc::c_int)
                                                           as
                                                           usize].as_mut_ptr(),
                                       clgame.cdtracks[(param2 -
                                                            1 as libc::c_int)
                                                           as
                                                           usize].as_mut_ptr(),
                                       0 as libc::c_int, false_0);
            }
            33 => { CL_ParseRestore(msg); }
            34 => { CL_ParseFinaleCutscene(msg, 3 as libc::c_int); }
            35 => {
                param1 = MSG_ReadByte(msg);
                param2 = MSG_ReadByte(msg);
                CL_WeaponAnim(param1, param2);
            }
            36 => { CL_ParseStaticDecal(msg); }
            37 => {
                param1 = MSG_ReadShort(msg);
                Cvar_SetValue(b"room_type\x00" as *const u8 as
                                  *const libc::c_char,
                              param1 as libc::c_float);
            }
            38 => { CL_ParseAddAngle(msg); }
            39 => { CL_RegisterUserMessage(msg); }
            40 => {
                playerbytes = CL_ParsePacketEntities(msg, false_0) as size_t;
                cl.frames[cl.parsecountmod as usize].graphdata.players =
                    (cl.frames[cl.parsecountmod as usize].graphdata.players as
                         libc::c_ulong).wrapping_add(playerbytes) as word as
                        word;
                cl.frames[cl.parsecountmod as usize].graphdata.entities =
                    (cl.frames[cl.parsecountmod as usize].graphdata.entities
                         as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart).wrapping_sub(playerbytes))
                        as word as word
            }
            41 => {
                playerbytes = CL_ParsePacketEntities(msg, true_0) as size_t;
                cl.frames[cl.parsecountmod as usize].graphdata.players =
                    (cl.frames[cl.parsecountmod as usize].graphdata.players as
                         libc::c_ulong).wrapping_add(playerbytes) as word as
                        word;
                cl.frames[cl.parsecountmod as usize].graphdata.entities =
                    (cl.frames[cl.parsecountmod as usize].graphdata.entities
                         as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart).wrapping_sub(playerbytes))
                        as word as word
            }
            42 => {
                cl.frames[(cls.netchan.incoming_sequence &
                               (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                                   libc::c_uint) as usize].choked = true_0;
                cl.frames[(cls.netchan.incoming_sequence &
                               (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                                   libc::c_uint) as usize].receivedtime =
                    -2.0f64
            }
            43 => { CL_ParseResourceList(msg); }
            44 => { CL_ParseMovevars(msg); }
            45 => { CL_ParseResourceRequest(msg); }
            46 => { CL_ParseCustomization(msg); }
            47 => { CL_ParseCrosshairAngle(msg); }
            48 => { CL_ParseSoundFade(msg); }
            49 => { CL_ParseFileTransferFailed(msg); }
            50 => { CL_ParseHLTV(msg); }
            51 => { CL_ParseDirector(msg); }
            52 => { CL_ParseVoiceInit(msg); }
            53 => { CL_ParseVoiceData(msg); }
            56 => { CL_ParseResLocation(msg); }
            57 => { CL_ParseCvarValue(msg, false_0); }
            58 => { CL_ParseCvarValue(msg, true_0); }
            59 => { CL_ParseExec(msg); }
            _ => {
                CL_ParseUserMessage(msg, cmd);
                cl.frames[cl.parsecountmod as usize].graphdata.usr =
                    (cl.frames[cl.parsecountmod as usize].graphdata.usr as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
        }
    }
    cl.frames[cl.parsecountmod as usize].graphdata.msgbytes =
        (cl.frames[cl.parsecountmod as usize].graphdata.msgbytes as
             libc::c_int + (MSG_GetNumBytesWritten(msg) - cls.starting_count))
            as word;
    CL_Parse_Debug(false_0);
    // we don't know if it is ok to save a demo message until
	// after we have parsed the frame
    if cls.demoplayback == 0 {
        if cls.demorecording as libc::c_uint != 0 &&
               cls.demowaiting as u64 == 0 {
            CL_WriteDemoMessage(false_0, cls.starting_count, msg);
        } else if cls.state as libc::c_uint !=
                      ca_active as libc::c_int as libc::c_uint {
            CL_WriteDemoMessage(true_0, cls.starting_count, msg);
        }
    };
}
/*
==================
CL_ParseBaseline
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyParseBaseline(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0; // finalize client delta's
    let mut newnum: libc::c_int = 0;
    let mut nullstate: entity_state_t =
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
                       vuser4: [0.; 3],};
    let mut player: qboolean = false_0;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    Delta_InitClient();
    memset(&mut nullstate as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    newnum = MSG_ReadWord(msg);
    player = CL_IsPlayerIndex(newnum);
    if newnum >= clgame.maxEntities {
        Host_Error(b"CL_AllocEdict: no free edicts\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    ent = CL_EDICT_NUM(newnum);
    memset(&mut (*ent).prevstate as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    (*ent).index = newnum;
    MSG_ReadDeltaEntity(msg, &mut (*ent).prevstate, &mut (*ent).baseline,
                        newnum, player as libc::c_int,
                        1.0f32 as libc::c_double);
}
/*
==================
CL_ParseServerData
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseLegacyServerData(mut msg: *mut sizebuf_t) {
    let mut gamefolder: string = [0; 256]; // server is changed
    let mut background: qboolean = false_0;
    let mut i: libc::c_int = 0;
    Con_Reportf(b"Legacy serverdata packet received.\n\x00" as *const u8 as
                    *const libc::c_char);
    cls.timestart = Sys_DoubleTime();
    cls.demowaiting = false_0;
    //clgame.load_sequence++;	// now all hud sprites are invalid
    // wipe the client_t struct
    if cls.changelevel as u64 == 0 && cls.changedemo as u64 == 0 {
        CL_ClearState();
    }
    cls.state = ca_connected;
    // parse protocol version number
    i = MSG_ReadLong(msg);
    //cls.serverProtocol = i;
    if i != 48 as libc::c_int {
        Host_Error(b"Server uses invalid protocol (%i should be %i)\n\x00" as
                       *const u8 as *const libc::c_char, i,
                   49 as libc::c_int);
    }
    cl.servercount = MSG_ReadLong(msg);
    cl.checksum = MSG_ReadLong(msg) as uint;
    cl.playernum = MSG_ReadByte(msg);
    cl.maxclients = MSG_ReadByte(msg);
    clgame.maxEntities = MSG_ReadWord(msg);
    clgame.maxEntities =
        if clgame.maxEntities >= 30 as libc::c_int {
            if clgame.maxEntities < 4096 as libc::c_int {
                clgame.maxEntities
            } else { 4096 as libc::c_int }
        } else { 30 as libc::c_int };
    clgame.maxModels = 512 as libc::c_int;
    Q_strncpy(clgame.mapname.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    Q_strncpy(clgame.maptitle.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    background = MSG_ReadOneBit(msg) as qboolean;
    Q_strncpy(gamefolder.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    host.features = MSG_ReadLong(msg) as uint;
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
    // set the background state
    if cls.demoplayback != 0 && cls.demonum != -(1 as libc::c_int) {
        // re-init mouse
        host.mouse_visible = false_0;
        cl.background = true_0
    } else { cl.background = background }
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
    if cls.changelevel as u64 == 0 {
        // continue playing if we are changing level
        S_StopBackgroundTrack();
    }
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
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        COM_ClearCustomizationList(&mut (*cl.players.as_mut_ptr().offset(i as
                                                                             isize)).customdata,
                                   true_0);
        i += 1
    }
    CL_CreateCustomizationList();
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
}
/*
==================
CL_ParseStaticEntity

static client entity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyParseStaticEntity(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0;
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
                       vuser4: [0.; 3],};
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    memset(&mut state as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    state.modelindex = MSG_ReadShort(msg);
    state.sequence = MSG_ReadByte(msg);
    state.frame = MSG_ReadByte(msg) as libc::c_float;
    state.colormap = MSG_ReadWord(msg);
    state.skin = MSG_ReadByte(msg) as libc::c_short;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        state.origin[i as usize] = MSG_ReadCoord(msg);
        state.angles[i as usize] = MSG_ReadBitAngle(msg, 16 as libc::c_int);
        i += 1
    }
    state.rendermode = MSG_ReadByte(msg);
    if state.rendermode != kRenderNormal as libc::c_int {
        state.renderamt = MSG_ReadByte(msg);
        state.rendercolor.r = MSG_ReadByte(msg) as byte;
        state.rendercolor.g = MSG_ReadByte(msg) as byte;
        state.rendercolor.b = MSG_ReadByte(msg) as byte;
        state.renderfx = MSG_ReadByte(msg)
    }
    i = clgame.numStatics;
    if i >= 3096 as libc::c_int {
        Con_Printf(b"^1Error:^7 MAX_STATIC_ENTITIES limit exceeded!\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    ent = &mut *clgame.static_entities.offset(i as isize) as *mut cl_entity_t;
    clgame.numStatics += 1;
    // all states are same
    (*ent).prevstate = state; // static entities doesn't has the numbers
    (*ent).curstate = (*ent).prevstate;
    (*ent).baseline = (*ent).curstate;
    (*ent).index = 0 as libc::c_int;
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
    if (*ent).curstate.rendermode == kRenderNormal as libc::c_int &&
           !(*ent).model.is_null() {
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
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyParseSoundPacket(mut msg: *mut sizebuf_t,
                                                   mut is_ambient: qboolean) {
    let mut pos: vec3_t = [0.; 3];
    let mut chan: libc::c_int = 0;
    let mut sound: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut attn: libc::c_float = 0.;
    let mut flags: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut entnum: libc::c_int = 0;
    let mut handle: sound_t = 0 as libc::c_int;
    flags = MSG_ReadWord(msg);
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        sound = MSG_ReadWord(msg);
        flags &= !((1 as libc::c_int) << 2 as libc::c_int)
    } else { sound = MSG_ReadByte(msg) }
    chan = MSG_ReadByte(msg);
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        volume = MSG_ReadByte(msg) as libc::c_float / 255.0f32
    } else { volume = 1.0f64 as libc::c_float }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        attn = MSG_ReadByte(msg) as libc::c_float / 64.0f32
    } else { attn = 0 as libc::c_int as libc::c_float }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        pitch = MSG_ReadByte(msg)
    } else { pitch = 100 as libc::c_int }
    // entity reletive
    entnum = MSG_ReadWord(msg);
    // positioned in space
    MSG_ReadVec3Coord(msg, pos.as_mut_ptr()); // see precached sound
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        let mut sentenceName: [libc::c_char; 32] = [0; 32];
        //if( FBitSet( flags, SND_SEQUENCE ))
			//Q_snprintf( sentenceName, sizeof( sentenceName ), "!#%i", sound + MAX_SOUNDS );
		//else
        Q_snprintf(sentenceName.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 32]>() as
                       libc::c_ulong,
                   b"!%i\x00" as *const u8 as *const libc::c_char,
                   sound); // too early
        handle = S_RegisterSound(sentenceName.as_mut_ptr())
    } else { handle = cl.sound_index[sound as usize] as sound_t }
    if cl.audio_prepped as u64 == 0 { return }
    // g-cont. sound and ambient sound have only difference with channel
    if is_ambient as u64 != 0 {
        S_AmbientSound(pos.as_mut_ptr() as *const vec_t, entnum, handle,
                       volume, attn, pitch, flags);
    } else {
        S_StartSound(pos.as_mut_ptr() as *const vec_t, entnum, chan, handle,
                     volume, attn, pitch, flags);
    };
}
/*
================
CL_PrecacheSound

prceache sound from server
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyPrecacheSound(mut msg: *mut sizebuf_t) {
    let mut soundIndex: libc::c_int = 0;
    soundIndex = MSG_ReadUBitLong(msg, 11 as libc::c_int) as libc::c_int;
    if soundIndex < 0 as libc::c_int ||
           soundIndex >= (1 as libc::c_int) << 11 as libc::c_int {
        Host_Error(b"CL_PrecacheSound: bad soundindex %i\n\x00" as *const u8
                       as *const libc::c_char, soundIndex);
    }
    Q_strncpy(cl.sound_precache[soundIndex as usize].as_mut_ptr(),
              MSG_ReadStringExt(msg, false_0),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // when we loading map all resources is precached sequentially
	//if( !cl.audio_prepped ) return;
    cl.sound_index[soundIndex as usize] =
        S_RegisterSound(cl.sound_precache[soundIndex as usize].as_mut_ptr())
            as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyPrecacheModel(mut msg: *mut sizebuf_t) {
    let mut modelIndex: libc::c_int = 0;
    let mut model: string = [0; 256];
    modelIndex = MSG_ReadUBitLong(msg, 11 as libc::c_int) as libc::c_int;
    if modelIndex < 0 as libc::c_int || modelIndex >= 1024 as libc::c_int {
        Host_Error(b"CL_PrecacheModel: bad modelindex %i\n\x00" as *const u8
                       as *const libc::c_char, modelIndex);
    }
    Q_strncpy(model.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    //Q_strncpy( cl.model_precache[modelIndex], BF_ReadString( msg ), sizeof( cl.model_precache[0] ));
    // when we loading map all resources is precached sequentially
	//if( !cl.video_prepped ) return;
    if modelIndex == 1 as libc::c_int && cl.worldmodel.is_null() {
        CL_ClearWorld();
        cl.worldmodel = Mod_LoadWorld(model.as_mut_ptr(), true_0);
        cl.models[modelIndex as usize] = cl.worldmodel;
        return
    }
    //Mod_RegisterModel( cl.model_precache[modelIndex], modelIndex );
    cl.models[modelIndex as usize] =
        Mod_ForName(model.as_mut_ptr(), false_0, false_0);
    cl.nummodels =
        if cl.nummodels > modelIndex { cl.nummodels } else { modelIndex };
}
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyPrecacheEvent(mut msg: *mut sizebuf_t) {
    let mut eventIndex: libc::c_int = 0;
    eventIndex = MSG_ReadUBitLong(msg, 10 as libc::c_int) as libc::c_int;
    if eventIndex < 0 as libc::c_int ||
           eventIndex >= (1 as libc::c_int) << 10 as libc::c_int {
        Host_Error(b"CL_PrecacheEvent: bad eventindex %i\n\x00" as *const u8
                       as *const libc::c_char, eventIndex);
    }
    Q_strncpy(cl.event_precache[eventIndex as usize].as_mut_ptr(),
              MSG_ReadStringExt(msg, false_0),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // can be set now
    CL_SetEventIndex(cl.event_precache[eventIndex as usize].as_mut_ptr(),
                     eventIndex);
}
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyUpdateUserinfo(mut msg: *mut sizebuf_t) {
    let mut slot: libc::c_int = 0;
    let mut id: libc::c_int = 0 as libc::c_int;
    let mut active: qboolean = false_0;
    let mut player: *mut player_info_t = 0 as *mut player_info_t;
    slot = MSG_ReadUBitLong(msg, 5 as libc::c_int) as libc::c_int;
    if slot >= (1 as libc::c_int) << 5 as libc::c_int {
        Host_Error(b"CL_ParseServerMessage: svc_updateuserinfo >= MAX_CLIENTS\n\x00"
                       as *const u8 as *const libc::c_char);
    }
    //id = MSG_ReadLong( msg );	// unique user ID
    player =
        &mut *cl.players.as_mut_ptr().offset(slot as isize) as
            *mut player_info_t;
    active =
        if MSG_ReadOneBit(msg) != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    if active as u64 != 0 {
        Q_strncpy((*player).userinfo.as_mut_ptr(),
                  MSG_ReadStringExt(msg, false_0),
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
        Q_strncpy((*player).name.as_mut_ptr(),
                  Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                   b"name\x00" as *const u8 as
                                       *const libc::c_char),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong);
        Q_strncpy((*player).model.as_mut_ptr(),
                  Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                   b"model\x00" as *const u8 as
                                       *const libc::c_char),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        (*player).topcolor =
            Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                    b"topcolor\x00" as *const u8 as
                                        *const libc::c_char));
        (*player).bottomcolor =
            Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                    b"bottomcolor\x00" as *const u8 as
                                        *const libc::c_char));
        (*player).spectator =
            Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                    b"*hltv\x00" as *const u8 as
                                        *const libc::c_char));
        //MSG_ReadBytes( msg, player->hashedcdkey, sizeof( player->hashedcdkey ));
        if slot == cl.playernum {
            memcpy(&mut gameui.playerinfo as *mut player_info_t as
                       *mut libc::c_void, player as *const libc::c_void,
                   ::std::mem::size_of::<player_info_t>() as libc::c_ulong);
        }
    } else {
        memset(player as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<player_info_t>() as libc::c_ulong);
    };
}
/*
==============
CL_ParseResourceList

==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyParseResourceList(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0 as libc::c_int; // already exists
    static mut reslist: C2RustUnnamed_3 =
        C2RustUnnamed_3{rescount: 0,
                        restype: [0; 2048],
                        resnames: [[0; 64]; 2048],};
    memset(&mut reslist as *mut C2RustUnnamed_3 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong);
    reslist.rescount = MSG_ReadWord(msg) - 1 as libc::c_int;
    if reslist.rescount > 2048 as libc::c_int {
        Host_Error(b"MAX_RESOURCES reached\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < reslist.rescount {
        reslist.restype[i as usize] = MSG_ReadWord(msg);
        Q_strncpy(reslist.resnames[i as usize].as_mut_ptr(),
                  MSG_ReadStringExt(msg, false_0),
                  64 as libc::c_int as size_t);
        i += 1
    }
    if CL_IsPlaybackDemo() as u64 != 0 { return }
    host.downloadcount = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < reslist.rescount {
        let mut path: *const libc::c_char = 0 as *const libc::c_char;
        if reslist.restype[i as usize] == t_sound as libc::c_int {
            path =
                va(b"sound/%s\x00" as *const u8 as *const libc::c_char,
                   reslist.resnames[i as usize].as_mut_ptr())
        } else { path = reslist.resnames[i as usize].as_mut_ptr() }
        if !(FS_FileExists(path, false_0 as libc::c_int) != 0) {
            host.downloadcount += 1;
            HTTP_AddDownload(path, -(1 as libc::c_int), true_0);
        }
        i += 1
    }
    if host.downloadcount == 0 {
        MSG_WriteByte(&mut cls.netchan.message, 3 as libc::c_int);
        MSG_WriteString(&mut cls.netchan.message,
                        b"continueloading\x00" as *const u8 as
                            *const libc::c_char);
    };
}
/*
=====================
CL_ParseLegacyServerMessage

dispatch messages
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseLegacyServerMessage(mut msg: *mut sizebuf_t,
                                                     mut normal_message:
                                                         qboolean) {
    let mut bufStart: size_t = 0; // updates each frame
    let mut playerbytes: size_t = 0; // begin parsing
    let mut cmd: libc::c_int = 0;
    let mut param1: libc::c_int = 0;
    let mut param2: libc::c_int = 0;
    let mut old_background: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    cls.starting_count = MSG_GetNumBytesWritten(msg);
    CL_Parse_Debug(true_0);
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
        // record command for debugging spew on parse problem
        CL_Parse_RecordCommand(cmd, bufStart as libc::c_int);
        // other commands
        match cmd {
            0 => {
                Host_Error(b"svc_bad\n\x00" as *const u8 as
                               *const libc::c_char);
            }
            1 => { }
            2 => { CL_Drop(); Host_AbortCurrentFrame(); }
            27 => {
                CL_ParseEvent(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.event =
                    (cl.frames[cl.parsecountmod as usize].graphdata.event as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            3 => {
                old_background = cl.background as libc::c_int;
                if MSG_ReadOneBit(msg) != 0 {
                    cls.changelevel = true_0;
                    S_StopAllSounds(true_0);
                    Con_Printf(b"Server changing, reconnecting\n\x00" as
                                   *const u8 as *const libc::c_char);
                    if cls.demoplayback != 0 {
                        SCR_BeginLoadingPlaque(cl.background);
                        cls.changedemo = true_0
                    }
                    CL_ClearState();
                    CL_InitEdicts();
                    // re-arrange edicts
                } else {
                    Con_Printf(b"Server disconnected, reconnecting\n\x00" as
                                   *const u8 as *const libc::c_char);
                }
                if cls.demoplayback != 0 {
                    cl.background =
                        if cls.demonum != -(1 as libc::c_int) {
                            true_0 as libc::c_int
                        } else { false_0 as libc::c_int } as qboolean;
                    cls.state = ca_connected
                } else {
                    // g-cont. local client skip the challenge
                    if SV_Active() as u64 != 0 {
                        cls.state = ca_disconnected
                    } else { cls.state = ca_connecting }
                    cl.background = old_background as qboolean;
                    cls.connect_time =
                        -(99999 as libc::c_int) as libc::c_double
                }
            }
            5 => { CL_ParseViewEntity(msg); }
            6 => {
                CL_LegacyParseSoundPacket(msg, false_0);
                cl.frames[cl.parsecountmod as usize].graphdata.sound =
                    (cl.frames[cl.parsecountmod as usize].graphdata.sound as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            29 => {
                CL_LegacyParseSoundPacket(msg, true_0);
                cl.frames[cl.parsecountmod as usize].graphdata.sound =
                    (cl.frames[cl.parsecountmod as usize].graphdata.sound as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            7 => { CL_ParseServerTime(msg); }
            8 => {
                Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                           MSG_ReadStringExt(msg, false_0));
            }
            9 => {
                s = MSG_ReadStringExt(msg, false_0);
                // disable Cry Of Fear antisave protection
                if !(Q_strnicmp(s,
                                b"disconnect\x00" as *const u8 as
                                    *const libc::c_char, 10 as libc::c_int) ==
                         0 && cls.signon != 2 as libc::c_int) {
                    Con_Reportf(b"Stufftext: %s\x00" as *const u8 as
                                    *const libc::c_char, s); // too early
                    Cbuf_AddFilteredText(s); // make sure any stuffed commands are done
                }
            }
            10 => {
                CL_ParseSetAngle(msg); // tracknum
            }
            11 => {
                Cbuf_Execute(); // loopnum
                CL_ParseLegacyServerData(msg);
            }
            12 => { CL_ParseLightStyle(msg); }
            13 => { CL_LegacyUpdateUserinfo(msg); }
            14 => { Delta_ParseTableField(msg); }
            15 => {
                CL_ParseClientData(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.client =
                    (cl.frames[cl.parsecountmod as usize].graphdata.client as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            16 => { CL_ParseResource(msg); }
            17 => { CL_UpdateUserPings(msg); }
            18 => { CL_ParseParticles(msg); }
            19 => {
                CL_ParseRestoreSoundPacket(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.sound =
                    (cl.frames[cl.parsecountmod as usize].graphdata.sound as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            20 => { CL_ParseStaticEntity(msg); }
            21 => {
                CL_ParseReliableEvent(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.event =
                    (cl.frames[cl.parsecountmod as usize].graphdata.event as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            22 => { CL_LegacyParseBaseline(msg); }
            23 => {
                CL_ParseTempEntity(msg);
                cl.frames[cl.parsecountmod as usize].graphdata.tentities =
                    (cl.frames[cl.parsecountmod as usize].graphdata.tentities
                         as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
            24 => {
                cl.paused =
                    (MSG_ReadOneBit(msg) != 0 as libc::c_int) as libc::c_int
                        as qboolean
            }
            25 => { CL_ParseSignon(msg); }
            26 => {
                CL_CenterPrint(MSG_ReadStringExt(msg, false_0), 0.25f32);
            }
            30 => { cl.intermission = 1 as libc::c_int }
            31 => { CL_LegacyPrecacheModel(msg); }
            28 => { CL_LegacyPrecacheSound(msg); }
            32 => {
                param1 = MSG_ReadByte(msg);
                param1 =
                    if param1 >= 1 as libc::c_int {
                        if param1 < 32 as libc::c_int {
                            param1
                        } else { 32 as libc::c_int }
                    } else { 1 as libc::c_int };
                param2 = MSG_ReadByte(msg);
                param2 =
                    if param2 >= 1 as libc::c_int {
                        if param2 < 32 as libc::c_int {
                            param2
                        } else { 32 as libc::c_int }
                    } else { 1 as libc::c_int };
                S_StartBackgroundTrack(clgame.cdtracks[(param1 -
                                                            1 as libc::c_int)
                                                           as
                                                           usize].as_mut_ptr(),
                                       clgame.cdtracks[(param2 -
                                                            1 as libc::c_int)
                                                           as
                                                           usize].as_mut_ptr(),
                                       0 as libc::c_int, false_0);
            }
            33 => { CL_ParseRestore(msg); }
            34 => {
                //CL_ParseFinaleCutscene( msg, 3 );
                CL_LegacyPrecacheEvent(msg); // iAnim
            }
            35 => {
                param1 = MSG_ReadByte(msg); // body
                param2 = MSG_ReadByte(msg); // done
                CL_WeaponAnim(param1, param2);
            }
            36 => { CL_ParseStaticDecal(msg); }
            37 => {
                param1 = MSG_ReadShort(msg);
                Cvar_SetValue(b"room_type\x00" as *const u8 as
                                  *const libc::c_char,
                              param1 as libc::c_float);
            }
            38 => { CL_ParseAddAngle(msg); }
            39 => { CL_RegisterUserMessage(msg); }
            40 => {
                playerbytes = CL_ParsePacketEntities(msg, false_0) as size_t;
                cl.frames[cl.parsecountmod as usize].graphdata.players =
                    (cl.frames[cl.parsecountmod as usize].graphdata.players as
                         libc::c_ulong).wrapping_add(playerbytes) as word as
                        word;
                cl.frames[cl.parsecountmod as usize].graphdata.entities =
                    (cl.frames[cl.parsecountmod as usize].graphdata.entities
                         as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart).wrapping_sub(playerbytes))
                        as word as word
            }
            41 => {
                playerbytes = CL_ParsePacketEntities(msg, true_0) as size_t;
                cl.frames[cl.parsecountmod as usize].graphdata.players =
                    (cl.frames[cl.parsecountmod as usize].graphdata.players as
                         libc::c_ulong).wrapping_add(playerbytes) as word as
                        word;
                cl.frames[cl.parsecountmod as usize].graphdata.entities =
                    (cl.frames[cl.parsecountmod as usize].graphdata.entities
                         as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart).wrapping_sub(playerbytes))
                        as word as word
            }
            42 => {
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                i = MSG_ReadByte(msg);
                j =
                    cls.netchan.incoming_acknowledged.wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                        as libc::c_int;
                while i > 0 as libc::c_int &&
                          j as libc::c_uint >
                              cls.netchan.outgoing_sequence.wrapping_sub(CL_UPDATE_BACKUP
                                                                             as
                                                                             libc::c_uint)
                      {
                    if cl.frames[(j & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                                     usize].receivedtime != -3.0f64 {
                        cl.frames[(j & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                                      usize].choked = true_0;
                        cl.frames[(j & CL_UPDATE_BACKUP - 1 as libc::c_int) as
                                      usize].receivedtime = -2.0f64;
                        i -= 1
                    }
                    j -= 1
                }
            }
            43 => { CL_LegacyParseResourceList(msg); }
            44 => { CL_ParseMovevars(msg); }
            45 => { CL_ParseResourceRequest(msg); }
            46 => { CL_ParseCustomization(msg); }
            47 => { CL_ParseCrosshairAngle(msg); }
            48 => { CL_ParseSoundFade(msg); }
            49 => { CL_ParseFileTransferFailed(msg); }
            50 => { CL_ParseHLTV(msg); }
            51 => { CL_ParseDirector(msg); }
            52 => { CL_ParseVoiceInit(msg); }
            53 => { CL_ParseVoiceData(msg); }
            56 => { CL_ParseResLocation(msg); }
            57 => { CL_ParseCvarValue(msg, false_0); }
            58 => { CL_ParseCvarValue(msg, true_0); }
            _ => {
                CL_ParseUserMessage(msg, cmd);
                cl.frames[cl.parsecountmod as usize].graphdata.usr =
                    (cl.frames[cl.parsecountmod as usize].graphdata.usr as
                         libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                          as
                                                          libc::c_ulong).wrapping_sub(bufStart))
                        as word as word
            }
        }
    }
    cl.frames[cl.parsecountmod as usize].graphdata.msgbytes =
        (cl.frames[cl.parsecountmod as usize].graphdata.msgbytes as
             libc::c_int + (MSG_GetNumBytesWritten(msg) - cls.starting_count))
            as word;
    CL_Parse_Debug(false_0);
    // we don't know if it is ok to save a demo message until
	// after we have parsed the frame
    if cls.demoplayback == 0 {
        if cls.demorecording as libc::c_uint != 0 &&
               cls.demowaiting as u64 == 0 {
            CL_WriteDemoMessage(false_0, cls.starting_count, msg);
        } else if cls.state as libc::c_uint !=
                      ca_active as libc::c_int as libc::c_uint {
            CL_WriteDemoMessage(true_0, cls.starting_count, msg);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyPrecache_f() {
    let mut spawncount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    if cls.legacymode as u64 == 0 { return }
    spawncount = Q_atoi(Cmd_Argv(1 as libc::c_int));
    Con_Printf(b"Setting up renderer...\n\x00" as *const u8 as
                   *const libc::c_char);
    // load tempent sprites (glowshell, muzzleflashes etc)
    CL_LoadClientSprites();
    // invalidate all decal indexes
    memset(cl.decal_index.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_short; 512]>() as libc::c_ulong);
    cl.video_prepped = true_0;
    cl.audio_prepped = true_0;
    if !clgame.entities.is_null() { (*clgame.entities).model = cl.worldmodel }
    // update the ref state.
    R_UpdateRefState();
    // tell rendering system we have a new set of models.
    ref_0.dllFuncs.R_NewMap.expect("non-null function pointer")();
    CL_SetupOverviewParams();
    // release unused SpriteTextures
    i = 1 as libc::c_int;
    mod_0 = clgame.sprites.as_mut_ptr();
    while i < 256 as libc::c_int {
        if (*mod_0).needload as libc::c_uint ==
               0 as libc::c_int as libc::c_uint &&
               (if (*mod_0).name.as_mut_ptr().is_null() ||
                       *(*mod_0).name.as_mut_ptr() == 0 {
                    0 as libc::c_int
                } else { 1 as libc::c_int }) != 0 {
            Mod_FreeModel(mod_0);
        }
        i += 1;
        mod_0 = mod_0.offset(1)
    }
    //	Mod_FreeUnused ();
    if host_developer.value <= DEV_NONE as libc::c_int as libc::c_float {
        Con_ClearNotify(); // clear any lines of console text
    }
    // done with all resources, issue prespawn command.
	// Include server count in case server disconnects and changes level during d/l
    MSG_WriteCmdExt(&mut cls.netchan.message, 3 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut cls.netchan.message,
                    va(b"begin %i\x00" as *const u8 as *const libc::c_char,
                       spawncount));
    cls.signon = 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyUpdateInfo() {
    if cls.legacymode as u64 == 0 { return }
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return
    }
    MSG_WriteCmdExt(&mut cls.netchan.message, 6 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut cls.netchan.message, cls.userinfo.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CL_LegacyMode() -> qboolean {
    return cls.legacymode;
}
