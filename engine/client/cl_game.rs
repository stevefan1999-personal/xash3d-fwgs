#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type SDL_Window;
    pub type file_s;
    pub type grasshdr_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type cmd_s;
    pub type mip_s;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Con_NPrintf(idx: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_NXPrintf(info: *mut con_nprint_t, fmt: *const libc::c_char,
                    _: ...);
    #[no_mangle]
    fn COM_RandomFloat(fMin: libc::c_float, fMax: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn S_StopSound(entnum: libc::c_int, channel: libc::c_int,
                   soundname: *const libc::c_char);
    #[no_mangle]
    fn Info_RemoveKey(s: *mut libc::c_char, key: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn Info_SetValueForStarKey(s: *mut libc::c_char, key: *const libc::c_char,
                               value: *const libc::c_char,
                               maxsize: libc::c_int) -> qboolean;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn CL_ServerCommand(reliable: qboolean, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cmd_GetName(cmd: *mut cmd_s) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_AliasGetList() -> *mut cmdalias_s;
    #[no_mangle]
    fn Cmd_GetNextFunctionHandle(cmd: *mut cmd_s) -> *mut cmd_s;
    #[no_mangle]
    fn Cmd_GetFirstFunctionHandle() -> *mut cmd_s;
    #[no_mangle]
    fn COM_ExpandFilename(fileName: *const libc::c_char,
                          nameOutBuffer: *mut libc::c_char,
                          nameOutBufferSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn COM_AddAppDirectoryToSearchPath(pszBaseDir: *const libc::c_char,
                                       appName: *const libc::c_char);
    #[no_mangle]
    fn COM_LoadFile(filename: *const libc::c_char, usehunk: libc::c_int,
                    pLength: *mut libc::c_int) -> *mut byte;
    #[no_mangle]
    fn pfnResetTutorMessageDecayData();
    #[no_mangle]
    fn pfnProcessTutorMessageDecayBuffer(buffer: *mut libc::c_int,
                                         bufferLength: libc::c_int);
    #[no_mangle]
    fn pfnConstructTutorMessageDecayBuffer(buffer: *mut libc::c_int,
                                           buflen: libc::c_int);
    #[no_mangle]
    fn pfnIsCareerMatch() -> libc::c_int;
    #[no_mangle]
    fn pfnSequencePickSentence(groupName: *const libc::c_char,
                               pickMethod: libc::c_int,
                               picked: *mut libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn pfnSequenceGet(fileName: *const libc::c_char,
                      entryName: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_CheckParm(parm: *mut libc::c_char, ppnext: *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn pfnCVarGetPointer(szVarName: *const libc::c_char) -> *mut cvar_t;
    #[no_mangle]
    fn pfnCvar_RegisterClientVariable(szName: *const libc::c_char,
                                      szValue: *const libc::c_char,
                                      flags: libc::c_int) -> *mut cvar_t;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Sound_GetApproxWavePlayLen(filepath: *const libc::c_char) -> uint;
    #[no_mangle]
    fn SDL_GetWindowPosition(window: *mut SDL_Window, x: *mut libc::c_int,
                             y: *mut libc::c_int);
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
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_ExtractFilePath(path: *const libc::c_char,
                           dest: *mut libc::c_char);
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Platform_GetMousePos(x: *mut libc::c_int, y: *mut libc::c_int);
    #[no_mangle]
    fn Platform_SetMousePos(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Cvar_GetList() -> *mut cvar_t;
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
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
    static mut scr_loading: *mut convar_t;
    #[no_mangle]
    fn NET_Config(net_enable: qboolean);
    #[no_mangle]
    fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    fn NET_StringToAdr(string: *const libc::c_char, adr: *mut netadr_t)
     -> qboolean;
    #[no_mangle]
    fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_SendPacket(sock: netsrc_t, length: size_t,
                      data: *const libc::c_void, to: netadr_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_AddFilteredText(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_Unlink(group: libc::c_int);
    #[no_mangle]
    fn Cmd_AddClientCommand(cmd_name: *const libc::c_char,
                            function: xcommand_t) -> libc::c_int;
    #[no_mangle]
    fn _Mem_Realloc(poolptr: poolhandle_t, memptr: *mut libc::c_void,
                    size: size_t, clear: qboolean,
                    filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
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
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn COM_FreeFile(buffer: *mut libc::c_void);
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Print(file: *mut file_t, msg: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
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
    static mut cl_crosshair: *mut convar_t;
    #[no_mangle]
    static mut hud_scale: *mut convar_t;
    #[no_mangle]
    static mut hud_utf8: *mut convar_t;
    #[no_mangle]
    static mut scr_centertime: *mut convar_t;
    #[no_mangle]
    static mut rate: *mut convar_t;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    fn S_RegisterSound(sample: *const libc::c_char) -> sound_t;
    #[no_mangle]
    fn S_StartSound(pos: *const vec_t, ent: libc::c_int, chan: libc::c_int,
                    sfx: sound_t, vol: libc::c_float, attn: libc::c_float,
                    pitch: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn S_StartBackgroundTrack(intro: *const libc::c_char,
                              loop_0: *const libc::c_char,
                              position: libc::c_int, fullpath: qboolean);
    #[no_mangle]
    fn Mobile_Init() -> qboolean;
    #[no_mangle]
    fn pfnPIC_DrawAdditive(x: libc::c_int, y: libc::c_int, width: libc::c_int,
                           height: libc::c_int, prc: *const wrect_t);
    #[no_mangle]
    fn pfnPIC_Set(hPic: HIMAGE, r: libc::c_int, g: libc::c_int,
                  b: libc::c_int, a: libc::c_int);
    #[no_mangle]
    fn Con_UtfProcessChar(in_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn R_InitRenderAPI() -> qboolean;
    #[no_mangle]
    fn CL_InitTempEnts();
    #[no_mangle]
    fn CL_InitViewBeams();
    #[no_mangle]
    fn CL_InitParticles();
    #[no_mangle]
    fn Mod_Handle(handle: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn Mod_FreeModel(mod_0: *mut model_t);
    #[no_mangle]
    fn Mod_LoadSpriteModel(mod_0: *mut model_t, buffer: *const libc::c_void,
                           loaded: *mut qboolean, texFlags: uint);
    #[no_mangle]
    fn CL_AddClientResource(filename: *const libc::c_char,
                            type_0: libc::c_int);
    #[no_mangle]
    fn CL_ClearAllRemaps();
    #[no_mangle]
    fn Netchan_OutOfBandPrint(net_socket: libc::c_int, adr: netadr_t,
                              format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn MSG_BigShort(swap: libc::c_ushort) -> libc::c_ushort;
    #[no_mangle]
    static mut net_local: netadr_t;
    #[no_mangle]
    fn CL_WriteDemoUserMessage(buffer: *const byte, size: size_t);
    #[no_mangle]
    fn CL_PopTraceBounds();
    #[no_mangle]
    fn CL_PushTraceBounds(hullnum: libc::c_int, mins: *const libc::c_float,
                          maxs: *const libc::c_float);
    #[no_mangle]
    fn CL_TestLine(start: *const vec_t, end: *const vec_t, flags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn CL_VisTraceLine(start: *mut vec_t, end: *mut vec_t, flags: libc::c_int)
     -> *mut pmtrace_t;
    #[no_mangle]
    fn CL_ResetEvent(ei: *mut event_info_t);
    #[no_mangle]
    fn CL_EventIndex(name: *const libc::c_char) -> word;
    #[no_mangle]
    fn CL_PlaybackEvent(flags: libc::c_int, pInvoker: *const edict_t,
                        eventindex: word, delay: libc::c_float,
                        origin: *mut libc::c_float,
                        angles: *mut libc::c_float, fparam1: libc::c_float,
                        fparam2: libc::c_float, iparam1: libc::c_int,
                        iparam2: libc::c_int, bparam1: libc::c_int,
                        bparam2: libc::c_int);
    #[no_mangle]
    fn CL_WeaponAnim(iAnim: libc::c_int, body: libc::c_int);
    #[no_mangle]
    fn CL_SetSolidPlayers(playernum: libc::c_int);
    #[no_mangle]
    fn CL_PopPMStates();
    #[no_mangle]
    fn CL_PushPMStates();
    #[no_mangle]
    fn CL_SetUpPlayerPrediction(dopred: libc::c_int,
                                bIncludeLocalClient: libc::c_int);
    #[no_mangle]
    fn CL_FireCustomDecal(textureIndex: libc::c_int, entityIndex: libc::c_int,
                          modelIndex: libc::c_int, pos: *mut libc::c_float,
                          flags: libc::c_int, scale: libc::c_float);
    #[no_mangle]
    fn CL_DecalShoot(textureIndex: libc::c_int, entityIndex: libc::c_int,
                     modelIndex: libc::c_int, pos: *mut libc::c_float,
                     flags: libc::c_int);
    #[no_mangle]
    fn Key_GetKey(binding: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Key_KeynumToString(keynum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Con_Visible() -> libc::c_int;
    #[no_mangle]
    fn CL_RegisterEvent(lastnum: libc::c_int, szEvName: *const libc::c_char,
                        func: pfnEventHook);
    #[no_mangle]
    fn CL_InitClientMove();
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    fn CL_AddVisibleEntity(ent: *mut cl_entity_t, entityType: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn CL_WaterEntity(rgflPos: *const libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn Key_Event(key: libc::c_int, down: libc::c_int);
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn R_RemoveEfrags(ent: *mut cl_entity_t);
    #[no_mangle]
    fn CL_KillDeadBeams(pDeadEntity: *mut cl_entity_t);
    #[no_mangle]
    static mut g_color_table: [rgba_t; 8];
    #[no_mangle]
    fn Con_DrawCharacterLen(number: libc::c_int, width: *mut libc::c_int,
                            height: *mut libc::c_int);
    #[no_mangle]
    fn Con_DrawCharacter(x: libc::c_int, y: libc::c_int, number: libc::c_int,
                         color: *mut byte) -> libc::c_int;
    #[no_mangle]
    fn Mod_ClearUserData();
    #[no_mangle]
    fn CL_FreeParticles();
    #[no_mangle]
    fn CL_FreeViewBeams();
    #[no_mangle]
    fn CL_FreeTempEnts();
    #[no_mangle]
    fn CL_DispatchUserMessage(pszName: *const libc::c_char,
                              iSize: libc::c_int, pbuf: *mut libc::c_void)
     -> qboolean;
    #[no_mangle]
    fn Con_DrawStringLen(pText: *const libc::c_char, length: *mut libc::c_int,
                         height: *mut libc::c_int);
    #[no_mangle]
    fn Con_SetFont(fontNum: libc::c_int);
    #[no_mangle]
    static mut con_fontsize: *mut convar_t;
    #[no_mangle]
    fn Con_DrawString(x: libc::c_int, y: libc::c_int,
                      string: *const libc::c_char, setColor: *mut byte)
     -> libc::c_int;
    #[no_mangle]
    fn CL_TextMessageParse(pMemFile: *mut byte, fileSize: libc::c_int);
    #[no_mangle]
    static mut world: world_static_t;
    #[no_mangle]
    fn Mod_BoxVisible(mins: *const vec_t, maxs: *const vec_t,
                      visbits: *const byte) -> qboolean;
    #[no_mangle]
    static mut svc_strings: [*const libc::c_char; 60];
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[no_mangle]
    fn R_GetTextureParms(w: *mut libc::c_int, h: *mut libc::c_int,
                         texnum: libc::c_int);
    #[no_mangle]
    fn Con_RestoreFont();
    #[no_mangle]
    fn PM_PlayerTraceExt(pm: *mut playermove_t, p1: *mut vec_t,
                         p2: *mut vec_t, flags: libc::c_int,
                         numents: libc::c_int, ents: *mut physent_t,
                         ignore_pe: libc::c_int, pmFilter: pfnIgnore)
     -> pmtrace_t;
    #[no_mangle]
    fn PM_PointContents(pmove: *mut playermove_t, p: *const vec_t)
     -> libc::c_int;
    #[no_mangle]
    fn PM_TraceTexture(pe: *mut physent_t, vstart: *mut vec_t,
                       vend: *mut vec_t) -> *const libc::c_char;
    #[no_mangle]
    fn PM_TraceSurface(pe: *mut physent_t, start: *mut vec_t, end: *mut vec_t)
     -> *mut msurface_t;
    #[no_mangle]
    fn R_AllocParticle(callback:
                           Option<unsafe extern "C" fn(_: *mut particle_s,
                                                       _: libc::c_float)
                                      -> ()>) -> *mut particle_s;
    #[no_mangle]
    fn R_Explosion(pos: *mut vec_t, model: libc::c_int, scale: libc::c_float,
                   framerate: libc::c_float, flags: libc::c_int);
    #[no_mangle]
    fn R_ParticleExplosion(org: *const vec_t);
    #[no_mangle]
    fn R_ParticleExplosion2(org: *const vec_t, colorStart: libc::c_int,
                            colorLength: libc::c_int);
    #[no_mangle]
    fn R_Implosion(end: *const vec_t, radius: libc::c_float,
                   count: libc::c_int, life: libc::c_float);
    #[no_mangle]
    fn R_Blood(org: *const vec_t, dir: *const vec_t, pcolor: libc::c_int,
               speed: libc::c_int);
    #[no_mangle]
    fn R_BloodStream(org: *const vec_t, dir: *const vec_t,
                     pcolor: libc::c_int, speed: libc::c_int);
    #[no_mangle]
    fn R_BlobExplosion(org: *const vec_t);
    #[no_mangle]
    fn R_EntityParticles(ent: *mut cl_entity_t);
    #[no_mangle]
    fn R_FlickerParticles(org: *const vec_t);
    #[no_mangle]
    fn R_RunParticleEffect(org: *const vec_t, dir: *const vec_t,
                           color: libc::c_int, count: libc::c_int);
    #[no_mangle]
    fn R_ParticleBurst(org: *const vec_t, size: libc::c_int,
                       color: libc::c_int, life: libc::c_float);
    #[no_mangle]
    fn R_LavaSplash(org: *const vec_t);
    #[no_mangle]
    fn R_TeleportSplash(org: *const vec_t);
    #[no_mangle]
    fn R_RocketTrail(start: *mut vec_t, end: *mut vec_t, type_0: libc::c_int);
    #[no_mangle]
    fn R_LookupColor(r: byte, g: byte, b: byte) -> libc::c_short;
    #[no_mangle]
    fn R_GetPackedColor(packed: *mut libc::c_short, color: libc::c_short);
    #[no_mangle]
    fn R_TracerEffect(start: *const vec_t, end: *const vec_t);
    #[no_mangle]
    fn R_UserTracerParticle(org: *mut libc::c_float, vel: *mut libc::c_float,
                            life: libc::c_float, colorIndex: libc::c_int,
                            length: libc::c_float, deathcontext: byte,
                            deathfunc:
                                Option<unsafe extern "C" fn(_:
                                                                *mut particle_s)
                                           -> ()>);
    #[no_mangle]
    fn R_TracerParticles(org: *mut libc::c_float, vel: *mut libc::c_float,
                         life: libc::c_float) -> *mut particle_s;
    #[no_mangle]
    fn R_ParticleLine(start: *const vec_t, end: *const vec_t, r: byte,
                      g: byte, b: byte, life: libc::c_float);
    #[no_mangle]
    fn R_ParticleBox(mins: *const vec_t, maxs: *const vec_t, r: byte, g: byte,
                     b: byte, life: libc::c_float);
    #[no_mangle]
    fn R_ShowLine(start: *const vec_t, end: *const vec_t);
    #[no_mangle]
    fn R_BulletImpactParticles(pos: *const vec_t);
    #[no_mangle]
    fn R_SparkShower(org: *const vec_t);
    #[no_mangle]
    fn CL_TempEntAlloc(org: *const vec_t, pmodel: *mut model_t)
     -> *mut tempent_s;
    #[no_mangle]
    fn CL_TempEntAllocHigh(org: *const vec_t, pmodel: *mut model_t)
     -> *mut tempent_s;
    #[no_mangle]
    fn CL_TempEntAllocNoModel(org: *const vec_t) -> *mut tempent_s;
    #[no_mangle]
    fn CL_TempEntAllocCustom(org: *const vec_t, model: *mut model_t,
                             high: libc::c_int,
                             callback:
                                 Option<unsafe extern "C" fn(_:
                                                                 *mut tempent_s,
                                                             _: libc::c_float,
                                                             _: libc::c_float)
                                            -> ()>) -> *mut tempent_s;
    #[no_mangle]
    fn R_FizzEffect(pent: *mut cl_entity_t, modelIndex: libc::c_int,
                    density: libc::c_int);
    #[no_mangle]
    fn R_Bubbles(mins: *const vec_t, maxs: *const vec_t,
                 height: libc::c_float, modelIndex: libc::c_int,
                 count: libc::c_int, speed: libc::c_float);
    #[no_mangle]
    fn R_BubbleTrail(start: *const vec_t, end: *const vec_t,
                     flWaterZ: libc::c_float, modelIndex: libc::c_int,
                     count: libc::c_int, speed: libc::c_float);
    #[no_mangle]
    fn R_AttachTentToPlayer(client: libc::c_int, modelIndex: libc::c_int,
                            zoffset: libc::c_float, life: libc::c_float);
    #[no_mangle]
    fn R_KillAttachedTents(client: libc::c_int);
    #[no_mangle]
    fn R_RicochetSprite(pos: *const vec_t, pmodel: *mut model_t,
                        duration: libc::c_float, scale: libc::c_float);
    #[no_mangle]
    fn R_RocketFlare(pos: *const vec_t);
    #[no_mangle]
    fn R_MuzzleFlash(pos: *const vec_t, type_0: libc::c_int);
    #[no_mangle]
    fn R_BloodSprite(org: *const vec_t, colorIndex: libc::c_int,
                     modelIndex: libc::c_int, modelIndex2: libc::c_int,
                     size: libc::c_float);
    #[no_mangle]
    fn R_BreakModel(pos: *const vec_t, size: *const vec_t, dir: *const vec_t,
                    random: libc::c_float, life: libc::c_float,
                    count: libc::c_int, modelIndex: libc::c_int,
                    flags: libc::c_char);
    #[no_mangle]
    fn R_TempModel(pos: *const vec_t, dir: *const vec_t, angles: *const vec_t,
                   life: libc::c_float, modelIndex: libc::c_int,
                   soundtype: libc::c_int) -> *mut tempent_s;
    #[no_mangle]
    fn R_TempSprite(pos: *mut vec_t, dir: *const vec_t, scale: libc::c_float,
                    modelIndex: libc::c_int, rendermode: libc::c_int,
                    renderfx: libc::c_int, a: libc::c_float,
                    life: libc::c_float, flags: libc::c_int)
     -> *mut tempent_s;
    #[no_mangle]
    fn R_DefaultSprite(pos: *const vec_t, spriteIndex: libc::c_int,
                       framerate: libc::c_float) -> *mut tempent_s;
    #[no_mangle]
    fn R_Sprite_Explode(pTemp: *mut tempent_s, scale: libc::c_float,
                        flags: libc::c_int);
    #[no_mangle]
    fn R_Sprite_Smoke(pTemp: *mut tempent_s, scale: libc::c_float);
    #[no_mangle]
    fn R_Spray(pos: *const vec_t, dir: *const vec_t, modelIndex: libc::c_int,
               count: libc::c_int, speed: libc::c_int, iRand: libc::c_int,
               renderMode: libc::c_int);
    #[no_mangle]
    fn R_Sprite_Spray(pos: *const vec_t, dir: *const vec_t,
                      modelIndex: libc::c_int, count: libc::c_int,
                      speed: libc::c_int, iRand: libc::c_int);
    #[no_mangle]
    fn R_Sprite_Trail(type_0: libc::c_int, vecStart: *mut vec_t,
                      vecEnd: *mut vec_t, modelIndex: libc::c_int,
                      nCount: libc::c_int, flLife: libc::c_float,
                      flSize: libc::c_float, flAmplitude: libc::c_float,
                      nRenderamt: libc::c_int, flSpeed: libc::c_float);
    #[no_mangle]
    fn R_FunnelSprite(pos: *const vec_t, spriteIndex: libc::c_int,
                      flags: libc::c_int);
    #[no_mangle]
    fn R_LargeFunnel(pos: *const vec_t, reverse: libc::c_int);
    #[no_mangle]
    fn R_SparkEffect(pos: *const vec_t, count: libc::c_int,
                     velocityMin: libc::c_int, velocityMax: libc::c_int);
    #[no_mangle]
    fn R_StreakSplash(pos: *const vec_t, dir: *const vec_t,
                      color: libc::c_int, count: libc::c_int,
                      speed: libc::c_float, velMin: libc::c_int,
                      velMax: libc::c_int);
    #[no_mangle]
    fn R_SparkStreaks(pos: *const vec_t, count: libc::c_int,
                      velocityMin: libc::c_int, velocityMax: libc::c_int);
    #[no_mangle]
    fn R_Projectile(origin: *const vec_t, velocity: *const vec_t,
                    modelIndex: libc::c_int, life: libc::c_int,
                    owner: libc::c_int,
                    hitcallback:
                        Option<unsafe extern "C" fn(_: *mut tempent_s,
                                                    _: *mut pmtrace_s)
                                   -> ()>);
    #[no_mangle]
    fn R_TempSphereModel(pos: *const vec_t, speed: libc::c_float,
                         life: libc::c_float, count: libc::c_int,
                         modelIndex: libc::c_int);
    #[no_mangle]
    fn R_MultiGunshot(org: *const vec_t, dir: *const vec_t,
                      noise: *const vec_t, count: libc::c_int,
                      decalCount: libc::c_int,
                      decalIndices: *mut libc::c_int);
    #[no_mangle]
    fn R_FireField(org: *mut libc::c_float, radius: libc::c_int,
                   modelIndex: libc::c_int, count: libc::c_int,
                   flags: libc::c_int, life: libc::c_float);
    #[no_mangle]
    fn R_PlayerSprites(client: libc::c_int, modelIndex: libc::c_int,
                       count: libc::c_int, size: libc::c_int);
    #[no_mangle]
    fn R_Sprite_WallPuff(pTemp: *mut tempent_s, scale: libc::c_float);
    #[no_mangle]
    fn R_RicochetSound(pos: *const vec_t);
    #[no_mangle]
    fn CL_AllocDlight(key: libc::c_int) -> *mut dlight_s;
    #[no_mangle]
    fn CL_AllocElight(key: libc::c_int) -> *mut dlight_s;
    #[no_mangle]
    fn CL_DecalRemoveAll(textureIndex: libc::c_int);
    #[no_mangle]
    fn CL_DecalIndexFromName(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn CL_DecalIndex(id: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn R_BeamLightning(start: *mut vec_t, end: *mut vec_t,
                       modelIndex: libc::c_int, life: libc::c_float,
                       width: libc::c_float, amplitude: libc::c_float,
                       brightness: libc::c_float, speed: libc::c_float)
     -> *mut beam_s;
    #[no_mangle]
    fn R_BeamEnts(startEnt: libc::c_int, endEnt: libc::c_int,
                  modelIndex: libc::c_int, life: libc::c_float,
                  width: libc::c_float, amplitude: libc::c_float,
                  brightness: libc::c_float, speed: libc::c_float,
                  startFrame: libc::c_int, framerate: libc::c_float,
                  r: libc::c_float, g: libc::c_float, b: libc::c_float)
     -> *mut beam_s;
    #[no_mangle]
    fn R_BeamPoints(start: *mut vec_t, end: *mut vec_t,
                    modelIndex: libc::c_int, life: libc::c_float,
                    width: libc::c_float, amplitude: libc::c_float,
                    brightness: libc::c_float, speed: libc::c_float,
                    startFrame: libc::c_int, framerate: libc::c_float,
                    r: libc::c_float, g: libc::c_float, b: libc::c_float)
     -> *mut beam_s;
    #[no_mangle]
    fn R_BeamCirclePoints(type_0: libc::c_int, start: *mut vec_t,
                          end: *mut vec_t, modelIndex: libc::c_int,
                          life: libc::c_float, width: libc::c_float,
                          amplitude: libc::c_float, brightness: libc::c_float,
                          speed: libc::c_float, startFrame: libc::c_int,
                          framerate: libc::c_float, r: libc::c_float,
                          g: libc::c_float, b: libc::c_float) -> *mut beam_s;
    #[no_mangle]
    fn R_BeamEntPoint(startEnt: libc::c_int, end: *mut vec_t,
                      modelIndex: libc::c_int, life: libc::c_float,
                      width: libc::c_float, amplitude: libc::c_float,
                      brightness: libc::c_float, speed: libc::c_float,
                      startFrame: libc::c_int, framerate: libc::c_float,
                      r: libc::c_float, g: libc::c_float, b: libc::c_float)
     -> *mut beam_s;
    #[no_mangle]
    fn R_BeamRing(startEnt: libc::c_int, endEnt: libc::c_int,
                  modelIndex: libc::c_int, life: libc::c_float,
                  width: libc::c_float, amplitude: libc::c_float,
                  brightness: libc::c_float, speed: libc::c_float,
                  startFrame: libc::c_int, framerate: libc::c_float,
                  r: libc::c_float, g: libc::c_float, b: libc::c_float)
     -> *mut beam_s;
    #[no_mangle]
    fn R_BeamFollow(startEnt: libc::c_int, modelIndex: libc::c_int,
                    life: libc::c_float, width: libc::c_float,
                    r: libc::c_float, g: libc::c_float, b: libc::c_float,
                    brightness: libc::c_float) -> *mut beam_s;
    #[no_mangle]
    fn R_BeamKill(deadEntity: libc::c_int);
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
    fn VGui_Startup(clientlib: *const libc::c_char, width: libc::c_int,
                    height: libc::c_int);
    #[no_mangle]
    fn VGui_Shutdown();
    #[no_mangle]
    fn VGui_GetPanel() -> *mut libc::c_void;
    #[no_mangle]
    fn S_GetSfxByHandle(handle: sound_t) -> *mut sfx_t;
}
pub type __uint8_t = libc::c_uchar;
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
pub type fs_offset_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dllfunc_s {
    pub name: *const libc::c_char,
    pub func: *mut *mut libc::c_void,
}
pub type dllfunc_t = dllfunc_s;
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
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
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
pub struct IVoiceTweak_s {
    pub StartVoiceTweakMode: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub EndVoiceTweakMode: Option<unsafe extern "C" fn() -> ()>,
    pub SetControlFloat: Option<unsafe extern "C" fn(_: VoiceTweakControl,
                                                     _: libc::c_float) -> ()>,
    pub GetControlFloat: Option<unsafe extern "C" fn(_: VoiceTweakControl)
                                    -> libc::c_float>,
}
pub type VoiceTweakControl = libc::c_uint;
pub const OtherSpeakerScale: VoiceTweakControl = 1;
pub const MicrophoneVolume: VoiceTweakControl = 0;
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
pub struct demo_api_s {
    pub IsRecording: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub IsPlayingback: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub IsTimeDemo: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub WriteBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *mut libc::c_uchar)
                                -> ()>,
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
pub struct pred_viewangle_t {
    pub starttime: libc::c_float,
    pub total: libc::c_float,
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
pub type local_state_t = local_state_s;
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
pub type connstate_t = connstate_e;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
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
pub type HIMAGE = libc::c_int;
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
pub type IVoiceTweak = IVoiceTweak_s;
pub type net_api_t = net_api_s;
pub type net_status_t = net_status_s;
pub type demo_api_t = demo_api_s;
pub type event_api_t = event_api_s;
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
pub type pfnIgnore
    =
    Option<unsafe extern "C" fn(_: *mut physent_t) -> libc::c_int>;
pub type efx_api_t = efx_api_s;
pub type triangleapi_t = triangleapi_s;
pub type CL_EXPORT_FUNCS
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type C2RustUnnamed_0 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_0 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_0 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_0 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_0 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_0 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_0 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_0 = -1;
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
pub type world_static_t = world_static_s;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CL_CHANGELEVEL: C2RustUnnamed_1 = 4;
pub const CL_PAUSED: C2RustUnnamed_1 = 3;
pub const CL_ACTIVE: C2RustUnnamed_1 = 2;
pub const CL_LOADING: C2RustUnnamed_1 = 1;
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_double) -> libc::c_double {
    return sin(__x);
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
#[no_mangle]
pub static mut cl_textbuffer: [[libc::c_char; 2048]; 8] = [[0; 2048]; 8];
#[no_mangle]
pub static mut cl_textmessage: [client_textmessage_t; 8] =
    [client_textmessage_t{effect: 0,
                          r1: 0,
                          g1: 0,
                          b1: 0,
                          a1: 0,
                          r2: 0,
                          g2: 0,
                          b2: 0,
                          a2: 0,
                          x: 0.,
                          y: 0.,
                          fadein: 0.,
                          fadeout: 0.,
                          holdtime: 0.,
                          fxtime: 0.,
                          pName: 0 as *const libc::c_char,
                          pMessage: 0 as *const libc::c_char,}; 8];
// Initialized in run_static_initializers
static mut cdll_exports: [dllfunc_t; 38] =
    [dllfunc_t{name: 0 as *const libc::c_char,
               func: 0 as *mut *mut libc::c_void,}; 38];
// optional exports
// Initialized in run_static_initializers
static mut cdll_new_exports: [dllfunc_t; 10] =
    [dllfunc_t{name: 0 as *const libc::c_char,
               func: 0 as *mut *mut libc::c_void,}; 10];
/*
====================
CL_GetEntityByIndex

Render callback for studio models
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetEntityByIndex(mut index: libc::c_int)
 -> *mut cl_entity_s {
    if clgame.entities.is_null() {
        // not in game yet
        return 0 as *mut cl_entity_s
    }
    if index < 0 as libc::c_int || index >= clgame.maxEntities {
        return 0 as *mut cl_entity_s
    }
    if index == 0 as libc::c_int { return clgame.entities }
    return CL_EDICT_NUM(index);
}
/*
================
CL_ModelHandle

get model handle by index
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ModelHandle(mut modelindex: libc::c_int)
 -> *mut model_t {
    if modelindex < 0 as libc::c_int || modelindex >= 1024 as libc::c_int {
        return 0 as *mut model_t
    }
    return cl.models[modelindex as usize];
}
/*
====================
CL_IsThirdPerson

returns true if thirdperson is enabled
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_IsThirdPerson() -> qboolean {
    cl.local.thirdperson =
        clgame.dllFuncs.CL_IsThirdPerson.expect("non-null function pointer")()
            as qboolean;
    if cl.local.thirdperson as u64 != 0 { return true_0 }
    return false_0;
}
/*
====================
CL_GetPlayerInfo

get player info by render request
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetPlayerInfo(mut playerIndex: libc::c_int)
 -> *mut player_info_s {
    if playerIndex < 0 as libc::c_int || playerIndex >= cl.maxclients {
        return 0 as *mut player_info_s
    }
    return &mut *cl.players.as_mut_ptr().offset(playerIndex as isize) as
               *mut player_info_t;
}
/*
====================
CL_CreatePlaylist

Create a default valve playlist
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CreatePlaylist(mut filename:
                                               *const libc::c_char) {
    let mut f: *mut file_t = 0 as *mut file_t;
    f =
        FS_Open(filename, b"w\x00" as *const u8 as *const libc::c_char,
                false_0);
    if f.is_null() { return }
    // make standard cdaudio playlist
    FS_Print(f, b"blank\n\x00" as *const u8 as *const libc::c_char); // #1
    FS_Print(f,
             b"Half-Life01.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #2
    FS_Print(f,
             b"Prospero01.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #3
    FS_Print(f,
             b"Half-Life12.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #4
    FS_Print(f,
             b"Half-Life07.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #5
    FS_Print(f,
             b"Half-Life10.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #6
    FS_Print(f,
             b"Suspense01.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #7
    FS_Print(f,
             b"Suspense03.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #8
    FS_Print(f,
             b"Half-Life09.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #9
    FS_Print(f,
             b"Half-Life02.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #10
    FS_Print(f,
             b"Half-Life13.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #11
    FS_Print(f,
             b"Half-Life04.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #12
    FS_Print(f,
             b"Half-Life15.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #13
    FS_Print(f,
             b"Half-Life14.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #14
    FS_Print(f,
             b"Half-Life16.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #15
    FS_Print(f,
             b"Suspense02.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #16
    FS_Print(f,
             b"Half-Life03.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #17
    FS_Print(f,
             b"Half-Life08.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #18
    FS_Print(f,
             b"Prospero02.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #19
    FS_Print(f,
             b"Half-Life05.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #20
    FS_Print(f,
             b"Prospero04.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #21
    FS_Print(f,
             b"Half-Life11.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #22
    FS_Print(f,
             b"Half-Life06.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #23
    FS_Print(f,
             b"Prospero03.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #24
    FS_Print(f,
             b"Half-Life17.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #25
    FS_Print(f,
             b"Prospero05.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #26
    FS_Print(f,
             b"Suspense05.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #27
    FS_Print(f,
             b"Suspense07.mp3\n\x00" as *const u8 as
                 *const libc::c_char); // #28
    FS_Close(f);
}
/*
====================
CL_InitCDAudio

Initialize CD playlist
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitCDAudio(mut filename: *const libc::c_char) {
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: string = [0; 256];
    let mut c: libc::c_int = 0 as libc::c_int;
    if FS_FileExists(filename, false_0 as libc::c_int) == 0 {
        // create a default playlist
        CL_CreatePlaylist(filename);
    }
    afile = FS_LoadFile(filename, 0 as *mut fs_offset_t, false_0);
    if afile.is_null() { return }
    pfile = afile as *mut libc::c_char;
    loop 
         // format: trackname\n [num]
         {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        if Q_strnicmp(token.as_mut_ptr(),
                      b"blank\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            token[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
        }
        Q_strncpy(clgame.cdtracks[c as usize].as_mut_ptr(),
                  token.as_mut_ptr(),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        c += 1;
        if !(c > 32 as libc::c_int - 1 as libc::c_int) { continue ; }
        Con_Reportf(b"^3Warning:^7 CD_Init: too many tracks %i in %s\n\x00" as
                        *const u8 as *const libc::c_char, 32 as libc::c_int,
                    filename);
        break ;
    }
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/client/cl_game.c\x00" as *const u8 as
                  *const libc::c_char, 249 as libc::c_int);
}
/*
====================
CL_PointContents

Return contents for point
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PointContents(mut p: *const vec_t)
 -> libc::c_int {
    let mut cont: libc::c_int = PM_PointContents(clgame.pmove, p);
    if cont <= -(9 as libc::c_int) && cont >= -(14 as libc::c_int) {
        cont = -(3 as libc::c_int)
    }
    return cont;
}
/*
=============
CL_AdjustXPos

adjust text by x pos
=============
*/
unsafe extern "C" fn CL_AdjustXPos(mut x: libc::c_float,
                                   mut width: libc::c_int,
                                   mut totalWidth: libc::c_int)
 -> libc::c_int {
    let mut xPos: libc::c_int = 0; // Alight right
    if x == -(1 as libc::c_int) as libc::c_float {
        xPos =
            ((refState.width - width) as libc::c_float * 0.5f32) as
                libc::c_int
    } else if x < 0 as libc::c_int as libc::c_float {
        xPos =
            ((1.0f32 + x) * refState.width as libc::c_float -
                 totalWidth as libc::c_float) as libc::c_int
    } else {
        // align left
        xPos = (x * refState.width as libc::c_float) as libc::c_int
    }
    if xPos + width > refState.width {
        xPos = refState.width - width
    } else if xPos < 0 as libc::c_int { xPos = 0 as libc::c_int }
    return xPos;
}
/*
=============
CL_AdjustYPos

adjust text by y pos
=============
*/
unsafe extern "C" fn CL_AdjustYPos(mut y: libc::c_float,
                                   mut height: libc::c_int) -> libc::c_int {
    let mut yPos: libc::c_int = 0;
    if y == -(1 as libc::c_int) as libc::c_float {
        // centered?
        yPos =
            ((refState.height - height) as libc::c_float * 0.5f32) as
                libc::c_int
    } else if y < 0 as libc::c_int as libc::c_float {
        // Alight bottom?
        yPos =
            ((1.0f32 + y) * refState.height as libc::c_float -
                 height as libc::c_float) as libc::c_int
    } else { // Alight bottom
        // align top
        yPos = (y * refState.height as libc::c_float) as libc::c_int
    }
    if yPos + height > refState.height {
        yPos = refState.height - height
    } else if yPos < 0 as libc::c_int { yPos = 0 as libc::c_int }
    return yPos;
}
/*
=============
CL_CenterPrint

print centerscreen message
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CenterPrint(mut text: *const libc::c_char,
                                        mut y: libc::c_float) {
    let mut length: libc::c_int =
        0 as libc::c_int; // allow pause for centerprint
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if if text.is_null() || *text == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    clgame.centerPrint.lines = 1 as libc::c_int;
    clgame.centerPrint.totalWidth = 0 as libc::c_int;
    clgame.centerPrint.time =
        cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    Q_strncpy(clgame.centerPrint.message.as_mut_ptr(), text,
              ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong);
    s = clgame.centerPrint.message.as_mut_ptr();
    // count the number of lines for centering
    while *s != 0 {
        if *s as libc::c_int == '\n' as i32 {
            clgame.centerPrint.lines += 1;
            if width > clgame.centerPrint.totalWidth {
                clgame.centerPrint.totalWidth = width
            }
            width = 0 as libc::c_int
        } else {
            width += clgame.scrInfo.charWidths[*s as usize] as libc::c_int
        }
        s = s.offset(1);
        length += 1
    }
    clgame.centerPrint.totalHeight =
        clgame.centerPrint.lines * clgame.scrInfo.iCharHeight;
    clgame.centerPrint.y = CL_AdjustYPos(y, clgame.centerPrint.totalHeight);
}
/*
====================
SPR_AdjustSize

draw hudsprite routine
====================
*/
unsafe extern "C" fn SPR_AdjustSize(mut x: *mut libc::c_float,
                                    mut y: *mut libc::c_float,
                                    mut w: *mut libc::c_float,
                                    mut h: *mut libc::c_float) {
    let mut xscale: libc::c_float = 0.;
    let mut yscale: libc::c_float = 0.;
    // scale for screen sizes
    xscale =
        refState.width as libc::c_float /
            clgame.scrInfo.iWidth as libc::c_float;
    yscale =
        refState.height as libc::c_float /
            clgame.scrInfo.iHeight as libc::c_float;
    if !x.is_null() { *x *= xscale }
    if !y.is_null() { *y *= yscale }
    if !w.is_null() { *w *= xscale }
    if !h.is_null() { *h *= yscale };
}
/*
====================
SPR_AdjustSize

draw hudsprite routine
====================
*/
unsafe extern "C" fn SPR_AdjustSizei(mut x: *mut libc::c_int,
                                     mut y: *mut libc::c_int,
                                     mut w: *mut libc::c_int,
                                     mut h: *mut libc::c_int) {
    let mut xscale: libc::c_float = 0.;
    let mut yscale: libc::c_float = 0.;
    // scale for screen sizes
    xscale =
        refState.width as libc::c_float /
            clgame.scrInfo.iWidth as libc::c_float;
    yscale =
        refState.height as libc::c_float /
            clgame.scrInfo.iHeight as libc::c_float;
    if !x.is_null() { *x = (*x as libc::c_float * xscale) as libc::c_int }
    if !y.is_null() { *y = (*y as libc::c_float * yscale) as libc::c_int }
    if !w.is_null() { *w = (*w as libc::c_float * xscale) as libc::c_int }
    if !h.is_null() { *h = (*h as libc::c_float * yscale) as libc::c_int };
}
/*
====================
PictAdjustSize

draw hudsprite routine
====================
*/
#[no_mangle]
pub unsafe extern "C" fn PicAdjustSize(mut x: *mut libc::c_float,
                                       mut y: *mut libc::c_float,
                                       mut w: *mut libc::c_float,
                                       mut h: *mut libc::c_float) {
    if clgame.ds.adjust_size as u64 == 0 { return }
    SPR_AdjustSize(x, y, w, h);
}
unsafe extern "C" fn SPR_Scissor(mut x: *mut libc::c_float,
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
    if *x + *width <= clgame.ds.scissor_x as libc::c_float { return false_0 }
    if *x >= (clgame.ds.scissor_x + clgame.ds.scissor_width) as libc::c_float
       {
        return false_0
    }
    if *y + *height <= clgame.ds.scissor_y as libc::c_float { return false_0 }
    if *y >= (clgame.ds.scissor_y + clgame.ds.scissor_height) as libc::c_float
       {
        return false_0
    }
    dudx = (*u1 - *u0) / *width;
    dvdy = (*v1 - *v0) / *height;
    if *x < clgame.ds.scissor_x as libc::c_float {
        *u0 += (clgame.ds.scissor_x as libc::c_float - *x) * dudx;
        *width -= clgame.ds.scissor_x as libc::c_float - *x;
        *x = clgame.ds.scissor_x as libc::c_float
    }
    if *x + *width >
           (clgame.ds.scissor_x + clgame.ds.scissor_width) as libc::c_float {
        *u1 -=
            (*x + *width -
                 (clgame.ds.scissor_x + clgame.ds.scissor_width) as
                     libc::c_float) * dudx;
        *width =
            (clgame.ds.scissor_x + clgame.ds.scissor_width) as libc::c_float -
                *x
    }
    if *y < clgame.ds.scissor_y as libc::c_float {
        *v0 += (clgame.ds.scissor_y as libc::c_float - *y) * dvdy;
        *height -= clgame.ds.scissor_y as libc::c_float - *y;
        *y = clgame.ds.scissor_y as libc::c_float
    }
    if *y + *height >
           (clgame.ds.scissor_y + clgame.ds.scissor_height) as libc::c_float {
        *v1 -=
            (*y + *height -
                 (clgame.ds.scissor_y + clgame.ds.scissor_height) as
                     libc::c_float) * dvdy;
        *height =
            (clgame.ds.scissor_y + clgame.ds.scissor_height) as libc::c_float
                - *y
    }
    return true_0;
}
/*
====================
SPR_DrawGeneric

draw hudsprite routine
====================
*/
unsafe extern "C" fn SPR_DrawGeneric(mut frame: libc::c_int,
                                     mut x: libc::c_float,
                                     mut y: libc::c_float,
                                     mut width: libc::c_float,
                                     mut height: libc::c_float,
                                     mut prc: *const wrect_t) {
    let mut s1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut texnum: libc::c_int = 0;
    if width == -(1 as libc::c_int) as libc::c_float &&
           height == -(1 as libc::c_int) as libc::c_float {
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        // assume we get sizes from image
        ref_0.dllFuncs.R_GetSpriteParms.expect("non-null function pointer")(&mut w,
                                                                            &mut h,
                                                                            0
                                                                                as
                                                                                *mut libc::c_int,
                                                                            frame,
                                                                            clgame.ds.pSprite);
        width = w as libc::c_float;
        height = h as libc::c_float
    }
    if !prc.is_null() {
        let mut rc: wrect_t = wrect_t{left: 0, right: 0, top: 0, bottom: 0,};
        rc = *prc;
        // Sigh! some stupid modmakers set wrong rectangles in hud.txt
        if rc.left <= 0 as libc::c_int || rc.left as libc::c_float >= width {
            rc.left = 0 as libc::c_int
        }
        if rc.top <= 0 as libc::c_int || rc.top as libc::c_float >= height {
            rc.top = 0 as libc::c_int
        }
        if rc.right <= 0 as libc::c_int || rc.right as libc::c_float > width {
            rc.right = width as libc::c_int
        }
        if rc.bottom <= 0 as libc::c_int ||
               rc.bottom as libc::c_float > height {
            rc.bottom = height as libc::c_int
        }
        // calc user-defined rectangle
        s1 = rc.left as libc::c_float / width;
        t1 = rc.top as libc::c_float / height;
        s2 = rc.right as libc::c_float / width;
        t2 = rc.bottom as libc::c_float / height;
        width = (rc.right - rc.left) as libc::c_float;
        height = (rc.bottom - rc.top) as libc::c_float
    } else { t1 = 0.0f32; s1 = t1; t2 = 1.0f32; s2 = t2 }
    // pass scissor test if supposed
    if clgame.ds.scissor_test as libc::c_uint != 0 &&
           SPR_Scissor(&mut x, &mut y, &mut width, &mut height, &mut s1,
                       &mut t1, &mut s2, &mut t2) as u64 == 0 {
        return
    }
    // scale for screen sizes
    SPR_AdjustSize(&mut x, &mut y, &mut width, &mut height);
    texnum =
        ref_0.dllFuncs.R_GetSpriteTexture.expect("non-null function pointer")(clgame.ds.pSprite,
                                                                              frame);
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(clgame.ds.spriteColor[0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize],
                                                                clgame.ds.spriteColor[1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize],
                                                                clgame.ds.spriteColor[2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize],
                                                                clgame.ds.spriteColor[3
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          usize]);
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(x, y,
                                                                        width,
                                                                        height,
                                                                        s1,
                                                                        t1,
                                                                        s2,
                                                                        t2,
                                                                        texnum);
}
/*
=============
CL_DrawCenterPrint

called each frame
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DrawCenterPrint() {
    let mut pText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut lineLength: libc::c_int = 0;
    let mut colorDefault: *mut byte = 0 as *mut byte;
    let mut line: [byte; 80] = [0; 80];
    let mut charWidth: libc::c_int = 0;
    let mut charHeight: libc::c_int = 0;
    if clgame.centerPrint.time == 0. { return }
    if cl.time - clgame.centerPrint.time as libc::c_double >=
           (*scr_centertime).value as libc::c_double {
        // time expired
        clgame.centerPrint.time = 0.0f32; // start y
        return
    } // Skip LineFeed
    y = clgame.centerPrint.y;
    colorDefault = g_color_table[7 as libc::c_int as usize].as_mut_ptr();
    pText = clgame.centerPrint.message.as_mut_ptr();
    Con_DrawCharacterLen(0 as libc::c_int, 0 as *mut libc::c_int,
                         &mut charHeight);
    i = 0 as libc::c_int;
    while i < clgame.centerPrint.lines {
        lineLength = 0 as libc::c_int;
        width = 0 as libc::c_int;
        while *pText as libc::c_int != 0 &&
                  *pText as libc::c_int != '\n' as i32 &&
                  lineLength < 80 as libc::c_int {
            let mut c: byte = *pText as byte;
            line[lineLength as usize] = c;
            Con_DrawCharacterLen(c as libc::c_int, &mut charWidth,
                                 0 as *mut libc::c_int);
            width += charWidth;
            lineLength += 1;
            pText = pText.offset(1)
        }
        if lineLength == 80 as libc::c_int { lineLength -= 1 }
        pText = pText.offset(1);
        line[lineLength as usize] = 0 as libc::c_int as byte;
        x =
            CL_AdjustXPos(-(1 as libc::c_int) as libc::c_float, width,
                          clgame.centerPrint.totalWidth);
        j = 0 as libc::c_int;
        while j < lineLength {
            if x >= 0 as libc::c_int && y >= 0 as libc::c_int &&
                   x <= refState.width {
                x +=
                    Con_DrawCharacter(x, y, line[j as usize] as libc::c_int,
                                      colorDefault)
            }
            j += 1
        }
        y += charHeight;
        i += 1
    };
}
/*
=============
CL_DrawScreenFade

fill screen with specfied color
can be modulated
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DrawScreenFade() {
    let mut sf: *mut screenfade_t = &mut clgame.fade;
    let mut iFadeAlpha: libc::c_int = 0;
    let mut testFlags: libc::c_int = 0;
    // keep pushing reset time out indefinitely
    if (*sf).fadeFlags & 0x4 as libc::c_int != 0 {
        (*sf).fadeReset =
            (cl.time + 0.1f32 as libc::c_double) as libc::c_float
    } // inactive
    if (*sf).fadeReset == 0.0f32 && (*sf).fadeEnd == 0.0f32 { return }
    // all done?
    if cl.time > (*sf).fadeReset as libc::c_double &&
           cl.time > (*sf).fadeEnd as libc::c_double {
        memset(&mut clgame.fade as *mut screenfade_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<screenfade_t>() as libc::c_ulong);
        return
    }
    testFlags = (*sf).fadeFlags & !(0x2 as libc::c_int);
    // fading...
    if testFlags == 0x4 as libc::c_int {
        iFadeAlpha = (*sf).fadealpha as libc::c_int
    } else {
        iFadeAlpha =
            ((*sf).fadeSpeed as libc::c_double *
                 ((*sf).fadeEnd as libc::c_double - cl.time)) as libc::c_int;
        if (*sf).fadeFlags & 0x1 as libc::c_int != 0 {
            iFadeAlpha += (*sf).fadealpha as libc::c_int
        }
        iFadeAlpha =
            if iFadeAlpha >= 0 as libc::c_int {
                if iFadeAlpha < (*sf).fadealpha as libc::c_int {
                    iFadeAlpha
                } else { (*sf).fadealpha as libc::c_int }
            } else { 0 as libc::c_int }
    }
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")((*sf).fader,
                                                                (*sf).fadeg,
                                                                (*sf).fadeb,
                                                                iFadeAlpha as
                                                                    libc::c_uchar);
    if (*sf).fadeFlags & 0x2 as libc::c_int != 0 {
        ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransAdd
                                                                                as
                                                                                libc::c_int);
    } else {
        ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                                as
                                                                                libc::c_int);
    }
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        refState.width
                                                                            as
                                                                            libc::c_float,
                                                                        refState.height
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
====================
CL_InitTitles

parse all messages that declared in titles.txt
and hold them into permament memory pool
====================
*/
unsafe extern "C" fn CL_InitTitles(mut filename: *const libc::c_char) {
    let mut fileSize: fs_offset_t = 0;
    let mut pMemFile: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    // initialize text messages (game_text)
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        cl_textmessage[i as usize].pName =
            _copystring(clgame.mempool,
                        va(b"TextMessage%i\x00" as *const u8 as
                               *const libc::c_char, i),
                        b"../engine/client/cl_game.c\x00" as *const u8 as
                            *const libc::c_char, 667 as libc::c_int);
        cl_textmessage[i as usize].pMessage =
            cl_textbuffer[i as usize].as_mut_ptr();
        i += 1
    }
    // clear out any old data that's sitting around.
    if !clgame.titles.is_null() {
        _Mem_Free(clgame.titles as *mut libc::c_void,
                  b"../engine/client/cl_game.c\x00" as *const u8 as
                      *const libc::c_char, 672 as libc::c_int);
    }
    clgame.titles = 0 as *mut client_textmessage_t;
    clgame.numTitles = 0 as libc::c_int;
    pMemFile = FS_LoadFile(filename, &mut fileSize, false_0);
    if pMemFile.is_null() { return }
    CL_TextMessageParse(pMemFile, fileSize as libc::c_int);
    _Mem_Free(pMemFile as *mut libc::c_void,
              b"../engine/client/cl_game.c\x00" as *const u8 as
                  *const libc::c_char, 681 as libc::c_int);
}
/*
====================
CL_HudMessage

Template to show hud messages
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_HudMessage(mut pMessage: *const libc::c_char) {
    if if pMessage.is_null() || *pMessage == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    CL_DispatchUserMessage(b"HudText\x00" as *const u8 as *const libc::c_char,
                           Q_strlen(pMessage) as libc::c_int,
                           pMessage as *mut libc::c_void);
}
/*
====================
CL_ParseTextMessage

Parse TE_TEXTMESSAGE
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseTextMessage(mut msg: *mut sizebuf_t) {
    static mut msgindex: libc::c_int = 0 as libc::c_int;
    let mut text: *mut client_textmessage_t = 0 as *mut client_textmessage_t;
    let mut channel: libc::c_int = 0;
    // read channel ( 0 - auto)
    channel = MSG_ReadByte(msg);
    if channel <= 0 as libc::c_int ||
           channel > 8 as libc::c_int - 1 as libc::c_int {
        channel = msgindex;
        msgindex =
            msgindex + 1 as libc::c_int & 8 as libc::c_int - 1 as libc::c_int
    }
    // grab message channel
    text =
        &mut *cl_textmessage.as_mut_ptr().offset(channel as isize) as
            *mut client_textmessage_t;
    (*text).x = MSG_ReadShort(msg) as libc::c_float / 8192.0f32;
    (*text).y = MSG_ReadShort(msg) as libc::c_float / 8192.0f32;
    (*text).effect = MSG_ReadByte(msg);
    (*text).r1 = MSG_ReadByte(msg) as byte;
    (*text).g1 = MSG_ReadByte(msg) as byte;
    (*text).b1 = MSG_ReadByte(msg) as byte;
    (*text).a1 = MSG_ReadByte(msg) as byte;
    (*text).r2 = MSG_ReadByte(msg) as byte;
    (*text).g2 = MSG_ReadByte(msg) as byte;
    (*text).b2 = MSG_ReadByte(msg) as byte;
    (*text).a2 = MSG_ReadByte(msg) as byte;
    (*text).fadein = MSG_ReadShort(msg) as libc::c_float / 256.0f32;
    (*text).fadeout = MSG_ReadShort(msg) as libc::c_float / 256.0f32;
    (*text).holdtime = MSG_ReadShort(msg) as libc::c_float / 256.0f32;
    if (*text).effect == 2 as libc::c_int {
        (*text).fxtime = MSG_ReadShort(msg) as libc::c_float / 256.0f32
    } else { (*text).fxtime = 0.0f32 }
    // to prevent grab too long messages
    Q_strncpy((*text).pMessage as *mut libc::c_char,
              MSG_ReadStringExt(msg, false_0), 2048 as libc::c_int as size_t);
    CL_HudMessage((*text).pName);
}
/*
================
CL_ParseFinaleCutscene

show display finale or cutscene message
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseFinaleCutscene(mut msg: *mut sizebuf_t,
                                                mut level: libc::c_int) {
    static mut msgindex: libc::c_int = 0 as libc::c_int;
    let mut text: *mut client_textmessage_t = 0 as *mut client_textmessage_t;
    let mut channel: libc::c_int = 0;
    cl.intermission = level;
    channel = msgindex;
    msgindex =
        msgindex + 1 as libc::c_int & 8 as libc::c_int - 1 as libc::c_int;
    // grab message channel
    text =
        &mut *cl_textmessage.as_mut_ptr().offset(channel as isize) as
            *mut client_textmessage_t;
    // NOTE: svc_finale and svc_cutscene has a
	// predefined settings like Quake-style
    (*text).x = -1.0f32; // scan out effect
    (*text).y = 0.15f32; // unused
    (*text).effect = 2 as libc::c_int;
    (*text).r1 = 245 as libc::c_int as byte;
    (*text).g1 = 245 as libc::c_int as byte;
    (*text).b1 = 245 as libc::c_int as byte;
    (*text).a1 = 0 as libc::c_int as byte;
    (*text).r2 = 0 as libc::c_int as byte;
    (*text).g2 = 0 as libc::c_int as byte;
    (*text).b2 = 0 as libc::c_int as byte;
    (*text).a2 = 0 as libc::c_int as byte;
    (*text).fadein = 0.15f32;
    (*text).fadeout = 0.0f32;
    (*text).holdtime = 99999.0f32;
    (*text).fxtime = 0.0f32;
    // to prevent grab too long messages
    Q_strncpy((*text).pMessage as *mut libc::c_char,
              MSG_ReadStringExt(msg, false_0),
              2048 as libc::c_int as size_t); // no real text
    if *(*text).pMessage as libc::c_int == '\u{0}' as i32 { return }
    CL_HudMessage((*text).pName);
}
/*
====================
CL_GetLocalPlayer

Render callback for studio models
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetLocalPlayer() -> *mut cl_entity_t {
    let mut player: *mut cl_entity_t = 0 as *mut cl_entity_t;
    player = CL_EDICT_NUM(cl.playernum + 1 as libc::c_int);
    return player;
}
/*
====================
CL_GetMaxlients

Render callback for studio models
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetMaxClients() -> libc::c_int {
    return cl.maxclients;
}
/*
====================
CL_SoundFromIndex

return soundname from index
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SoundFromIndex(mut index: libc::c_int)
 -> *const libc::c_char {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut hSound: libc::c_int = 0;
    // make sure what we in-bounds
    index =
        if index >= 0 as libc::c_int {
            if index < (1 as libc::c_int) << 11 as libc::c_int {
                index
            } else { ((1 as libc::c_int)) << 11 as libc::c_int }
        } else { 0 as libc::c_int };
    hSound = cl.sound_index[index as usize] as libc::c_int;
    if hSound == 0 {
        Con_DPrintf(b"^1Error:^7 CL_SoundFromIndex: invalid sound index %i\n\x00"
                        as *const u8 as *const libc::c_char, index);
        return 0 as *const libc::c_char
    }
    sfx = S_GetSfxByHandle(hSound);
    if sfx.is_null() {
        Con_DPrintf(b"^1Error:^7 CL_SoundFromIndex: bad sfx for index %i\n\x00"
                        as *const u8 as *const libc::c_char, index);
        return 0 as *const libc::c_char
    }
    return (*sfx).name.as_mut_ptr();
}
/*
=========
SPR_EnableScissor

=========
*/
unsafe extern "C" fn SPR_EnableScissor(mut x: libc::c_int, mut y: libc::c_int,
                                       mut width: libc::c_int,
                                       mut height: libc::c_int) {
    // check bounds
    x =
        if x >= 0 as libc::c_int {
            if x < clgame.scrInfo.iWidth { x } else { clgame.scrInfo.iWidth }
        } else { 0 as libc::c_int };
    y =
        if y >= 0 as libc::c_int {
            if y < clgame.scrInfo.iHeight {
                y
            } else { clgame.scrInfo.iHeight }
        } else { 0 as libc::c_int };
    width =
        if width >= 0 as libc::c_int {
            if width < clgame.scrInfo.iWidth - x {
                width
            } else { (clgame.scrInfo.iWidth) - x }
        } else { 0 as libc::c_int };
    height =
        if height >= 0 as libc::c_int {
            if height < clgame.scrInfo.iHeight - y {
                height
            } else { (clgame.scrInfo.iHeight) - y }
        } else { 0 as libc::c_int };
    clgame.ds.scissor_x = x;
    clgame.ds.scissor_width = width;
    clgame.ds.scissor_y = y;
    clgame.ds.scissor_height = height;
    clgame.ds.scissor_test = true_0;
}
/*
=========
SPR_DisableScissor

=========
*/
unsafe extern "C" fn SPR_DisableScissor() {
    clgame.ds.scissor_x = 0 as libc::c_int;
    clgame.ds.scissor_width = 0 as libc::c_int;
    clgame.ds.scissor_y = 0 as libc::c_int;
    clgame.ds.scissor_height = 0 as libc::c_int;
    clgame.ds.scissor_test = false_0;
}
/*
====================
CL_DrawCrosshair

Render crosshair
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DrawCrosshair() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut xscale: libc::c_float = 0.;
    let mut yscale: libc::c_float = 0.;
    if clgame.ds.pCrosshair.is_null() || (*cl_crosshair).value == 0. {
        return
    }
    // any camera on or client is died
    if cl.local.health <= 0 as libc::c_int ||
           cl.viewentity != cl.playernum + 1 as libc::c_int {
        return
    }
    // get crosshair dimension
    width = clgame.ds.rcCrosshair.right - clgame.ds.rcCrosshair.left;
    height = clgame.ds.rcCrosshair.bottom - clgame.ds.rcCrosshair.top;
    x =
        clgame.viewport[0 as libc::c_int as usize] +
            (clgame.viewport[2 as libc::c_int as usize] >> 1 as libc::c_int);
    y =
        clgame.viewport[1 as libc::c_int as usize] +
            (clgame.viewport[3 as libc::c_int as usize] >> 1 as libc::c_int);
    // g-cont - cl.crosshairangle is the autoaim angle.
	// if we're not using autoaim, just draw in the middle of the screen
    if !(cl.crosshairangle[0 as libc::c_int as usize] == 0.0f32 &&
             cl.crosshairangle[1 as libc::c_int as usize] == 0.0f32 &&
             cl.crosshairangle[2 as libc::c_int as usize] == 0.0f32) {
        let mut angles: vec3_t = [0.; 3];
        let mut forward: vec3_t = [0.; 3];
        let mut point: vec3_t = [0.; 3];
        let mut screen: vec3_t = [0.; 3];
        angles[0 as libc::c_int as usize] =
            refState.viewangles[0 as libc::c_int as usize] +
                cl.crosshairangle[0 as libc::c_int as usize];
        angles[1 as libc::c_int as usize] =
            refState.viewangles[1 as libc::c_int as usize] +
                cl.crosshairangle[1 as libc::c_int as usize];
        angles[2 as libc::c_int as usize] =
            refState.viewangles[2 as libc::c_int as usize] +
                cl.crosshairangle[2 as libc::c_int as usize];
        AngleVectors(angles.as_mut_ptr() as *const vec_t,
                     forward.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t);
        point[0 as libc::c_int as usize] =
            refState.vieworg[0 as libc::c_int as usize] +
                forward[0 as libc::c_int as usize];
        point[1 as libc::c_int as usize] =
            refState.vieworg[1 as libc::c_int as usize] +
                forward[1 as libc::c_int as usize];
        point[2 as libc::c_int as usize] =
            refState.vieworg[2 as libc::c_int as usize] +
                forward[2 as libc::c_int as usize];
        ref_0.dllFuncs.WorldToScreen.expect("non-null function pointer")(point.as_mut_ptr()
                                                                             as
                                                                             *const vec_t,
                                                                         screen.as_mut_ptr());
        x =
            (x as libc::c_float +
                 ((clgame.viewport[2 as libc::c_int as usize] >>
                       1 as libc::c_int) as libc::c_float *
                      screen[0 as libc::c_int as usize] + 0.5f32)) as
                libc::c_int;
        y =
            (y as libc::c_float +
                 ((clgame.viewport[3 as libc::c_int as usize] >>
                       1 as libc::c_int) as libc::c_float *
                      screen[1 as libc::c_int as usize] + 0.5f32)) as
                libc::c_int
    }
    // back to logical sizes
    xscale =
        clgame.scrInfo.iWidth as libc::c_float /
            refState.width as libc::c_float;
    yscale =
        clgame.scrInfo.iHeight as libc::c_float /
            refState.height as libc::c_float;
    x = (x as libc::c_float * xscale) as libc::c_int;
    y = (y as libc::c_float * yscale) as libc::c_int;
    // move at center the screen
    x = (x as libc::c_float - 0.5f32 * width as libc::c_float) as libc::c_int;
    y =
        (y as libc::c_float - 0.5f32 * height as libc::c_float) as
            libc::c_int;
    clgame.ds.pSprite = clgame.ds.pCrosshair;
    clgame.ds.spriteColor[0 as libc::c_int as usize] =
        clgame.ds.rgbaCrosshair[0 as libc::c_int as usize];
    clgame.ds.spriteColor[1 as libc::c_int as usize] =
        clgame.ds.rgbaCrosshair[1 as libc::c_int as usize];
    clgame.ds.spriteColor[2 as libc::c_int as usize] =
        clgame.ds.rgbaCrosshair[2 as libc::c_int as usize];
    clgame.ds.spriteColor[3 as libc::c_int as usize] =
        clgame.ds.rgbaCrosshair[3 as libc::c_int as usize];
    pfnSPR_DrawHoles(0 as libc::c_int, x, y, &mut clgame.ds.rcCrosshair);
}
/*
=============
CL_DrawLoading

draw loading progress bar
=============
*/
unsafe extern "C" fn CL_DrawLoadingOrPaused(mut paused: qboolean,
                                            mut percent: libc::c_float) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    R_GetTextureParms(&mut width, &mut height,
                      if paused as libc::c_uint != 0 {
                          cls.pauseIcon
                      } else { cls.loadingBar });
    x = clgame.scrInfo.iWidth - width >> 1 as libc::c_int;
    y = clgame.scrInfo.iHeight - height >> 1 as libc::c_int;
    SPR_AdjustSizei(&mut x, &mut y, &mut width, &mut height);
    if paused as u64 == 0 {
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
        ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                                as
                                                                                libc::c_int);
        ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(x
                                                                                as
                                                                                libc::c_float,
                                                                            y
                                                                                as
                                                                                libc::c_float,
                                                                            width
                                                                                as
                                                                                libc::c_float,
                                                                            height
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
                                                                            cls.loadingBar);
    } else {
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
        ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                                as
                                                                                libc::c_int);
        ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(x
                                                                                as
                                                                                libc::c_float,
                                                                            y
                                                                                as
                                                                                libc::c_float,
                                                                            width
                                                                                as
                                                                                libc::c_float,
                                                                            height
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
                                                                            cls.pauseIcon);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_DrawHUD(mut state: libc::c_int) {
    if state == CL_ACTIVE as libc::c_int && cl.video_prepped as u64 == 0 {
        state = CL_LOADING as libc::c_int
    }
    if state == CL_ACTIVE as libc::c_int && cl.paused as libc::c_uint != 0 {
        state = CL_PAUSED as libc::c_int
    }
    match state {
        2 => {
            if cl.intermission == 0 { CL_DrawScreenFade(); }
            CL_DrawCrosshair();
            CL_DrawCenterPrint();
            clgame.dllFuncs.pfnRedraw.expect("non-null function pointer")(cl.time
                                                                              as
                                                                              libc::c_float,
                                                                          cl.intermission);
            if cl.intermission != 0 { CL_DrawScreenFade(); }
        }
        3 => {
            CL_DrawScreenFade();
            CL_DrawCrosshair();
            CL_DrawCenterPrint();
            clgame.dllFuncs.pfnRedraw.expect("non-null function pointer")(cl.time
                                                                              as
                                                                              libc::c_float,
                                                                          cl.intermission);
            CL_DrawLoadingOrPaused(true_0, 0.0f32);
        }
        1 => { CL_DrawLoadingOrPaused(false_0, (*scr_loading).value); }
        4 => {
            if cls.draw_changelevel as u64 != 0 {
                CL_DrawLoadingOrPaused(false_0, 100.0f32);
                cls.draw_changelevel = false_0
            }
        }
        _ => { }
    };
}
unsafe extern "C" fn CL_ClearUserMessage(mut pszName: *mut libc::c_char,
                                         mut svc_num: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 197 as libc::c_int &&
              clgame.msg[i as usize].name[0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if clgame.msg[i as usize].number == svc_num &&
               Q_strncmp(clgame.msg[i as usize].name.as_mut_ptr(), pszName,
                         99999 as libc::c_int) != 0 {
            clgame.msg[i as usize].number = 0 as libc::c_int
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_LinkUserMessage(mut pszName: *mut libc::c_char,
                                            svc_num: libc::c_int,
                                            mut iSize: libc::c_int) {
    let mut i: libc::c_int = 0;
    if pszName.is_null() || *pszName == 0 {
        Host_Error(b"CL_LinkUserMessage: bad message name\n\x00" as *const u8
                       as *const libc::c_char);
    }
    if svc_num <= 59 as libc::c_int {
        Host_Error(b"CL_LinkUserMessage: tried to hook a system message \"%s\"\n\x00"
                       as *const u8 as *const libc::c_char,
                   svc_strings[svc_num as usize]);
    }
    // see if already hooked
    i = 0 as libc::c_int;
    while i < 197 as libc::c_int &&
              clgame.msg[i as usize].name[0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        // NOTE: no check for DispatchFunc, check only name
        if Q_strnicmp(clgame.msg[i as usize].name.as_mut_ptr(), pszName,
                      99999 as libc::c_int) == 0 {
            clgame.msg[i as usize].number = svc_num;
            clgame.msg[i as usize].size = iSize;
            CL_ClearUserMessage(pszName, svc_num);
            return
        }
        i += 1
    }
    if i == 197 as libc::c_int {
        Host_Error(b"CL_LinkUserMessage: MAX_USER_MESSAGES hit!\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    // register new message without DispatchFunc, so we should parse it properly
    Q_strncpy(clgame.msg[i as usize].name.as_mut_ptr(), pszName,
              ::std::mem::size_of::<[libc::c_char; 32]>() as
                  libc::c_ulong); // world model
    clgame.msg[i as usize].number = svc_num; // Host_Error without client
    clgame.msg[i as usize].size = iSize; // purge old remap info
    CL_ClearUserMessage(pszName, svc_num);
}
#[no_mangle]
pub unsafe extern "C" fn CL_FreeEntity(mut pEdict: *mut cl_entity_t) {
    R_RemoveEfrags(pEdict);
    CL_KillDeadBeams(pEdict);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ClearWorld() {
    let mut worldmodel: *mut cl_entity_t = 0 as *mut cl_entity_t;
    worldmodel = clgame.entities;
    (*worldmodel).curstate.modelindex = 1 as libc::c_int;
    (*worldmodel).curstate.solid = 4 as libc::c_int as libc::c_short;
    (*worldmodel).curstate.movetype = 7 as libc::c_int;
    (*worldmodel).model = cl.worldmodel;
    (*worldmodel).index = 0 as libc::c_int;
    world.max_recursion = 0 as libc::c_int;
    clgame.ds.cullMode = TRI_FRONT;
    clgame.numStatics = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_InitEdicts() {
    if clgame.mempool == 0 { return }
    CL_UPDATE_BACKUP =
        if cl.maxclients <= 1 as libc::c_int {
            16 as libc::c_int
        } else { 64 as libc::c_int };
    cls.num_client_entities = CL_UPDATE_BACKUP * 256 as libc::c_int;
    cls.packet_entities =
        _Mem_Realloc(clgame.mempool, cls.packet_entities as *mut libc::c_void,
                     (::std::mem::size_of::<entity_state_t>() as
                          libc::c_ulong).wrapping_mul(cls.num_client_entities
                                                          as libc::c_ulong),
                     true_0,
                     b"../engine/client/cl_game.c\x00" as *const u8 as
                         *const libc::c_char, 1099 as libc::c_int) as
            *mut entity_state_t;
    clgame.entities =
        _Mem_Alloc(clgame.mempool,
                   (::std::mem::size_of::<cl_entity_t>() as
                        libc::c_ulong).wrapping_mul(clgame.maxEntities as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_game.c\x00" as *const u8 as
                       *const libc::c_char, 1100 as libc::c_int) as
            *mut cl_entity_t;
    clgame.static_entities =
        _Mem_Alloc(clgame.mempool,
                   (::std::mem::size_of::<cl_entity_t>() as
                        libc::c_ulong).wrapping_mul(3096 as libc::c_int as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_game.c\x00" as *const u8 as
                       *const libc::c_char, 1101 as libc::c_int) as
            *mut cl_entity_t;
    clgame.numStatics = 0 as libc::c_int;
    if clgame.maxRemapInfos - 1 as libc::c_int != clgame.maxEntities {
        CL_ClearAllRemaps();
        clgame.maxRemapInfos = clgame.maxEntities + 1 as libc::c_int;
        clgame.remap_info =
            _Mem_Alloc(clgame.mempool,
                       (::std::mem::size_of::<*mut remap_info_t>() as
                            libc::c_ulong).wrapping_mul(clgame.maxRemapInfos
                                                            as libc::c_ulong),
                       true_0,
                       b"../engine/client/cl_game.c\x00" as *const u8 as
                           *const libc::c_char, 1108 as libc::c_int) as
                *mut *mut remap_info_t
    }
    ref_0.dllFuncs.R_ProcessEntData.expect("non-null function pointer")(true_0);
}
#[no_mangle]
pub unsafe extern "C" fn CL_FreeEdicts() {
    ref_0.dllFuncs.R_ProcessEntData.expect("non-null function pointer")(false_0);
    if !clgame.entities.is_null() {
        _Mem_Free(clgame.entities as *mut libc::c_void,
                  b"../engine/client/cl_game.c\x00" as *const u8 as
                      *const libc::c_char, 1119 as libc::c_int);
    }
    clgame.entities = 0 as *mut cl_entity_t;
    if !clgame.static_entities.is_null() {
        _Mem_Free(clgame.static_entities as *mut libc::c_void,
                  b"../engine/client/cl_game.c\x00" as *const u8 as
                      *const libc::c_char, 1123 as libc::c_int);
    }
    clgame.static_entities = 0 as *mut cl_entity_t;
    if !cls.packet_entities.is_null() {
        if !cls.packet_entities.is_null() {
            _Mem_Free(cls.packet_entities as *mut libc::c_void,
                      b"../engine/client/cl_game.c\x00" as *const u8 as
                          *const libc::c_char, 1127 as libc::c_int);
        }
    }
    cls.packet_entities = 0 as *mut entity_state_t;
    cls.num_client_entities = 0 as libc::c_int;
    cls.next_client_entities = 0 as libc::c_int;
    clgame.numStatics = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_ClearEdicts() {
    if !clgame.entities.is_null() { return }
    // in case we stopped with error
    clgame.maxEntities = 2 as libc::c_int;
    CL_InitEdicts();
}
/*
==================
CL_ClearSpriteTextures

free studio cache on change level
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearSpriteTextures() {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 256 as libc::c_int {
        clgame.sprites[i as usize].needload = false_0;
        i += 1
    };
}
/*
=============
CL_LoadHudSprite

upload sprite frames
=============
*/
unsafe extern "C" fn CL_LoadHudSprite(mut szSpriteName: *const libc::c_char,
                                      mut m_pSprite: *mut model_t,
                                      mut type_0: uint, mut texFlags: uint)
 -> qboolean {
    let mut buf: *mut byte = 0 as *mut byte;
    let mut size: fs_offset_t = 0;
    let mut loaded: qboolean = false_0;
    Q_strncpy((*m_pSprite).name.as_mut_ptr(), szSpriteName,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // it's hud sprite, make difference names to prevent free shared textures
    if type_0 == 0 as libc::c_int as libc::c_uint ||
           type_0 == 1 as libc::c_int as libc::c_uint {
        (*m_pSprite).flags =
            ((*m_pSprite).flags as libc::c_uint |
                 (1 as libc::c_uint) << 30 as libc::c_int) as libc::c_int
    } // store texFlags into numtexinfo
    (*m_pSprite).numtexinfo = texFlags as libc::c_int;
    if FS_FileExists(szSpriteName, false_0 as libc::c_int) == 0 {
        if cls.state as libc::c_uint !=
               ca_active as libc::c_int as libc::c_uint &&
               cl.maxclients > 1 as libc::c_int {
            // trying to download sprite from server
            CL_AddClientResource(szSpriteName, t_model as libc::c_int);
            (*m_pSprite).needload = true_0;
            return true_0
        } else {
            Con_Reportf(b"^1Error:^7 Could not load HUD sprite %s\n\x00" as
                            *const u8 as *const libc::c_char, szSpriteName);
            Mod_FreeModel(m_pSprite);
            return false_0
        }
    }
    buf = FS_LoadFile(szSpriteName, &mut size, false_0);
    if buf.is_null() { return false_0 }
    if type_0 == 2 as libc::c_int as libc::c_uint {
        ref_0.dllFuncs.Mod_LoadMapSprite.expect("non-null function pointer")(m_pSprite,
                                                                             buf
                                                                                 as
                                                                                 *const libc::c_void,
                                                                             size
                                                                                 as
                                                                                 size_t,
                                                                             &mut loaded);
    } else {
        Mod_LoadSpriteModel(m_pSprite, buf as *const libc::c_void,
                            &mut loaded, texFlags);
        ref_0.dllFuncs.Mod_ProcessRenderData.expect("non-null function pointer")(m_pSprite,
                                                                                 true_0,
                                                                                 buf);
    }
    _Mem_Free(buf as *mut libc::c_void,
              b"../engine/client/cl_game.c\x00" as *const u8 as
                  *const libc::c_char, 1211 as libc::c_int);
    if loaded as u64 == 0 { Mod_FreeModel(m_pSprite); return false_0 }
    (*m_pSprite).needload = 2 as qboolean;
    return true_0;
}
/*
=============
CL_LoadSpriteModel

some sprite models is exist only at client: HUD sprites,
tent sprites or overview images
=============
*/
unsafe extern "C" fn CL_LoadSpriteModel(mut filename: *const libc::c_char,
                                        mut type_0: uint, mut texFlags: uint)
 -> *mut model_t {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Reportf(b"^1Error:^7 CL_LoadSpriteModel: bad name!\n\x00" as
                        *const u8 as *const libc::c_char);
        return 0 as *mut model_t
    }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    // slot 0 isn't used
    i = 1 as libc::c_int;
    mod_0 = clgame.sprites.as_mut_ptr();
    while i < 256 as libc::c_int {
        if Q_strnicmp((*mod_0).name.as_mut_ptr(), name.as_mut_ptr(),
                      99999 as libc::c_int) == 0 {
            if (*mod_0).needload as libc::c_uint ==
                   1 as libc::c_int as libc::c_uint {
                if CL_LoadHudSprite(name.as_mut_ptr(), mod_0, type_0,
                                    texFlags) as u64 != 0 {
                    return mod_0
                }
            }
            // prolonge registration
            (*mod_0).needload = 2 as qboolean;
            return mod_0
        }
        i += 1;
        mod_0 = mod_0.offset(1)
    }
    // find a free model slot spot
    i = 1 as libc::c_int; // this is a valid spot
    mod_0 = clgame.sprites.as_mut_ptr();
    while i < 256 as libc::c_int {
        if (*mod_0).name[0 as libc::c_int as usize] == 0 { break ; }
        i += 1;
        mod_0 = mod_0.offset(1)
    }
    if i == 256 as libc::c_int {
        Con_Printf(b"^1Error:^7 MAX_CLIENT_SPRITES limit exceeded (%d)\n\x00"
                       as *const u8 as *const libc::c_char,
                   256 as libc::c_int);
        return 0 as *mut model_t
    }
    // load new map sprite
    if CL_LoadHudSprite(name.as_mut_ptr(), mod_0, type_0, texFlags) as u64 !=
           0 {
        return mod_0
    }
    return 0 as *mut model_t;
}
/*
=============
CL_LoadClientSprite

load sprites for temp ents
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LoadClientSprite(mut filename:
                                                 *const libc::c_char)
 -> *mut model_t {
    return CL_LoadSpriteModel(filename, 0 as libc::c_int as uint,
                              0 as libc::c_int as uint);
}
/*
===============================================================================
	CGame Builtin Functions

===============================================================================
*/
/*
=========
pfnSPR_LoadExt

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSPR_LoadExt(mut szPicName: *const libc::c_char,
                                        mut texFlags: uint) -> HSPRITE {
    let mut spr: *mut model_t = 0 as *mut model_t;
    spr = CL_LoadSpriteModel(szPicName, 0 as libc::c_int as uint, texFlags);
    if spr.is_null() { return 0 as libc::c_int }
    return spr.wrapping_offset_from(clgame.sprites.as_mut_ptr()) as
               libc::c_long as HSPRITE;
    // return index
}
/*
=========
pfnSPR_Load

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSPR_Load(mut szPicName: *const libc::c_char)
 -> HSPRITE {
    let mut spr: *mut model_t = 0 as *mut model_t;
    spr =
        CL_LoadSpriteModel(szPicName, 1 as libc::c_int as uint,
                           0 as libc::c_int as uint);
    if spr.is_null() { return 0 as libc::c_int }
    return spr.wrapping_offset_from(clgame.sprites.as_mut_ptr()) as
               libc::c_long as HSPRITE;
    // return index
}
/*
=============
CL_GetSpritePointer

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetSpritePointer(mut hSprite: HSPRITE)
 -> *const model_t {
    let mut mod_0: *mut model_t = 0 as *mut model_t; // bad image
    if hSprite <= 0 as libc::c_int || hSprite >= 256 as libc::c_int {
        return 0 as *const model_t
    }
    mod_0 =
        &mut *clgame.sprites.as_mut_ptr().offset(hSprite as isize) as
            *mut model_t;
    if (*mod_0).needload as libc::c_uint == 1 as libc::c_int as libc::c_uint {
        let mut type_0: libc::c_int =
            if (*mod_0).flags as libc::c_uint &
                   (1 as libc::c_uint) << 30 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 2 as libc::c_int };
        if CL_LoadHudSprite((*mod_0).name.as_mut_ptr(), mod_0, type_0 as uint,
                            (*mod_0).numtexinfo as uint) as u64 != 0 {
            return mod_0
        }
    }
    if (*mod_0).mempool != 0 {
        (*mod_0).needload = 2 as qboolean;
        return mod_0
    }
    return 0 as *const model_t;
}
/*
=========
pfnSPR_Frames

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSPR_Frames(mut hPic: HSPRITE) -> libc::c_int {
    let mut numFrames: libc::c_int = 0;
    ref_0.dllFuncs.R_GetSpriteParms.expect("non-null function pointer")(0 as
                                                                            *mut libc::c_int,
                                                                        0 as
                                                                            *mut libc::c_int,
                                                                        &mut numFrames,
                                                                        0 as
                                                                            libc::c_int,
                                                                        CL_GetSpritePointer(hPic));
    return numFrames;
}
/*
=========
pfnSPR_Height

=========
*/
unsafe extern "C" fn pfnSPR_Height(mut hPic: HSPRITE, mut frame: libc::c_int)
 -> libc::c_int {
    let mut sprHeight: libc::c_int = 0;
    ref_0.dllFuncs.R_GetSpriteParms.expect("non-null function pointer")(0 as
                                                                            *mut libc::c_int,
                                                                        &mut sprHeight,
                                                                        0 as
                                                                            *mut libc::c_int,
                                                                        frame,
                                                                        CL_GetSpritePointer(hPic));
    return sprHeight;
}
/*
=========
pfnSPR_Width

=========
*/
unsafe extern "C" fn pfnSPR_Width(mut hPic: HSPRITE, mut frame: libc::c_int)
 -> libc::c_int {
    let mut sprWidth: libc::c_int = 0;
    ref_0.dllFuncs.R_GetSpriteParms.expect("non-null function pointer")(&mut sprWidth,
                                                                        0 as
                                                                            *mut libc::c_int,
                                                                        0 as
                                                                            *mut libc::c_int,
                                                                        frame,
                                                                        CL_GetSpritePointer(hPic));
    return sprWidth;
}
/*
=========
pfnSPR_Set

=========
*/
unsafe extern "C" fn pfnSPR_Set(mut hPic: HSPRITE, mut r: libc::c_int,
                                mut g: libc::c_int, mut b: libc::c_int) {
    clgame.ds.pSprite = CL_GetSpritePointer(hPic);
    clgame.ds.spriteColor[0 as libc::c_int as usize] =
        if r >= 0 as libc::c_int {
            if r < 255 as libc::c_int { r } else { 255 as libc::c_int }
        } else { 0 as libc::c_int } as byte;
    clgame.ds.spriteColor[1 as libc::c_int as usize] =
        if g >= 0 as libc::c_int {
            if g < 255 as libc::c_int { g } else { 255 as libc::c_int }
        } else { 0 as libc::c_int } as byte;
    clgame.ds.spriteColor[2 as libc::c_int as usize] =
        if b >= 0 as libc::c_int {
            if b < 255 as libc::c_int { b } else { 255 as libc::c_int }
        } else { 0 as libc::c_int } as byte;
    clgame.ds.spriteColor[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
}
/*
=========
pfnSPR_Draw

=========
*/
unsafe extern "C" fn pfnSPR_Draw(mut frame: libc::c_int, mut x: libc::c_int,
                                 mut y: libc::c_int,
                                 mut prc: *const wrect_t) {
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderNormal
                                                                            as
                                                                            libc::c_int);
    SPR_DrawGeneric(frame, x as libc::c_float, y as libc::c_float,
                    -(1 as libc::c_int) as libc::c_float,
                    -(1 as libc::c_int) as libc::c_float, prc);
}
/*
=========
pfnSPR_DrawHoles

=========
*/
unsafe extern "C" fn pfnSPR_DrawHoles(mut frame: libc::c_int,
                                      mut x: libc::c_int, mut y: libc::c_int,
                                      mut prc: *const wrect_t) {
    // REFTODO
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransColor
                                                                            as
                                                                            libc::c_int);
    SPR_DrawGeneric(frame, x as libc::c_float, y as libc::c_float,
                    -(1 as libc::c_int) as libc::c_float,
                    -(1 as libc::c_int) as libc::c_float, prc);
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderNormal
                                                                            as
                                                                            libc::c_int);
}
/*
=========
pfnSPR_DrawAdditive

=========
*/
unsafe extern "C" fn pfnSPR_DrawAdditive(mut frame: libc::c_int,
                                         mut x: libc::c_int,
                                         mut y: libc::c_int,
                                         mut prc: *const wrect_t) {
    // REFTODO
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransAdd
                                                                            as
                                                                            libc::c_int);
    SPR_DrawGeneric(frame, x as libc::c_float, y as libc::c_float,
                    -(1 as libc::c_int) as libc::c_float,
                    -(1 as libc::c_int) as libc::c_float, prc);
    // REFTODO
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderNormal
                                                                            as
                                                                            libc::c_int);
}
/*
=========
pfnSPR_GetList

for parsing half-life scripts - hud.txt etc
=========
*/
unsafe extern "C" fn pfnSPR_GetList(mut psz: *mut libc::c_char,
                                    mut piCount: *mut libc::c_int)
 -> *mut client_sprite_t {
    let mut pEntry: *mut cached_spritelist_t =
        &mut *clgame.sprlist.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut cached_spritelist_t;
    let mut slot: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut numSprites: libc::c_int = 0 as libc::c_int;
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: string = [0; 256];
    if !piCount.is_null() { *piCount = 0 as libc::c_int }
    // see if already in list
	// NOTE: client.dll is cache hud.txt but reparse weapon lists again and again
	// obviously there a memory leak by-design. Cache the sprite lists to prevent it
    slot = 0 as libc::c_int;
    while slot < 256 as libc::c_int &&
              (*pEntry).szListName[0 as libc::c_int as usize] as libc::c_int
                  != 0 {
        pEntry =
            &mut *clgame.sprlist.as_mut_ptr().offset(slot as isize) as
                *mut cached_spritelist_t;
        if Q_strnicmp((*pEntry).szListName.as_mut_ptr(), psz,
                      99999 as libc::c_int) == 0 {
            if !piCount.is_null() { *piCount = (*pEntry).count }
            return (*pEntry).pList
        }
        slot += 1
    }
    if slot == 256 as libc::c_int {
        Con_Printf(b"^1Error:^7 SPR_GetList: overflow cache!\n\x00" as
                       *const u8 as *const libc::c_char);
        return 0 as *mut client_sprite_t
    }
    if clgame.itemspath[0 as libc::c_int as usize] == 0 {
        // typically it's sprites\*.txt
        COM_ExtractFilePath(psz, clgame.itemspath.as_mut_ptr());
    }
    afile = FS_LoadFile(psz, 0 as *mut fs_offset_t, false_0);
    if afile.is_null() { return 0 as *mut client_sprite_t }
    pfile = afile as *mut libc::c_char;
    pfile =
        _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    numSprites = Q_atoi(token.as_mut_ptr());
    Q_strncpy((*pEntry).szListName.as_mut_ptr(), psz,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // name, res, pic, x, y, w, h
    (*pEntry).pList =
        _Mem_Alloc(cls.mempool,
                   (::std::mem::size_of::<client_sprite_t>() as
                        libc::c_ulong).wrapping_mul(numSprites as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_game.c\x00" as *const u8 as
                       *const libc::c_char, 1532 as libc::c_int) as
            *mut client_sprite_t;
    index = 0 as libc::c_int;
    while index < numSprites {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        Q_strncpy((*(*pEntry).pList.offset(index as
                                               isize)).szName.as_mut_ptr(),
                  token.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        // read resolution
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        (*(*pEntry).pList.offset(index as isize)).iRes =
            Q_atoi(token.as_mut_ptr());
        // read spritename
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        Q_strncpy((*(*pEntry).pList.offset(index as
                                               isize)).szSprite.as_mut_ptr(),
                  token.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        // parse rectangle
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        (*(*pEntry).pList.offset(index as isize)).rc.left =
            Q_atoi(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        (*(*pEntry).pList.offset(index as isize)).rc.top =
            Q_atoi(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        (*(*pEntry).pList.offset(index as isize)).rc.right =
            (*(*pEntry).pList.offset(index as isize)).rc.left +
                Q_atoi(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        (*(*pEntry).pList.offset(index as isize)).rc.bottom =
            (*(*pEntry).pList.offset(index as isize)).rc.top +
                Q_atoi(token.as_mut_ptr());
        (*pEntry).count += 1;
        index += 1
    }
    if index < numSprites {
        Con_DPrintf(b"^3Warning:^7 unexpected end of %s (%i should be %i)\n\x00"
                        as *const u8 as *const libc::c_char, psz, numSprites,
                    index);
    }
    if !piCount.is_null() { *piCount = (*pEntry).count }
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/client/cl_game.c\x00" as *const u8 as
                  *const libc::c_char, 1568 as libc::c_int);
    return (*pEntry).pList;
}
/*
=============
CL_FillRGBA

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FillRGBA(mut x: libc::c_int, mut y: libc::c_int,
                                     mut w: libc::c_int, mut h: libc::c_int,
                                     mut r: libc::c_int, mut g: libc::c_int,
                                     mut b: libc::c_int, mut a: libc::c_int) {
    let mut _x: libc::c_float = x as libc::c_float;
    let mut _y: libc::c_float = y as libc::c_float;
    let mut _w: libc::c_float = w as libc::c_float;
    let mut _h: libc::c_float = h as libc::c_float;
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
    SPR_AdjustSize(&mut _x, &mut _y, &mut _w, &mut _h);
    ref_0.dllFuncs.FillRGBA.expect("non-null function pointer")(_x, _y, _w,
                                                                _h, r, g, b,
                                                                a);
}
/*
=============
pfnGetScreenInfo

get actual screen info
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetScreenInfo(mut pscrinfo: *mut SCREENINFO)
 -> libc::c_int {
    let mut scale_factor: libc::c_float = (*hud_scale).value;
    // setup screen info
    clgame.scrInfo.iSize =
        ::std::mem::size_of::<SCREENINFO>() as libc::c_ulong as libc::c_int;
    clgame.scrInfo.iFlags = 1 as libc::c_int;
    if scale_factor != 0. && scale_factor != 1.0f32 {
        clgame.scrInfo.iWidth =
            (refState.width as libc::c_float / scale_factor) as libc::c_int;
        clgame.scrInfo.iHeight =
            (refState.height as libc::c_float / scale_factor) as libc::c_int;
        clgame.scrInfo.iFlags |= 2 as libc::c_int
    } else {
        clgame.scrInfo.iWidth = refState.width;
        clgame.scrInfo.iHeight = refState.height;
        clgame.scrInfo.iFlags &= !(2 as libc::c_int)
    }
    if pscrinfo.is_null() { return 0 as libc::c_int }
    if (*pscrinfo).iSize != clgame.scrInfo.iSize {
        clgame.scrInfo.iSize = (*pscrinfo).iSize
    }
    // copy screeninfo out
    memcpy(pscrinfo as *mut libc::c_void,
           &mut clgame.scrInfo as *mut SCREENINFO as *const libc::c_void,
           clgame.scrInfo.iSize as libc::c_ulong);
    return 1 as libc::c_int;
}
/*
=============
pfnSetCrosshair

setup crosshair
=============
*/
unsafe extern "C" fn pfnSetCrosshair(mut hspr: HSPRITE, mut rc: wrect_t,
                                     mut r: libc::c_int, mut g: libc::c_int,
                                     mut b: libc::c_int) {
    clgame.ds.rgbaCrosshair[0 as libc::c_int as usize] = r as byte;
    clgame.ds.rgbaCrosshair[1 as libc::c_int as usize] = g as byte;
    clgame.ds.rgbaCrosshair[2 as libc::c_int as usize] = b as byte;
    clgame.ds.rgbaCrosshair[3 as libc::c_int as usize] =
        0xff as libc::c_int as byte;
    clgame.ds.pCrosshair = CL_GetSpritePointer(hspr);
    clgame.ds.rcCrosshair = rc;
}
/*
=============
pfnHookUserMsg

=============
*/
unsafe extern "C" fn pfnHookUserMsg(mut pszName: *const libc::c_char,
                                    mut pfn: pfnUserMsgHook) -> libc::c_int {
    let mut i: libc::c_int = 0;
    // ignore blank names or invalid callbacks
    if pszName.is_null() || *pszName == 0 || pfn.is_none() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 197 as libc::c_int &&
              clgame.msg[i as usize].name[0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        // see if already hooked
        if Q_strnicmp(clgame.msg[i as usize].name.as_mut_ptr(), pszName,
                      99999 as libc::c_int) == 0 {
            return 1 as libc::c_int
        }
        i += 1
    }
    if i == 197 as libc::c_int {
        Host_Error(b"HookUserMsg: MAX_USER_MESSAGES hit!\n\x00" as *const u8
                       as *const libc::c_char);
        return 0 as libc::c_int
    }
    // hook new message
    Q_strncpy(clgame.msg[i as usize].name.as_mut_ptr(), pszName,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    clgame.msg[i as usize].func = pfn;
    return 1 as libc::c_int;
}
/*
=============
pfnServerCmd

=============
*/
unsafe extern "C" fn pfnServerCmd(mut szCmdString: *const libc::c_char)
 -> libc::c_int {
    let mut buf: string = [0; 256];
    if if szCmdString.is_null() || *szCmdString == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    // just like the client typed "cmd xxxxx" at the console
    Q_snprintf(buf.as_mut_ptr(),
               (::std::mem::size_of::<string>() as
                    libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulong),
               b"cmd %s\n\x00" as *const u8 as *const libc::c_char,
               szCmdString);
    Cbuf_AddText(buf.as_mut_ptr());
    return 1 as libc::c_int;
}
/*
=============
pfnClientCmd

=============
*/
unsafe extern "C" fn pfnClientCmd(mut szCmdString: *const libc::c_char)
 -> libc::c_int {
    if if szCmdString.is_null() || *szCmdString == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    if cls.initialized as u64 != 0 {
        Cbuf_AddText(szCmdString);
        Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
    } else {
        // will exec later
        Q_strncat(host.deferred_cmd.as_mut_ptr(),
                  va(b"%s\n\x00" as *const u8 as *const libc::c_char,
                     szCmdString),
                  ::std::mem::size_of::<[libc::c_char; 128]>() as
                      libc::c_ulong);
    }
    return 1 as libc::c_int;
}
/*
=============
pfnFilteredClientCmd
=============
*/
unsafe extern "C" fn pfnFilteredClientCmd(mut szCmdString:
                                              *const libc::c_char)
 -> libc::c_int {
    if if szCmdString.is_null() || *szCmdString == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    // a1ba:
	// there should be stufftext validator, that checks
	// hardcoded commands and disallows them before passing to
	// filtered buffer, returning 0
	// I've replaced it by hooking potentially exploitable
	// commands and variables(motd_write, motdfile, etc) in client interfaces
    Cbuf_AddFilteredText(szCmdString);
    Cbuf_AddFilteredText(b"\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
/*
=============
pfnGetPlayerInfo

=============
*/
unsafe extern "C" fn pfnGetPlayerInfo(mut ent_num: libc::c_int,
                                      mut pinfo: *mut hud_player_info_t) {
    let mut player: *mut player_info_t =
        0 as *mut player_info_t; // player list if offset by 1 from ents
    ent_num -= 1 as libc::c_int;
    if ent_num >= cl.maxclients || ent_num < 0 as libc::c_int ||
           cl.players[ent_num as usize].name[0 as libc::c_int as usize] == 0 {
        (*pinfo).name = 0 as *mut libc::c_char;
        (*pinfo).thisplayer = false_0 as libc::c_int as byte;
        return
    }
    player =
        &mut *cl.players.as_mut_ptr().offset(ent_num as isize) as
            *mut player_info_t;
    (*pinfo).thisplayer =
        if ent_num == cl.playernum {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as byte;
    (*pinfo).name = (*player).name.as_mut_ptr();
    (*pinfo).model = (*player).model.as_mut_ptr();
    (*pinfo).spectator = (*player).spectator as byte;
    (*pinfo).ping = (*player).ping as libc::c_short;
    (*pinfo).packetloss = (*player).packet_loss as byte;
    (*pinfo).topcolor = (*player).topcolor as libc::c_short;
    (*pinfo).bottomcolor = (*player).bottomcolor as libc::c_short;
}
/*
=============
pfnPlaySoundByName

=============
*/
unsafe extern "C" fn pfnPlaySoundByName(mut szSound: *const libc::c_char,
                                        mut volume: libc::c_float) {
    let mut hSound: libc::c_int = S_RegisterSound(szSound);
    S_StartSound(0 as *const vec_t, cl.viewentity, 3 as libc::c_int, hSound,
                 volume, 0.8f64 as libc::c_float, 100 as libc::c_int,
                 (1 as libc::c_int) << 10 as libc::c_int);
}
/*
=============
pfnPlaySoundByIndex

=============
*/
unsafe extern "C" fn pfnPlaySoundByIndex(mut iSound: libc::c_int,
                                         mut volume: libc::c_float) {
    let mut hSound: libc::c_int = 0;
    // make sure what we in-bounds
    iSound =
        if iSound >= 0 as libc::c_int {
            if iSound < (1 as libc::c_int) << 11 as libc::c_int {
                iSound
            } else { ((1 as libc::c_int)) << 11 as libc::c_int }
        } else { 0 as libc::c_int };
    hSound = cl.sound_index[iSound as usize] as libc::c_int;
    if hSound == 0 { return }
    S_StartSound(0 as *const vec_t, cl.viewentity, 3 as libc::c_int, hSound,
                 volume, 0.8f64 as libc::c_float, 100 as libc::c_int,
                 (1 as libc::c_int) << 10 as libc::c_int);
}
/*
=============
pfnTextMessageGet

returns specified message from titles.txt
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TextMessageGet(mut pName: *const libc::c_char)
 -> *mut client_textmessage_t {
    let mut i: libc::c_int = 0;
    // first check internal messages
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if Q_strncmp(pName,
                     va(b"TextMessage%i\x00" as *const u8 as
                            *const libc::c_char, i), 99999 as libc::c_int) ==
               0 {
            return cl_textmessage.as_mut_ptr().offset(i as isize)
        }
        i += 1
    }
    // find desired message
    i = 0 as libc::c_int;
    while i < clgame.numTitles {
        if Q_strnicmp(pName, (*clgame.titles.offset(i as isize)).pName,
                      99999 as libc::c_int) == 0 {
            return clgame.titles.offset(i as isize)
        }
        i += 1
    }
    return 0 as *mut client_textmessage_t;
    // found nothing
}
/*
=============
pfnDrawCharacter

returns drawed chachter width (in real screen pixels)
=============
*/
unsafe extern "C" fn pfnDrawCharacter(mut x: libc::c_int, mut y: libc::c_int,
                                      mut number: libc::c_int,
                                      mut r: libc::c_int, mut g: libc::c_int,
                                      mut b: libc::c_int) -> libc::c_int {
    if cls.creditsFont.valid as u64 == 0 { return 0 as libc::c_int }
    if (*hud_utf8).value != 0. { number = Con_UtfProcessChar(number) }
    number &= 255 as libc::c_int;
    if number < 32 as libc::c_int { return 0 as libc::c_int }
    if y < -clgame.scrInfo.iCharHeight { return 0 as libc::c_int }
    clgame.ds.adjust_size = true_0;
    pfnPIC_Set(cls.creditsFont.hFontTexture, r, g, b, 255 as libc::c_int);
    pfnPIC_DrawAdditive(x, y, -(1 as libc::c_int), -(1 as libc::c_int),
                        &mut *cls.creditsFont.fontRc.as_mut_ptr().offset(number
                                                                             as
                                                                             isize));
    clgame.ds.adjust_size = false_0;
    return clgame.scrInfo.charWidths[number as usize] as libc::c_int;
}
/*
=============
pfnDrawConsoleString

drawing string like a console string
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnDrawConsoleString(mut x: libc::c_int,
                                              mut y: libc::c_int,
                                              mut string: *mut libc::c_char)
 -> libc::c_int {
    let mut drawLen: libc::c_int = 0; // silent ignore
    if if string.is_null() || *string == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    Con_SetFont((*con_fontsize).value as libc::c_int);
    clgame.ds.adjust_size = true_0;
    drawLen = Con_DrawString(x, y, string, clgame.ds.textColor.as_mut_ptr());
    clgame.ds.textColor[0 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    clgame.ds.textColor[1 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    clgame.ds.textColor[2 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    clgame.ds.textColor[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    clgame.ds.adjust_size = false_0;
    Con_RestoreFont();
    return x + drawLen;
    // exclude color prexfixes
}
/*
=============
pfnDrawSetTextColor

set color for anything
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnDrawSetTextColor(mut r: libc::c_float,
                                             mut g: libc::c_float,
                                             mut b: libc::c_float) {
    // bound color and convert to byte
    clgame.ds.textColor[0 as libc::c_int as usize] =
        if r * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (r * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (r) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as byte;
    clgame.ds.textColor[1 as libc::c_int as usize] =
        if g * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (g * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (g) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as byte;
    clgame.ds.textColor[2 as libc::c_int as usize] =
        if b * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (b * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (b) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as byte;
    clgame.ds.textColor[3 as libc::c_int as usize] =
        0xff as libc::c_int as byte;
}
/*
=============
pfnDrawConsoleStringLen

compute string length in screen pixels
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnDrawConsoleStringLen(mut pText:
                                                     *const libc::c_char,
                                                 mut length: *mut libc::c_int,
                                                 mut height:
                                                     *mut libc::c_int) {
    Con_SetFont((*con_fontsize).value as libc::c_int);
    Con_DrawStringLen(pText, length, height);
    Con_RestoreFont();
}
/*
=============
pfnConsolePrint

prints directly into console (can skip notify)
=============
*/
unsafe extern "C" fn pfnConsolePrint(mut string: *const libc::c_char) {
    Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char, string);
}
/*
=============
pfnCenterPrint

holds and fade message at center of screen
like trigger_multiple message in q1
=============
*/
unsafe extern "C" fn pfnCenterPrint(mut string: *const libc::c_char) {
    CL_CenterPrint(string, 0.25f32);
}
/*
=========
GetWindowCenterX

=========
*/
unsafe extern "C" fn pfnGetWindowCenterX() -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    SDL_GetWindowPosition(host.hWnd as *mut SDL_Window, &mut x,
                          0 as *mut libc::c_int);
    return host.window_center_x + x;
}
/*
=========
GetWindowCenterY

=========
*/
unsafe extern "C" fn pfnGetWindowCenterY() -> libc::c_int {
    let mut y: libc::c_int = 0 as libc::c_int;
    SDL_GetWindowPosition(host.hWnd as *mut SDL_Window, 0 as *mut libc::c_int,
                          &mut y);
    return host.window_center_y + y;
}
/*
=============
pfnGetViewAngles

return interpolated angles from previous frame
=============
*/
unsafe extern "C" fn pfnGetViewAngles(mut angles: *mut libc::c_float) {
    if !angles.is_null() {
        *angles.offset(0 as libc::c_int as isize) =
            cl.viewangles[0 as libc::c_int as usize];
        *angles.offset(1 as libc::c_int as isize) =
            cl.viewangles[1 as libc::c_int as usize];
        *angles.offset(2 as libc::c_int as isize) =
            cl.viewangles[2 as libc::c_int as usize]
    };
}
/*
=============
pfnSetViewAngles

return interpolated angles from previous frame
=============
*/
unsafe extern "C" fn pfnSetViewAngles(mut angles: *mut libc::c_float) {
    if !angles.is_null() {
        cl.viewangles[0 as libc::c_int as usize] =
            *angles.offset(0 as libc::c_int as isize);
        cl.viewangles[1 as libc::c_int as usize] =
            *angles.offset(1 as libc::c_int as isize);
        cl.viewangles[2 as libc::c_int as usize] =
            *angles.offset(2 as libc::c_int as isize)
    };
}
/*
=============
pfnPhysInfo_ValueForKey

=============
*/
unsafe extern "C" fn pfnPhysInfo_ValueForKey(mut key: *const libc::c_char)
 -> *const libc::c_char {
    return Info_ValueForKey(cls.physinfo.as_mut_ptr(), key);
}
/*
=============
pfnServerInfo_ValueForKey

=============
*/
unsafe extern "C" fn pfnServerInfo_ValueForKey(mut key: *const libc::c_char)
 -> *const libc::c_char {
    return Info_ValueForKey(cl.serverinfo.as_mut_ptr(), key);
}
/*
=============
pfnGetClientMaxspeed

value that come from server
=============
*/
unsafe extern "C" fn pfnGetClientMaxspeed() -> libc::c_float {
    return cl.local.maxspeed;
}
/*
=============
pfnIsNoClipping

=============
*/
unsafe extern "C" fn pfnIsNoClipping() -> libc::c_int {
    return (cl.frames[cl.parsecountmod as
                          usize].playerstate[cl.playernum as usize].movetype
                == 8 as libc::c_int) as libc::c_int;
}
/*
=============
pfnGetViewModel

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetViewModel() -> *mut cl_entity_t {
    return &mut clgame.viewent;
}
/*
=============
pfnGetClientTime

=============
*/
unsafe extern "C" fn pfnGetClientTime() -> libc::c_float {
    return cl.time as libc::c_float;
}
/*
=============
pfnCalcShake

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCalcShake() {
    let mut i: libc::c_int = 0;
    let mut fraction: libc::c_float = 0.;
    let mut freq: libc::c_float = 0.;
    let mut localAmp: libc::c_float = 0.;
    if clgame.shake.time == 0 as libc::c_int as libc::c_float { return }
    if cl.time > clgame.shake.time as libc::c_double ||
           clgame.shake.amplitude <= 0 as libc::c_int as libc::c_float ||
           clgame.shake.frequency <= 0 as libc::c_int as libc::c_float {
        memset(&mut clgame.shake as *mut screen_shake_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<screen_shake_t>() as libc::c_ulong);
        return
    }
    if cl.time > clgame.shake.next_shake as libc::c_double {
        // higher frequency means we recalc the extents more often and perturb the display again
        clgame.shake.next_shake =
            (cl.time + (1.0f32 / clgame.shake.frequency) as libc::c_double) as
                libc::c_float;
        // compute random shake extents (the shake will settle down from this)
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            clgame.shake.offset[i as usize] =
                COM_RandomFloat(-clgame.shake.amplitude,
                                clgame.shake.amplitude);
            i += 1
        }
        clgame.shake.angle =
            COM_RandomFloat(-clgame.shake.amplitude * 0.25f32,
                            clgame.shake.amplitude * 0.25f32)
    }
    // ramp down amplitude over duration (fraction goes from 1 to 0 linearly with slope 1/duration)
    fraction =
        ((clgame.shake.time as libc::c_double - cl.time) /
             clgame.shake.duration as libc::c_double) as libc::c_float;
    // ramp up frequency over duration
    if fraction != 0. {
        freq = clgame.shake.frequency / fraction
    } else { freq = 0 as libc::c_int as libc::c_float }
    // square fraction to approach zero more quickly
    fraction *= fraction;
    // Sine wave that slowly settles to zero
    fraction =
        (fraction as libc::c_double *
             __tg_sin(cl.time * freq as libc::c_double)) as libc::c_float;
    // add to view origin
    clgame.shake.applied_offset[0 as libc::c_int as usize] =
        clgame.shake.offset[0 as libc::c_int as usize] * fraction;
    clgame.shake.applied_offset[1 as libc::c_int as usize] =
        clgame.shake.offset[1 as libc::c_int as usize] * fraction;
    clgame.shake.applied_offset[2 as libc::c_int as usize] =
        clgame.shake.offset[2 as libc::c_int as usize] * fraction;
    // add to roll
    clgame.shake.applied_angle = clgame.shake.angle * fraction;
    // drop amplitude a bit, less for higher frequency shakes
    localAmp =
        (clgame.shake.amplitude as libc::c_double *
             (host.frametime /
                  (clgame.shake.duration * clgame.shake.frequency) as
                      libc::c_double)) as libc::c_float;
    clgame.shake.amplitude -= localAmp;
}
/*
=============
pfnApplyShake

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnApplyShake(mut origin: *mut libc::c_float,
                                       mut angles: *mut libc::c_float,
                                       mut factor: libc::c_float) {
    if !origin.is_null() {
        *origin.offset(0 as libc::c_int as isize) =
            *origin.offset(0 as libc::c_int as isize) +
                factor *
                    clgame.shake.applied_offset[0 as libc::c_int as usize];
        *origin.offset(1 as libc::c_int as isize) =
            *origin.offset(1 as libc::c_int as isize) +
                factor *
                    clgame.shake.applied_offset[1 as libc::c_int as usize];
        *origin.offset(2 as libc::c_int as isize) =
            *origin.offset(2 as libc::c_int as isize) +
                factor *
                    clgame.shake.applied_offset[2 as libc::c_int as usize]
    }
    if !angles.is_null() {
        *angles.offset(2 as libc::c_int as isize) +=
            clgame.shake.applied_angle * factor
    };
}
/*
=============
pfnIsSpectateOnly

=============
*/
unsafe extern "C" fn pfnIsSpectateOnly() -> libc::c_int {
    return (cls.spectator as libc::c_uint != 0 as libc::c_int as libc::c_uint)
               as libc::c_int;
}
/*
=============
pfnPointContents

=============
*/
unsafe extern "C" fn pfnPointContents(mut p: *const libc::c_float,
                                      mut truecontents: *mut libc::c_int)
 -> libc::c_int {
    let mut cont: libc::c_int = 0;
    let mut truecont: libc::c_int = 0;
    cont = PM_PointContents(clgame.pmove, p);
    truecont = cont;
    if !truecontents.is_null() { *truecontents = truecont }
    if cont <= -(9 as libc::c_int) && cont >= -(14 as libc::c_int) {
        cont = -(3 as libc::c_int)
    }
    return cont;
}
/*
=============
pfnTraceLine

=============
*/
unsafe extern "C" fn pfnTraceLine(mut start: *mut libc::c_float,
                                  mut end: *mut libc::c_float,
                                  mut flags: libc::c_int,
                                  mut usehull: libc::c_int,
                                  mut ignore_pe: libc::c_int)
 -> *mut pmtrace_t {
    static mut tr: pmtrace_t =
        pmtrace_t{allsolid: false_0,
                  startsolid: false_0,
                  inopen: false_0,
                  inwater: false_0,
                  fraction: 0.,
                  endpos: [0.; 3],
                  plane: pmplane_t{normal: [0.; 3], dist: 0.,},
                  ent: 0,
                  deltavelocity: [0.; 3],
                  hitgroup: 0,};
    let mut old_usehull: libc::c_int = 0;
    old_usehull = (*clgame.pmove).usehull;
    (*clgame.pmove).usehull = usehull;
    match flags {
        0 => {
            tr =
                PM_PlayerTraceExt(clgame.pmove, start, end, 0 as libc::c_int,
                                  (*clgame.pmove).numphysent,
                                  (*clgame.pmove).physents.as_mut_ptr(),
                                  ignore_pe, None)
        }
        1 => {
            tr =
                PM_PlayerTraceExt(clgame.pmove, start, end, 0 as libc::c_int,
                                  (*clgame.pmove).numvisent,
                                  (*clgame.pmove).visents.as_mut_ptr(),
                                  ignore_pe, None)
        }
        _ => { }
    }
    (*clgame.pmove).usehull = old_usehull;
    return &mut tr;
}
unsafe extern "C" fn pfnPlaySoundByNameAtLocation(mut szSound:
                                                      *mut libc::c_char,
                                                  mut volume: libc::c_float,
                                                  mut origin:
                                                      *mut libc::c_float) {
    let mut hSound: libc::c_int = S_RegisterSound(szSound);
    S_StartSound(origin as *const vec_t, cl.viewentity, 0 as libc::c_int,
                 hSound, volume, 0.8f64 as libc::c_float, 100 as libc::c_int,
                 0 as libc::c_int);
}
/*
=============
pfnPrecacheEvent

=============
*/
unsafe extern "C" fn pfnPrecacheEvent(mut type_0: libc::c_int,
                                      mut psz: *const libc::c_char) -> word {
    return CL_EventIndex(psz);
}
/*
=============
pfnHookEvent

=============
*/
unsafe extern "C" fn pfnHookEvent(mut filename: *const libc::c_char,
                                  mut pfn: pfnEventHook) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut ev: *mut cl_user_event_t = 0 as *mut cl_user_event_t;
    let mut i: libc::c_int = 0;
    // ignore blank names
    if filename.is_null() || *filename == 0 { return }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    // find an empty slot
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int {
        ev = clgame.events[i as usize];
        if ev.is_null() { break ; }
        if Q_strnicmp(name.as_mut_ptr(), (*ev).name.as_mut_ptr(),
                      99999 as libc::c_int) == 0 && (*ev).func.is_some() {
            Con_Reportf(b"^3Warning:^7 CL_HookEvent: %s already hooked!\n\x00"
                            as *const u8 as *const libc::c_char,
                        name.as_mut_ptr());
            return
        }
        i += 1
    }
    CL_RegisterEvent(i, name.as_mut_ptr(), pfn);
}
/*
=============
pfnKillEvent

=============
*/
unsafe extern "C" fn pfnKillEvents(mut entnum: libc::c_int,
                                   mut eventname: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut es: *mut event_state_t = 0 as *mut event_state_t;
    let mut ei: *mut event_info_t = 0 as *mut event_info_t;
    let mut eventIndex: libc::c_int = CL_EventIndex(eventname) as libc::c_int;
    if eventIndex < 0 as libc::c_int ||
           eventIndex >= (1 as libc::c_int) << 10 as libc::c_int {
        return
    }
    if entnum < 0 as libc::c_int || entnum > clgame.maxEntities { return }
    es = &mut cl.events;
    // find all events with specified index and kill it
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        ei =
            &mut *(*es).ei.as_mut_ptr().offset(i as isize) as
                *mut event_info_t;
        if (*ei).index as libc::c_int == eventIndex &&
               (*ei).entity_index as libc::c_int == entnum {
            CL_ResetEvent(ei);
            break ;
        } else { i += 1 }
    };
}
/*
=============
pfnPlaySound

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPlaySound(mut ent: libc::c_int,
                                      mut org: *mut libc::c_float,
                                      mut chan: libc::c_int,
                                      mut samp: *const libc::c_char,
                                      mut vol: libc::c_float,
                                      mut attn: libc::c_float,
                                      mut flags: libc::c_int,
                                      mut pitch: libc::c_int) {
    S_StartSound(org as *const vec_t, ent, chan, S_RegisterSound(samp), vol,
                 attn, pitch, flags);
}
/*
=============
CL_FindModelIndex

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FindModelIndex(mut m: *const libc::c_char)
 -> libc::c_int {
    let mut filepath: [libc::c_char; 64] = [0; 64];
    static mut lasttimewarn: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if if m.is_null() || *m == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    Q_strncpy(filepath.as_mut_ptr(), m,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(filepath.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < cl.nummodels {
        if !cl.models[(i + 1 as libc::c_int) as usize].is_null() {
            if Q_strnicmp((*cl.models[(i + 1 as libc::c_int) as
                                          usize]).name.as_mut_ptr(),
                          filepath.as_mut_ptr(), 99999 as libc::c_int) == 0 {
                return i + 1 as libc::c_int
            }
        }
        i += 1
    }
    if (lasttimewarn as libc::c_double) < host.realtime {
        // tell user about problem (but don't spam console)
        Con_Printf(b"^1Error:^7 Could not find index for model %s: not precached\n\x00"
                       as *const u8 as *const libc::c_char,
                   filepath.as_mut_ptr());
        lasttimewarn =
            (host.realtime + 1.0f32 as libc::c_double) as libc::c_float
    }
    return 0 as libc::c_int;
}
/*
=============
pfnIsLocal

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnIsLocal(mut playernum: libc::c_int)
 -> libc::c_int {
    if playernum == cl.playernum { return true_0 as libc::c_int }
    return false_0 as libc::c_int;
}
/*
=============
pfnLocalPlayerDucking

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnLocalPlayerDucking() -> libc::c_int {
    return if cl.local.usehull == 1 as libc::c_int {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int };
}
/*
=============
pfnLocalPlayerViewheight

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnLocalPlayerViewheight(mut view_ofs:
                                                      *mut libc::c_float) {
    if !view_ofs.is_null() {
        *view_ofs.offset(0 as libc::c_int as isize) =
            cl.viewheight[0 as libc::c_int as usize];
        *view_ofs.offset(1 as libc::c_int as isize) =
            cl.viewheight[1 as libc::c_int as usize];
        *view_ofs.offset(2 as libc::c_int as isize) =
            cl.viewheight[2 as libc::c_int as usize]
    };
}
/*
=============
pfnLocalPlayerBounds

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnLocalPlayerBounds(mut hull: libc::c_int,
                                              mut mins: *mut libc::c_float,
                                              mut maxs: *mut libc::c_float) {
    if hull >= 0 as libc::c_int && hull < 4 as libc::c_int {
        if !mins.is_null() {
            *mins.offset(0 as libc::c_int as isize) =
                (*clgame.pmove).player_mins[hull as
                                                usize][0 as libc::c_int as
                                                           usize];
            *mins.offset(1 as libc::c_int as isize) =
                (*clgame.pmove).player_mins[hull as
                                                usize][1 as libc::c_int as
                                                           usize];
            *mins.offset(2 as libc::c_int as isize) =
                (*clgame.pmove).player_mins[hull as
                                                usize][2 as libc::c_int as
                                                           usize]
        }
        if !maxs.is_null() {
            *maxs.offset(0 as libc::c_int as isize) =
                (*clgame.pmove).player_maxs[hull as
                                                usize][0 as libc::c_int as
                                                           usize];
            *maxs.offset(1 as libc::c_int as isize) =
                (*clgame.pmove).player_maxs[hull as
                                                usize][1 as libc::c_int as
                                                           usize];
            *maxs.offset(2 as libc::c_int as isize) =
                (*clgame.pmove).player_maxs[hull as
                                                usize][2 as libc::c_int as
                                                           usize]
        }
    };
}
/*
=============
pfnIndexFromTrace

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnIndexFromTrace(mut pTrace: *mut pmtrace_s)
 -> libc::c_int {
    // Velaron: breaks compatibility with mods that call the function after CL_PopPMStates
    return (*clgame.pmove).physents[(*pTrace).ent as usize].info;
}
/*
=============
pfnGetPhysent

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPhysent(mut idx: libc::c_int)
 -> *mut physent_t {
    if idx >= 0 as libc::c_int && idx < (*clgame.pmove).numphysent {
        // return physent
        return &mut *(*clgame.pmove).physents.as_mut_ptr().offset(idx as
                                                                      isize)
                   as *mut physent_t
    }
    return 0 as *mut physent_t;
}
/*
=============
pfnGetVisent

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetVisent(mut idx: libc::c_int)
 -> *mut physent_t {
    if idx >= 0 as libc::c_int && idx < (*clgame.pmove).numvisent {
        // return physent
        return &mut *(*clgame.pmove).visents.as_mut_ptr().offset(idx as isize)
                   as *mut physent_t
    }
    return 0 as *mut physent_t;
}
/*
=============
pfnSetTraceHull

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetTraceHull(mut hull: libc::c_int) {
    (*clgame.pmove).usehull =
        if hull >= 0 as libc::c_int {
            if hull < 3 as libc::c_int { hull } else { 3 as libc::c_int }
        } else { 0 as libc::c_int };
}
/*
=============
pfnPlayerTrace

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PlayerTrace(mut start: *mut libc::c_float,
                                        mut end: *mut libc::c_float,
                                        mut traceFlags: libc::c_int,
                                        mut ignore_pe: libc::c_int,
                                        mut tr: *mut pmtrace_t) {
    if tr.is_null() { return }
    *tr =
        PM_PlayerTraceExt(clgame.pmove, start, end, traceFlags,
                          (*clgame.pmove).numphysent,
                          (*clgame.pmove).physents.as_mut_ptr(), ignore_pe,
                          None);
}
/*
=============
pfnPlayerTraceExt

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PlayerTraceExt(mut start: *mut libc::c_float,
                                           mut end: *mut libc::c_float,
                                           mut traceFlags: libc::c_int,
                                           mut pfnIgnore:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut physent_t)
                                                          -> libc::c_int>,
                                           mut tr: *mut pmtrace_t) {
    if tr.is_null() { return }
    *tr =
        PM_PlayerTraceExt(clgame.pmove, start, end, traceFlags,
                          (*clgame.pmove).numphysent,
                          (*clgame.pmove).physents.as_mut_ptr(),
                          -(1 as libc::c_int), pfnIgnore);
}
/*
=============
pfnTraceTexture

=============
*/
unsafe extern "C" fn pfnTraceTexture(mut ground: libc::c_int,
                                     mut vstart: *mut libc::c_float,
                                     mut vend: *mut libc::c_float)
 -> *const libc::c_char {
    let mut pe: *mut physent_t = 0 as *mut physent_t; // bad ground
    if ground < 0 as libc::c_int || ground >= (*clgame.pmove).numphysent {
        return 0 as *const libc::c_char
    }
    pe =
        &mut *(*clgame.pmove).physents.as_mut_ptr().offset(ground as isize) as
            *mut physent_t;
    return PM_TraceTexture(pe, vstart, vend);
}
/*
=============
pfnTraceSurface

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnTraceSurface(mut ground: libc::c_int,
                                         mut vstart: *mut libc::c_float,
                                         mut vend: *mut libc::c_float)
 -> *mut msurface_s {
    let mut pe: *mut physent_t = 0 as *mut physent_t; // bad ground
    if ground < 0 as libc::c_int || ground >= (*clgame.pmove).numphysent {
        return 0 as *mut msurface_s
    }
    pe =
        &mut *(*clgame.pmove).physents.as_mut_ptr().offset(ground as isize) as
            *mut physent_t;
    return PM_TraceSurface(pe, vstart, vend);
}
/*
=============
pfnGetMovevars

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetMoveVars() -> *mut movevars_t {
    return &mut clgame.movevars;
}
/*
=============
pfnStopAllSounds

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnStopAllSounds(mut ent: libc::c_int,
                                          mut entchannel: libc::c_int) {
    S_StopSound(ent, entchannel, 0 as *const libc::c_char);
}
/*
=============
CL_LoadModel

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LoadModel(mut modelname: *const libc::c_char,
                                      mut index: *mut libc::c_int)
 -> *mut model_t {
    let mut i: libc::c_int = 0;
    if !index.is_null() { *index = -(1 as libc::c_int) }
    i = CL_FindModelIndex(modelname);
    if i == 0 as libc::c_int { return 0 as *mut model_t }
    if !index.is_null() { *index = i }
    return CL_ModelHandle(i);
}
#[no_mangle]
pub unsafe extern "C" fn CL_AddEntity(mut entityType: libc::c_int,
                                      mut pEnt: *mut cl_entity_t)
 -> libc::c_int {
    if pEnt.is_null() { return false_0 as libc::c_int }
    // clear effects for all temp entities
    if (*pEnt).index == 0 { (*pEnt).curstate.effects = 0 as libc::c_int }
    // let the render reject entity without model
    return CL_AddVisibleEntity(pEnt, entityType) as libc::c_int;
}
/*
=============
pfnGetGameDirectory

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetGameDirectory() -> *const libc::c_char {
    static mut szGetGameDir: [libc::c_char; 1024] = [0; 1024];
    Q_strncpy(szGetGameDir.as_mut_ptr(),
              (*SI.GameInfo).gamefolder.as_mut_ptr(),
              99999 as libc::c_int as size_t);
    return szGetGameDir.as_mut_ptr();
}
/*
=============
Key_LookupBinding

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Key_LookupBinding(mut pBinding: *const libc::c_char)
 -> *const libc::c_char {
    return Key_KeynumToString(Key_GetKey(pBinding));
}
/*
=============
pfnGetLevelName

=============
*/
unsafe extern "C" fn pfnGetLevelName() -> *const libc::c_char {
    static mut mapname: [libc::c_char; 64] = [0; 64]; // not in game
    if cls.state as libc::c_uint >=
           ca_connected as libc::c_int as libc::c_uint {
        Q_snprintf(mapname.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong,
                   b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
                   clgame.mapname.as_mut_ptr());
    } else {
        mapname[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    return mapname.as_mut_ptr();
}
/*
=============
pfnGetScreenFade

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetScreenFade(mut fade: *mut screenfade_s) {
    if !fade.is_null() { *fade = clgame.fade };
}
/*
=============
pfnSetScreenFade

=============
*/
unsafe extern "C" fn pfnSetScreenFade(mut fade: *mut screenfade_s) {
    if !fade.is_null() { clgame.fade = *fade };
}
/*
=============
pfnLoadMapSprite

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnLoadMapSprite(mut filename: *const libc::c_char)
 -> *mut model_t {
    return CL_LoadSpriteModel(filename, 2 as libc::c_int as uint,
                              0 as libc::c_int as uint);
}
/*
=============
PlayerInfo_ValueForKey

=============
*/
#[no_mangle]
pub unsafe extern "C" fn PlayerInfo_ValueForKey(mut playerNum: libc::c_int,
                                                mut key: *const libc::c_char)
 -> *const libc::c_char {
    // find the player
    if playerNum > cl.maxclients || playerNum < 1 as libc::c_int {
        return 0 as *const libc::c_char
    }
    if cl.players[(playerNum - 1 as libc::c_int) as
                      usize].name[0 as libc::c_int as usize] == 0 {
        return 0 as *const libc::c_char
    }
    return Info_ValueForKey(cl.players[(playerNum - 1 as libc::c_int) as
                                           usize].userinfo.as_mut_ptr(), key);
}
/*
=============
PlayerInfo_SetValueForKey

=============
*/
#[no_mangle]
pub unsafe extern "C" fn PlayerInfo_SetValueForKey(mut key:
                                                       *const libc::c_char,
                                                   mut value:
                                                       *const libc::c_char) {
    let mut var: *mut convar_t = 0 as *mut convar_t; // no changes ?
    if Q_strncmp(Info_ValueForKey(cls.userinfo.as_mut_ptr(), key), value,
                 99999 as libc::c_int) == 0 {
        return
    }
    var = Cvar_FindVarExt(key, 0 as libc::c_int);
    if !var.is_null() &&
           (*var).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        Cvar_DirectSet(var, value);
    } else if Info_SetValueForStarKey(cls.userinfo.as_mut_ptr(), key, value,
                                      256 as libc::c_int) as u64 != 0 {
        // time to update server copy of userinfo
        CL_ServerCommand(true_0,
                         b"setinfo \"%s\" \"%s\"\n\x00" as *const u8 as
                             *const libc::c_char, key, value);
    };
}
/*
=============
pfnGetPlayerUniqueID

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPlayerUniqueID(mut iPlayer: libc::c_int,
                                              mut playerID: *mut libc::c_char)
 -> qboolean {
    if iPlayer < 1 as libc::c_int || iPlayer > cl.maxclients {
        return false_0
    }
    // make sure there is a player here..
    if cl.players[(iPlayer - 1 as libc::c_int) as
                      usize].userinfo[0 as libc::c_int as usize] == 0 ||
           cl.players[(iPlayer - 1 as libc::c_int) as
                          usize].name[0 as libc::c_int as usize] == 0 {
        return false_0
    }
    memcpy(playerID as *mut libc::c_void,
           cl.players[(iPlayer - 1 as libc::c_int) as
                          usize].hashedcdkey.as_mut_ptr() as
               *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    return true_0;
}
/*
=============
pfnGetTrackerIDForPlayer

obsolete, unused
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetTrackerIDForPlayer(mut playerSlot: libc::c_int)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/*
=============
pfnGetPlayerForTrackerID

obsolete, unused
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetPlayerForTrackerID(mut trackerID: libc::c_int)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/*
=============
pfnServerCmdUnreliable

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnServerCmdUnreliable(mut szCmdString:
                                                    *mut libc::c_char)
 -> libc::c_int {
    if if szCmdString.is_null() || *szCmdString == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    MSG_WriteCmdExt(&mut cls.datagram, 3 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut cls.datagram, szCmdString);
    return 1 as libc::c_int;
}
/*
=============
pfnGetMousePos

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetMousePos(mut ppt: *mut tagPOINT) {
    if ppt.is_null() { return }
    Platform_GetMousePos(&mut (*ppt).x, &mut (*ppt).y);
}
/*
=============
pfnSetMouseEnable

legacy of dinput code
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSetMouseEnable(mut fEnable: qboolean) { }
/*
=============
pfnGetServerTime

=============
*/
unsafe extern "C" fn pfnGetClientOldTime() -> libc::c_float {
    return cl.oldtime as libc::c_float;
}
/*
=============
pfnGetGravity

=============
*/
unsafe extern "C" fn pfnGetGravity() -> libc::c_float {
    return clgame.movevars.gravity;
}
/*
=============
pfnEnableTexSort

TODO: implement
=============
*/
unsafe extern "C" fn pfnEnableTexSort(mut enable: libc::c_int) { }
/*
=============
pfnSetLightmapColor

TODO: implement
=============
*/
unsafe extern "C" fn pfnSetLightmapColor(mut red: libc::c_float,
                                         mut green: libc::c_float,
                                         mut blue: libc::c_float) {
}
/*
=============
pfnSetLightmapScale

TODO: implement
=============
*/
unsafe extern "C" fn pfnSetLightmapScale(mut scale: libc::c_float) { }
/*
=============
pfnSPR_DrawGeneric

=============
*/
unsafe extern "C" fn pfnSPR_DrawGeneric(mut frame: libc::c_int,
                                        mut x: libc::c_int,
                                        mut y: libc::c_int,
                                        mut prc: *const wrect_t,
                                        mut blendsrc: libc::c_int,
                                        mut blenddst: libc::c_int,
                                        mut width: libc::c_int,
                                        mut height: libc::c_int) {
    // REFTODO:
    SPR_DrawGeneric(frame, x as libc::c_float, y as libc::c_float,
                    width as libc::c_float, height as libc::c_float, prc);
}
/*
=============
LocalPlayerInfo_ValueForKey

=============
*/
unsafe extern "C" fn LocalPlayerInfo_ValueForKey(mut key: *const libc::c_char)
 -> *const libc::c_char {
    return Info_ValueForKey(cls.userinfo.as_mut_ptr(), key);
}
/*
=============
pfnVGUI2DrawCharacter

=============
*/
unsafe extern "C" fn pfnVGUI2DrawCharacter(mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut number: libc::c_int,
                                           mut font: libc::c_uint)
 -> libc::c_int {
    if cls.creditsFont.valid as u64 == 0 { return 0 as libc::c_int }
    number &= 255 as libc::c_int;
    number = Con_UtfProcessChar(number);
    if number < 32 as libc::c_int { return 0 as libc::c_int }
    if y < -clgame.scrInfo.iCharHeight { return 0 as libc::c_int }
    clgame.ds.adjust_size = true_0;
    gameui.ds.gl_texturenum = cls.creditsFont.hFontTexture;
    pfnPIC_DrawAdditive(x, y, -(1 as libc::c_int), -(1 as libc::c_int),
                        &mut *cls.creditsFont.fontRc.as_mut_ptr().offset(number
                                                                             as
                                                                             isize));
    clgame.ds.adjust_size = false_0;
    return clgame.scrInfo.charWidths[number as usize] as libc::c_int;
}
/*
=============
pfnVGUI2DrawCharacterAdditive

=============
*/
unsafe extern "C" fn pfnVGUI2DrawCharacterAdditive(mut x: libc::c_int,
                                                   mut y: libc::c_int,
                                                   mut ch: libc::c_int,
                                                   mut r: libc::c_int,
                                                   mut g: libc::c_int,
                                                   mut b: libc::c_int,
                                                   mut font: libc::c_uint)
 -> libc::c_int {
    if (*hud_utf8).value == 0. { ch = Con_UtfProcessChar(ch) }
    return pfnDrawCharacter(x, y, ch, r, g, b);
}
/*
=============
pfnDrawString

=============
*/
unsafe extern "C" fn pfnDrawString(mut x: libc::c_int, mut y: libc::c_int,
                                   mut str: *const libc::c_char,
                                   mut r: libc::c_int, mut g: libc::c_int,
                                   mut b: libc::c_int) -> libc::c_int {
    Con_UtfProcessChar(0 as libc::c_int);
    // draw the string until we hit the null character or a newline character
    while *str as libc::c_int != 0 as libc::c_int &&
              *str as libc::c_int != '\n' as i32 {
        x +=
            pfnVGUI2DrawCharacterAdditive(x, y,
                                          *str as libc::c_uchar as
                                              libc::c_int, r, g, b,
                                          0 as libc::c_int as libc::c_uint);
        str = str.offset(1)
    }
    return x;
}
/*
=============
pfnDrawStringReverse

=============
*/
unsafe extern "C" fn pfnDrawStringReverse(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut str: *const libc::c_char,
                                          mut r: libc::c_int,
                                          mut g: libc::c_int,
                                          mut b: libc::c_int) -> libc::c_int {
    // find the end of the string
    let mut szIt: *mut libc::c_char = 0 as *mut libc::c_char;
    szIt = str as *mut libc::c_char;
    while *szIt as libc::c_int != 0 as libc::c_int {
        x -=
            clgame.scrInfo.charWidths[*szIt as libc::c_uchar as usize] as
                libc::c_int;
        szIt = szIt.offset(1)
    }
    pfnDrawString(x, y, str, r, g, b);
    return x;
}
/*
=============
GetCareerGameInterface

=============
*/
unsafe extern "C" fn GetCareerGameInterface() -> *mut libc::c_void {
    Con_Printf(b"^1Career GameInterface called!\n\x00" as *const u8 as
                   *const libc::c_char);
    return 0 as *mut libc::c_void;
}
/*
=============
pfnPlaySoundVoiceByName

=============
*/
unsafe extern "C" fn pfnPlaySoundVoiceByName(mut filename: *mut libc::c_char,
                                             mut volume: libc::c_float,
                                             mut pitch: libc::c_int) {
    let mut hSound: libc::c_int = S_RegisterSound(filename);
    S_StartSound(0 as *const vec_t, cl.viewentity,
                 500 as libc::c_int + 1 as libc::c_int, hSound, volume,
                 1.0f64 as libc::c_float, pitch,
                 (1 as libc::c_int) << 10 as libc::c_int);
}
/*
=============
pfnMP3_InitStream

=============
*/
unsafe extern "C" fn pfnMP3_InitStream(mut filename: *mut libc::c_char,
                                       mut looping: libc::c_int) {
    if filename.is_null() { S_StopBackgroundTrack(); return }
    if looping != 0 {
        S_StartBackgroundTrack(filename, filename, 0 as libc::c_int, false_0);
    } else {
        S_StartBackgroundTrack(filename, 0 as *const libc::c_char,
                               0 as libc::c_int, false_0);
    };
}
/*
=============
pfnPlaySoundByNameAtPitch

=============
*/
unsafe extern "C" fn pfnPlaySoundByNameAtPitch(mut filename:
                                                   *mut libc::c_char,
                                               mut volume: libc::c_float,
                                               mut pitch: libc::c_int) {
    let mut hSound: libc::c_int = S_RegisterSound(filename);
    S_StartSound(0 as *const vec_t, cl.viewentity, 3 as libc::c_int, hSound,
                 volume, 1.0f64 as libc::c_float, pitch,
                 (1 as libc::c_int) << 10 as libc::c_int);
}
/*
=============
pfnFillRGBABlend

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FillRGBABlend(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut w: libc::c_int,
                                          mut h: libc::c_int,
                                          mut r: libc::c_int,
                                          mut g: libc::c_int,
                                          mut b: libc::c_int,
                                          mut a: libc::c_int) {
    let mut _x: libc::c_float = x as libc::c_float;
    let mut _y: libc::c_float = y as libc::c_float;
    let mut _w: libc::c_float = w as libc::c_float;
    let mut _h: libc::c_float = h as libc::c_float;
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
    SPR_AdjustSize(&mut _x, &mut _y, &mut _w, &mut _h);
    // REFTODO:
    ref_0.dllFuncs.FillRGBABlend.expect("non-null function pointer")(_x, _y,
                                                                     _w, _h,
                                                                     r, g, b,
                                                                     a);
}
/*
=============
pfnGetAppID

=============
*/
unsafe extern "C" fn pfnGetAppID() -> libc::c_int {
    return 70 as libc::c_int;
    // Half-Life AppID
}
/*
=============
pfnVguiWrap2_GetMouseDelta

TODO: implement
=============
*/
unsafe extern "C" fn pfnVguiWrap2_GetMouseDelta(mut x: *mut libc::c_int,
                                                mut y: *mut libc::c_int) {
}
/*
=============
pfnParseFile

handle colon separately
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnParseFile(mut data: *mut libc::c_char,
                                      mut token: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    out =
        _COM_ParseFileSafe(data, token, 2147483647 as libc::c_int,
                           ((1 as libc::c_int) << 1 as libc::c_int) as
                               libc::c_uint, 0 as *mut libc::c_int);
    return out;
}
/*
=================
TriAPI implementation

=================
*/
/*
=================
TriRenderMode
=================
*/
#[no_mangle]
pub unsafe extern "C" fn TriRenderMode(mut mode: libc::c_int) {
    clgame.ds.renderMode = mode;
    ref_0.dllFuncs.TriRenderMode.expect("non-null function pointer")(mode);
}
/*
=================
TriColor4f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn TriColor4f(mut r: libc::c_float,
                                    mut g: libc::c_float,
                                    mut b: libc::c_float,
                                    mut a: libc::c_float) {
    if clgame.ds.renderMode == kRenderTransAlpha as libc::c_int {
        ref_0.dllFuncs.Color4ub.expect("non-null function pointer")((r *
                                                                         255.9f32)
                                                                        as
                                                                        libc::c_uchar,
                                                                    (g *
                                                                         255.9f32)
                                                                        as
                                                                        libc::c_uchar,
                                                                    (b *
                                                                         255.9f32)
                                                                        as
                                                                        libc::c_uchar,
                                                                    (a *
                                                                         255.0f32)
                                                                        as
                                                                        libc::c_uchar);
    } else {
        ref_0.dllFuncs.Color4f.expect("non-null function pointer")(r * a,
                                                                   g * a,
                                                                   b * a,
                                                                   1.0f64 as
                                                                       libc::c_float);
    }
    clgame.ds.triRGBA[0 as libc::c_int as usize] = r;
    clgame.ds.triRGBA[1 as libc::c_int as usize] = g;
    clgame.ds.triRGBA[2 as libc::c_int as usize] = b;
    clgame.ds.triRGBA[3 as libc::c_int as usize] = a;
}
/*
=============
TriColor4ub
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriColor4ub(mut r: byte, mut g: byte, mut b: byte,
                                     mut a: byte) {
    clgame.ds.triRGBA[0 as libc::c_int as usize] =
        r as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    clgame.ds.triRGBA[1 as libc::c_int as usize] =
        g as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    clgame.ds.triRGBA[2 as libc::c_int as usize] =
        b as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    clgame.ds.triRGBA[3 as libc::c_int as usize] =
        a as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    ref_0.dllFuncs.Color4f.expect("non-null function pointer")(clgame.ds.triRGBA[0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     usize],
                                                               clgame.ds.triRGBA[1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     usize],
                                                               clgame.ds.triRGBA[2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     usize],
                                                               1.0f32);
}
/*
=============
TriBrightness
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriBrightness(mut brightness: libc::c_float) {
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    r =
        clgame.ds.triRGBA[0 as libc::c_int as usize] *
            clgame.ds.triRGBA[3 as libc::c_int as usize] * brightness;
    g =
        clgame.ds.triRGBA[1 as libc::c_int as usize] *
            clgame.ds.triRGBA[3 as libc::c_int as usize] * brightness;
    b =
        clgame.ds.triRGBA[2 as libc::c_int as usize] *
            clgame.ds.triRGBA[3 as libc::c_int as usize] * brightness;
    ref_0.dllFuncs.Color4f.expect("non-null function pointer")(r, g, b,
                                                               1.0f32);
}
/*
=============
TriCullFace
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriCullFace(mut style: TRICULLSTYLE) {
    clgame.ds.cullMode = style;
    ref_0.dllFuncs.CullFace.expect("non-null function pointer")(style);
}
/*
=============
TriWorldToScreen
convert world coordinates (x,y,z) into screen (x, y)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriWorldToScreen(mut world_0: *const libc::c_float,
                                          mut screen: *mut libc::c_float)
 -> libc::c_int {
    let mut retval: libc::c_int = 0;
    retval =
        ref_0.dllFuncs.WorldToScreen.expect("non-null function pointer")(world_0,
                                                                         screen);
    *screen.offset(0 as libc::c_int as isize) =
        0.5f32 * *screen.offset(0 as libc::c_int as isize) *
            clgame.viewport[2 as libc::c_int as usize] as libc::c_float;
    *screen.offset(1 as libc::c_int as isize) =
        -0.5f32 * *screen.offset(1 as libc::c_int as isize) *
            clgame.viewport[3 as libc::c_int as usize] as libc::c_float;
    *screen.offset(0 as libc::c_int as isize) +=
        0.5f32 * clgame.viewport[2 as libc::c_int as usize] as libc::c_float;
    *screen.offset(1 as libc::c_int as isize) +=
        0.5f32 * clgame.viewport[3 as libc::c_int as usize] as libc::c_float;
    return retval;
}
/*
=============
TriBoxInPVS

check box in pvs (absmin, absmax)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriBoxInPVS(mut mins: *mut libc::c_float,
                                     mut maxs: *mut libc::c_float)
 -> libc::c_int {
    return Mod_BoxVisible(mins as *const vec_t, maxs as *const vec_t,
                          ref_0.dllFuncs.Mod_GetCurrentVis.expect("non-null function pointer")())
               as libc::c_int;
}
/*
=============
TriLightAtPoint
NOTE: dlights are ignored
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriLightAtPoint(mut pos: *mut libc::c_float,
                                         mut value: *mut libc::c_float) {
    let mut vLightColor: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
    if pos.is_null() || value.is_null() { return }
    vLightColor =
        ref_0.dllFuncs.R_LightPoint.expect("non-null function pointer")(pos);
    *value.offset(0 as libc::c_int as isize) = vLightColor.r as libc::c_float;
    *value.offset(1 as libc::c_int as isize) = vLightColor.g as libc::c_float;
    *value.offset(2 as libc::c_int as isize) = vLightColor.b as libc::c_float;
}
/*
=============
TriColor4fRendermode
Heavy legacy of Quake...
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriColor4fRendermode(mut r: libc::c_float,
                                              mut g: libc::c_float,
                                              mut b: libc::c_float,
                                              mut a: libc::c_float,
                                              mut rendermode: libc::c_int) {
    if clgame.ds.renderMode == kRenderTransAlpha as libc::c_int {
        clgame.ds.triRGBA[3 as libc::c_int as usize] = a / 255.0f32;
        ref_0.dllFuncs.Color4f.expect("non-null function pointer")(r, g, b,
                                                                   a);
    } else {
        ref_0.dllFuncs.Color4f.expect("non-null function pointer")(r * a,
                                                                   g * a,
                                                                   b * a,
                                                                   1.0f32);
    };
}
/*
=============
TriSpriteTexture

bind current texture
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriSpriteTexture(mut pSpriteModel: *mut model_t,
                                          mut frame: libc::c_int)
 -> libc::c_int {
    let mut gl_texturenum: libc::c_int = 0;
    gl_texturenum =
        ref_0.dllFuncs.R_GetSpriteTexture.expect("non-null function pointer")(pSpriteModel,
                                                                              frame);
    if gl_texturenum <= 0 as libc::c_int { return 0 as libc::c_int }
    ref_0.dllFuncs.GL_Bind.expect("non-null function pointer")(XASH_TEXTURE0
                                                                   as
                                                                   libc::c_int,
                                                               gl_texturenum
                                                                   as
                                                                   libc::c_uint);
    return 1 as libc::c_int;
}
/*
=================
DemoApi implementation

=================
*/
/*
=================
Demo_IsRecording

=================
*/
unsafe extern "C" fn Demo_IsRecording() -> libc::c_int {
    return cls.demorecording as libc::c_int;
}
/*
=================
Demo_IsPlayingback

=================
*/
unsafe extern "C" fn Demo_IsPlayingback() -> libc::c_int {
    return cls.demoplayback;
}
/*
=================
Demo_IsTimeDemo

=================
*/
unsafe extern "C" fn Demo_IsTimeDemo() -> libc::c_int {
    return cls.timedemo as libc::c_int;
}
/*
=================
Demo_WriteBuffer

=================
*/
unsafe extern "C" fn Demo_WriteBuffer(mut size: libc::c_int,
                                      mut buffer: *mut byte) {
    CL_WriteDemoUserMessage(buffer, size as size_t);
}
/*
=================
NetworkApi implementation

=================
*/
/*
=================
NetAPI_InitNetworking

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_InitNetworking() {
    NET_Config(true_0);
    // allow remote
}
/*
=================
NetAPI_InitNetworking

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_Status(mut status: *mut net_status_t) {
    let mut connected: qboolean = false_0;
    let mut packet_loss: libc::c_int = 0 as libc::c_int;
    if cls.state as libc::c_uint >
           ca_disconnected as libc::c_int as libc::c_uint &&
           cls.state as libc::c_uint !=
               ca_cinematic as libc::c_int as libc::c_uint {
        connected = true_0
    }
    if cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint {
        packet_loss =
            if cls.packet_loss as libc::c_int >= 0 as libc::c_int {
                if (cls.packet_loss as libc::c_int) < 100 as libc::c_int {
                    cls.packet_loss as libc::c_int
                } else { 100 as libc::c_int }
            } else { 0 as libc::c_int }
    }
    (*status).connected = connected as libc::c_int;
    (*status).connection_time =
        if connected as libc::c_uint != 0 {
            (host.realtime) - cls.netchan.connect_time
        } else { 0.0f64 };
    (*status).latency =
        if connected as libc::c_uint != 0 {
            cl.frames[cl.parsecountmod as usize].latency
        } else { 0.0f64 };
    (*status).remote_address = cls.netchan.remote_address;
    (*status).packet_loss = packet_loss;
    (*status).local_address = net_local;
    (*status).rate = (*rate).value as libc::c_double;
}
/*
=================
NetAPI_SendRequest

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_SendRequest(mut context: libc::c_int,
                                            mut request: libc::c_int,
                                            mut flags: libc::c_int,
                                            mut timeout: libc::c_double,
                                            mut remote_address: *mut netadr_t,
                                            mut response:
                                                net_api_response_func_t) {
    let mut nr: *mut net_request_t =
        0 as *mut net_request_t; // IPX no longer support
    let mut req: string = [0; 256];
    let mut i: libc::c_int = 0;
    if response.is_none() {
        Con_DPrintf(b"^1Error:^7 Net_SendRequest: no callbcak specified for request with context %i!\n\x00"
                        as *const u8 as *const libc::c_char, context);
        return
    }
    if (*remote_address).type_0 as libc::c_uint >=
           NA_IPX as libc::c_int as libc::c_uint {
        return
    }
    // find a free request
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        nr =
            &mut *clgame.net_requests.as_mut_ptr().offset(i as isize) as
                *mut net_request_t;
        if (*nr).pfnFunc.is_none() { break ; }
        i += 1
    }
    if i == 64 as libc::c_int {
        let mut max_timeout: libc::c_double =
            0 as libc::c_int as libc::c_double;
        // no free requests? use oldest
        i = 0 as libc::c_int;
        nr = 0 as *mut net_request_t;
        while i < 64 as libc::c_int {
            if host.realtime - clgame.net_requests[i as usize].timesend >
                   max_timeout {
                max_timeout =
                    host.realtime - clgame.net_requests[i as usize].timesend;
                nr =
                    &mut *clgame.net_requests.as_mut_ptr().offset(i as isize)
                        as *mut net_request_t
            }
            i += 1
        }
    }
    // clear slot
    memset(nr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<net_request_t>() as libc::c_ulong);
    // create a new request
    (*nr).timesend = host.realtime;
    (*nr).timeout = (*nr).timesend + timeout;
    (*nr).pfnFunc = response;
    (*nr).resp.context = context;
    (*nr).resp.type_0 = request;
    (*nr).resp.remote_address = *remote_address;
    (*nr).flags = flags;
    if request == 0 as libc::c_int {
        let mut fullquery: [libc::c_char; 512] =
            *::std::mem::transmute::<&[u8; 512],
                                     &mut [libc::c_char; 512]>(b"1\xff0.0.0.0:0\x00\\gamedir\\\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        // holds the master request unitl the master acking
        if (*nr).resp.remote_address.port == 0 {
            (*nr).resp.remote_address.port =
                MSG_BigShort(27010 as libc::c_int as libc::c_ushort)
        }
        Q_strncpy(&mut *fullquery.as_mut_ptr().offset(22 as libc::c_int as
                                                          isize),
                  (*SI.GameInfo).gamefolder.as_mut_ptr(),
                  99999 as libc::c_int as size_t);
        NET_SendPacket(NS_CLIENT,
                       Q_strlen((*SI.GameInfo).gamefolder.as_mut_ptr()).wrapping_add(23
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong),
                       fullquery.as_mut_ptr() as *const libc::c_void,
                       (*nr).resp.remote_address);
        clgame.request_type = NET_REQUEST_CLIENT;
        clgame.master_request = nr
    } else {
        // make sure what port is specified
        // grab the list from the master server
        // local servers request
        Q_snprintf(req.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"netinfo %i %i %i\x00" as *const u8 as
                       *const libc::c_char, 49 as libc::c_int, context,
                   request);
        Netchan_OutOfBandPrint(NS_CLIENT as libc::c_int,
                               (*nr).resp.remote_address,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               req.as_mut_ptr());
    };
}
/*
=================
NetAPI_CancelRequest

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_CancelRequest(mut context: libc::c_int) {
    let mut nr: *mut net_request_t = 0 as *mut net_request_t;
    let mut i: libc::c_int = 0;
    // find a specified request
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        nr =
            &mut *clgame.net_requests.as_mut_ptr().offset(i as isize) as
                *mut net_request_t;
        if clgame.net_requests[i as usize].resp.context == context {
            if (*nr).pfnFunc.is_some() {
                (*nr).resp.error =
                    (*nr).resp.error | (1 as libc::c_int) << 0 as libc::c_int;
                (*nr).resp.ping = host.realtime - (*nr).timesend;
                (*nr).pfnFunc.expect("non-null function pointer")(&mut (*nr).resp);
            }
            if clgame.net_requests[i as usize].resp.type_0 == 0 as libc::c_int
                   &&
                   &mut *clgame.net_requests.as_mut_ptr().offset(i as isize)
                       as *mut net_request_t == clgame.master_request {
                if clgame.request_type as libc::c_uint ==
                       NET_REQUEST_CLIENT as libc::c_int as libc::c_uint {
                    clgame.request_type = NET_REQUEST_CANCEL
                }
                clgame.master_request = 0 as *mut net_request_t
            }
            memset(&mut *clgame.net_requests.as_mut_ptr().offset(i as isize)
                       as *mut net_request_t as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<net_request_t>() as libc::c_ulong);
            break ;
        } else { i += 1 }
    };
}
/*
=================
NetAPI_CancelAllRequests

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_CancelAllRequests() {
    let mut nr: *mut net_request_t = 0 as *mut net_request_t;
    let mut i: libc::c_int = 0;
    // tell the user about cancel
    i = 0 as libc::c_int; // not used
    while i < 64 as libc::c_int {
        nr =
            &mut *clgame.net_requests.as_mut_ptr().offset(i as isize) as
                *mut net_request_t;
        if !(*nr).pfnFunc.is_none() {
            (*nr).resp.error =
                (*nr).resp.error | (1 as libc::c_int) << 0 as libc::c_int;
            (*nr).resp.ping = host.realtime - (*nr).timesend;
            (*nr).pfnFunc.expect("non-null function pointer")(&mut (*nr).resp);
        }
        i += 1
    }
    memset(clgame.net_requests.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[net_request_t; 64]>() as libc::c_ulong);
    clgame.request_type = NET_REQUEST_CANCEL;
    clgame.master_request = 0 as *mut net_request_t;
}
/*
=================
NetAPI_AdrToString

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_AdrToString(mut a: *mut netadr_t)
 -> *const libc::c_char {
    return NET_AdrToString(*a);
}
/*
=================
NetAPI_CompareAdr

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_CompareAdr(mut a: *mut netadr_t,
                                           mut b: *mut netadr_t)
 -> libc::c_int {
    return NET_CompareAdr(*a, *b) as libc::c_int;
}
/*
=================
NetAPI_StringToAdr

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_StringToAdr(mut s: *mut libc::c_char,
                                            mut a: *mut netadr_t)
 -> libc::c_int {
    return NET_StringToAdr(s, a) as libc::c_int;
}
/*
=================
NetAPI_ValueForKey

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_ValueForKey(mut s: *const libc::c_char,
                                            mut key: *const libc::c_char)
 -> *const libc::c_char {
    return Info_ValueForKey(s, key);
}
/*
=================
NetAPI_RemoveKey

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_RemoveKey(mut s: *mut libc::c_char,
                                          mut key: *const libc::c_char) {
    Info_RemoveKey(s, key);
}
/*
=================
NetAPI_SetValueForKey

=================
*/
#[no_mangle]
pub unsafe extern "C" fn NetAPI_SetValueForKey(mut s: *mut libc::c_char,
                                               mut key: *const libc::c_char,
                                               mut value: *const libc::c_char,
                                               mut maxsize: libc::c_int) {
    if *key.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
        return
    }
    Info_SetValueForStarKey(s, key, value, maxsize);
}
/*
=================
IVoiceTweak implementation

TODO: implement
=================
*/
/*
=================
Voice_StartVoiceTweakMode

=================
*/
#[no_mangle]
pub unsafe extern "C" fn Voice_StartVoiceTweakMode() -> libc::c_int {
    return 0 as libc::c_int;
}
/*
=================
Voice_EndVoiceTweakMode

=================
*/
#[no_mangle]
pub unsafe extern "C" fn Voice_EndVoiceTweakMode() { }
/*
=================
Voice_SetControlFloat

=================
*/
#[no_mangle]
pub unsafe extern "C" fn Voice_SetControlFloat(mut iControl:
                                                   VoiceTweakControl,
                                               mut value: libc::c_float) {
}
/*
=================
Voice_GetControlFloat

=================
*/
#[no_mangle]
pub unsafe extern "C" fn Voice_GetControlFloat(mut iControl:
                                                   VoiceTweakControl)
 -> libc::c_float {
    return 1.0f32;
}
unsafe extern "C" fn VGui_ViewportPaintBackground(mut extents:
                                                      *mut libc::c_int) {
    // stub
}
// shared between client and server
#[no_mangle]
pub static mut gTriApi: triangleapi_t =
    triangleapi_t{version: 0,
                  RenderMode: None,
                  Begin: None,
                  End: None,
                  Color4f: None,
                  Color4ub: None,
                  TexCoord2f: None,
                  Vertex3fv: None,
                  Vertex3f: None,
                  Brightness: None,
                  CullFace: None,
                  SpriteTexture: None,
                  WorldToScreen: None,
                  Fog: None,
                  ScreenToWorld: None,
                  GetMatrix: None,
                  BoxInPVS: None,
                  LightAtPoint: None,
                  Color4fRendermode: None,
                  FogParams: None,};
static mut gEfxApi: efx_api_t =
    unsafe {
        {
            let mut init =
                efx_api_s{R_AllocParticle:
                              Some(R_AllocParticle as
                                       unsafe extern "C" fn(_:
                                                                Option<unsafe extern "C" fn(_:
                                                                                                *mut particle_s,
                                                                                            _:
                                                                                                libc::c_float)
                                                                           ->
                                                                               ()>)
                                           -> *mut particle_s),
                          R_BlobExplosion:
                              Some(R_BlobExplosion as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_Blood:
                              Some(R_Blood as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_BloodSprite:
                              Some(R_BloodSprite as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_BloodStream:
                              Some(R_BloodStream as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_BreakModel:
                              Some(R_BreakModel as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_char)
                                           -> ()),
                          R_Bubbles:
                              Some(R_Bubbles as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_BubbleTrail:
                              Some(R_BubbleTrail as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_BulletImpactParticles:
                              Some(R_BulletImpactParticles as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_EntityParticles:
                              Some(R_EntityParticles as
                                       unsafe extern "C" fn(_:
                                                                *mut cl_entity_t)
                                           -> ()),
                          R_Explosion:
                              Some(R_Explosion as
                                       unsafe extern "C" fn(_: *mut vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_int)
                                           -> ()),
                          R_FizzEffect:
                              Some(R_FizzEffect as
                                       unsafe extern "C" fn(_:
                                                                *mut cl_entity_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_FireField:
                              Some(R_FireField as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_FlickerParticles:
                              Some(R_FlickerParticles as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_FunnelSprite:
                              Some(R_FunnelSprite as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_Implosion:
                              Some(R_Implosion as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_LargeFunnel:
                              Some(R_LargeFunnel as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int)
                                           -> ()),
                          R_LavaSplash:
                              Some(R_LavaSplash as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_MultiGunshot:
                              Some(R_MultiGunshot as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _:
                                                                *mut libc::c_int)
                                           -> ()),
                          R_MuzzleFlash:
                              Some(R_MuzzleFlash as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int)
                                           -> ()),
                          R_ParticleBox:
                              Some(R_ParticleBox as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: byte, _: byte,
                                                            _: byte,
                                                            _: libc::c_float)
                                           -> ()),
                          R_ParticleBurst:
                              Some(R_ParticleBurst as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_ParticleExplosion:
                              Some(R_ParticleExplosion as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_ParticleExplosion2:
                              Some(R_ParticleExplosion2 as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_ParticleLine:
                              Some(R_ParticleLine as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: byte, _: byte,
                                                            _: byte,
                                                            _: libc::c_float)
                                           -> ()),
                          R_PlayerSprites:
                              Some(R_PlayerSprites as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_Projectile:
                              Some(R_Projectile as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _:
                                                                Option<unsafe extern "C" fn(_:
                                                                                                *mut tempent_s,
                                                                                            _:
                                                                                                *mut pmtrace_s)
                                                                           ->
                                                                               ()>)
                                           -> ()),
                          R_RicochetSound:
                              Some(R_RicochetSound as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_RicochetSprite:
                              Some(R_RicochetSprite as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *mut model_t,
                                                            _: libc::c_float,
                                                            _: libc::c_float)
                                           -> ()),
                          R_RocketFlare:
                              Some(R_RocketFlare as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_RocketTrail:
                              Some(R_RocketTrail as
                                       unsafe extern "C" fn(_: *mut vec_t,
                                                            _: *mut vec_t,
                                                            _: libc::c_int)
                                           -> ()),
                          R_RunParticleEffect:
                              Some(R_RunParticleEffect as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_ShowLine:
                              Some(R_ShowLine as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t)
                                           -> ()),
                          R_SparkEffect:
                              Some(R_SparkEffect as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_SparkShower:
                              Some(R_SparkShower as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_SparkStreaks:
                              Some(R_SparkStreaks as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_Spray:
                              Some(R_Spray as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_Sprite_Explode:
                              Some(R_Sprite_Explode as
                                       unsafe extern "C" fn(_: *mut tempent_s,
                                                            _: libc::c_float,
                                                            _: libc::c_int)
                                           -> ()),
                          R_Sprite_Smoke:
                              Some(R_Sprite_Smoke as
                                       unsafe extern "C" fn(_: *mut tempent_s,
                                                            _: libc::c_float)
                                           -> ()),
                          R_Sprite_Spray:
                              Some(R_Sprite_Spray as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_Sprite_Trail:
                              Some(R_Sprite_Trail as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: *mut vec_t,
                                                            _: *mut vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),
                          R_Sprite_WallPuff:
                              Some(R_Sprite_WallPuff as
                                       unsafe extern "C" fn(_: *mut tempent_s,
                                                            _: libc::c_float)
                                           -> ()),
                          R_StreakSplash:
                              Some(R_StreakSplash as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_TracerEffect:
                              Some(R_TracerEffect as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t)
                                           -> ()),
                          R_UserTracerParticle:
                              Some(R_UserTracerParticle as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _:
                                                                *mut libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: byte,
                                                            _:
                                                                Option<unsafe extern "C" fn(_:
                                                                                                *mut particle_s)
                                                                           ->
                                                                               ()>)
                                           -> ()),
                          R_TracerParticles:
                              Some(R_TracerParticles as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _:
                                                                *mut libc::c_float,
                                                            _: libc::c_float)
                                           -> *mut particle_s),
                          R_TeleportSplash:
                              Some(R_TeleportSplash as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> ()),
                          R_TempSphereModel:
                              Some(R_TempSphereModel as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> ()),
                          R_TempModel:
                              Some(R_TempModel as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *const vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int)
                                           -> *mut tempent_s),
                          R_DefaultSprite:
                              Some(R_DefaultSprite as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> *mut tempent_s),
                          R_TempSprite:
                              Some(R_TempSprite as
                                       unsafe extern "C" fn(_: *mut vec_t,
                                                            _: *const vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_int)
                                           -> *mut tempent_s),
                          Draw_DecalIndex:
                              Some(CL_DecalIndex as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> libc::c_int),
                          Draw_DecalIndexFromName:
                              Some(CL_DecalIndexFromName as
                                       unsafe extern "C" fn(_:
                                                                *const libc::c_char)
                                           -> libc::c_int),
                          R_DecalShoot:
                              Some(CL_DecalShoot as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _:
                                                                *mut libc::c_float,
                                                            _: libc::c_int)
                                           -> ()),
                          R_AttachTentToPlayer:
                              Some(R_AttachTentToPlayer as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_float)
                                           -> ()),
                          R_KillAttachedTents:
                              Some(R_KillAttachedTents as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> ()),
                          R_BeamCirclePoints:
                              Some(R_BeamCirclePoints as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: *mut vec_t,
                                                            _: *mut vec_t,
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
                                           -> *mut beam_s),
                          R_BeamEntPoint:
                              Some(R_BeamEntPoint as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: *mut vec_t,
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
                                           -> *mut beam_s),
                          R_BeamEnts:
                              Some(R_BeamEnts as
                                       unsafe extern "C" fn(_: libc::c_int,
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
                                           -> *mut beam_s),
                          R_BeamFollow:
                              Some(R_BeamFollow as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float)
                                           -> *mut beam_s),
                          R_BeamKill:
                              Some(R_BeamKill as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> ()),
                          R_BeamLightning:
                              Some(R_BeamLightning as
                                       unsafe extern "C" fn(_: *mut vec_t,
                                                            _: *mut vec_t,
                                                            _: libc::c_int,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float,
                                                            _: libc::c_float)
                                           -> *mut beam_s),
                          R_BeamPoints:
                              Some(R_BeamPoints as
                                       unsafe extern "C" fn(_: *mut vec_t,
                                                            _: *mut vec_t,
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
                                           -> *mut beam_s),
                          R_BeamRing:
                              Some(R_BeamRing as
                                       unsafe extern "C" fn(_: libc::c_int,
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
                                           -> *mut beam_s),
                          CL_AllocDlight:
                              Some(CL_AllocDlight as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> *mut dlight_s),
                          CL_AllocElight:
                              Some(CL_AllocElight as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> *mut dlight_s),
                          CL_TempEntAlloc:
                              Some(CL_TempEntAlloc as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *mut model_t)
                                           -> *mut tempent_s),
                          CL_TempEntAllocNoModel:
                              Some(CL_TempEntAllocNoModel as
                                       unsafe extern "C" fn(_: *const vec_t)
                                           -> *mut tempent_s),
                          CL_TempEntAllocHigh:
                              Some(CL_TempEntAllocHigh as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *mut model_t)
                                           -> *mut tempent_s),
                          CL_TentEntAllocCustom:
                              Some(CL_TempEntAllocCustom as
                                       unsafe extern "C" fn(_: *const vec_t,
                                                            _: *mut model_t,
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
                                           -> *mut tempent_s),
                          R_GetPackedColor:
                              Some(R_GetPackedColor as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_short,
                                                            _: libc::c_short)
                                           -> ()),
                          R_LookupColor:
                              Some(R_LookupColor as
                                       unsafe extern "C" fn(_: byte, _: byte,
                                                            _: byte)
                                           -> libc::c_short),
                          R_DecalRemoveAll:
                              Some(CL_DecalRemoveAll as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> ()),
                          R_FireCustomDecal:
                              Some(CL_FireCustomDecal as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _:
                                                                *mut libc::c_float,
                                                            _: libc::c_int,
                                                            _: libc::c_float)
                                           -> ()),};
            init
        }
    };
static mut gEventApi: event_api_t =
    unsafe {
        {
            let mut init =
                event_api_s{version: 1 as libc::c_int,
                            EV_PlaySound:
                                Some(pfnPlaySound as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: libc::c_int,
                                                              _:
                                                                  *const libc::c_char,
                                                              _:
                                                                  libc::c_float,
                                                              _:
                                                                  libc::c_float,
                                                              _: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()),
                            EV_StopSound:
                                Some(S_StopSound as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int,
                                                              _:
                                                                  *const libc::c_char)
                                             -> ()),
                            EV_FindModelIndex:
                                Some(CL_FindModelIndex as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char)
                                             -> libc::c_int),
                            EV_IsLocal:
                                Some(pfnIsLocal as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> libc::c_int),
                            EV_LocalPlayerDucking:
                                Some(pfnLocalPlayerDucking as
                                         unsafe extern "C" fn()
                                             -> libc::c_int),
                            EV_LocalPlayerViewheight:
                                Some(pfnLocalPlayerViewheight as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_float)
                                             -> ()),
                            EV_LocalPlayerBounds:
                                Some(pfnLocalPlayerBounds as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> ()),
                            EV_IndexFromTrace:
                                Some(pfnIndexFromTrace as
                                         unsafe extern "C" fn(_:
                                                                  *mut pmtrace_s)
                                             -> libc::c_int),
                            EV_GetPhysent:
                                Some(pfnGetPhysent as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> *mut physent_t),
                            EV_SetUpPlayerPrediction:
                                Some(CL_SetUpPlayerPrediction as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()),
                            EV_PushPMStates:
                                Some(CL_PushPMStates as
                                         unsafe extern "C" fn() -> ()),
                            EV_PopPMStates:
                                Some(CL_PopPMStates as
                                         unsafe extern "C" fn() -> ()),
                            EV_SetSolidPlayers:
                                Some(CL_SetSolidPlayers as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> ()),
                            EV_SetTraceHull:
                                Some(CL_SetTraceHull as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> ()),
                            EV_PlayerTrace:
                                Some(CL_PlayerTrace as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: libc::c_int,
                                                              _: libc::c_int,
                                                              _:
                                                                  *mut pmtrace_t)
                                             -> ()),
                            EV_WeaponAnimation:
                                Some(CL_WeaponAnim as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()),
                            EV_PrecacheEvent:
                                Some(pfnPrecacheEvent as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *const libc::c_char)
                                             -> word),
                            EV_PlaybackEvent:
                                Some(CL_PlaybackEvent as
                                         unsafe extern "C" fn(_: libc::c_int,
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
                                                              _: libc::c_int,
                                                              _: libc::c_int,
                                                              _: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()),
                            EV_TraceTexture:
                                Some(pfnTraceTexture as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> *const libc::c_char),
                            EV_StopAllSounds:
                                Some(pfnStopAllSounds as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()),
                            EV_KillEvents:
                                Some(pfnKillEvents as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *const libc::c_char)
                                             -> ()),
                            EV_PlayerTraceExt:
                                Some(CL_PlayerTraceExt as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: libc::c_int,
                                                              _:
                                                                  Option<unsafe extern "C" fn(_:
                                                                                                  *mut physent_t)
                                                                             ->
                                                                                 libc::c_int>,
                                                              _:
                                                                  *mut pmtrace_t)
                                             -> ()),
                            EV_SoundForIndex:
                                Some(CL_SoundFromIndex as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> *const libc::c_char),
                            EV_TraceSurface:
                                Some(pfnTraceSurface as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> *mut msurface_s),
                            EV_GetMovevars:
                                Some(pfnGetMoveVars as
                                         unsafe extern "C" fn()
                                             -> *mut movevars_t),
                            EV_VisTraceLine:
                                Some(CL_VisTraceLine as
                                         unsafe extern "C" fn(_: *mut vec_t,
                                                              _: *mut vec_t,
                                                              _: libc::c_int)
                                             -> *mut pmtrace_t),
                            EV_GetVisent:
                                Some(pfnGetVisent as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> *mut physent_t),
                            EV_TestLine:
                                Some(CL_TestLine as
                                         unsafe extern "C" fn(_: *const vec_t,
                                                              _: *const vec_t,
                                                              _: libc::c_int)
                                             -> libc::c_int),
                            EV_PushTraceBounds:
                                Some(CL_PushTraceBounds as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *const libc::c_float,
                                                              _:
                                                                  *const libc::c_float)
                                             -> ()),
                            EV_PopTraceBounds:
                                Some(CL_PopTraceBounds as
                                         unsafe extern "C" fn() -> ()),};
            init
        }
    };
static mut gDemoApi: demo_api_t =
    unsafe {
        {
            let mut init =
                demo_api_s{IsRecording:
                               Some(Demo_IsRecording as
                                        unsafe extern "C" fn()
                                            -> libc::c_int),
                           IsPlayingback:
                               Some(Demo_IsPlayingback as
                                        unsafe extern "C" fn()
                                            -> libc::c_int),
                           IsTimeDemo:
                               Some(Demo_IsTimeDemo as
                                        unsafe extern "C" fn()
                                            -> libc::c_int),
                           WriteBuffer:
                               Some(Demo_WriteBuffer as
                                        unsafe extern "C" fn(_: libc::c_int,
                                                             _: *mut byte)
                                            -> ()),};
            init
        }
    };
static mut gNetApi: net_api_t =
    unsafe {
        {
            let mut init =
                net_api_s{InitNetworking:
                              Some(NetAPI_InitNetworking as
                                       unsafe extern "C" fn() -> ()),
                          Status:
                              Some(NetAPI_Status as
                                       unsafe extern "C" fn(_:
                                                                *mut net_status_t)
                                           -> ()),
                          SendRequest:
                              Some(NetAPI_SendRequest as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_int,
                                                            _: libc::c_double,
                                                            _: *mut netadr_t,
                                                            _:
                                                                net_api_response_func_t)
                                           -> ()),
                          CancelRequest:
                              Some(NetAPI_CancelRequest as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> ()),
                          CancelAllRequests:
                              Some(NetAPI_CancelAllRequests as
                                       unsafe extern "C" fn() -> ()),
                          AdrToString:
                              Some(NetAPI_AdrToString as
                                       unsafe extern "C" fn(_: *mut netadr_t)
                                           -> *const libc::c_char),
                          CompareAdr:
                              Some(NetAPI_CompareAdr as
                                       unsafe extern "C" fn(_: *mut netadr_t,
                                                            _: *mut netadr_t)
                                           -> libc::c_int),
                          StringToAdr:
                              Some(NetAPI_StringToAdr as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_char,
                                                            _: *mut netadr_t)
                                           -> libc::c_int),
                          ValueForKey:
                              Some(NetAPI_ValueForKey as
                                       unsafe extern "C" fn(_:
                                                                *const libc::c_char,
                                                            _:
                                                                *const libc::c_char)
                                           -> *const libc::c_char),
                          RemoveKey:
                              Some(NetAPI_RemoveKey as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_char,
                                                            _:
                                                                *const libc::c_char)
                                           -> ()),
                          SetValueForKey:
                              Some(NetAPI_SetValueForKey as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_char,
                                                            _:
                                                                *const libc::c_char,
                                                            _:
                                                                *const libc::c_char,
                                                            _: libc::c_int)
                                           -> ()),};
            init
        }
    };
static mut gVoiceApi: IVoiceTweak =
    unsafe {
        {
            let mut init =
                IVoiceTweak_s{StartVoiceTweakMode:
                                  Some(Voice_StartVoiceTweakMode as
                                           unsafe extern "C" fn()
                                               -> libc::c_int),
                              EndVoiceTweakMode:
                                  Some(Voice_EndVoiceTweakMode as
                                           unsafe extern "C" fn() -> ()),
                              SetControlFloat:
                                  Some(Voice_SetControlFloat as
                                           unsafe extern "C" fn(_:
                                                                    VoiceTweakControl,
                                                                _:
                                                                    libc::c_float)
                                               -> ()),
                              GetControlFloat:
                                  Some(Voice_GetControlFloat as
                                           unsafe extern "C" fn(_:
                                                                    VoiceTweakControl)
                                               -> libc::c_float),};
            init
        }
    };
// engine callbacks
static mut gEngfuncs: cl_enginefunc_t =
    unsafe {
        {
            let mut init =
                cl_enginefuncs_s{pfnSPR_Load:
                                     Some(pfnSPR_Load as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> HSPRITE),
                                 pfnSPR_Frames:
                                     Some(pfnSPR_Frames as
                                              unsafe extern "C" fn(_: HSPRITE)
                                                  -> libc::c_int),
                                 pfnSPR_Height:
                                     Some(pfnSPR_Height as
                                              unsafe extern "C" fn(_: HSPRITE,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnSPR_Width:
                                     Some(pfnSPR_Width as
                                              unsafe extern "C" fn(_: HSPRITE,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnSPR_Set:
                                     Some(pfnSPR_Set as
                                              unsafe extern "C" fn(_: HSPRITE,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnSPR_Draw:
                                     Some(pfnSPR_Draw as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnSPR_DrawHoles:
                                     Some(pfnSPR_DrawHoles as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnSPR_DrawAdditive:
                                     Some(pfnSPR_DrawAdditive as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnSPR_EnableScissor:
                                     Some(SPR_EnableScissor as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnSPR_DisableScissor:
                                     Some(SPR_DisableScissor as
                                              unsafe extern "C" fn() -> ()),
                                 pfnSPR_GetList:
                                     Some(pfnSPR_GetList as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> *mut client_sprite_t),
                                 pfnFillRGBA:
                                     Some(CL_FillRGBA as
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
                                 pfnGetScreenInfo:
                                     Some(CL_GetScreenInfo as
                                              unsafe extern "C" fn(_:
                                                                       *mut SCREENINFO)
                                                  -> libc::c_int),
                                 pfnSetCrosshair:
                                     Some(pfnSetCrosshair as
                                              unsafe extern "C" fn(_: HSPRITE,
                                                                   _: wrect_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnRegisterVariable:
                                     Some(pfnCvar_RegisterClientVariable as
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
                                 pfnAddCommand:
                                     Some(Cmd_AddClientCommand as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       xcommand_t)
                                                  -> libc::c_int),
                                 pfnHookUserMsg:
                                     Some(pfnHookUserMsg as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       pfnUserMsgHook)
                                                  -> libc::c_int),
                                 pfnServerCmd:
                                     Some(pfnServerCmd as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> libc::c_int),
                                 pfnClientCmd:
                                     Some(pfnClientCmd as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> libc::c_int),
                                 pfnGetPlayerInfo:
                                     Some(pfnGetPlayerInfo as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut hud_player_info_t)
                                                  -> ()),
                                 pfnPlaySoundByName:
                                     Some(pfnPlaySoundByName as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnPlaySoundByIndex:
                                     Some(pfnPlaySoundByIndex as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnAngleVectors:
                                     Some(AngleVectors as
                                              unsafe extern "C" fn(_:
                                                                       *const vec_t,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       *mut vec_t)
                                                  -> ()),
                                 pfnTextMessageGet:
                                     Some(CL_TextMessageGet as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  ->
                                                      *mut client_textmessage_t),
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
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnDrawConsoleString:
                                     Some(pfnDrawConsoleString as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut libc::c_char)
                                                  -> libc::c_int),
                                 pfnDrawSetTextColor:
                                     Some(pfnDrawSetTextColor as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnDrawConsoleStringLen:
                                     Some(pfnDrawConsoleStringLen as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> ()),
                                 pfnConsolePrint:
                                     Some(pfnConsolePrint as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnCenterPrint:
                                     Some(pfnCenterPrint as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 GetWindowCenterX:
                                     Some(pfnGetWindowCenterX as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 GetWindowCenterY:
                                     Some(pfnGetWindowCenterY as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 GetViewAngles:
                                     Some(pfnGetViewAngles as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_float)
                                                  -> ()),
                                 SetViewAngles:
                                     Some(pfnSetViewAngles as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_float)
                                                  -> ()),
                                 GetMaxClients:
                                     Some(CL_GetMaxClients as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 Cvar_SetValue:
                                     Some(Cvar_SetValue as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 Cmd_Argc:
                                     Some(Cmd_Argc as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 Cmd_Argv:
                                     Some(Cmd_Argv as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
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
                                     Some(Con_NPrintf as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 Con_NXPrintf:
                                     Some(Con_NXPrintf as
                                              unsafe extern "C" fn(_:
                                                                       *mut con_nprint_t,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 PhysInfo_ValueForKey:
                                     Some(pfnPhysInfo_ValueForKey as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *const libc::c_char),
                                 ServerInfo_ValueForKey:
                                     Some(pfnServerInfo_ValueForKey as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *const libc::c_char),
                                 GetClientMaxspeed:
                                     Some(pfnGetClientMaxspeed as
                                              unsafe extern "C" fn()
                                                  -> libc::c_float),
                                 CheckParm:
                                     Some(COM_CheckParm as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       *mut *mut libc::c_char)
                                                  -> libc::c_int),
                                 Key_Event:
                                     Some(Key_Event as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 GetMousePosition:
                                     Some(Platform_GetMousePos as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> ()),
                                 IsNoClipping:
                                     Some(pfnIsNoClipping as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 GetLocalPlayer:
                                     Some(CL_GetLocalPlayer as
                                              unsafe extern "C" fn()
                                                  -> *mut cl_entity_t),
                                 GetViewModel:
                                     Some(CL_GetViewModel as
                                              unsafe extern "C" fn()
                                                  -> *mut cl_entity_t),
                                 GetEntityByIndex:
                                     Some(CL_GetEntityByIndex as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> *mut cl_entity_s),
                                 GetClientTime:
                                     Some(pfnGetClientTime as
                                              unsafe extern "C" fn()
                                                  -> libc::c_float),
                                 V_CalcShake:
                                     Some(pfnCalcShake as
                                              unsafe extern "C" fn() -> ()),
                                 V_ApplyShake:
                                     Some(pfnApplyShake as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_float,
                                                                   _:
                                                                       *mut libc::c_float,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 PM_PointContents:
                                     Some(pfnPointContents as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_float,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> libc::c_int),
                                 PM_WaterEntity:
                                     Some(CL_WaterEntity as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_float)
                                                  -> libc::c_int),
                                 PM_TraceLine:
                                     Some(pfnTraceLine as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_float,
                                                                   _:
                                                                       *mut libc::c_float,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> *mut pmtrace_t),
                                 CL_LoadModel:
                                     Some(CL_LoadModel as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> *mut model_t),
                                 CL_CreateVisibleEntity:
                                     Some(CL_AddEntity as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut cl_entity_t)
                                                  -> libc::c_int),
                                 GetSpritePointer:
                                     Some(CL_GetSpritePointer as
                                              unsafe extern "C" fn(_: HSPRITE)
                                                  -> *const model_t),
                                 pfnPlaySoundByNameAtLocation:
                                     Some(pfnPlaySoundByNameAtLocation as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       *mut libc::c_float)
                                                  -> ()),
                                 pfnPrecacheEvent:
                                     Some(pfnPrecacheEvent as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> word),
                                 pfnPlaybackEvent:
                                     Some(CL_PlaybackEvent as
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
                                 pfnWeaponAnim:
                                     Some(CL_WeaponAnim as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
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
                                 pfnHookEvent:
                                     Some(pfnHookEvent as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       pfnEventHook)
                                                  -> ()),
                                 Con_IsVisible:
                                     Some(Con_Visible as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnGetGameDirectory:
                                     Some(pfnGetGameDirectory as
                                              unsafe extern "C" fn()
                                                  -> *const libc::c_char),
                                 pfnGetCvarPointer:
                                     Some(pfnCVarGetPointer as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *mut cvar_t),
                                 Key_LookupBinding:
                                     Some(Key_LookupBinding as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *const libc::c_char),
                                 pfnGetLevelName:
                                     Some(pfnGetLevelName as
                                              unsafe extern "C" fn()
                                                  -> *const libc::c_char),
                                 pfnGetScreenFade:
                                     Some(pfnGetScreenFade as
                                              unsafe extern "C" fn(_:
                                                                       *mut screenfade_s)
                                                  -> ()),
                                 pfnSetScreenFade:
                                     Some(pfnSetScreenFade as
                                              unsafe extern "C" fn(_:
                                                                       *mut screenfade_s)
                                                  -> ()),
                                 VGui_GetPanel:
                                     Some(VGui_GetPanel as
                                              unsafe extern "C" fn()
                                                  -> *mut libc::c_void),
                                 VGui_ViewportPaintBackground:
                                     Some(VGui_ViewportPaintBackground as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_int)
                                                  -> ()),
                                 COM_LoadFile:
                                     Some(COM_LoadFile as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int,
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
                                 pTriAPI:
                                     &gTriApi as *const triangleapi_t as
                                         *mut triangleapi_t,
                                 pEfxAPI:
                                     &gEfxApi as *const efx_api_t as
                                         *mut efx_api_t,
                                 pEventAPI:
                                     &gEventApi as *const event_api_t as
                                         *mut event_api_t,
                                 pDemoAPI:
                                     &gDemoApi as *const demo_api_t as
                                         *mut demo_api_t,
                                 pNetAPI:
                                     &gNetApi as *const net_api_t as
                                         *mut net_api_t,
                                 pVoiceTweak:
                                     &gVoiceApi as *const IVoiceTweak as
                                         *mut IVoiceTweak,
                                 IsSpectateOnly:
                                     Some(pfnIsSpectateOnly as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 LoadMapSprite:
                                     Some(pfnLoadMapSprite as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *mut model_t),
                                 COM_AddAppDirectoryToSearchPath:
                                     Some(COM_AddAppDirectoryToSearchPath as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 COM_ExpandFilename:
                                     Some(COM_ExpandFilename as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 PlayerInfo_ValueForKey:
                                     Some(PlayerInfo_ValueForKey as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> *const libc::c_char),
                                 PlayerInfo_SetValueForKey:
                                     Some(PlayerInfo_SetValueForKey as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 GetPlayerUniqueID:
                                     Some(pfnGetPlayerUniqueID as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut libc::c_char)
                                                  -> qboolean),
                                 GetTrackerIDForPlayer:
                                     Some(pfnGetTrackerIDForPlayer as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 GetPlayerForTrackerID:
                                     Some(pfnGetPlayerForTrackerID as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnServerCmdUnreliable:
                                     Some(pfnServerCmdUnreliable as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char)
                                                  -> libc::c_int),
                                 pfnGetMousePos:
                                     Some(pfnGetMousePos as
                                              unsafe extern "C" fn(_:
                                                                       *mut tagPOINT)
                                                  -> ()),
                                 pfnSetMousePos:
                                     Some(Platform_SetMousePos as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnSetMouseEnable:
                                     Some(pfnSetMouseEnable as
                                              unsafe extern "C" fn(_:
                                                                       qboolean)
                                                  -> ()),
                                 pfnGetFirstCvarPtr:
                                     Some(Cvar_GetList as
                                              unsafe extern "C" fn()
                                                  -> *mut cvar_t),
                                 pfnGetFirstCmdFunctionHandle:
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn()
                                                                        ->
                                                                            *mut libc::c_void>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                   ->
                                                                                                                                       *mut cmd_s>,
                                                                                                                        *mut libc::c_void>(Some(Cmd_GetFirstFunctionHandle
                                                                                                                                                    as
                                                                                                                                                    unsafe extern "C" fn()
                                                                                                                                                        ->
                                                                                                                                                            *mut cmd_s))),
                                 pfnGetNextCmdFunctionHandle:
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *mut libc::c_void)
                                                                        ->
                                                                            *mut libc::c_void>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                        *mut cmd_s)
                                                                                                                                   ->
                                                                                                                                       *mut cmd_s>,
                                                                                                                        *mut libc::c_void>(Some(Cmd_GetNextFunctionHandle
                                                                                                                                                    as
                                                                                                                                                    unsafe extern "C" fn(_:
                                                                                                                                                                             *mut cmd_s)
                                                                                                                                                        ->
                                                                                                                                                            *mut cmd_s))),
                                 pfnGetCmdFunctionName:
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *mut libc::c_void)
                                                                        ->
                                                                            *const libc::c_char>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                          *mut cmd_s)
                                                                                                                                     ->
                                                                                                                                         *const libc::c_char>,
                                                                                                                          *mut libc::c_void>(Some(Cmd_GetName
                                                                                                                                                      as
                                                                                                                                                      unsafe extern "C" fn(_:
                                                                                                                                                                               *mut cmd_s)
                                                                                                                                                          ->
                                                                                                                                                              *const libc::c_char))),
                                 pfnGetClientOldTime:
                                     Some(pfnGetClientOldTime as
                                              unsafe extern "C" fn()
                                                  -> libc::c_float),
                                 pfnGetGravity:
                                     Some(pfnGetGravity as
                                              unsafe extern "C" fn()
                                                  -> libc::c_float),
                                 pfnGetModelByIndex:
                                     Some(Mod_Handle as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> *mut model_t),
                                 pfnSetFilterMode:
                                     Some(pfnEnableTexSort as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnSetFilterColor:
                                     Some(pfnSetLightmapColor as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnSetFilterBrightness:
                                     Some(pfnSetLightmapScale as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnSequenceGet:
                                     Some(pfnSequenceGet as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> *mut libc::c_void),
                                 pfnSPR_DrawGeneric:
                                     Some(pfnSPR_DrawGeneric as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnSequencePickSentence:
                                     Some(pfnSequencePickSentence as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> *mut libc::c_void),
                                 pfnDrawString:
                                     Some(pfnDrawString as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnDrawStringReverse:
                                     Some(pfnDrawStringReverse as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 LocalPlayerInfo_ValueForKey:
                                     Some(LocalPlayerInfo_ValueForKey as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *const libc::c_char),
                                 pfnVGUI2DrawCharacter:
                                     Some(pfnVGUI2DrawCharacter as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_uint)
                                                  -> libc::c_int),
                                 pfnVGUI2DrawCharacterAdditive:
                                     Some(pfnVGUI2DrawCharacterAdditive as
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
                                                                       libc::c_uint)
                                                  -> libc::c_int),
                                 pfnGetApproxWavePlayLen:
                                     Some(Sound_GetApproxWavePlayLen as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> uint),
                                 GetCareerGameUI:
                                     Some(GetCareerGameInterface as
                                              unsafe extern "C" fn()
                                                  -> *mut libc::c_void),
                                 Cvar_Set:
                                     Some(Cvar_Set as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnIsPlayingCareerMatch:
                                     Some(pfnIsCareerMatch as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnPlaySoundVoiceByName:
                                     Some(pfnPlaySoundVoiceByName as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnPrimeMusicStream:
                                     Some(pfnMP3_InitStream as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnSys_FloatTime:
                                     Some(Sys_DoubleTime as
                                              unsafe extern "C" fn()
                                                  -> libc::c_double),
                                 pfnProcessTutorMessageDecayBuffer:
                                     Some(pfnProcessTutorMessageDecayBuffer as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnConstructTutorMessageDecayBuffer:
                                     Some(pfnConstructTutorMessageDecayBuffer
                                              as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnResetTutorMessageDecayData:
                                     Some(pfnResetTutorMessageDecayData as
                                              unsafe extern "C" fn() -> ()),
                                 pfnPlaySoundByNameAtPitch:
                                     Some(pfnPlaySoundByNameAtPitch as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnFillRGBABlend:
                                     Some(CL_FillRGBABlend as
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
                                 pfnGetAppID:
                                     Some(pfnGetAppID as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnGetAliases:
                                     Some(Cmd_AliasGetList as
                                              unsafe extern "C" fn()
                                                  -> *mut cmdalias_s),
                                 pfnVguiWrap2_GetMouseDelta:
                                     Some(pfnVguiWrap2_GetMouseDelta as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> ()),
                                 pfnFilteredClientCmd:
                                     Some(pfnFilteredClientCmd as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> libc::c_int),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn CL_UnloadProgs() {
    if clgame.hInstance.is_null() { return }
    CL_FreeEdicts();
    CL_FreeTempEnts();
    CL_FreeViewBeams();
    CL_FreeParticles();
    CL_ClearAllRemaps();
    Mod_ClearUserData();
    // NOTE: HLFX 0.5 has strange bug: hanging on exit if no map was loaded
    if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                  b"hlfx\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 ||
           (*SI.GameInfo).version != 0.5f32 {
        clgame.dllFuncs.pfnShutdown.expect("non-null function pointer")(); // single export
    }
    Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    Cvar_FullSet(b"host_clientloaded\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    COM_FreeLibrary(clgame.hInstance);
    VGui_Shutdown();
    _Mem_FreePool(&mut cls.mempool,
                  b"../engine/client/cl_game.c\x00" as *const u8 as
                      *const libc::c_char, 3927 as libc::c_int);
    _Mem_FreePool(&mut clgame.mempool,
                  b"../engine/client/cl_game.c\x00" as *const u8 as
                      *const libc::c_char, 3928 as libc::c_int);
    memset(&mut clgame as *mut clgame_static_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<clgame_static_t>() as libc::c_ulong);
    Cvar_Unlink((1 as libc::c_int) << 4 as libc::c_int);
    Cmd_Unlink(((1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CL_LoadProgs(mut name: *const libc::c_char)
 -> qboolean {
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
    let mut func: *const dllfunc_t = 0 as *const dllfunc_t;
    let mut GetClientAPI: CL_EXPORT_FUNCS = None;
    let mut critical_exports: qboolean = true_0;
    if !clgame.hInstance.is_null() { CL_UnloadProgs(); }
    // initialize PlayerMove
    clgame.pmove = &mut gpMove;
    cls.mempool =
        _Mem_AllocPool(b"Client Static Pool\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/client/cl_game.c\x00" as *const u8 as
                           *const libc::c_char, 3947 as libc::c_int);
    clgame.mempool =
        _Mem_AllocPool(b"Client Edicts Zone\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/client/cl_game.c\x00" as *const u8 as
                           *const libc::c_char, 3948 as libc::c_int);
    clgame.entities = 0 as *mut cl_entity_t;
    // NOTE: important stuff!
	// vgui must startup BEFORE loading client.dll to avoid get error ERROR_NOACESS
	// during LoadLibrary
    VGui_Startup(name, (*gameui.globals).scrWidth,
                 (*gameui.globals).scrHeight);
    // a1ba: we need to check if client.dll has direct dependency on SDL2
	// and if so, disable relative mouse mode
    // this doesn't mean other platforms uses SDL2 in any case
	// it just helps input code to stay platform-independent
    clgame.client_dll_uses_sdl = false_0;
    clgame.hInstance = COM_LoadLibrary(name, false_0 as libc::c_int, false_0);
    if clgame.hInstance.is_null() { return false_0 }
    // clear exports
    func = cdll_exports.as_mut_ptr();
    while !func.is_null() && !(*func).name.is_null() {
        *(*func).func = 0 as *mut libc::c_void;
        func = func.offset(1)
    }
    // trying to get single export
    GetClientAPI =
        ::std::mem::transmute::<*mut libc::c_void,
                                CL_EXPORT_FUNCS>(COM_GetProcAddress(clgame.hInstance,
                                                                    b"GetClientAPI\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char));
    if GetClientAPI.is_some() {
        Con_Reportf(b"CL_LoadProgs: found single callback export\n\x00" as
                        *const u8 as *const libc::c_char);
        // trying to fill interface now
        GetClientAPI.expect("non-null function pointer")(&mut clgame.dllFuncs
                                                             as
                                                             *mut cldll_func_t
                                                             as
                                                             *mut libc::c_void);
        // check critical functions again
        func = cdll_exports.as_mut_ptr();
        while !func.is_null() && !(*func).name.is_null() {
            if (*func).func.is_null() {
                break ;
                // BAH critical function was missed
            }
            func = func.offset(1)
        }
        // because all the exports are loaded through function 'F"
        if func.is_null() || (*func).name.is_null() {
            critical_exports = false_0
        }
    } // already get through 'F'
    func = cdll_exports.as_mut_ptr();
    while !func.is_null() && !(*func).name.is_null() {
        if (*(*func).func).is_null() {
            // functions are cleared before all the extensions are evaluated
            *(*func).func =
                COM_GetProcAddress(clgame.hInstance, (*func).name);
            if (*(*func).func).is_null() {
                Con_Reportf(b"CL_LoadProgs: failed to get address of %s proc\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*func).name);
                if critical_exports as u64 != 0 {
                    COM_FreeLibrary(clgame.hInstance);
                    clgame.hInstance = 0 as *mut libc::c_void;
                    return false_0
                }
            }
        }
        func = func.offset(1)
    }
    // it may be loaded through 'GetClientAPI' so we don't need to clear them
    if critical_exports as u64 != 0 {
        // clear new exports
        func = cdll_new_exports.as_mut_ptr(); // already get through 'F'
        while !func.is_null() && !(*func).name.is_null() {
            *(*func).func = 0 as *mut libc::c_void;
            func = func.offset(1)
        }
    }
    func = cdll_new_exports.as_mut_ptr();
    while !func.is_null() && !(*func).name.is_null() {
        if (*(*func).func).is_null() {
            // functions are cleared before all the extensions are evaluated
		// NOTE: new exports can be missed without stop the engine
            *(*func).func =
                COM_GetProcAddress(clgame.hInstance,
                                   (*func).name); // will be alloc on first call CL_InitEdicts();
            if (*(*func).func).is_null() {
                Con_Reportf(b"CL_LoadProgs: failed to get address of %s proc\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*func).name); // world + localclient (have valid entities not in game)
            }
        }
        func = func.offset(1)
    }
    if clgame.dllFuncs.pfnInitialize.expect("non-null function pointer")(&mut gEngfuncs,
                                                                         7 as
                                                                             libc::c_int)
           == 0 {
        COM_FreeLibrary(clgame.hInstance);
        Con_Reportf(b"CL_LoadProgs: can\'t init client API\n\x00" as *const u8
                        as *const libc::c_char);
        clgame.hInstance = 0 as *mut libc::c_void;
        return false_0
    }
    Cvar_FullSet(b"host_clientloaded\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    clgame.maxRemapInfos = 0 as libc::c_int;
    clgame.maxEntities = 2 as libc::c_int;
    CL_InitCDAudio(b"media/cdaudio.txt\x00" as *const u8 as
                       *const libc::c_char);
    CL_InitTitles(b"titles.txt\x00" as *const u8 as *const libc::c_char);
    CL_InitParticles();
    CL_InitViewBeams();
    CL_InitTempEnts();
    if R_InitRenderAPI() as u64 == 0 {
        // Xash3D extension
        Con_Reportf(b"^3Warning:^7 CL_LoadProgs: couldn\'t get render API\n\x00"
                        as *const u8 as *const libc::c_char);
    }
    if Mobile_Init() as u64 == 0 {
        // Xash3D FWGS extension: mobile interface
        Con_Reportf(b"^3Warning:^7 CL_LoadProgs: couldn\'t get mobility API\n\x00"
                        as *const u8 as
                        *const libc::c_char); // initailize local player and world
    } // initialize pm_shared
    CL_InitEdicts();
    CL_InitClientMove();
    // initialize game
    clgame.dllFuncs.pfnInit.expect("non-null function pointer")();
    ref_0.dllFuncs.CL_InitStudioAPI.expect("non-null function pointer")();
    return true_0;
}
unsafe extern "C" fn run_static_initializers() {
    cdll_exports =
        [{
             let mut init =
                 dllfunc_s{name:
                               b"Initialize\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnInitialize as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut cl_enginefunc_t,
                                                                    _:
                                                                        libc::c_int)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_VidInit\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnVidInit as
                                   *mut Option<unsafe extern "C" fn()
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_Init\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnInit as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_Shutdown\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnShutdown as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_Redraw\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnRedraw as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_float,
                                                                    _:
                                                                        libc::c_int)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_UpdateClientData\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnUpdateClientData as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut client_data_t,
                                                                    _:
                                                                        libc::c_float)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_Reset\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnReset as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_PlayerMove\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnPlayerMove as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut playermove_s,
                                                                    _:
                                                                        libc::c_int)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_PlayerMoveInit\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnPlayerMoveInit as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut playermove_s)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_PlayerMoveTexture\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnPlayerMoveTexture as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_char)
                                                   -> libc::c_char> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_ConnectionlessPacket\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnConnectionlessPacket as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const netadr_s,
                                                                    _:
                                                                        *const libc::c_char,
                                                                    _:
                                                                        *mut libc::c_char,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_GetHullBounds\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnGetHullBounds as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut libc::c_float,
                                                                    _:
                                                                        *mut libc::c_float)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_Frame\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnFrame as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_double)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_PostRunCmd\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnPostRunCmd as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut local_state_s,
                                                                    _:
                                                                        *mut local_state_s,
                                                                    _:
                                                                        *mut usercmd_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_double,
                                                                    _:
                                                                        libc::c_uint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_Key_Event\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnKey_Event as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        *const libc::c_char)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_AddEntity\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnAddEntity as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut cl_entity_t,
                                                                    _:
                                                                        *const libc::c_char)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_CreateEntities\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnCreateEntities as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_StudioEvent\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnStudioEvent as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const mstudioevent_s,
                                                                    _:
                                                                        *const cl_entity_t)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_TxferLocalOverrides\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnTxferLocalOverrides as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut entity_state_t,
                                                                    _:
                                                                        *const clientdata_t)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_ProcessPlayerState\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnProcessPlayerState as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut entity_state_t,
                                                                    _:
                                                                        *const entity_state_t)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_TxferPredictionData\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnTxferPredictionData as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_TempEntUpdate\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnTempEntUpdate as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_double,
                                                                    _:
                                                                        libc::c_double,
                                                                    _:
                                                                        libc::c_double,
                                                                    _:
                                                                        *mut *mut tempent_s,
                                                                    _:
                                                                        *mut *mut tempent_s,
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
                                                                                   ->
                                                                                       ()>)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_DrawNormalTriangles\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnDrawNormalTriangles as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_DrawTransparentTriangles\x00" as
                                   *const u8 as *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnDrawTransparentTriangles
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_GetUserEntity\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnGetUserEntity as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int)
                                                   -> *mut cl_entity_t> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"Demo_ReadBuffer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnDemo_ReadBuffer as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut byte)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"CAM_Think\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.CAM_Think as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"CL_IsThirdPerson\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.CL_IsThirdPerson as
                                   *mut Option<unsafe extern "C" fn()
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"CL_CameraOffset\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.CL_CameraOffset as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_float)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"CL_CreateMove\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.CL_CreateMove as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_float,
                                                                    _:
                                                                        *mut usercmd_s,
                                                                    _:
                                                                        libc::c_int)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_ActivateMouse\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.IN_ActivateMouse as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_DeactivateMouse\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.IN_DeactivateMouse as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_MouseEvent\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.IN_MouseEvent as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_Accumulate\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.IN_Accumulate as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_ClearStates\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.IN_ClearStates as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"V_CalcRefdef\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnCalcRefdef as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut ref_params_t)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"KB_Find\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.KB_Find as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const libc::c_char)
                                                   -> *mut libc::c_void> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func: 0 as *mut *mut libc::c_void,};
             init
         }];
    cdll_new_exports =
        [{
             let mut init =
                 dllfunc_s{name:
                               b"HUD_GetStudioModelInterface\x00" as *const u8
                                   as *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnGetStudioModelInterface
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut *mut r_studio_interface_s,
                                                                    _:
                                                                        *mut engine_studio_api_s)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_DirectorMessage\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnDirectorMessage as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_VoiceStatus\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnVoiceStatus as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        qboolean)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_ChatInputPosition\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnChatInputPosition as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_GetRenderInterface\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnGetRenderInterface as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut render_api_t,
                                                                    _:
                                                                        *mut render_interface_t)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"HUD_ClipMoveToEntity\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnClipMoveToEntity as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut physent_s,
                                                                    _:
                                                                        *const vec_t,
                                                                    _:
                                                                        *mut vec_t,
                                                                    _:
                                                                        *mut vec_t,
                                                                    _:
                                                                        *const vec_t,
                                                                    _:
                                                                        *mut pmtrace_s)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_ClientTouchEvent\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnTouchEvent as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_float,
                                                                    _:
                                                                        libc::c_float,
                                                                    _:
                                                                        libc::c_float,
                                                                    _:
                                                                        libc::c_float)
                                                   -> libc::c_int> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_ClientMoveEvent\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnMoveEvent as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_float,
                                                                    _:
                                                                        libc::c_float)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"IN_ClientLookEvent\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &mut clgame.dllFuncs.pfnLookEvent as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_float,
                                                                    _:
                                                                        libc::c_float)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func: 0 as *mut *mut libc::c_void,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
