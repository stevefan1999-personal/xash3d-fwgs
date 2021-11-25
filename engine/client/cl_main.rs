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
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn CL_GetMaxClients() -> libc::c_int;
    #[no_mangle]
    fn Con_Print(txt: *const libc::c_char);
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
    fn SDL_ShowWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
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
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Platform_PreCreateMove();
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_GetCurrentUser() -> *const libc::c_char;
    #[no_mangle]
    fn Sys_Quit() -> !;
    #[no_mangle]
    fn COM_CreateCustomization(pHead: *mut customization_t,
                               pRes: *mut resource_t, playernum: libc::c_int,
                               flags: libc::c_int,
                               pCust: *mut *mut customization_t,
                               nLumps: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn ID_GetMD5() -> *const libc::c_char;
    #[no_mangle]
    fn Info_RemoveKey(s: *mut libc::c_char, key: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn SCR_UpdateScreen();
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_RegisterVariable(var: *mut convar_t);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Cvar_Reset(var_name: *const libc::c_char);
    #[no_mangle]
    fn CRC32_BlockSequence(base: *mut byte, length: libc::c_int,
                           sequence: libc::c_int) -> byte;
    #[no_mangle]
    fn CRC32_ProcessBuffer(pulCRC: *mut dword, pBuffer: *const libc::c_void,
                           nBuffer: libc::c_int);
    #[no_mangle]
    fn MD5Init(ctx: *mut MD5Context_t);
    #[no_mangle]
    fn MD5Update(ctx: *mut MD5Context_t, buf: *const byte, len: uint);
    #[no_mangle]
    fn MD5Final(digest: *mut byte, ctx: *mut MD5Context_t);
    #[no_mangle]
    fn MD5_Print(hash: *mut byte) -> *mut libc::c_char;
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut host_limitlocal: *mut convar_t;
    #[no_mangle]
    fn NET_Config(net_enable: qboolean);
    #[no_mangle]
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    fn NET_StringToAdr(string: *const libc::c_char, adr: *mut netadr_t)
     -> qboolean;
    #[no_mangle]
    fn NET_StringToAdrNB(string: *const libc::c_char, adr: *mut netadr_t)
     -> libc::c_int;
    #[no_mangle]
    fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_GetPacket(sock: netsrc_t, from: *mut netadr_t, data: *mut byte,
                     length: *mut size_t) -> qboolean;
    #[no_mangle]
    fn NET_SendPacket(sock: netsrc_t, length: size_t,
                      data: *const libc::c_void, to: netadr_t);
    #[no_mangle]
    fn Con_NPrintf(idx: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SCR_Shutdown();
    #[no_mangle]
    fn S_Shutdown();
    #[no_mangle]
    fn NET_SendToMasters(sock: netsrc_t, len: size_t,
                         data: *const libc::c_void) -> qboolean;
    #[no_mangle]
    fn Info_SetValueForKey(s: *mut libc::c_char, key: *const libc::c_char,
                           value: *const libc::c_char, maxsize: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn Info_Print(s: *const libc::c_char);
    #[no_mangle]
    fn UI_CreditsActive() -> qboolean;
    #[no_mangle]
    fn COM_HexConvert(pszInput: *const libc::c_char,
                      nInputLength: libc::c_int, pOutput: *mut byte);
    #[no_mangle]
    fn HPAK_ResourceForHash(filename: *const libc::c_char, hash: *mut byte,
                            pRes: *mut resource_s) -> qboolean;
    #[no_mangle]
    fn HPAK_GetDataPointer(filename: *const libc::c_char,
                           pRes: *mut resource_s, buffer: *mut *mut byte,
                           size: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn HPAK_RemoveLump(name_0: *const libc::c_char,
                       resource: *mut resource_t);
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_ClearCustomizationList(pHead: *mut customization_t,
                                  bCleanDecals: qboolean);
    #[no_mangle]
    fn S_StopAllSounds(ambient: qboolean);
    #[no_mangle]
    fn HPAK_FlushHostQueue();
    #[no_mangle]
    fn HPAK_AddLump(queue: qboolean, filename: *const libc::c_char,
                    pRes: *mut resource_s, data: *mut byte, f: *mut file_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn HTTP_ClearCustomServers();
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_Execute();
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
    fn Cmd_TokenizeString(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_ForwardToServer();
    #[no_mangle]
    fn Cmd_Escape(newCommand: *mut libc::c_char,
                  oldCommand: *const libc::c_char, len: libc::c_int);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Mem_PrintStats();
    #[no_mangle]
    fn MD5_HashFile(digest: *mut byte, pszFileName: *const libc::c_char,
                    seed: *mut uint) -> qboolean;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn FS_FileLength(f: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    fn Q_buildos() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildarch() -> *const libc::c_char;
    #[no_mangle]
    fn Host_EndGame(abort: qboolean, message: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_WriteOpenGLConfig();
    #[no_mangle]
    fn Host_WriteVideoConfig();
    #[no_mangle]
    fn Host_WriteConfig();
    #[no_mangle]
    fn Host_IsLocalGame() -> qboolean;
    #[no_mangle]
    fn Host_IsLocalClient() -> qboolean;
    #[no_mangle]
    fn Host_ShutdownServer();
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn UI_SetActiveMenu(fActive: qboolean);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn VID_Init();
    #[no_mangle]
    fn S_Init() -> qboolean;
    #[no_mangle]
    fn CL_LoadProgs(name_0: *const libc::c_char) -> qboolean;
    #[no_mangle]
    static mut CL_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut gl_showtextures: *mut convar_t;
    #[no_mangle]
    fn Con_LoadHistory();
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn CL_LegacyPrecache_f();
    #[no_mangle]
    fn MSG_BigShort(swap: libc::c_ushort) -> libc::c_ushort;
    #[no_mangle]
    fn Con_Shutdown();
    #[no_mangle]
    fn R_Shutdown();
    #[no_mangle]
    fn SCR_FreeCinematic();
    #[no_mangle]
    fn CL_UnloadProgs();
    #[no_mangle]
    fn Mobile_Shutdown();
    #[no_mangle]
    fn CL_CloseDemoHeader();
    #[no_mangle]
    fn Netchan_UpdateProgress(chan: *mut netchan_t);
    #[no_mangle]
    fn CL_WriteDemoUserCmd(cmdnumber: libc::c_int);
    #[no_mangle]
    fn MSG_WriteBits(sb: *mut sizebuf_t, pData: *const libc::c_void,
                     nBits: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn Netchan_CanPacket(chan: *mut netchan_t, choke: qboolean) -> qboolean;
    #[no_mangle]
    fn CL_PredictMovement(repredicting: qboolean);
    #[no_mangle]
    static mut world: world_static_t;
    #[no_mangle]
    fn CL_PopPMStates();
    #[no_mangle]
    fn CL_SetSolidPlayers(playernum: libc::c_int);
    #[no_mangle]
    fn CL_PushPMStates();
    #[no_mangle]
    fn CL_SetSolidEntities();
    #[no_mangle]
    fn Netchan_ReportFlow(chan: *mut netchan_t);
    #[no_mangle]
    fn SCR_MakeLevelShot();
    #[no_mangle]
    fn SCR_RunCinematic();
    #[no_mangle]
    fn SND_UpdateSound();
    #[no_mangle]
    fn CL_MoveSpectatorCamera();
    #[no_mangle]
    fn CL_MoveThirdpersonCamera();
    #[no_mangle]
    fn CL_RequestMissingResources() -> qboolean;
    #[no_mangle]
    fn CL_Record_f();
    #[no_mangle]
    fn CL_EmitEntities();
    #[no_mangle]
    fn CL_RedoPrediction();
    #[no_mangle]
    fn CL_BatchResourceRequest(initialize: qboolean);
    #[no_mangle]
    fn CL_EstimateNeededResources() -> libc::c_int;
    #[no_mangle]
    fn Netchan_CreateFragments(chan: *mut netchan_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_RegisterResources(msg: *mut sizebuf_t);
    #[no_mangle]
    fn S_EndRegistration();
    #[no_mangle]
    fn CL_SetEventIndex(szEvName: *const libc::c_char, ev_index: libc::c_int);
    #[no_mangle]
    fn CL_LoadClientSprite(filename: *const libc::c_char) -> *mut model_t;
    #[no_mangle]
    fn Mod_ForName(name_0: *const libc::c_char, crash: qboolean,
                   trackCRC: qboolean) -> *mut model_t;
    #[no_mangle]
    fn S_RegisterSound(sample: *const libc::c_char) -> sound_t;
    #[no_mangle]
    fn S_BeginRegistration();
    #[no_mangle]
    fn Mod_LoadWorld(name_0: *const libc::c_char, preload: qboolean)
     -> *mut model_t;
    #[no_mangle]
    fn CL_MoveToOnHandList(pResource: *mut resource_t);
    #[no_mangle]
    static mut net_message: sizebuf_t;
    #[no_mangle]
    fn Netchan_CopyFileFragments(chan: *mut netchan_t, msg: *mut sizebuf_t)
     -> qboolean;
    #[no_mangle]
    fn CL_ParseServerMessage(msg: *mut sizebuf_t, normal_message: qboolean);
    #[no_mangle]
    static mut net_message_buffer: [byte; 131120];
    #[no_mangle]
    fn Netchan_CopyNormalFragments(chan: *mut netchan_t, msg: *mut sizebuf_t,
                                   length: *mut size_t) -> qboolean;
    #[no_mangle]
    fn Netchan_IncomingReady(chan: *mut netchan_t) -> qboolean;
    #[no_mangle]
    fn CL_ParseLegacyServerMessage(msg: *mut sizebuf_t,
                                   normal_message: qboolean);
    #[no_mangle]
    fn CL_ParseQuakeMessage(msg: *mut sizebuf_t, normal_message: qboolean);
    #[no_mangle]
    fn Netchan_Process(chan: *mut netchan_t, msg: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn Netchan_OutOfBand(net_socket: libc::c_int, adr: netadr_t,
                         length: libc::c_int, data: *mut byte);
    #[no_mangle]
    fn UI_ResetPing();
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadBytes(sb: *mut sizebuf_t, pOut: *mut libc::c_void,
                     nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn UI_ShowUpdateDialog(preferStore: qboolean);
    #[no_mangle]
    fn UI_ShowMessageBox(text: *const libc::c_char);
    #[no_mangle]
    fn UI_IsVisible() -> qboolean;
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[no_mangle]
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn Netchan_OutOfBandPrint(net_socket: libc::c_int, adr: netadr_t,
                              format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn UI_AddServerToList(adr: netadr_t, info: *const libc::c_char);
    #[no_mangle]
    fn NetSplit_GetLong(ns: *mut netsplit_t, from: *mut netadr_t,
                        data: *mut byte, length: *mut size_t) -> qboolean;
    #[no_mangle]
    fn CL_DemoReadMessage(buffer: *mut byte, length: *mut size_t) -> qboolean;
    #[no_mangle]
    fn CL_SetLastUpdate();
    #[no_mangle]
    fn CL_PlayCDTrack_f();
    #[no_mangle]
    fn CL_WavePlayLen_f();
    #[no_mangle]
    fn CL_PlayDemo_f();
    #[no_mangle]
    fn CL_TimeDemo_f();
    #[no_mangle]
    fn CL_DeleteDemo_f();
    #[no_mangle]
    fn CL_StartDemos_f();
    #[no_mangle]
    fn CL_Demos_f();
    #[no_mangle]
    fn CL_PlayVideo_f();
    #[no_mangle]
    fn SCR_NextMovie() -> qboolean;
    #[no_mangle]
    fn CL_ReadPointFile_f();
    #[no_mangle]
    fn CL_ReadLineFile_f();
    #[no_mangle]
    fn Netchan_CreateFileFragmentsFromBuffer(chan: *mut netchan_t,
                                             filename: *const libc::c_char,
                                             pbuf: *mut byte,
                                             size: libc::c_int);
    #[no_mangle]
    fn Netchan_FragSend(chan: *mut netchan_t);
    #[no_mangle]
    fn CL_ScreenShot_f();
    #[no_mangle]
    fn CL_SnapShot_f();
    #[no_mangle]
    fn CL_EnvShot_f();
    #[no_mangle]
    fn CL_SkyShot_f();
    #[no_mangle]
    fn CL_LevelShot_f();
    #[no_mangle]
    fn CL_SaveShot_f();
    #[no_mangle]
    fn Key_SetKeyDest(key_dest: libc::c_int);
    #[no_mangle]
    fn Netchan_Setup(sock: netsrc_t, chan: *mut netchan_t, adr: netadr_t,
                     qport: libc::c_int, client: *mut libc::c_void,
                     pfnBlockSize:
                         Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: fragsize_t)
                                    -> libc::c_int>);
    #[no_mangle]
    static mut net_from: netadr_t;
    #[no_mangle]
    fn Netchan_IsLocal(chan: *mut netchan_t) -> qboolean;
    #[no_mangle]
    fn CL_StartupDemoHeader();
    #[no_mangle]
    fn CL_Stop_f();
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name_0: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn Netchan_TransmitBits(chan: *mut netchan_t, lengthInBits: libc::c_int,
                            data: *mut byte);
    #[no_mangle]
    fn CL_ClearResourceLists();
    #[no_mangle]
    fn CL_ClearEffects();
    #[no_mangle]
    fn CL_FreeEdicts();
    #[no_mangle]
    fn CL_ClearPhysEnts();
    #[no_mangle]
    fn NetAPI_CancelAllRequests();
    #[no_mangle]
    fn MSG_Clear(sb: *mut sizebuf_t);
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    fn CL_ClearSpriteTextures();
    #[no_mangle]
    fn SCR_EndLoadingPlaque();
    #[no_mangle]
    fn Netchan_Clear(chan: *mut netchan_t);
    #[no_mangle]
    fn MSG_WriteDeltaUsercmd(msg: *mut sizebuf_t, from: *mut usercmd_s,
                             to: *mut usercmd_s);
    #[no_mangle]
    fn IN_Shutdown();
    #[no_mangle]
    fn IN_CollectInputDevices() -> uint;
    #[no_mangle]
    fn IN_LockInputDevices(lock: qboolean);
    #[no_mangle]
    fn IN_EngineAppendMove(frametime: libc::c_float, cmd: *mut libc::c_void,
                           active: qboolean);
    #[no_mangle]
    fn Touch_WriteConfig();
    #[no_mangle]
    fn Joy_Shutdown();
    #[no_mangle]
    fn VGui_RunFrame();
    #[no_mangle]
    fn COM_GetLibraryError() -> *const libc::c_char;
    #[no_mangle]
    fn COM_GetCommonLibraryPath(eLibType: ECommonLibraryType,
                                out: *mut libc::c_char, size: size_t);
    #[no_mangle]
    fn VID_CheckChanges();
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
pub struct MD5Context_t {
    pub buf: [uint; 4],
    pub bits: [uint; 2],
    pub in_0: [uint; 16],
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
pub type host_parm_t = host_parm_s;
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
pub type host_redirect_t = host_redirect_s;
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
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
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
pub type ECommonLibraryType = libc::c_uint;
pub const LIBRARY_GAMEUI: ECommonLibraryType = 2;
pub const LIBRARY_SERVER: ECommonLibraryType = 1;
pub const LIBRARY_CLIENT: ECommonLibraryType = 0;
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
pub type world_static_t = world_static_s;
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
pub const DEMO_QUAKE1: C2RustUnnamed_1 = 2;
pub type net_adrlist_t = net_adrlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_adrlist_s {
    pub next: *mut net_adrlist_s,
    pub remote_address: netadr_t,
}
pub const DEMO_XASH3D: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DEMO_INACTIVE: C2RustUnnamed_1 = 0;
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
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return BitByte((*sb).iCurBit);
}
#[inline]
unsafe extern "C" fn BitByte(mut bits: libc::c_int) -> libc::c_int {
    return (bits + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int *
               8 as libc::c_int >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetRealBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit >> 3 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs_0(mut __x: libc::c_double) -> libc::c_double {
    return fabs(__x);
}
#[inline]
unsafe extern "C" fn MSG_GetMaxBytes(mut sb: *mut sizebuf_t) -> libc::c_int {
    return (*sb).nDataBits >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetData(mut sb: *mut sizebuf_t) -> *mut byte {
    return (*sb).pData;
}
#[no_mangle]
pub static mut mp_decals: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mp_decals\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"300\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"decals limit in multiplayer\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut dev_overview: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"dev_overview\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"draw level in overview-mode\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_resend: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_resend\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"6.0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"time to resend connect\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_allow_download: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_allow_download\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow to downloading resources from the server\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_allow_upload: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_allow_upload\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow to uploading resources to the server\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_download_ingame: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_download_ingame\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow to downloading resources while client is active\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_logofile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_logofile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"lambda\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"player logo name\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_logocolor: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_logocolor\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"orange\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"player logo color\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut cl_test_bandwidth: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_test_bandwidth\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"test network bandwith before connection\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut rcon_client_password: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut rcon_address: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_timeout: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_nopred: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_nodelta: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_crosshair: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_cmdbackup: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_showerror: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_bmodelinterp: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_draw_particles: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_draw_tracers: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_lightstyle_lerping: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_idealpitchscale: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_nosmooth: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_smoothtime: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_clockreset: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_fixtimerate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut hud_scale: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_solid_players: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_draw_beams: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_updaterate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_showevents: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_cmdrate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_interp: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_nointerp: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_dlmax: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_upmax: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_lw: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_charset: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_trace_messages: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut hud_utf8: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut ui_renderworld: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
//
// userinfo
//
#[no_mangle]
pub static mut name: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut model: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut topcolor: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut bottomcolor: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut rate: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl: client_t =
    client_t{servercount: 0,
             validsequence: 0,
             parsecount: 0,
             parsecountmod: 0,
             video_prepped: false_0,
             audio_prepped: false_0,
             paused: false_0,
             delta_sequence: 0,
             mtime: [0.; 2],
             lerpFrac: 0.,
             last_command_ack: 0,
             last_incoming_sequence: 0,
             send_reply: false_0,
             background: false_0,
             first_frame: false_0,
             proxy_redirect: false_0,
             skip_interp: false_0,
             checksum: 0,
             frames:
                 [frame_t{receivedtime: 0.,
                          latency: 0.,
                          time: 0.,
                          valid: false_0,
                          choked: false_0,
                          clientdata:
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
                                           vuser4: [0.; 3],},
                          playerstate:
                              [entity_state_t{entityType: 0,
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
                                                  color24{r: 0, g: 0, b: 0,},
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
                                              vuser4: [0.; 3],}; 32],
                          weapondata:
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
                                             fuser4: 0.,}; 64],
                          graphdata:
                              netbandwidthgraph_t{client: 0,
                                                  players: 0,
                                                  entities: 0,
                                                  tentities: 0,
                                                  sound: 0,
                                                  event: 0,
                                                  usr: 0,
                                                  msgbytes: 0,
                                                  voicebytes: 0,},
                          flags: [0; 256],
                          num_entities: 0,
                          first_entity: 0,}; 64],
             commands:
                 [runcmd_t{senttime: 0.,
                           receivedtime: 0.,
                           frame_lerp: 0.,
                           cmd:
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
                                         impact_position: [0.; 3],},
                           processedfuncs: false_0,
                           heldback: false_0,
                           sendsize: 0,}; 64],
             predicted_frames:
                 [local_state_t{playerstate:
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
                                client:
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
                                                 vuser4: [0.; 3],},
                                weapondata:
                                    [weapon_data_t{m_iId: 0,
                                                   m_iClip: 0,
                                                   m_flNextPrimaryAttack: 0.,
                                                   m_flNextSecondaryAttack:
                                                       0.,
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
                                                   fuser4: 0.,}; 64],}; 64],
             time: 0.,
             oldtime: 0.,
             timedelta: 0.,
             serverinfo: [0; 512],
             players:
                 [player_info_t{userid: 0,
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
                                                                   nIndex: 0,
                                                                   nDownloadSize:
                                                                       0,
                                                                   ucFlags: 0,
                                                                   rgucMD5_hash:
                                                                       [0;
                                                                           16],
                                                                   playernum:
                                                                       0,
                                                                   rguc_reserved:
                                                                       [0;
                                                                           32],
                                                                   pNext:
                                                                       0 as
                                                                           *const resource_s
                                                                           as
                                                                           *mut resource_s,
                                                                   pPrev:
                                                                       0 as
                                                                           *const resource_s
                                                                           as
                                                                           *mut resource_s,},
                                                    bTranslated: false_0,
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
                                hashedcdkey: [0; 16],}; 32],
             lastresourcecheck: 0.,
             downloadUrl: [0; 256],
             events:
                 event_state_t{ei:
                                   [event_info_t{index: 0,
                                                 packet_index: 0,
                                                 entity_index: 0,
                                                 fire_time: 0.,
                                                 args:
                                                     event_args_t{flags: 0,
                                                                  entindex: 0,
                                                                  origin:
                                                                      [0.; 3],
                                                                  angles:
                                                                      [0.; 3],
                                                                  velocity:
                                                                      [0.; 3],
                                                                  ducking: 0,
                                                                  fparam1: 0.,
                                                                  fparam2: 0.,
                                                                  iparam1: 0,
                                                                  iparam2: 0,
                                                                  bparam1: 0,
                                                                  bparam2:
                                                                      0,},
                                                 flags: 0,}; 64],},
             local:
                 cl_local_data_t{predicted_origins: [[0.; 3]; 64],
                                 prediction_error: [0.; 3],
                                 lastorigin: [0.; 3],
                                 lastground: 0,
                                 interp_amount: 0.,
                                 repredicting: false_0,
                                 thirdperson: false_0,
                                 apply_effects: false_0,
                                 idealpitch: 0.,
                                 viewmodel: 0,
                                 health: 0,
                                 onground: 0,
                                 light_level: 0,
                                 waterlevel: 0,
                                 usehull: 0,
                                 moving: 0,
                                 pushmsec: 0,
                                 weapons: 0,
                                 maxspeed: 0.,
                                 scr_fov: 0.,
                                 weaponsequence: 0,
                                 weaponstarttime: 0.,},
             cmd: 0 as *const usercmd_t as *mut usercmd_t,
             viewentity: 0,
             viewangles: [0.; 3],
             viewheight: [0.; 3],
             punchangle: [0.; 3],
             intermission: 0,
             crosshairangle: [0.; 3],
             predicted_angle:
                 [pred_viewangle_t{starttime: 0., total: 0.,}; 16],
             angle_position: 0,
             addangletotal: 0.,
             prevaddangletotal: 0.,
             simorg: [0.; 3],
             simvel: [0.; 3],
             playernum: 0,
             maxclients: 0,
             instanced_baseline:
                 [entity_state_t{entityType: 0,
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
                                 vuser4: [0.; 3],}; 64],
             instanced_baseline_count: 0,
             sound_precache: [[0; 64]; 2048],
             event_precache: [[0; 64]; 1024],
             files_precache: [[0; 64]; 1024],
             lightstyles:
                 [lightstyle_t{pattern: [0; 256],
                               map: [0.; 256],
                               length: 0,
                               value: 0.,
                               interp: false_0,
                               time: 0.,}; 64],
             models: [0 as *const model_t as *mut model_t; 1025],
             nummodels: 0,
             numfiles: 0,
             consistency_list:
                 [consistency_t{filename: 0 as *const libc::c_char,
                                orig_index: 0,
                                check_type: 0,
                                issound: false_0,
                                value: 0,
                                mins: [0.; 3],
                                maxs: [0.; 3],}; 1024],
             num_consistency: 0,
             need_force_consistency_response: false_0,
             resourcesonhand:
                 resource_t{szFileName: [0; 64],
                            type_0: t_sound,
                            nIndex: 0,
                            nDownloadSize: 0,
                            ucFlags: 0,
                            rgucMD5_hash: [0; 16],
                            playernum: 0,
                            rguc_reserved: [0; 32],
                            pNext: 0 as *const resource_s as *mut resource_s,
                            pPrev:
                                0 as *const resource_s as *mut resource_s,},
             resourcesneeded:
                 resource_t{szFileName: [0; 64],
                            type_0: t_sound,
                            nIndex: 0,
                            nDownloadSize: 0,
                            ucFlags: 0,
                            rgucMD5_hash: [0; 16],
                            playernum: 0,
                            rguc_reserved: [0; 32],
                            pNext: 0 as *const resource_s as *mut resource_s,
                            pPrev:
                                0 as *const resource_s as *mut resource_s,},
             resourcelist:
                 [resource_t{szFileName: [0; 64],
                             type_0: t_sound,
                             nIndex: 0,
                             nDownloadSize: 0,
                             ucFlags: 0,
                             rgucMD5_hash: [0; 16],
                             playernum: 0,
                             rguc_reserved: [0; 32],
                             pNext: 0 as *const resource_s as *mut resource_s,
                             pPrev:
                                 0 as *const resource_s as *mut resource_s,};
                     5120],
             num_resources: 0,
             sound_index: [0; 2048],
             decal_index: [0; 512],
             worldmodel: 0 as *const model_t as *mut model_t,
             lostpackets: 0,};
#[no_mangle]
pub static mut cls: client_static_t =
    client_static_t{state: ca_disconnected,
                    initialized: false_0,
                    changelevel: false_0,
                    changedemo: false_0,
                    timestart: 0.,
                    disable_screen: 0.,
                    disable_servercount: 0,
                    draw_changelevel: false_0,
                    key_dest: key_console,
                    mempool: 0,
                    hltv_listen_address:
                        netadr_t{type_0: NA_UNUSED,
                                 ip: [0; 4],
                                 ipx: [0; 10],
                                 port: 0,},
                    signon: 0,
                    quakePort: 0,
                    servername: [0; 64],
                    connect_time: 0.,
                    max_fragment_size: 0,
                    connect_retry: 0,
                    spectator: false_0,
                    spectator_state:
                        local_state_t{playerstate:
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
                                                         basevelocity:
                                                             [0.; 3],
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
                                      client:
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
                                                       vuser4: [0.; 3],},
                                      weapondata:
                                          [weapon_data_t{m_iId: 0,
                                                         m_iClip: 0,
                                                         m_flNextPrimaryAttack:
                                                             0.,
                                                         m_flNextSecondaryAttack:
                                                             0.,
                                                         m_flTimeWeaponIdle:
                                                             0.,
                                                         m_fInReload: 0,
                                                         m_fInSpecialReload:
                                                             0,
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
                                                         fuser4: 0.,}; 64],},
                    userinfo: [0; 256],
                    physinfo: [0; 256],
                    datagram:
                        sizebuf_t{bOverflow: false_0,
                                  pDebugName: 0 as *const libc::c_char,
                                  pData: 0 as *const byte as *mut byte,
                                  iCurBit: 0,
                                  nDataBits: 0,},
                    datagram_buf: [0; 16384],
                    netchan:
                        netchan_t{sock: NS_CLIENT,
                                  remote_address:
                                      netadr_t{type_0: NA_UNUSED,
                                               ip: [0; 4],
                                               ipx: [0; 10],
                                               port: 0,},
                                  qport: 0,
                                  last_received: 0.,
                                  connect_time: 0.,
                                  rate: 0.,
                                  cleartime: 0.,
                                  incoming_sequence: 0,
                                  incoming_acknowledged: 0,
                                  incoming_reliable_acknowledged: 0,
                                  incoming_reliable_sequence: 0,
                                  outgoing_sequence: 0,
                                  reliable_sequence: 0,
                                  last_reliable_sequence: 0,
                                  client:
                                      0 as *const libc::c_void as
                                          *mut libc::c_void,
                                  pfnBlockSize: None,
                                  message:
                                      sizebuf_t{bOverflow: false_0,
                                                pDebugName:
                                                    0 as *const libc::c_char,
                                                pData:
                                                    0 as *const byte as
                                                        *mut byte,
                                                iCurBit: 0,
                                                nDataBits: 0,},
                                  message_buf: [0; 131120],
                                  reliable_length: 0,
                                  reliable_buf: [0; 131120],
                                  waitlist:
                                      [0 as *const fragbufwaiting_t as
                                           *mut fragbufwaiting_t; 2],
                                  reliable_fragment: [0; 2],
                                  reliable_fragid: [0; 2],
                                  fragbufs:
                                      [0 as *const fragbuf_t as
                                           *mut fragbuf_t; 2],
                                  fragbufcount: [0; 2],
                                  frag_startpos: [0; 2],
                                  frag_length: [0; 2],
                                  incomingbufs:
                                      [0 as *const fragbuf_t as
                                           *mut fragbuf_t; 2],
                                  incomingready: [false_0; 2],
                                  incomingfilename: [0; 260],
                                  tempbuffer:
                                      0 as *const libc::c_void as
                                          *mut libc::c_void,
                                  tempbuffersize: 0,
                                  flow:
                                      [flow_t{stats:
                                                  [flowstats_t{size: 0,
                                                               time: 0.,};
                                                      32],
                                              current: 0,
                                              nextcompute: 0.,
                                              kbytespersec: 0.,
                                              avgkbytespersec: 0.,
                                              totalbytes: 0,}; 2],
                                  total_sended: 0,
                                  total_received: 0,
                                  split: false_0,
                                  maxpacket: 0,
                                  splitid: 0,
                                  netsplit:
                                      netsplit_t{packets:
                                                     [netsplit_chain_packet_t{recieved_v:
                                                                                  [0;
                                                                                      8],
                                                                              id:
                                                                                  0,
                                                                              data:
                                                                                  [0;
                                                                                      131072],
                                                                              received:
                                                                                  0,
                                                                              count:
                                                                                  0,};
                                                         8],
                                                 total_received: 0,
                                                 total_received_uncompressed:
                                                     0,},},
                    challenge: 0,
                    packet_loss: 0.,
                    packet_loss_recalc_time: 0.,
                    starting_count: 0,
                    nextcmdtime: 0.,
                    lastoutgoingcommand: 0,
                    lastupdate_sequence: 0,
                    td_lastframe: 0,
                    td_startframe: 0,
                    td_starttime: 0.,
                    forcetrack: 0,
                    pauseIcon: 0,
                    tileImage: 0,
                    loadingBar: 0,
                    creditsFont:
                        cl_font_t{hFontTexture: 0,
                                  fontRc:
                                      [wrect_t{left: 0,
                                               right: 0,
                                               top: 0,
                                               bottom: 0,}; 256],
                                  charWidths: [0; 256],
                                  charHeight: 0,
                                  type_0: 0,
                                  valid: false_0,},
                    latency: 0.,
                    num_client_entities: 0,
                    next_client_entities: 0,
                    packet_entities:
                        0 as *const entity_state_t as *mut entity_state_t,
                    predicted_players:
                        [predicted_player_t{movetype: 0,
                                            solid: 0,
                                            usehull: 0,
                                            active: false_0,
                                            origin: [0.; 3],
                                            angles: [0.; 3],}; 32],
                    correction_time: 0.,
                    scrshot_request: scrshot_inactive,
                    scrshot_action: scrshot_inactive,
                    envshot_vieworg: 0 as *const libc::c_float,
                    envshot_viewsize: 0,
                    envshot_disable_vis: false_0,
                    shotname: [0; 256],
                    dl:
                        incomingtransfer_t{doneregistering: false_0,
                                           percent: 0,
                                           downloadrequested: false_0,
                                           rgStats:
                                               [downloadtime_t{bUsed: false_0,
                                                               fTime: 0.,
                                                               nBytesRemaining:
                                                                   0,}; 8],
                                           nCurStat: 0,
                                           nTotalSize: 0,
                                           nTotalToTransfer: 0,
                                           nRemainingToTransfer: 0,
                                           fLastStatusUpdate: 0.,
                                           custom: false_0,},
                    demonum: 0,
                    olddemonum: 0,
                    demos: [[0; 64]; 32],
                    demos_pending: false_0,
                    movienum: 0,
                    movies: [[0; 256]; 8],
                    demorecording: false_0,
                    demoplayback: 0,
                    demowaiting: false_0,
                    timedemo: false_0,
                    demoname: [0; 256],
                    demotime: 0.,
                    set_lastdemo: false_0,
                    demofile: 0 as *const file_t as *mut file_t,
                    demoheader: 0 as *const file_t as *mut file_t,
                    internetservers_wait: false_0,
                    internetservers_pending: false_0,
                    legacymode: false_0,
                    legacyserver:
                        netadr_t{type_0: NA_UNUSED,
                                 ip: [0; 4],
                                 ipx: [0; 10],
                                 port: 0,},
                    legacyservers:
                        [netadr_t{type_0: NA_UNUSED,
                                  ip: [0; 4],
                                  ipx: [0; 10],
                                  port: 0,}; 256],
                    legacyservercount: 0,
                    extensions: 0,
                    serveradr:
                        netadr_t{type_0: NA_UNUSED,
                                 ip: [0; 4],
                                 ipx: [0; 10],
                                 port: 0,},};
#[no_mangle]
pub static mut clgame: clgame_static_t =
    clgame_static_t{hInstance: 0 as *const libc::c_void as *mut libc::c_void,
                    dllFuncs:
                        cldll_func_t{pfnInitialize: None,
                                     pfnInit: None,
                                     pfnVidInit: None,
                                     pfnRedraw: None,
                                     pfnUpdateClientData: None,
                                     pfnReset: None,
                                     pfnPlayerMove: None,
                                     pfnPlayerMoveInit: None,
                                     pfnPlayerMoveTexture: None,
                                     IN_ActivateMouse: None,
                                     IN_DeactivateMouse: None,
                                     IN_MouseEvent: None,
                                     IN_ClearStates: None,
                                     IN_Accumulate: None,
                                     CL_CreateMove: None,
                                     CL_IsThirdPerson: None,
                                     CL_CameraOffset: None,
                                     KB_Find: None,
                                     CAM_Think: None,
                                     pfnCalcRefdef: None,
                                     pfnAddEntity: None,
                                     pfnCreateEntities: None,
                                     pfnDrawNormalTriangles: None,
                                     pfnDrawTransparentTriangles: None,
                                     pfnStudioEvent: None,
                                     pfnPostRunCmd: None,
                                     pfnShutdown: None,
                                     pfnTxferLocalOverrides: None,
                                     pfnProcessPlayerState: None,
                                     pfnTxferPredictionData: None,
                                     pfnDemo_ReadBuffer: None,
                                     pfnConnectionlessPacket: None,
                                     pfnGetHullBounds: None,
                                     pfnFrame: None,
                                     pfnKey_Event: None,
                                     pfnTempEntUpdate: None,
                                     pfnGetUserEntity: None,
                                     pfnVoiceStatus: None,
                                     pfnDirectorMessage: None,
                                     pfnGetStudioModelInterface: None,
                                     pfnChatInputPosition: None,
                                     pfnGetRenderInterface: None,
                                     pfnClipMoveToEntity: None,
                                     pfnTouchEvent: None,
                                     pfnMoveEvent: None,
                                     pfnLookEvent: None,},
                    drawFuncs:
                        render_interface_t{version: 0,
                                           GL_RenderFrame: None,
                                           GL_BuildLightmaps: None,
                                           GL_OrthoBounds: None,
                                           R_CreateStudioDecalList: None,
                                           R_ClearStudioDecals: None,
                                           R_SpeedsMessage: None,
                                           Mod_ProcessUserData: None,
                                           R_ProcessEntData: None,
                                           Mod_GetCurrentVis: None,
                                           R_NewMap: None,
                                           R_ClearScene: None,
                                           CL_UpdateLatchedVars: None,},
                    mempool: 0,
                    mapname: [0; 256],
                    maptitle: [0; 256],
                    itemspath: [0; 256],
                    entities: 0 as *const cl_entity_t as *mut cl_entity_t,
                    static_entities:
                        0 as *const cl_entity_t as *mut cl_entity_t,
                    remap_info:
                        0 as *const *mut remap_info_t as
                            *mut *mut remap_info_t,
                    maxEntities: 0,
                    maxRemapInfos: 0,
                    numStatics: 0,
                    maxModels: 0,
                    movevars:
                        movevars_t{gravity: 0.,
                                   stopspeed: 0.,
                                   maxspeed: 0.,
                                   spectatormaxspeed: 0.,
                                   accelerate: 0.,
                                   airaccelerate: 0.,
                                   wateraccelerate: 0.,
                                   friction: 0.,
                                   edgefriction: 0.,
                                   waterfriction: 0.,
                                   entgravity: 0.,
                                   bounce: 0.,
                                   stepsize: 0.,
                                   maxvelocity: 0.,
                                   zmax: 0.,
                                   waveHeight: 0.,
                                   footsteps: false_0,
                                   skyName: [0; 32],
                                   rollangle: 0.,
                                   rollspeed: 0.,
                                   skycolor_r: 0.,
                                   skycolor_g: 0.,
                                   skycolor_b: 0.,
                                   skyvec_x: 0.,
                                   skyvec_y: 0.,
                                   skyvec_z: 0.,
                                   features: 0,
                                   fog_settings: 0,
                                   wateralpha: 0.,
                                   skydir_x: 0.,
                                   skydir_y: 0.,
                                   skydir_z: 0.,
                                   skyangle: 0.,},
                    oldmovevars:
                        movevars_t{gravity: 0.,
                                   stopspeed: 0.,
                                   maxspeed: 0.,
                                   spectatormaxspeed: 0.,
                                   accelerate: 0.,
                                   airaccelerate: 0.,
                                   wateraccelerate: 0.,
                                   friction: 0.,
                                   edgefriction: 0.,
                                   waterfriction: 0.,
                                   entgravity: 0.,
                                   bounce: 0.,
                                   stepsize: 0.,
                                   maxvelocity: 0.,
                                   zmax: 0.,
                                   waveHeight: 0.,
                                   footsteps: false_0,
                                   skyName: [0; 32],
                                   rollangle: 0.,
                                   rollspeed: 0.,
                                   skycolor_r: 0.,
                                   skycolor_g: 0.,
                                   skycolor_b: 0.,
                                   skyvec_x: 0.,
                                   skyvec_y: 0.,
                                   skyvec_z: 0.,
                                   features: 0,
                                   fog_settings: 0,
                                   wateralpha: 0.,
                                   skydir_x: 0.,
                                   skydir_y: 0.,
                                   skydir_z: 0.,
                                   skyangle: 0.,},
                    pmove: 0 as *const playermove_t as *mut playermove_t,
                    pushed: false_0,
                    oldviscount: 0,
                    oldphyscount: 0,
                    msg:
                        [cl_user_message_t{name: [0; 32],
                                           number: 0,
                                           size: 0,
                                           func: None,}; 197],
                    events:
                        [0 as *const cl_user_event_t as *mut cl_user_event_t;
                            1024],
                    cdtracks: [[0; 256]; 32],
                    sprites:
                        [model_t{name: [0; 64],
                                 needload: false_0,
                                 type_0: mod_brush,
                                 numframes: 0,
                                 mempool: 0,
                                 flags: 0,
                                 mins: [0.; 3],
                                 maxs: [0.; 3],
                                 radius: 0.,
                                 firstmodelsurface: 0,
                                 nummodelsurfaces: 0,
                                 numsubmodels: 0,
                                 submodels:
                                     0 as *const dmodel_t as *mut dmodel_t,
                                 numplanes: 0,
                                 planes:
                                     0 as *const mplane_t as *mut mplane_t,
                                 numleafs: 0,
                                 leafs: 0 as *const mleaf_t as *mut mleaf_t,
                                 numvertexes: 0,
                                 vertexes:
                                     0 as *const mvertex_t as *mut mvertex_t,
                                 numedges: 0,
                                 edges: 0 as *const medge_t as *mut medge_t,
                                 numnodes: 0,
                                 nodes: 0 as *const mnode_t as *mut mnode_t,
                                 numtexinfo: 0,
                                 texinfo:
                                     0 as *const mtexinfo_t as
                                         *mut mtexinfo_t,
                                 numsurfaces: 0,
                                 surfaces:
                                     0 as *const msurface_t as
                                         *mut msurface_t,
                                 numsurfedges: 0,
                                 surfedges:
                                     0 as *const libc::c_int as
                                         *mut libc::c_int,
                                 numclipnodes: 0,
                                 clipnodes:
                                     0 as *const mclipnode_t as
                                         *mut mclipnode_t,
                                 nummarksurfaces: 0,
                                 marksurfaces:
                                     0 as *const *mut msurface_t as
                                         *mut *mut msurface_t,
                                 hulls:
                                     [hull_t{clipnodes:
                                                 0 as *const mclipnode_t as
                                                     *mut mclipnode_t,
                                             planes:
                                                 0 as *const mplane_t as
                                                     *mut mplane_t,
                                             firstclipnode: 0,
                                             lastclipnode: 0,
                                             clip_mins: [0.; 3],
                                             clip_maxs: [0.; 3],}; 4],
                                 numtextures: 0,
                                 textures:
                                     0 as *const *mut texture_t as
                                         *mut *mut texture_t,
                                 visdata: 0 as *const byte as *mut byte,
                                 lightdata:
                                     0 as *const color24 as *mut color24,
                                 entities:
                                     0 as *const libc::c_char as
                                         *mut libc::c_char,
                                 cache:
                                     cache_user_t{data:
                                                      0 as *const libc::c_void
                                                          as
                                                          *mut libc::c_void,},};
                            256],
                    viewport: [0; 4],
                    ds:
                        client_draw_t{pSprite: 0 as *const model_t,
                                      scissor_x: 0,
                                      scissor_y: 0,
                                      scissor_width: 0,
                                      scissor_height: 0,
                                      scissor_test: false_0,
                                      adjust_size: false_0,
                                      renderMode: 0,
                                      cullMode: TRI_FRONT,
                                      textColor: [0; 4],
                                      spriteColor: [0; 4],
                                      triRGBA: [0.; 4],
                                      pCrosshair: 0 as *const model_t,
                                      rcCrosshair:
                                          wrect_t{left: 0,
                                                  right: 0,
                                                  top: 0,
                                                  bottom: 0,},
                                      rgbaCrosshair: [0; 4],},
                    fade:
                        screenfade_t{fadeSpeed: 0.,
                                     fadeEnd: 0.,
                                     fadeTotalEnd: 0.,
                                     fadeReset: 0.,
                                     fader: 0,
                                     fadeg: 0,
                                     fadeb: 0,
                                     fadealpha: 0,
                                     fadeFlags: 0,},
                    shake:
                        screen_shake_t{time: 0.,
                                       duration: 0.,
                                       amplitude: 0.,
                                       frequency: 0.,
                                       next_shake: 0.,
                                       offset: [0.; 3],
                                       angle: 0.,
                                       applied_offset: [0.; 3],
                                       applied_angle: 0.,},
                    centerPrint:
                        center_print_t{time: 0.,
                                       y: 0,
                                       lines: 0,
                                       message: [0; 2048],
                                       totalWidth: 0,
                                       totalHeight: 0,},
                    scrInfo:
                        SCREENINFO{iSize: 0,
                                   iWidth: 0,
                                   iHeight: 0,
                                   iFlags: 0,
                                   iCharHeight: 0,
                                   charWidths: [0; 256],},
                    overView:
                        ref_overview_t{origin: [0.; 3],
                                       rotated: false_0,
                                       xLeft: 0.,
                                       xRight: 0.,
                                       yTop: 0.,
                                       yBottom: 0.,
                                       zFar: 0.,
                                       zNear: 0.,
                                       flZoom: 0.,},
                    palette: [color24{r: 0, g: 0, b: 0,}; 256],
                    sprlist:
                        [cached_spritelist_t{szListName: [0; 64],
                                             pList:
                                                 0 as *const client_sprite_t
                                                     as *mut client_sprite_t,
                                             count: 0,}; 256],
                    titles:
                        0 as *const client_textmessage_t as
                            *mut client_textmessage_t,
                    numTitles: 0,
                    request_type: NET_REQUEST_CANCEL,
                    net_requests:
                        [net_request_t{resp:
                                           net_response_t{error: 0,
                                                          context: 0,
                                                          type_0: 0,
                                                          remote_address:
                                                              netadr_t{type_0:
                                                                           NA_UNUSED,
                                                                       ip:
                                                                           [0;
                                                                               4],
                                                                       ipx:
                                                                           [0;
                                                                               10],
                                                                       port:
                                                                           0,},
                                                          ping: 0.,
                                                          response:
                                                              0 as
                                                                  *const libc::c_void
                                                                  as
                                                                  *mut libc::c_void,},
                                       pfnFunc: None,
                                       timeout: 0.,
                                       timesend: 0.,
                                       flags: 0,}; 64],
                    master_request:
                        0 as *const net_request_t as *mut net_request_t,
                    free_efrags: 0 as *const efrag_t as *mut efrag_t,
                    viewent:
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
                    client_dll_uses_sdl: false_0,};
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn CL_Active() -> libc::c_int {
    return (cls.state as libc::c_uint ==
                ca_active as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_Initialized() -> qboolean {
    return cls.initialized;
}
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn CL_IsInGame() -> qboolean {
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        return true_0
    } // always active for dedicated servers
    if cl.background as libc::c_uint != 0 ||
           CL_GetMaxClients() > 1 as libc::c_int {
        return true_0
    } // always active for multiplayer or background map
    return (cls.key_dest as libc::c_uint ==
                key_game as libc::c_int as libc::c_uint) as libc::c_int as
               qboolean;
    // active if not menu or console
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsInMenu() -> qboolean {
    return (cls.key_dest as libc::c_uint ==
                key_menu as libc::c_int as libc::c_uint) as libc::c_int as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsInConsole() -> qboolean {
    return (cls.key_dest as libc::c_uint ==
                key_console as libc::c_int as libc::c_uint) as libc::c_int as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsIntermission() -> qboolean {
    return cl.intermission as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsPlaybackDemo() -> qboolean {
    return cls.demoplayback as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsRecordDemo() -> qboolean {
    return cls.demorecording;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsTimeDemo() -> qboolean { return cls.timedemo; }
#[no_mangle]
pub unsafe extern "C" fn CL_DisableVisibility() -> qboolean {
    return cls.envshot_disable_vis;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsBackgroundDemo() -> qboolean {
    return (cls.demoplayback != 0 && cls.demonum != -(1 as libc::c_int)) as
               libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsBackgroundMap() -> qboolean {
    return (cl.background as libc::c_uint != 0 && cls.demoplayback == 0) as
               libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn CL_Userinfo() -> *mut libc::c_char {
    return cls.userinfo.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn CL_IsDevOverviewMode() -> libc::c_int {
    if dev_overview.value > 0.0f32 {
        if host_developer.value != 0. || cls.spectator as libc::c_uint != 0 {
            return dev_overview.value as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*
===============
CL_CheckClientState

finalize connection process and begin new frame
with new cls.state
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CheckClientState() {
    // first update is the pre-final signon stage
    if (cls.state as libc::c_uint ==
            ca_connected as libc::c_int as libc::c_uint ||
            cls.state as libc::c_uint ==
                ca_validate as libc::c_int as libc::c_uint) &&
           cls.signon == 2 as libc::c_int {
        cls.state = ca_active;
        // get rid of loading plaque
        cls.changelevel = false_0; // changelevel is done
        cls.changedemo = false_0; // changedemo is done
        cl.first_frame = true_0; // first rendering frame
        SCR_MakeLevelShot(); // make levelshot if needs
        Cvar_SetValue(b"scr_loading\x00" as *const u8 as *const libc::c_char,
                      0.0f32); // reset progress bar
        Netchan_ReportFlow(&mut cls.netchan);
        Con_DPrintf(b"client connected at %.2f sec\n\x00" as *const u8 as
                        *const libc::c_char,
                    Sys_DoubleTime() - cls.timestart);
        if (cls.demoplayback != 0 ||
                cls.disable_servercount != cl.servercount) &&
               cl.video_prepped as libc::c_uint != 0 {
            SCR_EndLoadingPlaque();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetFragmentSize(mut unused: *mut libc::c_void,
                                            mut mode: fragsize_t)
 -> libc::c_int {
    if mode as libc::c_uint == FRAGSIZE_SPLIT as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if mode as libc::c_uint ==
           FRAGSIZE_UNRELIABLE as libc::c_int as libc::c_uint {
        return (0x20000 as libc::c_int +
                    (8 as libc::c_int + 2 as libc::c_int * 13 as libc::c_int)
                    + (16 as libc::c_int - 1 as libc::c_int)) /
                   16 as libc::c_int * 16 as libc::c_int
    }
    if Netchan_IsLocal(&mut cls.netchan) as u64 != 0 {
        return 64000 as libc::c_int
    }
    return (*cl_upmax).value as libc::c_int;
}
/*
=====================
CL_SignonReply

An svc_signonnum has been received, perform a client side setup
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SignonReply() {
    // g-cont. my favorite message :-)
    Con_Reportf(b"CL_SignonReply: %i\n\x00" as *const u8 as
                    *const libc::c_char, cls.signon);
    match cls.signon {
        1 => {
            CL_ServerCommand(true_0,
                             b"begin\x00" as *const u8 as
                                 *const libc::c_char);
            if host_developer.value >=
                   DEV_EXTENDED as libc::c_int as libc::c_float {
                Mem_PrintStats();
            }
        }
        2 => {
            if cl.proxy_redirect as libc::c_uint != 0 &&
                   cls.spectator as u64 == 0 {
                CL_Disconnect();
            }
            cl.proxy_redirect = false_0
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_LerpInterval() -> libc::c_float {
    return if (*cl_interp).value > 1.0f32 / (*cl_updaterate).value {
               (*cl_interp).value
           } else { (1.0f32) / (*cl_updaterate).value };
}
/*
===============
CL_LerpPoint

Determines the fraction between the last two messages that the objects
should be put at.
===============
*/
unsafe extern "C" fn CL_LerpPoint() -> libc::c_float {
    let mut frac: libc::c_float = 1.0f32;
    let mut server_frametime: libc::c_float =
        (cl.mtime[0 as libc::c_int as usize] -
             cl.mtime[1 as libc::c_int as usize]) as libc::c_float;
    if server_frametime == 0.0f32 || cls.timedemo as libc::c_uint != 0 {
        cl.time = cl.mtime[0 as libc::c_int as usize];
        return 1.0f32
    }
    if server_frametime > 0.1f32 {
        // dropped packet, or start of demo
        cl.mtime[1 as libc::c_int as usize] =
            cl.mtime[0 as libc::c_int as usize] - 0.1f32 as libc::c_double;
        server_frametime = 0.1f32
    }
    // for multiplayer
    if (*cl_interp).value > 0.001f32 {
        // manual lerp value (goldsrc mode)
        let mut td: libc::c_float =
            if 0.0f32 as libc::c_double >
                   cl.time - cl.mtime[0 as libc::c_int as usize] {
                0.0f32 as libc::c_double
            } else { (cl.time) - cl.mtime[0 as libc::c_int as usize] } as
                libc::c_float;
        frac = td / CL_LerpInterval()
    } else if server_frametime > 0.001f32 {
        // automatic lerp (classic mode)
        frac =
            ((cl.time - cl.mtime[1 as libc::c_int as usize]) /
                 server_frametime as libc::c_double) as libc::c_float
    }
    return frac;
}
/*
===============
CL_DriftInterpolationAmount

Drift interpolation value (this is used for server unlag system)
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DriftInterpolationAmount(mut goal: libc::c_int)
 -> libc::c_int {
    let mut fgoal: libc::c_float = 0.;
    let mut maxmove: libc::c_float = 0.;
    let mut diff: libc::c_float = 0.;
    let mut msec: libc::c_int = 0;
    fgoal = goal as libc::c_float / 1000.0f32;
    if fgoal != cl.local.interp_amount {
        maxmove = (host.frametime * 0.05f64) as libc::c_float;
        diff = fgoal - cl.local.interp_amount;
        diff =
            if diff >= -maxmove {
                if diff < maxmove { diff } else { maxmove }
            } else { -maxmove };
        cl.local.interp_amount += diff
    }
    msec = (cl.local.interp_amount * 1000.0f32) as libc::c_int;
    msec =
        if msec >= 0 as libc::c_int {
            if msec < 100 as libc::c_int { msec } else { 100 as libc::c_int }
        } else { 0 as libc::c_int };
    return msec;
}
/*
===============
CL_ComputeClientInterpolationAmount

Validate interpolation cvars, calc interpolation window
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ComputeClientInterpolationAmount(mut cmd:
                                                                 *mut usercmd_t) {
    let epsilon: libc::c_float =
        0.001f32; // to avoid float invalid comparision
    let mut min_interp: libc::c_float = 0.005f32;
    let mut max_interp: libc::c_float = 0.1f32;
    let mut interpolation_time: libc::c_float = 0.;
    if (*cl_updaterate).value < 10.0f32 {
        Con_Printf(b"cl_updaterate minimum is %f, resetting to default (20)\n\x00"
                       as *const u8 as *const libc::c_char,
                   10.0f32 as libc::c_double);
        Cvar_Reset(b"cl_updaterate\x00" as *const u8 as *const libc::c_char);
    }
    if (*cl_updaterate).value > 102.0f32 {
        Con_Printf(b"cl_updaterate clamped at maximum (%f)\n\x00" as *const u8
                       as *const libc::c_char, 102.0f32 as libc::c_double);
        Cvar_SetValue(b"cl_updaterate\x00" as *const u8 as
                          *const libc::c_char, 102.0f32);
    }
    if cls.spectator as u64 != 0 { max_interp = 0.2f32 }
    min_interp = 1.0f32 / (*cl_updaterate).value;
    interpolation_time = CL_LerpInterval();
    if (*cl_interp).value + epsilon < min_interp {
        Con_Printf(b"ex_interp forced up to %.1f msec\n\x00" as *const u8 as
                       *const libc::c_char,
                   (min_interp * 1000.0f32) as libc::c_double);
        Cvar_SetValue(b"ex_interp\x00" as *const u8 as *const libc::c_char,
                      min_interp);
    } else if (*cl_interp).value - epsilon > max_interp {
        Con_Printf(b"ex_interp forced down to %.1f msec\n\x00" as *const u8 as
                       *const libc::c_char,
                   (max_interp * 1000.0f32) as libc::c_double);
        Cvar_SetValue(b"ex_interp\x00" as *const u8 as *const libc::c_char,
                      max_interp);
    }
    interpolation_time =
        if interpolation_time >= min_interp {
            if interpolation_time < max_interp {
                interpolation_time
            } else { max_interp }
        } else { min_interp };
    (*cmd).lerp_msec =
        CL_DriftInterpolationAmount((interpolation_time *
                                         1000 as libc::c_int as libc::c_float)
                                        as libc::c_int) as libc::c_short;
}
/*
=================
CL_ComputePacketLoss

=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ComputePacketLoss() {
    let mut i: libc::c_int = 0;
    let mut frm: libc::c_int = 0;
    let mut frame: *mut frame_t = 0 as *mut frame_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut lost: libc::c_int = 0 as libc::c_int;
    if host.realtime < cls.packet_loss_recalc_time { return }
    // recalc every second
    cls.packet_loss_recalc_time = host.realtime + 1.0f64;
    // compuate packet loss
    i =
        cls.netchan.incoming_sequence.wrapping_sub(CL_UPDATE_BACKUP as
                                                       libc::c_uint).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)
            as libc::c_int;
    while i as libc::c_uint <= cls.netchan.incoming_sequence {
        frm = i;
        frame =
            &mut *cl.frames.as_mut_ptr().offset((frm &
                                                     CL_UPDATE_BACKUP -
                                                         1 as libc::c_int) as
                                                    isize) as *mut frame_t;
        if (*frame).receivedtime == -1.0f64 { lost += 1 }
        count += 1;
        i += 1
    }
    if count <= 0 as libc::c_int {
        cls.packet_loss = 0.0f32
    } else {
        cls.packet_loss =
            100.0f32 * lost as libc::c_float / count as libc::c_float
    };
}
/*
=================
CL_UpdateFrameLerp

=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateFrameLerp() {
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
           || cl.validsequence == 0 {
        return
    }
    // compute last interpolation amount
    cl.lerpFrac = CL_LerpPoint();
    cl.commands[(cls.netchan.outgoing_sequence.wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                     & (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint)
                    as usize].frame_lerp = cl.lerpFrac;
}
#[no_mangle]
pub unsafe extern "C" fn CL_FindInterpolatedAddAngle(mut t: libc::c_float,
                                                     mut frac:
                                                         *mut libc::c_float,
                                                     mut prev:
                                                         *mut *mut pred_viewangle_t,
                                                     mut next:
                                                         *mut *mut pred_viewangle_t) {
    let mut i: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut imod: libc::c_int = 0;
    let mut at: libc::c_float = 0.;
    imod = cl.angle_position - 1 as libc::c_int;
    i0 = imod + 1 as libc::c_int & 16 as libc::c_int - 1 as libc::c_int;
    i1 = imod + 0 as libc::c_int & 16 as libc::c_int - 1 as libc::c_int;
    if cl.predicted_angle[i0 as usize].starttime >= t {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int - 2 as libc::c_int {
            at =
                cl.predicted_angle[(imod &
                                        16 as libc::c_int - 1 as libc::c_int)
                                       as usize].starttime;
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
        &mut *cl.predicted_angle.as_mut_ptr().offset(i0 as isize) as
            *mut pred_viewangle_t;
    *prev =
        &mut *cl.predicted_angle.as_mut_ptr().offset(i1 as isize) as
            *mut pred_viewangle_t;
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
#[no_mangle]
pub unsafe extern "C" fn CL_ApplyAddAngle() {
    let mut prev: *mut pred_viewangle_t = 0 as *mut pred_viewangle_t;
    let mut next: *mut pred_viewangle_t = 0 as *mut pred_viewangle_t;
    let mut addangletotal: libc::c_float = 0.0f32;
    let mut amove: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.0f32;
    CL_FindInterpolatedAddAngle(cl.time as libc::c_float, &mut frac,
                                &mut prev, &mut next);
    if !prev.is_null() && !next.is_null() {
        addangletotal = (*prev).total + frac * ((*next).total - (*prev).total)
    } else { addangletotal = cl.prevaddangletotal }
    amove = addangletotal - cl.prevaddangletotal;
    // update input angles
    cl.viewangles[1 as libc::c_int as usize] += amove;
    // remember last total
    cl.prevaddangletotal = addangletotal;
}
/*
=======================================================================

CLIENT MOVEMENT COMMUNICATION

=======================================================================
*/
/*
===============
CL_ProcessShowTexturesCmds

navigate around texture atlas
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessShowTexturesCmds(mut cmd: *mut usercmd_t)
 -> qboolean {
    static mut oldbuttons: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut pressed: libc::c_int = 0;
    let mut released: libc::c_int = 0;
    if (*gl_showtextures).value == 0. || CL_IsDevOverviewMode() != 0 {
        return false_0
    }
    changed = oldbuttons ^ (*cmd).buttons as libc::c_int;
    pressed = changed & (*cmd).buttons as libc::c_int;
    released = changed & !((*cmd).buttons as libc::c_int);
    if released as libc::c_uint &
           ((1 as libc::c_uint) << 8 as libc::c_int |
                (1 as libc::c_uint) << 10 as libc::c_int) != 0 {
        Cvar_SetValue(b"r_showtextures\x00" as *const u8 as
                          *const libc::c_char,
                      (*gl_showtextures).value +
                          1 as libc::c_int as libc::c_float);
    }
    if released as libc::c_uint &
           ((1 as libc::c_uint) << 7 as libc::c_int |
                (1 as libc::c_uint) << 9 as libc::c_int) != 0 {
        Cvar_SetValue(b"r_showtextures\x00" as *const u8 as
                          *const libc::c_char,
                      if 1 as libc::c_int as libc::c_float >
                             (*gl_showtextures).value -
                                 1 as libc::c_int as libc::c_float {
                          1 as libc::c_int as libc::c_float
                      } else {
                          ((*gl_showtextures).value) -
                              1 as libc::c_int as libc::c_float
                      });
    }
    oldbuttons = (*cmd).buttons as libc::c_int;
    return true_0;
}
/*
===============
CL_ProcessOverviewCmds

Transform user movement into overview adjust
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessOverviewCmds(mut cmd: *mut usercmd_t)
 -> qboolean {
    let mut ov: *mut ref_overview_t =
        &mut clgame.overView; // to prevent disivion by zero
    let mut sign: libc::c_int = 1 as libc::c_int;
    let mut size: libc::c_float =
        world.size[((*ov).rotated as u64 == 0) as libc::c_int as usize] /
            world.size[(*ov).rotated as usize];
    let mut step: libc::c_float =
        ((2.0f32 / size) as libc::c_double * host.realframetime) as
            libc::c_float;
    let mut step2: libc::c_float = step * 100.0f32 * (2.0f32 / (*ov).flZoom);
    if CL_IsDevOverviewMode() == 0 || (*gl_showtextures).value != 0. {
        return false_0
    }
    if (*ov).flZoom < 0.0f32 { sign = -(1 as libc::c_int) }
    if (*cmd).upmove > 0.0f32 {
        (*ov).zNear += step
    } else if (*cmd).upmove < 0.0f32 { (*ov).zNear -= step }
    if (*cmd).buttons as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        (*ov).zFar += step
    } else if (*cmd).buttons as libc::c_uint &
                  (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        (*ov).zFar -= step
    }
    if (*cmd).buttons as libc::c_uint &
           (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        (*ov).origin[(*ov).rotated as usize] -= sign as libc::c_float * step2
    } else if (*cmd).buttons as libc::c_uint &
                  (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        (*ov).origin[(*ov).rotated as usize] += sign as libc::c_float * step2
    }
    if (*ov).rotated as u64 != 0 {
        if (*cmd).buttons as libc::c_uint &
               ((1 as libc::c_uint) << 8 as libc::c_int |
                    (1 as libc::c_uint) << 10 as libc::c_int) != 0 {
            (*ov).origin[0 as libc::c_int as usize] -=
                sign as libc::c_float * step2
        } else if (*cmd).buttons as libc::c_uint &
                      ((1 as libc::c_uint) << 7 as libc::c_int |
                           (1 as libc::c_uint) << 9 as libc::c_int) != 0 {
            (*ov).origin[0 as libc::c_int as usize] +=
                sign as libc::c_float * step2
        }
    } else if (*cmd).buttons as libc::c_uint &
                  ((1 as libc::c_uint) << 8 as libc::c_int |
                       (1 as libc::c_uint) << 10 as libc::c_int) != 0 {
        (*ov).origin[1 as libc::c_int as usize] +=
            sign as libc::c_float * step2
    } else if (*cmd).buttons as libc::c_uint &
                  ((1 as libc::c_uint) << 7 as libc::c_int |
                       (1 as libc::c_uint) << 9 as libc::c_int) != 0 {
        (*ov).origin[1 as libc::c_int as usize] -=
            sign as libc::c_float * step2
    }
    if (*cmd).buttons as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        (*ov).flZoom += step
    } else if (*cmd).buttons as libc::c_uint &
                  (1 as libc::c_uint) << 11 as libc::c_int != 0 {
        (*ov).flZoom -= step
    }
    if (*ov).flZoom == 0.0f32 { (*ov).flZoom = 0.0001f32 }
    return true_0;
}
/*
=================
CL_UpdateClientData

tell the client.dll about player origin, angles, fov, etc
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateClientData() {
    let mut cdat: client_data_t =
        client_data_t{origin: [0.; 3],
                      viewangles: [0.; 3],
                      iWeaponBits: 0,
                      fov: 0.,};
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return
    }
    memset(&mut cdat as *mut client_data_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<client_data_t>() as libc::c_ulong);
    cdat.viewangles[0 as libc::c_int as usize] =
        cl.viewangles[0 as libc::c_int as usize];
    cdat.viewangles[1 as libc::c_int as usize] =
        cl.viewangles[1 as libc::c_int as usize];
    cdat.viewangles[2 as libc::c_int as usize] =
        cl.viewangles[2 as libc::c_int as usize];
    cdat.origin[0 as libc::c_int as usize] =
        (*clgame.entities.offset(cl.viewentity as
                                     isize)).origin[0 as libc::c_int as
                                                        usize];
    cdat.origin[1 as libc::c_int as usize] =
        (*clgame.entities.offset(cl.viewentity as
                                     isize)).origin[1 as libc::c_int as
                                                        usize];
    cdat.origin[2 as libc::c_int as usize] =
        (*clgame.entities.offset(cl.viewentity as
                                     isize)).origin[2 as libc::c_int as
                                                        usize];
    cdat.iWeaponBits = cl.local.weapons;
    cdat.fov = cl.local.scr_fov;
    if clgame.dllFuncs.pfnUpdateClientData.expect("non-null function pointer")(&mut cdat,
                                                                               cl.time
                                                                                   as
                                                                                   libc::c_float)
           != 0 {
        // grab changes if successful
        cl.viewangles[0 as libc::c_int as usize] =
            cdat.viewangles[0 as libc::c_int as usize];
        cl.viewangles[1 as libc::c_int as usize] =
            cdat.viewangles[1 as libc::c_int as usize];
        cl.viewangles[2 as libc::c_int as usize] =
            cdat.viewangles[2 as libc::c_int as usize];
        cl.local.scr_fov = cdat.fov
    };
}
/*
=================
CL_CreateCmd
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CreateCmd() {
    let mut cmd: usercmd_t =
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
    let mut pcmd: *mut runcmd_t = 0 as *mut runcmd_t;
    let mut angles: vec3_t = [0.; 3];
    let mut active: qboolean = false_0;
    let mut input_override: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ms: libc::c_int = 0;
    if (cls.state as libc::c_uint) <
           ca_connected as libc::c_int as libc::c_uint ||
           cls.state as libc::c_uint ==
               ca_cinematic as libc::c_int as libc::c_uint {
        return
    }
    // store viewangles in case it's will be freeze
    angles[0 as libc::c_int as usize] =
        cl.viewangles[0 as libc::c_int as usize];
    angles[1 as libc::c_int as usize] =
        cl.viewangles[1 as libc::c_int as usize];
    angles[2 as libc::c_int as usize] =
        cl.viewangles[2 as libc::c_int as usize];
    ms =
        if host.frametime * 1000 as libc::c_int as libc::c_double >=
               1 as libc::c_int as libc::c_double {
            if (host.frametime * 1000 as libc::c_int as libc::c_double) <
                   255 as libc::c_int as libc::c_double {
                (host.frametime) * 1000 as libc::c_int as libc::c_double
            } else { 255 as libc::c_int as libc::c_double }
        } else { 1 as libc::c_int as libc::c_double } as libc::c_int;
    memset(&mut cmd as *mut usercmd_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    input_override = 0 as libc::c_int;
    CL_SetSolidEntities();
    CL_PushPMStates();
    CL_SetSolidPlayers(cl.playernum);
    // message we are constructing.
    i =
        (cls.netchan.outgoing_sequence &
             (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint) as
            libc::c_int;
    pcmd = &mut *cl.commands.as_mut_ptr().offset(i as isize) as *mut runcmd_t;
    (*pcmd).processedfuncs = false_0;
    if cls.demoplayback == 0 {
        (*pcmd).senttime = host.realtime;
        memset(&mut (*pcmd).cmd as *mut usercmd_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
        (*pcmd).receivedtime = -1.0f64;
        (*pcmd).heldback = false_0;
        (*pcmd).sendsize = 0 as libc::c_int
    }
    active =
        (cls.signon == 2 as libc::c_int && cl.paused as u64 == 0 &&
             cls.demoplayback == 0) as libc::c_int as qboolean;
    Platform_PreCreateMove();
    clgame.dllFuncs.CL_CreateMove.expect("non-null function pointer")(host.frametime
                                                                          as
                                                                          libc::c_float,
                                                                      &mut (*pcmd).cmd,
                                                                      active
                                                                          as
                                                                          libc::c_int);
    IN_EngineAppendMove(host.frametime as libc::c_float,
                        &mut (*pcmd).cmd as *mut usercmd_t as
                            *mut libc::c_void, active);
    CL_PopPMStates();
    if cls.demoplayback == 0 {
        CL_ComputeClientInterpolationAmount(&mut (*pcmd).cmd);
        (*pcmd).cmd.lightlevel = cl.local.light_level as byte;
        (*pcmd).cmd.msec = ms as byte
    }
    input_override =
        (input_override as libc::c_uint |
             CL_ProcessOverviewCmds(&mut (*pcmd).cmd) as libc::c_uint) as
            libc::c_int;
    input_override =
        (input_override as libc::c_uint |
             CL_ProcessShowTexturesCmds(&mut (*pcmd).cmd) as libc::c_uint) as
            libc::c_int;
    if cl.background as libc::c_uint != 0 && cls.demoplayback == 0 ||
           input_override != 0 || cls.changelevel as libc::c_uint != 0 {
        (*pcmd).cmd.viewangles[0 as libc::c_int as usize] =
            angles[0 as libc::c_int as usize];
        (*pcmd).cmd.viewangles[1 as libc::c_int as usize] =
            angles[1 as libc::c_int as usize];
        (*pcmd).cmd.viewangles[2 as libc::c_int as usize] =
            angles[2 as libc::c_int as usize];
        cl.viewangles[0 as libc::c_int as usize] =
            angles[0 as libc::c_int as usize];
        cl.viewangles[1 as libc::c_int as usize] =
            angles[1 as libc::c_int as usize];
        cl.viewangles[2 as libc::c_int as usize] =
            angles[2 as libc::c_int as usize];
        if cl.background as u64 == 0 {
            (*pcmd).cmd.msec = 0 as libc::c_int as byte
        }
    }
    // demo always have commands so don't overwrite them
    if cls.demoplayback == 0 { cl.cmd = &mut (*pcmd).cmd }
    // predict all unacknowledged movements
    CL_PredictMovement(false_0);
}
#[no_mangle]
pub unsafe extern "C" fn CL_WriteUsercmd(mut msg: *mut sizebuf_t,
                                         mut from: libc::c_int,
                                         mut to: libc::c_int) {
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
    let mut f: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut t: *mut usercmd_t = 0 as *mut usercmd_t;
    if from == -(1 as libc::c_int) {
        memset(&mut nullcmd as *mut usercmd_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
        f = &mut nullcmd
    } else { f = &mut (*cl.commands.as_mut_ptr().offset(from as isize)).cmd }
    t = &mut (*cl.commands.as_mut_ptr().offset(to as isize)).cmd;
    // write it into the buffer
    MSG_WriteDeltaUsercmd(msg, f, t);
}
/*
===================
CL_WritePacket

Create and send the command packet to the server
Including both the reliable commands and the usercmds
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WritePacket() {
    let mut buf: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut send_command: qboolean = false_0;
    let mut data: [byte; 8000] = [0; 8000];
    let mut i: libc::c_int = 0;
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut key: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut numbackup: libc::c_int = 2 as libc::c_int;
    let mut numcmds: libc::c_int = 0;
    let mut newcmds: libc::c_int = 0;
    let mut cmdnumber: libc::c_int = 0;
    // don't send anything if playing back a demo
    if cls.demoplayback != 0 ||
           (cls.state as libc::c_uint) <
               ca_connected as libc::c_int as libc::c_uint ||
           cls.state as libc::c_uint ==
               ca_cinematic as libc::c_int as libc::c_uint {
        return
    }
    CL_ComputePacketLoss();
    MSG_InitExt(&mut buf,
                b"ClientData\x00" as *const u8 as *const libc::c_char,
                data.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 8000]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // Determine number of backup commands to send along
    numbackup =
        if (*cl_cmdbackup).value >= 0 as libc::c_int as libc::c_float {
            if (*cl_cmdbackup).value <
                   ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_float {
                (*cl_cmdbackup).value
            } else {
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_float
            }
        } else { 0 as libc::c_int as libc::c_float } as libc::c_int;
    if cls.state as libc::c_uint ==
           ca_connected as libc::c_int as libc::c_uint {
        numbackup = 0 as libc::c_int
    }
    // clamp cmdrate
    if (*cl_cmdrate).value < 0.0f32 {
        Cvar_SetValue(b"cl_cmdrate\x00" as *const u8 as *const libc::c_char,
                      0.0f32);
    } else if (*cl_cmdrate).value > 100.0f32 {
        Cvar_SetValue(b"cl_cmdrate\x00" as *const u8 as *const libc::c_char,
                      100.0f32);
    }
    // Check to see if we can actually send this command
    // In single player, send commands as fast as possible
	// Otherwise, only send when ready and when not choking bandwidth
    if cl.maxclients == 1 as libc::c_int ||
           NET_IsLocalAddress(cls.netchan.remote_address) as libc::c_uint != 0
               && (*host_limitlocal).value == 0. {
        send_command = true_0
    }
    if host.realtime >= cls.nextcmdtime as libc::c_double &&
           Netchan_CanPacket(&mut cls.netchan, true_0) as libc::c_uint != 0 {
        send_command = true_0
    }
    if cl.send_reply as u64 != 0 {
        cl.send_reply = false_0;
        send_command = true_0
    }
    // spectator is not sending cmds to server
    if cls.spectator as libc::c_uint != 0 &&
           cls.state as libc::c_uint ==
               ca_active as libc::c_int as libc::c_uint &&
           cl.delta_sequence == cl.validsequence {
        if !(cls.demorecording as libc::c_uint != 0 &&
                 cls.demowaiting as libc::c_uint != 0) &&
               (cls.nextcmdtime + 1.0f32) as libc::c_double > host.realtime {
            return
        }
    } // always able to send right away
    if cls.netchan.outgoing_sequence.wrapping_sub(cls.netchan.incoming_acknowledged)
           >= (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint {
        if host.realtime - cls.netchan.last_received > 15.0f64 {
            Con_NPrintf(1 as libc::c_int,
                        b"^3Warning:^1 Connection Problem^7\n\x00" as
                            *const u8 as *const libc::c_char);
            cl.validsequence = 0 as libc::c_int
        }
    }
    if (*cl_nodelta).value != 0. { cl.validsequence = 0 as libc::c_int }
    if send_command as u64 != 0 {
        let mut outgoing_sequence: libc::c_int = 0;
        if (*cl_cmdrate).value > 0 as libc::c_int as libc::c_float {
            // clamped between 10 and 100 fps
            cls.nextcmdtime =
                (host.realtime +
                     (if 1.0f32 / (*cl_cmdrate).value >= 0.1f32 {
                          (if 1.0f32 / (*cl_cmdrate).value < 0.01f32 {
                               (1.0f32) / (*cl_cmdrate).value
                           } else { 0.01f32 })
                      } else { 0.1f32 }) as libc::c_double) as libc::c_float
        } else { cls.nextcmdtime = host.realtime as libc::c_float }
        if cls.lastoutgoingcommand == -(1 as libc::c_int) {
            outgoing_sequence = cls.netchan.outgoing_sequence as libc::c_int;
            cls.lastoutgoingcommand =
                cls.netchan.outgoing_sequence as libc::c_int
        } else {
            outgoing_sequence = cls.lastoutgoingcommand + 1 as libc::c_int
        }
        // begin a client move command
        MSG_WriteCmdExt(&mut buf, 2 as libc::c_int, NS_CLIENT,
                        0 as *const libc::c_char);
        // save the position for a checksum byte
        key = MSG_GetRealBytesWritten(&mut buf);
        MSG_WriteByte(&mut buf, 0 as libc::c_int);
        // write packet lossage percentation
        MSG_WriteByte(&mut buf, cls.packet_loss as libc::c_int);
        // say how many backups we'll be sending
        MSG_WriteByte(&mut buf, numbackup);
        // how many real commands have queued up
        newcmds =
            cls.netchan.outgoing_sequence.wrapping_sub(cls.lastoutgoingcommand
                                                           as libc::c_uint) as
                libc::c_int;
        // put an upper/lower bound on this
        newcmds =
            if newcmds >= 0 as libc::c_int {
                if newcmds <
                       (if cls.legacymode as libc::c_uint != 0 {
                            28 as libc::c_int
                        } else { 32 as libc::c_int }) {
                    newcmds
                } else if cls.legacymode as libc::c_uint != 0 {
                    28 as libc::c_int
                } else { 32 as libc::c_int }
            } else { 0 as libc::c_int };
        if cls.state as libc::c_uint ==
               ca_connected as libc::c_int as libc::c_uint {
            newcmds = 0 as libc::c_int
        }
        MSG_WriteByte(&mut buf, newcmds);
        numcmds = newcmds + numbackup;
        from = -(1 as libc::c_int);
        i = numcmds - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            cmdnumber =
                (cls.netchan.outgoing_sequence.wrapping_sub(i as libc::c_uint)
                     & (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint)
                    as libc::c_int;
            to = cmdnumber;
            CL_WriteUsercmd(&mut buf, from, to);
            from = to;
            if MSG_CheckOverflow(&mut buf) as u64 != 0 {
                Host_Error(b"CL_WritePacket: overflowed command buffer (%i bytes)\n\x00"
                               as *const u8 as *const libc::c_char,
                           8000 as libc::c_int);
            }
            i -= 1
        }
        // calculate a checksum over the move commands
        size = MSG_GetRealBytesWritten(&mut buf) - key - 1 as libc::c_int;
        *buf.pData.offset(key as isize) =
            CRC32_BlockSequence(buf.pData.offset(key as
                                                     isize).offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                                size,
                                cls.netchan.outgoing_sequence as libc::c_int);
        // message we are constructing.
        i =
            (cls.netchan.outgoing_sequence &
                 (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint) as
                libc::c_int;
        // determine if we need to ask for a new set of delta's.
        if cl.validsequence != 0 &&
               cls.state as libc::c_uint ==
                   ca_active as libc::c_int as libc::c_uint &&
               !(cls.demorecording as libc::c_uint != 0 &&
                     cls.demowaiting as libc::c_uint != 0) {
            cl.delta_sequence = cl.validsequence;
            MSG_WriteCmdExt(&mut buf, 4 as libc::c_int, NS_CLIENT,
                            0 as *const libc::c_char);
            MSG_WriteByte(&mut buf, cl.validsequence & 0xff as libc::c_int);
        } else {
            // request delta compression of entities
            cl.delta_sequence = -(1 as libc::c_int)
        }
        if MSG_CheckOverflow(&mut buf) as u64 != 0 {
            Host_Error(b"CL_WritePacket: overflowed command buffer (%i bytes)\n\x00"
                           as *const u8 as *const libc::c_char,
                       8000 as libc::c_int);
        }
        // remember outgoing command that we are sending
        cls.lastoutgoingcommand =
            cls.netchan.outgoing_sequence as libc::c_int;
        // update size counter for netgraph
        cl.commands[(cls.netchan.outgoing_sequence &
                         (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                             libc::c_uint) as usize].sendsize =
            MSG_GetNumBytesWritten(&mut buf);
        cl.commands[(cls.netchan.outgoing_sequence &
                         (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                             libc::c_uint) as usize].heldback = false_0;
        // composite the rest of the datagram..
        if MSG_GetNumBitsWritten(&mut cls.datagram) <=
               MSG_GetNumBitsLeft(&mut buf) {
            MSG_WriteBits(&mut buf,
                          MSG_GetData(&mut cls.datagram) as
                              *const libc::c_void,
                          MSG_GetNumBitsWritten(&mut cls.datagram));
        }
        MSG_Clear(&mut cls.datagram);
        // deliver the message (or update reliable)
        Netchan_TransmitBits(&mut cls.netchan,
                             MSG_GetNumBitsWritten(&mut buf),
                             MSG_GetData(&mut buf));
    } else {
        // mark command as held back so we'll send it next time
        cl.commands[(cls.netchan.outgoing_sequence &
                         (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                             libc::c_uint) as usize].heldback = true_0;
        // increment sequence number so we can detect that we've held back packets.
        cls.netchan.outgoing_sequence =
            cls.netchan.outgoing_sequence.wrapping_add(1)
    }
    if cls.demorecording as libc::c_uint != 0 && numbackup > 0 as libc::c_int
       {
        // Back up one because we've incremented outgoing_sequence each frame by 1 unit
        cmdnumber =
            (cls.netchan.outgoing_sequence.wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint) &
                 (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint) as
                libc::c_int;
        CL_WriteDemoUserCmd(cmdnumber);
    }
    // update download/upload slider.
    Netchan_UpdateProgress(&mut cls.netchan);
}
/*
=================
CL_SendCommand

Called every frame to builds and sends a command packet to the server.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SendCommand() {
    // we create commands even if a demo is playing,
    CL_CreateCmd();
    // clc_move, userinfo etc
    CL_WritePacket();
}
/*
==================
CL_BeginUpload_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_BeginUpload_f() {
    let mut name_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut custResource: resource_t =
        resource_t{szFileName: [0; 64],
                   type_0: t_sound,
                   nIndex: 0,
                   nDownloadSize: 0,
                   ucFlags: 0,
                   rgucMD5_hash: [0; 16],
                   playernum: 0,
                   rguc_reserved: [0; 32],
                   pNext: 0 as *const resource_s as *mut resource_s,
                   pPrev: 0 as *const resource_s as *mut resource_s,};
    let mut buf: *mut byte = 0 as *mut byte;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut md5: [byte; 16] = [0; 16];
    name_0 = Cmd_Argv(1 as libc::c_int);
    if if name_0.is_null() || *name_0 == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    if cl_allow_upload.value == 0. { return }
    if Q_strlen(name_0) != 36 as libc::c_int as libc::c_ulong ||
           Q_strnicmp(name_0, b"!MD5\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int) != 0 {
        Con_Printf(b"Ingoring upload of non-customization\n\x00" as *const u8
                       as *const libc::c_char);
        return
    }
    memset(&mut custResource as *mut resource_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<resource_t>() as libc::c_ulong);
    COM_HexConvert(name_0.offset(4 as libc::c_int as isize),
                   32 as libc::c_int, md5.as_mut_ptr());
    if HPAK_ResourceForHash(b"custom.hpk\x00" as *const u8 as
                                *const libc::c_char, md5.as_mut_ptr(),
                            &mut custResource) as u64 != 0 {
        if memcmp(md5.as_mut_ptr() as *const libc::c_void,
                  custResource.rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void, 16 as libc::c_int as libc::c_ulong)
               != 0 {
            Con_Reportf(b"Bogus data retrieved from %s, attempting to delete entry\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"custom.hpk\x00" as *const u8 as
                            *const libc::c_char);
            HPAK_RemoveLump(b"custom.hpk\x00" as *const u8 as
                                *const libc::c_char, &mut custResource);
            return
        }
        if HPAK_GetDataPointer(b"custom.hpk\x00" as *const u8 as
                                   *const libc::c_char, &mut custResource,
                               &mut buf, &mut size) as u64 != 0 {
            let mut md5_0: [byte; 16] = [0; 16];
            let mut ctx: MD5Context_t =
                MD5Context_t{buf: [0; 4], bits: [0; 2], in_0: [0; 16],};
            memset(&mut ctx as *mut MD5Context_t as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<MD5Context_t>() as libc::c_ulong);
            MD5Init(&mut ctx);
            MD5Update(&mut ctx, buf, size as uint);
            MD5Final(md5_0.as_mut_ptr(), &mut ctx);
            if memcmp(custResource.rgucMD5_hash.as_mut_ptr() as
                          *const libc::c_void,
                      md5_0.as_mut_ptr() as *const libc::c_void,
                      16 as libc::c_int as libc::c_ulong) != 0 {
                Con_Reportf(b"HPAK_AddLump called with bogus lump, md5 mismatch\n\x00"
                                as *const u8 as *const libc::c_char);
                Con_Reportf(b"Purported:  %s\n\x00" as *const u8 as
                                *const libc::c_char,
                            MD5_Print(custResource.rgucMD5_hash.as_mut_ptr()));
                Con_Reportf(b"Actual   :  %s\n\x00" as *const u8 as
                                *const libc::c_char,
                            MD5_Print(md5_0.as_mut_ptr()));
                Con_Reportf(b"Removing conflicting lump\n\x00" as *const u8 as
                                *const libc::c_char);
                HPAK_RemoveLump(b"custom.hpk\x00" as *const u8 as
                                    *const libc::c_char, &mut custResource);
                return
            }
        }
    }
    if !buf.is_null() && size > 0 as libc::c_int {
        Netchan_CreateFileFragmentsFromBuffer(&mut cls.netchan, name_0, buf,
                                              size);
        Netchan_FragSend(&mut cls.netchan);
        _Mem_Free(buf as *mut libc::c_void,
                  b"../engine/client/cl_main.c\x00" as *const u8 as
                      *const libc::c_char, 976 as libc::c_int);
    };
}
/*
==================
CL_Quit_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Quit_f() -> ! { CL_Disconnect(); Sys_Quit(); }
/*
================
CL_Drop

Called after an Host_Error was thrown
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Drop() {
    if cls.initialized as u64 == 0 { return }
    CL_Disconnect();
}
/*
=======================
CL_SendConnectPacket

We have gotten a challenge from the server, so try and
connect.
======================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SendConnectPacket() {
    let mut protinfo: [libc::c_char; 256] = [0; 256];
    let mut qport: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut adr: netadr_t =
        netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
    if NET_StringToAdr(cls.servername.as_mut_ptr(), &mut adr) as u64 == 0 {
        Con_Printf(b"CL_SendConnectPacket: bad server address\n\x00" as
                       *const u8 as *const libc::c_char);
        cls.connect_time = 0 as libc::c_int as libc::c_double;
        return
    }
    if adr.port as libc::c_int == 0 as libc::c_int {
        adr.port = MSG_BigShort(27015 as libc::c_int as libc::c_ushort)
    }
    qport =
        Cvar_VariableString(b"net_qport\x00" as *const u8 as
                                *const libc::c_char);
    key = ID_GetMD5();
    memset(protinfo.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    if adr.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        IN_LockInputDevices(false_0);
    } else {
        let mut input_devices: libc::c_int = 0;
        input_devices = IN_CollectInputDevices() as libc::c_int;
        IN_LockInputDevices(true_0);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"d\x00" as *const u8 as *const libc::c_char,
                            va(b"%d\x00" as *const u8 as *const libc::c_char,
                               input_devices),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"v\x00" as *const u8 as *const libc::c_char,
                            b"0.20\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"b\x00" as *const u8 as *const libc::c_char,
                            va(b"%d\x00" as *const u8 as *const libc::c_char,
                               Q_buildnum()),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"o\x00" as *const u8 as *const libc::c_char,
                            Q_buildos(),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"a\x00" as *const u8 as *const libc::c_char,
                            Q_buildarch(),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
    }
    if cls.legacymode as u64 != 0 {
        // set related userinfo keys
        if (*cl_dlmax).value >= 40000 as libc::c_int as libc::c_float ||
               (*cl_dlmax).value < 100 as libc::c_int as libc::c_float {
            Info_SetValueForKey(cls.userinfo.as_mut_ptr(),
                                b"cl_maxpacket\x00" as *const u8 as
                                    *const libc::c_char,
                                b"1400\x00" as *const u8 as
                                    *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong as libc::c_int);
        } else {
            Info_SetValueForKey(cls.userinfo.as_mut_ptr(),
                                b"cl_maxpacket\x00" as *const u8 as
                                    *const libc::c_char, (*cl_dlmax).string,
                                ::std::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong as libc::c_int);
        }
        if *Info_ValueForKey(cls.userinfo.as_mut_ptr(),
                             b"cl_maxpayload\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            Info_SetValueForKey(cls.userinfo.as_mut_ptr(),
                                b"cl_maxpayload\x00" as *const u8 as
                                    *const libc::c_char,
                                b"1000\x00" as *const u8 as
                                    *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong as libc::c_int);
        }
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"i\x00" as *const u8 as *const libc::c_char, key,
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, adr,
                               b"connect %i %i %i \"%s\" %d \"%s\"\n\x00" as
                                   *const u8 as *const libc::c_char,
                               48 as libc::c_int, Q_atoi(qport),
                               cls.challenge, cls.userinfo.as_mut_ptr(),
                               (1 as libc::c_uint) << 1 as libc::c_int,
                               protinfo.as_mut_ptr());
        Con_Printf(b"Trying to connect by legacy protocol\n\x00" as *const u8
                       as *const libc::c_char);
    } else {
        let mut extensions: libc::c_int =
            ((1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int;
        if (*cl_dlmax).value > 64000 as libc::c_int as libc::c_float ||
               (*cl_dlmax).value < 508 as libc::c_int as libc::c_float {
            Cvar_SetValue(b"cl_dlmax\x00" as *const u8 as *const libc::c_char,
                          1200 as libc::c_int as libc::c_float);
        }
        Info_RemoveKey(cls.userinfo.as_mut_ptr(),
                       b"cl_maxpacket\x00" as *const u8 as
                           *const libc::c_char);
        Info_RemoveKey(cls.userinfo.as_mut_ptr(),
                       b"cl_maxpayload\x00" as *const u8 as
                           *const libc::c_char);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"uuid\x00" as *const u8 as *const libc::c_char,
                            key,
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"qport\x00" as *const u8 as *const libc::c_char,
                            qport,
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Info_SetValueForKey(protinfo.as_mut_ptr(),
                            b"ext\x00" as *const u8 as *const libc::c_char,
                            va(b"%d\x00" as *const u8 as *const libc::c_char,
                               extensions),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as
                                libc::c_ulong as libc::c_int);
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, adr,
                               b"connect %i %i \"%s\" \"%s\"\n\x00" as
                                   *const u8 as *const libc::c_char,
                               49 as libc::c_int, cls.challenge,
                               protinfo.as_mut_ptr(),
                               cls.userinfo.as_mut_ptr());
        Con_Printf(b"Trying to connect by modern protocol\n\x00" as *const u8
                       as *const libc::c_char);
    }
    cls.timestart = Sys_DoubleTime();
}
/*
=================
CL_CheckForResend

Resend a connect message if the last one has timed out
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CheckForResend() {
    let mut adr: netadr_t =
        netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
    let mut res: libc::c_int = 0;
    if cls.internetservers_wait as u64 != 0 { CL_InternetServers_f(); }
    // if the local server is running and we aren't then connect
    if cls.state as libc::c_uint ==
           ca_disconnected as libc::c_int as libc::c_uint &&
           SV_Active() as libc::c_uint != 0 {
        cls.signon = 0 as libc::c_int;
        cls.state = ca_connecting;
        Q_strncpy(cls.servername.as_mut_ptr(),
                  b"localhost\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        cls.serveradr.type_0 = NA_LOOPBACK;
        // we don't need a challenge on the localhost
        CL_SendConnectPacket();
        return
    }
    // resend if we haven't gotten a reply yet
    if cls.demoplayback != 0 ||
           cls.state as libc::c_uint !=
               ca_connecting as libc::c_int as libc::c_uint {
        return
    }
    if cl_resend.value < 1.5f32 {
        Cvar_SetValue(b"cl_resend\x00" as *const u8 as *const libc::c_char,
                      1.5f32);
    } else if cl_resend.value > 20.0f32 {
        Cvar_SetValue(b"cl_resend\x00" as *const u8 as *const libc::c_char,
                      20.0f32);
    }
    if host.realtime - cls.connect_time < cl_resend.value as libc::c_double {
        return
    }
    res = NET_StringToAdrNB(cls.servername.as_mut_ptr(), &mut adr);
    if res == 0 { CL_Disconnect(); return }
    if res == 2 as libc::c_int {
        cls.connect_time = -(99999 as libc::c_int) as libc::c_double;
        return
    }
    // only retry so many times before failure.
    if cls.connect_retry >= 10 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 CL_CheckForResend: couldn\'t connected\n\x00"
                        as *const u8 as *const libc::c_char);
        CL_Disconnect();
        return
    }
    if adr.port as libc::c_int == 0 as libc::c_int {
        adr.port = MSG_BigShort(27015 as libc::c_int as libc::c_ushort)
    }
    if cls.connect_retry == 2 as libc::c_int {
        // too many fails use default connection method
        Con_Printf(b"hi-speed connection is failed, use default method\n\x00"
                       as *const u8 as
                       *const libc::c_char); // for retransmit requests
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, adr,
                               b"getchallenge\n\x00" as *const u8 as
                                   *const libc::c_char);
        Cvar_SetValue(b"cl_dlmax\x00" as *const u8 as *const libc::c_char,
                      508 as libc::c_int as libc::c_float);
        cls.connect_time = host.realtime;
        cls.connect_retry += 1;
        return
    }
    cls.serveradr = adr;
    cls.max_fragment_size =
        if 64000 as libc::c_int >
               cls.max_fragment_size >>
                   (if (1 as libc::c_int) < cls.connect_retry {
                        1 as libc::c_int
                    } else { cls.connect_retry }) {
            64000 as libc::c_int
        } else {
            (cls.max_fragment_size) >>
                (if (1 as libc::c_int) < cls.connect_retry {
                     1 as libc::c_int
                 } else { cls.connect_retry })
        };
    cls.connect_time = host.realtime;
    cls.connect_retry += 1;
    Con_Printf(b"Connecting to %s... [retry #%i]\n\x00" as *const u8 as
                   *const libc::c_char, cls.servername.as_mut_ptr(),
               cls.connect_retry);
    if cls.legacymode as u64 == 0 && cl_test_bandwidth.value != 0. {
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, adr,
                               b"bandwidth %i %i\n\x00" as *const u8 as
                                   *const libc::c_char, 49 as libc::c_int,
                               cls.max_fragment_size);
    } else {
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, adr,
                               b"getchallenge\n\x00" as *const u8 as
                                   *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_AddResource(mut type_0: resourcetype_t,
                                        mut name_0: *const libc::c_char,
                                        mut size: libc::c_int,
                                        mut bFatalIfMissing: qboolean,
                                        mut index: libc::c_int)
 -> *mut resource_t {
    let mut r: *mut resource_t =
        &mut *cl.resourcelist.as_mut_ptr().offset(cl.num_resources as isize)
            as *mut resource_t;
    if cl.num_resources >=
           1024 as libc::c_int + ((1 as libc::c_int) << 11 as libc::c_int) +
               ((1 as libc::c_int) << 10 as libc::c_int) +
               ((1 as libc::c_int) << 10 as libc::c_int) {
        Host_Error(b"Too many resources on client\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    cl.num_resources += 1;
    Q_strncpy((*r).szFileName.as_mut_ptr(), name_0,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*r).ucFlags =
        ((*r).ucFlags as libc::c_int |
             if bFatalIfMissing as libc::c_uint != 0 {
                 ((1 as libc::c_int)) << 0 as libc::c_int
             } else { 0 as libc::c_int }) as libc::c_uchar;
    (*r).nDownloadSize = size;
    (*r).nIndex = index;
    (*r).type_0 = type_0;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn CL_CreateResourceList() {
    let mut szFileName: [libc::c_char; 260] = [0; 260];
    let mut rgucMD5_hash: [byte; 16] = [0; 16];
    let mut pNewResource: *mut resource_t = 0 as *mut resource_t;
    let mut nSize: libc::c_int = 0;
    let mut fp: *mut file_t = 0 as *mut file_t;
    HPAK_FlushHostQueue();
    cl.num_resources = 0 as libc::c_int;
    Q_snprintf(szFileName.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong,
               b"logos/remapped.bmp\x00" as *const u8 as *const libc::c_char);
    memset(rgucMD5_hash.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
    fp =
        FS_Open(szFileName.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    if !fp.is_null() {
        MD5_HashFile(rgucMD5_hash.as_mut_ptr(), szFileName.as_mut_ptr(),
                     0 as *mut uint);
        nSize = FS_FileLength(fp) as libc::c_int;
        if nSize != 0 as libc::c_int {
            pNewResource =
                CL_AddResource(t_decal, szFileName.as_mut_ptr(), nSize,
                               false_0, 0 as libc::c_int);
            if !pNewResource.is_null() {
                (*pNewResource).ucFlags =
                    ((*pNewResource).ucFlags as libc::c_int |
                         (1 as libc::c_int) << 2 as libc::c_int) as
                        libc::c_uchar;
                memcpy((*pNewResource).rgucMD5_hash.as_mut_ptr() as
                           *mut libc::c_void,
                       rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
                       16 as libc::c_int as libc::c_ulong);
                HPAK_AddLump(false_0,
                             b"custom.hpk\x00" as *const u8 as
                                 *const libc::c_char, pNewResource,
                             0 as *mut byte, fp);
            }
        }
        FS_Close(fp);
    };
}
/*
================
CL_Connect_f

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Connect_f() {
    let mut server: string = [0; 256];
    let mut legacyconnect: qboolean = false_0;
    // hidden hint to connect by using legacy protocol
    if Cmd_Argc() == 3 as libc::c_int {
        legacyconnect =
            (Q_strncmp(Cmd_Argv(2 as libc::c_int),
                       b"legacy\x00" as *const u8 as *const libc::c_char,
                       99999 as libc::c_int) == 0) as libc::c_int as qboolean
    } else if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: connect <server>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Q_strncpy(server.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    // if running a local server, kill it and reissue
    if SV_Active() as u64 != 0 {
        Host_ShutdownServer(); // allow remote
    }
    NET_Config(true_0);
    Con_Printf(b"server %s\n\x00" as *const u8 as *const libc::c_char,
               server.as_mut_ptr());
    CL_Disconnect();
    // TESTTEST: a see console during connection
    UI_SetActiveMenu(false_0); // CL_CheckForResend() will fire immediately
    Key_SetKeyDest(key_console as
                       libc::c_int); // guess a we can establish connection with maximum fragment size
    cls.state = ca_connecting;
    cls.legacymode = legacyconnect;
    Q_strncpy(cls.servername.as_mut_ptr(), server.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    cls.connect_time = -(99999 as libc::c_int) as libc::c_double;
    cls.max_fragment_size = 64000 as libc::c_int;
    cls.connect_retry = 0 as libc::c_int;
    cls.spectator = false_0;
    cls.signon = 0 as libc::c_int;
}
/*
=====================
CL_Rcon_f

Send the rest of the command line over as
an unconnected command.
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Rcon_f() {
    let mut message: [libc::c_char; 1024] = [0; 1024]; // allow remote
    let mut to: netadr_t =
        netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
    let mut command: string = [0; 256];
    let mut i: libc::c_int = 0;
    if if (*rcon_client_password).string.is_null() ||
              *(*rcon_client_password).string == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Printf(b"You must set \'rcon_password\' before issuing an rcon command.\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    message[0 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[1 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[2 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[3 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    NET_Config(true_0);
    Q_strncat(message.as_mut_ptr(),
              b"rcon \x00" as *const u8 as *const libc::c_char,
              99999 as libc::c_int as size_t);
    Q_strncat(message.as_mut_ptr(), (*rcon_client_password).string,
              99999 as libc::c_int as size_t);
    Q_strncat(message.as_mut_ptr(),
              b" \x00" as *const u8 as *const libc::c_char,
              99999 as libc::c_int as size_t);
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        Cmd_Escape(command.as_mut_ptr(), Cmd_Argv(i),
                   ::std::mem::size_of::<string>() as libc::c_ulong as
                       libc::c_int);
        Q_strncat(message.as_mut_ptr(), command.as_mut_ptr(),
                  99999 as libc::c_int as size_t);
        Q_strncat(message.as_mut_ptr(),
                  b" \x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
        i += 1
    }
    if cls.state as libc::c_uint >=
           ca_connected as libc::c_int as libc::c_uint {
        to = cls.netchan.remote_address
    } else {
        if if (*rcon_address).string.is_null() || *(*rcon_address).string == 0
              {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            Con_Printf(b"You must either be connected or set the \'rcon_address\' cvar to issue rcon commands\n\x00"
                           as *const u8 as *const libc::c_char);
            return
        }
        NET_StringToAdr((*rcon_address).string, &mut to);
        if to.port as libc::c_int == 0 as libc::c_int {
            to.port = MSG_BigShort(27015 as libc::c_int as libc::c_ushort)
        }
    }
    NET_SendPacket(NS_CLIENT,
                   Q_strlen(message.as_mut_ptr()).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong),
                   message.as_mut_ptr() as *const libc::c_void, to);
}
/*
=====================
CL_ClearState

=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearState() {
    let mut i: libc::c_int = 0;
    CL_ClearResourceLists();
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        COM_ClearCustomizationList(&mut (*cl.players.as_mut_ptr().offset(i as
                                                                             isize)).customdata,
                                   false_0);
        i += 1
    }
    S_StopAllSounds(true_0);
    CL_ClearEffects();
    CL_FreeEdicts();
    CL_ClearPhysEnts();
    NetAPI_CancelAllRequests();
    // wipe the entire cl structure
    memset(&mut cl as *mut client_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<client_t>() as
               libc::c_ulong); // allow to drawing player in menu
    MSG_Clear(&mut cls.netchan.message); // because level starts from 1.0f second
    memset(&mut clgame.fade as *mut screenfade_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<screenfade_t>() as
               libc::c_ulong); // now all hud sprites are invalid
    memset(&mut clgame.shake as *mut screen_shake_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<screen_shake_t>() as libc::c_ulong);
    Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    cl.maxclients = 1 as libc::c_int;
    cl.mtime[1 as libc::c_int as usize] = 1.0f32 as libc::c_double;
    cl.mtime[0 as libc::c_int as usize] = cl.mtime[1 as libc::c_int as usize];
    cls.signon = 0 as libc::c_int;
    cl.resourcesneeded.pPrev = &mut cl.resourcesneeded;
    cl.resourcesneeded.pNext = cl.resourcesneeded.pPrev;
    cl.resourcesonhand.pPrev = &mut cl.resourcesonhand;
    cl.resourcesonhand.pNext = cl.resourcesonhand.pPrev;
    CL_CreateResourceList();
    CL_ClearSpriteTextures();
    cl.local.interp_amount = 0.1f32;
    cl.local.scr_fov = 90.0f32;
    Cvar_SetValue(b"scr_download\x00" as *const u8 as *const libc::c_char,
                  -1.0f32);
    Cvar_SetValue(b"scr_loading\x00" as *const u8 as *const libc::c_char,
                  0.0f32);
    host.allow_console = host.allow_console_init;
    HTTP_ClearCustomServers();
}
/*
=====================
CL_SendDisconnectMessage

Sends a disconnect message to the server
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SendDisconnectMessage() {
    let mut buf: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut data: [byte; 32] = [0; 32];
    if cls.state as libc::c_uint ==
           ca_disconnected as libc::c_int as libc::c_uint {
        return
    }
    MSG_InitExt(&mut buf,
                b"LastMessage\x00" as *const u8 as *const libc::c_char,
                data.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 32]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    MSG_WriteCmdExt(&mut buf, 3 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut buf,
                    b"disconnect\x00" as *const u8 as *const libc::c_char);
    if cls.netchan.remote_address.type_0 as u64 == 0 {
        cls.netchan.remote_address.type_0 = NA_LOOPBACK
    }
    // make sure message will be delivered
    Netchan_TransmitBits(&mut cls.netchan, MSG_GetNumBitsWritten(&mut buf),
                         MSG_GetData(&mut buf));
    Netchan_TransmitBits(&mut cls.netchan, MSG_GetNumBitsWritten(&mut buf),
                         MSG_GetData(&mut buf));
    Netchan_TransmitBits(&mut cls.netchan, MSG_GetNumBitsWritten(&mut buf),
                         MSG_GetData(&mut buf));
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetSplitSize() -> libc::c_int {
    let mut splitsize: libc::c_int = 0;
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if cls.extensions as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int == 0 {
        return 1400 as libc::c_int
    }
    splitsize = (*cl_dlmax).value as libc::c_int;
    if splitsize < 508 as libc::c_int || splitsize > 64000 as libc::c_int {
        Cvar_SetValue(b"cl_dlmax\x00" as *const u8 as *const libc::c_char,
                      1200 as libc::c_int as libc::c_float);
    }
    return (*cl_dlmax).value as libc::c_int;
}
/*
=====================
CL_Reconnect

build a request to reconnect client
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Reconnect(mut setup_netchan: qboolean) {
    if setup_netchan as u64 != 0 {
        Netchan_Setup(NS_CLIENT, &mut cls.netchan, net_from,
                      Cvar_VariableInteger(b"net_qport\x00" as *const u8 as
                                               *const libc::c_char),
                      0 as *mut libc::c_void,
                      Some(CL_GetFragmentSize as
                               unsafe extern "C" fn(_: *mut libc::c_void,
                                                    _: fragsize_t)
                                   -> libc::c_int));
        if cls.legacymode as u64 != 0 {
            let mut extensions: libc::c_uint =
                Q_atoi(Cmd_Argv(1 as libc::c_int)) as libc::c_uint;
            if extensions & (1 as libc::c_uint) << 1 as libc::c_int != 0 {
                // only enable incoming split for legacy mode
                cls.netchan.split = true_0;
                Con_Reportf(b"^2NET_EXT_SPLIT enabled^7 (packet sizes is %d/%d)\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*cl_dlmax).value as libc::c_int,
                            65536 as libc::c_int);
            }
        } else {
            cls.extensions =
                Q_atoi(Info_ValueForKey(Cmd_Argv(1 as libc::c_int),
                                        b"ext\x00" as *const u8 as
                                            *const libc::c_char));
            if cls.extensions as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                Con_Reportf(b"^2NET_EXT_SPLITSIZE enabled^7 (packet size is %d)\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*cl_dlmax).value as libc::c_int);
            }
        }
    } else {
        // clear channel and stuff
        Netchan_Clear(&mut cls.netchan); // not in the demo loop now
        MSG_Clear(&mut cls.netchan.message); // haven't gotten a valid frame update yet
    } // we'll request a full delta from the baseline
    cls.movienum =
        -(1 as libc::c_int); // we don't have a backed up cmd history yet
    cls.demonum = cls.movienum; // we can send a cmd right away
    cls.state = ca_connected;
    cls.signon = 0 as libc::c_int;
    CL_ServerCommand(true_0, b"new\x00" as *const u8 as *const libc::c_char);
    cl.validsequence = 0 as libc::c_int;
    cl.delta_sequence = -(1 as libc::c_int);
    cls.lastoutgoingcommand = -(1 as libc::c_int);
    cls.nextcmdtime = host.realtime as libc::c_float;
    cl.last_command_ack = -(1 as libc::c_int);
    CL_StartupDemoHeader();
}
/*
=====================
CL_Disconnect

Goes from a connected state to full screen console state
Sends a disconnect message to the server
This is also called on Host_Error, so it shouldn't cause any errors
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Disconnect() {
    cls.legacymode = false_0; // reset fragment size
    if cls.state as libc::c_uint ==
           ca_disconnected as libc::c_int as libc::c_uint {
        return
    }
    cls.connect_time = 0 as libc::c_int as libc::c_double;
    cls.changedemo = false_0;
    cls.max_fragment_size = 64000 as libc::c_int;
    CL_Stop_f();
    // send a disconnect message to the server
    CL_SendDisconnectMessage(); // get rid of loading plaque
    CL_ClearState();
    S_StopBackgroundTrack();
    SCR_EndLoadingPlaque();
    // clear the network channel, too.
    Netchan_Clear(&mut cls.netchan); // unlock input devices
    IN_LockInputDevices(false_0);
    cls.state = ca_disconnected;
    memset(&mut cls.serveradr as *mut netadr_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    cls.set_lastdemo = false_0;
    cls.connect_retry = 0 as libc::c_int;
    cls.signon = 0 as libc::c_int;
    // back to menu in non-developer mode
    if host_developer.value != 0. || CL_IsInMenu() as libc::c_uint != 0 {
        return
    }
    UI_SetActiveMenu(true_0);
}
#[no_mangle]
pub unsafe extern "C" fn CL_Disconnect_f() {
    if Host_IsLocalClient() as u64 != 0 {
        Host_EndGame(true_0,
                     b"disconnected from server\n\x00" as *const u8 as
                         *const libc::c_char);
    } else { CL_Disconnect(); };
}
#[no_mangle]
pub unsafe extern "C" fn CL_Crashed() {
    // already freed
    if host.status as libc::c_uint ==
           HOST_CRASHED as libc::c_int as libc::c_uint {
        return
    } // stop any demos
    if host.type_0 != HOST_NORMAL as libc::c_int as libc::c_uint { return }
    if cls.initialized as u64 == 0 { return }
    host.status = HOST_CRASHED;
    CL_Stop_f();
    // send a disconnect message to the server
    CL_SendDisconnectMessage();
    Host_WriteOpenGLConfig();
    Host_WriteConfig();
    // write config
}
/*
=================
CL_LocalServers_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LocalServers_f() {
    let mut adr: netadr_t =
        netadr_t{type_0: NA_UNUSED,
                 ip: [0; 4],
                 ipx: [0; 10],
                 port: 0,}; // allow remote
    Con_Printf(b"Scanning for servers on the local network area...\n\x00" as
                   *const u8 as *const libc::c_char);
    NET_Config(true_0);
    cls.legacyservercount = 0 as libc::c_int;
    // send a broadcast packet
    adr.type_0 = NA_BROADCAST;
    adr.port = MSG_BigShort(27015 as libc::c_int as libc::c_ushort);
    Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, adr,
                           b"info %i\x00" as *const u8 as *const libc::c_char,
                           49 as libc::c_int);
}
/*
=================
CL_InternetServers_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InternetServers_f() {
    let mut fullquery: [libc::c_char; 512] =
        *::std::mem::transmute::<&[u8; 512],
                                 &mut [libc::c_char; 512]>(b"1\xff0.0.0.0:0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // allow remote
    let mut info: *mut libc::c_char =
        fullquery.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 13]>()
                                          as libc::c_ulong as
                                          isize).offset(-(1 as libc::c_int as
                                                              isize)); // let master know about client version
    let remaining: size_t =
        (::std::mem::size_of::<[libc::c_char; 512]>() as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 13]>()
                                             as libc::c_ulong);
    NET_Config(true_0);
    Con_Printf(b"Scanning for servers on the internet area...\n\x00" as
                   *const u8 as *const libc::c_char);
    Info_SetValueForKey(info,
                        b"gamedir\x00" as *const u8 as *const libc::c_char,
                        (*SI.GameInfo).gamefolder.as_mut_ptr(),
                        remaining as libc::c_int);
    Info_SetValueForKey(info,
                        b"clver\x00" as *const u8 as *const libc::c_char,
                        b"0.20\x00" as *const u8 as *const libc::c_char,
                        remaining as libc::c_int);
    // Info_SetValueForKey( info, "nat", cl_nat->string, remaining );
    cls.legacyservercount = 0 as libc::c_int;
    cls.internetservers_wait =
        NET_SendToMasters(NS_CLIENT,
                          (::std::mem::size_of::<[libc::c_char; 13]>() as
                               libc::c_ulong).wrapping_add(Q_strlen(info)),
                          fullquery.as_mut_ptr() as *const libc::c_void);
    cls.internetservers_pending = true_0;
    if cls.internetservers_wait as u64 == 0 {
        // now we clearing the vgui request
        if !clgame.master_request.is_null() {
            memset(clgame.master_request as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<net_request_t>() as libc::c_ulong);
        }
        clgame.request_type = NET_REQUEST_GAMEUI
    };
}
/*
=================
CL_Reconnect_f

The server is changing levels
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Reconnect_f() {
    if cls.state as libc::c_uint ==
           ca_disconnected as libc::c_int as libc::c_uint {
        return
    } // fire immediately
    S_StopAllSounds(true_0); // not in the demo loop now
    if cls.state as libc::c_uint ==
           ca_connected as libc::c_int as libc::c_uint {
        CL_Reconnect(false_0);
        return
    }
    if if cls.servername.as_mut_ptr().is_null() ||
              *cls.servername.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        if cls.state as libc::c_uint >=
               ca_connected as libc::c_int as libc::c_uint {
            CL_Disconnect();
        }
        cls.connect_time = -(99999 as libc::c_int) as libc::c_double;
        cls.movienum = -(1 as libc::c_int);
        cls.demonum = cls.movienum;
        cls.state = ca_connecting;
        cls.signon = 0 as libc::c_int;
        Con_Printf(b"reconnecting...\n\x00" as *const u8 as
                       *const libc::c_char);
    };
}
/*
=================
CL_FixupColorStringsForInfoString

all the keys and values must be ends with ^7
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FixupColorStringsForInfoString(mut in_0:
                                                               *const libc::c_char,
                                                           mut out:
                                                               *mut libc::c_char) {
    let mut hasPrefix: qboolean = false_0;
    let mut endOfKeyVal: qboolean = false_0;
    let mut color: libc::c_int = 7 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    if *in_0 as libc::c_int == '\\' as i32 {
        let fresh0 = in_0;
        in_0 = in_0.offset(1);
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = *fresh0;
        count += 1
    }
    while *in_0 as libc::c_int != 0 && count < 256 as libc::c_int {
        if !in_0.is_null() && *in_0 as libc::c_int == '^' as i32 &&
               *in_0.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
               *in_0.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
               *in_0.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            color =
                *in_0.offset(1 as libc::c_int as isize) as libc::c_int -
                    '0' as i32 & 7 as libc::c_int
        }
        // color the not reset while end of key (or value) was found!
        if *in_0 as libc::c_int == '\\' as i32 && color != 7 as libc::c_int {
            if !out.offset(-(2 as libc::c_int as isize)).is_null() &&
                   *out.offset(-(2 as libc::c_int as isize)) as libc::c_int ==
                       '^' as i32 &&
                   *out.offset(-(2 as libc::c_int as
                                     isize)).offset(1 as libc::c_int as isize)
                       as libc::c_int != 0 &&
                   *out.offset(-(2 as libc::c_int as
                                     isize)).offset(1 as libc::c_int as isize)
                       as libc::c_int >= '0' as i32 &&
                   *out.offset(-(2 as libc::c_int as
                                     isize)).offset(1 as libc::c_int as isize)
                       as libc::c_int <= '9' as i32 {
                *out.offset(-(1 as libc::c_int as isize)) =
                    '7' as i32 as libc::c_char
            } else {
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = '^' as i32 as libc::c_char;
                let fresh3 = out;
                out = out.offset(1);
                *fresh3 = '7' as i32 as libc::c_char;
                count += 2 as libc::c_int
            }
            color = 7 as libc::c_int
        }
        let fresh4 = in_0;
        in_0 = in_0.offset(1);
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 = *fresh4;
        count += 1
    }
    // check the remaining value
    if color != 7 as libc::c_int {
        // if the ends with another color rewrite it
        if !out.offset(-(2 as libc::c_int as isize)).is_null() &&
               *out.offset(-(2 as libc::c_int as isize)) as libc::c_int ==
                   '^' as i32 &&
               *out.offset(-(2 as libc::c_int as
                                 isize)).offset(1 as libc::c_int as isize) as
                   libc::c_int != 0 &&
               *out.offset(-(2 as libc::c_int as
                                 isize)).offset(1 as libc::c_int as isize) as
                   libc::c_int >= '0' as i32 &&
               *out.offset(-(2 as libc::c_int as
                                 isize)).offset(1 as libc::c_int as isize) as
                   libc::c_int <= '9' as i32 {
            *out.offset(-(1 as libc::c_int as isize)) =
                '7' as i32 as libc::c_char
        } else {
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = '^' as i32 as libc::c_char;
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = '7' as i32 as libc::c_char;
            count += 2 as libc::c_int
        }
    }
    *out = '\u{0}' as i32 as libc::c_char;
}
/*
=================
CL_ParseStatusMessage

Handle a reply from a info
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseStatusMessage(mut from: netadr_t,
                                               mut msg: *mut sizebuf_t) {
    static mut infostring: [libc::c_char; 264] = [0; 264];
    let mut s: *mut libc::c_char = MSG_ReadStringExt(msg, false_0);
    let mut i: libc::c_int = 0;
    CL_FixupColorStringsForInfoString(s, infostring.as_mut_ptr());
    if !Q_strstr(infostring.as_mut_ptr(),
                 b"wrong version\x00" as *const u8 as
                     *const libc::c_char).is_null() {
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, from,
                               b"info %i\x00" as *const u8 as
                                   *const libc::c_char, 48 as libc::c_int);
        Con_Printf(b"^1Server^7: %s, Info: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_AdrToString(from),
                   infostring.as_mut_ptr());
        if cls.legacyservercount < 256 as libc::c_int {
            let fresh8 = cls.legacyservercount;
            cls.legacyservercount = cls.legacyservercount + 1;
            cls.legacyservers[fresh8 as usize] = from
        }
        return
    }
    if if Info_ValueForKey(infostring.as_mut_ptr(),
                           b"gamedir\x00" as *const u8 as
                               *const libc::c_char).is_null() ||
              *Info_ValueForKey(infostring.as_mut_ptr(),
                                b"gamedir\x00" as *const u8 as
                                    *const libc::c_char) == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Printf(b"^1Server^7: %s, Info: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_AdrToString(from),
                   infostring.as_mut_ptr());
        return
        // unsupported proto
    }
    i = 0 as libc::c_int;
    while i < cls.legacyservercount {
        if NET_CompareAdr(cls.legacyservers[i as usize], from) as u64 != 0 {
            Info_SetValueForKey(infostring.as_mut_ptr(),
                                b"legacy\x00" as *const u8 as
                                    *const libc::c_char,
                                b"1\x00" as *const u8 as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 264]>()
                                    as libc::c_ulong as libc::c_int);
            Con_Print(b"Legacy: \x00" as *const u8 as *const libc::c_char);
            break ;
        } else { i += 1 }
    }
    // more info about servers
    Con_Printf(b"^2Server^7: %s, Game: %s\n\x00" as *const u8 as
                   *const libc::c_char, NET_AdrToString(from),
               Info_ValueForKey(infostring.as_mut_ptr(),
                                b"gamedir\x00" as *const u8 as
                                    *const libc::c_char));
    UI_AddServerToList(from, infostring.as_mut_ptr());
}
/*
=================
CL_ParseNETInfoMessage

Handle a reply from a netinfo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseNETInfoMessage(mut from: netadr_t,
                                                mut msg: *mut sizebuf_t,
                                                mut s: *const libc::c_char) {
    let mut nr: *mut net_request_t =
        0 as *mut net_request_t; // fetching infostring
    static mut infostring: [libc::c_char; 264] = [0; 264];
    let mut i: libc::c_int = 0;
    let mut context: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut errorBits: libc::c_int = 0 as libc::c_int;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    context = Q_atoi(Cmd_Argv(1 as libc::c_int));
    type_0 = Q_atoi(Cmd_Argv(2 as libc::c_int));
    while *s as libc::c_int != '\\' as i32 { s = s.offset(1) }
    // check for errors
    val =
        Info_ValueForKey(s,
                         b"neterror\x00" as *const u8 as *const libc::c_char);
    if Q_strnicmp(val, b"protocol\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        errorBits = errorBits | (1 as libc::c_int) << 1 as libc::c_int
    } else if Q_strnicmp(val,
                         b"undefined\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
        errorBits = errorBits | (1 as libc::c_int) << 2 as libc::c_int
    }
    CL_FixupColorStringsForInfoString(s, infostring.as_mut_ptr());
    // find a request with specified context
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        nr =
            &mut *clgame.net_requests.as_mut_ptr().offset(i as isize) as
                *mut net_request_t;
        if (*nr).resp.context == context && (*nr).resp.type_0 == type_0 {
            // setup the answer
            (*nr).resp.response =
                infostring.as_mut_ptr() as
                    *mut libc::c_void; // misc error bits
            (*nr).resp.remote_address = from; // done
            (*nr).resp.error = 0 as libc::c_int;
            (*nr).resp.ping = host.realtime - (*nr).timesend;
            if (*nr).timeout <= host.realtime {
                (*nr).resp.error =
                    (*nr).resp.error | (1 as libc::c_int) << 0 as libc::c_int
            }
            (*nr).resp.error = (*nr).resp.error | errorBits;
            (*nr).pfnFunc.expect("non-null function pointer")(&mut (*nr).resp);
            if (*nr).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
                memset(nr as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<net_request_t>() as
                           libc::c_ulong);
            }
            return
        }
        i += 1
    };
}
/*
=================
CL_ProcessNetRequests

check for timeouts
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessNetRequests() {
    let mut nr: *mut net_request_t = 0 as *mut net_request_t;
    let mut i: libc::c_int = 0;
    // find a request with specified context
    i = 0 as libc::c_int; // not used
    while i < 64 as libc::c_int {
        nr =
            &mut *clgame.net_requests.as_mut_ptr().offset(i as isize) as
                *mut net_request_t;
        if !(*nr).pfnFunc.is_none() {
            if (*nr).timeout <= host.realtime {
                // setup the answer
                (*nr).resp.error =
                    (*nr).resp.error | (1 as libc::c_int) << 0 as libc::c_int;
                (*nr).resp.ping = host.realtime - (*nr).timesend;
                (*nr).pfnFunc.expect("non-null function pointer")(&mut (*nr).resp);
                memset(nr as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<net_request_t>() as
                           libc::c_ulong);
                // done
            }
        }
        i += 1
    };
}
//===================================================================
/*
===============
CL_SetupOverviewParams

Get initial overview values
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetupOverviewParams() {
    let mut ov: *mut ref_overview_t = &mut clgame.overView;
    let mut mapAspect: libc::c_float = 0.;
    let mut screenAspect: libc::c_float = 0.;
    let mut aspect: libc::c_float = 0.;
    (*ov).rotated =
        if world.size[1 as libc::c_int as usize] <=
               world.size[0 as libc::c_int as usize] {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    // calculate nearest aspect
    mapAspect =
        world.size[((*ov).rotated as u64 == 0) as libc::c_int as usize] /
            world.size[(*ov).rotated as usize];
    screenAspect =
        refState.width as libc::c_float / refState.height as libc::c_float;
    aspect = if mapAspect > screenAspect { mapAspect } else { screenAspect };
    (*ov).zNear = world.maxs[2 as libc::c_int as usize];
    (*ov).zFar = world.mins[2 as libc::c_int as usize];
    (*ov).flZoom = 8192.0f32 / world.size[(*ov).rotated as usize] / aspect;
    (*ov).origin[0 as libc::c_int as usize] =
        (world.mins[0 as libc::c_int as usize] +
             world.maxs[0 as libc::c_int as usize]) * 0.5f32;
    (*ov).origin[1 as libc::c_int as usize] =
        (world.mins[1 as libc::c_int as usize] +
             world.maxs[1 as libc::c_int as usize]) * 0.5f32;
    (*ov).origin[2 as libc::c_int as usize] =
        (world.mins[2 as libc::c_int as usize] +
             world.maxs[2 as libc::c_int as usize]) * 0.5f32;
    memset(&mut cls.spectator_state as *mut local_state_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<local_state_t>() as libc::c_ulong);
    if cls.spectator as u64 != 0 {
        cls.spectator_state.playerstate.friction =
            1 as libc::c_int as libc::c_float;
        cls.spectator_state.playerstate.gravity =
            1 as libc::c_int as libc::c_float;
        cls.spectator_state.playerstate.number =
            cl.playernum + 1 as libc::c_int;
        cls.spectator_state.playerstate.usehull = 1 as libc::c_int;
        cls.spectator_state.playerstate.movetype = 8 as libc::c_int;
        cls.spectator_state.client.maxspeed =
            clgame.movevars.spectatormaxspeed
    };
}
/*
=================
CL_IsFromConnectingServer

Used for connectionless packets, when netchan may not be ready.
=================
*/
unsafe extern "C" fn CL_IsFromConnectingServer(mut from: netadr_t)
 -> qboolean {
    return (NET_IsLocalAddress(from) as libc::c_uint != 0 ||
                NET_CompareAdr(cls.serveradr, from) as libc::c_uint != 0) as
               libc::c_int as qboolean;
}
/*
=================
CL_ConnectionlessPacket

Responses to broadcasts, etc
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ConnectionlessPacket(mut from: netadr_t,
                                                 mut msg: *mut sizebuf_t) {
    let mut args: *mut libc::c_char = 0 as *mut libc::c_char; // skip the -1
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut len: libc::c_int =
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as
            libc::c_int;
    let mut dataoffset: libc::c_int = 0 as libc::c_int;
    let mut servadr: netadr_t =
        netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
    MSG_Clear(msg);
    MSG_ReadLong(msg);
    args = MSG_ReadStringExt(msg, true_0);
    Cmd_TokenizeString(args);
    c = Cmd_Argv(0 as libc::c_int);
    Con_Reportf(b"CL_ConnectionlessPacket: %s : %s\n\x00" as *const u8 as
                    *const libc::c_char, NET_AdrToString(from), c);
    // server connection
    if Q_strncmp(c, b"client_connect\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        if CL_IsFromConnectingServer(from) as u64 == 0 { return }
        if cls.state as libc::c_uint ==
               ca_connected as libc::c_int as libc::c_uint {
            Con_DPrintf(b"^1Error:^7 dup connect received. ignored\n\x00" as
                            *const u8 as *const libc::c_char);
            return
        }
        CL_Reconnect(true_0);
        UI_SetActiveMenu(cl.background);
    } else if Q_strncmp(c, b"info\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        // server responding to a status broadcast
        CL_ParseStatusMessage(from, msg);
    } else if Q_strncmp(c, b"netinfo\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        // server responding to a status broadcast
        CL_ParseNETInfoMessage(from, msg, args);
    } else if Q_strncmp(c, b"cmd\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        // remote command from gui front end
        if NET_IsLocalAddress(from) as u64 == 0 {
            Con_Printf(b"Command packet from remote host. Ignored.\n\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
        SDL_ShowWindow(host.hWnd as *mut SDL_Window);
        args = MSG_ReadStringExt(msg, false_0);
        Cbuf_AddText(args);
        Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
    } else if Q_strncmp(c, b"print\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        // print command from somewhere
        Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   MSG_ReadStringExt(msg, false_0));
    } else if Q_strncmp(c,
                        b"testpacket\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        let mut recv_buf: [byte; 65535] = [0; 65535];
        let mut crcValue: dword = 0;
        let mut realsize: libc::c_int = 0;
        let mut crcValue2: dword = 0 as libc::c_int as dword;
        if CL_IsFromConnectingServer(from) as u64 == 0 { return }
        crcValue = MSG_ReadLong(msg) as dword;
        realsize = MSG_GetMaxBytes(msg) - MSG_GetNumBytesWritten(msg);
        if cls.max_fragment_size != MSG_GetMaxBytes(msg) {
            if cls.connect_retry >= 5 as libc::c_int {
                // too many fails use default connection method
                Con_Printf(b"hi-speed connection is failed, use default method\n\x00"
                               as *const u8 as *const libc::c_char);
                Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, from,
                                       b"getchallenge\n\x00" as *const u8 as
                                           *const libc::c_char);
                Cvar_SetValue(b"cl_dlmax\x00" as *const u8 as
                                  *const libc::c_char,
                              1200 as libc::c_int as libc::c_float);
                cls.connect_time = host.realtime;
                return
            }
            // just wait for a next responce
            cls.connect_time = -(99999 as libc::c_int) as libc::c_double;
            return
        }
        // if we waiting more than cl_timeout or packet was trashed
        // reading test buffer
        MSG_ReadBytes(msg, recv_buf.as_mut_ptr() as *mut libc::c_void,
                      realsize);
        // procssing the CRC
        CRC32_ProcessBuffer(&mut crcValue2,
                            recv_buf.as_mut_ptr() as *const libc::c_void,
                            realsize);
        if crcValue == crcValue2 {
            // packet was sucessfully delivered, adjust the fragment size and get challenge
            Con_DPrintf(b"CRC %x is matched, get challenge, fragment size %d\n\x00"
                            as *const u8 as *const libc::c_char, crcValue,
                        cls.max_fragment_size);
            Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, from,
                                   b"getchallenge\n\x00" as *const u8 as
                                       *const libc::c_char);
            Cvar_SetValue(b"cl_dlmax\x00" as *const u8 as *const libc::c_char,
                          cls.max_fragment_size as libc::c_float);
            cls.connect_time = host.realtime
        } else {
            if cls.connect_retry >= 5 as libc::c_int {
                // too many fails use default connection method
                Con_Printf(b"hi-speed connection is failed, use default method\n\x00"
                               as *const u8 as *const libc::c_char);
                Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, from,
                                       b"getchallenge\n\x00" as *const u8 as
                                           *const libc::c_char);
                Cvar_SetValue(b"cl_dlmax\x00" as *const u8 as
                                  *const libc::c_char,
                              508 as libc::c_int as libc::c_float);
                cls.connect_time = host.realtime;
                return
            }
            Con_Printf(b"got testpacket, CRC mismatched 0x%08x should be 0x%08x, trying next fragment size %d\n\x00"
                           as *const u8 as *const libc::c_char, crcValue2,
                       crcValue, cls.max_fragment_size >> 1 as libc::c_int);
            // trying the next size of packet
            cls.connect_time = -(99999 as libc::c_int) as libc::c_double
        }
    } else if Q_strncmp(c, b"ping\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        // ping from somewhere
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, from,
                               b"ack\x00" as *const u8 as
                                   *const libc::c_char);
    } else if Q_strncmp(c,
                        b"challenge\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        if CL_IsFromConnectingServer(from) as u64 == 0 { return }
        // challenge from the server we are connecting to
        cls.challenge = Q_atoi(Cmd_Argv(1 as libc::c_int));
        CL_SendConnectPacket();
        return
    } else {
        if Q_strncmp(c, b"echo\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            if CL_IsFromConnectingServer(from) as u64 == 0 { return }
            // echo request from server
            Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, from,
                                   b"%s\x00" as *const u8 as
                                       *const libc::c_char,
                                   Cmd_Argv(1 as libc::c_int));
        } else if Q_strncmp(c,
                            b"disconnect\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            if CL_IsFromConnectingServer(from) as u64 == 0 { return }
            // a disconnect message from the server, which will happen if the server
		// dropped the connection but it is still getting packets from us
            CL_Disconnect_f();
            if NET_CompareAdr(from, cls.legacyserver) as u64 != 0 {
                Cbuf_AddText(va(b"connect %s legacy\n\x00" as *const u8 as
                                    *const libc::c_char,
                                NET_AdrToString(from)));
                memset(&mut cls.legacyserver as *mut netadr_t as
                           *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
            }
        } else if Q_strncmp(c,
                            b"errormsg\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            if CL_IsFromConnectingServer(from) as u64 == 0 { return }
            args = MSG_ReadStringExt(msg, false_0);
            if Q_strncmp(args,
                         b"Server uses protocol version 48.\n\x00" as
                             *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
                cls.legacyserver = from
            } else {
                if UI_IsVisible() as u64 != 0 {
                    UI_ShowMessageBox(va(b"^3Server message^7\n%s\x00" as
                                             *const u8 as *const libc::c_char,
                                         args));
                }
                Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                           args);
            }
        } else if Q_strncmp(c,
                            b"updatemsg\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            // got an update message from master server
		// show update dialog from menu
            let mut adr: netadr_t =
                netadr_t{type_0: NA_UNUSED,
                         ip: [0; 4],
                         ipx: [0; 10],
                         port: 0,};
            let mut preferStore: qboolean = true_0;
            if Q_strncmp(Cmd_Argv(1 as libc::c_int),
                         b"nostore\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
                preferStore = false_0
            }
            // trust only hardcoded master server
            if NET_StringToAdr(b"ms.xash.su:27010\x00" as *const u8 as
                                   *const libc::c_char, &mut adr) as u64 != 0
               {
                if NET_CompareAdr(from, adr) as u64 != 0 {
                    UI_ShowUpdateDialog(preferStore);
                }
            } else {
                // in case we don't have master anymore
                UI_ShowUpdateDialog(preferStore);
            }
        } else if Q_strncmp(c, b"f\x00" as *const u8 as *const libc::c_char,
                            99999 as libc::c_int) == 0 {
            // serverlist got from masterserver
            while MSG_GetNumBitsLeft(msg) > 8 as libc::c_int {
                MSG_ReadBytes(msg,
                              servadr.ip.as_mut_ptr() as *mut libc::c_void,
                              ::std::mem::size_of::<[libc::c_uchar; 4]>() as
                                  libc::c_ulong as
                                  libc::c_int); // 4 bytes for IP
                servadr.port =
                    MSG_ReadShort(msg) as libc::c_ushort; // 2 bytes for Port
                servadr.type_0 = NA_IP;
                // list is ends here
                if servadr.port == 0 {
                    if clgame.request_type as libc::c_uint ==
                           NET_REQUEST_CLIENT as libc::c_int as libc::c_uint
                           && !clgame.master_request.is_null() {
                        let mut nr: *mut net_request_t =
                            clgame.master_request;
                        let mut list: *mut net_adrlist_t =
                            0 as *mut net_adrlist_t;
                        let mut prev: *mut *mut net_adrlist_t =
                            0 as *mut *mut net_adrlist_t;
                        // setup the answer
                        (*nr).resp.remote_address = from;
                        (*nr).resp.error = 0 as libc::c_int;
                        (*nr).resp.ping = host.realtime - (*nr).timesend;
                        if (*nr).timeout <= host.realtime {
                            (*nr).resp.error =
                                (*nr).resp.error |
                                    (1 as libc::c_int) << 0 as libc::c_int
                        }
                        Con_Printf(b"serverlist call: %s\n\x00" as *const u8
                                       as *const libc::c_char,
                                   NET_AdrToString(from));
                        (*nr).pfnFunc.expect("non-null function pointer")(&mut (*nr).resp);
                        // throw the list, now it will be stored in user area
                        prev =
                            &mut (*nr).resp.response as *mut *mut libc::c_void
                                as *mut *mut net_adrlist_t;
                        loop  {
                            list = *prev;
                            if list.is_null() { break ; }
                            // throw out any variables the game created
                            *prev = (*list).next; // done
                            _Mem_Free(list as *mut libc::c_void,
                                      b"../engine/client/cl_main.c\x00" as
                                          *const u8 as *const libc::c_char,
                                      2142 as libc::c_int);
                        }
                        memset(nr as *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<net_request_t>() as
                                   libc::c_ulong);
                        clgame.request_type = NET_REQUEST_CANCEL;
                        clgame.master_request = 0 as *mut net_request_t
                    }
                    break ;
                } else if clgame.request_type as libc::c_uint ==
                              NET_REQUEST_CLIENT as libc::c_int as
                                  libc::c_uint &&
                              !clgame.master_request.is_null() {
                    let mut nr_0: *mut net_request_t = clgame.master_request;
                    let mut list_0: *mut net_adrlist_t =
                        0 as *mut net_adrlist_t;
                    // adding addresses into list
                    list_0 =
                        _Mem_Alloc(host.mempool,
                                   ::std::mem::size_of::<net_adrlist_t>() as
                                       libc::c_ulong, false_0,
                                   b"../engine/client/cl_main.c\x00" as
                                       *const u8 as *const libc::c_char,
                                   2157 as libc::c_int) as
                            *mut net_adrlist_t; // allow remote
                    (*list_0).remote_address = servadr;
                    (*list_0).next =
                        (*nr_0).resp.response as *mut net_adrlist_s;
                    (*nr_0).resp.response = list_0 as *mut libc::c_void
                } else if clgame.request_type as libc::c_uint ==
                              NET_REQUEST_GAMEUI as libc::c_int as
                                  libc::c_uint {
                    NET_Config(true_0);
                    Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int, servadr,
                                           b"info %i\x00" as *const u8 as
                                               *const libc::c_char,
                                           49 as libc::c_int);
                }
            }
            if cls.internetservers_pending as u64 != 0 {
                UI_ResetPing();
                cls.internetservers_pending = false_0
            }
        } else if clgame.dllFuncs.pfnConnectionlessPacket.expect("non-null function pointer")(&mut from,
                                                                                              args,
                                                                                              buf.as_mut_ptr(),
                                                                                              &mut len)
                      != 0 {
            // user out of band message (must be handled in CL_ConnectionlessPacket)
            if len > 0 as libc::c_int {
                Netchan_OutOfBand(NS_SERVER as libc::c_int, from, len,
                                  buf.as_mut_ptr() as *mut byte);
            }
        } else {
            Con_DPrintf(b"^1Error:^7 bad connectionless packet from %s:\n%s\n\x00"
                            as *const u8 as *const libc::c_char,
                        NET_AdrToString(from), args);
        }
    };
}
/*
====================
CL_GetMessage

Handles recording and playback of demos, on top of NET_ code
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetMessage(mut data: *mut byte,
                                       mut length: *mut size_t)
 -> libc::c_int {
    if cls.demoplayback != 0 {
        if CL_DemoReadMessage(data, length) as u64 != 0 {
            return true_0 as libc::c_int
        }
        return false_0 as libc::c_int
    }
    if NET_GetPacket(NS_CLIENT, &mut net_from, data, length) as u64 != 0 {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
/*
=================
CL_ReadNetMessage
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadNetMessage() {
    let mut curSize: size_t = 0;
    while CL_GetMessage(net_message_buffer.as_mut_ptr(), &mut curSize) != 0 {
        if cls.legacymode as libc::c_uint != 0 &&
               *(&mut net_message_buffer as *mut [byte; 131120] as
                     *mut libc::c_int) as libc::c_uint ==
                   0xfffffffe as libc::c_uint {
            // Will rewrite existing packet by merged
            if NetSplit_GetLong(&mut cls.netchan.netsplit, &mut net_from,
                                net_message_buffer.as_mut_ptr(), &mut curSize)
                   as u64 == 0 {
                continue ;
            }
        }
        MSG_InitExt(&mut net_message,
                    b"ServerData\x00" as *const u8 as *const libc::c_char,
                    net_message_buffer.as_mut_ptr() as *mut libc::c_void,
                    curSize as libc::c_int, -(1 as libc::c_int));
        // check for connectionless packet (0xffffffff) first
        if MSG_GetMaxBytes(&mut net_message) >= 4 as libc::c_int &&
               *(net_message.pData as *mut libc::c_int) == -(1 as libc::c_int)
           {
            CL_ConnectionlessPacket(net_from, &mut net_message);
        } else {
            // can't be a valid sequenced packet
            if (cls.state as libc::c_uint) <
                   ca_connected as libc::c_int as libc::c_uint {
                continue ;
            }
            if cls.demoplayback == 0 &&
                   MSG_GetMaxBytes(&mut net_message) < 8 as libc::c_int {
                Con_Printf(b"^3Warning:^7 CL_ReadPackets: %s:runt packet\n\x00"
                               as *const u8 as *const libc::c_char,
                           NET_AdrToString(net_from));
            } else if cls.demoplayback == 0 &&
                          NET_CompareAdr(net_from, cls.netchan.remote_address)
                              as u64 == 0 {
                Con_DPrintf(b"^1Error:^7 CL_ReadPackets: %s:sequenced packet without connection\n\x00"
                                as *const u8 as *const libc::c_char,
                            NET_AdrToString(net_from));
            } else {
                // packet from server
                if cls.demoplayback == 0 &&
                       Netchan_Process(&mut cls.netchan, &mut net_message) as
                           u64 == 0 {
                    continue ; // wasn't accepted for some reason
                }
                // run special handler for quake demos
                if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
                    CL_ParseQuakeMessage(&mut net_message, true_0);
                } else if cls.legacymode as u64 != 0 {
                    CL_ParseLegacyServerMessage(&mut net_message, true_0);
                } else { CL_ParseServerMessage(&mut net_message, true_0); }
                cl.send_reply = true_0
            }
        }
    }
    // build list of all solid entities per next frame (exclude clients)
    CL_SetSolidEntities();
    // check for fragmentation/reassembly related packets.
    if cls.state as libc::c_uint !=
           ca_disconnected as libc::c_int as libc::c_uint &&
           Netchan_IncomingReady(&mut cls.netchan) as libc::c_uint != 0 {
        // process the incoming buffer(s)
        if Netchan_CopyNormalFragments(&mut cls.netchan, &mut net_message,
                                       &mut curSize) as u64 != 0 {
            MSG_InitExt(&mut net_message,
                        b"ServerData\x00" as *const u8 as *const libc::c_char,
                        net_message_buffer.as_mut_ptr() as *mut libc::c_void,
                        curSize as libc::c_int, -(1 as libc::c_int));
            CL_ParseServerMessage(&mut net_message, false_0);
        }
        if Netchan_CopyFileFragments(&mut cls.netchan, &mut net_message) as
               u64 != 0 {
            // remove from resource request stuff.
            CL_ProcessFile(true_0, cls.netchan.incomingfilename.as_mut_ptr());
        }
    }
    Netchan_UpdateProgress(&mut cls.netchan);
    // check requests for time-expire
    CL_ProcessNetRequests();
}
/*
=================
CL_ReadPackets

Updates the local time and reads/handles messages
on client net connection.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadPackets() {
    // decide the simulation time
    cl.oldtime = cl.time;
    if cls.demoplayback != DEMO_XASH3D as libc::c_int && cl.paused as u64 == 0
       {
        cl.time += host.frametime
    }
    // demo time
    if cls.demorecording as libc::c_uint != 0 && cls.demowaiting as u64 == 0 {
        cls.demotime += host.frametime
    }
    CL_ReadNetMessage();
    CL_ApplyAddAngle();
    // hot precache and downloading resources
    if cls.signon == 2 as libc::c_int && cl.lastresourcecheck < host.realtime
       {
        let mut checktime: libc::c_double =
            if Host_IsLocalGame() as libc::c_uint != 0 {
                0.1f64
            } else { 1.0f64 };
        if cls.dl.custom as u64 == 0 &&
               cl.resourcesneeded.pNext !=
                   &mut cl.resourcesneeded as *mut resource_t {
            // check resource for downloading and precache
            CL_EstimateNeededResources();
            CL_BatchResourceRequest(false_0);
            cls.dl.doneregistering = false_0;
            cls.dl.custom = true_0
        }
        cl.lastresourcecheck = host.realtime + checktime
    }
    // singleplayer never has connection timeout
    if NET_IsLocalAddress(cls.netchan.remote_address) as u64 != 0 { return }
    // if in the debugger last frame, don't timeout
    if host.frametime > 5.0f32 as libc::c_double {
        cls.netchan.last_received = Sys_DoubleTime()
    }
    // check timeout
    if cls.state as libc::c_uint >=
           ca_connected as libc::c_int as libc::c_uint &&
           cls.state as libc::c_uint !=
               ca_cinematic as libc::c_int as libc::c_uint &&
           cls.demoplayback == 0 {
        if host.realtime - cls.netchan.last_received >
               (*cl_timeout).value as libc::c_double {
            Con_Printf(b"\nServer connection timed out.\n\x00" as *const u8 as
                           *const libc::c_char);
            CL_Disconnect();
            return
        }
    };
}
/*
====================
CL_CleanFileName

Replace the displayed name for some resources
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CleanFileName(mut filename: *const libc::c_char)
 -> *const libc::c_char {
    let mut pfilename: *const libc::c_char = filename;
    if (if filename.is_null() || *filename == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) != 0 &&
           *filename.offset(0 as libc::c_int as isize) as libc::c_int ==
               '!' as i32 {
        pfilename = b"customization\x00" as *const u8 as *const libc::c_char
    }
    return pfilename;
}
/*
====================
CL_RegisterCustomization

register custom resource for player
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RegisterCustomization(mut resource:
                                                      *mut resource_t) {
    let mut bFound: qboolean = false_0;
    let mut pList: *mut customization_t = 0 as *mut customization_t;
    pList = cl.players[(*resource).playernum as usize].customdata.pNext;
    while !pList.is_null() {
        if memcmp((*pList).resource.rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void,
                  (*resource).rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void, 16 as libc::c_int as libc::c_ulong)
               == 0 {
            bFound = true_0;
            break ;
        } else { pList = (*pList).pNext }
    }
    if bFound as u64 == 0 {
        let mut player: *mut player_info_t =
            &mut *cl.players.as_mut_ptr().offset((*resource).playernum as
                                                     isize) as
                *mut player_info_t;
        if COM_CreateCustomization(&mut (*player).customdata, resource,
                                   (*resource).playernum as libc::c_int,
                                   (1 as libc::c_int) << 0 as libc::c_int,
                                   0 as *mut *mut customization_t,
                                   0 as *mut libc::c_int) as u64 == 0 {
            Con_Printf(b"Unable to create custom decal for player %i\n\x00" as
                           *const u8 as *const libc::c_char,
                       (*resource).playernum as libc::c_int);
        }
    } else {
        Con_DPrintf(b"Duplicate resource received and ignored.\n\x00" as
                        *const u8 as *const libc::c_char);
    };
}
/*
====================
CL_ProcessFile

A file has been received via the fragmentation/reassembly layer, put it in the right spot and
 see if we have finished downloading files.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessFile(mut successfully_received: qboolean,
                                        mut filename: *const libc::c_char) {
    let mut sound_len: libc::c_int =
        (::std::mem::size_of::<[libc::c_char; 7]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let mut rgucMD5_hash: [byte; 16] = [0; 16];
    let mut pfilename: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut resource_t = 0 as *mut resource_t;
    if (if filename.is_null() || *filename == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) != 0 &&
           successfully_received as libc::c_uint != 0 {
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int !=
               '!' as i32 {
            Con_Printf(b"processing %s\n\x00" as *const u8 as
                           *const libc::c_char, filename);
        }
    } else if successfully_received as u64 == 0 {
        Con_Printf(b"^1Error:^7 server failed to transmit file \'%s\'\n\x00"
                       as *const u8 as *const libc::c_char,
                   CL_CleanFileName(filename));
    }
    if cls.legacymode as u64 != 0 {
        if host.downloadcount > 0 as libc::c_int { host.downloadcount -= 1 }
        if host.downloadcount == 0 {
            MSG_WriteByte(&mut cls.netchan.message, 3 as libc::c_int);
            MSG_WriteString(&mut cls.netchan.message,
                            b"continueloading\x00" as *const u8 as
                                *const libc::c_char);
        }
        return
    }
    pfilename = filename;
    if Q_strnicmp(filename, b"sound/\x00" as *const u8 as *const libc::c_char,
                  sound_len) == 0 {
        pfilename = pfilename.offset(sound_len as isize)
    }
    p = cl.resourcesneeded.pNext;
    while p != &mut cl.resourcesneeded as *mut resource_t {
        if Q_strnicmp(filename,
                      b"!MD5\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int) == 0 {
            COM_HexConvert(filename.offset(4 as libc::c_int as isize),
                           32 as libc::c_int, rgucMD5_hash.as_mut_ptr());
            if memcmp((*p).rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
                      rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
                      16 as libc::c_int as libc::c_ulong) == 0 {
                break ;
            }
        } else if (*p).type_0 as libc::c_uint ==
                      t_generic as libc::c_int as libc::c_uint {
            if Q_strnicmp((*p).szFileName.as_mut_ptr(), filename,
                          99999 as libc::c_int) == 0 {
                break ;
            }
        } else if Q_strnicmp((*p).szFileName.as_mut_ptr(), pfilename,
                             99999 as libc::c_int) == 0 {
            break ;
        }
        p = (*p).pNext
    }
    if p != &mut cl.resourcesneeded as *mut resource_t {
        if successfully_received as u64 != 0 {
            (*p).ucFlags =
                ((*p).ucFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as
                    libc::c_uchar
        }
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int ==
               '!' as i32 {
            if !cls.netchan.tempbuffer.is_null() {
                if (*p).nDownloadSize == cls.netchan.tempbuffersize {
                    if (*p).ucFlags as libc::c_int &
                           (1 as libc::c_int) << 2 as libc::c_int != 0 {
                        HPAK_AddLump(true_0,
                                     b"custom.hpk\x00" as *const u8 as
                                         *const libc::c_char, p,
                                     cls.netchan.tempbuffer as *mut byte,
                                     0 as *mut file_t);
                        CL_RegisterCustomization(p);
                    }
                } else {
                    Con_Printf(b"Downloaded %i bytes for purported %i byte file, ignoring download\n\x00"
                                   as *const u8 as *const libc::c_char,
                               cls.netchan.tempbuffersize,
                               (*p).nDownloadSize);
                }
                if !cls.netchan.tempbuffer.is_null() {
                    _Mem_Free(cls.netchan.tempbuffer,
                              b"../engine/client/cl_main.c\x00" as *const u8
                                  as *const libc::c_char,
                              2487 as libc::c_int);
                }
            }
            cls.netchan.tempbuffersize = 0 as libc::c_int;
            cls.netchan.tempbuffer = 0 as *mut libc::c_void
        }
        // moving to 'onhandle' list even if file was missed
        CL_MoveToOnHandList(p);
    }
    if cls.state as libc::c_uint !=
           ca_disconnected as libc::c_int as libc::c_uint {
        host.downloadcount = 0 as libc::c_int;
        p = cl.resourcesneeded.pNext;
        while p != &mut cl.resourcesneeded as *mut resource_t {
            host.downloadcount += 1;
            p = (*p).pNext
        }
        if cl.resourcesneeded.pNext ==
               &mut cl.resourcesneeded as *mut resource_t {
            let mut msg_buf: [byte; 131072] = [0; 131072];
            let mut msg: sizebuf_t =
                sizebuf_t{bOverflow: false_0,
                          pDebugName: 0 as *const libc::c_char,
                          pData: 0 as *const byte as *mut byte,
                          iCurBit: 0,
                          nDataBits: 0,};
            MSG_InitExt(&mut msg,
                        b"Resource Registration\x00" as *const u8 as
                            *const libc::c_char,
                        msg_buf.as_mut_ptr() as *mut libc::c_void,
                        ::std::mem::size_of::<[byte; 131072]>() as
                            libc::c_ulong as libc::c_int,
                        -(1 as libc::c_int));
            if CL_PrecacheResources() as u64 != 0 {
                CL_RegisterResources(&mut msg);
            }
            if MSG_GetNumBytesWritten(&mut msg) > 0 as libc::c_int {
                Netchan_CreateFragments(&mut cls.netchan, &mut msg);
                Netchan_FragSend(&mut cls.netchan);
            }
        }
        if !cls.netchan.tempbuffer.is_null() {
            Con_Printf(b"Received a decal %s, but didn\'t find it in resources needed list!\n\x00"
                           as *const u8 as *const libc::c_char, pfilename);
            _Mem_Free(cls.netchan.tempbuffer,
                      b"../engine/client/cl_main.c\x00" as *const u8 as
                          *const libc::c_char, 2525 as libc::c_int);
        }
        cls.netchan.tempbuffer = 0 as *mut libc::c_void;
        cls.netchan.tempbuffersize = 0 as libc::c_int
    };
}
/*
====================
CL_ServerCommand

send command to a server
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ServerCommand(mut reliable: qboolean,
                                          mut fmt: *const libc::c_char,
                                          mut args: ...) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    if (cls.state as libc::c_uint) <
           ca_connecting as libc::c_int as libc::c_uint {
        return
    }
    argptr = args.clone();
    Q_vsnprintf(string.as_mut_ptr(), 99999 as libc::c_int as size_t, fmt,
                argptr.as_va_list());
    if reliable as u64 != 0 {
        MSG_WriteCmdExt(&mut cls.netchan.message, 3 as libc::c_int, NS_CLIENT,
                        0 as *const libc::c_char);
        MSG_WriteString(&mut cls.netchan.message, string.as_mut_ptr());
    } else {
        MSG_WriteCmdExt(&mut cls.datagram, 3 as libc::c_int, NS_CLIENT,
                        0 as *const libc::c_char);
        MSG_WriteString(&mut cls.datagram, string.as_mut_ptr());
    };
}
//=============================================================================
/*
==============
CL_SetInfo_f
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetInfo_f() {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"User info settings:\n\x00" as *const u8 as
                       *const libc::c_char);
        Info_Print(cls.userinfo.as_mut_ptr());
        Con_Printf(b"Total %i symbols\n\x00" as *const u8 as
                       *const libc::c_char,
                   Q_strlen(cls.userinfo.as_mut_ptr()));
        return
    }
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: setinfo [ <key> <value> ]\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // NOTE: some userinfo comed from cvars, e.g. cl_lw but we can call "setinfo cl_lw 1"
	// without real cvar changing. So we need to lookup for cvar first to make sure what
	// our key is not linked with console variable
    var = Cvar_FindVarExt(Cmd_Argv(1 as libc::c_int), 0 as libc::c_int);
    // make sure what cvar is existed and really part of userinfo
    if !var.is_null() &&
           (*var).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        Cvar_DirectSet(var, Cmd_Argv(2 as libc::c_int));
    } else if Info_SetValueForKey(cls.userinfo.as_mut_ptr(),
                                  Cmd_Argv(1 as libc::c_int),
                                  Cmd_Argv(2 as libc::c_int),
                                  256 as libc::c_int) as u64 != 0 {
        // send update only on successfully changed userinfo
        Cmd_ForwardToServer();
    };
}
/*
==============
CL_Physinfo_f
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Physinfo_f() {
    Con_Printf(b"Phys info settings:\n\x00" as *const u8 as
                   *const libc::c_char);
    Info_Print(cls.physinfo.as_mut_ptr());
    Con_Printf(b"Total %i symbols\n\x00" as *const u8 as *const libc::c_char,
               Q_strlen(cls.physinfo.as_mut_ptr()));
}
#[no_mangle]
pub unsafe extern "C" fn CL_PrecacheResources() -> qboolean {
    let mut pRes: *mut resource_t = 0 as *mut resource_t;
    // NOTE: world need to be loaded as first model
    pRes = cl.resourcesonhand.pNext;
    while !pRes.is_null() &&
              pRes != &mut cl.resourcesonhand as *mut resource_t {
        if !((*pRes).ucFlags as libc::c_int &
                 (1 as libc::c_int) << 4 as libc::c_int != 0) {
            if !((*pRes).type_0 as libc::c_uint !=
                     t_model as libc::c_int as libc::c_uint ||
                     (*pRes).nIndex != 1 as libc::c_int) {
                cl.models[(*pRes).nIndex as usize] =
                    Mod_LoadWorld((*pRes).szFileName.as_mut_ptr(), true_0);
                (*pRes).ucFlags =
                    ((*pRes).ucFlags as libc::c_int |
                         (1 as libc::c_int) << 4 as libc::c_int) as
                        libc::c_uchar;
                cl.nummodels = 1 as libc::c_int;
                break ;
            }
        }
        pRes = (*pRes).pNext
    }
    // then we set up all the world submodels
    pRes = cl.resourcesonhand.pNext;
    while !pRes.is_null() &&
              pRes != &mut cl.resourcesonhand as *mut resource_t {
        if !((*pRes).ucFlags as libc::c_int &
                 (1 as libc::c_int) << 4 as libc::c_int != 0) {
            if (*pRes).type_0 as libc::c_uint ==
                   t_model as libc::c_int as libc::c_uint &&
                   (*pRes).szFileName[0 as libc::c_int as usize] as
                       libc::c_int == '*' as i32 {
                cl.models[(*pRes).nIndex as usize] =
                    Mod_ForName((*pRes).szFileName.as_mut_ptr(), false_0,
                                false_0);
                cl.nummodels =
                    if cl.nummodels > (*pRes).nIndex + 1 as libc::c_int {
                        cl.nummodels
                    } else { ((*pRes).nIndex) + 1 as libc::c_int };
                (*pRes).ucFlags =
                    ((*pRes).ucFlags as libc::c_int |
                         (1 as libc::c_int) << 4 as libc::c_int) as
                        libc::c_uchar;
                if cl.models[(*pRes).nIndex as usize].is_null() {
                    Con_Printf(b"^1Error:^7 submodel %s not found\n\x00" as
                                   *const u8 as *const libc::c_char,
                               (*pRes).szFileName.as_mut_ptr());
                    if (*pRes).ucFlags as libc::c_int &
                           (1 as libc::c_int) << 0 as libc::c_int != 0 {
                        CL_Disconnect_f();
                        return false_0
                    }
                }
            }
        }
        pRes = (*pRes).pNext
    }
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        S_BeginRegistration();
    }
    // precache all the remaining resources where order is doesn't matter
    pRes = cl.resourcesonhand.pNext;
    while !pRes.is_null() &&
              pRes != &mut cl.resourcesonhand as *mut resource_t {
        if !((*pRes).ucFlags as libc::c_int &
                 (1 as libc::c_int) << 4 as libc::c_int != 0) {
            match (*pRes).type_0 as libc::c_uint {
                0 => {
                    if (*pRes).nIndex != -(1 as libc::c_int) {
                        if (*pRes).ucFlags as libc::c_int &
                               (1 as libc::c_int) << 1 as libc::c_int != 0 {
                            Con_Printf(b"^1Error:^7 Could not load sound sound/%s\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       (*pRes).szFileName.as_mut_ptr());
                            cl.sound_precache[(*pRes).nIndex as
                                                  usize][0 as libc::c_int as
                                                             usize] =
                                0 as libc::c_int as libc::c_char;
                            cl.sound_index[(*pRes).nIndex as usize] =
                                0 as libc::c_int as libc::c_short
                        } else {
                            Q_strncpy(cl.sound_precache[(*pRes).nIndex as
                                                            usize].as_mut_ptr(),
                                      (*pRes).szFileName.as_mut_ptr(),
                                      ::std::mem::size_of::<[libc::c_char; 64]>()
                                          as libc::c_ulong);
                            cl.sound_index[(*pRes).nIndex as usize] =
                                S_RegisterSound((*pRes).szFileName.as_mut_ptr())
                                    as libc::c_short;
                            if cl.sound_index[(*pRes).nIndex as usize] == 0 {
                                if (*pRes).ucFlags as libc::c_int &
                                       (1 as libc::c_int) << 0 as libc::c_int
                                       != 0 {
                                    S_EndRegistration();
                                    CL_Disconnect_f();
                                    return false_0
                                }
                            }
                        }
                    } else {
                        // client sounds
                        S_RegisterSound((*pRes).szFileName.as_mut_ptr());
                    }
                }
                2 => {
                    cl.nummodels =
                        if cl.nummodels > (*pRes).nIndex + 1 as libc::c_int {
                            cl.nummodels
                        } else { ((*pRes).nIndex) + 1 as libc::c_int };
                    if (*pRes).szFileName[0 as libc::c_int as usize] as
                           libc::c_int != '*' as i32 {
                        if (*pRes).nIndex != -(1 as libc::c_int) {
                            cl.models[(*pRes).nIndex as usize] =
                                Mod_ForName((*pRes).szFileName.as_mut_ptr(),
                                            false_0, true_0);
                            if cl.models[(*pRes).nIndex as usize].is_null() {
                                if (*pRes).ucFlags as libc::c_int &
                                       (1 as libc::c_int) << 0 as libc::c_int
                                       != 0 {
                                    S_EndRegistration();
                                    CL_Disconnect_f();
                                    return false_0
                                }
                            }
                        } else {
                            CL_LoadClientSprite((*pRes).szFileName.as_mut_ptr());
                        }
                    }
                }
                3 => {
                    if (*pRes).ucFlags as libc::c_int &
                           (1 as libc::c_int) << 2 as libc::c_int == 0 {
                        Q_strncpy(host.draw_decals[(*pRes).nIndex as
                                                       usize].as_mut_ptr(),
                                  (*pRes).szFileName.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 64]>()
                                      as libc::c_ulong);
                    }
                }
                4 => {
                    Q_strncpy(cl.files_precache[(*pRes).nIndex as
                                                    usize].as_mut_ptr(),
                              (*pRes).szFileName.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong);
                    cl.numfiles =
                        if cl.numfiles > (*pRes).nIndex + 1 as libc::c_int {
                            cl.numfiles
                        } else { ((*pRes).nIndex) + 1 as libc::c_int }
                }
                5 => {
                    Q_strncpy(cl.event_precache[(*pRes).nIndex as
                                                    usize].as_mut_ptr(),
                              (*pRes).szFileName.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong);
                    CL_SetEventIndex(cl.event_precache[(*pRes).nIndex as
                                                           usize].as_mut_ptr(),
                                     (*pRes).nIndex);
                }
                1 | _ => { }
            }
            (*pRes).ucFlags =
                ((*pRes).ucFlags as libc::c_int |
                     (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uchar
        }
        pRes = (*pRes).pNext
    }
    // make sure modelcount is in-range
    cl.nummodels =
        if cl.nummodels >= 0 as libc::c_int {
            if cl.nummodels < 1024 as libc::c_int {
                cl.nummodels
            } else { 1024 as libc::c_int }
        } else { 0 as libc::c_int };
    cl.numfiles =
        if cl.numfiles >= 0 as libc::c_int {
            if cl.numfiles < (1 as libc::c_int) << 10 as libc::c_int {
                cl.numfiles
            } else { ((1 as libc::c_int)) << 10 as libc::c_int }
        } else { 0 as libc::c_int };
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        S_EndRegistration();
    }
    return true_0;
}
/*
==================
CL_FullServerinfo_f

Sent by server when serverinfo changes
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FullServerinfo_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: fullserverinfo <complete info string>\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    Q_strncpy(cl.serverinfo.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong);
}
/*
=================
CL_Escape_f

Escape to menu from game
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Escape_f() {
    if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint
       {
        return
    }
    // the final credits is running
    if UI_CreditsActive() as u64 != 0 { return } // jump to next movie
    if cls.state as libc::c_uint ==
           ca_cinematic as libc::c_int as libc::c_uint {
        SCR_NextMovie();
    } else { UI_SetActiveMenu(true_0); };
}
/*
=================
CL_InitLocal
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitLocal() {
    cls.state = ca_disconnected;
    cls.signon = 0 as libc::c_int;
    memset(&mut cls.serveradr as *mut netadr_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    cl.resourcesneeded.pPrev = &mut cl.resourcesneeded;
    cl.resourcesneeded.pNext = cl.resourcesneeded.pPrev;
    cl.resourcesonhand.pPrev = &mut cl.resourcesonhand;
    cl.resourcesonhand.pNext = cl.resourcesonhand.pPrev;
    Cvar_RegisterVariable(&mut mp_decals);
    Cvar_RegisterVariable(&mut dev_overview);
    Cvar_RegisterVariable(&mut cl_resend);
    Cvar_RegisterVariable(&mut cl_allow_upload);
    Cvar_RegisterVariable(&mut cl_allow_download);
    Cvar_RegisterVariable(&mut cl_download_ingame);
    Cvar_RegisterVariable(&mut cl_logofile);
    Cvar_RegisterVariable(&mut cl_logocolor);
    Cvar_RegisterVariable(&mut cl_test_bandwidth);
    // register our variables
    cl_crosshair =
        Cvar_Get(b"crosshair\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"show weapon chrosshair\x00" as *const u8 as
                     *const libc::c_char);
    cl_nodelta =
        Cvar_Get(b"cl_nodelta\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"disable delta-compression for server messages\x00" as
                     *const u8 as *const libc::c_char);
    cl_idealpitchscale =
        Cvar_Get(b"cl_idealpitchscale\x00" as *const u8 as
                     *const libc::c_char,
                 b"0.8\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"how much to look up/down slopes and stairs when not using freelook\x00"
                     as *const u8 as *const libc::c_char);
    cl_solid_players =
        Cvar_Get(b"cl_solid_players\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"Make all players not solid (can\'t traceline them)\x00" as
                     *const u8 as *const libc::c_char);
    cl_interp =
        Cvar_Get(b"ex_interp\x00" as *const u8 as *const libc::c_char,
                 b"0.1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"Interpolate object positions starting this many seconds in past\x00"
                     as *const u8 as *const libc::c_char);
    cl_timeout =
        Cvar_Get(b"cl_timeout\x00" as *const u8 as *const libc::c_char,
                 b"60\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"connect timeout (in-seconds)\x00" as *const u8 as
                     *const libc::c_char);
    cl_charset =
        Cvar_Get(b"cl_charset\x00" as *const u8 as *const libc::c_char,
                 b"utf-8\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"1-byte charset to use (iconv style)\x00" as *const u8 as
                     *const libc::c_char);
    hud_utf8 =
        Cvar_Get(b"hud_utf8\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"Use utf-8 encoding for hud text\x00" as *const u8 as
                     *const libc::c_char);
    rcon_client_password =
        Cvar_Get(b"rcon_password\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 10 as libc::c_int,
                 b"remote control client password\x00" as *const u8 as
                     *const libc::c_char);
    rcon_address =
        Cvar_Get(b"rcon_address\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 10 as libc::c_int,
                 b"remote control address\x00" as *const u8 as
                     *const libc::c_char);
    cl_trace_messages =
        Cvar_Get(b"cl_trace_messages\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 15 as libc::c_int,
                 b"enable message names tracing (good for developers)\x00" as
                     *const u8 as *const libc::c_char);
    // userinfo
    cl_nopred =
        Cvar_Get(b"cl_nopred\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 1 as libc::c_int,
                 b"disable client movement prediction\x00" as *const u8 as
                     *const libc::c_char);
    name =
        Cvar_Get(b"name\x00" as *const u8 as *const libc::c_char,
                 Sys_GetCurrentUser(),
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 7 as libc::c_int,
                 b"player name\x00" as *const u8 as *const libc::c_char);
    model =
        Cvar_Get(b"model\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int,
                 b"player model (\'player\' is a singleplayer model)\x00" as
                     *const u8 as *const libc::c_char);
    cl_updaterate =
        Cvar_Get(b"cl_updaterate\x00" as *const u8 as *const libc::c_char,
                 b"20\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int,
                 b"refresh rate of server messages\x00" as *const u8 as
                     *const libc::c_char);
    cl_dlmax =
        Cvar_Get(b"cl_dlmax\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int,
                 b"max allowed outcoming fragment size\x00" as *const u8 as
                     *const libc::c_char);
    cl_upmax =
        Cvar_Get(b"cl_upmax\x00" as *const u8 as *const libc::c_char,
                 b"1200\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"max allowed incoming fragment size\x00" as *const u8 as
                     *const libc::c_char);
    rate =
        Cvar_Get(b"rate\x00" as *const u8 as *const libc::c_char,
                 b"3500\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"player network rate\x00" as *const u8 as
                     *const libc::c_char);
    topcolor =
        Cvar_Get(b"topcolor\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int,
                 b"player top color\x00" as *const u8 as *const libc::c_char);
    bottomcolor =
        Cvar_Get(b"bottomcolor\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 1 as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int,
                 b"player bottom color\x00" as *const u8 as
                     *const libc::c_char);
    cl_lw =
        Cvar_Get(b"cl_lw\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 1 as libc::c_int,
                 b"enable client weapon predicting\x00" as *const u8 as
                     *const libc::c_char);
    Cvar_Get(b"cl_lc\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 1 as libc::c_int,
             b"enable lag compensation\x00" as *const u8 as
                 *const libc::c_char);
    Cvar_Get(b"password\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 1 as libc::c_int,
             b"server password\x00" as *const u8 as *const libc::c_char);
    Cvar_Get(b"team\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 1 as libc::c_int,
             b"player team\x00" as *const u8 as *const libc::c_char);
    Cvar_Get(b"skin\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 1 as libc::c_int,
             b"player skin\x00" as *const u8 as *const libc::c_char);
    cl_nosmooth =
        Cvar_Get(b"cl_nosmooth\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"disable smooth up stair climbing\x00" as *const u8 as
                     *const libc::c_char);
    cl_nointerp =
        Cvar_Get(b"cl_nointerp\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 4 as libc::c_int,
                 b"disable interpolation of entities and players\x00" as
                     *const u8 as *const libc::c_char);
    cl_smoothtime =
        Cvar_Get(b"cl_smoothtime\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"time to smooth up\x00" as *const u8 as
                     *const libc::c_char);
    cl_cmdbackup =
        Cvar_Get(b"cl_cmdbackup\x00" as *const u8 as *const libc::c_char,
                 b"10\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"how many additional history commands are sent\x00" as
                     *const u8 as *const libc::c_char);
    cl_cmdrate =
        Cvar_Get(b"cl_cmdrate\x00" as *const u8 as *const libc::c_char,
                 b"30\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"Max number of command packets sent to server per second\x00"
                     as *const u8 as *const libc::c_char);
    cl_draw_particles =
        Cvar_Get(b"r_drawparticles\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 15 as libc::c_int,
                 b"render particles\x00" as *const u8 as *const libc::c_char);
    cl_draw_tracers =
        Cvar_Get(b"r_drawtracers\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 15 as libc::c_int,
                 b"render tracers\x00" as *const u8 as *const libc::c_char);
    cl_draw_beams =
        Cvar_Get(b"r_drawbeams\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 15 as libc::c_int,
                 b"render beams\x00" as *const u8 as *const libc::c_char);
    cl_lightstyle_lerping =
        Cvar_Get(b"cl_lightstyle_lerping\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"enables animated light lerping (perfomance option)\x00" as
                     *const u8 as *const libc::c_char);
    cl_showerror =
        Cvar_Get(b"cl_showerror\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"show prediction error\x00" as *const u8 as
                     *const libc::c_char);
    cl_bmodelinterp =
        Cvar_Get(b"cl_bmodelinterp\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"enable bmodel interpolation\x00" as *const u8 as
                     *const libc::c_char);
    cl_clockreset =
        Cvar_Get(b"cl_clockreset\x00" as *const u8 as *const libc::c_char,
                 b"0.1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"frametime delta maximum value before reset\x00" as
                     *const u8 as *const libc::c_char);
    cl_fixtimerate =
        Cvar_Get(b"cl_fixtimerate\x00" as *const u8 as *const libc::c_char,
                 b"7.5\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"time in msec to client clock adjusting\x00" as *const u8 as
                     *const libc::c_char);
    hud_scale =
        Cvar_Get(b"hud_scale\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 30 as libc::c_int,
                 b"scale hud at current resolution\x00" as *const u8 as
                     *const libc::c_char);
    Cvar_Get(b"cl_background\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 17 as libc::c_int,
             b"indicate what background map is running\x00" as *const u8 as
                 *const libc::c_char);
    cl_showevents =
        Cvar_Get(b"cl_showevents\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"show events playback\x00" as *const u8 as
                     *const libc::c_char);
    Cvar_Get(b"lastdemo\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int,
             b"last played demo\x00" as *const u8 as *const libc::c_char);
    ui_renderworld =
        Cvar_Get(b"ui_renderworld\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"render world when UI is visible\x00" as *const u8 as
                     *const libc::c_char);
    // these two added to shut up CS 1.5 about 'unknown' commands
    Cvar_Get(b"lightgamma\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int,
             b"ambient lighting level (legacy, unused)\x00" as *const u8 as
                 *const libc::c_char);
    Cvar_Get(b"direct\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int,
             b"direct lighting level (legacy, unused)\x00" as *const u8 as
                 *const libc::c_char);
    // server commands
    Cmd_AddCommand(b"noclip\x00" as *const u8 as *const libc::c_char, None,
                   b"enable or disable no clipping mode\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"notarget\x00" as *const u8 as *const libc::c_char, None,
                   b"notarget mode (monsters do not see you)\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"fullupdate\x00" as *const u8 as *const libc::c_char,
                   None,
                   b"re-init HUD on start demo recording\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"give\x00" as *const u8 as *const libc::c_char, None,
                   b"give specified item or weapon\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"drop\x00" as *const u8 as *const libc::c_char, None,
                   b"drop current/specified item or weapon\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"gametitle\x00" as *const u8 as *const libc::c_char, None,
                   b"show game logo\x00" as *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"kill\x00" as *const u8 as *const libc::c_char,
                             None,
                             b"die instantly\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddCommand(b"god\x00" as *const u8 as *const libc::c_char, None,
                   b"enable godmode\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"fov\x00" as *const u8 as *const libc::c_char, None,
                   b"set client field of view\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"log\x00" as *const u8 as *const libc::c_char, None,
                   b"logging server events\x00" as *const u8 as
                       *const libc::c_char);
    // register our commands
    Cmd_AddCommand(b"pause\x00" as *const u8 as *const libc::c_char, None,
                   b"pause the game (if the server allows pausing)\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"localservers\x00" as *const u8 as *const libc::c_char,
                   Some(CL_LocalServers_f as unsafe extern "C" fn() -> ()),
                   b"collect info about local servers\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"internetservers\x00" as *const u8 as *const libc::c_char,
                   Some(CL_InternetServers_f as unsafe extern "C" fn() -> ()),
                   b"collect info about internet servers\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"cd\x00" as *const u8 as *const libc::c_char,
                   Some(CL_PlayCDTrack_f as unsafe extern "C" fn() -> ()),
                   b"Play cd-track (not real cd-player of course)\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"mp3\x00" as *const u8 as *const libc::c_char,
                   Some(CL_PlayCDTrack_f as unsafe extern "C" fn() -> ()),
                   b"Play mp3-track (based on virtual cd-player)\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"waveplaylen\x00" as *const u8 as *const libc::c_char,
                   Some(CL_WavePlayLen_f as unsafe extern "C" fn() -> ()),
                   b"Get approximate length of wave file\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddRestrictedCommand(b"setinfo\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(CL_SetInfo_f as
                                      unsafe extern "C" fn() -> ()),
                             b"examine or change the userinfo string (alias of userinfo)\x00"
                                 as *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"userinfo\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(CL_SetInfo_f as
                                      unsafe extern "C" fn() -> ()),
                             b"examine or change the userinfo string (alias of setinfo)\x00"
                                 as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"physinfo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Physinfo_f as unsafe extern "C" fn() -> ()),
                   b"print current client physinfo\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"disconnect\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Disconnect_f as unsafe extern "C" fn() -> ()),
                   b"disconnect from server\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"record\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Record_f as unsafe extern "C" fn() -> ()),
                   b"record a demo\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"playdemo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_PlayDemo_f as unsafe extern "C" fn() -> ()),
                   b"play a demo\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"timedemo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_TimeDemo_f as unsafe extern "C" fn() -> ()),
                   b"demo benchmark\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"killdemo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_DeleteDemo_f as unsafe extern "C" fn() -> ()),
                   b"delete a specified demo file\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"startdemos\x00" as *const u8 as *const libc::c_char,
                   Some(CL_StartDemos_f as unsafe extern "C" fn() -> ()),
                   b"start playing back the selected demos sequentially\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"demos\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Demos_f as unsafe extern "C" fn() -> ()),
                   b"restart looping demos defined by the last startdemos command\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"movie\x00" as *const u8 as *const libc::c_char,
                   Some(CL_PlayVideo_f as unsafe extern "C" fn() -> ()),
                   b"play a movie\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"stop\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Stop_f as unsafe extern "C" fn() -> ()),
                   b"stop playing or recording a demo\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"info\x00" as *const u8 as *const libc::c_char, None,
                   b"collect info about local servers with specified protocol\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"escape\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Escape_f as unsafe extern "C" fn() -> ()),
                   b"escape from game to menu\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"togglemenu\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Escape_f as unsafe extern "C" fn() -> ()),
                   b"toggle between game and menu\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"pointfile\x00" as *const u8 as *const libc::c_char,
                   Some(CL_ReadPointFile_f as unsafe extern "C" fn() -> ()),
                   b"show leaks on a map (if present of course)\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"linefile\x00" as *const u8 as *const libc::c_char,
                   Some(CL_ReadLineFile_f as unsafe extern "C" fn() -> ()),
                   b"show leaks on a map (if present of course)\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"fullserverinfo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_FullServerinfo_f as unsafe extern "C" fn() -> ()),
                   b"sent by server when serverinfo changes\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"upload\x00" as *const u8 as *const libc::c_char,
                   Some(CL_BeginUpload_f as unsafe extern "C" fn() -> ()),
                   b"uploading file to the server\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddRestrictedCommand(b"quit\x00" as *const u8 as *const libc::c_char,
                             ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                -> !>,
                                                     xcommand_t>(Some(CL_Quit_f
                                                                          as
                                                                          unsafe extern "C" fn()
                                                                              ->
                                                                                  !)),
                             b"quit from game\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"exit\x00" as *const u8 as *const libc::c_char,
                             ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                -> !>,
                                                     xcommand_t>(Some(CL_Quit_f
                                                                          as
                                                                          unsafe extern "C" fn()
                                                                              ->
                                                                                  !)),
                             b"quit from game\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddCommand(b"screenshot\x00" as *const u8 as *const libc::c_char,
                   Some(CL_ScreenShot_f as unsafe extern "C" fn() -> ()),
                   b"takes a screenshot of the next rendered frame\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"snapshot\x00" as *const u8 as *const libc::c_char,
                   Some(CL_SnapShot_f as unsafe extern "C" fn() -> ()),
                   b"takes a snapshot of the next rendered frame\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"envshot\x00" as *const u8 as *const libc::c_char,
                   Some(CL_EnvShot_f as unsafe extern "C" fn() -> ()),
                   b"takes a six-sides cubemap shot with specified name\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"skyshot\x00" as *const u8 as *const libc::c_char,
                   Some(CL_SkyShot_f as unsafe extern "C" fn() -> ()),
                   b"takes a six-sides envmap (skybox) shot with specified name\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"levelshot\x00" as *const u8 as *const libc::c_char,
                   Some(CL_LevelShot_f as unsafe extern "C" fn() -> ()),
                   b"same as \"screenshot\", used for create plaque images\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"saveshot\x00" as *const u8 as *const libc::c_char,
                   Some(CL_SaveShot_f as unsafe extern "C" fn() -> ()),
                   b"used for create save previews with LoadGame menu\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"connect\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(CL_Connect_f as
                                      unsafe extern "C" fn() -> ()),
                             b"connect to a server by hostname\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"reconnect\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Reconnect_f as unsafe extern "C" fn() -> ()),
                   b"reconnect to current level\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"rcon\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Rcon_f as unsafe extern "C" fn() -> ()),
                   b"sends a command to the server console (rcon_password and rcon_address required)\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"precache\x00" as *const u8 as *const libc::c_char,
                   Some(CL_LegacyPrecache_f as unsafe extern "C" fn() -> ()),
                   b"legacy server compatibility\x00" as *const u8 as
                       *const libc::c_char);
}
//============================================================================
/*
==================
CL_AdjustClock

slowly adjuct client clock
to smooth lag effect
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AdjustClock() {
    if cl.timedelta == 0.0f32 || (*cl_fixtimerate).value == 0. { return }
    if (*cl_fixtimerate).value < 0.0f32 {
        Cvar_SetValue(b"cl_fixtimerate\x00" as *const u8 as
                          *const libc::c_char, 7.5f32);
    }
    if __tg_fabs(cl.timedelta) >= 0.001f32 {
        let mut msec: libc::c_double = 0.;
        let mut adjust: libc::c_double = 0.;
        let mut sign: libc::c_float = 0.;
        msec = (cl.timedelta * 1000.0f32) as libc::c_double;
        sign =
            if msec < 0 as libc::c_int as libc::c_double {
                1.0f32
            } else { -1.0f32 };
        msec = __tg_fabs_0(msec);
        adjust =
            (sign * ((*cl_fixtimerate).value / 1000.0f32)) as libc::c_double;
        if __tg_fabs_0(adjust) < __tg_fabs(cl.timedelta) as libc::c_double {
            cl.timedelta =
                (cl.timedelta as libc::c_double + adjust) as libc::c_float;
            cl.time += adjust
        }
        if cl.oldtime > cl.time { cl.oldtime = cl.time }
    };
}
/*
==================
Host_ClientBegin

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_ClientBegin() {
    // exec console commands
    Cbuf_Execute();
    // if client is not active, do nothing
    if cls.initialized as u64 == 0 { return }
    // finalize connection process if needs
    CL_CheckClientState();
    // tell the client.dll about client data
    CL_UpdateClientData();
    // if running the server locally, make intentions now
    if SV_Active() as u64 != 0 { CL_SendCommand(); };
}
/*
==================
Host_ClientFrame

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_ClientFrame() {
    // if client is not active, do nothing
    if cls.initialized as u64 == 0 { return }
    // if running the server remotely, send intentions now after
	// the incoming messages have been read
    if SV_Active() as u64 == 0 { CL_SendCommand(); }
    clgame.dllFuncs.pfnFrame.expect("non-null function pointer")(host.frametime);
    // remember last received framenum
    CL_SetLastUpdate();
    // read updates from server
    CL_ReadPackets();
    // do prediction again in case we got
	// a new portion updates from server
    CL_RedoPrediction();
    // TODO: implement
//	Voice_Idle( host.frametime );
    // emit visible entities
    CL_EmitEntities();
    // in case we lost connection
    CL_CheckForResend();
    // procssing resources on handle
    while CL_RequestMissingResources() as u64 != 0 { }
    // handle thirdperson camera
    CL_MoveThirdpersonCamera();
    // handle spectator movement
    CL_MoveSpectatorCamera();
    // catch changes video settings
    VID_CheckChanges();
    // process VGUI
    VGui_RunFrame();
    // update the screen
    SCR_UpdateScreen();
    // update audio
    SND_UpdateSound();
    // play avi-files
    SCR_RunCinematic();
    // adjust client time
    CL_AdjustClock();
}
//============================================================================
/*
====================
CL_Init
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Init() {
    let mut libpath: string = [0; 256]; // nothing running on the client
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        return
    } // init video
    CL_InitLocal(); // init sound
    VID_Init();
    S_Init();
    // unreliable buffer. unsed for unreliable commands and voice stream
    MSG_InitExt(&mut cls.datagram,
                b"cls.datagram\x00" as *const u8 as *const libc::c_char,
                cls.datagram_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // IN_TouchInit();
    Con_LoadHistory(); // allow to drawing player in menu
    COM_GetCommonLibraryPath(LIBRARY_CLIENT, libpath.as_mut_ptr(),
                             ::std::mem::size_of::<string>() as
                                 libc::c_ulong);
    if CL_LoadProgs(libpath.as_mut_ptr()) as u64 == 0 {
        Host_Error(b"can\'t initialize %s: %s\n\x00" as *const u8 as
                       *const libc::c_char, libpath.as_mut_ptr(),
                   COM_GetLibraryError());
    }
    cls.initialized = true_0;
    cl.maxclients = 1 as libc::c_int;
    cls.olddemonum = -(1 as libc::c_int);
    cls.demonum = -(1 as libc::c_int);
}
/*
===============
CL_Shutdown

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Shutdown() {
    // already freed
    if cls.initialized as u64 == 0 { return }
    cls.initialized = false_0;
    Con_Printf(b"CL_Shutdown()\n\x00" as *const u8 as *const libc::c_char);
    if host.crashed as u64 == 0 {
        Host_WriteOpenGLConfig();
        Host_WriteVideoConfig();
        Touch_WriteConfig();
    }
    // IN_TouchShutdown ();
    Joy_Shutdown(); // remove tmp file
    CL_CloseDemoHeader(); // release AVI's *after* client.dll because custom renderer may use them
    IN_Shutdown();
    Mobile_Shutdown();
    SCR_Shutdown();
    CL_UnloadProgs();
    FS_Delete(b"demoheader.tmp\x00" as *const u8 as *const libc::c_char);
    SCR_FreeCinematic();
    S_Shutdown();
    R_Shutdown();
    Con_Shutdown();
}
