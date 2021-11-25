#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, const_transmute,
           extern_types, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type file_s;
    pub type grasshdr_s;
    pub type ref_viewpass_s;
    pub type mip_s;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn S_GetCurrentStaticSounds(pout: *mut soundlist_t, size: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn S_StopSound(entnum: libc::c_int, channel: libc::c_int,
                   soundname: *const libc::c_char);
    #[no_mangle]
    fn COM_RandomFloat(fMin: libc::c_float, fMax: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn cosf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Info_SetValueForStarKey(s: *mut libc::c_char, key: *const libc::c_char,
                               value: *const libc::c_char,
                               maxsize: libc::c_int) -> qboolean;
    #[no_mangle]
    fn Info_SetValueForKey(s: *mut libc::c_char, key: *const libc::c_char,
                           value: *const libc::c_char, maxsize: libc::c_int)
     -> qboolean;
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
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Sys_GetParmFromCmdLine(parm: *const libc::c_char,
                               out: *mut libc::c_char, size: size_t)
     -> qboolean;
    #[no_mangle]
    fn Info_RemoveKey(s: *mut libc::c_char, key: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
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
    fn Cvar_Reset(var_name: *const libc::c_char);
    #[no_mangle]
    fn Cvar_Unlink(group: libc::c_int);
    #[no_mangle]
    fn CRC32_Init(pulCRC: *mut dword);
    #[no_mangle]
    fn CRC32_ProcessBuffer(pulCRC: *mut dword, pBuffer: *const libc::c_void,
                           nBuffer: libc::c_int);
    #[no_mangle]
    fn CRC32_ProcessByte(pulCRC: *mut dword, ch: byte);
    #[no_mangle]
    fn CRC32_Final(pulCRC: dword) -> dword;
    #[no_mangle]
    static mut host_developer: convar_t;
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
    fn Cmd_AddServerCommand(cmd_name: *const libc::c_char,
                            function: xcommand_t);
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
    fn Mem_IsAllocatedExt(poolptr: poolhandle_t, data: *mut libc::c_void)
     -> qboolean;
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn FS_WriteFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn COM_ParseVector(pfile: *mut *mut libc::c_char, v: *mut libc::c_float,
                       size: size_t) -> qboolean;
    #[no_mangle]
    fn COM_FileSize(filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn COM_FreeFile(buffer: *mut libc::c_void);
    #[no_mangle]
    fn COM_CompareFileTime(filename1: *const libc::c_char,
                           filename2: *const libc::c_char,
                           iCompare: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Seek(file: *mut file_t, offset: fs_offset_t, whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileTime(filename: *const libc::c_char, gamedironly: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Sound_GetApproxWavePlayLen(filepath: *const libc::c_char) -> uint;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_Credits();
    #[no_mangle]
    fn COM_ChangeLevel(pNewLevel: *const libc::c_char,
                       pLandmarkName: *const libc::c_char,
                       background: qboolean);
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn pfnCvar_RegisterServerVariable(variable: *mut cvar_t);
    #[no_mangle]
    fn pfnCvar_RegisterEngineVariable(variable: *mut cvar_t);
    #[no_mangle]
    fn COM_LoadFileForMe(filename: *const libc::c_char,
                         pLength: *mut libc::c_int) -> *mut byte;
    #[no_mangle]
    fn pfnCVarGetPointer(szVarName: *const libc::c_char) -> *mut cvar_t;
    #[no_mangle]
    fn S_StreamGetCurrentState(currentTrack: *mut libc::c_char,
                               loopTrack: *mut libc::c_char,
                               position: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn CL_ClearStaticEntities();
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn pfnCVarDirectSet(var: *mut cvar_t, szValue: *const libc::c_char);
    #[no_mangle]
    fn COM_CheckParm(parm: *mut libc::c_char, ppnext: *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn pfnGetGameDir(szGetGameDir: *mut libc::c_char);
    #[no_mangle]
    fn SV_BroadcastPrintf(ignore: *mut sv_client_s, fmt: *const libc::c_char,
                          _: ...);
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn pfnTime() -> libc::c_float;
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn pfnSequenceGet(fileName: *const libc::c_char,
                      entryName: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn pfnSequencePickSentence(groupName: *const libc::c_char,
                               pickMethod: libc::c_int,
                               picked: *mut libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn pfnIsCareerMatch() -> libc::c_int;
    #[no_mangle]
    fn pfnGetTimesTutorMessageShown(mid: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pfnRegisterTutorMessageShown(mid: libc::c_int);
    #[no_mangle]
    fn pfnConstructTutorMessageDecayBuffer(buffer: *mut libc::c_int,
                                           buflen: libc::c_int);
    #[no_mangle]
    fn pfnProcessTutorMessageDecayBuffer(buffer: *mut libc::c_int,
                                         bufferLength: libc::c_int);
    #[no_mangle]
    fn pfnResetTutorMessageDecayData();
    #[no_mangle]
    fn CL_DisableVisibility() -> qboolean;
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_Clear(sb: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_WriteOneBit(sb: *mut sizebuf_t, nValue: libc::c_int);
    #[no_mangle]
    fn MSG_WriteUBitLong(sb: *mut sizebuf_t, curData: uint,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteBits(sb: *mut sizebuf_t, pData: *const libc::c_void,
                     nBits: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_WriteWord(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteShort(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn SV_IsPlayerIndex(idx: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    fn Mod_GetPVSForPoint(p: *const vec_t) -> *mut byte;
    #[no_mangle]
    fn SV_SoundIndex(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Mod_TestBmodelLumps(name: *const libc::c_char, mod_base: *const byte,
                           silent: qboolean) -> qboolean;
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteVec3Coord(sb: *mut sizebuf_t, fa: *const libc::c_float);
    #[no_mangle]
    fn Mod_FatPVS(org: *const vec_t, radius: libc::c_float,
                  visbuffer: *mut byte, visbytes: libc::c_int,
                  merge: qboolean, fullvis: qboolean) -> libc::c_int;
    #[no_mangle]
    static mut world: world_static_t;
    #[no_mangle]
    fn anglemod(a: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn VectorAngles(forward: *const libc::c_float,
                    angles: *mut libc::c_float);
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    fn Mod_StudioExtradata(mod_0: *mut model_t) -> *mut libc::c_void;
    #[no_mangle]
    fn Mod_ForName(name: *const libc::c_char, crash: qboolean,
                   trackCRC: qboolean) -> *mut model_t;
    #[no_mangle]
    fn Mod_HeadnodeVisible(node: *mut mnode_t, visbits: *const byte,
                           lastleaf: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn Mod_BoxVisible(mins: *const vec_t, maxs: *const vec_t,
                      visbits: *const byte) -> qboolean;
    #[no_mangle]
    fn Mod_InitStudioAPI();
    #[no_mangle]
    fn Mod_ResetStudioAPI();
    #[no_mangle]
    fn Mod_StudioGetAttachment(e: *const edict_t, iAttachment: libc::c_int,
                               org: *mut libc::c_float,
                               ang: *mut libc::c_float);
    #[no_mangle]
    fn Mod_GetBonePosition(e: *const edict_t, iBone: libc::c_int,
                           org: *mut libc::c_float, ang: *mut libc::c_float);
    #[no_mangle]
    static mut svc_strings: [*const libc::c_char; 60];
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    fn Mod_PointInLeaf(p: *const vec_t, node: *mut mnode_t) -> *mut mleaf_t;
    #[no_mangle]
    fn MSG_WriteChar(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteLong(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteCoord(sb: *mut sizebuf_t, val: libc::c_float);
    #[no_mangle]
    fn MSG_WriteBytes(sb: *mut sizebuf_t, pBuf: *const libc::c_void,
                      nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut sv_novis: *mut convar_t;
    #[no_mangle]
    static mut sv_validate_changelevel: *mut convar_t;
    #[no_mangle]
    fn SV_ModelIndex(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn SV_EventIndex(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn SV_GenericIndex(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn SV_InitOperatorCommands();
    #[no_mangle]
    fn SV_KillOperatorCommands();
    #[no_mangle]
    fn SV_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn SV_DeactivateServer();
    #[no_mangle]
    fn SV_InitPhysicsAPI() -> qboolean;
    #[no_mangle]
    fn SV_MoveStep(ent: *mut edict_t, move_0: *mut vec_t, relink: qboolean)
     -> qboolean;
    #[no_mangle]
    fn SV_MoveTest(ent: *mut edict_t, move_0: *mut vec_t, relink: qboolean)
     -> qboolean;
    #[no_mangle]
    fn SV_MoveToOrigin(ed: *mut edict_t, goal: *const vec_t,
                       dist: libc::c_float, iMode: libc::c_int);
    #[no_mangle]
    fn SV_CheckBottom(ent: *mut edict_t, iMode: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_VecToYaw(src: *const vec_t) -> libc::c_float;
    #[no_mangle]
    fn SV_ClientPrintf(cl: *mut sv_client_t, fmt: *const libc::c_char,
                       _: ...);
    #[no_mangle]
    fn SV_GetClientIDString(cl: *mut sv_client_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_FakeConnect(netname: *const libc::c_char) -> *mut edict_t;
    #[no_mangle]
    fn SV_RunCmd(cl: *mut sv_client_t, ucmd: *mut usercmd_t,
                 random_seed: libc::c_int);
    #[no_mangle]
    fn SV_InitClientMove();
    #[no_mangle]
    fn SV_FindBestBaselineForStatic(index: libc::c_int,
                                    baseline: *mut *mut entity_state_t,
                                    to: *mut entity_state_t) -> libc::c_int;
    #[no_mangle]
    fn SV_SkipUpdates();
    #[no_mangle]
    fn SV_InitSaveRestore();
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    fn SV_LightForEntity(pEdict: *mut edict_t) -> libc::c_int;
    #[no_mangle]
    fn SV_UnlinkEdict(ent: *mut edict_t);
    #[no_mangle]
    fn SV_LinkEdict(ent: *mut edict_t, touch_triggers: qboolean);
    #[no_mangle]
    fn SV_MoveToss(tossent: *mut edict_t, ignore: *mut edict_t) -> trace_t;
    #[no_mangle]
    fn SV_CustomClipMoveToEntity(ent: *mut edict_t, start: *const vec_t,
                                 mins: *mut vec_t, maxs: *mut vec_t,
                                 end: *const vec_t, trace: *mut trace_t);
    #[no_mangle]
    fn SV_ClipMoveToEntity(ent: *mut edict_t, start: *const vec_t,
                           mins: *mut vec_t, maxs: *mut vec_t,
                           end: *const vec_t, trace: *mut trace_t);
    #[no_mangle]
    fn SV_TraceTexture(ent: *mut edict_t, start: *const vec_t,
                       end: *const vec_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_Move(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
               end: *const vec_t, type_0: libc::c_int, e: *mut edict_t,
               monsterclip: qboolean) -> trace_t;
    #[no_mangle]
    fn SV_PointContents(p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn SV_SetLightStyle(style: libc::c_int, s: *const libc::c_char,
                        f: libc::c_float);
    #[no_mangle]
    fn Delta_Init();
    #[no_mangle]
    fn Delta_Shutdown();
    #[no_mangle]
    fn Delta_AddEncoder(name: *mut libc::c_char, encodeFunc: pfnDeltaEncode);
    #[no_mangle]
    fn Delta_FindField(pFields: *mut delta_t, fieldname: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn Delta_SetField(pFields: *mut delta_t, fieldname: *const libc::c_char);
    #[no_mangle]
    fn Delta_UnsetField(pFields: *mut delta_t,
                        fieldname: *const libc::c_char);
    #[no_mangle]
    fn Delta_SetFieldByIndex(pFields: *mut delta_t, fieldNumber: libc::c_int);
    #[no_mangle]
    fn Delta_UnsetFieldByIndex(pFields: *mut delta_t,
                               fieldNumber: libc::c_int);
    #[no_mangle]
    fn MSG_WriteDeltaEvent(msg: *mut sizebuf_t, from: *mut event_args_s,
                           to: *mut event_args_s);
    #[no_mangle]
    fn MSG_WriteDeltaEntity(from: *mut entity_state_s,
                            to: *mut entity_state_s, msg: *mut sizebuf_t,
                            force: qboolean, type_0: libc::c_int,
                            timebase: libc::c_double, ofs: libc::c_int);
    #[no_mangle]
    fn COM_LoadLibrary(dllname: *const libc::c_char,
                       build_ordinals_table: libc::c_int,
                       directpath: qboolean) -> *mut libc::c_void;
    #[no_mangle]
    fn COM_GetProcAddress(hInstance: *mut libc::c_void,
                          name: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn COM_NameForFunction(hInstance: *mut libc::c_void,
                           function: *mut libc::c_void)
     -> *const libc::c_char;
    #[no_mangle]
    fn COM_FunctionFromName_SR(hInstance: *mut libc::c_void,
                               pName: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn COM_FreeLibrary(hInstance: *mut libc::c_void);
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
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
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DEV_EXTENDED: C2RustUnnamed_0 = 2;
pub const DEV_NORMAL: C2RustUnnamed_0 = 1;
pub const DEV_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed_1 = 1;
pub const HOST_NORMAL: C2RustUnnamed_1 = 0;
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
pub struct dlump_t {
    pub fileofs: libc::c_int,
    pub filelen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dheader_t {
    pub version: libc::c_int,
    pub lumps: [dlump_t; 15],
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
pub type SAVERESTOREDATA = saverestore_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TYPEDESCRIPTION {
    pub fieldType: FIELDTYPE,
    pub fieldName: *const libc::c_char,
    pub fieldOffset: libc::c_int,
    pub fieldSize: libc::c_short,
    pub flags: libc::c_short,
}
pub type FIELDTYPE = _fieldtypes;
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
pub type KeyValueData = KeyValueData_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyValueData_s {
    pub szClassName: *mut libc::c_char,
    pub szKeyName: *mut libc::c_char,
    pub szValue: *mut libc::c_char,
    pub fHandled: libc::c_int,
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
pub type clientdata_t = clientdata_s;
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
pub type decallist_t = decallist_s;
pub type ALERT_TYPE = libc::c_uint;
pub const at_logged: ALERT_TYPE = 5;
pub const at_error: ALERT_TYPE = 4;
pub const at_warning: ALERT_TYPE = 3;
pub const at_aiconsole: ALERT_TYPE = 2;
pub const at_console: ALERT_TYPE = 1;
pub const at_notice: ALERT_TYPE = 0;
pub type PRINT_TYPE = libc::c_uint;
pub const print_chat: PRINT_TYPE = 2;
pub const print_center: PRINT_TYPE = 1;
pub const print_console: PRINT_TYPE = 0;
pub type FORCE_TYPE = libc::c_uint;
pub const force_model_specifybounds: FORCE_TYPE = 2;
pub const force_model_samebounds: FORCE_TYPE = 1;
pub const force_exactfile: FORCE_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceResult {
    pub fAllSolid: libc::c_int,
    pub fStartSolid: libc::c_int,
    pub fInOpen: libc::c_int,
    pub fInWater: libc::c_int,
    pub flFraction: libc::c_float,
    pub vecEndPos: vec3_t,
    pub flPlaneDist: libc::c_float,
    pub vecPlaneNormal: vec3_t,
    pub pHit: *mut edict_t,
    pub iHitgroup: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta_s {
    pub name: *const libc::c_char,
    pub offset: libc::c_int,
    pub size: libc::c_int,
    pub flags: libc::c_int,
    pub multiplier: libc::c_float,
    pub post_multiplier: libc::c_float,
    pub bits: libc::c_int,
    pub bInactive: qboolean,
}
pub type delta_t = delta_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct enginefuncs_s {
    pub pfnPrecacheModel: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> libc::c_int>,
    pub pfnPrecacheSound: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> libc::c_int>,
    pub pfnSetModel: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                 _: *const libc::c_char)
                                -> ()>,
    pub pfnModelIndex: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> libc::c_int>,
    pub pfnModelFrames: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> libc::c_int>,
    pub pfnSetSize: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                _: *const libc::c_float,
                                                _: *const libc::c_float)
                               -> ()>,
    pub pfnChangeLevel: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char)
                                   -> ()>,
    pub pfnGetSpawnParms: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnSaveSpawnParms: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> ()>,
    pub pfnVecToYaw: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                -> libc::c_float>,
    pub pfnVecToAngles: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *mut libc::c_float)
                                   -> ()>,
    pub pfnMoveToOrigin: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *const libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_int) -> ()>,
    pub pfnChangeYaw: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnChangePitch: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnFindEntityByString: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                           _:
                                                               *const libc::c_char,
                                                           _:
                                                               *const libc::c_char)
                                          -> *mut edict_t>,
    pub pfnGetEntityIllum: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> libc::c_int>,
    pub pfnFindEntityInSphere: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                           _:
                                                               *const libc::c_float,
                                                           _: libc::c_float)
                                          -> *mut edict_t>,
    pub pfnFindClientInPVS: Option<unsafe extern "C" fn(_: *mut edict_t)
                                       -> *mut edict_t>,
    pub pfnEntitiesInPVS: Option<unsafe extern "C" fn(_: *mut edict_t)
                                     -> *mut edict_t>,
    pub pfnMakeVectors: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                   -> ()>,
    pub pfnAngleVectors: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> ()>,
    pub pfnCreateEntity: Option<unsafe extern "C" fn() -> *mut edict_t>,
    pub pfnRemoveEntity: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnCreateNamedEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> *mut edict_t>,
    pub pfnMakeStatic: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnEntIsOnFloor: Option<unsafe extern "C" fn(_: *mut edict_t)
                                    -> libc::c_int>,
    pub pfnDropToFloor: Option<unsafe extern "C" fn(_: *mut edict_t)
                                   -> libc::c_int>,
    pub pfnWalkMove: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                 _: libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub pfnSetOrigin: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                  _: *const libc::c_float)
                                 -> ()>,
    pub pfnEmitSound: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub pfnEmitAmbientSound: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                         _:
                                                             *mut libc::c_float,
                                                         _:
                                                             *const libc::c_char,
                                                         _: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
    pub pfnTraceLine: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_int,
                                                  _: *mut edict_t,
                                                  _: *mut TraceResult) -> ()>,
    pub pfnTraceToss: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                  _: *mut edict_t,
                                                  _: *mut TraceResult) -> ()>,
    pub pfnTraceMonsterHull: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                         _:
                                                             *const libc::c_float,
                                                         _:
                                                             *const libc::c_float,
                                                         _: libc::c_int,
                                                         _: *mut edict_t,
                                                         _: *mut TraceResult)
                                        -> libc::c_int>,
    pub pfnTraceHull: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut edict_t,
                                                  _: *mut TraceResult) -> ()>,
    pub pfnTraceModel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: *mut edict_t,
                                                   _: *mut TraceResult)
                                  -> ()>,
    pub pfnTraceTexture: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *const libc::c_float,
                                                     _: *const libc::c_float)
                                    -> *const libc::c_char>,
    pub pfnTraceSphere: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: *mut edict_t,
                                                    _: *mut TraceResult)
                                   -> ()>,
    pub pfnGetAimVector: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> ()>,
    pub pfnServerCommand: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> ()>,
    pub pfnServerExecute: Option<unsafe extern "C" fn() -> ()>,
    pub pfnClientCommand: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *mut libc::c_char,
                                                      _: ...) -> ()>,
    pub pfnParticleEffect: Option<unsafe extern "C" fn(_:
                                                           *const libc::c_float,
                                                       _:
                                                           *const libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float)
                                      -> ()>,
    pub pfnLightStyle: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *const libc::c_char)
                                  -> ()>,
    pub pfnDecalIndex: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> libc::c_int>,
    pub pfnPointContents: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                     -> libc::c_int>,
    pub pfnMessageBegin: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *const libc::c_float,
                                                     _: *mut edict_t) -> ()>,
    pub pfnMessageEnd: Option<unsafe extern "C" fn() -> ()>,
    pub pfnWriteByte: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnWriteChar: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnWriteShort: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnWriteLong: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnWriteAngle: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    pub pfnWriteCoord: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    pub pfnWriteString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> ()>,
    pub pfnWriteEntity: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnCVarRegister: Option<unsafe extern "C" fn(_: *mut cvar_t) -> ()>,
    pub pfnCVarGetFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnCVarGetString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub pfnCVarSetFloat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: libc::c_float) -> ()>,
    pub pfnCVarSetString: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *const libc::c_char)
                                     -> ()>,
    pub pfnAlertMessage: Option<unsafe extern "C" fn(_: ALERT_TYPE,
                                                     _: *mut libc::c_char,
                                                     _: ...) -> ()>,
    pub pfnEngineFprintf: Option<unsafe extern "C" fn(_: *mut FILE,
                                                      _: *mut libc::c_char,
                                                      _: ...) -> ()>,
    pub pfnPvAllocEntPrivateData: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                              _: libc::c_long)
                                             -> *mut libc::c_void>,
    pub pfnPvEntPrivateData: Option<unsafe extern "C" fn(_: *mut edict_t)
                                        -> *mut libc::c_void>,
    pub pfnFreeEntPrivateData: Option<unsafe extern "C" fn(_: *mut edict_t)
                                          -> ()>,
    pub pfnSzFromIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *const libc::c_char>,
    pub pfnAllocString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
    pub pfnGetVarsOfEnt: Option<unsafe extern "C" fn(_: *mut edict_t)
                                    -> *mut entvars_s>,
    pub pfnPEntityOfEntOffset: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> *mut edict_t>,
    pub pfnEntOffsetOfPEntity: Option<unsafe extern "C" fn(_: *const edict_t)
                                          -> libc::c_int>,
    pub pfnIndexOfEdict: Option<unsafe extern "C" fn(_: *const edict_t)
                                    -> libc::c_int>,
    pub pfnPEntityOfEntIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> *mut edict_t>,
    pub pfnFindEntityByVars: Option<unsafe extern "C" fn(_: *mut entvars_s)
                                        -> *mut edict_t>,
    pub pfnGetModelPtr: Option<unsafe extern "C" fn(_: *mut edict_t)
                                   -> *mut libc::c_void>,
    pub pfnRegUserMsg: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnAnimationAutomove: Option<unsafe extern "C" fn(_: *const edict_t,
                                                          _: libc::c_float)
                                         -> ()>,
    pub pfnGetBonePosition: Option<unsafe extern "C" fn(_: *const edict_t,
                                                        _: libc::c_int,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float)
                                       -> ()>,
    pub pfnFunctionFromName: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char)
                                        -> libc::c_ulong>,
    pub pfnNameForFunction: Option<unsafe extern "C" fn(_: libc::c_ulong)
                                       -> *const libc::c_char>,
    pub pfnClientPrintf: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: PRINT_TYPE,
                                                     _: *const libc::c_char)
                                    -> ()>,
    pub pfnServerPrint: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> ()>,
    pub pfnCmd_Args: Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub pfnCmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                                -> *const libc::c_char>,
    pub pfnCmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetAttachment: Option<unsafe extern "C" fn(_: *const edict_t,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float)
                                     -> ()>,
    pub pfnCRC32_Init: Option<unsafe extern "C" fn(_: *mut CRC32_t) -> ()>,
    pub pfnCRC32_ProcessBuffer: Option<unsafe extern "C" fn(_: *mut CRC32_t,
                                                            _:
                                                                *const libc::c_void,
                                                            _: libc::c_int)
                                           -> ()>,
    pub pfnCRC32_ProcessByte: Option<unsafe extern "C" fn(_: *mut CRC32_t,
                                                          _: libc::c_uchar)
                                         -> ()>,
    pub pfnCRC32_Final: Option<unsafe extern "C" fn(_: CRC32_t) -> CRC32_t>,
    pub pfnRandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnRandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                    _: libc::c_float)
                                   -> libc::c_float>,
    pub pfnSetView: Option<unsafe extern "C" fn(_: *const edict_t,
                                                _: *const edict_t) -> ()>,
    pub pfnTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub pfnCrosshairAngle: Option<unsafe extern "C" fn(_: *const edict_t,
                                                       _: libc::c_float,
                                                       _: libc::c_float)
                                      -> ()>,
    pub pfnLoadFileForMe: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut libc::c_int)
                                     -> *mut byte>,
    pub pfnFreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub pfnEndSection: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> ()>,
    pub pfnCompareFileTime: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int>,
    pub pfnGetGameDir: Option<unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> ()>,
    pub pfnCvar_RegisterVariable: Option<unsafe extern "C" fn(_: *mut cvar_t)
                                             -> ()>,
    pub pfnFadeClientVolume: Option<unsafe extern "C" fn(_: *const edict_t,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
    pub pfnSetClientMaxspeed: Option<unsafe extern "C" fn(_: *const edict_t,
                                                          _: libc::c_float)
                                         -> ()>,
    pub pfnCreateFakeClient: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char)
                                        -> *mut edict_t>,
    pub pfnRunPlayerMove: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *const libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_ushort,
                                                      _: byte, _: byte)
                                     -> ()>,
    pub pfnNumberOfEntities: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetInfoKeyBuffer: Option<unsafe extern "C" fn(_: *mut edict_t)
                                        -> *mut libc::c_char>,
    pub pfnInfoKeyValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *const libc::c_char)
                                    -> *const libc::c_char>,
    pub pfnSetKeyValue: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: *mut libc::c_char,
                                                    _: *mut libc::c_char)
                                   -> ()>,
    pub pfnSetClientKeyValue: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_char,
                                                          _:
                                                              *mut libc::c_char,
                                                          _:
                                                              *mut libc::c_char)
                                         -> ()>,
    pub pfnIsMapValid: Option<unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> libc::c_int>,
    pub pfnStaticDecal: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub pfnPrecacheGeneric: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char)
                                       -> libc::c_int>,
    pub pfnGetPlayerUserId: Option<unsafe extern "C" fn(_: *mut edict_t)
                                       -> libc::c_int>,
    pub pfnBuildSoundMsg: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: libc::c_int,
                                                      _: *const libc::c_char,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_float,
                                                      _: *mut edict_t) -> ()>,
    pub pfnIsDedicatedServer: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnCVarGetPointer: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut cvar_t>,
    pub pfnGetPlayerWONId: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> libc::c_uint>,
    pub pfnInfo_RemoveKey: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                       _: *const libc::c_char)
                                      -> ()>,
    pub pfnGetPhysicsKeyValue: Option<unsafe extern "C" fn(_: *const edict_t,
                                                           _:
                                                               *const libc::c_char)
                                          -> *const libc::c_char>,
    pub pfnSetPhysicsKeyValue: Option<unsafe extern "C" fn(_: *const edict_t,
                                                           _:
                                                               *const libc::c_char,
                                                           _:
                                                               *const libc::c_char)
                                          -> ()>,
    pub pfnGetPhysicsInfoString: Option<unsafe extern "C" fn(_:
                                                                 *const edict_t)
                                            -> *const libc::c_char>,
    pub pfnPrecacheEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_char)
                                     -> libc::c_ushort>,
    pub pfnPlaybackEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const edict_t,
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
    pub pfnSetFatPVS: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> *mut libc::c_uchar>,
    pub pfnSetFatPAS: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> *mut libc::c_uchar>,
    pub pfnCheckVisibility: Option<unsafe extern "C" fn(_: *const edict_t,
                                                        _: *mut libc::c_uchar)
                                       -> libc::c_int>,
    pub pfnDeltaSetField: Option<unsafe extern "C" fn(_: *mut delta_s,
                                                      _: *const libc::c_char)
                                     -> ()>,
    pub pfnDeltaUnsetField: Option<unsafe extern "C" fn(_: *mut delta_s,
                                                        _:
                                                            *const libc::c_char)
                                       -> ()>,
    pub pfnDeltaAddEncoder: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _:
                                                            Option<unsafe extern "C" fn(_:
                                                                                            *mut delta_s,
                                                                                        _:
                                                                                            *const libc::c_uchar,
                                                                                        _:
                                                                                            *const libc::c_uchar)
                                                                       -> ()>)
                                       -> ()>,
    pub pfnGetCurrentPlayer: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnCanSkipPlayer: Option<unsafe extern "C" fn(_: *const edict_t)
                                     -> libc::c_int>,
    pub pfnDeltaFindField: Option<unsafe extern "C" fn(_: *mut delta_s,
                                                       _: *const libc::c_char)
                                      -> libc::c_int>,
    pub pfnDeltaSetFieldByIndex: Option<unsafe extern "C" fn(_: *mut delta_s,
                                                             _: libc::c_int)
                                            -> ()>,
    pub pfnDeltaUnsetFieldByIndex: Option<unsafe extern "C" fn(_:
                                                                   *mut delta_s,
                                                               _: libc::c_int)
                                              -> ()>,
    pub pfnSetGroupMask: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub pfnCreateInstancedBaseline: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut entity_state_s)
                                               -> libc::c_int>,
    pub pfnCvar_DirectSet: Option<unsafe extern "C" fn(_: *mut cvar_s,
                                                       _: *const libc::c_char)
                                      -> ()>,
    pub pfnForceUnmodified: Option<unsafe extern "C" fn(_: FORCE_TYPE,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _:
                                                            *const libc::c_char)
                                       -> ()>,
    pub pfnGetPlayerStats: Option<unsafe extern "C" fn(_: *const edict_t,
                                                       _: *mut libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> ()>,
    pub pfnAddServerCommand: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             Option<unsafe extern "C" fn()
                                                                        ->
                                                                            ()>)
                                        -> ()>,
    pub pfnVoice_GetClientListening: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> qboolean>,
    pub pfnVoice_SetClientListening: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _: qboolean)
                                                -> qboolean>,
    pub pfnGetPlayerAuthId: Option<unsafe extern "C" fn(_: *mut edict_t)
                                       -> *const libc::c_char>,
    pub pfnSequenceGet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char)
                                   -> *mut libc::c_void>,
    pub pfnSequencePickSentence: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> *mut libc::c_void>,
    pub pfnGetFileSize: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
    pub pfnGetApproxWavePlayLen: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_uint>,
    pub pfnIsCareerMatch: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetLocalizedStringLength: Option<unsafe extern "C" fn(_:
                                                                     *const libc::c_char)
                                                -> libc::c_int>,
    pub pfnRegisterTutorMessageShown: Option<unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()>,
    pub pfnGetTimesTutorMessageShown: Option<unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> libc::c_int>,
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
    pub pfnQueryClientCvarValue: Option<unsafe extern "C" fn(_:
                                                                 *const edict_t,
                                                             _:
                                                                 *const libc::c_char)
                                            -> ()>,
    pub pfnQueryClientCvarValue2: Option<unsafe extern "C" fn(_:
                                                                  *const edict_t,
                                                              _:
                                                                  *const libc::c_char,
                                                              _: libc::c_int)
                                             -> ()>,
    pub pfnCheckParm: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                  _: *mut *mut libc::c_char)
                                 -> libc::c_int>,
}
pub type enginefuncs_t = enginefuncs_s;
pub type NEW_DLL_FUNCTIONS_FN
    =
    Option<unsafe extern "C" fn(_: *mut NEW_DLL_FUNCTIONS,
                                _: *mut libc::c_int) -> libc::c_int>;
pub type APIFUNCTION
    =
    Option<unsafe extern "C" fn(_: *mut DLL_FUNCTIONS, _: libc::c_int)
               -> libc::c_int>;
pub type APIFUNCTION2
    =
    Option<unsafe extern "C" fn(_: *mut DLL_FUNCTIONS, _: *mut libc::c_int)
               -> libc::c_int>;
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
pub type cl_entity_t = cl_entity_s;
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
pub type particle_t = particle_s;
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
pub type BEAM = beam_s;
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
pub type ref_screen_rotation_t = ref_screen_rotation_e;
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
pub struct str64_s {
    pub maxstringarray: size_t,
    pub allowdup: qboolean,
    pub staticstringarray: *mut libc::c_char,
    pub pstringarray: *mut libc::c_char,
    pub pstringarraystatic: *mut libc::c_char,
    pub pstringbase: *mut libc::c_char,
    pub poldstringbase: *mut libc::c_char,
    pub plast: *mut libc::c_char,
    pub dynamic: qboolean,
    pub maxalloc: size_t,
    pub numdups: size_t,
    pub numoverflows: size_t,
    pub totalalloc: size_t,
}
pub type GIVEFNPTRSTODLL
    =
    Option<unsafe extern "C" fn(_: *mut enginefuncs_t, _: *mut globalvars_t)
               -> ()>;
pub type pfnDeltaEncode
    =
    Option<unsafe extern "C" fn(_: *mut delta_s, _: *const byte,
                                _: *const byte) -> ()>;
pub const DELTA_STATIC: C2RustUnnamed_2 = 2;
// exports
pub type LINK_ENTITY_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut entvars_t) -> ()>;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DELTA_PLAYER: C2RustUnnamed_2 = 1;
pub const DELTA_ENTITY: C2RustUnnamed_2 = 0;
#[inline]
unsafe extern "C" fn BitByte(mut bits: libc::c_int) -> libc::c_int {
    return (bits + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int *
               8 as libc::c_int >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return MSG_GetNumBitsLeft(sb) >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline(always)]
unsafe extern "C" fn __tg_cos(mut __x: libc::c_float) -> libc::c_float {
    return cosf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_float) -> libc::c_float {
    return sinf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return BitByte((*sb).iCurBit);
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
// fatpvs stuff
static mut fatpvs: [byte; 4095] = [0; 4095];
static mut fatphs: [byte; 4095] = [0; 4095];
static mut clientpvs: [byte; 4095] = [0; 4095];
// for find client in PVS
static mut viewPoint: [vec3_t; 32] = [[0.; 3]; 32];
#[no_mangle]
pub unsafe extern "C" fn SV_EdictNum(mut n: libc::c_int) -> *mut edict_t {
    if n >= 0 as libc::c_int && n < (*SI.GameInfo).max_edicts {
        return svgame.edicts.offset(n as isize)
    } // may be NULL
    return 0 as *mut edict_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_CheckEdict(mut e: *const edict_t,
                                       mut file: *const libc::c_char,
                                       line: libc::c_int) -> qboolean {
    let mut n: libc::c_int = 0;
    if e.is_null() { return false_0 }
    n =
        (e as *mut edict_t).wrapping_offset_from(svgame.edicts) as
            libc::c_long as libc::c_int;
    if n >= 0 as libc::c_int && n < (*SI.GameInfo).max_edicts {
        return ((*e).free as u64 == 0) as libc::c_int as qboolean
    }
    Con_Printf(b"bad entity %i (called at %s:%i)\n\x00" as *const u8 as
                   *const libc::c_char, n, file, line);
    return false_0;
}
/*
=============
EntvarsDescription

entavrs table for FindEntityByString
=============
*/
static mut gEntvarsDescription: [TYPEDESCRIPTION; 13] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"classname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"globalname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 4 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_MODELNAME,
                             fieldName:
                                 b"model\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 184 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_MODELNAME,
                             fieldName:
                                 b"viewmodel\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 188 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_MODELNAME,
                             fieldName:
                                 b"weaponmodel\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 192 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"target\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 480 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"targetname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 484 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"netname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 488 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"message\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 492 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SOUNDNAME,
                             fieldName:
                                 b"noise\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 512 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SOUNDNAME,
                             fieldName:
                                 b"noise1\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 516 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SOUNDNAME,
                             fieldName:
                                 b"noise2\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 520 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SOUNDNAME,
                             fieldName:
                                 b"noise3\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 524 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
/*
=============
SV_GetEntvarsDescription

entavrs table for FindEntityByString
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetEntvarsDescirption(mut number: libc::c_int)
 -> *mut TYPEDESCRIPTION {
    if number < 0 as libc::c_int &&
           number as libc::c_ulong >=
               (::std::mem::size_of::<[TYPEDESCRIPTION; 13]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                    as libc::c_ulong) {
        return 0 as *mut TYPEDESCRIPTION
    }
    return &mut *gEntvarsDescription.as_mut_ptr().offset(number as isize) as
               *mut TYPEDESCRIPTION;
}
/*
=============
SV_SysError

tell the game.dll about system error
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SysError(mut error_string: *const libc::c_char) {
    Log_Printf(b"FATAL ERROR (shutting down): %s\n\x00" as *const u8 as
                   *const libc::c_char, error_string);
    if !svgame.hInstance.is_null() {
        svgame.dllFuncs.pfnSys_Error.expect("non-null function pointer")(error_string);
    };
}
/*
=============
SV_Serverinfo

get server infostring
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Serverinfo() -> *mut libc::c_char {
    return svs.serverinfo.as_mut_ptr();
}
/*
=============
SV_LocalInfo

get local infostring
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Localinfo() -> *mut libc::c_char {
    return svs.localinfo.as_mut_ptr();
}
/*
=============
SV_AngleMod

do modulo on entity angles
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AngleMod(mut ideal: libc::c_float,
                                     mut current: libc::c_float,
                                     mut speed: libc::c_float)
 -> libc::c_float {
    let mut move_0: libc::c_float = 0.;
    current = anglemod(current);
    if current == ideal {
        // already there?
        return current
    }
    move_0 = ideal - current;
    if ideal > current {
        if move_0 >= 180 as libc::c_int as libc::c_float {
            move_0 = move_0 - 360 as libc::c_int as libc::c_float
        }
    } else if move_0 <= -(180 as libc::c_int) as libc::c_float {
        move_0 = move_0 + 360 as libc::c_int as libc::c_float
    }
    if move_0 > 0 as libc::c_int as libc::c_float {
        if move_0 > speed { move_0 = speed }
    } else if move_0 < -speed { move_0 = -speed }
    return anglemod(current + move_0);
}
/*
=============
SV_SetMinMaxSize

update entity bounds, relink into world
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SetMinMaxSize(mut e: *mut edict_t,
                                          mut mins: *const libc::c_float,
                                          mut maxs: *const libc::c_float,
                                          mut relink: qboolean) {
    let mut i: libc::c_int = 0; // just relink edict and exit
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 194 as libc::c_int) as u64 == 0
       {
        return
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *mins.offset(i as isize) > *maxs.offset(i as isize) {
            Con_Printf(b"^1Error:^7 %s[%i] has backwards mins/maxs\n\x00" as
                           *const u8 as *const libc::c_char, SV_ClassName(e),
                       e.wrapping_offset_from(svgame.edicts) as libc::c_long
                           as libc::c_int);
            if relink as u64 != 0 { SV_LinkEdict(e, false_0); }
            return
        }
        i += 1
    }
    (*e).v.mins[0 as libc::c_int as usize] =
        *mins.offset(0 as libc::c_int as isize);
    (*e).v.mins[1 as libc::c_int as usize] =
        *mins.offset(1 as libc::c_int as isize);
    (*e).v.mins[2 as libc::c_int as usize] =
        *mins.offset(2 as libc::c_int as isize);
    (*e).v.maxs[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize);
    (*e).v.maxs[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize);
    (*e).v.maxs[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize);
    (*e).v.size[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize) -
            *mins.offset(0 as libc::c_int as isize);
    (*e).v.size[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize) -
            *mins.offset(1 as libc::c_int as isize);
    (*e).v.size[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize) -
            *mins.offset(2 as libc::c_int as isize);
    if relink as u64 != 0 { SV_LinkEdict(e, false_0); };
}
/*
=============
SV_CopyTraceToGlobal

each trace will share their result into global state
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CopyTraceToGlobal(mut trace: *mut trace_t) {
    (*svgame.globals).trace_allsolid =
        (*trace).allsolid as
            libc::c_float; // g-cont: always reset config flags when trace is finished
    (*svgame.globals).trace_startsolid = (*trace).startsolid as libc::c_float;
    (*svgame.globals).trace_fraction = (*trace).fraction;
    (*svgame.globals).trace_plane_dist = (*trace).plane.dist;
    (*svgame.globals).trace_inopen = (*trace).inopen as libc::c_float;
    (*svgame.globals).trace_inwater = (*trace).inwater as libc::c_float;
    (*svgame.globals).trace_endpos[0 as libc::c_int as usize] =
        (*trace).endpos[0 as libc::c_int as usize];
    (*svgame.globals).trace_endpos[1 as libc::c_int as usize] =
        (*trace).endpos[1 as libc::c_int as usize];
    (*svgame.globals).trace_endpos[2 as libc::c_int as usize] =
        (*trace).endpos[2 as libc::c_int as usize];
    (*svgame.globals).trace_plane_normal[0 as libc::c_int as usize] =
        (*trace).plane.normal[0 as libc::c_int as usize];
    (*svgame.globals).trace_plane_normal[1 as libc::c_int as usize] =
        (*trace).plane.normal[1 as libc::c_int as usize];
    (*svgame.globals).trace_plane_normal[2 as libc::c_int as usize] =
        (*trace).plane.normal[2 as libc::c_int as usize];
    (*svgame.globals).trace_hitgroup = (*trace).hitgroup;
    (*svgame.globals).trace_flags = 0 as libc::c_int;
    if SV_CheckEdict((*trace).ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 233 as libc::c_int) as u64 != 0
       {
        (*svgame.globals).trace_ent = (*trace).ent
    } else { (*svgame.globals).trace_ent = svgame.edicts };
}
/*
=============
SV_ConvertTrace

convert trace_t to TraceResult
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ConvertTrace(mut dst: *mut TraceResult,
                                         mut src: *mut trace_t) {
    if src.is_null() || dst.is_null() { return }
    (*dst).fAllSolid = (*src).allsolid as libc::c_int;
    (*dst).fStartSolid = (*src).startsolid as libc::c_int;
    (*dst).fInOpen = (*src).inopen as libc::c_int;
    (*dst).fInWater = (*src).inwater as libc::c_int;
    (*dst).flFraction = (*src).fraction;
    (*dst).vecEndPos[0 as libc::c_int as usize] =
        (*src).endpos[0 as libc::c_int as usize];
    (*dst).vecEndPos[1 as libc::c_int as usize] =
        (*src).endpos[1 as libc::c_int as usize];
    (*dst).vecEndPos[2 as libc::c_int as usize] =
        (*src).endpos[2 as libc::c_int as usize];
    (*dst).flPlaneDist = (*src).plane.dist;
    (*dst).vecPlaneNormal[0 as libc::c_int as usize] =
        (*src).plane.normal[0 as libc::c_int as usize];
    (*dst).vecPlaneNormal[1 as libc::c_int as usize] =
        (*src).plane.normal[1 as libc::c_int as usize];
    (*dst).vecPlaneNormal[2 as libc::c_int as usize] =
        (*src).plane.normal[2 as libc::c_int as usize];
    (*dst).pHit = (*src).ent;
    (*dst).iHitgroup = (*src).hitgroup;
    // g-cont: always reset config flags when trace is finished
    (*svgame.globals).trace_flags = 0 as libc::c_int;
}
/*
=============
SV_CheckClientVisiblity

Check visibility through client camera, portal camera, etc
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckClientVisiblity(mut cl: *mut sv_client_t,
                                                 mut mask: *const byte)
 -> qboolean {
    let mut i: libc::c_int = 0; // GoldSrc rules
    let mut clientnum: libc::c_int = 0;
    let mut vieworg: vec3_t = [0.; 3];
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if mask.is_null() { return true_0 }
    clientnum =
        cl.wrapping_offset_from(svs.clients) as libc::c_long as libc::c_int;
    vieworg[0 as libc::c_int as usize] =
        viewPoint[clientnum as usize][0 as libc::c_int as usize];
    vieworg[1 as libc::c_int as usize] =
        viewPoint[clientnum as usize][1 as libc::c_int as usize];
    vieworg[2 as libc::c_int as usize] =
        viewPoint[clientnum as usize][2 as libc::c_int as usize];
    // Invasion issues: wrong camera position received in ENGINE_SET_PVS
    if !(*cl).pViewEntity.is_null() &&
           !(vieworg[0 as libc::c_int as usize] ==
                 (*(*cl).pViewEntity).v.origin[0 as libc::c_int as usize] &&
                 vieworg[1 as libc::c_int as usize] ==
                     (*(*cl).pViewEntity).v.origin[1 as libc::c_int as usize]
                 &&
                 vieworg[2 as libc::c_int as usize] ==
                     (*(*cl).pViewEntity).v.origin[2 as libc::c_int as usize])
       {
        vieworg[0 as libc::c_int as usize] =
            (*(*cl).pViewEntity).v.origin[0 as libc::c_int as
                                              usize]; // visible from player view or camera view
        vieworg[1 as libc::c_int as usize] =
            (*(*cl).pViewEntity).v.origin[1 as libc::c_int as usize];
        vieworg[2 as libc::c_int as usize] =
            (*(*cl).pViewEntity).v.origin[2 as libc::c_int as usize]
    }
    leaf =
        Mod_PointInLeaf(vieworg.as_mut_ptr() as *const vec_t,
                        (*sv.worldmodel).nodes);
    if if (*leaf).cluster >= 0 as libc::c_int {
           (*mask.offset(((*leaf).cluster >> 3 as libc::c_int) as isize) as
                libc::c_int &
                (1 as libc::c_int) << ((*leaf).cluster & 7 as libc::c_int)) as
               byte as libc::c_int
       } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
        return true_0
    }
    // now check all the portal cameras
    i = 0 as libc::c_int;
    while i < (*cl).num_viewents {
        let mut view: *mut edict_t = (*cl).viewentity[i as usize];
        if !(SV_CheckEdict(view,
                           b"../engine/server/sv_game.c\x00" as *const u8 as
                               *const libc::c_char, 296 as libc::c_int) as u64
                 == 0) {
            vieworg[0 as libc::c_int as usize] =
                (*view).v.origin[0 as libc::c_int as usize] +
                    (*view).v.view_ofs[0 as libc::c_int as usize];
            vieworg[1 as libc::c_int as usize] =
                (*view).v.origin[1 as libc::c_int as usize] +
                    (*view).v.view_ofs[1 as libc::c_int as usize];
            vieworg[2 as libc::c_int as usize] =
                (*view).v.origin[2 as libc::c_int as usize] +
                    (*view).v.view_ofs[2 as libc::c_int as usize];
            leaf =
                Mod_PointInLeaf(vieworg.as_mut_ptr() as *const vec_t,
                                (*sv.worldmodel).nodes);
            if if (*leaf).cluster >= 0 as libc::c_int {
                   (*mask.offset(((*leaf).cluster >> 3 as libc::c_int) as
                                     isize) as libc::c_int &
                        (1 as libc::c_int) <<
                            ((*leaf).cluster & 7 as libc::c_int)) as byte as
                       libc::c_int
               } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
                return true_0
            }
        }
        i += 1
        // visible from portal camera view
    }
    // not visible from any viewpoint
    return false_0;
}
/*
=================
SV_Multicast

Sends the contents of sv.multicast to a subset of the clients,
then clears sv.multicast.

MSG_INIT	write message into signon buffer
MSG_ONE	send to one client (ent can't be NULL)
MSG_ALL	same as broadcast (origin can be NULL)
MSG_PVS	send to clients potentially visible from org
MSG_PHS	send to clients potentially audible from org
=================
*/
unsafe extern "C" fn SV_Multicast(mut dest: libc::c_int,
                                  mut origin: *const vec_t,
                                  mut ent: *const edict_t,
                                  mut usermessage: qboolean,
                                  mut filter: qboolean) -> libc::c_int {
    let mut mask: *mut byte = 0 as *mut byte;
    let mut j: libc::c_int = 0;
    let mut numclients: libc::c_int = svs.maxclients;
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut current: *mut sv_client_t = svs.clients;
    let mut reliable: qboolean = false_0;
    let mut specproxy: qboolean = false_0;
    let mut numsends: libc::c_int = 0 as libc::c_int;
    // some mods trying to send messages after SV_FinalMessage
    if svs.initialized as u64 == 0 ||
           sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint
       {
        MSG_Clear(&mut sv.multicast);
        return 0 as libc::c_int
    }
    let mut current_block_29: u64;
    match dest {
        3 => {
            if sv.state as libc::c_uint ==
                   ss_loading as libc::c_int as libc::c_uint {
                // copy to signon buffer
                MSG_WriteBits(&mut sv.signon,
                              MSG_GetData(&mut sv.multicast) as
                                  *const libc::c_void,
                              MSG_GetNumBitsWritten(&mut sv.multicast));
                MSG_Clear(&mut sv.multicast);
                return 1 as libc::c_int
            }
            current_block_29 = 15777170020524123985;
        }
        2 => { current_block_29 = 15777170020524123985; }
        0 => { current_block_29 = 11385396242402735691; }
        7 => { reliable = true_0; current_block_29 = 11385656547815987632; }
        5 => { current_block_29 = 11385656547815987632; }
        6 => { reliable = true_0; current_block_29 = 15468919264548431707; }
        4 => { current_block_29 = 15468919264548431707; }
        1 => { reliable = true_0; current_block_29 = 2665227789553733198; }
        8 => { current_block_29 = 2665227789553733198; }
        9 => {
            reliable = true_0;
            specproxy = reliable;
            current_block_29 = 11385396242402735691;
        }
        _ => {
            Host_Error(b"SV_Multicast: bad dest: %i\n\x00" as *const u8 as
                           *const libc::c_char, dest);
            return 0 as libc::c_int
        }
    }
    match current_block_29 {
        2665227789553733198 =>
        // intentional fallthrough
        {
            if SV_CheckEdict(ent,
                             b"../engine/server/sv_game.c\x00" as *const u8 as
                                 *const libc::c_char, 377 as libc::c_int) as
                   u64 == 0 {
                return 0 as libc::c_int
            } // send to one
            j =
                (ent as *mut edict_t).wrapping_offset_from(svgame.edicts) as
                    libc::c_long as libc::c_int;
            if j < 1 as libc::c_int || j > numclients {
                return 0 as libc::c_int
            }
            current = svs.clients.offset((j - 1 as libc::c_int) as isize);
            numclients = 1 as libc::c_int
        }
        15468919264548431707 =>
        // intentional fallthrough
        {
            if origin.is_null() { return 0 as libc::c_int }
            mask = Mod_GetPVSForPoint(origin)
        }
        11385656547815987632 =>
        // intentional fallthrough
        {
            if origin.is_null() { return false_0 as libc::c_int }
            // NOTE: GoldSource not using PHS for singleplayer
            Mod_FatPVS(origin, 16.0f32, fatphs.as_mut_ptr(),
                       world.fatbytes as libc::c_int, false_0,
                       (svs.maxclients == 1 as libc::c_int) as libc::c_int as
                           qboolean); // using the FatPVS like a PHS
            mask = fatphs.as_mut_ptr()
        }
        15777170020524123985 =>
        // intentional fallthrough (in-game MSG_INIT it's a MSG_ALL reliable)
        {
            reliable = true_0
        }
        _ => { }
    }
    let mut current_block_37: u64;
    // send the data to all relevent clients (or once only)
    j = 0 as libc::c_int;
    cl = current;
    while j < numclients {
        if !((*cl).state as libc::c_uint ==
                 cs_free as libc::c_int as libc::c_uint ||
                 (*cl).state as libc::c_uint ==
                     cs_zombie as libc::c_int as libc::c_uint) {
            if !((*cl).state as libc::c_uint !=
                     cs_spawned as libc::c_int as libc::c_uint &&
                     (reliable as u64 == 0 ||
                          usermessage as libc::c_uint != 0)) {
                if !(specproxy as libc::c_uint != 0 &&
                         (*cl).flags & (1 as libc::c_uint) << 8 as libc::c_int
                             == 0) {
                    if !((*cl).edict.is_null() ||
                             (*cl).flags &
                                 (1 as libc::c_uint) << 7 as libc::c_int != 0)
                       {
                        // reject step sounds while predicting is enabled
		// FIXME: make sure what this code doesn't cutoff something important!!!
                        if !(filter as libc::c_uint != 0 &&
                                 cl == sv.current_client &&
                                 (*sv.current_client).flags &
                                     (1 as libc::c_uint) << 4 as libc::c_int
                                     != 0) {
                            if SV_CheckEdict(ent,
                                             b"../engine/server/sv_game.c\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             411 as libc::c_int) as
                                   libc::c_uint != 0 &&
                                   (*ent).v.groupinfo != 0 &&
                                   (*(*cl).edict).v.groupinfo != 0 {
                                if svs.groupop == 0 as libc::c_int &&
                                       (*(*cl).edict).v.groupinfo &
                                           (*ent).v.groupinfo == 0 {
                                    current_block_37 = 2604890879466389055;
                                } else if svs.groupop == 1 as libc::c_int &&
                                              (*(*cl).edict).v.groupinfo &
                                                  (*ent).v.groupinfo != 0 {
                                    current_block_37 = 2604890879466389055;
                                } else {
                                    current_block_37 = 6174974146017752131;
                                }
                            } else { current_block_37 = 6174974146017752131; }
                            match current_block_37 {
                                2604890879466389055 => { }
                                _ => {
                                    if !(SV_CheckClientVisiblity(cl, mask) as
                                             u64 == 0) {
                                        if specproxy as u64 != 0 {
                                            MSG_WriteBits(&mut sv.spec_datagram,
                                                          MSG_GetData(&mut sv.multicast)
                                                              as
                                                              *const libc::c_void,
                                                          MSG_GetNumBitsWritten(&mut sv.multicast));
                                        } else if reliable as u64 != 0 {
                                            MSG_WriteBits(&mut (*cl).netchan.message,
                                                          MSG_GetData(&mut sv.multicast)
                                                              as
                                                              *const libc::c_void,
                                                          MSG_GetNumBitsWritten(&mut sv.multicast));
                                        } else {
                                            MSG_WriteBits(&mut (*cl).datagram,
                                                          MSG_GetData(&mut sv.multicast)
                                                              as
                                                              *const libc::c_void,
                                                          MSG_GetNumBitsWritten(&mut sv.multicast));
                                        }
                                        numsends += 1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        j += 1;
        cl = cl.offset(1)
    }
    MSG_Clear(&mut sv.multicast);
    return numsends;
    // just for debug
}
/*
=======================
SV_GetReliableDatagram

Get shared reliable buffer
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetReliableDatagram() -> *mut sizebuf_t {
    return &mut sv.reliable_datagram;
}
/*
=======================
SV_RestoreCustomDecal

Let the user spawn decal in game code
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RestoreCustomDecal(mut entry: *mut decallist_t,
                                               mut pEdict: *mut edict_t,
                                               mut adjacent: qboolean)
 -> qboolean {
    if svgame.physFuncs.pfnRestoreDecal.is_some() {
        if pEdict.is_null() {
            pEdict = SV_EdictNum((*entry).entityIndex as libc::c_int)
        }
        // true if decal was sucessfully restored at the game-side
        return svgame.physFuncs.pfnRestoreDecal.expect("non-null function pointer")(entry,
                                                                                    pEdict,
                                                                                    adjacent)
                   as qboolean
    }
    return false_0;
}
/*
=======================
SV_CreateDecal

NOTE: static decals only accepted when game is loading
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateDecal(mut msg: *mut sizebuf_t,
                                        mut origin: *const libc::c_float,
                                        mut decalIndex: libc::c_int,
                                        mut entityIndex: libc::c_int,
                                        mut modelIndex: libc::c_int,
                                        mut flags: libc::c_int,
                                        mut scale: libc::c_float) {
    if msg == &mut sv.signon as *mut sizebuf_t &&
           sv.state as libc::c_uint !=
               ss_loading as libc::c_int as libc::c_uint {
        return
    }
    // this can happens if serialized map contain 4096 static decals...
    if MSG_GetNumBytesLeft(msg) < 20 as libc::c_int {
        sv.ignored_world_decals += 1;
        return
    }
    // static decals are posters, it's always reliable
    MSG_WriteCmdExt(msg, 36 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteVec3Coord(msg, origin);
    MSG_WriteWord(msg, decalIndex);
    MSG_WriteShort(msg, entityIndex);
    if entityIndex > 0 as libc::c_int { MSG_WriteWord(msg, modelIndex); }
    MSG_WriteByte(msg, flags);
    MSG_WriteWord(msg,
                  (scale * 4096 as libc::c_int as libc::c_float) as
                      libc::c_int);
}
/*
=======================
SV_CreateStaticEntity

NOTE: static entities only accepted when game is loading
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateStaticEntity(mut msg: *mut sizebuf_t,
                                               mut index: libc::c_int)
 -> qboolean {
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
                       vuser4: [0.; 3],}; // continue overflowed entities
    let mut baseline: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut offset: libc::c_int = 0;
    if index >= 3096 as libc::c_int - 1 as libc::c_int {
        if sv.static_ents_overflow == 0 {
            Con_Printf(b"^3Warning:^7 MAX_STATIC_ENTITIES limit exceeded (%d)\n\x00"
                           as *const u8 as *const libc::c_char,
                       3096 as libc::c_int);
            sv.static_ents_overflow = true_0 as libc::c_int
        }
        sv.ignored_static_ents += 1;
        return false_0
    }
    // this can happens if serialized map contain too many static entities...
    if MSG_GetNumBytesLeft(msg) < 50 as libc::c_int {
        sv.ignored_static_ents += 1; // allocate a new one
        return false_0
    }
    state =
        &mut *svs.static_entities.offset(index as isize) as
            *mut entity_state_t;
    memset(&mut nullstate as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    baseline = &mut nullstate;
    // restore modelindex from modelname (already precached)
    (*state).modelindex =
        pfnModelIndex(SV_GetString((*state).messagenum)); // select delta-encode
    (*state).entityType = (1 as libc::c_int) << 0 as libc::c_int;
    (*state).number = 0 as libc::c_int;
    // trying to compress with previous delta's
    offset = SV_FindBestBaselineForStatic(index, &mut baseline, state);
    MSG_WriteCmdExt(msg, 20 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteDeltaEntity(baseline, state, msg, true_0,
                         DELTA_STATIC as libc::c_int, sv.time, offset);
    return true_0;
}
/*
=================
SV_RestartStaticEnts

Write all the static ents into demo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RestartStaticEnts() {
    let mut i: libc::c_int = 0;
    // remove all the static entities on the client
    CL_ClearStaticEntities();
    // resend them again
    i = 0 as libc::c_int;
    while i < sv.num_static_entities {
        SV_CreateStaticEntity(&mut sv.reliable_datagram, i);
        i += 1
    };
}
/*
=================
SV_RestartAmbientSounds

Write ambient sounds into demo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RestartAmbientSounds() {
    let mut soundInfo: [soundlist_t; 256] =
        [soundlist_t{name: [0; 64],
                     entnum: 0,
                     origin: [0.; 3],
                     volume: 0.,
                     attenuation: 0.,
                     looping: false_0,
                     channel: 0,
                     pitch: 0,
                     wordIndex: 0,
                     samplePos: 0.,
                     forcedEnd: 0.,}; 256];
    let mut curtrack: string = [0; 256];
    let mut looptrack: string = [0; 256];
    let mut i: libc::c_int = 0;
    let mut nSounds: libc::c_int = 0;
    let mut position: libc::c_int = 0;
    if SV_Active() as u64 == 0 { return }
    nSounds =
        S_GetCurrentStaticSounds(soundInfo.as_mut_ptr(), 256 as libc::c_int);
    i = 0 as libc::c_int;
    while i < nSounds {
        let mut si: *mut soundlist_t =
            &mut *soundInfo.as_mut_ptr().offset(i as isize) as
                *mut soundlist_t;
        if !((*si).looping as u64 == 0 ||
                 (*si).entnum as libc::c_int == -(1 as libc::c_int)) {
            S_StopSound((*si).entnum as libc::c_int,
                        (*si).channel as libc::c_int,
                        (*si).name.as_mut_ptr());
            SV_StartSound(pfnPEntityOfEntIndex((*si).entnum as libc::c_int),
                          6 as libc::c_int, (*si).name.as_mut_ptr(),
                          (*si).volume, (*si).attenuation, 0 as libc::c_int,
                          (*si).pitch as libc::c_int);
        }
        i += 1
    }
    // TODO: ???
    // restart soundtrack
    if S_StreamGetCurrentState(curtrack.as_mut_ptr(), looptrack.as_mut_ptr(),
                               &mut position) as u64 != 0 {
        SV_StartMusic(curtrack.as_mut_ptr(), looptrack.as_mut_ptr(),
                      position);
    };
}
/*
=================
SV_RestartDecals

Write all the decals into demo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RestartDecals() {
    let mut entry: *mut decallist_t = 0 as *mut decallist_t;
    let mut decalIndex: libc::c_int = 0;
    let mut modelIndex: libc::c_int = 0;
    let mut msg: *mut sizebuf_t = 0 as *mut sizebuf_t;
    let mut i: libc::c_int = 0;
    if SV_Active() as u64 == 0 { return }
    // g-cont. add space for studiodecals if present
    host.decalList =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<decallist_t>() as
                        libc::c_ulong).wrapping_mul(4096 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_game.c\x00" as *const u8 as
                       *const libc::c_char, 619 as libc::c_int) as
            *mut decallist_t;
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        host.numdecals =
            ref_0.dllFuncs.R_CreateDecalList.expect("non-null function pointer")(host.decalList);
        // remove decals from map
        ref_0.dllFuncs.R_ClearAllDecals.expect("non-null function pointer")();
    } else {
        // XASH_DEDICATED
        // we probably running a dedicated server
        host.numdecals = 0 as libc::c_int
    }
    // write decals into reliable datagram
    msg = SV_GetReliableDatagram();
    // restore decals and write them into network message
    i = 0 as libc::c_int;
    while i < host.numdecals {
        entry = &mut *host.decalList.offset(i as isize) as *mut decallist_s;
        modelIndex =
            (*pfnPEntityOfEntIndex((*entry).entityIndex as
                                       libc::c_int)).v.modelindex;
        // game override
        if !(SV_RestoreCustomDecal(entry,
                                   pfnPEntityOfEntIndex((*entry).entityIndex
                                                            as libc::c_int),
                                   false_0) as u64 != 0) {
            decalIndex = pfnDecalIndex((*entry).name.as_mut_ptr());
            // studiodecals will be restored at game-side
            if (*entry).flags as libc::c_int & 0x40 as libc::c_int == 0 {
                SV_CreateDecal(msg, (*entry).position.as_mut_ptr(),
                               decalIndex,
                               (*entry).entityIndex as libc::c_int,
                               modelIndex, (*entry).flags as libc::c_int,
                               (*entry).scale);
            }
        }
        i += 1
    }
    if !host.decalList.is_null() {
        _Mem_Free(host.decalList as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 656 as libc::c_int);
    }
    host.decalList = 0 as *mut decallist_s;
    host.numdecals = 0 as libc::c_int;
}
/*
==============
SV_BoxInPVS

check brush boxes in fat pvs
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BoxInPVS(mut org: *const vec_t,
                                     mut absmin: *const vec_t,
                                     mut absmax: *const vec_t) -> qboolean {
    if Mod_BoxVisible(absmin, absmax, Mod_GetPVSForPoint(org)) as u64 == 0 {
        return false_0
    }
    return true_0;
}
/*
=============
SV_ChangeLevel

Issue changing level
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_QueueChangeLevel(mut level: *const libc::c_char,
                                             mut landname:
                                                 *const libc::c_char) {
    let mut flags: uint = 0;
    let mut smooth: uint = false_0 as libc::c_int as uint;
    let mut mapname: [libc::c_char; 64] = [0; 64];
    let mut spawn_entity: *mut libc::c_char = 0 as *mut libc::c_char;
    // hold mapname to other place
    Q_strncpy(mapname.as_mut_ptr(), level,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_StripExtension(mapname.as_mut_ptr());
    if if landname.is_null() || *landname == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        smooth = true_0 as libc::c_int as uint
    }
    // determine spawn entity classname
    if svs.maxclients == 1 as libc::c_int {
        spawn_entity = (*SI.GameInfo).sp_entity.as_mut_ptr()
    } else { spawn_entity = (*SI.GameInfo).mp_entity.as_mut_ptr() }
    flags = SV_MapIsValid(mapname.as_mut_ptr(), spawn_entity, landname);
    if flags & (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        Con_Printf(b"^1Error:^7 changelevel: %s is invalid or not supported\n\x00"
                       as *const u8 as *const libc::c_char,
                   mapname.as_mut_ptr());
        return
    }
    if flags & (1 as libc::c_uint) << 0 as libc::c_int == 0 {
        Con_Printf(b"^1Error:^7 changelevel: map %s doesn\'t exist\n\x00" as
                       *const u8 as *const libc::c_char,
                   mapname.as_mut_ptr());
        return
    }
    if smooth != 0 && flags & (1 as libc::c_uint) << 2 as libc::c_int == 0 {
        if (*sv_validate_changelevel).value != 0. {
            // NOTE: we find valid map but specified landmark it's doesn't exist
			// run simple changelevel like in q1, throw warning
            Con_Printf(b"^3Warning:^7 changelevel: %s doesn\'t contain landmark [%s]. smooth transition was disabled\n\x00"
                           as *const u8 as *const libc::c_char,
                       mapname.as_mut_ptr(),
                       landname); // multiplayer doesn't support smooth transition
            smooth = false_0 as libc::c_int as uint
        }
    }
    if svs.maxclients > 1 as libc::c_int {
        smooth = false_0 as libc::c_int as uint
    }
    if smooth != 0 &&
           Q_strnicmp(sv.name.as_mut_ptr(), level, 99999 as libc::c_int) == 0
       {
        Con_Printf(b"^1Error:^7 can\'t changelevel with same map. Ignored.\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    if smooth == 0 && flags & (1 as libc::c_uint) << 1 as libc::c_int == 0 {
        if (*sv_validate_changelevel).value != 0. {
            Con_Printf(b"^1Error:^7 changelevel: %s doesn\'t have a valid spawnpoint. Ignored.\n\x00"
                           as *const u8 as *const libc::c_char,
                       mapname.as_mut_ptr());
            return
        }
    }
    // bad changelevel position invoke enables in one-way transition
    if sv.framecount < 15 as libc::c_int {
        if (*sv_validate_changelevel).value != 0. {
            Con_Printf(b"^3Warning:^7 an infinite changelevel was detected and will be disabled until a next save\\restore\n\x00"
                           as *const u8 as *const libc::c_char);
            return
            // lock with svs.spawncount here
        }
    }
    SV_SkipUpdates();
    // changelevel will be executed on a next frame
    if smooth != 0 {
        COM_ChangeLevel(mapname.as_mut_ptr(), landname,
                        sv.background); // Smoothed Half-Life changelevel
    } else {
        COM_ChangeLevel(mapname.as_mut_ptr(), 0 as *const libc::c_char,
                        sv.background);
    };
    // Classic Quake changlevel
}
/*
==============
SV_WriteEntityPatch

Create entity patch for selected map
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_WriteEntityPatch(mut filename:
                                                 *const libc::c_char) {
    let mut lumpofs: libc::c_int = 0 as libc::c_int; // 1 kb
    let mut lumplen: libc::c_int = 0 as libc::c_int;
    let mut buf: [byte; 2048] = [0; 2048];
    let mut bspfilename: string = [0; 256];
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    let mut f: *mut file_t = 0 as *mut file_t;
    Q_snprintf(bspfilename.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
               filename);
    f =
        FS_Open(bspfilename.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() { return }
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           2048 as libc::c_int as libc::c_ulong);
    FS_Read(f, buf.as_mut_ptr() as *mut libc::c_void,
            2048 as libc::c_int as size_t);
    header = buf.as_mut_ptr() as *mut dheader_t;
    // check all the lumps and some other errors
    if Mod_TestBmodelLumps(bspfilename.as_mut_ptr(), buf.as_mut_ptr(), true_0)
           as u64 == 0 {
        FS_Close(f);
        return
    }
    lumpofs = (*header).lumps[0 as libc::c_int as usize].fileofs;
    lumplen = (*header).lumps[0 as libc::c_int as usize].filelen;
    if lumplen >= 10 as libc::c_int {
        let mut entities: *mut libc::c_char = 0 as *mut libc::c_char;
        FS_Seek(f, lumpofs as fs_offset_t, 0 as libc::c_int);
        entities =
            _Mem_Alloc(host.mempool, (lumplen + 1 as libc::c_int) as size_t,
                       true_0,
                       b"../engine/server/sv_game.c\x00" as *const u8 as
                           *const libc::c_char, 799 as libc::c_int) as
                *mut libc::c_char;
        FS_Read(f, entities as *mut libc::c_void, lumplen as size_t);
        FS_WriteFile(va(b"maps/%s.ent\x00" as *const u8 as
                            *const libc::c_char, filename),
                     entities as *const libc::c_void, lumplen as fs_offset_t);
        Con_Printf(b"Write \'maps/%s.ent\'\n\x00" as *const u8 as
                       *const libc::c_char, filename);
        _Mem_Free(entities as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 803 as libc::c_int);
    }
    FS_Close(f);
}
/*
==============
SV_ReadEntityScript

pfnMapIsValid use this
==============
*/
unsafe extern "C" fn SV_ReadEntityScript(mut filename: *const libc::c_char,
                                         mut flags: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut bspfilename: string = [0; 256];
    let mut entfilename: string = [0; 256];
    let mut lumpofs: libc::c_int = 0 as libc::c_int;
    let mut lumplen: libc::c_int = 0 as libc::c_int;
    let mut buf: [byte; 2048] = [0; 2048];
    let mut ents: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    let mut ft1: size_t = 0;
    let mut ft2: size_t = 0;
    let mut f: *mut file_t = 0 as *mut file_t;
    *flags = 0 as libc::c_int;
    Q_snprintf(bspfilename.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
               filename);
    f =
        FS_Open(bspfilename.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() { return 0 as *mut libc::c_char }
    *flags =
        (*flags as libc::c_uint | (1 as libc::c_uint) << 0 as libc::c_int) as
            libc::c_int;
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           2048 as libc::c_int as libc::c_ulong);
    FS_Read(f, buf.as_mut_ptr() as *mut libc::c_void,
            2048 as libc::c_int as size_t);
    header = buf.as_mut_ptr() as *mut dheader_t;
    // check all the lumps and some other errors
    if Mod_TestBmodelLumps(bspfilename.as_mut_ptr(), buf.as_mut_ptr(),
                           if host_developer.value != 0. {
                               false_0 as libc::c_int
                           } else { true_0 as libc::c_int } as qboolean) as
           u64 == 0 {
        *flags =
            (*flags as libc::c_uint | (1 as libc::c_uint) << 3 as libc::c_int)
                as libc::c_int;
        FS_Close(f);
        return 0 as *mut libc::c_char
    }
    // after call Mod_TestBmodelLumps we gurantee what map is valid
    lumpofs = (*header).lumps[0 as libc::c_int as usize].fileofs;
    lumplen = (*header).lumps[0 as libc::c_int as usize].filelen;
    // check for entfile too
    Q_snprintf(entfilename.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"maps/%s.ent\x00" as *const u8 as *const libc::c_char,
               filename);
    // make sure what entity patch is newer than bsp
    ft1 = FS_FileTime(bspfilename.as_mut_ptr(), false_0) as size_t;
    ft2 = FS_FileTime(entfilename.as_mut_ptr(), true_0) as size_t;
    if ft2 != -(1 as libc::c_int) as libc::c_ulong && ft1 < ft2 {
        // grab .ent files only from gamedir
        ents =
            FS_LoadFile(entfilename.as_mut_ptr(), 0 as *mut fs_offset_t,
                        true_0) as *mut libc::c_char
    }
    // at least entities should contain "{ "classname" "worldspawn" }\0"
	// for correct spawn the level
    if ents.is_null() && lumplen >= 32 as libc::c_int {
        FS_Seek(f, lumpofs as fs_offset_t, 0 as libc::c_int); // all done
        ents =
            _Mem_Alloc(host.mempool, (lumplen + 1 as libc::c_int) as size_t,
                       true_0,
                       b"../engine/server/sv_game.c\x00" as *const u8 as
                           *const libc::c_char, 868 as libc::c_int) as
                *mut libc::c_char;
        FS_Read(f, ents as *mut libc::c_void, lumplen as size_t);
    }
    FS_Close(f);
    return ents;
}
/*
==============
SV_MapIsValid

Validate map
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_MapIsValid(mut filename: *const libc::c_char,
                                       mut spawn_entity: *const libc::c_char,
                                       mut landmark_name: *const libc::c_char)
 -> uint {
    let mut flags: uint = 0 as libc::c_int as uint;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ents: *mut libc::c_char = 0 as *mut libc::c_char;
    ents =
        SV_ReadEntityScript(filename,
                            &mut flags as *mut uint as *mut libc::c_int);
    if !ents.is_null() {
        let mut need_landmark: qboolean = false_0;
        let mut token: [libc::c_char; 2048] = [0; 2048];
        let mut check_name: string = [0; 256];
        need_landmark =
            if landmark_name.is_null() || *landmark_name == 0 {
                0 as libc::c_int
            } else { 1 as libc::c_int } as qboolean;
        // g-cont. in-dev mode we can entering on map even without "info_player_start"
        if need_landmark as u64 == 0 && host_developer.value != 0. {
            // not transition
            _Mem_Free(ents as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 903 as libc::c_int);
            // skip spawnpoint checks in devmode
            return flags | (1 as libc::c_uint) << 1 as libc::c_int
        }
        pfile = ents;
        loop  {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if pfile.is_null() { break ; }
            if Q_strncmp(token.as_mut_ptr(),
                         b"classname\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
                // check classname for spawn entity
                pfile =
                    _COM_ParseFileSafe(pfile, check_name.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                if !(Q_strncmp(spawn_entity, check_name.as_mut_ptr(),
                               99999 as libc::c_int) == 0) {
                    continue ;
                }
                flags = flags | (1 as libc::c_uint) << 1 as libc::c_int;
                // we already find landmark, stop the parsing
                if need_landmark as libc::c_uint != 0 &&
                       flags & (1 as libc::c_uint) << 2 as libc::c_int != 0 {
                    break ;
                }
            } else {
                if !(need_landmark as libc::c_uint != 0 &&
                         Q_strncmp(token.as_mut_ptr(),
                                   b"targetname\x00" as *const u8 as
                                       *const libc::c_char,
                                   99999 as libc::c_int) == 0) {
                    continue ;
                }
                // check targetname for landmark entity
                pfile =
                    _COM_ParseFileSafe(pfile, check_name.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                if !(Q_strncmp(landmark_name, check_name.as_mut_ptr(),
                               99999 as libc::c_int) == 0) {
                    continue ;
                }
                flags = flags | (1 as libc::c_uint) << 2 as libc::c_int;
                // we already find spawnpoint, stop the parsing
                if flags & (1 as libc::c_uint) << 1 as libc::c_int != 0 {
                    break ;
                }
            }
        }
        _Mem_Free(ents as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 942 as libc::c_int);
    }
    return flags;
}
/*
==============
SV_FreePrivateData

release private edict memory
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FreePrivateData(mut pEdict: *mut edict_t) {
    if pEdict.is_null() || (*pEdict).pvPrivateData.is_null() { return }
    // NOTE: new interface can be missing
    if svgame.dllFuncs2.pfnOnFreeEntPrivateData.is_some() {
        svgame.dllFuncs2.pfnOnFreeEntPrivateData.expect("non-null function pointer")(pEdict);
    }
    if Mem_IsAllocatedExt(svgame.mempool, (*pEdict).pvPrivateData) as u64 != 0
       {
        _Mem_Free((*pEdict).pvPrivateData,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 965 as libc::c_int);
    }
    (*pEdict).pvPrivateData = 0 as *mut libc::c_void;
}
/*
==============
SV_InitEdict

clear edict for reuse
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitEdict(mut pEdict: *mut edict_t) {
    SV_FreePrivateData(pEdict);
    memset(&mut (*pEdict).v as *mut entvars_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entvars_t>() as libc::c_ulong);
    (*pEdict).v.pContainingEntity = pEdict;
    (*pEdict).v.controller[0 as libc::c_int as usize] =
        0x7f as libc::c_int as byte;
    (*pEdict).v.controller[1 as libc::c_int as usize] =
        0x7f as libc::c_int as byte;
    (*pEdict).v.controller[2 as libc::c_int as usize] =
        0x7f as libc::c_int as byte;
    (*pEdict).v.controller[3 as libc::c_int as usize] =
        0x7f as libc::c_int as byte;
    (*pEdict).free = false_0;
}
/*
==============
SV_FreeEdict

unlink edict from world and free it
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FreeEdict(mut pEdict: *mut edict_t) {
    if (*pEdict).free as u64 != 0 { return }
    // unlink from world
    SV_UnlinkEdict(pEdict);
    SV_FreePrivateData(pEdict);
    // mark edict as freed
    (*pEdict).freetime = sv.time as libc::c_float; // invalidate EHANDLE's
    (*pEdict).serialnumber += 1;
    (*pEdict).v.solid = 0 as libc::c_int;
    (*pEdict).v.flags = 0 as libc::c_int;
    (*pEdict).v.model = 0 as libc::c_int;
    (*pEdict).v.takedamage = 0 as libc::c_int as libc::c_float;
    (*pEdict).v.modelindex = 0 as libc::c_int;
    (*pEdict).v.nextthink = -(1 as libc::c_int) as libc::c_float;
    (*pEdict).v.colormap = 0 as libc::c_int;
    (*pEdict).v.frame = 0 as libc::c_int as libc::c_float;
    (*pEdict).v.scale = 0 as libc::c_int as libc::c_float;
    (*pEdict).v.gravity = 0 as libc::c_int as libc::c_float;
    (*pEdict).v.skin = 0 as libc::c_int;
    (*pEdict).v.angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pEdict).v.angles[1 as libc::c_int as usize] =
        (*pEdict).v.angles[2 as libc::c_int as usize];
    (*pEdict).v.angles[0 as libc::c_int as usize] =
        (*pEdict).v.angles[1 as libc::c_int as usize];
    (*pEdict).v.origin[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pEdict).v.origin[1 as libc::c_int as usize] =
        (*pEdict).v.origin[2 as libc::c_int as usize];
    (*pEdict).v.origin[0 as libc::c_int as usize] =
        (*pEdict).v.origin[1 as libc::c_int as usize];
    (*pEdict).free = true_0;
}
/*
==============
SV_AllocEdict

allocate new or reuse existing
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AllocEdict() -> *mut edict_t {
    let mut e: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    i = svs.maxclients + 1 as libc::c_int;
    while i < svgame.numEntities {
        e = SV_EdictNum(i);
        // the first couple seconds of server time can involve a lot of
		// freeing and allocating, so relax the replacement policy
        if (*e).free as libc::c_uint != 0 &&
               ((*e).freetime < 2.0f32 ||
                    sv.time - (*e).freetime as libc::c_double >
                        0.5f32 as libc::c_double) {
            SV_InitEdict(e);
            return e
        }
        i += 1
    }
    if i >= (*SI.GameInfo).max_edicts {
        Host_Error(b"ED_AllocEdict: no free edicts (max is %d)\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*SI.GameInfo).max_edicts);
    }
    svgame.numEntities += 1;
    e = SV_EdictNum(i);
    SV_InitEdict(e);
    return e;
}
/*
==============
SV_GetEntityClass

get pointer for entity class
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetEntityClass(mut pszClassName:
                                               *const libc::c_char)
 -> LINK_ENTITY_FUNC {
    // allocate edict private memory (passed by dlls)
    return ::std::mem::transmute::<*mut libc::c_void,
                                   LINK_ENTITY_FUNC>(COM_GetProcAddress(svgame.hInstance,
                                                                        pszClassName));
}
/*
==============
SV_AllocPrivateData

allocate private data for a given edict
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AllocPrivateData(mut ent: *mut edict_t,
                                             mut className: string_t)
 -> *mut edict_t {
    let mut pszClassName: *const libc::c_char = 0 as *const libc::c_char;
    let mut SpawnEdict: LINK_ENTITY_FUNC = None;
    pszClassName = SV_GetString(className);
    if ent.is_null() {
        // allocate a new one
        ent = SV_AllocEdict()
    } else if (*ent).free as u64 != 0 {
        SV_InitEdict(ent);
        // re-init edict
    } // re-link
    (*ent).v.classname = className;
    (*ent).v.pContainingEntity = ent;
    // allocate edict private memory (passed by dlls)
    SpawnEdict = SV_GetEntityClass(pszClassName);
    if SpawnEdict.is_none() {
        // attempt to create custom entity (Xash3D extension)
        if svgame.physFuncs.SV_CreateEntity.is_some() &&
               svgame.physFuncs.SV_CreateEntity.expect("non-null function pointer")(ent,
                                                                                    pszClassName)
                   != -(1 as libc::c_int) {
            return ent
        }
        SpawnEdict =
            SV_GetEntityClass(b"custom\x00" as *const u8 as
                                  *const libc::c_char);
        if SpawnEdict.is_none() {
            Con_Printf(b"^1Error:^7 No spawn function for %s\n\x00" as
                           *const u8 as *const libc::c_char,
                       SV_GetString(className));
            // it's a custom entity but not a beam!
            // free entity immediately
            SV_FreeEdict(ent);
            return 0 as *mut edict_t
        }
        (*ent).v.flags =
            ((*ent).v.flags as libc::c_uint |
                 (1 as libc::c_uint) << 29 as libc::c_int) as libc::c_int
    }
    SpawnEdict.expect("non-null function pointer")(&mut (*ent).v);
    return ent;
}
/*
==============
SV_CreateNamedEntity

create specified entity, alloc private data
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateNamedEntity(mut ent: *mut edict_t,
                                              mut className: string_t)
 -> *mut edict_t {
    let mut ed: *mut edict_t = SV_AllocPrivateData(ent, className);
    // for some reasons this flag should be immediately cleared
    if !ed.is_null() {
        (*ed).v.flags =
            ((*ed).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 29 as libc::c_int)) as libc::c_int
    }
    return ed;
}
/*
==============
SV_FreeEdicts

release all the edicts from server
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FreeEdicts() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    i = 0 as libc::c_int;
    while i < svgame.numEntities {
        ent = SV_EdictNum(i);
        if !((*ent).free as u64 != 0) { SV_FreeEdict(ent); }
        i += 1
    };
}
/*
==============
SV_PlaybackReliableEvent

reliable event is must be delivered always
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PlaybackReliableEvent(mut msg: *mut sizebuf_t,
                                                  mut eventindex: word,
                                                  mut delay: libc::c_float,
                                                  mut args:
                                                      *mut event_args_t) {
    let mut nullargs: event_args_t =
        event_args_t{flags: 0,
                     entindex: 0,
                     origin: [0.; 3],
                     angles: [0.; 3],
                     velocity: [0.; 3],
                     ducking: 0,
                     fparam1: 0.,
                     fparam2: 0.,
                     iparam1: 0,
                     iparam2: 0,
                     bparam1: 0,
                     bparam2: 0,};
    memset(&mut nullargs as *mut event_args_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<event_args_t>() as libc::c_ulong);
    MSG_WriteCmdExt(msg, 21 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    // send event index
    MSG_WriteUBitLong(msg, eventindex as uint, 10 as libc::c_int);
    if delay != 0. {
        // send event delay
        MSG_WriteOneBit(msg, 1 as libc::c_int);
        MSG_WriteWord(msg, (delay * 100.0f32) as libc::c_int);
    } else { MSG_WriteOneBit(msg, 0 as libc::c_int); }
    // reliable events not use delta-compression just null-compression
    MSG_WriteDeltaEvent(msg, &mut nullargs, args);
}
/*
==============
SV_ClassName

template to get edict classname
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClassName(mut e: *const edict_t)
 -> *const libc::c_char {
    if e.is_null() {
        return b"(null)\x00" as *const u8 as *const libc::c_char
    }
    if (*e).free as u64 != 0 {
        return b"freed\x00" as *const u8 as *const libc::c_char
    }
    return SV_GetString((*e).v.classname);
}
/*
==============
SV_IsValidCmd

command validation
==============
*/
unsafe extern "C" fn SV_IsValidCmd(mut pCmd: *const libc::c_char)
 -> qboolean {
    let mut len: size_t = Q_strlen(pCmd);
    // valid commands all have a ';' or newline '\n' as their last character
    if len != 0 &&
           (*pCmd.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize) as libc::c_int == '\n' as i32 ||
                *pCmd.offset(len.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) as isize) as
                    libc::c_int == ';' as i32) {
        return true_0
    }
    return false_0;
}
/*
==============
SV_AllocPrivateData

get edict that attached to the client structure
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClientFromEdict(mut pEdict: *const edict_t,
                                            mut spawned_only: qboolean)
 -> *mut sv_client_t {
    let mut i: libc::c_int = 0;
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1240 as libc::c_int) as u64 == 0
       {
        return 0 as *mut sv_client_t
    }
    i =
        (pEdict as *mut edict_t).wrapping_offset_from(svgame.edicts) as
            libc::c_long as libc::c_int - 1 as libc::c_int;
    if i < 0 as libc::c_int || i >= svs.maxclients {
        return 0 as *mut sv_client_t
    }
    if spawned_only as u64 != 0 {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint !=
               cs_spawned as libc::c_int as libc::c_uint {
            return 0 as *mut sv_client_t
        }
    }
    return svs.clients.offset(i as isize);
}
/*
===============================================================================

	Game Builtin Functions

===============================================================================
*/
/*
=========
pfnPrecacheModel

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPrecacheModel(mut s: *const libc::c_char)
 -> libc::c_int {
    let mut optional: qboolean = false_0;
    let mut i: libc::c_int = 0;
    if *s as libc::c_int == '!' as i32 { optional = true_0; s = s.offset(1) }
    i = SV_ModelIndex(s);
    if i == 0 as libc::c_int { return 0 as libc::c_int }
    sv.models[i as usize] =
        Mod_ForName(sv.model_precache[i as usize].as_mut_ptr(), false_0,
                    true_0);
    if optional as u64 == 0 {
        sv.model_precache_flags[i as usize] =
            (sv.model_precache_flags[i as usize] as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as byte
    }
    return i;
}
/*
=================
pfnSetModel

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetModel(mut e: *mut edict_t,
                                     mut m: *const libc::c_char) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut found: qboolean = false_0;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 1 as libc::c_int;
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1305 as libc::c_int) as u64 == 0
       {
        return
    }
    if *m as libc::c_int == '\\' as i32 || *m as libc::c_int == '/' as i32 {
        m = m.offset(1)
    }
    Q_strncpy(name.as_mut_ptr(), m,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    if if name.as_mut_ptr().is_null() || *name.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        // check to see if model was properly precached
        while i < 1024 as libc::c_int &&
                  sv.model_precache[i as usize][0 as libc::c_int as usize] as
                      libc::c_int != 0 {
            if Q_strnicmp(sv.model_precache[i as usize].as_mut_ptr(),
                          name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
                found = true_0;
                break ;
            } else { i += 1 }
        }
        if found as u64 == 0 {
            Con_Printf(b"^1Error:^7 Failed to set model %s: was not precached\n\x00"
                           as *const u8 as *const libc::c_char,
                       name.as_mut_ptr());
            return
        }
    }
    if e == svgame.edicts {
        if sv.state as libc::c_uint ==
               ss_active as libc::c_int as libc::c_uint {
            Con_Printf(b"^1Error:^7 Failed to set model %s: world model cannot be changed\n\x00"
                           as *const u8 as *const libc::c_char,
                       name.as_mut_ptr());
        }
        return
    }
    if if name.as_mut_ptr().is_null() || *name.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        (*e).v.model =
            SV_MakeString(sv.model_precache[i as usize].as_mut_ptr());
        (*e).v.modelindex = i;
        mod_0 = sv.models[i as usize]
    } else {
        // model will be cleared
        (*e).v.modelindex = 0 as libc::c_int;
        (*e).v.model = (*e).v.modelindex;
        mod_0 = 0 as *mut model_t
    }
    // set the model size
    if !mod_0.is_null() &&
           (*mod_0).type_0 as libc::c_int != mod_studio as libc::c_int {
        SV_SetMinMaxSize(e, (*mod_0).mins.as_mut_ptr(),
                         (*mod_0).maxs.as_mut_ptr(), true_0);
    } else {
        SV_SetMinMaxSize(e, vec3_origin.as_mut_ptr(),
                         vec3_origin.as_mut_ptr(), true_0);
    };
}
/*
=================
pfnModelIndex

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnModelIndex(mut m: *const libc::c_char)
 -> libc::c_int {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    if if m.is_null() || *m == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    if *m as libc::c_int == '\\' as i32 || *m as libc::c_int == '/' as i32 {
        m = m.offset(1)
    }
    Q_strncpy(name.as_mut_ptr(), m,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < 1024 as libc::c_int &&
              sv.model_precache[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(sv.model_precache[i as usize].as_mut_ptr(),
                      name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    Con_Printf(b"^1Error:^7 Cannot get index for model %s: not precached\n\x00"
                   as *const u8 as *const libc::c_char, name.as_mut_ptr());
    return 0 as libc::c_int;
}
/*
=================
pfnModelFrames

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnModelFrames(mut modelIndex: libc::c_int)
 -> libc::c_int {
    let mut pmodel: *mut model_t = SV_ModelHandle(modelIndex);
    if !pmodel.is_null() { return (*pmodel).numframes }
    return 1 as libc::c_int;
}
/*
=================
pfnSetSize

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetSize(mut e: *mut edict_t,
                                    mut rgflMin: *const libc::c_float,
                                    mut rgflMax: *const libc::c_float) {
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1408 as libc::c_int) as u64 == 0
       {
        return
    }
    SV_SetMinMaxSize(e, rgflMin, rgflMax, true_0);
}
/*
=================
pfnChangeLevel

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnChangeLevel(mut level: *const libc::c_char,
                                        mut landmark: *const libc::c_char) {
    static mut last_spawncount: uint = 0 as libc::c_int as uint; // ???
    let mut landname: [libc::c_char; 64] = [0; 64];
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    if (if level.is_null() || *level == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 ||
           sv.state as libc::c_uint !=
               ss_active as libc::c_int as libc::c_uint {
        return
    }
    // make sure we don't issue two changelevels
    if svs.spawncount as libc::c_uint == last_spawncount { return }
    last_spawncount = svs.spawncount as uint;
    landname[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    // g-cont. some level-designers wrote landmark name with space
	// and Cmd_TokenizeString separating all the after space as next argument
	// emulate this bug for compatibility
    if if landmark.is_null() || *landmark == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        text = landname.as_mut_ptr();
        while *landmark as libc::c_int != 0 &&
                  *landmark as byte as libc::c_int != ' ' as i32 {
            let fresh0 = landmark;
            landmark = landmark.offset(1);
            let fresh1 = text;
            text = text.offset(1);
            *fresh1 = *fresh0
        }
        *text = '\u{0}' as i32 as libc::c_char
    }
    SV_QueueChangeLevel(level, landname.as_mut_ptr());
}
/*
=================
pfnGetSpawnParms

OBSOLETE, UNUSED
=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetSpawnParms(mut ent: *mut edict_t) { }
/*
=================
pfnSaveSpawnParms

OBSOLETE, UNUSED
=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSaveSpawnParms(mut ent: *mut edict_t) { }
/*
=================
pfnVecToYaw

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnVecToYaw(mut rgflVector: *const libc::c_float)
 -> libc::c_float {
    return SV_VecToYaw(rgflVector);
}
/*
=================
pfnMoveToOrigin

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnMoveToOrigin(mut ent: *mut edict_t,
                                         mut pflGoal: *const libc::c_float,
                                         mut dist: libc::c_float,
                                         mut iMoveType: libc::c_int) {
    if pflGoal.is_null() ||
           SV_CheckEdict(ent,
                         b"../engine/server/sv_game.c\x00" as *const u8 as
                             *const libc::c_char, 1493 as libc::c_int) as u64
               == 0 {
        return
    }
    SV_MoveToOrigin(ent, pflGoal, dist, iMoveType);
}
/*
==============
pfnChangeYaw

==============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnChangeYaw(mut ent: *mut edict_t) {
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1507 as libc::c_int) as u64 == 0
       {
        return
    }
    (*ent).v.angles[1 as libc::c_int as usize] =
        SV_AngleMod((*ent).v.ideal_yaw,
                    (*ent).v.angles[1 as libc::c_int as usize],
                    (*ent).v.yaw_speed);
}
/*
==============
pfnChangePitch

==============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnChangePitch(mut ent: *mut edict_t) {
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1521 as libc::c_int) as u64 == 0
       {
        return
    }
    (*ent).v.angles[0 as libc::c_int as usize] =
        SV_AngleMod((*ent).v.idealpitch,
                    (*ent).v.angles[0 as libc::c_int as usize],
                    (*ent).v.pitch_speed);
}
/*
=========
SV_FindEntityByString

=========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FindEntityByString(mut pStartEdict: *mut edict_t,
                                               mut pszField:
                                                   *const libc::c_char,
                                               mut pszValue:
                                                   *const libc::c_char)
 -> *mut edict_t {
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut desc: *mut TYPEDESCRIPTION = 0 as *mut TYPEDESCRIPTION;
    let mut ed: *mut edict_t = 0 as *mut edict_t;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    if if pszValue.is_null() || *pszValue == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return svgame.edicts
    }
    if !pStartEdict.is_null() {
        e =
            pStartEdict.wrapping_offset_from(svgame.edicts) as libc::c_long as
                libc::c_int
    }
    loop  {
        let fresh2 = index;
        index = index + 1;
        desc = SV_GetEntvarsDescirption(fresh2);
        if desc.is_null() { break ; }
        if Q_strncmp(pszField, (*desc).fieldName, 99999 as libc::c_int) == 0 {
            break ;
        }
    }
    if desc.is_null() {
        Con_Printf(b"^1Error:^7 FindEntityByString: field %s not a string\n\x00"
                       as *const u8 as *const libc::c_char, pszField);
        return svgame.edicts
    }
    e += 1;
    while e < svgame.numEntities {
        ed = SV_EdictNum(e);
        if !(SV_CheckEdict(ed,
                           b"../engine/server/sv_game.c\x00" as *const u8 as
                               *const libc::c_char, 1560 as libc::c_int) as
                 u64 == 0) {
            if !(e <= svs.maxclients &&
                     SV_ClientFromEdict(ed,
                                        (svs.maxclients != 1 as libc::c_int)
                                            as libc::c_int as
                                            qboolean).is_null()) {
                match (*desc).fieldType as libc::c_uint {
                    1 | 16 | 17 => {
                        t =
                            SV_GetString(*(&mut *(&mut (*ed).v as
                                                      *mut entvars_t as
                                                      *mut byte).offset((*desc).fieldOffset
                                                                            as
                                                                            isize)
                                               as *mut byte as
                                               *mut string_t));
                        if !t.is_null() && t != (*svgame.globals).pStringBase
                           {
                            if Q_strncmp(t, pszValue, 99999 as libc::c_int) ==
                                   0 {
                                return ed
                            }
                        }
                    }
                    _ => {
                        if 0 as libc::c_int == 0 {
                            Sys_Error(b"assert failed at %s:%i\n\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"../engine/server/sv_game.c\x00" as
                                          *const u8 as *const libc::c_char,
                                      1578 as libc::c_int);
                        }
                    }
                }
            }
        }
        e += 1
    }
    return svgame.edicts;
}
/*
=========
SV_FindGlobalEntity

ripped out from the hl.dll
=========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FindGlobalEntity(mut classname: string_t,
                                             mut globalname: string_t)
 -> *mut edict_t {
    let mut pent: *mut edict_t =
        SV_FindEntityByString(0 as *mut edict_t,
                              b"globalname\x00" as *const u8 as
                                  *const libc::c_char,
                              SV_GetString(globalname));
    if SV_CheckEdict(pent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1597 as libc::c_int) as u64 != 0
       {
        // don't spam about error - game code already tell us
        if Q_strncmp(SV_ClassName(pent), SV_GetString(classname),
                     99999 as libc::c_int) != 0 {
            pent = 0 as *mut edict_t
        }
    }
    return pent;
}
/*
==============
pfnGetEntityIllum

returns averaged lightvalue for entity
==============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetEntityIllum(mut pEnt: *mut edict_t)
 -> libc::c_int {
    if SV_CheckEdict(pEnt,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1616 as libc::c_int) as u64 == 0
       {
        return -(1 as libc::c_int)
    }
    return SV_LightForEntity(pEnt);
}
/*
=================
pfnFindEntityInSphere

find the entity in sphere
=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnFindEntityInSphere(mut pStartEdict: *mut edict_t,
                                               mut org: *const libc::c_float,
                                               mut flRadius: libc::c_float)
 -> *mut edict_t {
    let mut distSquared: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut eorg: libc::c_float = 0.;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    flRadius *= flRadius;
    if SV_CheckEdict(pStartEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1638 as libc::c_int) as u64 != 0
       {
        e =
            pStartEdict.wrapping_offset_from(svgame.edicts) as libc::c_long as
                libc::c_int
    }
    e += 1;
    while e < svgame.numEntities {
        ent = SV_EdictNum(e);
        if !(SV_CheckEdict(ent,
                           b"../engine/server/sv_game.c\x00" as *const u8 as
                               *const libc::c_char, 1645 as libc::c_int) as
                 u64 == 0) {
            // ignore clients that not in a game
            if !(e <= svs.maxclients &&
                     SV_ClientFromEdict(ent, true_0).is_null()) {
                distSquared = 0.0f32;
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int && distSquared <= flRadius {
                    if *org.offset(j as isize) < (*ent).v.absmin[j as usize] {
                        eorg =
                            *org.offset(j as isize) -
                                (*ent).v.absmin[j as usize]
                    } else if *org.offset(j as isize) >
                                  (*ent).v.absmax[j as usize] {
                        eorg =
                            *org.offset(j as isize) -
                                (*ent).v.absmax[j as usize]
                    } else { eorg = 0.0f32 }
                    distSquared += eorg * eorg;
                    j += 1
                }
                if distSquared < flRadius { return ent }
            }
        }
        e += 1
    }
    return svgame.edicts;
}
/*
=================
SV_CheckClientPVS

build the new client PVS
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckClientPVS(mut check: libc::c_int,
                                           mut bMergePVS: qboolean)
 -> libc::c_int {
    let mut pvs: *mut byte = 0 as *mut byte;
    let mut vieworg: vec3_t = [0.; 3];
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    // cycle to the next one
    check =
        if check >= 1 as libc::c_int {
            if check < svs.maxclients { check } else { svs.maxclients }
        } else { 1 as libc::c_int }; // reset cycle
    if check == svs.maxclients {
        i = 1 as libc::c_int
    } else { i = check + 1 as libc::c_int } // didn't find anything else
    loop  {
        if i == svs.maxclients + 1 as libc::c_int { i = 1 as libc::c_int }
        ent = SV_EdictNum(i);
        if i == check { break ; }
        if !((*ent).free as libc::c_uint != 0 ||
                 (*ent).pvPrivateData.is_null() ||
                 (*ent).v.flags as libc::c_uint &
                     (1 as libc::c_uint) << 7 as libc::c_int != 0) {
            break ;
        }
        i += 1
    }
    cl = SV_ClientFromEdict(ent, true_0);
    memset(clientpvs.as_mut_ptr() as *mut libc::c_void, 0xff as libc::c_int,
           world.visbytes);
    // get the PVS for the entity
    vieworg[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            (*ent).v.view_ofs[0 as libc::c_int as usize];
    vieworg[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            (*ent).v.view_ofs[1 as libc::c_int as usize];
    vieworg[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            (*ent).v.view_ofs[2 as libc::c_int as usize];
    pvs = Mod_GetPVSForPoint(vieworg.as_mut_ptr() as *const vec_t);
    if !pvs.is_null() {
        memcpy(clientpvs.as_mut_ptr() as *mut libc::c_void,
               pvs as *const libc::c_void, world.visbytes);
    }
    // transition in progress
    if cl.is_null() { return i }
    // now merge PVS with all the portal cameras
    k = 0 as libc::c_int;
    while k < (*cl).num_viewents && bMergePVS as libc::c_uint != 0 {
        let mut view: *mut edict_t = (*cl).viewentity[k as usize];
        if !(SV_CheckEdict(view,
                           b"../engine/server/sv_game.c\x00" as *const u8 as
                               *const libc::c_char, 1725 as libc::c_int) as
                 u64 == 0) {
            vieworg[0 as libc::c_int as usize] =
                (*view).v.origin[0 as libc::c_int as usize] +
                    (*view).v.view_ofs[0 as libc::c_int as usize];
            vieworg[1 as libc::c_int as usize] =
                (*view).v.origin[1 as libc::c_int as usize] +
                    (*view).v.view_ofs[1 as libc::c_int as usize];
            vieworg[2 as libc::c_int as usize] =
                (*view).v.origin[2 as libc::c_int as usize] +
                    (*view).v.view_ofs[2 as libc::c_int as usize];
            pvs = Mod_GetPVSForPoint(vieworg.as_mut_ptr() as *const vec_t);
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < world.visbytes && !pvs.is_null() {
                clientpvs[j as usize] =
                    (clientpvs[j as usize] as libc::c_int |
                         *pvs.offset(j as isize) as libc::c_int) as byte;
                j += 1
            }
        }
        k += 1
    }
    return i;
}
/*
=================
pfnFindClientInPVS

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnFindClientInPVS(mut pEdict: *mut edict_t)
 -> *mut edict_t {
    let mut pClient: *mut edict_t = 0 as *mut edict_t;
    let mut view: vec3_t = [0.; 3];
    let mut delta: libc::c_float = 0.;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut bMergePVS: qboolean = false_0;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1753 as libc::c_int) as u64 == 0
       {
        return svgame.edicts
    }
    delta = (sv.time - sv.lastchecktime) as libc::c_float;
    // don't merge visibility for portal entity, only for monsters
    bMergePVS =
        if (*pEdict).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 5 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    // find a new check if on a new frame
    if delta < 0.0f32 || delta >= 0.1f32 {
        sv.lastcheck = SV_CheckClientPVS(sv.lastcheck, bMergePVS);
        sv.lastchecktime = sv.time
    }
    // return check if it might be visible
    pClient = SV_EdictNum(sv.lastcheck);
    if SV_ClientFromEdict(pClient, true_0).is_null() { return svgame.edicts }
    mod_0 = SV_ModelHandle((*pEdict).v.modelindex);
    // portals & monitors
	// NOTE: this specific break "radiaton tick" in normal half-life. use only as feature
    if host.features &
           ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 &&
           !mod_0.is_null() &&
           (*mod_0).type_0 as libc::c_int == mod_brush as libc::c_int &&
           (*mod_0).flags as libc::c_uint &
               (1 as libc::c_uint) << 1 as libc::c_int == 0 {
        // handle PVS origin for bmodels
        view[0 as libc::c_int as usize] =
            ((*pEdict).v.mins[0 as libc::c_int as usize] +
                 (*pEdict).v.maxs[0 as libc::c_int as usize]) *
                0.5f32; // client which currently in PVS
        view[1 as libc::c_int as usize] =
            ((*pEdict).v.mins[1 as libc::c_int as usize] +
                 (*pEdict).v.maxs[1 as libc::c_int as usize]) * 0.5f32;
        view[2 as libc::c_int as usize] =
            ((*pEdict).v.mins[2 as libc::c_int as usize] +
                 (*pEdict).v.maxs[2 as libc::c_int as usize]) * 0.5f32;
        view[0 as libc::c_int as usize] =
            view[0 as libc::c_int as usize] +
                (*pEdict).v.origin[0 as libc::c_int as usize];
        view[1 as libc::c_int as usize] =
            view[1 as libc::c_int as usize] +
                (*pEdict).v.origin[1 as libc::c_int as usize];
        view[2 as libc::c_int as usize] =
            view[2 as libc::c_int as usize] +
                (*pEdict).v.origin[2 as libc::c_int as usize]
    } else {
        view[0 as libc::c_int as usize] =
            (*pEdict).v.origin[0 as libc::c_int as usize] +
                (*pEdict).v.view_ofs[0 as libc::c_int as usize];
        view[1 as libc::c_int as usize] =
            (*pEdict).v.origin[1 as libc::c_int as usize] +
                (*pEdict).v.view_ofs[1 as libc::c_int as usize];
        view[2 as libc::c_int as usize] =
            (*pEdict).v.origin[2 as libc::c_int as usize] +
                (*pEdict).v.view_ofs[2 as libc::c_int as usize]
    }
    leaf =
        Mod_PointInLeaf(view.as_mut_ptr() as *const vec_t,
                        (*sv.worldmodel).nodes);
    if if (*leaf).cluster >= 0 as libc::c_int {
           (clientpvs[((*leaf).cluster >> 3 as libc::c_int) as usize] as
                libc::c_int &
                (1 as libc::c_int) << ((*leaf).cluster & 7 as libc::c_int)) as
               byte as libc::c_int
       } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
        return pClient
    }
    return svgame.edicts;
}
/*
=================
pfnEntitiesInPVS

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnEntitiesInPVS(mut pview: *mut edict_t)
 -> *mut edict_t {
    let mut pchain: *mut edict_t = 0 as *mut edict_t;
    let mut ptest: *mut edict_t = 0 as *mut edict_t;
    let mut viewpoint: vec3_t = [0.; 3];
    let mut pent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    if SV_CheckEdict(pview,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1810 as libc::c_int) as u64 == 0
       {
        return 0 as *mut edict_t
    }
    viewpoint[0 as libc::c_int as usize] =
        (*pview).v.origin[0 as libc::c_int as usize] +
            (*pview).v.view_ofs[0 as libc::c_int as usize];
    viewpoint[1 as libc::c_int as usize] =
        (*pview).v.origin[1 as libc::c_int as usize] +
            (*pview).v.view_ofs[1 as libc::c_int as usize];
    viewpoint[2 as libc::c_int as usize] =
        (*pview).v.origin[2 as libc::c_int as usize] +
            (*pview).v.view_ofs[2 as libc::c_int as usize];
    pchain = SV_EdictNum(0 as libc::c_int);
    i = 1 as libc::c_int;
    while i < svgame.numEntities {
        pent = SV_EdictNum(i);
        if !(SV_CheckEdict(pent,
                           b"../engine/server/sv_game.c\x00" as *const u8 as
                               *const libc::c_char, 1820 as libc::c_int) as
                 u64 == 0) {
            if (*pent).v.movetype == 12 as libc::c_int &&
                   SV_CheckEdict((*pent).v.aiment,
                                 b"../engine/server/sv_game.c\x00" as
                                     *const u8 as *const libc::c_char,
                                 1823 as libc::c_int) as libc::c_uint != 0 {
                ptest = (*pent).v.aiment
            } else { ptest = pent }
            if SV_BoxInPVS(viewpoint.as_mut_ptr() as *const vec_t,
                           (*ptest).v.absmin.as_mut_ptr() as *const vec_t,
                           (*ptest).v.absmax.as_mut_ptr() as *const vec_t) as
                   u64 != 0 {
                (*pent).v.chain = pchain;
                pchain = pent
            }
        }
        i += 1
    }
    return pchain;
}
/*
==============
pfnMakeVectors

==============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnMakeVectors(mut rgflVector:
                                            *const libc::c_float) {
    AngleVectors(rgflVector, (*svgame.globals).v_forward.as_mut_ptr(),
                 (*svgame.globals).v_right.as_mut_ptr(),
                 (*svgame.globals).v_up.as_mut_ptr());
}
/*
==============
pfnRemoveEntity

free edict private mem, unlink physics etc
==============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnRemoveEntity(mut e: *mut edict_t) {
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1857 as libc::c_int) as u64 == 0
       {
        return
    }
    // never free client or world entity
    if (e.wrapping_offset_from(svgame.edicts) as libc::c_long as libc::c_int)
           < svs.maxclients + 1 as libc::c_int {
        Con_Printf(b"^1Error:^7 can\'t delete %s\n\x00" as *const u8 as
                       *const libc::c_char,
                   if e == SV_EdictNum(0 as libc::c_int) {
                       b"world\x00" as *const u8 as *const libc::c_char
                   } else {
                       b"client\x00" as *const u8 as *const libc::c_char
                   });
        return
    }
    SV_FreeEdict(e);
}
/*
==============
pfnCreateNamedEntity

==============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCreateNamedEntity(mut className: string_t)
 -> *mut edict_t {
    return SV_CreateNamedEntity(0 as *mut edict_t, className);
}
/*
=============
pfnMakeStatic

move entity to client
=============
*/
unsafe extern "C" fn pfnMakeStatic(mut ent: *mut edict_t) {
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1892 as libc::c_int) as u64 == 0
       {
        return
    }
    // fill the entity state
    state =
        &mut *svs.static_entities.offset(sv.num_static_entities as isize) as
            *mut entity_state_t; // allocate a new one
    svgame.dllFuncs.pfnCreateBaseline.expect("non-null function pointer")(false_0
                                                                              as
                                                                              libc::c_int,
                                                                          ent.wrapping_offset_from(svgame.edicts)
                                                                              as
                                                                              libc::c_long
                                                                              as
                                                                              libc::c_int,
                                                                          state,
                                                                          ent,
                                                                          0 as
                                                                              libc::c_int,
                                                                          vec3_origin.as_mut_ptr(),
                                                                          vec3_origin.as_mut_ptr()); // member modelname
    (*state).messagenum = (*ent).v.model;
    if SV_CreateStaticEntity(&mut sv.signon, sv.num_static_entities) as u64 !=
           0 {
        sv.num_static_entities += 1
    }
    // remove at end of the frame
    (*ent).v.flags =
        ((*ent).v.flags as libc::c_uint |
             (1 as libc::c_uint) << 30 as libc::c_int) as libc::c_int;
}
/*
=============
pfnEntIsOnFloor

legacy builtin
=============
*/
unsafe extern "C" fn pfnEntIsOnFloor(mut e: *mut edict_t) -> libc::c_int {
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1916 as libc::c_int) as u64 == 0
       {
        return 0 as libc::c_int
    }
    return SV_CheckBottom(e, 0 as libc::c_int) as libc::c_int;
}
/*
===============
pfnDropToFloor

===============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnDropToFloor(mut e: *mut edict_t) -> libc::c_int {
    let mut monsterClip: qboolean = false_0;
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    let mut end: vec3_t = [0.; 3];
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1934 as libc::c_int) as u64 == 0
       {
        return 0 as libc::c_int
    }
    monsterClip =
        if (*e).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    end[0 as libc::c_int as usize] = (*e).v.origin[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = (*e).v.origin[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = (*e).v.origin[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] -= 256.0f32;
    trace =
        SV_Move((*e).v.origin.as_mut_ptr() as *const vec_t,
                (*e).v.mins.as_mut_ptr(), (*e).v.maxs.as_mut_ptr(),
                end.as_mut_ptr() as *const vec_t, 0 as libc::c_int, e,
                monsterClip);
    if trace.allsolid as u64 != 0 { return -(1 as libc::c_int) }
    if trace.fraction == 1.0f32 { return 0 as libc::c_int }
    (*e).v.origin[0 as libc::c_int as usize] =
        trace.endpos[0 as libc::c_int as usize];
    (*e).v.origin[1 as libc::c_int as usize] =
        trace.endpos[1 as libc::c_int as usize];
    (*e).v.origin[2 as libc::c_int as usize] =
        trace.endpos[2 as libc::c_int as usize];
    SV_LinkEdict(e, false_0);
    (*e).v.flags =
        ((*e).v.flags as libc::c_uint |
             (1 as libc::c_uint) << 9 as libc::c_int) as libc::c_int;
    (*e).v.groundentity = trace.ent;
    return 1 as libc::c_int;
}
/*
===============
pfnWalkMove

===============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWalkMove(mut ent: *mut edict_t,
                                     mut yaw: libc::c_float,
                                     mut dist: libc::c_float,
                                     mut iMode: libc::c_int) -> libc::c_int {
    let mut move_0: vec3_t = [0.; 3];
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1967 as libc::c_int) as u64 == 0
       {
        return 0 as libc::c_int
    }
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 0 as libc::c_int |
                (1 as libc::c_uint) << 1 as libc::c_int |
                (1 as libc::c_uint) << 9 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    yaw = yaw * (3.14159265358979323846f64 as libc::c_float / 180.0f32);
    move_0[0 as libc::c_int as usize] = __tg_cos(yaw) * dist;
    move_0[1 as libc::c_int as usize] = __tg_sin(yaw) * dist;
    move_0[2 as libc::c_int as usize] = 0.0f32;
    match iMode {
        0 => {
            return SV_MoveStep(ent, move_0.as_mut_ptr(), true_0) as
                       libc::c_int
        }
        1 => {
            return SV_MoveTest(ent, move_0.as_mut_ptr(), true_0) as
                       libc::c_int
        }
        2 => {
            return SV_MoveStep(ent, move_0.as_mut_ptr(), false_0) as
                       libc::c_int
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
=================
pfnSetOrigin

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetOrigin(mut e: *mut edict_t,
                                      mut rgflOrigin: *const libc::c_float) {
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 1996 as libc::c_int) as u64 == 0
       {
        return
    }
    (*e).v.origin[0 as libc::c_int as usize] =
        *rgflOrigin.offset(0 as libc::c_int as isize);
    (*e).v.origin[1 as libc::c_int as usize] =
        *rgflOrigin.offset(1 as libc::c_int as isize);
    (*e).v.origin[2 as libc::c_int as usize] =
        *rgflOrigin.offset(2 as libc::c_int as isize);
    SV_LinkEdict(e, false_0);
}
/*
=================
SV_BuildSoundMsg

=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BuildSoundMsg(mut msg: *mut sizebuf_t,
                                          mut ent: *mut edict_t,
                                          mut chan: libc::c_int,
                                          mut sample: *const libc::c_char,
                                          mut vol: libc::c_int,
                                          mut attn: libc::c_float,
                                          mut flags: libc::c_int,
                                          mut pitch: libc::c_int,
                                          mut pos: *const vec_t)
 -> libc::c_int {
    let mut entityIndex: libc::c_int = 0;
    let mut sound_idx: libc::c_int = 0;
    let mut spawn: qboolean = false_0;
    if vol < 0 as libc::c_int || vol > 255 as libc::c_int {
        Con_Reportf(b"^1Error:^7 SV_StartSound: volume = %i\n\x00" as
                        *const u8 as *const libc::c_char, vol);
        vol =
            if vol >= 0 as libc::c_int {
                if vol < 255 as libc::c_int {
                    vol
                } else { 255 as libc::c_int }
            } else { 0 as libc::c_int }
    }
    if attn < 0.0f32 || attn > 4.0f32 {
        Con_Reportf(b"^1Error:^7 SV_StartSound: attenuation %g must be in range 0-4\n\x00"
                        as *const u8 as *const libc::c_char,
                    attn as libc::c_double);
        attn =
            if attn >= 0.0f32 {
                if attn < 4.0f32 { attn } else { 4.0f32 }
            } else { 0.0f32 }
    }
    if chan < 0 as libc::c_int || chan > 7 as libc::c_int {
        Con_Reportf(b"^1Error:^7 SV_StartSound: channel must be in range 0-7\n\x00"
                        as *const u8 as *const libc::c_char);
        chan =
            if chan >= 0 as libc::c_int {
                if chan < 7 as libc::c_int { chan } else { 7 as libc::c_int }
            } else { 0 as libc::c_int }
    }
    if pitch < 0 as libc::c_int || pitch > 255 as libc::c_int {
        Con_Reportf(b"^1Error:^7 SV_StartSound: pitch = %i\n\x00" as *const u8
                        as *const libc::c_char, pitch);
        pitch =
            if pitch >= 0 as libc::c_int {
                if pitch < 255 as libc::c_int {
                    pitch
                } else { 255 as libc::c_int }
            } else { 0 as libc::c_int }
    }
    if if sample.is_null() || *sample == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Reportf(b"^1Error:^7 SV_StartSound: passed NULL sample\n\x00" as
                        *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if *sample.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
           &&
           Q_isdigit(sample.offset(1 as libc::c_int as isize)) as libc::c_uint
               != 0 {
        sound_idx = Q_atoi(sample.offset(1 as libc::c_int as isize));
        if sound_idx >= (1 as libc::c_int) << 11 as libc::c_int {
            flags =
                flags |
                    ((1 as libc::c_int) << 4 as libc::c_int |
                         (1 as libc::c_int) << 2 as libc::c_int);
            sound_idx -= (1 as libc::c_int) << 11 as libc::c_int
        } else { flags = flags | (1 as libc::c_int) << 4 as libc::c_int }
    } else if *sample.offset(0 as libc::c_int as isize) as libc::c_int ==
                  '#' as i32 &&
                  Q_isdigit(sample.offset(1 as libc::c_int as isize)) as
                      libc::c_uint != 0 {
        flags =
            flags |
                ((1 as libc::c_int) << 4 as libc::c_int |
                     (1 as libc::c_int) << 2 as libc::c_int);
        sound_idx = Q_atoi(sample.offset(1 as libc::c_int as isize))
    } else {
        // TESTTEST
        if *sample as libc::c_int == '*' as i32 { chan = 0 as libc::c_int }
        // precache_sound can be used twice: cache sounds when loading
		// and return sound index when server is active
        sound_idx = SV_SoundIndex(sample)
    } // assume world
    if sound_idx == 0 {
        Con_Printf(b"^1Error:^7 SV_StartSound: %s not precached (%d)\n\x00" as
                       *const u8 as *const libc::c_char, sample, sound_idx);
        return 0 as libc::c_int
    }
    spawn =
        if flags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2079 as libc::c_int) as
           libc::c_uint != 0 &&
           SV_CheckEdict((*ent).v.aiment,
                         b"../engine/server/sv_game.c\x00" as *const u8 as
                             *const libc::c_char, 2079 as libc::c_int) as
               libc::c_uint != 0 {
        entityIndex =
            (*ent).v.aiment.wrapping_offset_from(svgame.edicts) as
                libc::c_long as libc::c_int
    } else if SV_CheckEdict(ent,
                            b"../engine/server/sv_game.c\x00" as *const u8 as
                                *const libc::c_char, 2081 as libc::c_int) as
                  u64 != 0 {
        entityIndex =
            ent.wrapping_offset_from(svgame.edicts) as libc::c_long as
                libc::c_int
    } else { entityIndex = 0 as libc::c_int }
    if vol != 255 as libc::c_int {
        flags = flags | (1 as libc::c_int) << 0 as libc::c_int
    }
    if attn != 0 as libc::c_int as libc::c_float {
        flags = flags | (1 as libc::c_int) << 1 as libc::c_int
    }
    if pitch != 100 as libc::c_int {
        flags = flags | (1 as libc::c_int) << 3 as libc::c_int
    }
    // not sending (because this is out of range)
    flags = flags & !((1 as libc::c_int) << 12 as libc::c_int);
    flags = flags & !((1 as libc::c_int) << 11 as libc::c_int);
    flags = flags & !((1 as libc::c_int) << 8 as libc::c_int);
    if spawn as u64 != 0 {
        MSG_WriteCmdExt(msg, 6 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
    } else {
        MSG_WriteCmdExt(msg, 19 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
    }
    MSG_WriteUBitLong(msg, flags as uint, 14 as libc::c_int);
    MSG_WriteUBitLong(msg, sound_idx as uint, 11 as libc::c_int);
    MSG_WriteUBitLong(msg, chan as uint, 4 as libc::c_int);
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        MSG_WriteByte(msg, vol);
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        MSG_WriteByte(msg,
                      (attn * 64 as libc::c_int as libc::c_float) as
                          libc::c_int);
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        MSG_WriteByte(msg, pitch);
    }
    MSG_WriteUBitLong(msg, entityIndex as uint, 13 as libc::c_int);
    MSG_WriteVec3Coord(msg, pos);
    return 1 as libc::c_int;
}
/*
=================
SV_StartSound

=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_StartSound(mut ent: *mut edict_t,
                                       mut chan: libc::c_int,
                                       mut sample: *const libc::c_char,
                                       mut vol: libc::c_float,
                                       mut attn: libc::c_float,
                                       mut flags: libc::c_int,
                                       mut pitch: libc::c_int) {
    let mut filter: qboolean = false_0;
    let mut msg_dest: libc::c_int = 0;
    let mut origin: vec3_t = [0.; 3];
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2122 as libc::c_int) as u64 == 0
       {
        return
    }
    origin[0 as libc::c_int as usize] =
        ((*ent).v.mins[0 as libc::c_int as usize] +
             (*ent).v.maxs[0 as libc::c_int as usize]) * 0.5f32;
    origin[1 as libc::c_int as usize] =
        ((*ent).v.mins[1 as libc::c_int as usize] +
             (*ent).v.maxs[1 as libc::c_int as usize]) * 0.5f32;
    origin[2 as libc::c_int as usize] =
        ((*ent).v.mins[2 as libc::c_int as usize] +
             (*ent).v.maxs[2 as libc::c_int as usize]) * 0.5f32;
    origin[0 as libc::c_int as usize] =
        origin[0 as libc::c_int as usize] +
            (*ent).v.origin[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] =
        origin[1 as libc::c_int as usize] +
            (*ent).v.origin[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] =
        origin[2 as libc::c_int as usize] +
            (*ent).v.origin[2 as libc::c_int as usize];
    if flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        msg_dest = 3 as libc::c_int
    } else if chan == 6 as libc::c_int {
        msg_dest = 2 as libc::c_int
    } else if host.features &
                  ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
                  0 {
        msg_dest = 2 as libc::c_int
    } else {
        msg_dest =
            if svs.maxclients <= 1 as libc::c_int {
                2 as libc::c_int
            } else { 7 as libc::c_int }
    }
    // always sending stop sound command
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        msg_dest = 2 as libc::c_int
    }
    if flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        filter = true_0
    }
    if SV_BuildSoundMsg(&mut sv.multicast, ent, chan, sample,
                        (vol * 255 as libc::c_int as libc::c_float) as
                            libc::c_int, attn, flags, pitch,
                        origin.as_mut_ptr() as *const vec_t) != 0 {
        SV_Multicast(msg_dest, origin.as_mut_ptr() as *const vec_t,
                     0 as *const edict_t, false_0, filter);
    };
}
/*
=================
pfnEmitAmbientSound

=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnEmitAmbientSound(mut ent: *mut edict_t,
                                             mut pos: *mut libc::c_float,
                                             mut sample: *const libc::c_char,
                                             mut vol: libc::c_float,
                                             mut attn: libc::c_float,
                                             mut flags: libc::c_int,
                                             mut pitch: libc::c_int) {
    let mut msg_dest: libc::c_int = 0;
    if sv.state as libc::c_uint == ss_loading as libc::c_int as libc::c_uint {
        flags = flags | (1 as libc::c_int) << 8 as libc::c_int
    }
    if flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        msg_dest = 3 as libc::c_int
    } else { msg_dest = 2 as libc::c_int }
    // always sending stop sound command
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        msg_dest = 2 as libc::c_int
    }
    if SV_BuildSoundMsg(&mut sv.multicast, ent, 6 as libc::c_int, sample,
                        (vol * 255 as libc::c_int as libc::c_float) as
                            libc::c_int, attn, flags, pitch,
                        pos as *const vec_t) != 0 {
        SV_Multicast(msg_dest, pos as *const vec_t, 0 as *const edict_t,
                     false_0, false_0);
    };
}
/*
=================
SV_StartMusic

=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_StartMusic(mut curtrack: *const libc::c_char,
                                       mut looptrack: *const libc::c_char,
                                       mut position: libc::c_int) {
    MSG_WriteCmdExt(&mut sv.multicast, 9 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut sv.multicast,
                    va(b"music \"%s\" \"%s\" %d\n\x00" as *const u8 as
                           *const libc::c_char, curtrack, looptrack,
                       position));
    SV_Multicast(2 as libc::c_int, 0 as *const vec_t, 0 as *const edict_t,
                 false_0, false_0);
}
/*
=================
pfnTraceLine

=================
*/
unsafe extern "C" fn pfnTraceLine(mut v1: *const libc::c_float,
                                  mut v2: *const libc::c_float,
                                  mut fNoMonsters: libc::c_int,
                                  mut pentToSkip: *mut edict_t,
                                  mut ptr: *mut TraceResult) {
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    trace =
        SV_Move(v1, vec3_origin.as_mut_ptr(), vec3_origin.as_mut_ptr(), v2,
                fNoMonsters, pentToSkip, false_0);
    if SV_CheckEdict(trace.ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2196 as libc::c_int) as u64 == 0
       {
        trace.ent = svgame.edicts
    }
    SV_ConvertTrace(ptr, &mut trace);
}
/*
=================
pfnTraceToss

=================
*/
unsafe extern "C" fn pfnTraceToss(mut pent: *mut edict_t,
                                  mut pentToIgnore: *mut edict_t,
                                  mut ptr: *mut TraceResult) {
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    if SV_CheckEdict(pent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2211 as libc::c_int) as u64 == 0
       {
        return
    }
    trace = SV_MoveToss(pent, pentToIgnore);
    SV_ConvertTrace(ptr, &mut trace);
}
/*
=================
pfnTraceHull

=================
*/
unsafe extern "C" fn pfnTraceHull(mut v1: *const libc::c_float,
                                  mut v2: *const libc::c_float,
                                  mut fNoMonsters: libc::c_int,
                                  mut hullNumber: libc::c_int,
                                  mut pentToSkip: *mut edict_t,
                                  mut ptr: *mut TraceResult) {
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    if hullNumber < 0 as libc::c_int || hullNumber > 3 as libc::c_int {
        hullNumber = 0 as libc::c_int
    }
    trace =
        SV_Move(v1,
                (*sv.worldmodel).hulls[hullNumber as
                                           usize].clip_mins.as_mut_ptr(),
                (*sv.worldmodel).hulls[hullNumber as
                                           usize].clip_maxs.as_mut_ptr(), v2,
                fNoMonsters, pentToSkip, false_0);
    SV_ConvertTrace(ptr, &mut trace);
}
/*
=============
pfnTraceMonsterHull

=============
*/
unsafe extern "C" fn pfnTraceMonsterHull(mut pEdict: *mut edict_t,
                                         mut v1: *const libc::c_float,
                                         mut v2: *const libc::c_float,
                                         mut fNoMonsters: libc::c_int,
                                         mut pentToSkip: *mut edict_t,
                                         mut ptr: *mut TraceResult)
 -> libc::c_int {
    let mut monsterClip: qboolean = false_0;
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2246 as libc::c_int) as u64 == 0
       {
        return 0 as libc::c_int
    }
    monsterClip =
        if (*pEdict).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    trace =
        SV_Move(v1, (*pEdict).v.mins.as_mut_ptr(),
                (*pEdict).v.maxs.as_mut_ptr(), v2, fNoMonsters, pentToSkip,
                monsterClip);
    SV_ConvertTrace(ptr, &mut trace);
    if trace.allsolid as libc::c_uint != 0 || trace.fraction != 1.0f32 {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
/*
=============
pfnTraceModel

=============
*/
unsafe extern "C" fn pfnTraceModel(mut v1: *const libc::c_float,
                                   mut v2: *const libc::c_float,
                                   mut hullNumber: libc::c_int,
                                   mut pent: *mut edict_t,
                                   mut ptr: *mut TraceResult) {
    let mut mins: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut maxs: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    if SV_CheckEdict(pent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2270 as libc::c_int) as u64 == 0
       {
        return
    }
    if hullNumber < 0 as libc::c_int || hullNumber > 3 as libc::c_int {
        hullNumber = 0 as libc::c_int
    }
    mins = (*sv.worldmodel).hulls[hullNumber as usize].clip_mins.as_mut_ptr();
    maxs = (*sv.worldmodel).hulls[hullNumber as usize].clip_maxs.as_mut_ptr();
    model = SV_ModelHandle((*pent).v.modelindex);
    if (*pent).v.solid == 5 as libc::c_int {
        // NOTE: always goes through custom clipping move
		// even if our callbacks is not initialized
        SV_CustomClipMoveToEntity(pent, v1, mins, maxs, v2, &mut trace);
    } else if !model.is_null() &&
                  (*model).type_0 as libc::c_int == mod_brush as libc::c_int {
        let mut oldmovetype: libc::c_int = (*pent).v.movetype;
        let mut oldsolid: libc::c_int = (*pent).v.solid;
        (*pent).v.movetype = 7 as libc::c_int;
        (*pent).v.solid = 4 as libc::c_int;
        SV_ClipMoveToEntity(pent, v1, mins, maxs, v2, &mut trace);
        (*pent).v.movetype = oldmovetype;
        (*pent).v.solid = oldsolid
    } else { SV_ClipMoveToEntity(pent, v1, mins, maxs, v2, &mut trace); }
    SV_ConvertTrace(ptr, &mut trace);
}
/*
=============
pfnTraceTexture

returns texture basename
=============
*/
unsafe extern "C" fn pfnTraceTexture(mut pTextureEntity: *mut edict_t,
                                     mut v1: *const libc::c_float,
                                     mut v2: *const libc::c_float)
 -> *const libc::c_char {
    if SV_CheckEdict(pTextureEntity,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2315 as libc::c_int) as u64 == 0
       {
        return 0 as *const libc::c_char
    }
    return SV_TraceTexture(pTextureEntity, v1, v2);
}
/*
=============
pfnTraceSphere

OBSOLETE, UNUSED
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnTraceSphere(mut v1: *const libc::c_float,
                                        mut v2: *const libc::c_float,
                                        mut fNoMonsters: libc::c_int,
                                        mut radius: libc::c_float,
                                        mut pentToSkip: *mut edict_t,
                                        mut ptr: *mut TraceResult) {
}
/*
=============
pfnGetAimVector

NOTE: speed is unused
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetAimVector(mut ent: *mut edict_t,
                                         mut speed: libc::c_float,
                                         mut rgflReturn: *mut libc::c_float) {
    let mut check: *mut edict_t =
        0 as *mut edict_t; // assume failure if it returns early
    let mut start: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut bestdir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tr: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    *rgflReturn.offset(0 as libc::c_int as isize) =
        (*svgame.globals).v_forward[0 as libc::c_int as usize];
    *rgflReturn.offset(1 as libc::c_int as isize) =
        (*svgame.globals).v_forward[1 as libc::c_int as usize];
    *rgflReturn.offset(2 as libc::c_int as isize) =
        (*svgame.globals).v_forward[2 as libc::c_int as usize];
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 2349 as libc::c_int) as u64 == 0
           ||
           (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 13 as libc::c_int != 0 {
        return
    }
    start[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize];
    start[0 as libc::c_int as usize] =
        start[0 as libc::c_int as usize] +
            (*ent).v.view_ofs[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] =
        start[1 as libc::c_int as usize] +
            (*ent).v.view_ofs[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] =
        start[2 as libc::c_int as usize] +
            (*ent).v.view_ofs[2 as libc::c_int as usize];
    // try sending a trace straight
    dir[0 as libc::c_int as usize] =
        (*svgame.globals).v_forward[0 as libc::c_int as usize];
    dir[1 as libc::c_int as usize] =
        (*svgame.globals).v_forward[1 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] =
        (*svgame.globals).v_forward[2 as libc::c_int as usize];
    end[0 as libc::c_int as usize] =
        start[0 as libc::c_int as usize] +
            2048 as libc::c_int as libc::c_float *
                dir[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] =
        start[1 as libc::c_int as usize] +
            2048 as libc::c_int as libc::c_float *
                dir[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] =
        start[2 as libc::c_int as usize] +
            2048 as libc::c_int as libc::c_float *
                dir[2 as libc::c_int as usize];
    tr =
        SV_Move(start.as_mut_ptr() as *const vec_t, vec3_origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(), end.as_mut_ptr() as *const vec_t,
                0 as libc::c_int, ent, false_0);
    // don't aim at teammate
    if !tr.ent.is_null() &&
           ((*tr.ent).v.takedamage == 2 as libc::c_int as libc::c_float ||
                (*ent).v.team <= 0 as libc::c_int ||
                (*ent).v.team != (*tr.ent).v.team) {
        return
    }
    // try all possible entities
    bestdir[0 as libc::c_int as usize] =
        (*svgame.globals).v_forward[0 as libc::c_int as
                                        usize]; // start at first client
    bestdir[1 as libc::c_int as usize] =
        (*svgame.globals).v_forward[1 as libc::c_int as
                                        usize]; // to far to turn
    bestdir[2 as libc::c_int as usize] =
        (*svgame.globals).v_forward[2 as libc::c_int as usize];
    bestdist =
        Cvar_VariableValue(b"sv_aim\x00" as *const u8 as *const libc::c_char);
    check = SV_EdictNum(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i < svgame.numEntities {
        if !((*check).v.takedamage != 2 as libc::c_int as libc::c_float) {
            if !((*check).v.flags as libc::c_uint &
                     (1 as libc::c_uint) << 13 as libc::c_int != 0) {
                if !((*ent).v.team > 0 as libc::c_int &&
                         (*ent).v.team == (*check).v.team) {
                    if !(check == ent) {
                        j = 0 as libc::c_int;
                        while j < 3 as libc::c_int {
                            end[j as usize] =
                                (*check).v.origin[j as usize] +
                                    0.5f32 *
                                        ((*check).v.mins[j as usize] +
                                             (*check).v.maxs[j as usize]);
                            j += 1
                        }
                        dir[0 as libc::c_int as usize] =
                            end[0 as libc::c_int as usize] -
                                start[0 as libc::c_int as usize];
                        dir[1 as libc::c_int as usize] =
                            end[1 as libc::c_int as usize] -
                                start[1 as libc::c_int as usize];
                        dir[2 as libc::c_int as usize] =
                            end[2 as libc::c_int as usize] -
                                start[2 as libc::c_int as usize];
                        let mut ilength: libc::c_float =
                            __tg_sqrt(dir[0 as libc::c_int as usize] *
                                          dir[0 as libc::c_int as usize] +
                                          dir[1 as libc::c_int as usize] *
                                              dir[1 as libc::c_int as usize] +
                                          dir[2 as libc::c_int as usize] *
                                              dir[2 as libc::c_int as usize]);
                        if ilength != 0. { ilength = 1.0f32 / ilength }
                        dir[0 as libc::c_int as usize] *= ilength;
                        dir[1 as libc::c_int as usize] *= ilength;
                        dir[2 as libc::c_int as usize] *= ilength;
                        dist =
                            dir[0 as libc::c_int as usize] *
                                (*svgame.globals).v_forward[0 as libc::c_int
                                                                as usize] +
                                dir[1 as libc::c_int as usize] *
                                    (*svgame.globals).v_forward[1 as
                                                                    libc::c_int
                                                                    as usize]
                                +
                                dir[2 as libc::c_int as usize] *
                                    (*svgame.globals).v_forward[2 as
                                                                    libc::c_int
                                                                    as usize];
                        if !(dist < bestdist) {
                            tr =
                                SV_Move(start.as_mut_ptr() as *const vec_t,
                                        vec3_origin.as_mut_ptr(),
                                        vec3_origin.as_mut_ptr(),
                                        end.as_mut_ptr() as *const vec_t,
                                        0 as libc::c_int, ent, false_0);
                            if tr.ent == check {
                                // can shoot at this one
                                bestdir[0 as libc::c_int as usize] =
                                    dir[0 as libc::c_int as usize];
                                bestdir[1 as libc::c_int as usize] =
                                    dir[1 as libc::c_int as usize];
                                bestdir[2 as libc::c_int as usize] =
                                    dir[2 as libc::c_int as usize];
                                bestdist = dist
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        check = check.offset(1)
    }
    *rgflReturn.offset(0 as libc::c_int as isize) =
        bestdir[0 as libc::c_int as usize];
    *rgflReturn.offset(1 as libc::c_int as isize) =
        bestdir[1 as libc::c_int as usize];
    *rgflReturn.offset(2 as libc::c_int as isize) =
        bestdir[2 as libc::c_int as usize];
}
/*
=========
pfnServerCommand

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnServerCommand(mut str: *const libc::c_char) {
    if SV_IsValidCmd(str) as u64 == 0 {
        Con_Printf(b"^1Error:^7 bad server command %s\n\x00" as *const u8 as
                       *const libc::c_char, str);
    } else { Cbuf_AddText(str); };
}
/*
=========
pfnServerExecute

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnServerExecute() {
    Cbuf_Execute();
    if svgame.config_executed as u64 != 0 { return }
    // here we restore arhcived cvars only from game.dll
    host.apply_game_config = true_0;
    Cbuf_AddText(b"exec config.cfg\n\x00" as *const u8 as
                     *const libc::c_char);
    Cbuf_Execute();
    if host.sv_cvars_restored > 0 as libc::c_int {
        Con_Reportf(b"server executing ^2config.cfg^7 (%i cvars)\n\x00" as
                        *const u8 as *const libc::c_char,
                    host.sv_cvars_restored);
    }
    host.apply_game_config = false_0;
    svgame.config_executed = true_0;
    host.sv_cvars_restored = 0 as libc::c_int;
}
/*
=========
pfnClientCommand

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnClientCommand(mut pEdict: *mut edict_t,
                                          mut szFmt: *mut libc::c_char,
                                          mut args: ...) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t; // early out
    let mut buffer: string = [0; 256];
    let mut args_0: ::std::ffi::VaListImpl;
    if sv.state as libc::c_uint != ss_active as libc::c_int as libc::c_uint {
        return
    }
    cl = SV_ClientFromEdict(pEdict, true_0);
    if cl.is_null() {
        Con_Printf(b"^1Error:^7 stuffcmd: client is not spawned!\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 { return }
    args_0 = args.clone();
    Q_vsnprintf(buffer.as_mut_ptr(), 256 as libc::c_int as size_t, szFmt,
                args_0.as_va_list());
    if SV_IsValidCmd(buffer.as_mut_ptr()) as u64 != 0 {
        MSG_WriteCmdExt(&mut (*cl).netchan.message, 9 as libc::c_int,
                        NS_SERVER, 0 as *const libc::c_char);
        MSG_WriteString(&mut (*cl).netchan.message, buffer.as_mut_ptr());
    } else {
        Con_Printf(b"^1Error:^7 Tried to stuff bad command %s\n\x00" as
                       *const u8 as *const libc::c_char, buffer.as_mut_ptr());
    };
}
/*
=================
pfnParticleEffect

Make sure the event gets sent to all clients
=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnParticleEffect(mut org: *const libc::c_float,
                                           mut dir: *const libc::c_float,
                                           mut color: libc::c_float,
                                           mut count: libc::c_float) {
    let mut v: libc::c_int = 0;
    if MSG_GetNumBytesLeft(&mut sv.datagram) < 16 as libc::c_int { return }
    MSG_WriteCmdExt(&mut sv.datagram, 18 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteVec3Coord(&mut sv.datagram, org);
    v =
        if *dir.offset(0 as libc::c_int as isize) * 16.0f32 >=
               -(128 as libc::c_int) as libc::c_float {
            if *dir.offset(0 as libc::c_int as isize) * 16.0f32 <
                   127 as libc::c_int as libc::c_float {
                (*dir.offset(0 as libc::c_int as isize)) * 16.0f32
            } else { 127 as libc::c_int as libc::c_float }
        } else { -(128 as libc::c_int) as libc::c_float } as libc::c_int;
    MSG_WriteChar(&mut sv.datagram, v);
    v =
        if *dir.offset(1 as libc::c_int as isize) * 16.0f32 >=
               -(128 as libc::c_int) as libc::c_float {
            if *dir.offset(1 as libc::c_int as isize) * 16.0f32 <
                   127 as libc::c_int as libc::c_float {
                (*dir.offset(1 as libc::c_int as isize)) * 16.0f32
            } else { 127 as libc::c_int as libc::c_float }
        } else { -(128 as libc::c_int) as libc::c_float } as libc::c_int;
    MSG_WriteChar(&mut sv.datagram, v);
    v =
        if *dir.offset(2 as libc::c_int as isize) * 16.0f32 >=
               -(128 as libc::c_int) as libc::c_float {
            if *dir.offset(2 as libc::c_int as isize) * 16.0f32 <
                   127 as libc::c_int as libc::c_float {
                (*dir.offset(2 as libc::c_int as isize)) * 16.0f32
            } else { 127 as libc::c_int as libc::c_float }
        } else { -(128 as libc::c_int) as libc::c_float } as libc::c_int;
    MSG_WriteChar(&mut sv.datagram, v);
    MSG_WriteByte(&mut sv.datagram, count as libc::c_int);
    MSG_WriteByte(&mut sv.datagram, color as libc::c_int);
    MSG_WriteByte(&mut sv.datagram, 0 as libc::c_int);
    // z-vel
}
/*
===============
pfnLightStyle

===============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnLightStyle(mut style: libc::c_int,
                                       mut val: *const libc::c_char) {
    if style < 0 as libc::c_int {
        style = 0 as libc::c_int
    } // don't let the world overwrite our restored styles
    if style >= 64 as libc::c_int {
        Host_Error(b"SV_LightStyle: style: %i >= %d\x00" as *const u8 as
                       *const libc::c_char, style, 64 as libc::c_int);
    }
    if sv.loadgame as u64 != 0 { return }
    SV_SetLightStyle(style, val, 0.0f32);
    // set correct style
}
/*
=================
pfnDecalIndex

register decal name on client
=================
*/
#[no_mangle]
pub unsafe extern "C" fn pfnDecalIndex(mut m: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if if m.is_null() || *m == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return -(1 as libc::c_int)
    }
    i = 1 as libc::c_int;
    while i < 512 as libc::c_int &&
              host.draw_decals[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(host.draw_decals[i as usize].as_mut_ptr(), m,
                      99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
=============
pfnMessageBegin

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnMessageBegin(mut msg_dest: libc::c_int,
                                         mut msg_num: libc::c_int,
                                         mut pOrigin: *const libc::c_float,
                                         mut ed: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut iSize: libc::c_int = 0;
    if svgame.msg_started as u64 != 0 {
        Host_Error(b"MessageBegin: New message started when msg \'%s\' has not been sent yet\n\x00"
                       as *const u8 as *const libc::c_char, svgame.msg_name);
    }
    svgame.msg_started = true_0;
    // check range
    msg_num =
        if msg_num >= 0 as libc::c_int {
            if msg_num < 255 as libc::c_int {
                msg_num
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int }; // this is a system message
    if msg_num <= 59 as libc::c_int {
        svgame.msg_index = -msg_num; // temp entity have variable size
        svgame.msg_name = svc_strings[msg_num as usize];
        if msg_num == 23 as libc::c_int {
            iSize = -(1 as libc::c_int)
        } else { iSize = 0 as libc::c_int }
    } else {
        // check for existing
        i = 1 as libc::c_int;
        while i < 197 as libc::c_int &&
                  svgame.msg[i as usize].name[0 as libc::c_int as usize] as
                      libc::c_int != 0 {
            if svgame.msg[i as usize].number == msg_num {
                break ;
                // found
            }
            i += 1
        }
        if i == 197 as libc::c_int {
            Host_Error(b"MessageBegin: tried to send unregistered message %i\n\x00"
                           as *const u8 as *const libc::c_char, msg_num);
            return
        }
        svgame.msg_name = svgame.msg[i as usize].name.as_mut_ptr();
        iSize = svgame.msg[i as usize].size;
        svgame.msg_index = i
    }
    MSG_WriteCmdExt(&mut sv.multicast, msg_num, NS_SERVER, svgame.msg_name);
    // save message destination
    if !pOrigin.is_null() {
        svgame.msg_org[0 as libc::c_int as usize] =
            *pOrigin.offset(0 as libc::c_int as
                                isize); // message has constant size
        svgame.msg_org[1 as libc::c_int as usize] =
            *pOrigin.offset(1 as libc::c_int as isize);
        svgame.msg_org[2 as libc::c_int as usize] =
            *pOrigin.offset(2 as libc::c_int as isize)
    } else {
        svgame.msg_org[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        svgame.msg_org[1 as libc::c_int as usize] =
            svgame.msg_org[2 as libc::c_int as usize];
        svgame.msg_org[0 as libc::c_int as usize] =
            svgame.msg_org[1 as libc::c_int as usize]
    }
    if iSize == -(1 as libc::c_int) {
        // variable sized messages sent size as first short
        svgame.msg_size_index = MSG_GetNumBytesWritten(&mut sv.multicast);
        MSG_WriteWord(&mut sv.multicast, 0 as libc::c_int);
        // reserve space for now
    } else { svgame.msg_size_index = -(1 as libc::c_int) }
    svgame.msg_realsize = 0 as libc::c_int;
    svgame.msg_dest = msg_dest;
    svgame.msg_ent = ed;
}
/*
=============
pfnMessageEnd

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnMessageEnd() {
    let mut name: *const libc::c_char =
        b"Unknown\x00" as *const u8 as *const libc::c_char;
    let mut org: *mut libc::c_float = 0 as *mut libc::c_float;
    if !svgame.msg_name.is_null() { name = svgame.msg_name }
    if svgame.msg_started as u64 == 0 {
        Host_Error(b"MessageEnd: called with no active message\n\x00" as
                       *const u8 as *const libc::c_char);
    }
    svgame.msg_started = false_0;
    if MSG_CheckOverflow(&mut sv.multicast) as u64 != 0 {
        Con_Printf(b"^1Error:^7 MessageEnd: %s has overflow multicast buffer\n\x00"
                       as *const u8 as *const libc::c_char, name);
        MSG_Clear(&mut sv.multicast);
        return
    }
    // check for system message
    if svgame.msg_index < 0 as libc::c_int {
        if svgame.msg_size_index != -(1 as libc::c_int) {
            // variable sized message
            if svgame.msg_realsize > 2048 as libc::c_int {
                Con_Printf(b"^1Error:^7 SV_Multicast: %s too long (more than %d bytes)\n\x00"
                               as *const u8 as *const libc::c_char, name,
                           2048 as libc::c_int);
                MSG_Clear(&mut sv.multicast);
                return
            } else {
                if svgame.msg_realsize < 0 as libc::c_int {
                    Con_Printf(b"^1Error:^7 SV_Multicast: %s writes NULL message\n\x00"
                                   as *const u8 as *const libc::c_char, name);
                    MSG_Clear(&mut sv.multicast);
                    return
                }
            }
            *sv.multicast.pData.offset(svgame.msg_size_index as isize) =
                svgame.msg_realsize as byte
        }
    } else if svgame.msg[svgame.msg_index as usize].size !=
                  -(1 as libc::c_int) {
        let mut expsize: libc::c_int =
            svgame.msg[svgame.msg_index as usize].size;
        let mut realsize: libc::c_int = svgame.msg_realsize;
        // compare sizes
        if expsize != realsize {
            Con_Printf(b"^1Error:^7 SV_Multicast: %s expected %i bytes, it written %i. Ignored.\n\x00"
                           as *const u8 as *const libc::c_char, name, expsize,
                       realsize);
            MSG_Clear(&mut sv.multicast);
            return
        }
    } else if svgame.msg_size_index != -(1 as libc::c_int) {
        // variable sized message
        if svgame.msg_realsize > 2048 as libc::c_int {
            Con_Printf(b"^1Error:^7 SV_Multicast: %s too long (more than %d bytes)\n\x00"
                           as *const u8 as *const libc::c_char, name,
                       2048 as libc::c_int);
            MSG_Clear(&mut sv.multicast);
            return
        } else {
            if svgame.msg_realsize < 0 as libc::c_int {
                Con_Printf(b"^1Error:^7 SV_Multicast: %s writes NULL message\n\x00"
                               as *const u8 as *const libc::c_char, name);
                MSG_Clear(&mut sv.multicast);
                return
            }
        }
        *(&mut *sv.multicast.pData.offset(svgame.msg_size_index as isize) as
              *mut byte as *mut word) = svgame.msg_realsize as word
    } else {
        // this should never happen
        Con_Printf(b"^1Error:^7 SV_Multicast: %s have encountered error\n\x00"
                       as *const u8 as *const libc::c_char, name);
        MSG_Clear(&mut sv.multicast);
        return
    }
    // update some messages in case their was format was changed and we want to keep backward compatibility
    if svgame.msg_index < 0 as libc::c_int {
        let mut svc_msg: libc::c_int = abs(svgame.msg_index);
        if (svc_msg == 31 as libc::c_int || svc_msg == 34 as libc::c_int) &&
               svgame.msg_realsize == 0 as libc::c_int {
            MSG_WriteChar(&mut sv.multicast, 0 as libc::c_int);
        }
        // write null string
    }
    if !(svgame.msg_org[0 as libc::c_int as usize] == 0.0f32 &&
             svgame.msg_org[1 as libc::c_int as usize] == 0.0f32 &&
             svgame.msg_org[2 as libc::c_int as usize] == 0.0f32) {
        org = svgame.msg_org.as_mut_ptr()
    }
    svgame.msg_dest =
        if svgame.msg_dest >= 0 as libc::c_int {
            if svgame.msg_dest < 9 as libc::c_int {
                svgame.msg_dest
            } else { 9 as libc::c_int }
        } else { 0 as libc::c_int };
    SV_Multicast(svgame.msg_dest, org as *const vec_t, svgame.msg_ent, true_0,
                 false_0);
}
/*
=============
pfnWriteByte

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteByte(mut iValue: libc::c_int) {
    if iValue == -(1 as libc::c_int) {
        iValue = 0xff as libc::c_int
    } // convert char to byte
    MSG_WriteByte(&mut sv.multicast, iValue as byte as libc::c_int);
    svgame.msg_realsize += 1;
}
/*
=============
pfnWriteChar

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteChar(mut iValue: libc::c_int) {
    MSG_WriteChar(&mut sv.multicast, iValue as libc::c_schar as libc::c_int);
    svgame.msg_realsize += 1;
}
/*
=============
pfnWriteShort

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteShort(mut iValue: libc::c_int) {
    MSG_WriteShort(&mut sv.multicast, iValue as libc::c_short as libc::c_int);
    svgame.msg_realsize += 2 as libc::c_int;
}
/*
=============
pfnWriteLong

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteLong(mut iValue: libc::c_int) {
    MSG_WriteLong(&mut sv.multicast, iValue);
    svgame.msg_realsize += 4 as libc::c_int;
}
/*
=============
pfnWriteAngle

this is low-res angle
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteAngle(mut flValue: libc::c_float) {
    let mut iAngle: libc::c_int =
        (flValue * 256 as libc::c_int as libc::c_float /
             360 as libc::c_int as libc::c_float) as libc::c_int &
            255 as libc::c_int;
    MSG_WriteChar(&mut sv.multicast, iAngle);
    svgame.msg_realsize += 1 as libc::c_int;
}
/*
=============
pfnWriteCoord

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteCoord(mut flValue: libc::c_float) {
    MSG_WriteCoord(&mut sv.multicast, flValue);
    svgame.msg_realsize += 2 as libc::c_int;
}
/*
=============
pfnWriteBytes

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteBytes(mut bytes: *const byte,
                                       mut count: libc::c_int) {
    MSG_WriteBytes(&mut sv.multicast, bytes as *const libc::c_void, count);
    svgame.msg_realsize += count;
}
/*
=============
pfnWriteString

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteString(mut src: *const libc::c_char) {
    static mut string: [libc::c_char; 2048] = [0; 2048];
    let mut len: libc::c_int =
        Q_strlen(src).wrapping_add(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let mut rem: libc::c_int =
        (::std::mem::size_of::<[libc::c_char; 2048]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    if len == 1 as libc::c_int {
        MSG_WriteChar(&mut sv.multicast, 0 as libc::c_int);
        svgame.msg_realsize += 1 as libc::c_int;
        return
        // fast exit
    }
    // prepare string to sending
    dst = string.as_mut_ptr();
    loop 
         // some escaped chars parsed as two symbols - merge it here
         {
        if *src.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\\' as i32 &&
               *src.offset(1 as libc::c_int as isize) as libc::c_int ==
                   'n' as i32 {
            let fresh3 = dst; // string end (not included in count)
            dst = dst.offset(1); // string end (not included in count)
            *fresh3 = '\n' as i32 as libc::c_char;
            src = src.offset(2 as libc::c_int as isize);
            len -= 1 as libc::c_int
        } else if *src.offset(0 as libc::c_int as isize) as libc::c_int ==
                      '\\' as i32 &&
                      *src.offset(1 as libc::c_int as isize) as libc::c_int ==
                          'r' as i32 {
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = '\r' as i32 as libc::c_char;
            src = src.offset(2 as libc::c_int as isize);
            len -= 1 as libc::c_int
        } else if *src.offset(0 as libc::c_int as isize) as libc::c_int ==
                      '\\' as i32 &&
                      *src.offset(1 as libc::c_int as isize) as libc::c_int ==
                          't' as i32 {
            let fresh5 = dst;
            dst = dst.offset(1);
            *fresh5 = '\t' as i32 as libc::c_char;
            src = src.offset(2 as libc::c_int as isize);
            len -= 1 as libc::c_int
        } else {
            let fresh6 = src;
            src = src.offset(1);
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = *fresh6;
            if *fresh7 as libc::c_int == 0 as libc::c_int { break ; }
        }
        rem -= 1;
        if !(rem <= 0 as libc::c_int) { continue ; }
        Con_Printf(b"^1Error:^7 pfnWriteString: exceeds %i symbols\n\x00" as
                       *const u8 as *const libc::c_char, len);
        *dst = '\u{0}' as i32 as libc::c_char;
        len =
            Q_strlen(string.as_mut_ptr()).wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong) as
                libc::c_int;
        break ;
    }
    *dst = '\u{0}' as i32 as libc::c_char;
    MSG_WriteString(&mut sv.multicast, string.as_mut_ptr());
    // NOTE: some messages with constant string length can be marked as known sized
    svgame.msg_realsize += len;
}
/*
=============
pfnWriteEntity

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnWriteEntity(mut iValue: libc::c_int) {
    if iValue < 0 as libc::c_int || iValue >= svgame.numEntities {
        Host_Error(b"MSG_WriteEntity: invalid entnumber %i\n\x00" as *const u8
                       as *const libc::c_char, iValue);
    }
    MSG_WriteShort(&mut sv.multicast, iValue as libc::c_short as libc::c_int);
    svgame.msg_realsize += 2 as libc::c_int;
}
/*
=============
pfnAlertMessage

=============
*/
unsafe extern "C" fn pfnAlertMessage(mut type_0: ALERT_TYPE,
                                     mut szFmt: *mut libc::c_char,
                                     mut args: ...) {
    let mut buffer: [libc::c_char; 2048] = [0; 2048];
    let mut args_0: ::std::ffi::VaListImpl;
    if type_0 as libc::c_uint == at_logged as libc::c_int as libc::c_uint &&
           svs.maxclients > 1 as libc::c_int {
        args_0 = args.clone();
        Q_vsnprintf(buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 2048]>() as
                        libc::c_ulong, szFmt, args_0.as_va_list());
        Log_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   buffer.as_mut_ptr());
        return
    }
    if host_developer.value <= DEV_NONE as libc::c_int as libc::c_float {
        return
    }
    // g-cont: some mods have wrong aiconsole messages that crash the engine
    if type_0 as libc::c_uint == at_aiconsole as libc::c_int as libc::c_uint
           &&
           host_developer.value < DEV_EXTENDED as libc::c_int as libc::c_float
       {
        return
    }
    args_0 = args.clone();
    Q_vsnprintf(buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 2048]>() as
                    libc::c_ulong, szFmt, args_0.as_va_list());
    // check message for pass
    match type_0 as libc::c_uint {
        0 => {
            Con_Printf(b"^2Note:^7 %s\x00" as *const u8 as
                           *const libc::c_char, buffer.as_mut_ptr());
        }
        1 => {
            Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       buffer.as_mut_ptr());
        }
        2 => {
            Con_DPrintf(b"%s\x00" as *const u8 as *const libc::c_char,
                        buffer.as_mut_ptr());
        }
        3 => {
            Con_Printf(b"^3Warning:^7 %s\x00" as *const u8 as
                           *const libc::c_char, buffer.as_mut_ptr());
        }
        4 => {
            Con_Printf(b"^1Error:^7 %s\x00" as *const u8 as
                           *const libc::c_char, buffer.as_mut_ptr());
        }
        _ => { }
    };
}
/*
=============
pfnEngineFprintf

OBSOLETE, UNUSED
=============
*/
unsafe extern "C" fn pfnEngineFprintf(mut pfile: *mut FILE,
                                      mut szFmt: *mut libc::c_char,
                                      mut args: ...) {
}
/*
=============
pfnBuildSoundMsg

Customizable sound message
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnBuildSoundMsg(mut pSource: *mut edict_t,
                                          mut chan: libc::c_int,
                                          mut samp: *const libc::c_char,
                                          mut fvol: libc::c_float,
                                          mut attn: libc::c_float,
                                          mut fFlags: libc::c_int,
                                          mut pitch: libc::c_int,
                                          mut msg_dest: libc::c_int,
                                          mut msg_type: libc::c_int,
                                          mut pOrigin: *const libc::c_float,
                                          mut pSend: *mut edict_t) {
    pfnMessageBegin(msg_dest, msg_type, pOrigin, pSend);
    SV_BuildSoundMsg(&mut sv.multicast, pSource, chan, samp,
                     (fvol * 255 as libc::c_int as libc::c_float) as
                         libc::c_int, attn, fFlags, pitch, pOrigin);
    pfnMessageEnd();
}
/*
=============
pfnPvAllocEntPrivateData

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPvAllocEntPrivateData(mut pEdict: *mut edict_t,
                                                  mut cb: libc::c_long)
 -> *mut libc::c_void {
    SV_FreePrivateData(pEdict);
    if cb > 0 as libc::c_int as libc::c_long {
        // a poke646 have memory corrupt in somewhere - this is trashed last sixteen bytes :(
        (*pEdict).pvPrivateData =
            _Mem_Alloc(svgame.mempool,
                       (cb + 15 as libc::c_int as libc::c_long &
                            !(15 as libc::c_int) as libc::c_long) as size_t,
                       true_0,
                       b"../engine/server/sv_game.c\x00" as *const u8 as
                           *const libc::c_char, 2970 as libc::c_int)
    }
    return (*pEdict).pvPrivateData;
}
/*
=============
pfnPvEntPrivateData

we already have copy of this function in 'enginecallback.h' :-)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPvEntPrivateData(mut pEdict: *mut edict_t)
 -> *mut libc::c_void {
    if !pEdict.is_null() { return (*pEdict).pvPrivateData }
    return 0 as *mut libc::c_void;
}
static mut str64: str64_s =
    str64_s{maxstringarray: 0,
            allowdup: false_0,
            staticstringarray: 0 as *const libc::c_char as *mut libc::c_char,
            pstringarray: 0 as *const libc::c_char as *mut libc::c_char,
            pstringarraystatic: 0 as *const libc::c_char as *mut libc::c_char,
            pstringbase: 0 as *const libc::c_char as *mut libc::c_char,
            poldstringbase: 0 as *const libc::c_char as *mut libc::c_char,
            plast: 0 as *const libc::c_char as *mut libc::c_char,
            dynamic: false_0,
            maxalloc: 0,
            numdups: 0,
            numoverflows: 0,
            totalalloc: 0,};
/*
==================
SV_EmptyStringPool

Free strings on server stop. Reset string pointer on 64 bits
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EmptyStringPool() {
    if str64.dynamic as u64 != 0 {
        // switch only after array fill (more space for multiplayer games)
        str64.pstringbase = str64.pstringarray
    } else {
        str64.poldstringbase = str64.pstringarraystatic;
        str64.pstringbase = str64.poldstringbase;
        str64.plast = str64.pstringbase.offset(1 as libc::c_int as isize)
    };
}
/*
===============
SV_SetStringArrayMode

use different arrays on 64 bit platforms
set dynamic after complete server spawn
this helps not to lose strings that belongs to static game part
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SetStringArrayMode(mut dynamic: qboolean) {
    Con_Reportf(b"SV_SetStringArrayMode(%d) %d\n\x00" as *const u8 as
                    *const libc::c_char, dynamic as libc::c_uint,
                str64.dynamic as libc::c_uint);
    if dynamic as libc::c_uint == str64.dynamic as libc::c_uint { return }
    str64.dynamic = dynamic;
    SV_EmptyStringPool();
}
/*
==================
SV_AllocStringPool

alloc string pool on 32bit platforms
alloc string array near the server library on 64bit platforms if possible
alloc string array somewhere if not (MAKE_STRING will not work. Always call ALLOC_STRING instead, or crash)
this case need patched game dll with MAKE_STRING checking ptrdiff size
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AllocStringPool() {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut lenstr: string = [0; 256];
    Con_Reportf(b"SV_AllocStringPool()\n\x00" as *const u8 as
                    *const libc::c_char);
    if _Sys_GetParmFromCmdLine(b"-str64alloc\x00" as *const u8 as
                                   *const libc::c_char, lenstr.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong) as u64 != 0 {
        str64.maxstringarray = Q_atoi(lenstr.as_mut_ptr()) as size_t;
        if str64.maxstringarray < 1024 as libc::c_int as libc::c_ulong ||
               str64.maxstringarray >=
                   2147483647 as libc::c_int as libc::c_ulong {
            str64.maxstringarray = 65536 as libc::c_int as size_t
        }
    } else { str64.maxstringarray = 65536 as libc::c_int as size_t }
    if Sys_CheckParm(b"-str64dup\x00" as *const u8 as *const libc::c_char) !=
           0 {
        str64.allowdup = true_0
    }
    let mut pagesize: size_t = sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
    let mut arrlen: libc::c_int =
        (str64.maxstringarray.wrapping_mul(2 as libc::c_int as libc::c_ulong)
             & !pagesize.wrapping_sub(1 as libc::c_int as libc::c_ulong)) as
            libc::c_int;
    let mut base: *mut libc::c_void =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                *mut libc::c_void>(svgame.dllFuncs.pfnGameInit);
    let mut start: *mut libc::c_void =
        svgame.hInstance.offset(-(arrlen as isize));
    while start.wrapping_offset_from(base) as libc::c_long >
              (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                  libc::c_long {
        let mut mapptr: *mut libc::c_void =
            mmap((start as libc::c_ulong &
                      !pagesize.wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong)) as
                     *mut libc::c_void, arrlen as size_t,
                 0x1 as libc::c_int | 0x2 as libc::c_int,
                 0x20 as libc::c_int | 0x2 as libc::c_int, 0 as libc::c_int,
                 0 as libc::c_int as __off_t);
        if !mapptr.is_null() &&
               mapptr != -(1 as libc::c_int) as *mut libc::c_void &&
               mapptr.wrapping_offset_from(base) as libc::c_long >
                   (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                       libc::c_long &&
               (mapptr.wrapping_offset_from(base) as libc::c_long) <
                   2147483647 as libc::c_int as libc::c_long {
            ptr = mapptr;
            break ;
        } else {
            if !mapptr.is_null() { munmap(mapptr, arrlen as size_t); }
            start = start.offset(-(arrlen as isize))
        }
    }
    if ptr.is_null() {
        start = base;
        while (start.wrapping_offset_from(base) as libc::c_long) <
                  2147483647 as libc::c_int as libc::c_long {
            let mut mapptr_0: *mut libc::c_void =
                mmap((start as libc::c_ulong &
                          !pagesize.wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong)) as
                         *mut libc::c_void, arrlen as size_t,
                     0x1 as libc::c_int | 0x2 as libc::c_int,
                     0x20 as libc::c_int | 0x2 as libc::c_int,
                     0 as libc::c_int, 0 as libc::c_int as __off_t);
            if !mapptr_0.is_null() &&
                   mapptr_0 != -(1 as libc::c_int) as *mut libc::c_void &&
                   mapptr_0.wrapping_offset_from(base) as libc::c_long >
                       (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                           libc::c_long &&
                   (mapptr_0.wrapping_offset_from(base) as libc::c_long) <
                       2147483647 as libc::c_int as libc::c_long {
                ptr = mapptr_0;
                break ;
            } else {
                if !mapptr_0.is_null() { munmap(mapptr_0, arrlen as size_t); }
                start = start.offset(arrlen as isize)
            }
        }
    }
    if !ptr.is_null() {
        Con_Reportf(b"SV_AllocStringPool: Allocated string array near the server library: %p %p\n\x00"
                        as *const u8 as *const libc::c_char, base, ptr);
    } else {
        Con_Reportf(b"SV_AllocStringPool: Failed to allocate string array near the server library!\n\x00"
                        as *const u8 as *const libc::c_char);
        str64.staticstringarray =
            _Mem_Alloc(host.mempool,
                       str64.maxstringarray.wrapping_mul(2 as libc::c_int as
                                                             libc::c_ulong),
                       true_0,
                       b"../engine/server/sv_game.c\x00" as *const u8 as
                           *const libc::c_char, 3133 as libc::c_int) as
                *mut libc::c_char;
        ptr = str64.staticstringarray as *mut libc::c_void
    }
    str64.pstringarray = ptr as *mut libc::c_char;
    str64.pstringarraystatic =
        (ptr as *mut byte).offset(str64.maxstringarray as isize) as
            *mut libc::c_char;
    str64.poldstringbase = ptr as *mut libc::c_char;
    str64.pstringbase = str64.poldstringbase;
    str64.plast =
        (ptr as *mut byte).offset(1 as libc::c_int as isize) as
            *mut libc::c_char;
    (*svgame.globals).pStringBase = ptr as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn SV_FreeStringPool() {
    Con_Reportf(b"SV_FreeStringPool()\n\x00" as *const u8 as
                    *const libc::c_char);
    if str64.pstringarray != str64.staticstringarray {
        munmap(str64.pstringarray as *mut libc::c_void,
               str64.maxstringarray.wrapping_mul(2 as libc::c_int as
                                                     libc::c_ulong) &
                   !(sysconf(_SC_PAGESIZE as libc::c_int) -
                         1 as libc::c_int as libc::c_long) as libc::c_ulong);
    } else {
        _Mem_Free(str64.staticstringarray as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 3161 as libc::c_int);
    };
}
/*
=============
SV_AllocString

allocate new engine string
on 64bit platforms find in array string if deduplication enabled (default)
if not found, add to array
use -str64dup to disable deduplication, -str64alloc to set array size
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AllocString(mut szValue: *const libc::c_char)
 -> string_t {
    let mut newString: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmp: libc::c_int = 0;
    if svgame.physFuncs.pfnAllocString.is_some() {
        return svgame.physFuncs.pfnAllocString.expect("non-null function pointer")(szValue)
    }
    cmp = 1 as libc::c_int;
    if str64.allowdup as u64 == 0 {
        newString = str64.poldstringbase.offset(1 as libc::c_int as isize);
        while newString < str64.plast as *const libc::c_char &&
                  {
                      cmp =
                          Q_strncmp(newString, szValue, 99999 as libc::c_int);
                      (cmp) != 0
                  } {
            newString =
                newString.offset(Q_strlen(newString).wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                                     as isize)
        }
    }
    if cmp != 0 {
        let mut len: uint = Q_strlen(szValue) as uint;
        if (str64.plast.wrapping_offset_from(str64.poldstringbase) as
                libc::c_long + len as libc::c_long +
                2 as libc::c_int as libc::c_long) as libc::c_ulong >
               str64.maxstringarray {
            str64.plast = str64.pstringbase.offset(1 as libc::c_int as isize);
            str64.poldstringbase = str64.pstringbase;
            str64.numoverflows = str64.numoverflows.wrapping_add(1)
        }
        //MsgDev( D_NOTE, "SV_AllocString: %ld %s\n", str64.plast - svgame.globals->pStringBase, szValue );
        memcpy(str64.plast as *mut libc::c_void,
               szValue as *const libc::c_void,
               len.wrapping_add(1 as libc::c_int as libc::c_uint) as
                   libc::c_ulong);
        str64.totalalloc =
            (str64.totalalloc as
                 libc::c_ulong).wrapping_add(len.wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                                 as libc::c_ulong) as size_t
                as size_t;
        newString = str64.plast;
        str64.plast =
            str64.plast.offset(len.wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as isize)
    } else { str64.numdups = str64.numdups.wrapping_add(1) }
    //MsgDev( D_NOTE, "SV_AllocString: dup %ld %s\n", newString - svgame.globals->pStringBase, szValue );
    if newString.wrapping_offset_from(str64.pstringarray) as libc::c_long as
           libc::c_ulong > str64.maxalloc {
        str64.maxalloc =
            newString.wrapping_offset_from(str64.pstringarray) as libc::c_long
                as size_t
    }
    return newString.wrapping_offset_from((*svgame.globals).pStringBase) as
               libc::c_long as string_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_PrintStr64Stats_f() {
    Con_Printf(b"====================\n\x00" as *const u8 as
                   *const libc::c_char);
    Con_Printf(b"64 bit string pool statistics\n\x00" as *const u8 as
                   *const libc::c_char);
    Con_Printf(b"====================\n\x00" as *const u8 as
                   *const libc::c_char);
    Con_Printf(b"string array size: %lu\n\x00" as *const u8 as
                   *const libc::c_char, str64.maxstringarray);
    Con_Printf(b"total alloc %lu\n\x00" as *const u8 as *const libc::c_char,
               str64.totalalloc);
    Con_Printf(b"maximum array usage: %lu\n\x00" as *const u8 as
                   *const libc::c_char, str64.maxalloc);
    Con_Printf(b"overflow counter: %lu\n\x00" as *const u8 as
                   *const libc::c_char, str64.numoverflows);
    Con_Printf(b"dup string counter: %lu\n\x00" as *const u8 as
                   *const libc::c_char, str64.numdups);
}
/*
=============
SV_MakeString

make constant string
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_MakeString(mut szValue: *const libc::c_char)
 -> string_t {
    if svgame.physFuncs.pfnMakeString.is_some() {
        return svgame.physFuncs.pfnMakeString.expect("non-null function pointer")(szValue)
    }
    let mut ptrdiff: libc::c_longlong =
        szValue.wrapping_offset_from((*svgame.globals).pStringBase) as
            libc::c_long as libc::c_longlong;
    if ptrdiff > 2147483647 as libc::c_int as libc::c_longlong ||
           ptrdiff <
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_longlong {
        return SV_AllocString(szValue)
    } else { return ptrdiff as libc::c_int };
}
/*
=============
SV_GetString

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetString(mut iString: string_t)
 -> *const libc::c_char {
    if svgame.physFuncs.pfnGetString.is_some() {
        return svgame.physFuncs.pfnGetString.expect("non-null function pointer")(iString)
    }
    return (*svgame.globals).pStringBase.offset(iString as isize);
}
/*
=============
pfnGetVarsOfEnt

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetVarsOfEnt(mut pEdict: *mut edict_t)
 -> *mut entvars_t {
    if !pEdict.is_null() { return &mut (*pEdict).v }
    return 0 as *mut entvars_t;
}
/*
=============
pfnPEntityOfEntOffset

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPEntityOfEntOffset(mut iEntOffset: libc::c_int)
 -> *mut edict_t {
    return (svgame.edicts as *mut byte).offset(iEntOffset as isize) as
               *mut edict_t;
}
/*
=============
pfnEntOffsetOfPEntity

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnEntOffsetOfPEntity(mut pEdict: *const edict_t)
 -> libc::c_int {
    return (pEdict as
                *mut byte).wrapping_offset_from(svgame.edicts as *mut byte) as
               libc::c_long as libc::c_int;
}
/*
=============
pfnIndexOfEdict

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnIndexOfEdict(mut pEdict: *const edict_t)
 -> libc::c_int {
    let mut number: libc::c_int = 0; // world ?
    if pEdict.is_null() { return 0 as libc::c_int }
    number =
        (pEdict as *mut edict_t).wrapping_offset_from(svgame.edicts) as
            libc::c_long as libc::c_int;
    if number < 0 as libc::c_int || number > (*SI.GameInfo).max_edicts {
        Host_Error(b"bad entity number %d\n\x00" as *const u8 as
                       *const libc::c_char, number);
    }
    return number;
}
/*
=============
pfnPEntityOfEntIndex

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPEntityOfEntIndex(mut iEntIndex: libc::c_int)
 -> *mut edict_t {
    if iEntIndex >= 0 as libc::c_int && iEntIndex < (*SI.GameInfo).max_edicts
       {
        let mut pEdict: *mut edict_t =
            SV_EdictNum(iEntIndex); // just get access to array
        if iEntIndex == 0 ||
               host.features &
                   ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
                   0 {
            return pEdict
        }
        if SV_CheckEdict(pEdict,
                         b"../engine/server/sv_game.c\x00" as *const u8 as
                             *const libc::c_char, 3344 as libc::c_int) as
               libc::c_uint != 0 && !(*pEdict).pvPrivateData.is_null() {
            return pEdict
        }
        // g-cont: world and clients can be acessed even without private data!
        if SV_CheckEdict(pEdict,
                         b"../engine/server/sv_game.c\x00" as *const u8 as
                             *const libc::c_char, 3348 as libc::c_int) as
               libc::c_uint != 0 &&
               SV_IsPlayerIndex(iEntIndex) as libc::c_uint != 0 {
            return pEdict
        }
    }
    return 0 as *mut edict_t;
}
/*
=============
pfnFindEntityByVars

debug thing
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnFindEntityByVars(mut pvars: *mut entvars_t)
 -> *mut edict_t {
    let mut pEdict: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    // don't pass invalid arguments
    if pvars.is_null() { return 0 as *mut edict_t }
    i = 0 as libc::c_int;
    while i < (*SI.GameInfo).max_edicts {
        pEdict = SV_EdictNum(i);
        // found it
        if &mut (*pEdict).v as *mut entvars_t == pvars { return pEdict }
        i += 1
    }
    return 0 as *mut edict_t;
}
// g-cont: we should compare pointers
/*
=============
pfnGetModelPtr

returns pointer to a studiomodel
=============
*/
unsafe extern "C" fn pfnGetModelPtr(mut pEdict: *mut edict_t)
 -> *mut libc::c_void {
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3393 as libc::c_int) as u64 == 0
       {
        return 0 as *mut libc::c_void
    }
    mod_0 = SV_ModelHandle((*pEdict).v.modelindex);
    return Mod_StudioExtradata(mod_0);
}
/*
=============
SV_SendUserReg

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SendUserReg(mut msg: *mut sizebuf_t,
                                        mut user: *mut sv_user_message_t) {
    MSG_WriteCmdExt(msg, 39 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteByte(msg, (*user).number);
    MSG_WriteWord(msg, (*user).size as word as libc::c_int);
    MSG_WriteString(msg, (*user).name.as_mut_ptr());
}
/*
=============
pfnRegUserMsg

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnRegUserMsg(mut pszName: *const libc::c_char,
                                       mut iSize: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if if pszName.is_null() || *pszName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    if Q_strlen(pszName) >=
           ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong {
        Con_Printf(b"^1Error:^7 REG_USER_MSG: too long name %s\n\x00" as
                       *const u8 as *const libc::c_char, pszName);
        return 0 as libc::c_int
        // force error
    }
    if iSize > 2048 as libc::c_int {
        Con_Printf(b"^1Error:^7 REG_USER_MSG: %s has too big size %i\n\x00" as
                       *const u8 as *const libc::c_char, pszName, iSize);
        return 0 as libc::c_int
        // force error
    }
    // make sure what size inrange
    iSize =
        if iSize >= -(1 as libc::c_int) {
            if iSize < 2048 as libc::c_int {
                iSize
            } else { 2048 as libc::c_int }
        } else { -(1 as libc::c_int) };
    // message 0 is reserved for svc_bad
    i = 1 as libc::c_int;
    while i < 197 as libc::c_int &&
              svgame.msg[i as usize].name[0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        // see if already registered
        if Q_strncmp(svgame.msg[i as usize].name.as_mut_ptr(), pszName,
                     99999 as libc::c_int) == 0 {
            return 59 as libc::c_int + i
        }
        i += 1
        // offset
    }
    if i == 197 as libc::c_int {
        Con_Printf(b"^1Error:^7 REG_USER_MSG: user messages limit exceeded\n\x00"
                       as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    // register new message
    Q_strncpy(svgame.msg[i as usize].name.as_mut_ptr(), pszName,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    svgame.msg[i as usize].number = 59 as libc::c_int + i;
    svgame.msg[i as usize].size = iSize;
    if sv.state as libc::c_uint == ss_active as libc::c_int as libc::c_uint {
        // tell the client about new user message
        SV_SendUserReg(&mut sv.multicast,
                       &mut *svgame.msg.as_mut_ptr().offset(i as isize));
        SV_Multicast(2 as libc::c_int, 0 as *const vec_t, 0 as *const edict_t,
                     false_0, false_0);
    }
    return svgame.msg[i as usize].number;
}
/*
=============
pfnAnimationAutomove

OBSOLETE, UNUSED
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnAnimationAutomove(mut pEdict: *const edict_t,
                                              mut flTime: libc::c_float) {
}
/*
=============
pfnGetBonePosition

=============
*/
unsafe extern "C" fn pfnGetBonePosition(mut pEdict: *const edict_t,
                                        mut iBone: libc::c_int,
                                        mut rgflOrigin: *mut libc::c_float,
                                        mut rgflAngles: *mut libc::c_float) {
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3490 as libc::c_int) as u64 == 0
       {
        return
    }
    Mod_GetBonePosition(pEdict, iBone, rgflOrigin, rgflAngles);
}
/*
=============
pfnFunctionFromName

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnFunctionFromName(mut pName: *const libc::c_char)
 -> *mut libc::c_void {
    return COM_FunctionFromName_SR(svgame.hInstance, pName);
}
/*
=============
pfnNameForFunction

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnNameForFunction(mut function: *mut libc::c_void)
 -> *const libc::c_char {
    return COM_NameForFunction(svgame.hInstance, function);
}
/*
=============
pfnClientPrintf

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnClientPrintf(mut pEdict: *mut edict_t,
                                         mut ptype: PRINT_TYPE,
                                         mut szMsg: *const libc::c_char) {
    let mut client: *mut sv_client_t = 0 as *mut sv_client_t;
    client = SV_ClientFromEdict(pEdict, false_0);
    if client.is_null() {
        Con_Printf(b"tried to sprint to a non-client\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if (*client).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        return
    }
    match ptype as libc::c_uint {
        0 | 2 => {
            SV_ClientPrintf(client,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            szMsg);
        }
        1 => {
            MSG_WriteCmdExt(&mut (*client).netchan.message, 26 as libc::c_int,
                            NS_SERVER, 0 as *const libc::c_char);
            MSG_WriteString(&mut (*client).netchan.message, szMsg);
        }
        _ => { }
    };
}
/*
=============
pfnServerPrint

print to the server console
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnServerPrint(mut szMsg: *const libc::c_char) {
    if host.features &
           ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        SV_BroadcastPrintf(0 as *mut sv_client_s,
                           b"%s\x00" as *const u8 as *const libc::c_char,
                           szMsg);
    } else {
        Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char, szMsg);
    };
}
/*
=============
pfnGetAttachment

=============
*/
unsafe extern "C" fn pfnGetAttachment(mut pEdict: *const edict_t,
                                      mut iAttachment: libc::c_int,
                                      mut rgflOrigin: *mut libc::c_float,
                                      mut rgflAngles: *mut libc::c_float) {
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3571 as libc::c_int) as u64 == 0
       {
        return
    }
    Mod_StudioGetAttachment(pEdict, iAttachment, rgflOrigin, rgflAngles);
}
/*
=============
pfnCrosshairAngle

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCrosshairAngle(mut pClient: *const edict_t,
                                           mut pitch: libc::c_float,
                                           mut yaw: libc::c_float) {
    let mut client: *mut sv_client_t = 0 as *mut sv_client_t;
    client = SV_ClientFromEdict(pClient, true_0);
    if client.is_null() { return }
    // fakeclients ignores it silently
    if (*client).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        return
    }
    if pitch > 180.0f32 { pitch -= 360 as libc::c_int as libc::c_float }
    if pitch < -180.0f32 { pitch += 360 as libc::c_int as libc::c_float }
    if yaw > 180.0f32 { yaw -= 360 as libc::c_int as libc::c_float }
    if yaw < -180.0f32 { yaw += 360 as libc::c_int as libc::c_float }
    MSG_WriteCmdExt(&mut (*client).netchan.message, 47 as libc::c_int,
                    NS_SERVER, 0 as *const libc::c_char);
    MSG_WriteChar(&mut (*client).netchan.message,
                  (pitch * 5 as libc::c_int as libc::c_float) as libc::c_int);
    MSG_WriteChar(&mut (*client).netchan.message,
                  (yaw * 5 as libc::c_int as libc::c_float) as libc::c_int);
}
/*
=============
pfnSetView

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetView(mut pClient: *const edict_t,
                                    mut pViewent: *const edict_t) {
    let mut client: *mut sv_client_t =
        0 as *mut sv_client_t; // just reset viewentity
    let mut viewEnt: libc::c_int = 0;
    if SV_CheckEdict(pClient,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3614 as libc::c_int) as u64 == 0
       {
        return
    }
    client = SV_ClientFromEdict(pClient, false_0);
    if client.is_null() {
        Con_Printf(b"^1Error:^7 PF_SetView_I: not a client!\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    if SV_CheckEdict(pViewent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3623 as libc::c_int) as u64 == 0
           || pClient == pViewent {
        (*client).pViewEntity = 0 as *mut edict_t
    } else { (*client).pViewEntity = pViewent as *mut edict_t }
    // fakeclients ignore to send client message (but can see into the trigger_camera through the PVS)
    if (*client).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        return
    }
    if !(*client).pViewEntity.is_null() {
        viewEnt =
            (*client).pViewEntity.wrapping_offset_from(svgame.edicts) as
                libc::c_long as libc::c_int
    } else {
        viewEnt =
            (*client).edict.wrapping_offset_from(svgame.edicts) as
                libc::c_long as libc::c_int
    }
    MSG_WriteCmdExt(&mut (*client).netchan.message, 5 as libc::c_int,
                    NS_SERVER, 0 as *const libc::c_char);
    MSG_WriteWord(&mut (*client).netchan.message, viewEnt);
}
/*
=============
pfnStaticDecal

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnStaticDecal(mut origin: *const libc::c_float,
                                        mut decalIndex: libc::c_int,
                                        mut entityIndex: libc::c_int,
                                        mut modelIndex: libc::c_int) {
    SV_CreateDecal(&mut sv.signon, origin, decalIndex, entityIndex,
                   modelIndex, 0x1 as libc::c_int, 1.0f32);
}
/*
=============
pfnIsDedicatedServer

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnIsDedicatedServer() -> libc::c_int {
    return (host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/*
=============
pfnGetPlayerWONId

OBSOLETE, UNUSED
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPlayerWONId(mut e: *mut edict_t) -> uint {
    return -(1 as libc::c_int) as uint;
}
/*
=============
pfnIsMapValid

vaild map must contain one info_player_deatchmatch
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnIsMapValid(mut filename: *mut libc::c_char)
 -> libc::c_int {
    let mut flags: uint =
        SV_MapIsValid(filename, (*SI.GameInfo).mp_entity.as_mut_ptr(),
                      0 as *const libc::c_char);
    if flags & (1 as libc::c_uint) << 0 as libc::c_int != 0 &&
           flags & (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
/*
=============
pfnFadeClientVolume

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnFadeClientVolume(mut pEdict: *const edict_t,
                                             mut fadePercent: libc::c_int,
                                             mut fadeOutSeconds: libc::c_int,
                                             mut holdTime: libc::c_int,
                                             mut fadeInSeconds: libc::c_int) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    cl = SV_ClientFromEdict(pEdict, true_0);
    if cl.is_null() { return }
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 { return }
    MSG_WriteCmdExt(&mut (*cl).netchan.message, 48 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteByte(&mut (*cl).netchan.message, fadePercent);
    MSG_WriteByte(&mut (*cl).netchan.message, holdTime);
    MSG_WriteByte(&mut (*cl).netchan.message, fadeOutSeconds);
    MSG_WriteByte(&mut (*cl).netchan.message, fadeInSeconds);
}
/*
=============
pfnSetClientMaxspeed

fakeclients can be changed speed to
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetClientMaxspeed(mut pEdict: *const edict_t,
                                              mut fNewMaxspeed:
                                                  libc::c_float) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    // not spawned clients allowed
    cl = SV_ClientFromEdict(pEdict, false_0);
    if cl.is_null() { return }
    fNewMaxspeed =
        if fNewMaxspeed >= -svgame.movevars.maxspeed {
            if fNewMaxspeed < svgame.movevars.maxspeed {
                fNewMaxspeed
            } else { svgame.movevars.maxspeed }
        } else { -svgame.movevars.maxspeed };
    Info_SetValueForKey((*cl).physinfo.as_mut_ptr(),
                        b"maxspd\x00" as *const u8 as *const libc::c_char,
                        va(b"%.f\x00" as *const u8 as *const libc::c_char,
                           fNewMaxspeed as libc::c_double),
                        256 as libc::c_int);
    (*(*cl).edict).v.maxspeed = fNewMaxspeed;
}
/*
=============
pfnRunPlayerMove

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnRunPlayerMove(mut pClient: *mut edict_t,
                                          mut viewangles:
                                              *const libc::c_float,
                                          mut fmove: libc::c_float,
                                          mut smove: libc::c_float,
                                          mut upmove: libc::c_float,
                                          mut buttons: word,
                                          mut impulse: byte, mut msec: byte) {
    let mut cl: *mut sv_client_t =
        0 as *mut sv_client_t; // only fakeclients allows
    let mut oldcl: *mut sv_client_t = 0 as *mut sv_client_t; // full range
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
    let mut seed: uint = 0;
    cl = SV_ClientFromEdict(pClient, true_0);
    if cl.is_null() { return }
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 { return }
    oldcl = sv.current_client;
    sv.current_client = SV_ClientFromEdict(pClient, true_0);
    (*sv.current_client).timebase =
        sv.time + sv.frametime as libc::c_double -
            msec as libc::c_double / 1000.0f64;
    memset(&mut cmd as *mut usercmd_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    cmd.viewangles[0 as libc::c_int as usize] =
        *viewangles.offset(0 as libc::c_int as isize);
    cmd.viewangles[1 as libc::c_int as usize] =
        *viewangles.offset(1 as libc::c_int as isize);
    cmd.viewangles[2 as libc::c_int as usize] =
        *viewangles.offset(2 as libc::c_int as isize);
    cmd.forwardmove = fmove;
    cmd.sidemove = smove;
    cmd.upmove = upmove;
    cmd.buttons = buttons;
    cmd.impulse = impulse;
    cmd.msec = msec;
    seed =
        COM_RandomLong(0 as libc::c_int, 0x7fffffff as libc::c_int) as uint;
    SV_RunCmd(cl, &mut cmd, seed as libc::c_int);
    (*cl).lastcmd = cmd;
    sv.current_client = oldcl;
}
/*
=============
pfnNumberOfEntities

returns actual entity count
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnNumberOfEntities() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < svgame.numEntities {
        if !((*svgame.edicts.offset(i as isize)).free as u64 != 0) {
            total += 1
        }
        i += 1
    }
    return total;
}
/*
=============
pfnGetInfoKeyBuffer

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetInfoKeyBuffer(mut e: *mut edict_t)
 -> *mut libc::c_char {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    // NULL passes localinfo
    if SV_CheckEdict(e,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3804 as libc::c_int) as u64 == 0
       {
        return SV_Localinfo()
    }
    // world passes serverinfo
    if e == svgame.edicts { return SV_Serverinfo() }
    // userinfo for specified edict
    cl = SV_ClientFromEdict(e, false_0);
    if !cl.is_null() { return (*cl).userinfo.as_mut_ptr() }
    return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    // assume error
}
/*
=============
pfnSetValueForKey

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetValueForKey(mut infobuffer: *mut libc::c_char,
                                           mut key: *mut libc::c_char,
                                           mut value: *mut libc::c_char) {
    if infobuffer == svs.localinfo.as_mut_ptr() {
        Info_SetValueForStarKey(infobuffer, key, value, 32768 as libc::c_int);
    } else if infobuffer == svs.serverinfo.as_mut_ptr() {
        Info_SetValueForStarKey(infobuffer, key, value, 512 as libc::c_int);
    } else {
        Con_Printf(b"^1Error:^7 can\'t set client keys with SetValueForKey\n\x00"
                       as *const u8 as *const libc::c_char);
    };
}
/*
=============
pfnSetClientKeyValue

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetClientKeyValue(mut clientIndex: libc::c_int,
                                              mut infobuffer:
                                                  *mut libc::c_char,
                                              mut key: *mut libc::c_char,
                                              mut value: *mut libc::c_char) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if infobuffer == svs.localinfo.as_mut_ptr() ||
           infobuffer == svs.serverinfo.as_mut_ptr() {
        return
    }
    clientIndex -= 1 as libc::c_int;
    if svs.clients.is_null() || clientIndex < 0 as libc::c_int ||
           clientIndex >= svs.maxclients {
        return
    }
    // value not changed?
    if Q_strncmp(Info_ValueForKey(infobuffer, key), value,
                 99999 as libc::c_int) == 0 {
        return
    }
    cl = &mut *svs.clients.offset(clientIndex as isize) as *mut sv_client_t;
    Info_SetValueForStarKey(infobuffer, key, value, 256 as libc::c_int);
    (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 0 as libc::c_int;
    (*cl).next_sendinfotime = 0.0f64;
    // send immediately
}
/*
=============
pfnGetPhysicsKeyValue

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPhysicsKeyValue(mut pClient: *const edict_t,
                                               mut key: *const libc::c_char)
 -> *const libc::c_char {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    // pfnUserInfoChanged passed
    cl = SV_ClientFromEdict(pClient, false_0);
    if cl.is_null() {
        Con_Printf(b"^1Error:^7 GetPhysicsKeyValue: tried to a non-client!\n\x00"
                       as *const u8 as *const libc::c_char);
        return b"\x00" as *const u8 as *const libc::c_char
    }
    return Info_ValueForKey((*cl).physinfo.as_mut_ptr(), key);
}
/*
=============
pfnSetPhysicsKeyValue

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetPhysicsKeyValue(mut pClient: *const edict_t,
                                               mut key: *const libc::c_char,
                                               mut value:
                                                   *const libc::c_char) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    // pfnUserInfoChanged passed
    cl = SV_ClientFromEdict(pClient, false_0);
    if cl.is_null() {
        Con_Printf(b"^1Error:^7 SetPhysicsKeyValue: tried to a non-client!\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    Info_SetValueForKey((*cl).physinfo.as_mut_ptr(), key, value,
                        256 as libc::c_int);
}
/*
=============
pfnGetPhysicsInfoString

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPhysicsInfoString(mut pClient: *const edict_t)
 -> *const libc::c_char {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    // pfnUserInfoChanged passed
    cl = SV_ClientFromEdict(pClient, false_0);
    if cl.is_null() {
        Con_Printf(b"^1Error:^7 GetPhysicsInfoString: tried to a non-client!\n\x00"
                       as *const u8 as *const libc::c_char);
        return b"\x00" as *const u8 as *const libc::c_char
    }
    return (*cl).physinfo.as_mut_ptr();
}
/*
=============
pfnPrecacheEvent

register or returns already registered event id
a type of event is ignored at this moment
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPrecacheEvent(mut type_0: libc::c_int,
                                          mut psz: *const libc::c_char)
 -> word {
    return SV_EventIndex(psz) as word;
}
/*
=============
pfnPlaybackEvent

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PlaybackEventFull(mut flags: libc::c_int,
                                              mut pInvoker: *const edict_t,
                                              mut eventindex: word,
                                              mut delay: libc::c_float,
                                              mut origin: *mut libc::c_float,
                                              mut angles: *mut libc::c_float,
                                              mut fparam1: libc::c_float,
                                              mut fparam2: libc::c_float,
                                              mut iparam1: libc::c_int,
                                              mut iparam2: libc::c_int,
                                              mut bparam1: libc::c_int,
                                              mut bparam2: libc::c_int) {
    let mut cl: *mut sv_client_t =
        0 as *mut sv_client_t; // someone stupid joke
    let mut es: *mut event_state_t = 0 as *mut event_state_t;
    let mut args: event_args_t =
        event_args_t{flags: 0,
                     entindex: 0,
                     origin: [0.; 3],
                     angles: [0.; 3],
                     velocity: [0.; 3],
                     ducking: 0,
                     fparam1: 0.,
                     fparam2: 0.,
                     iparam1: 0,
                     iparam2: 0,
                     bparam1: 0,
                     bparam2: 0,};
    let mut ei: *mut event_info_t = 0 as *mut event_info_t;
    let mut j: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut bestslot: libc::c_int = 0;
    let mut invokerIndex: libc::c_int = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    let mut pvspoint: vec3_t = [0.; 3];
    if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 { return }
    // first check event for out of bounds
    if (eventindex as libc::c_int) < 1 as libc::c_int ||
           eventindex as libc::c_int > (1 as libc::c_int) << 10 as libc::c_int
       {
        Con_Printf(b"^1Error:^7 EV_Playback: invalid eventindex %i\n\x00" as
                       *const u8 as *const libc::c_char,
                   eventindex as libc::c_int);
        return
    }
    // check event for precached
    if if sv.event_precache[eventindex as usize].as_mut_ptr().is_null() ||
              *sv.event_precache[eventindex as usize].as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Printf(b"^1Error:^7 EV_Playback: event %i was not precached\n\x00"
                       as *const u8 as *const libc::c_char,
                   eventindex as libc::c_int);
        return
    }
    memset(&mut args as *mut event_args_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<event_args_t>() as libc::c_ulong);
    if !origin.is_null() &&
           !(*origin.offset(0 as libc::c_int as isize) == 0.0f32 &&
                 *origin.offset(1 as libc::c_int as isize) == 0.0f32 &&
                 *origin.offset(2 as libc::c_int as isize) == 0.0f32) {
        args.origin[0 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        args.origin[1 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        args.origin[2 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize);
        args.flags |= (1 as libc::c_int) << 0 as libc::c_int
    }
    if !angles.is_null() &&
           !(*angles.offset(0 as libc::c_int as isize) == 0.0f32 &&
                 *angles.offset(1 as libc::c_int as isize) == 0.0f32 &&
                 *angles.offset(2 as libc::c_int as isize) == 0.0f32) {
        args.angles[0 as libc::c_int as usize] =
            *angles.offset(0 as libc::c_int as isize);
        args.angles[1 as libc::c_int as usize] =
            *angles.offset(1 as libc::c_int as isize);
        args.angles[2 as libc::c_int as usize] =
            *angles.offset(2 as libc::c_int as isize);
        args.flags |= (1 as libc::c_int) << 1 as libc::c_int
    }
    // copy other parms
    args.fparam1 = fparam1;
    args.fparam2 = fparam2;
    args.iparam1 = iparam1;
    args.iparam2 = iparam2;
    args.bparam1 = bparam1;
    args.bparam2 = bparam2;
    pvspoint[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    pvspoint[1 as libc::c_int as usize] = pvspoint[2 as libc::c_int as usize];
    pvspoint[0 as libc::c_int as usize] = pvspoint[1 as libc::c_int as usize];
    if SV_CheckEdict(pInvoker,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 3994 as libc::c_int) as u64 != 0
       {
        // add the view_ofs to avoid problems with crossed contents line
        pvspoint[0 as libc::c_int as usize] =
            (*pInvoker).v.origin[0 as libc::c_int as usize] +
                (*pInvoker).v.view_ofs[0 as libc::c_int as usize];
        pvspoint[1 as libc::c_int as usize] =
            (*pInvoker).v.origin[1 as libc::c_int as usize] +
                (*pInvoker).v.view_ofs[1 as libc::c_int as usize];
        pvspoint[2 as libc::c_int as usize] =
            (*pInvoker).v.origin[2 as libc::c_int as usize] +
                (*pInvoker).v.view_ofs[2 as libc::c_int as usize];
        invokerIndex =
            (pInvoker as *mut edict_t).wrapping_offset_from(svgame.edicts) as
                libc::c_long as libc::c_int;
        args.entindex = invokerIndex;
        // g-cont. allow 'ducking' param for all entities
        args.ducking =
            if (*pInvoker).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 14 as libc::c_int != 0 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int };
        // this will be send only for reliable event
        if args.flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
            args.origin[0 as libc::c_int as usize] =
                (*pInvoker).v.origin[0 as libc::c_int as usize];
            args.origin[1 as libc::c_int as usize] =
                (*pInvoker).v.origin[1 as libc::c_int as usize];
            args.origin[2 as libc::c_int as usize] =
                (*pInvoker).v.origin[2 as libc::c_int as usize]
        }
        // this will be send only for reliable event
        if args.flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            args.angles[0 as libc::c_int as usize] =
                (*pInvoker).v.angles[0 as libc::c_int as usize];
            args.angles[1 as libc::c_int as usize] =
                (*pInvoker).v.angles[1 as libc::c_int as usize];
            args.angles[2 as libc::c_int as usize] =
                (*pInvoker).v.angles[2 as libc::c_int as usize]
        }
    } else {
        pvspoint[0 as libc::c_int as usize] =
            args.origin[0 as libc::c_int as usize];
        pvspoint[1 as libc::c_int as usize] =
            args.origin[1 as libc::c_int as usize];
        pvspoint[2 as libc::c_int as usize] =
            args.origin[2 as libc::c_int as usize];
        args.entindex = 0 as libc::c_int;
        invokerIndex = -(1 as libc::c_int)
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 &&
           (pvspoint[0 as libc::c_int as usize] == 0.0f32 &&
                pvspoint[1 as libc::c_int as usize] == 0.0f32 &&
                pvspoint[2 as libc::c_int as usize] == 0.0f32) {
        Con_DPrintf(b"^1Error:^7 %s: not a FEV_GLOBAL event missing origin. Ignored.\n\x00"
                        as *const u8 as *const libc::c_char,
                    sv.event_precache[eventindex as usize].as_mut_ptr());
        return
    }
    // check event for some user errors
    if flags &
           ((1 as libc::c_int) << 0 as libc::c_int |
                (1 as libc::c_int) << 4 as libc::c_int) != 0 {
        if SV_ClientFromEdict(pInvoker, true_0).is_null() {
            let mut ev_name: *const libc::c_char =
                sv.event_precache[eventindex as
                                      usize].as_mut_ptr(); // it's a server event!
            if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                Con_DPrintf(b"^3Warning:^7 %s: specified FEV_NOTHOST when invoker not a client\n\x00"
                                as *const u8 as *const libc::c_char,
                            ev_name); // fixup negative delays
                flags = flags & !((1 as libc::c_int) << 0 as libc::c_int)
            }
            if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                Con_DPrintf(b"^3Warning:^7 %s: specified FEV_HOSTONLY when invoker not a client\n\x00"
                                as *const u8 as *const libc::c_char, ev_name);
                flags = flags & !((1 as libc::c_int) << 4 as libc::c_int)
            }
        }
    }
    flags = flags | (1 as libc::c_int) << 5 as libc::c_int;
    if delay < 0.0f32 { delay = 0.0f32 }
    // setup pvs cluster for invoker
    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        Mod_FatPVS(pvspoint.as_mut_ptr() as *const vec_t, 16.0f32,
                   fatphs.as_mut_ptr(), world.fatbytes as libc::c_int,
                   false_0,
                   (svs.maxclients == 1 as libc::c_int) as libc::c_int as
                       qboolean);
        mask = fatphs.as_mut_ptr()
        // using the FatPVS like a PHS
    }
    let mut current_block_83: u64;
    // process all the clients
    slot = 0 as libc::c_int; // will be played on client side
    cl = svs.clients; // sending only to invoker
    while slot < svs.maxclients {
        if !((*cl).state as libc::c_uint !=
                 cs_spawned as libc::c_int as libc::c_uint ||
                 (*cl).edict.is_null() ||
                 (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0) {
            if SV_CheckEdict(pInvoker,
                             b"../engine/server/sv_game.c\x00" as *const u8 as
                                 *const libc::c_char, 4061 as libc::c_int) as
                   libc::c_uint != 0 && (*pInvoker).v.groupinfo != 0 &&
                   (*(*cl).edict).v.groupinfo != 0 {
                if svs.groupop == 0 as libc::c_int &&
                       (*(*cl).edict).v.groupinfo & (*pInvoker).v.groupinfo ==
                           0 {
                    current_block_83 = 11441799814184323368;
                } else if svs.groupop == 1 as libc::c_int &&
                              (*(*cl).edict).v.groupinfo &
                                  (*pInvoker).v.groupinfo != 0 {
                    current_block_83 = 11441799814184323368;
                } else { current_block_83 = 7494008139977416618; }
            } else { current_block_83 = 7494008139977416618; }
            match current_block_83 {
                11441799814184323368 => { }
                _ => {
                    if SV_CheckEdict(pInvoker,
                                     b"../engine/server/sv_game.c\x00" as
                                         *const u8 as *const libc::c_char,
                                     4070 as libc::c_int) as u64 != 0 {
                        if SV_CheckClientVisiblity(cl, mask) as u64 == 0 {
                            current_block_83 = 11441799814184323368;
                        } else { current_block_83 = 4216521074440650966; }
                    } else { current_block_83 = 4216521074440650966; }
                    match current_block_83 {
                        11441799814184323368 => { }
                        _ => {
                            if !(flags &
                                     (1 as libc::c_int) << 0 as libc::c_int !=
                                     0 && cl == sv.current_client &&
                                     (*cl).flags &
                                         (1 as libc::c_uint) <<
                                             5 as libc::c_int != 0) {
                                if !(flags &
                                         (1 as libc::c_int) <<
                                             4 as libc::c_int != 0 &&
                                         (*cl).edict !=
                                             pInvoker as *mut edict_t) {
                                    // all checks passed, send the event
                                    // reliable event
                                    if flags &
                                           (1 as libc::c_int) <<
                                               1 as libc::c_int != 0 {
                                        // skipping queue, write direct into reliable datagram
                                        SV_PlaybackReliableEvent(&mut (*cl).netchan.message,
                                                                 eventindex,
                                                                 delay,
                                                                 &mut args);
                                    } else {
                                        // unreliable event (stores in queue)
                                        es = &mut (*cl).events;
                                        bestslot = -(1 as libc::c_int);
                                        if flags &
                                               (1 as libc::c_int) <<
                                                   3 as libc::c_int != 0 {
                                            j = 0 as libc::c_int;
                                            while j < 64 as libc::c_int {
                                                ei =
                                                    &mut *(*es).ei.as_mut_ptr().offset(j
                                                                                           as
                                                                                           isize)
                                                        as *mut event_info_t;
                                                if (*ei).index as libc::c_int
                                                       ==
                                                       eventindex as
                                                           libc::c_int &&
                                                       invokerIndex !=
                                                           -(1 as libc::c_int)
                                                       &&
                                                       invokerIndex ==
                                                           (*ei).entity_index
                                                               as libc::c_int
                                                   {
                                                    bestslot = j;
                                                    break ;
                                                } else { j += 1 }
                                            }
                                        }
                                        if bestslot == -(1 as libc::c_int) {
                                            j = 0 as libc::c_int;
                                            while j < 64 as libc::c_int {
                                                ei =
                                                    &mut *(*es).ei.as_mut_ptr().offset(j
                                                                                           as
                                                                                           isize)
                                                        as *mut event_info_t;
                                                if (*ei).index as libc::c_int
                                                       == 0 as libc::c_int {
                                                    // found an empty slot
                                                    bestslot = j;
                                                    break ;
                                                } else { j += 1 }
                                            }
                                        }
                                        // no slot found for this player, oh well
                                        if !(bestslot == -(1 as libc::c_int))
                                           {
                                            // add event to queue
                                            (*ei).index = eventindex;
                                            (*ei).fire_time = delay;
                                            (*ei).entity_index =
                                                invokerIndex as libc::c_short;
                                            (*ei).packet_index =
                                                -(1 as libc::c_int) as
                                                    libc::c_short;
                                            (*ei).flags = flags;
                                            (*ei).args = args
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        slot += 1;
        cl = cl.offset(1)
    };
}
/*
=============
pfnSetFatPVS

The client will interpolate the view position,
so we can't use a single PVS point
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetFatPVS(mut org: *const libc::c_float)
 -> *mut byte {
    let mut fullvis: qboolean = false_0;
    if (*sv.worldmodel).visdata.is_null() || (*sv_novis).value != 0. ||
           org.is_null() || CL_DisableVisibility() as libc::c_uint != 0 {
        fullvis = true_0
    }
    if !(pfnGetCurrentPlayer() != -(1 as libc::c_int)) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 4153 as libc::c_int);
    }
    // portals can't change viewpoint!
    if sv.hostflags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           == 0 {
        let mut viewPos: vec3_t = [0.; 3];
        let mut offset: vec3_t = [0.; 3];
        // see code from client.cpp for understanding:
		// org = pView->v.origin + pView->v.view_ofs;
		// if ( pView->v.flags & FL_DUCKING )
		// {
		//	org = org + ( VEC_HULL_MIN - VEC_DUCK_HULL_MIN );
		// }
		// so we have unneeded duck calculations who have affect when player
		// is ducked into water. Remove offset to restore right PVS position
        if (*(*sv.current_client).edict).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 14 as libc::c_int != 0 {
            offset[0 as libc::c_int as usize] =
                (*svgame.pmove).player_mins[0 as libc::c_int as
                                                usize][0 as libc::c_int as
                                                           usize] -
                    (*svgame.pmove).player_mins[1 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize];
            offset[1 as libc::c_int as usize] =
                (*svgame.pmove).player_mins[0 as libc::c_int as
                                                usize][1 as libc::c_int as
                                                           usize] -
                    (*svgame.pmove).player_mins[1 as libc::c_int as
                                                    usize][1 as libc::c_int as
                                                               usize];
            offset[2 as libc::c_int as usize] =
                (*svgame.pmove).player_mins[0 as libc::c_int as
                                                usize][2 as libc::c_int as
                                                           usize] -
                    (*svgame.pmove).player_mins[1 as libc::c_int as
                                                    usize][2 as libc::c_int as
                                                               usize];
            viewPos[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) -
                    offset[0 as libc::c_int as usize];
            viewPos[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) -
                    offset[1 as libc::c_int as usize];
            viewPos[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) -
                    offset[2 as libc::c_int as usize]
        } else {
            viewPos[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize);
            viewPos[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize);
            viewPos[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize)
        }
        // build a new PVS frame
        Mod_FatPVS(viewPos.as_mut_ptr() as *const vec_t, 8.0f32,
                   fatpvs.as_mut_ptr(), world.fatbytes as libc::c_int,
                   false_0, fullvis);
        viewPoint[pfnGetCurrentPlayer() as usize][0 as libc::c_int as usize] =
            viewPos[0 as libc::c_int as usize];
        viewPoint[pfnGetCurrentPlayer() as usize][1 as libc::c_int as usize] =
            viewPos[1 as libc::c_int as usize];
        viewPoint[pfnGetCurrentPlayer() as usize][2 as libc::c_int as usize] =
            viewPos[2 as libc::c_int as usize]
    } else {
        // merge PVS
        Mod_FatPVS(org, 8.0f32, fatpvs.as_mut_ptr(),
                   world.fatbytes as libc::c_int, true_0, fullvis);
    }
    return fatpvs.as_mut_ptr();
}
/*
=============
pfnSetFatPHS

The client will interpolate the hear position,
so we can't use a single PHS point
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetFatPAS(mut org: *const libc::c_float)
 -> *mut byte {
    let mut fullvis: qboolean = false_0;
    if (*sv.worldmodel).visdata.is_null() || (*sv_novis).value != 0. ||
           org.is_null() || CL_DisableVisibility() as libc::c_uint != 0 {
        fullvis = true_0
    }
    if !(pfnGetCurrentPlayer() != -(1 as libc::c_int)) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 4203 as libc::c_int);
    }
    // portals can't change viewpoint!
    if sv.hostflags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           == 0 {
        let mut viewPos: vec3_t = [0.; 3];
        let mut offset: vec3_t = [0.; 3];
        // see code from client.cpp for understanding:
		// org = pView->v.origin + pView->v.view_ofs;
		// if ( pView->v.flags & FL_DUCKING )
		// {
		//	org = org + ( VEC_HULL_MIN - VEC_DUCK_HULL_MIN );
		// }
		// so we have unneeded duck calculations who have affect when player
		// is ducked into water. Remove offset to restore right PVS position
        if (*(*sv.current_client).edict).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 14 as libc::c_int != 0 {
            offset[0 as libc::c_int as usize] =
                (*svgame.pmove).player_mins[0 as libc::c_int as
                                                usize][0 as libc::c_int as
                                                           usize] -
                    (*svgame.pmove).player_mins[1 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize];
            offset[1 as libc::c_int as usize] =
                (*svgame.pmove).player_mins[0 as libc::c_int as
                                                usize][1 as libc::c_int as
                                                           usize] -
                    (*svgame.pmove).player_mins[1 as libc::c_int as
                                                    usize][1 as libc::c_int as
                                                               usize];
            offset[2 as libc::c_int as usize] =
                (*svgame.pmove).player_mins[0 as libc::c_int as
                                                usize][2 as libc::c_int as
                                                           usize] -
                    (*svgame.pmove).player_mins[1 as libc::c_int as
                                                    usize][2 as libc::c_int as
                                                               usize];
            viewPos[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) -
                    offset[0 as libc::c_int as usize];
            viewPos[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) -
                    offset[1 as libc::c_int as usize];
            viewPos[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) -
                    offset[2 as libc::c_int as usize]
        } else {
            viewPos[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize);
            viewPos[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize);
            viewPos[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize)
        }
        // build a new PHS frame
        Mod_FatPVS(viewPos.as_mut_ptr() as *const vec_t, 16.0f32,
                   fatphs.as_mut_ptr(), world.fatbytes as libc::c_int,
                   false_0, fullvis);
    } else {
        // merge PHS
        Mod_FatPVS(org, 16.0f32, fatphs.as_mut_ptr(),
                   world.fatbytes as libc::c_int, true_0, fullvis);
    }
    return fatphs.as_mut_ptr();
}
/*
=============
pfnCheckVisibility

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCheckVisibility(mut ent: *const edict_t,
                                            mut pset: *mut byte)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut leafnum: libc::c_int = 0;
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 4247 as libc::c_int) as u64 == 0
       {
        return 0 as libc::c_int
    }
    // vis not set - fullvis enabled
    if pset.is_null() { return 1 as libc::c_int } // upcast beams to my owner
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 29 as libc::c_int != 0 &&
           !(*ent).v.owner.is_null() &&
           (*(*ent).v.owner).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        ent = (*ent).v.owner
    }
    if (*ent).headnode < 0 as libc::c_int {
        // check individual leafs
        i = 0 as libc::c_int;
        while i < (*ent).num_leafs {
            if if (*ent).leafnums[i as usize] as libc::c_int >=
                      0 as libc::c_int {
                   (*pset.offset(((*ent).leafnums[i as usize] as libc::c_int
                                      >> 3 as libc::c_int) as isize) as
                        libc::c_int &
                        (1 as libc::c_int) <<
                            ((*ent).leafnums[i as usize] as libc::c_int &
                                 7 as libc::c_int)) as byte as libc::c_int
               } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
                return 1 as libc::c_int
            }
            i += 1
            // visible passed by leaf
        }
        return 0 as libc::c_int
    } else {
        i = 0 as libc::c_int;
        while i < 48 as libc::c_int {
            leafnum = (*ent).leafnums[i as usize] as libc::c_int;
            if leafnum == -(1 as libc::c_int) { break ; }
            if if leafnum >= 0 as libc::c_int {
                   (*pset.offset((leafnum >> 3 as libc::c_int) as isize) as
                        libc::c_int &
                        (1 as libc::c_int) << (leafnum & 7 as libc::c_int)) as
                       byte as libc::c_int
               } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
                return 1 as libc::c_int
            }
            i += 1
            // visible passed by leaf
        }
        // visible passed by headnode
        if Mod_HeadnodeVisible(&mut *(*sv.worldmodel).nodes.offset((*ent).headnode
                                                                       as
                                                                       isize),
                               pset, &mut leafnum) as u64 == 0 {
            return 0 as libc::c_int
        }
        (*(ent as *mut edict_t)).leafnums[(*ent).num_leafs as usize] =
            leafnum as libc::c_short;
        (*(ent as *mut edict_t)).num_leafs =
            ((*ent).num_leafs + 1 as libc::c_int) % 48 as libc::c_int;
        return 2 as libc::c_int
    };
}
// too many leafs for individual check, go by headnode
/*
=============
pfnCanSkipPlayer

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCanSkipPlayer(mut player: *const edict_t)
 -> libc::c_int {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    cl = SV_ClientFromEdict(player, false_0);
    if cl.is_null() { return false_0 as libc::c_int }
    return if (*cl).flags & (1 as libc::c_uint) << 5 as libc::c_int != 0 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int };
}
/*
=============
pfnGetCurrentPlayer

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetCurrentPlayer() -> libc::c_int {
    let mut idx: libc::c_int =
        sv.current_client.wrapping_offset_from(svs.clients) as libc::c_long as
            libc::c_int;
    if idx < 0 as libc::c_int || idx >= svs.maxclients {
        return -(1 as libc::c_int)
    }
    return idx;
}
/*
=============
pfnSetGroupMask

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetGroupMask(mut mask: libc::c_int,
                                         mut op: libc::c_int) {
    svs.groupmask = mask;
    svs.groupop = op;
}
/*
=============
pfnCreateInstancedBaseline

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCreateInstancedBaseline(mut classname:
                                                        libc::c_int,
                                                    mut baseline:
                                                        *mut entity_state_s)
 -> libc::c_int {
    if baseline.is_null() || sv.num_instanced >= 64 as libc::c_int {
        return 0 as libc::c_int
    }
    // g-cont. must sure that classname is really allocated
    sv.instanced[sv.num_instanced as usize].classname =
        _copystring(svgame.stringspool, SV_GetString(classname),
                    b"../engine/server/sv_game.c\x00" as *const u8 as
                        *const libc::c_char, 4344 as libc::c_int);
    sv.instanced[sv.num_instanced as usize].baseline = *baseline;
    sv.num_instanced += 1;
    return sv.num_instanced;
}
/*
=============
pfnEndSection

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnEndSection(mut pszSection: *const libc::c_char) {
    if Q_strnicmp(b"oem_end_credits\x00" as *const u8 as *const libc::c_char,
                  pszSection, 99999 as libc::c_int) == 0 {
        Host_Credits();
    } else {
        Cbuf_AddText(b"\ndisconnect\n\x00" as *const u8 as
                         *const libc::c_char);
    };
}
/*
=============
pfnGetPlayerUserId

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPlayerUserId(mut e: *mut edict_t)
 -> libc::c_int {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    cl = SV_ClientFromEdict(e, false_0);
    if cl.is_null() { return -(1 as libc::c_int) }
    return (*cl).userid;
}
/*
=============
pfnGetPlayerStats

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPlayerStats(mut pClient: *const edict_t,
                                           mut ping: *mut libc::c_int,
                                           mut packet_loss:
                                               *mut libc::c_int) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if !packet_loss.is_null() { *packet_loss = 0 as libc::c_int }
    if !ping.is_null() { *ping = 0 as libc::c_int }
    cl = SV_ClientFromEdict(pClient, false_0);
    if cl.is_null() { return }
    if !packet_loss.is_null() { *packet_loss = (*cl).packet_loss }
    if !ping.is_null() {
        *ping =
            ((*cl).latency * 1000 as libc::c_int as libc::c_float) as
                libc::c_int
    };
}
/*
=============
pfnForceUnmodified

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnForceUnmodified(mut type_0: FORCE_TYPE,
                                            mut mins: *mut libc::c_float,
                                            mut maxs: *mut libc::c_float,
                                            mut filename:
                                                *const libc::c_char) {
    let mut pc: *mut consistency_t = 0 as *mut consistency_t;
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    if sv.state as libc::c_uint == ss_loading as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 1024 as libc::c_int {
            pc =
                &mut *sv.consistency_list.as_mut_ptr().offset(i as isize) as
                    *mut consistency_t;
            if (*pc).filename.is_null() {
                if !mins.is_null() {
                    (*pc).mins[0 as libc::c_int as usize] =
                        *mins.offset(0 as libc::c_int as isize);
                    (*pc).mins[1 as libc::c_int as usize] =
                        *mins.offset(1 as libc::c_int as isize);
                    (*pc).mins[2 as libc::c_int as usize] =
                        *mins.offset(2 as libc::c_int as isize)
                }
                if !maxs.is_null() {
                    (*pc).maxs[0 as libc::c_int as usize] =
                        *maxs.offset(0 as libc::c_int as isize);
                    (*pc).maxs[1 as libc::c_int as usize] =
                        *maxs.offset(1 as libc::c_int as isize);
                    (*pc).maxs[2 as libc::c_int as usize] =
                        *maxs.offset(2 as libc::c_int as isize)
                }
                (*pc).filename =
                    _copystring(svgame.stringspool, filename,
                                b"../engine/server/sv_game.c\x00" as *const u8
                                    as *const libc::c_char,
                                4423 as libc::c_int);
                (*pc).check_type = type_0 as libc::c_int;
                return
            } else {
                if Q_strncmp(filename, (*pc).filename, 99999 as libc::c_int)
                       == 0 {
                    return
                }
            }
            i += 1
        }
        Host_Error(b"MAX_MODELS limit exceeded (%d)\n\x00" as *const u8 as
                       *const libc::c_char, 1024 as libc::c_int);
    } else {
        i = 0 as libc::c_int;
        while i < 1024 as libc::c_int {
            pc =
                &mut *sv.consistency_list.as_mut_ptr().offset(i as isize) as
                    *mut consistency_t;
            if !(*pc).filename.is_null() {
                if Q_strncmp(filename, (*pc).filename, 99999 as libc::c_int)
                       == 0 {
                    return
                }
            }
            i += 1
        }
        Con_Printf(b"^1Error:^7 Failed to enforce consistency for %s: was not precached\n\x00"
                       as *const u8 as *const libc::c_char, filename);
    };
}
/*
=============
pfnVoice_GetClientListening

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnVoice_GetClientListening(mut iReceiver:
                                                         libc::c_int,
                                                     mut iSender: libc::c_int)
 -> qboolean {
    iReceiver -= 1 as libc::c_int;
    iSender -= 1 as libc::c_int;
    if iReceiver < 0 as libc::c_int || iReceiver >= svs.maxclients ||
           iSender < 0 as libc::c_int || iSender > svs.maxclients {
        return false_0
    }
    return ((*svs.clients.offset(iSender as isize)).listeners &
                (1 as libc::c_uint) << iReceiver !=
                0 as libc::c_int as libc::c_uint) as libc::c_int as qboolean;
}
/*
=============
pfnVoice_SetClientListening

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnVoice_SetClientListening(mut iReceiver:
                                                         libc::c_int,
                                                     mut iSender: libc::c_int,
                                                     mut bListen: qboolean)
 -> qboolean {
    iReceiver -= 1 as libc::c_int;
    iSender -= 1 as libc::c_int;
    if iReceiver < 0 as libc::c_int || iReceiver >= svs.maxclients ||
           iSender < 0 as libc::c_int || iSender > svs.maxclients {
        return false_0
    }
    if bListen as u64 != 0 {
        (*svs.clients.offset(iSender as isize)).listeners =
            (*svs.clients.offset(iSender as isize)).listeners |
                (1 as libc::c_uint) << iReceiver
    } else {
        (*svs.clients.offset(iSender as isize)).listeners =
            (*svs.clients.offset(iSender as isize)).listeners &
                !((1 as libc::c_uint) << iReceiver)
    }
    return true_0;
}
/*
=============
pfnGetPlayerAuthId

These function must returns cd-key hashed value
but Xash3D currently doesn't have any security checks
return nullstring for now
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPlayerAuthId(mut e: *mut edict_t)
 -> *const libc::c_char {
    return SV_GetClientIDString(SV_ClientFromEdict(e, false_0));
}
/*
=============
pfnQueryClientCvarValue

request client cvar value
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnQueryClientCvarValue(mut player: *const edict_t,
                                                 mut cvarName:
                                                     *const libc::c_char) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if if cvarName.is_null() || *cvarName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    cl = SV_ClientFromEdict(player, true_0);
    if !cl.is_null() {
        MSG_WriteCmdExt(&mut (*cl).netchan.message, 57 as libc::c_int,
                        NS_SERVER, 0 as *const libc::c_char);
        MSG_WriteString(&mut (*cl).netchan.message, cvarName);
    } else {
        if svgame.dllFuncs2.pfnCvarValue.is_some() {
            svgame.dllFuncs2.pfnCvarValue.expect("non-null function pointer")(player,
                                                                              b"Bad Player\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char);
        }
        Con_Printf(b"^1Error:^7 QueryClientCvarValue: tried to send to a non-client!\n\x00"
                       as *const u8 as *const libc::c_char);
    };
}
/*
=============
pfnQueryClientCvarValue2

request client cvar value (bugfixed)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnQueryClientCvarValue2(mut player: *const edict_t,
                                                  mut cvarName:
                                                      *const libc::c_char,
                                                  mut requestID:
                                                      libc::c_int) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if if cvarName.is_null() || *cvarName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    cl = SV_ClientFromEdict(player, true_0);
    if !cl.is_null() {
        MSG_WriteCmdExt(&mut (*cl).netchan.message, 58 as libc::c_int,
                        NS_SERVER, 0 as *const libc::c_char);
        MSG_WriteLong(&mut (*cl).netchan.message, requestID);
        MSG_WriteString(&mut (*cl).netchan.message, cvarName);
    } else {
        if svgame.dllFuncs2.pfnCvarValue2.is_some() {
            svgame.dllFuncs2.pfnCvarValue2.expect("non-null function pointer")(player,
                                                                               requestID,
                                                                               cvarName,
                                                                               b"Bad Player\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char);
        }
        Con_Printf(b"^1Error:^7 QueryClientCvarValue: tried to send to a non-client!\n\x00"
                       as *const u8 as *const libc::c_char);
    };
}
/*
=============
pfnEngineStub

extended iface stubs
=============
*/
unsafe extern "C" fn pfnGetLocalizedStringLength(mut label:
                                                     *const libc::c_char)
 -> libc::c_int {
    return 0 as libc::c_int;
}
// engine callbacks
static mut gEngfuncs: enginefuncs_t =
    unsafe {
        {
            let mut init =
                enginefuncs_s{pfnPrecacheModel:
                                  Some(pfnPrecacheModel as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnPrecacheSound:
                                  Some(SV_SoundIndex as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnSetModel:
                                  Some(pfnSetModel as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnModelIndex:
                                  Some(pfnModelIndex as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnModelFrames:
                                  Some(pfnModelFrames as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              pfnSetSize:
                                  Some(pfnSetSize as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float)
                                               -> ()),
                              pfnChangeLevel:
                                  Some(pfnChangeLevel as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnGetSpawnParms:
                                  Some(pfnGetSpawnParms as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnSaveSpawnParms:
                                  Some(pfnSaveSpawnParms as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnVecToYaw:
                                  Some(pfnVecToYaw as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float)
                                               -> libc::c_float),
                              pfnVecToAngles:
                                  Some(VectorAngles as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *mut libc::c_float)
                                               -> ()),
                              pfnMoveToOrigin:
                                  Some(pfnMoveToOrigin as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnChangeYaw:
                                  Some(pfnChangeYaw as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnChangePitch:
                                  Some(pfnChangePitch as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnFindEntityByString:
                                  Some(SV_FindEntityByString as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> *mut edict_t),
                              pfnGetEntityIllum:
                                  Some(pfnGetEntityIllum as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> libc::c_int),
                              pfnFindEntityInSphere:
                                  Some(pfnFindEntityInSphere as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_float)
                                               -> *mut edict_t),
                              pfnFindClientInPVS:
                                  Some(pfnFindClientInPVS as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *mut edict_t),
                              pfnEntitiesInPVS:
                                  Some(pfnEntitiesInPVS as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *mut edict_t),
                              pfnMakeVectors:
                                  Some(pfnMakeVectors as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float)
                                               -> ()),
                              pfnAngleVectors:
                                  Some(AngleVectors as
                                           unsafe extern "C" fn(_:
                                                                    *const vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t)
                                               -> ()),
                              pfnCreateEntity:
                                  Some(SV_AllocEdict as
                                           unsafe extern "C" fn()
                                               -> *mut edict_t),
                              pfnRemoveEntity:
                                  Some(pfnRemoveEntity as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnCreateNamedEntity:
                                  Some(pfnCreateNamedEntity as
                                           unsafe extern "C" fn(_: string_t)
                                               -> *mut edict_t),
                              pfnMakeStatic:
                                  Some(pfnMakeStatic as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnEntIsOnFloor:
                                  Some(pfnEntIsOnFloor as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> libc::c_int),
                              pfnDropToFloor:
                                  Some(pfnDropToFloor as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> libc::c_int),
                              pfnWalkMove:
                                  Some(pfnWalkMove as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              pfnSetOrigin:
                                  Some(pfnSetOrigin as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float)
                                               -> ()),
                              pfnEmitSound:
                                  Some(SV_StartSound as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnEmitAmbientSound:
                                  Some(pfnEmitAmbientSound as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnTraceLine:
                                  Some(pfnTraceLine as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut TraceResult)
                                               -> ()),
                              pfnTraceToss:
                                  Some(pfnTraceToss as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut TraceResult)
                                               -> ()),
                              pfnTraceMonsterHull:
                                  Some(pfnTraceMonsterHull as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut TraceResult)
                                               -> libc::c_int),
                              pfnTraceHull:
                                  Some(pfnTraceHull as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut TraceResult)
                                               -> ()),
                              pfnTraceModel:
                                  Some(pfnTraceModel as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut TraceResult)
                                               -> ()),
                              pfnTraceTexture:
                                  Some(pfnTraceTexture as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float)
                                               -> *const libc::c_char),
                              pfnTraceSphere:
                                  Some(pfnTraceSphere as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut TraceResult)
                                               -> ()),
                              pfnGetAimVector:
                                  Some(pfnGetAimVector as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    *mut libc::c_float)
                                               -> ()),
                              pfnServerCommand:
                                  Some(pfnServerCommand as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnServerExecute:
                                  Some(pfnServerExecute as
                                           unsafe extern "C" fn() -> ()),
                              pfnClientCommand:
                                  Some(pfnClientCommand as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: ...)
                                               -> ()),
                              pfnParticleEffect:
                                  Some(pfnParticleEffect as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float)
                                               -> ()),
                              pfnLightStyle:
                                  Some(pfnLightStyle as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnDecalIndex:
                                  Some(pfnDecalIndex as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnPointContents:
                                  Some(SV_PointContents as
                                           unsafe extern "C" fn(_:
                                                                    *const vec_t)
                                               -> libc::c_int),
                              pfnMessageBegin:
                                  Some(pfnMessageBegin as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnMessageEnd:
                                  Some(pfnMessageEnd as
                                           unsafe extern "C" fn() -> ()),
                              pfnWriteByte:
                                  Some(pfnWriteByte as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()),
                              pfnWriteChar:
                                  Some(pfnWriteChar as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()),
                              pfnWriteShort:
                                  Some(pfnWriteShort as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()),
                              pfnWriteLong:
                                  Some(pfnWriteLong as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()),
                              pfnWriteAngle:
                                  Some(pfnWriteAngle as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_float)
                                               -> ()),
                              pfnWriteCoord:
                                  Some(pfnWriteCoord as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_float)
                                               -> ()),
                              pfnWriteString:
                                  Some(pfnWriteString as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnWriteEntity:
                                  Some(pfnWriteEntity as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()),
                              pfnCVarRegister:
                                  Some(pfnCvar_RegisterServerVariable as
                                           unsafe extern "C" fn(_:
                                                                    *mut cvar_t)
                                               -> ()),
                              pfnCVarGetFloat:
                                  Some(Cvar_VariableValue as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_float),
                              pfnCVarGetString:
                                  Some(Cvar_VariableString as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> *const libc::c_char),
                              pfnCVarSetFloat:
                                  Some(Cvar_SetValue as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_float)
                                               -> ()),
                              pfnCVarSetString:
                                  Some(Cvar_Set as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnAlertMessage:
                                  Some(pfnAlertMessage as
                                           unsafe extern "C" fn(_: ALERT_TYPE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: ...)
                                               -> ()),
                              pfnEngineFprintf:
                                  Some(pfnEngineFprintf as
                                           unsafe extern "C" fn(_: *mut FILE,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _: ...)
                                               -> ()),
                              pfnPvAllocEntPrivateData:
                                  Some(pfnPvAllocEntPrivateData as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    libc::c_long)
                                               -> *mut libc::c_void),
                              pfnPvEntPrivateData:
                                  Some(pfnPvEntPrivateData as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *mut libc::c_void),
                              pfnFreeEntPrivateData:
                                  Some(SV_FreePrivateData as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnSzFromIndex:
                                  Some(SV_GetString as
                                           unsafe extern "C" fn(_: string_t)
                                               -> *const libc::c_char),
                              pfnAllocString:
                                  Some(SV_AllocString as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> string_t),
                              pfnGetVarsOfEnt:
                                  Some(pfnGetVarsOfEnt as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *mut entvars_t),
                              pfnPEntityOfEntOffset:
                                  Some(pfnPEntityOfEntOffset as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> *mut edict_t),
                              pfnEntOffsetOfPEntity:
                                  Some(pfnEntOffsetOfPEntity as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t)
                                               -> libc::c_int),
                              pfnIndexOfEdict:
                                  Some(pfnIndexOfEdict as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t)
                                               -> libc::c_int),
                              pfnPEntityOfEntIndex:
                                  Some(pfnPEntityOfEntIndex as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> *mut edict_t),
                              pfnFindEntityByVars:
                                  Some(pfnFindEntityByVars as
                                           unsafe extern "C" fn(_:
                                                                    *mut entvars_t)
                                               -> *mut edict_t),
                              pfnGetModelPtr:
                                  Some(pfnGetModelPtr as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *mut libc::c_void),
                              pfnRegUserMsg:
                                  Some(pfnRegUserMsg as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              pfnAnimationAutomove:
                                  Some(pfnAnimationAutomove as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    libc::c_float)
                                               -> ()),
                              pfnGetBonePosition:
                                  Some(pfnGetBonePosition as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    *mut libc::c_float)
                                               -> ()),
                              pfnFunctionFromName:
                                  ::std::mem::transmute::<*mut libc::c_void,
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *const libc::c_char)
                                                                     ->
                                                                         libc::c_ulong>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                 *const libc::c_char)
                                                                                                                            ->
                                                                                                                                *mut libc::c_void>,
                                                                                                                 *mut libc::c_void>(Some(pfnFunctionFromName
                                                                                                                                             as
                                                                                                                                             unsafe extern "C" fn(_:
                                                                                                                                                                      *const libc::c_char)
                                                                                                                                                 ->
                                                                                                                                                     *mut libc::c_void))),
                              pfnNameForFunction:
                                  ::std::mem::transmute::<*mut libc::c_void,
                                                          Option<unsafe extern "C" fn(_:
                                                                                          libc::c_ulong)
                                                                     ->
                                                                         *const libc::c_char>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                       *mut libc::c_void)
                                                                                                                                  ->
                                                                                                                                      *const libc::c_char>,
                                                                                                                       *mut libc::c_void>(Some(pfnNameForFunction
                                                                                                                                                   as
                                                                                                                                                   unsafe extern "C" fn(_:
                                                                                                                                                                            *mut libc::c_void)
                                                                                                                                                       ->
                                                                                                                                                           *const libc::c_char))),
                              pfnClientPrintf:
                                  Some(pfnClientPrintf as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _: PRINT_TYPE,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnServerPrint:
                                  Some(pfnServerPrint as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnCmd_Args:
                                  Some(Cmd_Args as
                                           unsafe extern "C" fn()
                                               -> *const libc::c_char),
                              pfnCmd_Argv:
                                  Some(Cmd_Argv as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> *const libc::c_char),
                              pfnCmd_Argc:
                                  ::std::mem::transmute::<*mut libc::c_void,
                                                          Option<unsafe extern "C" fn()
                                                                     ->
                                                                         libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                          ->
                                                                                                                              libc::c_int>,
                                                                                                               *mut libc::c_void>(Some(Cmd_Argc
                                                                                                                                           as
                                                                                                                                           unsafe extern "C" fn()
                                                                                                                                               ->
                                                                                                                                                   libc::c_int))),
                              pfnGetAttachment:
                                  Some(pfnGetAttachment as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    *mut libc::c_float)
                                               -> ()),
                              pfnCRC32_Init:
                                  Some(CRC32_Init as
                                           unsafe extern "C" fn(_: *mut dword)
                                               -> ()),
                              pfnCRC32_ProcessBuffer:
                                  Some(CRC32_ProcessBuffer as
                                           unsafe extern "C" fn(_: *mut dword,
                                                                _:
                                                                    *const libc::c_void,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnCRC32_ProcessByte:
                                  Some(CRC32_ProcessByte as
                                           unsafe extern "C" fn(_: *mut dword,
                                                                _: byte)
                                               -> ()),
                              pfnCRC32_Final:
                                  Some(CRC32_Final as
                                           unsafe extern "C" fn(_: dword)
                                               -> dword),
                              pfnRandomLong:
                                  Some(COM_RandomLong as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              pfnRandomFloat:
                                  Some(COM_RandomFloat as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float)
                                               -> libc::c_float),
                              pfnSetView:
                                  Some(pfnSetView as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    *const edict_t)
                                               -> ()),
                              pfnTime:
                                  Some(pfnTime as
                                           unsafe extern "C" fn()
                                               -> libc::c_float),
                              pfnCrosshairAngle:
                                  Some(pfnCrosshairAngle as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float)
                                               -> ()),
                              pfnLoadFileForMe:
                                  Some(COM_LoadFileForMe as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> *mut byte),
                              pfnFreeFile:
                                  Some(COM_FreeFile as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                              pfnEndSection:
                                  Some(pfnEndSection as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
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
                              pfnGetGameDir:
                                  Some(pfnGetGameDir as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char)
                                               -> ()),
                              pfnCvar_RegisterVariable:
                                  Some(pfnCvar_RegisterEngineVariable as
                                           unsafe extern "C" fn(_:
                                                                    *mut cvar_t)
                                               -> ()),
                              pfnFadeClientVolume:
                                  Some(pfnFadeClientVolume as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnSetClientMaxspeed:
                                  Some(pfnSetClientMaxspeed as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    libc::c_float)
                                               -> ()),
                              pfnCreateFakeClient:
                                  Some(SV_FakeConnect as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> *mut edict_t),
                              pfnRunPlayerMove:
                                  Some(pfnRunPlayerMove as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _: word,
                                                                _: byte,
                                                                _: byte)
                                               -> ()),
                              pfnNumberOfEntities:
                                  Some(pfnNumberOfEntities as
                                           unsafe extern "C" fn()
                                               -> libc::c_int),
                              pfnGetInfoKeyBuffer:
                                  Some(pfnGetInfoKeyBuffer as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *mut libc::c_char),
                              pfnInfoKeyValue:
                                  Some(Info_ValueForKey as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> *const libc::c_char),
                              pfnSetKeyValue:
                                  Some(pfnSetValueForKey as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut libc::c_char)
                                               -> ()),
                              pfnSetClientKeyValue:
                                  Some(pfnSetClientKeyValue as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut libc::c_char)
                                               -> ()),
                              pfnIsMapValid:
                                  Some(pfnIsMapValid as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char)
                                               -> libc::c_int),
                              pfnStaticDecal:
                                  Some(pfnStaticDecal as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnPrecacheGeneric:
                                  Some(SV_GenericIndex as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnGetPlayerUserId:
                                  Some(pfnGetPlayerUserId as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> libc::c_int),
                              pfnBuildSoundMsg:
                                  Some(pfnBuildSoundMsg as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *mut edict_t)
                                               -> ()),
                              pfnIsDedicatedServer:
                                  Some(pfnIsDedicatedServer as
                                           unsafe extern "C" fn()
                                               -> libc::c_int),
                              pfnCVarGetPointer:
                                  Some(pfnCVarGetPointer as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> *mut cvar_t),
                              pfnGetPlayerWONId:
                                  Some(pfnGetPlayerWONId as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> uint),
                              pfnInfo_RemoveKey:
                                  ::std::mem::transmute::<*mut libc::c_void,
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut libc::c_char,
                                                                                      _:
                                                                                          *const libc::c_char)
                                                                     ->
                                                                         ()>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                      *mut libc::c_char,
                                                                                                                                  _:
                                                                                                                                      *const libc::c_char)
                                                                                                                 ->
                                                                                                                     qboolean>,
                                                                                                      *mut libc::c_void>(Some(Info_RemoveKey
                                                                                                                                  as
                                                                                                                                  unsafe extern "C" fn(_:
                                                                                                                                                           *mut libc::c_char,
                                                                                                                                                       _:
                                                                                                                                                           *const libc::c_char)
                                                                                                                                      ->
                                                                                                                                          qboolean))),
                              pfnGetPhysicsKeyValue:
                                  Some(pfnGetPhysicsKeyValue as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> *const libc::c_char),
                              pfnSetPhysicsKeyValue:
                                  Some(pfnSetPhysicsKeyValue as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnGetPhysicsInfoString:
                                  Some(pfnGetPhysicsInfoString as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t)
                                               -> *const libc::c_char),
                              pfnPrecacheEvent:
                                  Some(pfnPrecacheEvent as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *const libc::c_char)
                                               -> word),
                              pfnPlaybackEvent:
                                  Some(SV_PlaybackEventFull as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *const edict_t,
                                                                _: word,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnSetFatPVS:
                                  Some(pfnSetFatPVS as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float)
                                               -> *mut byte),
                              pfnSetFatPAS:
                                  Some(pfnSetFatPAS as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_float)
                                               -> *mut byte),
                              pfnCheckVisibility:
                                  Some(pfnCheckVisibility as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _: *mut byte)
                                               -> libc::c_int),
                              pfnDeltaSetField:
                                  Some(Delta_SetField as
                                           unsafe extern "C" fn(_:
                                                                    *mut delta_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnDeltaUnsetField:
                                  Some(Delta_UnsetField as
                                           unsafe extern "C" fn(_:
                                                                    *mut delta_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnDeltaAddEncoder:
                                  Some(Delta_AddEncoder as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    pfnDeltaEncode)
                                               -> ()),
                              pfnGetCurrentPlayer:
                                  Some(pfnGetCurrentPlayer as
                                           unsafe extern "C" fn()
                                               -> libc::c_int),
                              pfnCanSkipPlayer:
                                  Some(pfnCanSkipPlayer as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t)
                                               -> libc::c_int),
                              pfnDeltaFindField:
                                  Some(Delta_FindField as
                                           unsafe extern "C" fn(_:
                                                                    *mut delta_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnDeltaSetFieldByIndex:
                                  Some(Delta_SetFieldByIndex as
                                           unsafe extern "C" fn(_:
                                                                    *mut delta_t,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnDeltaUnsetFieldByIndex:
                                  Some(Delta_UnsetFieldByIndex as
                                           unsafe extern "C" fn(_:
                                                                    *mut delta_t,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnSetGroupMask:
                                  Some(pfnSetGroupMask as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnCreateInstancedBaseline:
                                  Some(pfnCreateInstancedBaseline as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut entity_state_s)
                                               -> libc::c_int),
                              pfnCvar_DirectSet:
                                  Some(pfnCVarDirectSet as
                                           unsafe extern "C" fn(_:
                                                                    *mut cvar_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnForceUnmodified:
                                  Some(pfnForceUnmodified as
                                           unsafe extern "C" fn(_: FORCE_TYPE,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnGetPlayerStats:
                                  Some(pfnGetPlayerStats as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> ()),
                              pfnAddServerCommand:
                                  Some(Cmd_AddServerCommand as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _: xcommand_t)
                                               -> ()),
                              pfnVoice_GetClientListening:
                                  Some(pfnVoice_GetClientListening as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> qboolean),
                              pfnVoice_SetClientListening:
                                  Some(pfnVoice_SetClientListening as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _: qboolean)
                                               -> qboolean),
                              pfnGetPlayerAuthId:
                                  Some(pfnGetPlayerAuthId as
                                           unsafe extern "C" fn(_:
                                                                    *mut edict_t)
                                               -> *const libc::c_char),
                              pfnSequenceGet:
                                  Some(pfnSequenceGet as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> *mut libc::c_void),
                              pfnSequencePickSentence:
                                  Some(pfnSequencePickSentence as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> *mut libc::c_void),
                              pfnGetFileSize:
                                  Some(COM_FileSize as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnGetApproxWavePlayLen:
                                  Some(Sound_GetApproxWavePlayLen as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> uint),
                              pfnIsCareerMatch:
                                  Some(pfnIsCareerMatch as
                                           unsafe extern "C" fn()
                                               -> libc::c_int),
                              pfnGetLocalizedStringLength:
                                  Some(pfnGetLocalizedStringLength as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char)
                                               -> libc::c_int),
                              pfnRegisterTutorMessageShown:
                                  Some(pfnRegisterTutorMessageShown as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()),
                              pfnGetTimesTutorMessageShown:
                                  Some(pfnGetTimesTutorMessageShown as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              pfnProcessTutorMessageDecayBuffer:
                                  Some(pfnProcessTutorMessageDecayBuffer as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnConstructTutorMessageDecayBuffer:
                                  Some(pfnConstructTutorMessageDecayBuffer as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnResetTutorMessageDecayData:
                                  Some(pfnResetTutorMessageDecayData as
                                           unsafe extern "C" fn() -> ()),
                              pfnQueryClientCvarValue:
                                  Some(pfnQueryClientCvarValue as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    *const libc::c_char)
                                               -> ()),
                              pfnQueryClientCvarValue2:
                                  Some(pfnQueryClientCvarValue2 as
                                           unsafe extern "C" fn(_:
                                                                    *const edict_t,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_int)
                                               -> ()),
                              pfnCheckParm:
                                  Some(COM_CheckParm as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    *mut *mut libc::c_char)
                                               -> libc::c_int),};
            init
        }
    };
/*
====================
SV_ParseEdict

Parses an edict out of the given string, returning the new position
ed should be a properly initialized empty edict.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ParseEdict(mut pfile: *mut *mut libc::c_char,
                                       mut ent: *mut edict_t) -> qboolean {
    let mut pkvd: [KeyValueData; 256] =
        [KeyValueData{szClassName: 0 as *mut libc::c_char,
                      szKeyName: 0 as *mut libc::c_char,
                      szValue: 0 as *mut libc::c_char,
                      fHandled: 0,}; 256]; // per one entity
    let mut adjust_origin: qboolean = false_0;
    let mut i: libc::c_int = 0;
    let mut numpairs: libc::c_int = 0 as libc::c_int;
    let mut classname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: [libc::c_char; 2048] = [0; 2048];
    let mut origin: vec3_t = [0.; 3];
    loop 
         // go through all the dictionary pairs
         {
        let mut keyname: string = [0; 256];
        // parse key
        *pfile =
            _COM_ParseFileSafe(*pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 2048]>()
                                   as libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int); // end of desc
        if (*pfile).is_null() {
            Host_Error(b"ED_ParseEdict: EOF without closing brace\n\x00" as
                           *const u8 as *const libc::c_char);
        }
        if token[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
            break ;
        }
        Q_strncpy(keyname.as_mut_ptr(), token.as_mut_ptr(),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        // parse value
        *pfile =
            _COM_ParseFileSafe(*pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 2048]>()
                                   as libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if (*pfile).is_null() {
            Host_Error(b"ED_ParseEdict: EOF without closing brace\n\x00" as
                           *const u8 as *const libc::c_char);
        }
        if token[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
            Host_Error(b"ED_ParseEdict: closing brace without data\n\x00" as
                           *const u8 as *const libc::c_char);
        }
        // ignore attempts to set key ""
        if keyname[0 as libc::c_int as usize] == 0 { continue ; }
        // "wad" field is already handled
        if Q_strncmp(keyname.as_mut_ptr(),
                     b"wad\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            continue ;
        }
        // keynames with a leading underscore are used for
		// utility comments and are immediately discarded by engine
        if world.flags as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int != 0 &&
               keyname[0 as libc::c_int as usize] as libc::c_int == '_' as i32
           {
            continue ;
        }
        // ignore attempts to set value ""
        if token[0 as libc::c_int as usize] == 0 { continue ; }
        // create keyvalue strings
        pkvd[numpairs as usize].szClassName =
            b"\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char; // unknown at this moment
        pkvd[numpairs as usize].szKeyName =
            _copystring(host.mempool, keyname.as_mut_ptr(),
                        b"../engine/server/sv_game.c\x00" as *const u8 as
                            *const libc::c_char, 4780 as libc::c_int);
        pkvd[numpairs as usize].szValue =
            _copystring(host.mempool, token.as_mut_ptr(),
                        b"../engine/server/sv_game.c\x00" as *const u8 as
                            *const libc::c_char, 4781 as libc::c_int);
        pkvd[numpairs as usize].fHandled = false_0 as libc::c_int;
        if Q_strncmp(keyname.as_mut_ptr(),
                     b"classname\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 && classname.is_null() {
            classname =
                _copystring(host.mempool, pkvd[numpairs as usize].szValue,
                            b"../engine/server/sv_game.c\x00" as *const u8 as
                                *const libc::c_char, 4785 as libc::c_int)
        }
        numpairs += 1;
        if numpairs >= 256 as libc::c_int { break ; }
    }
    if classname.is_null() {
        // release allocated strings
        i = 0 as libc::c_int;
        while i < numpairs {
            _Mem_Free(pkvd[i as usize].szKeyName as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4794 as libc::c_int);
            _Mem_Free(pkvd[i as usize].szValue as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4795 as libc::c_int);
            i += 1
        }
        return false_0
    }
    ent = SV_AllocPrivateData(ent, SV_AllocString(classname));
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_game.c\x00" as *const u8 as
                         *const libc::c_char, 4802 as libc::c_int) as u64 == 0
           ||
           (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        // release allocated strings
        i = 0 as libc::c_int;
        while i < numpairs {
            _Mem_Free(pkvd[i as usize].szKeyName as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4807 as libc::c_int);
            _Mem_Free(pkvd[i as usize].szValue as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4808 as libc::c_int);
            i += 1
        }
        return false_0
    }
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 29 as libc::c_int != 0 {
        if numpairs < 256 as libc::c_int {
            pkvd[numpairs as usize].szClassName =
                b"custom\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            pkvd[numpairs as usize].szKeyName =
                b"customclass\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            pkvd[numpairs as usize].szValue = classname;
            pkvd[numpairs as usize].fHandled = false_0 as libc::c_int;
            numpairs += 1
        }
        // clear it now - no longer used
        (*ent).v.flags =
            ((*ent).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 29 as libc::c_int)) as libc::c_int
    }
    // chemical existence have broked changelevels
    if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                  b"ce\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        if Q_strnicmp(sv.name.as_mut_ptr(),
                      b"ce08_02\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 &&
               Q_strnicmp(classname,
                          b"info_player_start_force\x00" as *const u8 as
                              *const libc::c_char, 99999 as libc::c_int) == 0
           {
            adjust_origin = true_0
        }
    }
    i = 0 as libc::c_int;
    while i < numpairs {
        if Q_strncmp(pkvd[i as usize].szKeyName,
                     b"angle\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            let mut flYawAngle: libc::c_float =
                Q_atof(pkvd[i as usize].szValue);
            // technically an error
            _Mem_Free(pkvd[i as usize].szKeyName as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char,
                      4843 as libc::c_int); // will be replace with 'angles'
            _Mem_Free(pkvd[i as usize].szValue as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char,
                      4844 as
                          libc::c_int); // release old value, so we don't need these
            pkvd[i as usize].szKeyName =
                _copystring(host.mempool,
                            b"angles\x00" as *const u8 as *const libc::c_char,
                            b"../engine/server/sv_game.c\x00" as *const u8 as
                                *const libc::c_char,
                            4845 as
                                libc::c_int); // release old value, so we don't need these
            if flYawAngle >= 0.0f32 {
                pkvd[i as usize].szValue =
                    _copystring(host.mempool,
                                va(b"%g %g %g\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*ent).v.angles[0 as libc::c_int as usize]
                                       as libc::c_double,
                                   flYawAngle as libc::c_double,
                                   (*ent).v.angles[2 as libc::c_int as usize]
                                       as libc::c_double),
                                b"../engine/server/sv_game.c\x00" as *const u8
                                    as *const libc::c_char,
                                4848 as libc::c_int)
            } else if flYawAngle == -1.0f32 {
                pkvd[i as usize].szValue =
                    _copystring(host.mempool,
                                b"-90 0 0\x00" as *const u8 as
                                    *const libc::c_char,
                                b"../engine/server/sv_game.c\x00" as *const u8
                                    as *const libc::c_char,
                                4850 as libc::c_int)
            } else if flYawAngle == -2.0f32 {
                pkvd[i as usize].szValue =
                    _copystring(host.mempool,
                                b"90 0 0\x00" as *const u8 as
                                    *const libc::c_char,
                                b"../engine/server/sv_game.c\x00" as *const u8
                                    as *const libc::c_char,
                                4852 as libc::c_int)
            } else {
                pkvd[i as usize].szValue =
                    _copystring(host.mempool,
                                b"0 0 0\x00" as *const u8 as
                                    *const libc::c_char,
                                b"../engine/server/sv_game.c\x00" as *const u8
                                    as *const libc::c_char,
                                4853 as libc::c_int)
            }
        }
        if adjust_origin as libc::c_uint != 0 &&
               Q_strncmp(pkvd[i as usize].szKeyName,
                         b"origin\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
            let mut pstart: *mut libc::c_char = pkvd[i as usize].szValue;
            COM_ParseVector(&mut pstart, origin.as_mut_ptr(),
                            3 as libc::c_int as size_t);
            _Mem_Free(pkvd[i as usize].szValue as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4862 as libc::c_int);
            pkvd[i as usize].szValue =
                _copystring(host.mempool,
                            va(b"%g %g %g\x00" as *const u8 as
                                   *const libc::c_char,
                               origin[0 as libc::c_int as usize] as
                                   libc::c_double,
                               origin[1 as libc::c_int as usize] as
                                   libc::c_double,
                               (origin[2 as libc::c_int as usize] - 16.0f32)
                                   as libc::c_double),
                            b"../engine/server/sv_game.c\x00" as *const u8 as
                                *const libc::c_char, 4863 as libc::c_int)
        }
        if Q_strncmp(pkvd[i as usize].szKeyName,
                     b"light\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            _Mem_Free(pkvd[i as usize].szKeyName as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4868 as libc::c_int);
            pkvd[i as usize].szKeyName =
                _copystring(host.mempool,
                            b"light_level\x00" as *const u8 as
                                *const libc::c_char,
                            b"../engine/server/sv_game.c\x00" as *const u8 as
                                *const libc::c_char, 4869 as libc::c_int)
        }
        if pkvd[i as usize].fHandled == 0 {
            pkvd[i as usize].szClassName = classname;
            svgame.dllFuncs.pfnKeyValue.expect("non-null function pointer")(ent,
                                                                            &mut *pkvd.as_mut_ptr().offset(i
                                                                                                               as
                                                                                                               isize));
        }
        // no reason to keep this data
        if Mem_IsAllocatedExt(host.mempool,
                              pkvd[i as usize].szKeyName as *mut libc::c_void)
               as u64 != 0 {
            _Mem_Free(pkvd[i as usize].szKeyName as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4880 as libc::c_int);
        }
        if Mem_IsAllocatedExt(host.mempool,
                              pkvd[i as usize].szValue as *mut libc::c_void)
               as u64 != 0 {
            _Mem_Free(pkvd[i as usize].szValue as *mut libc::c_void,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 4883 as libc::c_int);
        }
        i += 1
    }
    if !classname.is_null() &&
           Mem_IsAllocatedExt(host.mempool, classname as *mut libc::c_void) as
               libc::c_uint != 0 {
        _Mem_Free(classname as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 4887 as libc::c_int);
    }
    return true_0;
}
/*
================
SV_LoadFromFile

The entities are directly placed in the array, rather than allocated with
ED_Alloc, because otherwise an error loading the map would have entity
number references out of order.

Creates a server's entity / program execution context by
parsing textual entity definitions out of an ent file.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LoadFromFile(mut mapname: *const libc::c_char,
                                         mut entities: *mut libc::c_char) {
    let mut token: [libc::c_char; 2048] = [0; 2048];
    let mut create_world: qboolean = true_0;
    let mut inhibited: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    // user dll can override spawn entities function (Xash3D extension)
    if svgame.physFuncs.SV_LoadEntities.is_none() ||
           svgame.physFuncs.SV_LoadEntities.expect("non-null function pointer")(mapname,
                                                                                entities)
               == 0 {
        inhibited = 0 as libc::c_int;
        loop 
             // parse ents
             {
            entities =
                _COM_ParseFileSafe(entities, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if entities.is_null() { break ; }
            if token[0 as libc::c_int as usize] as libc::c_int != '{' as i32 {
                Host_Error(b"ED_LoadFromFile: found %s when expecting {\n\x00"
                               as *const u8 as *const libc::c_char,
                           token.as_mut_ptr());
            }
            if create_world as u64 != 0 {
                create_world = false_0;
                ent = SV_EdictNum(0 as libc::c_int)
                // already initialized
            } else { ent = SV_AllocEdict() }
            if SV_ParseEdict(&mut entities, ent) as u64 == 0 { continue ; }
            if svgame.dllFuncs.pfnSpawn.expect("non-null function pointer")(ent)
                   == -(1 as libc::c_int) {
                // game rejected the spawn
                if (*ent).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 30 as libc::c_int == 0 {
                    SV_FreeEdict(ent);
                    inhibited += 1
                }
            }
        }
        Con_DPrintf(b"\n%i entities inhibited\n\x00" as *const u8 as
                        *const libc::c_char, inhibited);
    }
    // reset world origin and angles for some reason
    (*svgame.edicts).v.origin[2 as libc::c_int as usize] =
        0 as libc::c_int as vec_t;
    (*svgame.edicts).v.origin[1 as libc::c_int as usize] =
        (*svgame.edicts).v.origin[2 as libc::c_int as usize];
    (*svgame.edicts).v.origin[0 as libc::c_int as usize] =
        (*svgame.edicts).v.origin[1 as libc::c_int as usize];
    (*svgame.edicts).v.angles[2 as libc::c_int as usize] =
        0 as libc::c_int as vec_t;
    (*svgame.edicts).v.angles[1 as libc::c_int as usize] =
        (*svgame.edicts).v.angles[2 as libc::c_int as usize];
    (*svgame.edicts).v.angles[0 as libc::c_int as usize] =
        (*svgame.edicts).v.angles[1 as libc::c_int as usize];
}
/*
==============
SpawnEntities

Creates a server's entity / program execution context by
parsing textual entity definitions out of an ent file.
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SpawnEntities(mut mapname: *const libc::c_char) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    // reset misc parms
    Cvar_Reset(b"sv_zmax\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_wateramp\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_wateralpha\x00" as *const u8 as *const libc::c_char);
    // reset sky parms
    Cvar_Reset(b"sv_skycolor_r\x00" as *const u8 as
                   *const libc::c_char); // world model
    Cvar_Reset(b"sv_skycolor_g\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_skycolor_b\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_skyvec_x\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_skyvec_y\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_skyvec_z\x00" as *const u8 as *const libc::c_char);
    Cvar_Reset(b"sv_skyname\x00" as *const u8 as *const libc::c_char);
    ent = SV_EdictNum(0 as libc::c_int);
    if (*ent).free as u64 != 0 { SV_InitEdict(ent); }
    (*ent).v.model =
        SV_MakeString(sv.model_precache[1 as libc::c_int as
                                            usize].as_mut_ptr());
    (*ent).v.modelindex = 1 as libc::c_int;
    (*ent).v.solid = 4 as libc::c_int;
    (*ent).v.movetype = 7 as libc::c_int;
    svgame.movevars.fog_settings = 0 as libc::c_int;
    (*svgame.globals).maxEntities = (*SI.GameInfo).max_edicts;
    (*svgame.globals).mapname = SV_MakeString(sv.name.as_mut_ptr());
    (*svgame.globals).startspot = SV_MakeString(sv.startspot.as_mut_ptr());
    (*svgame.globals).time = sv.time as libc::c_float;
    // spawn the rest of the entities on the map
    SV_LoadFromFile(mapname, (*sv.worldmodel).entities);
}
#[no_mangle]
pub unsafe extern "C" fn SV_UnloadProgs() {
    if svgame.hInstance.is_null() { return }
    SV_DeactivateServer();
    Delta_Shutdown();
    // / TODO: reenable this when
	// / SV_UnloadProgs will be disabled
	//Mod_ClearUserData ();
    SV_FreeStringPool();
    if svgame.dllFuncs2.pfnGameShutdown.is_some() {
        svgame.dllFuncs2.pfnGameShutdown.expect("non-null function pointer")();
    }
    // now we can unload cvars
    Cvar_FullSet(b"host_gameloaded\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    Cvar_FullSet(b"sv_background\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    // free entity baselines
    if !svs.static_entities.is_null() {
        _Mem_Free(svs.static_entities as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 5017 as libc::c_int);
    }
    if !svs.baselines.is_null() {
        _Mem_Free(svs.baselines as *mut libc::c_void,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 5018 as libc::c_int);
    }
    svs.baselines = 0 as *mut entity_state_t;
    // remove server cmds
    SV_KillOperatorCommands();
    // must unlink all game cvars,
	// before pointers on them will be lost...
    Cvar_Unlink((1 as libc::c_int) << 3 as libc::c_int);
    Cmd_Unlink(((1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int);
    Mod_ResetStudioAPI();
    svs.game_library_loaded = false_0;
    COM_FreeLibrary(svgame.hInstance);
    _Mem_FreePool(&mut svgame.mempool,
                  b"../engine/server/sv_game.c\x00" as *const u8 as
                      *const libc::c_char, 5033 as libc::c_int);
    memset(&mut svgame as *mut svgame_static_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<svgame_static_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn SV_LoadProgs(mut name: *const libc::c_char)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    static mut GetEntityAPI: APIFUNCTION = None;
    static mut GetEntityAPI2: APIFUNCTION2 = None;
    static mut GiveFnptrsToDll: GIVEFNPTRSTODLL = None;
    static mut GiveNewDllFuncs: NEW_DLL_FUNCTIONS_FN = None;
    static mut gpEngfuncs: enginefuncs_t =
        enginefuncs_t{pfnPrecacheModel: None,
                      pfnPrecacheSound: None,
                      pfnSetModel: None,
                      pfnModelIndex: None,
                      pfnModelFrames: None,
                      pfnSetSize: None,
                      pfnChangeLevel: None,
                      pfnGetSpawnParms: None,
                      pfnSaveSpawnParms: None,
                      pfnVecToYaw: None,
                      pfnVecToAngles: None,
                      pfnMoveToOrigin: None,
                      pfnChangeYaw: None,
                      pfnChangePitch: None,
                      pfnFindEntityByString: None,
                      pfnGetEntityIllum: None,
                      pfnFindEntityInSphere: None,
                      pfnFindClientInPVS: None,
                      pfnEntitiesInPVS: None,
                      pfnMakeVectors: None,
                      pfnAngleVectors: None,
                      pfnCreateEntity: None,
                      pfnRemoveEntity: None,
                      pfnCreateNamedEntity: None,
                      pfnMakeStatic: None,
                      pfnEntIsOnFloor: None,
                      pfnDropToFloor: None,
                      pfnWalkMove: None,
                      pfnSetOrigin: None,
                      pfnEmitSound: None,
                      pfnEmitAmbientSound: None,
                      pfnTraceLine: None,
                      pfnTraceToss: None,
                      pfnTraceMonsterHull: None,
                      pfnTraceHull: None,
                      pfnTraceModel: None,
                      pfnTraceTexture: None,
                      pfnTraceSphere: None,
                      pfnGetAimVector: None,
                      pfnServerCommand: None,
                      pfnServerExecute: None,
                      pfnClientCommand: None,
                      pfnParticleEffect: None,
                      pfnLightStyle: None,
                      pfnDecalIndex: None,
                      pfnPointContents: None,
                      pfnMessageBegin: None,
                      pfnMessageEnd: None,
                      pfnWriteByte: None,
                      pfnWriteChar: None,
                      pfnWriteShort: None,
                      pfnWriteLong: None,
                      pfnWriteAngle: None,
                      pfnWriteCoord: None,
                      pfnWriteString: None,
                      pfnWriteEntity: None,
                      pfnCVarRegister: None,
                      pfnCVarGetFloat: None,
                      pfnCVarGetString: None,
                      pfnCVarSetFloat: None,
                      pfnCVarSetString: None,
                      pfnAlertMessage: None,
                      pfnEngineFprintf: None,
                      pfnPvAllocEntPrivateData: None,
                      pfnPvEntPrivateData: None,
                      pfnFreeEntPrivateData: None,
                      pfnSzFromIndex: None,
                      pfnAllocString: None,
                      pfnGetVarsOfEnt: None,
                      pfnPEntityOfEntOffset: None,
                      pfnEntOffsetOfPEntity: None,
                      pfnIndexOfEdict: None,
                      pfnPEntityOfEntIndex: None,
                      pfnFindEntityByVars: None,
                      pfnGetModelPtr: None,
                      pfnRegUserMsg: None,
                      pfnAnimationAutomove: None,
                      pfnGetBonePosition: None,
                      pfnFunctionFromName: None,
                      pfnNameForFunction: None,
                      pfnClientPrintf: None,
                      pfnServerPrint: None,
                      pfnCmd_Args: None,
                      pfnCmd_Argv: None,
                      pfnCmd_Argc: None,
                      pfnGetAttachment: None,
                      pfnCRC32_Init: None,
                      pfnCRC32_ProcessBuffer: None,
                      pfnCRC32_ProcessByte: None,
                      pfnCRC32_Final: None,
                      pfnRandomLong: None,
                      pfnRandomFloat: None,
                      pfnSetView: None,
                      pfnTime: None,
                      pfnCrosshairAngle: None,
                      pfnLoadFileForMe: None,
                      pfnFreeFile: None,
                      pfnEndSection: None,
                      pfnCompareFileTime: None,
                      pfnGetGameDir: None,
                      pfnCvar_RegisterVariable: None,
                      pfnFadeClientVolume: None,
                      pfnSetClientMaxspeed: None,
                      pfnCreateFakeClient: None,
                      pfnRunPlayerMove: None,
                      pfnNumberOfEntities: None,
                      pfnGetInfoKeyBuffer: None,
                      pfnInfoKeyValue: None,
                      pfnSetKeyValue: None,
                      pfnSetClientKeyValue: None,
                      pfnIsMapValid: None,
                      pfnStaticDecal: None,
                      pfnPrecacheGeneric: None,
                      pfnGetPlayerUserId: None,
                      pfnBuildSoundMsg: None,
                      pfnIsDedicatedServer: None,
                      pfnCVarGetPointer: None,
                      pfnGetPlayerWONId: None,
                      pfnInfo_RemoveKey: None,
                      pfnGetPhysicsKeyValue: None,
                      pfnSetPhysicsKeyValue: None,
                      pfnGetPhysicsInfoString: None,
                      pfnPrecacheEvent: None,
                      pfnPlaybackEvent: None,
                      pfnSetFatPVS: None,
                      pfnSetFatPAS: None,
                      pfnCheckVisibility: None,
                      pfnDeltaSetField: None,
                      pfnDeltaUnsetField: None,
                      pfnDeltaAddEncoder: None,
                      pfnGetCurrentPlayer: None,
                      pfnCanSkipPlayer: None,
                      pfnDeltaFindField: None,
                      pfnDeltaSetFieldByIndex: None,
                      pfnDeltaUnsetFieldByIndex: None,
                      pfnSetGroupMask: None,
                      pfnCreateInstancedBaseline: None,
                      pfnCvar_DirectSet: None,
                      pfnForceUnmodified: None,
                      pfnGetPlayerStats: None,
                      pfnAddServerCommand: None,
                      pfnVoice_GetClientListening: None,
                      pfnVoice_SetClientListening: None,
                      pfnGetPlayerAuthId: None,
                      pfnSequenceGet: None,
                      pfnSequencePickSentence: None,
                      pfnGetFileSize: None,
                      pfnGetApproxWavePlayLen: None,
                      pfnIsCareerMatch: None,
                      pfnGetLocalizedStringLength: None,
                      pfnRegisterTutorMessageShown: None,
                      pfnGetTimesTutorMessageShown: None,
                      pfnProcessTutorMessageDecayBuffer: None,
                      pfnConstructTutorMessageDecayBuffer: None,
                      pfnResetTutorMessageDecayData: None,
                      pfnQueryClientCvarValue: None,
                      pfnQueryClientCvarValue2: None,
                      pfnCheckParm: None,};
    static mut gpGlobals: globalvars_t =
        globalvars_t{time: 0.,
                     frametime: 0.,
                     force_retouch: 0.,
                     mapname: 0,
                     startspot: 0,
                     deathmatch: 0.,
                     coop: 0.,
                     teamplay: 0.,
                     serverflags: 0.,
                     found_secrets: 0.,
                     v_forward: [0.; 3],
                     v_up: [0.; 3],
                     v_right: [0.; 3],
                     trace_allsolid: 0.,
                     trace_startsolid: 0.,
                     trace_fraction: 0.,
                     trace_endpos: [0.; 3],
                     trace_plane_normal: [0.; 3],
                     trace_plane_dist: 0.,
                     trace_ent: 0 as *const edict_t as *mut edict_t,
                     trace_inopen: 0.,
                     trace_inwater: 0.,
                     trace_hitgroup: 0,
                     trace_flags: 0,
                     changelevel: 0,
                     cdAudioTrack: 0,
                     maxClients: 0,
                     maxEntities: 0,
                     pStringBase: 0 as *const libc::c_char,
                     pSaveData: 0 as *const libc::c_void as *mut libc::c_void,
                     vecLandmarkOffset: [0.; 3],};
    static mut gpMove: playermove_t =
        playermove_t{player_index: 0,
                     server: false_0,
                     multiplayer: false_0,
                     time: 0.,
                     frametime: 0.,
                     forward: [0.; 3],
                     right: [0.; 3],
                     up: [0.; 3],
                     origin: [0.; 3],
                     angles: [0.; 3],
                     oldangles: [0.; 3],
                     velocity: [0.; 3],
                     movedir: [0.; 3],
                     basevelocity: [0.; 3],
                     view_ofs: [0.; 3],
                     flDuckTime: 0.,
                     bInDuck: false_0,
                     flTimeStepSound: 0,
                     iStepLeft: 0,
                     flFallVelocity: 0.,
                     punchangle: [0.; 3],
                     flSwimTime: 0.,
                     flNextPrimaryAttack: 0.,
                     effects: 0,
                     flags: 0,
                     usehull: 0,
                     gravity: 0.,
                     friction: 0.,
                     oldbuttons: 0,
                     waterjumptime: 0.,
                     dead: false_0,
                     deadflag: 0,
                     spectator: 0,
                     movetype: 0,
                     onground: 0,
                     waterlevel: 0,
                     watertype: 0,
                     oldwaterlevel: 0,
                     sztexturename: [0; 256],
                     chtexturetype: 0,
                     maxspeed: 0.,
                     clientmaxspeed: 0.,
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
                     vuser4: [0.; 3],
                     numphysent: 0,
                     physents:
                         [physent_t{name: [0; 32],
                                    player: 0,
                                    origin: [0.; 3],
                                    model:
                                        0 as *const model_s as *mut model_s,
                                    studiomodel:
                                        0 as *const model_s as *mut model_s,
                                    mins: [0.; 3],
                                    maxs: [0.; 3],
                                    info: 0,
                                    angles: [0.; 3],
                                    solid: 0,
                                    skin: 0,
                                    rendermode: 0,
                                    frame: 0.,
                                    sequence: 0,
                                    controller: [0; 4],
                                    blending: [0; 2],
                                    movetype: 0,
                                    takedamage: 0,
                                    blooddecal: 0,
                                    team: 0,
                                    classnumber: 0,
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
                                    vuser4: [0.; 3],}; 600],
                     nummoveent: 0,
                     moveents:
                         [physent_t{name: [0; 32],
                                    player: 0,
                                    origin: [0.; 3],
                                    model:
                                        0 as *const model_s as *mut model_s,
                                    studiomodel:
                                        0 as *const model_s as *mut model_s,
                                    mins: [0.; 3],
                                    maxs: [0.; 3],
                                    info: 0,
                                    angles: [0.; 3],
                                    solid: 0,
                                    skin: 0,
                                    rendermode: 0,
                                    frame: 0.,
                                    sequence: 0,
                                    controller: [0; 4],
                                    blending: [0; 2],
                                    movetype: 0,
                                    takedamage: 0,
                                    blooddecal: 0,
                                    team: 0,
                                    classnumber: 0,
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
                     numvisent: 0,
                     visents:
                         [physent_t{name: [0; 32],
                                    player: 0,
                                    origin: [0.; 3],
                                    model:
                                        0 as *const model_s as *mut model_s,
                                    studiomodel:
                                        0 as *const model_s as *mut model_s,
                                    mins: [0.; 3],
                                    maxs: [0.; 3],
                                    info: 0,
                                    angles: [0.; 3],
                                    solid: 0,
                                    skin: 0,
                                    rendermode: 0,
                                    frame: 0.,
                                    sequence: 0,
                                    controller: [0; 4],
                                    blending: [0; 2],
                                    movetype: 0,
                                    takedamage: 0,
                                    blooddecal: 0,
                                    team: 0,
                                    classnumber: 0,
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
                                    vuser4: [0.; 3],}; 600],
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
                     numtouch: 0,
                     touchindex:
                         [pmtrace_t{allsolid: false_0,
                                    startsolid: false_0,
                                    inopen: false_0,
                                    inwater: false_0,
                                    fraction: 0.,
                                    endpos: [0.; 3],
                                    plane:
                                        pmplane_t{normal: [0.; 3], dist: 0.,},
                                    ent: 0,
                                    deltavelocity: [0.; 3],
                                    hitgroup: 0,}; 600],
                     physinfo: [0; 256],
                     movevars: 0 as *const movevars_s as *mut movevars_s,
                     player_mins: [[0.; 3]; 4],
                     player_maxs: [[0.; 3]; 4],
                     PM_Info_ValueForKey: None,
                     PM_Particle: None,
                     PM_TestPlayerPosition: None,
                     Con_NPrintf: None,
                     Con_DPrintf: None,
                     Con_Printf: None,
                     Sys_FloatTime: None,
                     PM_StuckTouch: None,
                     PM_PointContents: None,
                     PM_TruePointContents: None,
                     PM_HullPointContents: None,
                     PM_PlayerTrace: None,
                     PM_TraceLine: None,
                     RandomLong: None,
                     RandomFloat: None,
                     PM_GetModelType: None,
                     PM_GetModelBounds: None,
                     PM_HullForBsp: None,
                     PM_TraceModel: None,
                     COM_FileSize: None,
                     COM_LoadFile: None,
                     COM_FreeFile: None,
                     memfgets: None,
                     runfuncs: false_0,
                     PM_PlaySound: None,
                     PM_TraceTexture: None,
                     PM_PlaybackEventFull: None,
                     PM_PlayerTraceEx: None,
                     PM_TestPlayerPositionEx: None,
                     PM_TraceLineEx: None,
                     PM_TraceSurface: None,};
    let mut e: *mut edict_t = 0 as *mut edict_t;
    if !svgame.hInstance.is_null() { SV_UnloadProgs(); }
    // fill it in
    svgame.pmove = &mut gpMove;
    svgame.globals = &mut gpGlobals;
    svgame.mempool =
        _Mem_AllocPool(b"Server Edicts Zone\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/server/sv_game.c\x00" as *const u8 as
                           *const libc::c_char, 5054 as libc::c_int);
    svgame.hInstance = COM_LoadLibrary(name, true_0 as libc::c_int, false_0);
    if svgame.hInstance.is_null() {
        _Mem_FreePool(&mut svgame.mempool,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 5059 as libc::c_int);
        return false_0
    }
    // make sure what new dll functions is cleared
    memset(&mut svgame.dllFuncs2 as *mut NEW_DLL_FUNCTIONS as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<NEW_DLL_FUNCTIONS>() as libc::c_ulong);
    // make sure what physic functions is cleared
    memset(&mut svgame.physFuncs as *mut physics_interface_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<physics_interface_t>() as libc::c_ulong);
    // make local copy of engfuncs to prevent overwrite it with bots.dll
    memcpy(&mut gpEngfuncs as *mut enginefuncs_t as *mut libc::c_void,
           &mut gEngfuncs as *mut enginefuncs_t as *const libc::c_void,
           ::std::mem::size_of::<enginefuncs_t>() as libc::c_ulong);
    GetEntityAPI =
        ::std::mem::transmute::<*mut libc::c_void,
                                APIFUNCTION>(COM_GetProcAddress(svgame.hInstance,
                                                                b"GetEntityAPI\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char));
    GetEntityAPI2 =
        ::std::mem::transmute::<*mut libc::c_void,
                                APIFUNCTION2>(COM_GetProcAddress(svgame.hInstance,
                                                                 b"GetEntityAPI2\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char));
    GiveNewDllFuncs =
        ::std::mem::transmute::<*mut libc::c_void,
                                NEW_DLL_FUNCTIONS_FN>(COM_GetProcAddress(svgame.hInstance,
                                                                         b"GetNewDLLFunctions\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
    if GetEntityAPI.is_none() && GetEntityAPI2.is_none() {
        COM_FreeLibrary(svgame.hInstance);
        Con_Printf(b"^1Error:^7 SV_LoadProgs: failed to get address of GetEntityAPI proc\n\x00"
                       as *const u8 as *const libc::c_char);
        svgame.hInstance = 0 as *mut libc::c_void;
        _Mem_FreePool(&mut svgame.mempool,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 5081 as libc::c_int);
        return false_0
    }
    GiveFnptrsToDll =
        ::std::mem::transmute::<*mut libc::c_void,
                                GIVEFNPTRSTODLL>(COM_GetProcAddress(svgame.hInstance,
                                                                    b"GiveFnptrsToDll\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char));
    if GiveFnptrsToDll.is_none() {
        COM_FreeLibrary(svgame.hInstance);
        Con_Printf(b"^1Error:^7 SV_LoadProgs: failed to get address of GiveFnptrsToDll proc\n\x00"
                       as *const u8 as *const libc::c_char);
        svgame.hInstance = 0 as *mut libc::c_void;
        _Mem_FreePool(&mut svgame.mempool,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 5092 as libc::c_int);
        return false_0
    }
    GiveFnptrsToDll.expect("non-null function pointer")(&mut gpEngfuncs,
                                                        svgame.globals);
    // get extended callbacks
    if GiveNewDllFuncs.is_some() {
        version = 1 as libc::c_int;
        if GiveNewDllFuncs.expect("non-null function pointer")(&mut svgame.dllFuncs2,
                                                               &mut version)
               == 0 {
            if version != 1 as libc::c_int {
                Con_Printf(b"^3Warning:^7 SV_LoadProgs: new interface version %i should be %i\n\x00"
                               as *const u8 as *const libc::c_char,
                           1 as libc::c_int, version);
            }
            memset(&mut svgame.dllFuncs2 as *mut NEW_DLL_FUNCTIONS as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<NEW_DLL_FUNCTIONS>() as
                       libc::c_ulong);
        }
    }
    version = 140 as libc::c_int;
    if GetEntityAPI2.is_some() {
        if GetEntityAPI2.expect("non-null function pointer")(&mut svgame.dllFuncs,
                                                             &mut version) ==
               0 {
            Con_Printf(b"^3Warning:^7 SV_LoadProgs: interface version %i should be %i\n\x00"
                           as *const u8 as *const libc::c_char,
                       140 as libc::c_int, version);
            // fallback to old API
            if GetEntityAPI.expect("non-null function pointer")(&mut svgame.dllFuncs,
                                                                version) == 0
               {
                COM_FreeLibrary(svgame.hInstance);
                Con_Printf(b"^1Error:^7 SV_LoadProgs: couldn\'t get entity API\n\x00"
                               as *const u8 as *const libc::c_char);
                svgame.hInstance = 0 as *mut libc::c_void;
                _Mem_FreePool(&mut svgame.mempool,
                              b"../engine/server/sv_game.c\x00" as *const u8
                                  as *const libc::c_char,
                              5125 as libc::c_int);
                return false_0
            }
        } else {
            Con_Reportf(b"SV_LoadProgs: ^2initailized extended EntityAPI ^7ver. %i\n\x00"
                            as *const u8 as *const libc::c_char, version);
        }
    } else if GetEntityAPI.expect("non-null function pointer")(&mut svgame.dllFuncs,
                                                               version) == 0 {
        COM_FreeLibrary(svgame.hInstance);
        Con_Printf(b"^1Error:^7 SV_LoadProgs: couldn\'t get entity API\n\x00"
                       as *const u8 as *const libc::c_char);
        svgame.hInstance = 0 as *mut libc::c_void;
        _Mem_FreePool(&mut svgame.mempool,
                      b"../engine/server/sv_game.c\x00" as *const u8 as
                          *const libc::c_char, 5136 as libc::c_int);
        return false_0
    }
    SV_InitOperatorCommands();
    Mod_InitStudioAPI();
    if SV_InitPhysicsAPI() as u64 == 0 {
        Con_Printf(b"^3Warning:^7 SV_LoadProgs: couldn\'t get physics API\n\x00"
                       as *const u8 as *const libc::c_char);
    }
    // grab function SV_SaveGameComment
    SV_InitSaveRestore(); // setup string base
    (*svgame.globals).pStringBase =
        b"\x00" as *const u8 as *const libc::c_char; // clients + world
    (*svgame.globals).maxEntities =
        (*SI.GameInfo).max_edicts; // mark all edicts as freed
    (*svgame.globals).maxClients = svs.maxclients;
    svgame.edicts =
        _Mem_Alloc(svgame.mempool,
                   (::std::mem::size_of::<edict_t>() as
                        libc::c_ulong).wrapping_mul((*SI.GameInfo).max_edicts
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_game.c\x00" as *const u8 as
                       *const libc::c_char, 5155 as libc::c_int) as
            *mut edict_t;
    svs.static_entities =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<entity_state_t>() as
                        libc::c_ulong).wrapping_mul(3096 as libc::c_int as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_game.c\x00" as *const u8 as
                       *const libc::c_char, 5156 as libc::c_int) as
            *mut entity_state_t;
    svs.baselines =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<entity_state_t>() as
                        libc::c_ulong).wrapping_mul((*SI.GameInfo).max_edicts
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_game.c\x00" as *const u8 as
                       *const libc::c_char, 5157 as libc::c_int) as
            *mut entity_state_t;
    svgame.numEntities = svs.maxclients + 1 as libc::c_int;
    i = 0 as libc::c_int;
    e = svgame.edicts;
    while i < (*SI.GameInfo).max_edicts {
        (*e).free = true_0;
        i += 1;
        e = e.offset(1)
    }
    Cvar_FullSet(b"host_gameloaded\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    SV_AllocStringPool();
    // fire once
    Con_Printf(b"Dll loaded for game ^2\"%s\"\n\x00" as *const u8 as
                   *const libc::c_char,
               svgame.dllFuncs.pfnGetGameDescription.expect("non-null function pointer")());
    // all done, initialize game
    svgame.dllFuncs.pfnGameInit.expect("non-null function pointer")();
    // initialize pm_shared
    SV_InitClientMove();
    Delta_Init();
    // register custom encoders
    svgame.dllFuncs.pfnRegisterEncoders.expect("non-null function pointer")();
    return true_0;
}
