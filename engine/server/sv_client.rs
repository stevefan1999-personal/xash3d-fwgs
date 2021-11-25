#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, const_transmute,
           extern_types, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
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
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileExtension(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Info_Print(s: *const libc::c_char);
    #[no_mangle]
    fn Info_IsValid(s: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Info_SetValueForKey(s: *mut libc::c_char, key: *const libc::c_char,
                           value: *const libc::c_char, maxsize: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn Info_RemovePrefixedKeys(start: *mut libc::c_char,
                               prefix: libc::c_char);
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
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
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    fn NET_IsReservedAdr(a: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_CompareClassBAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_CompareBaseAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_SendPacket(sock: netsrc_t, length: size_t,
                      data: *const libc::c_void, to: netadr_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_TokenizeString(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_ExecuteString(text: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
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
    fn Q_buildcommit() -> *const libc::c_char;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_HexConvert(pszInput: *const libc::c_char,
                      nInputLength: libc::c_int, pOutput: *mut byte);
    #[no_mangle]
    fn COM_IsSafeFileToDownload(filename: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn COM_TrimSpace(source: *const libc::c_char, dest: *mut libc::c_char);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_ClearCustomizationList(pHead: *mut customization_t,
                                  bCleanDecals: qboolean);
    #[no_mangle]
    fn COM_SizeofResourceList(pList: *mut resource_t, ri: *mut resourceinfo_t)
     -> libc::c_int;
    #[no_mangle]
    fn HPAK_GetDataPointer(filename: *const libc::c_char,
                           pRes: *mut resource_s, buffer: *mut *mut byte,
                           size: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn HPAK_ResourceForHash(filename: *const libc::c_char, hash: *mut byte,
                            pRes: *mut resource_s) -> qboolean;
    #[no_mangle]
    fn SV_Serverinfo() -> *mut libc::c_char;
    #[no_mangle]
    fn CL_IsInGame() -> qboolean;
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SV_BroadcastPrintf(ignore: *mut sv_client_s, fmt: *const libc::c_char,
                          _: ...);
    #[no_mangle]
    fn UI_CreditsActive() -> qboolean;
    #[no_mangle]
    fn Mod_StudioTexName(modname: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn MSG_SeekToBit(sb: *mut sizebuf_t, bitPos: libc::c_int,
                     whence: libc::c_int) -> libc::c_int;
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
    fn MSG_WriteChar(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteWord(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteLong(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteFloat(sb: *mut sizebuf_t, val: libc::c_float);
    #[no_mangle]
    fn MSG_WriteVec3Angles(sb: *mut sizebuf_t, fa: *const libc::c_float);
    #[no_mangle]
    fn MSG_WriteBytes(sb: *mut sizebuf_t, pBuf: *const libc::c_void,
                      nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_ReadCmd(sb: *mut sizebuf_t, type_0: netsrc_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadBytes(sb: *mut sizebuf_t, pOut: *mut libc::c_void,
                     nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut net_local: netadr_t;
    #[no_mangle]
    static mut sv_lan: convar_t;
    #[no_mangle]
    static mut net_drop: libc::c_int;
    #[no_mangle]
    fn Netchan_Setup(sock: netsrc_t, chan: *mut netchan_t, adr: netadr_t,
                     qport: libc::c_int, client: *mut libc::c_void,
                     pfnBlockSize:
                         Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: fragsize_t)
                                    -> libc::c_int>);
    #[no_mangle]
    fn Netchan_CreateFileFragmentsFromBuffer(chan: *mut netchan_t,
                                             filename: *const libc::c_char,
                                             pbuf: *mut byte,
                                             size: libc::c_int);
    #[no_mangle]
    fn Netchan_CreateFragments(chan: *mut netchan_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn Netchan_CreateFileFragments(chan: *mut netchan_t,
                                   filename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn Netchan_TransmitBits(chan: *mut netchan_t, lengthInBits: libc::c_int,
                            data: *mut byte);
    #[no_mangle]
    fn Netchan_OutOfBand(net_socket: libc::c_int, adr: netadr_t,
                         length: libc::c_int, data: *mut byte);
    #[no_mangle]
    fn Netchan_OutOfBandPrint(net_socket: libc::c_int, adr: netadr_t,
                              format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Netchan_IsLocal(chan: *mut netchan_t) -> qboolean;
    #[no_mangle]
    fn Netchan_FragSend(chan: *mut netchan_t);
    #[no_mangle]
    fn Netchan_Clear(chan: *mut netchan_t);
    #[no_mangle]
    static mut SV_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    static mut sv_unlagsamples: convar_t;
    #[no_mangle]
    static mut rcon_password: convar_t;
    #[no_mangle]
    static mut sv_minupdaterate: convar_t;
    #[no_mangle]
    static mut sv_maxupdaterate: convar_t;
    #[no_mangle]
    static mut sv_minrate: convar_t;
    #[no_mangle]
    static mut sv_maxrate: convar_t;
    #[no_mangle]
    static mut sv_send_resources: convar_t;
    #[no_mangle]
    static mut sv_send_logos: convar_t;
    #[no_mangle]
    static mut sv_allow_upload: convar_t;
    #[no_mangle]
    static mut sv_allow_download: convar_t;
    #[no_mangle]
    static mut sv_password: convar_t;
    #[no_mangle]
    static mut sv_uploadmax: convar_t;
    #[no_mangle]
    static mut hostname: convar_t;
    #[no_mangle]
    static mut sv_pausable: *mut convar_t;
    #[no_mangle]
    fn SV_MakeString(szValue: *const libc::c_char) -> string_t;
    #[no_mangle]
    fn SV_AddToMaster(from: netadr_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_ProcessUserAgent(from: netadr_t, useragent: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn SV_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn SV_ClientPrintf(cl: *mut sv_client_t, fmt: *const libc::c_char,
                       _: ...);
    #[no_mangle]
    fn SV_SendUserReg(msg: *mut sizebuf_t, user: *mut sv_user_message_t);
    #[no_mangle]
    fn SV_GetString(iString: string_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_ParseConsistencyResponse(cl: *mut sv_client_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_BatchUploadRequest(cl: *mut sv_client_t);
    #[no_mangle]
    fn SV_ClearResourceList(pList: *mut resource_t);
    #[no_mangle]
    fn SV_EstimateNeededResources(cl: *mut sv_client_t) -> libc::c_int;
    #[no_mangle]
    fn SV_AddToResourceList(pResource: *mut resource_t,
                            pList: *mut resource_t);
    #[no_mangle]
    fn SV_RestartStaticEnts();
    #[no_mangle]
    fn SV_RestartDecals();
    #[no_mangle]
    fn SV_RestartAmbientSounds();
    #[no_mangle]
    fn SV_SendResources(cl: *mut sv_client_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_InitEdict(pEdict: *mut edict_t);
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_RunCmd(cl: *mut sv_client_t, ucmd: *mut usercmd_t,
                 random_seed: libc::c_int);
    #[no_mangle]
    fn SV_PlayerIsFrozen(pClient: *mut edict_t) -> qboolean;
    #[no_mangle]
    fn SV_ClearResourceLists(cl: *mut sv_client_t);
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn SV_CheckIP(adr: *mut netadr_t) -> qboolean;
    #[no_mangle]
    fn Delta_NumTables() -> libc::c_int;
    #[no_mangle]
    fn Delta_FindStructByIndex(index: libc::c_int) -> *mut delta_info_t;
    #[no_mangle]
    fn Delta_WriteTableField(msg: *mut sizebuf_t, tableIndex: libc::c_int,
                             pField: *const delta_t);
    #[no_mangle]
    fn MSG_ReadDeltaUsercmd(msg: *mut sizebuf_t, from: *mut usercmd_s,
                            to: *mut usercmd_s);
    #[no_mangle]
    fn MSG_WriteDeltaMovevars(msg: *mut sizebuf_t, from: *mut movevars_s,
                              to: *mut movevars_s) -> qboolean;
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
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
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
pub type sv_state_t = libc::c_uint;
pub const ss_active: sv_state_t = 2;
pub const ss_loading: sv_state_t = 1;
pub const ss_dead: sv_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_baseline_t {
    pub classname: *const libc::c_char,
    pub baseline: entity_state_t,
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
pub type server_t = server_s;
pub type sv_client_t = sv_client_s;
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
pub struct delta_info_t {
    pub pName: *const libc::c_char,
    pub pInfo: *const delta_field_t,
    pub maxFields: libc::c_int,
    pub numFields: libc::c_int,
    pub pFields: *mut delta_t,
    pub customEncode: libc::c_int,
    pub funcName: [libc::c_char; 32],
    pub userCallback: pfnDeltaEncode,
    pub bInitialized: qboolean,
}
pub type pfnDeltaEncode
    =
    Option<unsafe extern "C" fn(_: *mut delta_s, _: *const byte,
                                _: *const byte) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta_field_t {
    pub name: *const libc::c_char,
    pub offset: libc::c_int,
    pub size: libc::c_int,
}
pub type ucmd_t = ucmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucmd_s {
    pub name: *const libc::c_char,
    pub func: Option<unsafe extern "C" fn(_: *mut sv_client_t) -> qboolean>,
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
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
unsafe extern "C" fn MSG_GetRealBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetData(mut sb: *mut sizebuf_t) -> *mut byte {
    return (*sb).pData;
}
/*
sv_client.c - client interactions
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
pub static mut clc_strings: [*const libc::c_char; 11] =
    [b"clc_bad\x00" as *const u8 as *const libc::c_char,
     b"clc_nop\x00" as *const u8 as *const libc::c_char,
     b"clc_move\x00" as *const u8 as *const libc::c_char,
     b"clc_stringcmd\x00" as *const u8 as *const libc::c_char,
     b"clc_delta\x00" as *const u8 as *const libc::c_char,
     b"clc_resourcelist\x00" as *const u8 as *const libc::c_char,
     b"clc_unused6\x00" as *const u8 as *const libc::c_char,
     b"clc_fileconsistency\x00" as *const u8 as *const libc::c_char,
     b"clc_voicedata\x00" as *const u8 as *const libc::c_char,
     b"clc_cvarvalue\x00" as *const u8 as *const libc::c_char,
     b"clc_cvarvalue2\x00" as *const u8 as *const libc::c_char];
static mut g_userid: libc::c_int = 1 as libc::c_int;
/*
=================
SV_GetChallenge

Returns a challenge number that can be used
in a subsequent client_connect command.
We do this to prevent denial of service attacks that
flood the server with invalid connection IPs.  With a
challenge, they must give a valid IP address.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetChallenge(mut from: netadr_t) {
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0 as libc::c_int;
    let mut oldestTime: libc::c_double = 0.;
    oldestTime = 0x7fffffff as libc::c_int as libc::c_double;
    // see if we already have a challenge for this ip
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if svs.challenges[i as usize].connected as u64 == 0 &&
               NET_CompareAdr(from, svs.challenges[i as usize].adr) as
                   libc::c_uint != 0 {
            break ;
        }
        if svs.challenges[i as usize].time < oldestTime {
            oldestTime = svs.challenges[i as usize].time;
            oldest = i
        }
        i += 1
    }
    if i == 1024 as libc::c_int {
        // this is the first time this client has asked for a challenge
        svs.challenges[oldest as usize].challenge =
            COM_RandomLong(0 as libc::c_int, 0xffff as libc::c_int) <<
                16 as libc::c_int |
                COM_RandomLong(0 as libc::c_int, 0xffff as libc::c_int);
        svs.challenges[oldest as usize].adr = from;
        svs.challenges[oldest as usize].time = host.realtime;
        svs.challenges[oldest as usize].connected = false_0;
        i = oldest
    }
    // send it back
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int,
                           svs.challenges[i as usize].adr,
                           b"challenge %i\x00" as *const u8 as
                               *const libc::c_char,
                           svs.challenges[i as usize].challenge);
}
#[no_mangle]
pub unsafe extern "C" fn SV_GetFragmentSize(mut pcl: *mut libc::c_void,
                                            mut mode: fragsize_t)
 -> libc::c_int {
    let mut cl: *mut sv_client_t = pcl as *mut sv_client_t;
    let mut cl_frag_size: libc::c_int = 0;
    if Netchan_IsLocal(&mut (*cl).netchan) as u64 != 0 {
        return 64000 as libc::c_int
    }
    if mode as libc::c_uint ==
           FRAGSIZE_UNRELIABLE as libc::c_int as libc::c_uint {
        // allow setting unreliable limit with "setinfo cl_urmax"
        cl_frag_size =
            Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                                    b"cl_urmax\x00" as *const u8 as
                                        *const libc::c_char));
        if cl_frag_size == 0 as libc::c_int {
            return (0x20000 as libc::c_int +
                        (8 as libc::c_int +
                             2 as libc::c_int * 13 as libc::c_int) +
                        (16 as libc::c_int - 1 as libc::c_int)) /
                       16 as libc::c_int * 16 as libc::c_int
        }
        return if cl_frag_size >= 64000 as libc::c_int {
                   if cl_frag_size <
                          (0x20000 as libc::c_int +
                               (8 as libc::c_int +
                                    2 as libc::c_int * 13 as libc::c_int) +
                               (16 as libc::c_int - 1 as libc::c_int)) /
                              16 as libc::c_int * 16 as libc::c_int {
                       cl_frag_size
                   } else {
                       ((0x20000 as libc::c_int +
                             (8 as libc::c_int +
                                  2 as libc::c_int * 13 as libc::c_int) +
                             (16 as libc::c_int - 1 as libc::c_int)) /
                            16 as libc::c_int) * 16 as libc::c_int
                   }
               } else { 64000 as libc::c_int }
    }
    cl_frag_size =
        Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                                b"cl_dlmax\x00" as *const u8 as
                                    *const libc::c_char));
    cl_frag_size =
        if cl_frag_size >= 508 as libc::c_int {
            if cl_frag_size < 64000 as libc::c_int {
                cl_frag_size
            } else { 64000 as libc::c_int }
        } else { 508 as libc::c_int };
    if mode as libc::c_uint != FRAGSIZE_FRAG as libc::c_int as libc::c_uint {
        if (*cl).extensions as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int != 0 {
            return cl_frag_size
        } else { return 0 as libc::c_int }
        // original engine behaviour
    }
    // get in-game fragmentation size
    if (*cl).state as libc::c_uint ==
           cs_spawned as libc::c_int as libc::c_uint {
        // allow setting in-game fragsize with "setinfo cl_frmax"
        let mut frmax: libc::c_int =
            Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                                    b"cl_frmax\x00" as *const u8 as
                                        *const libc::c_char)); // add window for unreliable
        if frmax < 508 as libc::c_int || frmax > 64000 as libc::c_int {
            cl_frag_size /= 2 as libc::c_int
        } else { cl_frag_size = frmax }
    }
    return cl_frag_size -
               (8 as libc::c_int + 2 as libc::c_int * 13 as libc::c_int);
}
/*
================
SV_RejectConnection

Rejects connection request and sends back a message
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RejectConnection(mut from: netadr_t,
                                             mut fmt: *const libc::c_char,
                                             mut args: ...) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    Q_vsnprintf(text.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as
                    libc::c_ulong, fmt, argptr.as_va_list());
    Con_Reportf(b"%s connection refused. Reason: %s\n\x00" as *const u8 as
                    *const libc::c_char, NET_AdrToString(from),
                text.as_mut_ptr());
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                           b"errormsg\n^1Server was reject the connection:^7 %s\x00"
                               as *const u8 as *const libc::c_char,
                           text.as_mut_ptr());
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                           b"print\n^1Server was reject the connection:^7 %s\x00"
                               as *const u8 as *const libc::c_char,
                           text.as_mut_ptr());
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                           b"disconnect\n\x00" as *const u8 as
                               *const libc::c_char);
}
/*
================
SV_FailDownload

for some reasons file can't be downloaded
tell the client about this problem
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FailDownload(mut cl: *mut sv_client_t,
                                         mut filename: *const libc::c_char) {
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    MSG_WriteCmdExt(&mut (*cl).netchan.message, 49 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut (*cl).netchan.message, filename);
}
/*
================
SV_CheckChallenge

Make sure connecting client is not spoofing
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckChallenge(mut from: netadr_t,
                                           mut challenge: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    // see if the challenge is valid
	// don't care if it is a local address.
    if NET_IsLocalAddress(from) as u64 != 0 { return 1 as libc::c_int }
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if NET_CompareAdr(from, svs.challenges[i as usize].adr) as u64 != 0 {
            if challenge == svs.challenges[i as usize].challenge {
                break ;
                // valid challenge
            }
        }
        i += 1
    }
    if i == 1024 as libc::c_int {
        SV_RejectConnection(from,
                            b"no challenge for your address\n\x00" as
                                *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    svs.challenges[i as usize].connected = true_0;
    return 1 as libc::c_int;
}
/*
================
SV_CheckIPRestrictions

Determine if client is outside appropriate address range
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckIPRestrictions(mut from: netadr_t)
 -> libc::c_int {
    if sv_lan.value != 0. {
        if NET_CompareClassBAdr(from, net_local) as u64 == 0 &&
               NET_IsReservedAdr(from) as u64 == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/*
================
SV_FindEmptySlot

Get slot # and set client_t pointer for player, if possible
We don't do this search on a "reconnect, we just reuse the slot
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FindEmptySlot(mut from: netadr_t,
                                          mut pslot: *mut libc::c_int,
                                          mut ppClient: *mut *mut sv_client_t)
 -> libc::c_int {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint ==
               cs_free as libc::c_int as libc::c_uint {
            *ppClient = cl;
            *pslot = i;
            return 1 as libc::c_int
        }
        i += 1;
        cl = cl.offset(1)
    }
    SV_RejectConnection(from,
                        b"server is full\n\x00" as *const u8 as
                            *const libc::c_char);
    return 0 as libc::c_int;
}
/*
==================
SV_ConnectClient

A connection request that did not come from the master
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ConnectClient(mut from: netadr_t) {
    let mut userinfo: [libc::c_char; 256] = [0; 256]; // get challenge
    let mut protinfo: [libc::c_char; 256] = [0; 256];
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut newcl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut reconnect: qboolean = false_0;
    let mut nClientSlot: libc::c_int = 0 as libc::c_int;
    let mut qport: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut challenge: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut extensions: libc::c_int = 0;
    if Cmd_Argc() < 5 as libc::c_int {
        SV_RejectConnection(from,
                            b"insufficient connection info\n\x00" as *const u8
                                as *const libc::c_char);
        return
    }
    version = Q_atoi(Cmd_Argv(1 as libc::c_int));
    if version != 49 as libc::c_int {
        SV_RejectConnection(from,
                            b"unsupported protocol (%i should be %i)\n\x00" as
                                *const u8 as *const libc::c_char, version,
                            49 as libc::c_int);
        return
    }
    challenge = Q_atoi(Cmd_Argv(2 as libc::c_int));
    // see if the challenge is valid (local clients don't need to challenge)
    if SV_CheckChallenge(from, challenge) == 0 { return } // protocol info
    s = Cmd_Argv(3 as libc::c_int);
    if Info_IsValid(s) as u64 == 0 {
        SV_RejectConnection(from,
                            b"invalid protinfo in connect command\n\x00" as
                                *const u8 as *const libc::c_char);
        return
    }
    Q_strncpy(protinfo.as_mut_ptr(), s,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    if SV_ProcessUserAgent(from, protinfo.as_mut_ptr()) as u64 == 0 { return }
    // extract qport from protocol info
    qport =
        Q_atoi(Info_ValueForKey(protinfo.as_mut_ptr(),
                                b"qport\x00" as *const u8 as
                                    *const libc::c_char));
    s =
        Info_ValueForKey(protinfo.as_mut_ptr(),
                         b"uuid\x00" as *const u8 as *const libc::c_char);
    if Q_strlen(s) != 32 as libc::c_int as libc::c_ulong {
        SV_RejectConnection(from,
                            b"invalid authentication certificate length\n\x00"
                                as *const u8 as *const libc::c_char);
        return
    }
    extensions =
        Q_atoi(Info_ValueForKey(protinfo.as_mut_ptr(),
                                b"ext\x00" as *const u8 as
                                    *const libc::c_char));
    // LAN servers restrict to class b IP addresses
    if SV_CheckIPRestrictions(from) == 0 {
        SV_RejectConnection(from,
                            b"LAN servers are restricted to local clients (class C)\n\x00"
                                as *const u8 as
                                *const libc::c_char); // user info
        return
    }
    s = Cmd_Argv(4 as libc::c_int);
    if Q_strlen(s) > 256 as libc::c_int as libc::c_ulong ||
           Info_IsValid(s) as u64 == 0 {
        SV_RejectConnection(from,
                            b"invalid userinfo in connect command\n\x00" as
                                *const u8 as *const libc::c_char);
        return
    }
    Q_strncpy(userinfo.as_mut_ptr(), s,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    // check connection password (don't verify local client)
    if NET_IsLocalAddress(from) as u64 == 0 &&
           *sv_password.string.offset(0 as libc::c_int as isize) as
               libc::c_int != 0 &&
           Q_strnicmp(sv_password.string,
                      Info_ValueForKey(userinfo.as_mut_ptr(),
                                       b"password\x00" as *const u8 as
                                           *const libc::c_char),
                      99999 as libc::c_int) != 0 {
        SV_RejectConnection(from,
                            b"invalid password\n\x00" as *const u8 as
                                *const libc::c_char);
        return
    }
    // if there is already a slot for this ip, reuse it
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !((*cl).state as libc::c_uint ==
                 cs_free as libc::c_int as libc::c_uint ||
                 (*cl).state as libc::c_uint ==
                     cs_zombie as libc::c_int as libc::c_uint) {
            if NET_CompareBaseAdr(from, (*cl).netchan.remote_address) as
                   libc::c_uint != 0 &&
                   ((*cl).netchan.qport == qport ||
                        from.port as libc::c_int ==
                            (*cl).netchan.remote_address.port as libc::c_int)
               {
                reconnect = true_0;
                newcl = cl;
                break ;
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    // A reconnecting client will re-use the slot found above when checking for reconnection.
	// the slot will be wiped clean.
    if reconnect as u64 == 0 {
        // connect the client if there are empty slots.
        if SV_FindEmptySlot(from, &mut nClientSlot, &mut newcl) == 0 {
            return
        }
    } else {
        Con_Reportf(b"^2Note:^7 %s:reconnect\n\x00" as *const u8 as
                        *const libc::c_char, NET_AdrToString(from));
    }
    // find a client slot
    if newcl.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char, 375 as libc::c_int);
    }
    // build a new connection
	// accept the new client
    sv.current_client = newcl; // save challenge for checksumming
    (*newcl).edict =
        SV_EdictNum((newcl.wrapping_offset_from(svs.clients) as libc::c_long +
                         1 as libc::c_int as libc::c_long) as
                        libc::c_int); // create unique userid
    (*newcl).challenge = challenge;
    if !(*newcl).frames.is_null() {
        _Mem_Free((*newcl).frames as *mut libc::c_void,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char, 383 as libc::c_int);
    }
    (*newcl).frames =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<client_frame_t>() as
                        libc::c_ulong).wrapping_mul(SV_UPDATE_BACKUP as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_client.c\x00" as *const u8 as
                       *const libc::c_char, 384 as libc::c_int) as
            *mut client_frame_t;
    let fresh0 = g_userid;
    g_userid = g_userid + 1;
    (*newcl).userid = fresh0;
    (*newcl).state = cs_connected;
    (*newcl).extensions =
        (extensions as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int)
            as libc::c_int;
    Q_strncpy((*newcl).useragent.as_mut_ptr(), protinfo.as_mut_ptr(),
              256 as libc::c_int as size_t);
    // reset viewentities (from previous level)
    memset((*newcl).viewentity.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut edict_t; 128]>() as libc::c_ulong);
    (*newcl).num_viewents = 0 as libc::c_int;
    (*newcl).listeners = 0 as libc::c_int as uint;
    // initailize netchan
    Netchan_Setup(NS_SERVER, &mut (*newcl).netchan, from, qport,
                  newcl as *mut libc::c_void,
                  Some(SV_GetFragmentSize as
                           unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: fragsize_t)
                               -> libc::c_int)); // datagram buf
    MSG_InitExt(&mut (*newcl).datagram,
                b"Datagram\x00" as *const u8 as *const libc::c_char,
                (*newcl).datagram_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    Q_strncpy((*newcl).hashedcdkey.as_mut_ptr(),
              Info_ValueForKey(protinfo.as_mut_ptr(),
                               b"uuid\x00" as *const u8 as
                                   *const libc::c_char),
              32 as libc::c_int as size_t);
    (*newcl).hashedcdkey[32 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    // build protinfo answer
    protinfo[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    Info_SetValueForKey(protinfo.as_mut_ptr(),
                        b"ext\x00" as *const u8 as *const libc::c_char,
                        va(b"%d\x00" as *const u8 as *const libc::c_char,
                           (*newcl).extensions),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as
                            libc::c_ulong as libc::c_int);
    // send the connect packet to the client
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                           b"client_connect %s\x00" as *const u8 as
                               *const libc::c_char,
                           protinfo.as_mut_ptr()); // 20 fps as default
    (*newcl).upstate = us_inactive;
    (*newcl).connection_started = host.realtime;
    (*newcl).cl_updaterate = 0.05f64;
    (*newcl).delta_sequence = -(1 as libc::c_int);
    (*newcl).flags = 0 as libc::c_int as uint;
    // reset any remaining events
    memset(&mut (*newcl).events as *mut event_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<event_state_t>() as libc::c_ulong);
    // parse some info from the info strings (this can override cl_updaterate)
    Q_strncpy((*newcl).userinfo.as_mut_ptr(), userinfo.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    SV_UserinfoChanged(newcl);
    SV_ClearResourceLists(newcl);
    (*newcl).next_messagetime = host.realtime + (*newcl).cl_updaterate;
    (*newcl).next_sendinfotime = 0.0f64;
    (*newcl).ignored_ents = 0 as libc::c_int;
    (*newcl).chokecount = 0 as libc::c_int;
    // reset stats
    (*newcl).next_checkpingtime = -1.0f64;
    (*newcl).packet_loss = 0.0f32 as libc::c_int;
    // if this was the first client on the server, or the last client
	// the server can hold, send a heartbeat to the master.
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint {
            count += 1
        }
        i += 1;
        cl = cl.offset(1)
    }
    Log_Printf(b"\"%s<%i><%i><>\" connected, address \"%s\"\n\x00" as
                   *const u8 as *const libc::c_char,
               (*newcl).name.as_mut_ptr(), (*newcl).userid, i,
               NET_AdrToString((*newcl).netchan.remote_address));
    if count == 1 as libc::c_int || count == svs.maxclients {
        svs.last_heartbeat = -(99999 as libc::c_int) as libc::c_double
    };
}
/*
==================
SV_FakeConnect

A connection request that came from the game module
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FakeConnect(mut netname: *const libc::c_char)
 -> *mut edict_t {
    let mut userinfo: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if if netname.is_null() || *netname == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        netname = b"Bot\x00" as *const u8 as *const libc::c_char
    }
    // find a client slot
    i = 0 as libc::c_int; // server is full
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint ==
               cs_free as libc::c_int as libc::c_uint {
            break ;
        }
        i += 1;
        cl = cl.offset(1)
    }
    if i == svs.maxclients { return 0 as *mut edict_t }
    userinfo[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    // setup fake client params
    Info_SetValueForKey(userinfo.as_mut_ptr(),
                        b"name\x00" as *const u8 as *const libc::c_char,
                        netname, 256 as libc::c_int);
    Info_SetValueForKey(userinfo.as_mut_ptr(),
                        b"model\x00" as *const u8 as *const libc::c_char,
                        b"gordon\x00" as *const u8 as *const libc::c_char,
                        256 as libc::c_int);
    Info_SetValueForKey(userinfo.as_mut_ptr(),
                        b"topcolor\x00" as *const u8 as *const libc::c_char,
                        b"1\x00" as *const u8 as *const libc::c_char,
                        256 as libc::c_int);
    Info_SetValueForKey(userinfo.as_mut_ptr(),
                        b"bottomcolor\x00" as *const u8 as
                            *const libc::c_char,
                        b"1\x00" as *const u8 as *const libc::c_char,
                        256 as libc::c_int);
    // build a new connection
	// accept the new client
    sv.current_client = cl; // fakeclients doesn't have frames
    if !(*cl).frames.is_null() {
        _Mem_Free((*cl).frames as *mut libc::c_void,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char,
                  488 as libc::c_int); // create unique userid
    }
    memset(cl as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sv_client_t>() as libc::c_ulong);
    (*cl).edict =
        SV_EdictNum((cl.wrapping_offset_from(svs.clients) as libc::c_long +
                         1 as libc::c_int as libc::c_long) as libc::c_int);
    let fresh1 = g_userid;
    g_userid = g_userid + 1;
    (*cl).userid = fresh1;
    (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 7 as libc::c_int;
    // parse some info from the info strings
    Q_strncpy((*cl).userinfo.as_mut_ptr(), userinfo.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    SV_UserinfoChanged(cl);
    (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 0 as libc::c_int;
    (*cl).next_sendinfotime = 0.0f64;
    // if this was the first client on the server, or the last client
	// the server can hold, send a heartbeat to the master.
    i = 0 as libc::c_int; // mark it as fakeclient
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint {
            count += 1
        }
        i += 1;
        cl = cl.offset(1)
    }
    cl = sv.current_client;
    Log_Printf(b"\"%s<%i><%i><>\" connected, address \"local\"\n\x00" as
                   *const u8 as *const libc::c_char, (*cl).name.as_mut_ptr(),
               (*cl).userid, i);
    (*(*cl).edict).v.flags =
        ((*(*cl).edict).v.flags as libc::c_uint |
             ((1 as libc::c_uint) << 3 as libc::c_int |
                  (1 as libc::c_uint) << 13 as libc::c_int)) as libc::c_int;
    (*cl).connection_started = host.realtime;
    (*cl).state = cs_spawned;
    if count == 1 as libc::c_int || count == svs.maxclients {
        svs.last_heartbeat = -(99999 as libc::c_int) as libc::c_double
    }
    return (*cl).edict;
}
/*
=====================
SV_DropClient

Called when the player is totally leaving the server, either willingly
or unwillingly.  This is NOT called if the entire server is quiting
or crashing.
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_DropClient(mut cl: *mut sv_client_t,
                                       mut crash: qboolean) {
    let mut i: libc::c_int = 0; // already dropped
    if (*cl).state as libc::c_uint == cs_zombie as libc::c_int as libc::c_uint
       {
        return
    }
    if crash as u64 == 0 {
        // add the disconnect
        if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
            MSG_WriteCmdExt(&mut (*cl).netchan.message, 2 as libc::c_int,
                            NS_SERVER,
                            0 as
                                *const libc::c_char); // become free in a few seconds
        } // release delta
        if !(*cl).edict.is_null() &&
               (*cl).state as libc::c_uint ==
                   cs_spawned as libc::c_int as libc::c_uint {
            svgame.dllFuncs.pfnClientDisconnect.expect("non-null function pointer")((*cl).edict);
        }
        if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
            Netchan_TransmitBits(&mut (*cl).netchan, 0 as libc::c_int,
                                 0 as *mut byte);
        }
    }
    (*cl).flags = (*cl).flags & !((1 as libc::c_uint) << 7 as libc::c_int);
    (*cl).flags = (*cl).flags & !((1 as libc::c_uint) << 8 as libc::c_int);
    (*cl).state = cs_zombie;
    (*cl).name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if !(*cl).frames.is_null() {
        _Mem_Free((*cl).frames as *mut libc::c_void,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char, 560 as libc::c_int);
    }
    (*cl).frames = 0 as *mut client_frame_t;
    if NET_CompareBaseAdr((*cl).netchan.remote_address, host.rd.address) as
           u64 != 0 {
        SV_EndRedirect();
    }
    // throw away any residual garbage in the channel.
    Netchan_Clear(&mut (*cl).netchan);
    // clean client data on disconnect
    memset((*cl).userinfo.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    memset((*cl).physinfo.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    COM_ClearCustomizationList(&mut (*cl).customdata, false_0);
    // don't send to other clients
    (*cl).edict = 0 as *mut edict_t;
    // send notification to all other clients
    SV_FullClientUpdate(cl, &mut sv.reliable_datagram);
    // if this was the last client on the server, send a heartbeat
	// to the master so it is known the server is empty
	// send a heartbeat now so the master will get up to date info
	// if there is already a slot for this ip, reuse it
    i = 0 as libc::c_int;
    while i < svs.maxclients {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint {
            break ;
        }
        i += 1
    }
    if i == svs.maxclients {
        svs.last_heartbeat = -(99999 as libc::c_int) as libc::c_double
    };
}
/*
==============================================================================

SVC COMMAND REDIRECT

==============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BeginRedirect(mut adr: netadr_t,
                                          mut target: rdtype_t,
                                          mut buffer: *mut libc::c_char,
                                          mut buffersize: size_t,
                                          mut flush: *mut libc::c_void) {
    if target as u64 == 0 || buffer.is_null() || buffersize == 0 ||
           flush.is_null() {
        return
    } // client not set
    host.rd.target = target;
    host.rd.buffer = buffer;
    host.rd.buffersize = buffersize;
    host.rd.flush =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: netadr_t,
                                                            _: rdtype_t,
                                                            _:
                                                                *mut libc::c_char)
                                           -> ()>>(flush);
    host.rd.address = adr;
    *host.rd.buffer.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    if host.rd.lines == 0 as libc::c_int {
        host.rd.lines = -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_FlushRedirect(mut adr: netadr_t,
                                          mut dest: libc::c_int,
                                          mut buf: *mut libc::c_char) {
    if !sv.current_client.is_null() &&
           (*sv.current_client).flags &
               (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        return
    }
    match dest {
        2 => {
            Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, adr,
                                   b"print\n%s\x00" as *const u8 as
                                       *const libc::c_char, buf);
        }
        1 => {
            if sv.current_client.is_null() { return }
            MSG_WriteCmdExt(&mut (*sv.current_client).netchan.message,
                            8 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteString(&mut (*sv.current_client).netchan.message, buf);
        }
        0 => {
            Con_Printf(b"^1Error:^7 SV_FlushRedirect: %s: invalid destination\n\x00"
                           as *const u8 as *const libc::c_char,
                       NET_AdrToString(adr));
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_EndRedirect() {
    if host.rd.lines > 0 as libc::c_int { return }
    if host.rd.flush.is_some() {
        host.rd.flush.expect("non-null function pointer")(host.rd.address,
                                                          host.rd.target,
                                                          host.rd.buffer);
    }
    host.rd.target = RD_NONE;
    host.rd.buffer = 0 as *mut libc::c_char;
    host.rd.buffersize = 0 as libc::c_int as size_t;
    host.rd.flush = None;
}
/*
================
Rcon_Print

Print message to rcon buffer and send to rcon redirect target
================
*/
#[no_mangle]
pub unsafe extern "C" fn Rcon_Print(mut pMsg: *const libc::c_char) {
    if host.rd.target as libc::c_uint != 0 && host.rd.lines != 0 &&
           host.rd.flush.is_some() && !host.rd.buffer.is_null() {
        let mut len: size_t =
            Q_strncat(host.rd.buffer, pMsg, host.rd.buffersize);
        if len != 0 &&
               *host.rd.buffer.offset(len.wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong) as
                                          isize) as libc::c_int == '\n' as i32
           {
            host.rd.flush.expect("non-null function pointer")(host.rd.address,
                                                              host.rd.target,
                                                              host.rd.buffer);
            if host.rd.lines > 0 as libc::c_int { host.rd.lines -= 1 }
            *host.rd.buffer.offset(0 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char;
            if host.rd.lines == 0 {
                Con_Printf(b"End of redirection!\n\x00" as *const u8 as
                               *const libc::c_char);
            }
        }
    };
}
/*
===============
SV_GetClientIDString

Returns a pointer to a static char for most likely only printing.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetClientIDString(mut cl: *mut sv_client_t)
 -> *const libc::c_char {
    static mut result: [libc::c_char; 64] = [0; 64];
    if cl.is_null() { return b"\x00" as *const u8 as *const libc::c_char }
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        Q_strncpy(result.as_mut_ptr(),
                  b"ID_BOT\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
    } else if NET_IsLocalAddress((*cl).netchan.remote_address) as u64 != 0 {
        Q_strncpy(result.as_mut_ptr(),
                  b"ID_LOOPBACK\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
    } else if sv_lan.value != 0. {
        Q_strncpy(result.as_mut_ptr(),
                  b"ID_LAN\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
    } else {
        Q_snprintf(result.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong,
                   b"ID_%s\x00" as *const u8 as *const libc::c_char,
                   MD5_Print((*cl).hashedcdkey.as_mut_ptr() as *mut byte));
    }
    return result.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClientById(mut id: libc::c_int)
 -> *mut sv_client_t {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    if !(id >= 0 as libc::c_int) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char, 717 as libc::c_int);
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < (*svgame.globals).maxClients {
        if !((*cl).state as u64 == 0) { if (*cl).userid == id { return cl } }
        i += 1;
        cl = cl.offset(1)
    }
    return 0 as *mut sv_client_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClientByName(mut name: *const libc::c_char)
 -> *mut sv_client_t {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    if !(!name.is_null() && *name as libc::c_int != 0) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char, 736 as libc::c_int);
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < (*svgame.globals).maxClients {
        if !((*cl).state as u64 == 0) {
            if Q_strncmp((*cl).name.as_mut_ptr(), name, 99999 as libc::c_int)
                   == 0 {
                return cl
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    return 0 as *mut sv_client_t;
}
/*
================
SV_TestBandWidth

================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TestBandWidth(mut from: netadr_t) {
    let mut version: libc::c_int = Q_atoi(Cmd_Argv(1 as libc::c_int));
    let mut packetsize: libc::c_int = Q_atoi(Cmd_Argv(2 as libc::c_int));
    let mut send_buf: [byte; 64000] = [0; 64000];
    let mut crcValue: dword = 0 as libc::c_int as dword;
    let mut filepos: *mut byte = 0 as *mut byte;
    let mut crcpos: libc::c_int = 0;
    let mut test: *mut file_t = 0 as *mut file_t;
    let mut send: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    // don't waste time of protocol mismatched
    if version != 49 as libc::c_int {
        SV_RejectConnection(from,
                            b"unsupported protocol (%i should be %i)\n\x00" as
                                *const u8 as *const libc::c_char, version,
                            49 as libc::c_int);
        return
    }
    test =
        FS_Open(b"gfx.wad\x00" as *const u8 as *const libc::c_char,
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if (FS_FileLength(test) as libc::c_ulong) <
           ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong {
        // skip the test and just get challenge
        SV_GetChallenge(from);
        return
    }
    // write the packet header
    MSG_InitExt(&mut send,
                b"BandWidthPacket\x00" as *const u8 as *const libc::c_char,
                send_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong as
                    libc::c_int,
                -(1 as libc::c_int)); // -1 sequence means out of band
    MSG_WriteLong(&mut send, -(1 as libc::c_int)); // reserve space for crc
    MSG_WriteString(&mut send,
                    b"testpacket\x00" as *const u8 as
                        *const libc::c_char); // adjust the packet size
    crcpos = MSG_GetNumBytesWritten(&mut send); // calc CRC
    MSG_WriteLong(&mut send, 0 as libc::c_int);
    filepos = send.pData.offset(MSG_GetNumBytesWritten(&mut send) as isize);
    packetsize = packetsize - MSG_GetNumBytesWritten(&mut send);
    FS_Read(test, filepos as *mut libc::c_void, packetsize as size_t);
    FS_Close(test);
    CRC32_ProcessBuffer(&mut crcValue, filepos as *const libc::c_void,
                        packetsize);
    MSG_SeekToBit(&mut send, packetsize << 3 as libc::c_int,
                  1 as libc::c_int);
    *(&mut *send.pData.offset(crcpos as isize) as *mut byte as *mut uint) =
        crcValue;
    // send the datagram
    NET_SendPacket(NS_SERVER, MSG_GetNumBytesWritten(&mut send) as size_t,
                   MSG_GetData(&mut send) as *const libc::c_void, from);
}
/*
================
SV_Ack

================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Ack(mut from: netadr_t) {
    Con_Printf(b"ping %s\n\x00" as *const u8 as *const libc::c_char,
               NET_AdrToString(from));
}
/*
================
SV_Info

Responds with short info for broadcast scans
The second parameter should be the current protocol version number.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Info(mut from: netadr_t) {
    let mut string: [libc::c_char; 256] = [0; 256];
    let mut version: libc::c_int = 0;
    // ignore in single player
    if svs.maxclients == 1 as libc::c_int || svs.initialized as u64 == 0 {
        return
    }
    version = Q_atoi(Cmd_Argv(1 as libc::c_int));
    string[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if version != 49 as libc::c_int {
        Q_snprintf(string.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 256]>() as
                       libc::c_ulong,
                   b"%s: wrong version\n\x00" as *const u8 as
                       *const libc::c_char, hostname.string);
    } else {
        let mut i: libc::c_int = 0;
        let mut count: libc::c_int = 0 as libc::c_int;
        let mut havePassword: qboolean =
            if *sv_password.string == 0 {
                0 as libc::c_int
            } else { 1 as libc::c_int } as qboolean;
        i = 0 as libc::c_int;
        while i < svs.maxclients {
            if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
                   cs_connected as libc::c_int as libc::c_uint {
                count += 1
            }
            i += 1
        }
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"host\x00" as *const u8 as *const libc::c_char,
                            hostname.string, 256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"map\x00" as *const u8 as *const libc::c_char,
                            sv.name.as_mut_ptr(), 256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"dm\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               (*svgame.globals).deathmatch as libc::c_int),
                            256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"team\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               (*svgame.globals).teamplay as libc::c_int),
                            256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"coop\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               (*svgame.globals).coop as libc::c_int),
                            256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"numcl\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               count), 256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"maxcl\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               svs.maxclients), 256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"gamedir\x00" as *const u8 as
                                *const libc::c_char,
                            (*SI.GameInfo).gamefolder.as_mut_ptr(),
                            256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"password\x00" as *const u8 as
                                *const libc::c_char,
                            if havePassword as libc::c_uint != 0 {
                                b"1\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"0\x00" as *const u8 as *const libc::c_char
                            }, 256 as libc::c_int);
    }
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                           b"info\n%s\x00" as *const u8 as
                               *const libc::c_char, string.as_mut_ptr());
}
/*
================
SV_BuildNetAnswer

Responds with long info for local and broadcast requests
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BuildNetAnswer(mut from: netadr_t) {
    let mut string: [libc::c_char; 256] = [0; 256];
    let mut version: libc::c_int = 0;
    let mut context: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    // ignore in single player
    if svs.maxclients == 1 as libc::c_int || svs.initialized as u64 == 0 {
        return
    }
    version = Q_atoi(Cmd_Argv(1 as libc::c_int));
    context = Q_atoi(Cmd_Argv(2 as libc::c_int));
    type_0 = Q_atoi(Cmd_Argv(3 as libc::c_int));
    if version != 49 as libc::c_int {
        // handle the unsupported protocol
        string[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"neterror\x00" as *const u8 as
                                *const libc::c_char,
                            b"protocol\x00" as *const u8 as
                                *const libc::c_char, 256 as libc::c_int);
        // send error unsupported protocol
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                               b"netinfo %i %i %s\n\x00" as *const u8 as
                                   *const libc::c_char, context, type_0,
                               string.as_mut_ptr());
        return
    }
    if type_0 == 1 as libc::c_int {
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                               b"netinfo %i %i %s\n\x00" as *const u8 as
                                   *const libc::c_char, context, type_0,
                               b"\x00" as *const u8 as *const libc::c_char);
    } else if type_0 == 2 as libc::c_int {
        // send serverinfo
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                               b"netinfo %i %i %s\n\x00" as *const u8 as
                                   *const libc::c_char, context, type_0,
                               svs.serverinfo.as_mut_ptr());
    } else if type_0 == 3 as libc::c_int {
        string[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < svs.maxclients {
            if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
                   cs_connected as libc::c_int as libc::c_uint {
                let mut ed: *mut edict_t =
                    (*svs.clients.offset(i as isize)).edict;
                let mut time: libc::c_float =
                    (host.realtime -
                         (*svs.clients.offset(i as isize)).connection_started)
                        as libc::c_float;
                Q_strncat(string.as_mut_ptr(),
                          va(b"%c\\%s\\%i\\%f\\\x00" as *const u8 as
                                 *const libc::c_char, count,
                             (*svs.clients.offset(i as
                                                      isize)).name.as_mut_ptr(),
                             (*ed).v.frags as libc::c_int,
                             time as libc::c_double),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong);
                count += 1
            }
            i += 1
        }
        // send playernames
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                               b"netinfo %i %i %s\n\x00" as *const u8 as
                                   *const libc::c_char, context, type_0,
                               string.as_mut_ptr());
    } else if type_0 == 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < svs.maxclients {
            if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
                   cs_connected as libc::c_int as libc::c_uint {
                count += 1
            }
            i += 1
        }
        string[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"hostname\x00" as *const u8 as
                                *const libc::c_char, hostname.string,
                            256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"gamedir\x00" as *const u8 as
                                *const libc::c_char,
                            (*SI.GameInfo).gamefolder.as_mut_ptr(),
                            256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"current\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               count), 256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"max\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char,
                               svs.maxclients), 256 as libc::c_int);
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"map\x00" as *const u8 as *const libc::c_char,
                            sv.name.as_mut_ptr(), 256 as libc::c_int);
        // send serverinfo
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                               b"netinfo %i %i %s\n\x00" as *const u8 as
                                   *const libc::c_char, context, type_0,
                               string.as_mut_ptr());
    } else {
        string[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        Info_SetValueForKey(string.as_mut_ptr(),
                            b"neterror\x00" as *const u8 as
                                *const libc::c_char,
                            b"undefined\x00" as *const u8 as
                                *const libc::c_char, 256 as libc::c_int);
        // send error undefined request type
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                               b"netinfo %i %i %s\n\x00" as *const u8 as
                                   *const libc::c_char, context, type_0,
                               string.as_mut_ptr());
    };
}
/*
================
SV_Ping

Just responds with an acknowledgement
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Ping(mut from: netadr_t) {
    Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, from,
                           b"ack\x00" as *const u8 as *const libc::c_char);
}
/*
================
Rcon_Validate
================
*/
#[no_mangle]
pub unsafe extern "C" fn Rcon_Validate() -> qboolean {
    if if rcon_password.string.is_null() || *rcon_password.string == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    if Q_strncmp(Cmd_Argv(1 as libc::c_int), rcon_password.string,
                 99999 as libc::c_int) != 0 {
        return false_0
    }
    return true_0;
}
/*
===============
SV_RemoteCommand

A client issued an rcon command.
Shift down the remaining args
Redirect all printfs
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RemoteCommand(mut from: netadr_t,
                                          mut msg: *mut sizebuf_t) {
    static mut outputbuf: [libc::c_char; 2048] = [0; 2048];
    let mut remaining: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    Con_Printf(b"Rcon from %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
               NET_AdrToString(from),
               MSG_GetData(msg).offset(4 as libc::c_int as isize));
    Log_Printf(b"Rcon: \"%s\" from \"%s\"\n\x00" as *const u8 as
                   *const libc::c_char,
               MSG_GetData(msg).offset(4 as libc::c_int as isize),
               NET_AdrToString(from));
    SV_BeginRedirect(from, RD_PACKET, outputbuf.as_mut_ptr(),
                     (::std::mem::size_of::<[libc::c_char; 2048]>() as
                          libc::c_ulong).wrapping_sub(16 as libc::c_int as
                                                          libc::c_ulong),
                     ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                             netadr_t,
                                                                         _:
                                                                             libc::c_int,
                                                                         _:
                                                                             *mut libc::c_char)
                                                        -> ()>,
                                             *mut libc::c_void>(Some(SV_FlushRedirect
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  netadr_t,
                                                                                              _:
                                                                                                  libc::c_int,
                                                                                              _:
                                                                                                  *mut libc::c_char)
                                                                             ->
                                                                                 ())));
    if Rcon_Validate() as u64 != 0 {
        remaining[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        i = 2 as libc::c_int;
        while i < Cmd_Argc() {
            Q_strncat(remaining.as_mut_ptr(), Cmd_Argv(i),
                      99999 as libc::c_int as size_t);
            Q_strncat(remaining.as_mut_ptr(),
                      b" \x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int as size_t);
            i += 1
        }
        Cmd_ExecuteString(remaining.as_mut_ptr());
    } else {
        Con_Printf(b"^1Error:^7 Bad rcon_password.\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    SV_EndRedirect();
}
/*
===================
SV_CalcPing

recalc ping on current client
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CalcPing(mut cl: *mut sv_client_t)
 -> libc::c_int {
    let mut ping: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut back: libc::c_int = 0;
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    // bots don't have a real ping
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 ||
           (*cl).frames.is_null() {
        return 5 as libc::c_int
    }
    if SV_UPDATE_BACKUP <= 31 as libc::c_int {
        back = SV_UPDATE_BACKUP / 2 as libc::c_int;
        if back <= 0 as libc::c_int { return 0 as libc::c_int }
    } else { back = 16 as libc::c_int }
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < back {
        idx =
            (*cl).netchan.incoming_acknowledged.wrapping_add(!i as
                                                                 libc::c_uint)
                as libc::c_int;
        frame =
            &mut *(*cl).frames.offset((idx &
                                           SV_UPDATE_BACKUP -
                                               1 as libc::c_int) as isize) as
                *mut client_frame_t;
        if (*frame).ping_time > 0.0f32 {
            ping += (*frame).ping_time;
            count += 1
        }
        i += 1
    }
    if count > 0 as libc::c_int {
        return (ping / count as libc::c_float * 1000.0f32) as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
===================
SV_EstablishTimeBase

Finangles latency and the like.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EstablishTimeBase(mut cl: *mut sv_client_t,
                                              mut cmds: *mut usercmd_t,
                                              mut dropped: libc::c_int,
                                              mut numbackup: libc::c_int,
                                              mut numcmds: libc::c_int) {
    let mut runcmd_time: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut cmdnum: libc::c_int = dropped;
    if dropped < 24 as libc::c_int {
        while dropped > numbackup {
            runcmd_time = (*cl).lastcmd.msec as libc::c_double / 1000.0f64;
            dropped -= 1
        }
        while dropped > 0 as libc::c_int {
            cmdnum = dropped + numcmds - 1 as libc::c_int;
            runcmd_time +=
                (*cmds.offset(cmdnum as isize)).msec as libc::c_double /
                    1000.0f64;
            dropped -= 1
        }
    }
    i = numcmds - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        runcmd_time +=
            (*cmds.offset(i as isize)).msec as libc::c_int as libc::c_double /
                1000.0f64;
        i -= 1
    }
    (*cl).timebase = sv.time + sv.frametime as libc::c_double - runcmd_time;
}
/*
===================
SV_CalcClientTime

compute latency for client
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CalcClientTime(mut cl: *mut sv_client_t)
 -> libc::c_float {
    let mut minping: libc::c_float = 0.;
    let mut maxping: libc::c_float = 0.;
    let mut ping: libc::c_float = 0.0f32;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut backtrack: libc::c_int = 0;
    backtrack = sv_unlagsamples.value as libc::c_int;
    if backtrack < 1 as libc::c_int { backtrack = 1 as libc::c_int }
    if backtrack >=
           (if SV_UPDATE_BACKUP <= 16 as libc::c_int {
                SV_UPDATE_BACKUP
            } else { 16 as libc::c_int }) {
        backtrack =
            if SV_UPDATE_BACKUP <= 16 as libc::c_int {
                SV_UPDATE_BACKUP
            } else { 16 as libc::c_int }
    }
    if backtrack <= 0 as libc::c_int { return 0.0f32 }
    i = 0 as libc::c_int;
    while i < backtrack {
        let mut frame: *mut client_frame_t =
            &mut *(*cl).frames.offset(((SV_UPDATE_BACKUP - 1 as libc::c_int)
                                           as libc::c_uint &
                                           (*cl).netchan.incoming_acknowledged.wrapping_sub(i
                                                                                                as
                                                                                                libc::c_uint))
                                          as isize) as *mut client_frame_t;
        if !((*frame).ping_time <= 0.0f32) {
            ping += (*frame).ping_time;
            count += 1
        }
        i += 1
    }
    if count == 0 { return 0.0f32 }
    minping = 9999.0f32;
    maxping = -9999.0f32;
    ping /= count as libc::c_float;
    i = 0 as libc::c_int;
    while i <
              (if SV_UPDATE_BACKUP <= 4 as libc::c_int {
                   SV_UPDATE_BACKUP
               } else { 4 as libc::c_int }) {
        let mut frame_0: *mut client_frame_t =
            &mut *(*cl).frames.offset(((SV_UPDATE_BACKUP - 1 as libc::c_int)
                                           as libc::c_uint &
                                           (*cl).netchan.incoming_acknowledged.wrapping_sub(i
                                                                                                as
                                                                                                libc::c_uint))
                                          as isize) as *mut client_frame_t;
        if !((*frame_0).ping_time <= 0.0f32) {
            if (*frame_0).ping_time < minping {
                minping = (*frame_0).ping_time
            }
            if (*frame_0).ping_time > maxping {
                maxping = (*frame_0).ping_time
            }
        }
        i += 1
    }
    if maxping < minping || __tg_fabs(maxping - minping) <= 0.2f32 {
        return ping
    }
    return 0.0f32;
}
/*
===================
SV_FullClientUpdate

Writes all update values to a bitbuf
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FullClientUpdate(mut cl: *mut sv_client_t,
                                             mut msg: *mut sizebuf_t) {
    let mut info: [libc::c_char; 256] = [0; 256];
    let mut digest: [libc::c_char; 16] = [0; 16];
    let mut ctx: MD5Context_t =
        MD5Context_t{buf: [0; 4], bits: [0; 2], in_0: [0; 16],};
    let mut i: libc::c_int = 0;
    // process userinfo before updating
    SV_UserinfoChanged(cl);
    i = cl.wrapping_offset_from(svs.clients) as libc::c_long as libc::c_int;
    MSG_WriteCmdExt(msg, 13 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteUBitLong(msg, i as uint, 5 as libc::c_int);
    MSG_WriteLong(msg, (*cl).userid);
    if (*cl).name[0 as libc::c_int as usize] != 0 {
        MSG_WriteOneBit(msg, 1 as libc::c_int);
        Q_strncpy(info.as_mut_ptr(), (*cl).userinfo.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
        // remove server passwords, etc.
        Info_RemovePrefixedKeys(info.as_mut_ptr(),
                                '_' as i32 as libc::c_char);
        MSG_WriteString(msg, info.as_mut_ptr());
        MD5Init(&mut ctx);
        MD5Update(&mut ctx, (*cl).hashedcdkey.as_mut_ptr() as *mut byte,
                  ::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                      as uint);
        MD5Final(digest.as_mut_ptr() as *mut byte, &mut ctx);
        MSG_WriteBytes(msg, digest.as_mut_ptr() as *const libc::c_void,
                       ::std::mem::size_of::<[libc::c_char; 16]>() as
                           libc::c_ulong as libc::c_int);
    } else { MSG_WriteOneBit(msg, 0 as libc::c_int); };
}
/*
===================
SV_RefreshUserinfo

===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RefreshUserinfo() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint {
            (*cl).flags =
                (*cl).flags | (1 as libc::c_uint) << 0 as libc::c_int
        }
        i += 1;
        cl = cl.offset(1)
    };
}
/*
===================
SV_FullUpdateMovevars

this is send all movevars values when client connected
otherwise see code SV_UpdateMovevars()
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FullUpdateMovevars(mut cl: *mut sv_client_t,
                                               mut msg: *mut sizebuf_t) {
    let mut nullmovevars: movevars_t =
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
                   skyangle: 0.,};
    memset(&mut nullmovevars as *mut movevars_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
    MSG_WriteDeltaMovevars(msg, &mut nullmovevars, &mut svgame.movevars);
}
/*
===================
SV_ShouldUpdatePing

determine should we recalculate
ping times now
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ShouldUpdatePing(mut cl: *mut sv_client_t)
 -> qboolean {
    if (*cl).flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
        if host.realtime < (*cl).next_checkpingtime { return false_0 }
        (*cl).next_checkpingtime = host.realtime + 2.0f64;
        return true_0
    }
    // they are viewing the scoreboard.  Send them pings.
    return if (*cl).lastcmd.buttons as libc::c_uint &
                  (1 as libc::c_uint) << 15 as libc::c_int != 0 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
===================
SV_IsPlayerIndex

===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_IsPlayerIndex(mut idx: libc::c_int) -> qboolean {
    if idx > 0 as libc::c_int && idx <= svs.maxclients { return true_0 }
    return false_0;
}
/*
===================
SV_GetPlayerStats

This function and its static vars track some of the networking
conditions.  I haven't bothered to trace it beyond that, because
this fucntion sucks pretty badly.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetPlayerStats(mut cl: *mut sv_client_t,
                                           mut ping: *mut libc::c_int,
                                           mut packet_loss:
                                               *mut libc::c_int) {
    static mut last_ping: [libc::c_int; 32] = [0; 32];
    static mut last_loss: [libc::c_int; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    i = cl.wrapping_offset_from(svs.clients) as libc::c_long as libc::c_int;
    if host.realtime >= (*cl).next_checkpingtime {
        (*cl).next_checkpingtime = host.realtime + 2.0f64;
        last_ping[i as usize] = SV_CalcPing(cl);
        last_loss[i as usize] = (*cl).packet_loss
    }
    if !ping.is_null() { *ping = last_ping[i as usize] }
    if !packet_loss.is_null() { *packet_loss = last_loss[i as usize] };
}
/*
===========
PutClientInServer

Called when a player connects to a server or respawns in
a deathmatch.
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PutClientInServer(mut cl: *mut sv_client_t) {
    static mut msg_buf: [byte; 131584] =
        [0; 131584]; // MAX_INIT_MSG + some space
    let mut ent: *mut edict_t = (*cl).edict;
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    MSG_InitExt(&mut msg, b"Spawn\x00" as *const u8 as *const libc::c_char,
                msg_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131584]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    if sv.loadgame as u64 != 0 {
        // NOTE: we needs to setup angles on restore here
        if (*ent).v.fixangle == 1 as libc::c_int {
            MSG_WriteCmdExt(&mut msg, 10 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteVec3Angles(&mut msg, (*ent).v.angles.as_mut_ptr());
            (*ent).v.fixangle = 0 as libc::c_int
        }
        if svgame.dllFuncs.pfnParmsChangeLevel.is_some() {
            let mut levelData: SAVERESTOREDATA =
                SAVERESTOREDATA{pBaseData: 0 as *mut libc::c_char,
                                pCurrentData: 0 as *mut libc::c_char,
                                size: 0,
                                bufferSize: 0,
                                tokenSize: 0,
                                tokenCount: 0,
                                pTokens: 0 as *mut *mut libc::c_char,
                                currentIndex: 0,
                                tableCount: 0,
                                connectionCount: 0,
                                pTable: 0 as *mut ENTITYTABLE,
                                levelList:
                                    [LEVELLIST{mapName: [0; 32],
                                               landmarkName: [0; 32],
                                               pentLandmark:
                                                   0 as *mut edict_t,
                                               vecLandmarkOrigin: [0.; 3],};
                                        16],
                                fUseLandmark: 0,
                                szLandmarkName: [0; 20],
                                vecLandmarkOffset: [0.; 3],
                                time: 0.,
                                szCurrentMapName: [0; 32],};
            let mut name: string = [0; 256];
            let mut i: libc::c_int = 0;
            memset(&mut levelData as *mut SAVERESTOREDATA as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<SAVERESTOREDATA>() as libc::c_ulong);
            (*svgame.globals).pSaveData =
                &mut levelData as *mut SAVERESTOREDATA as *mut libc::c_void;
            svgame.dllFuncs.pfnParmsChangeLevel.expect("non-null function pointer")();
            MSG_WriteCmdExt(&mut msg, 33 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            Q_snprintf(name.as_mut_ptr(),
                       ::std::mem::size_of::<string>() as libc::c_ulong,
                       b"save/%s.HL2\x00" as *const u8 as *const libc::c_char,
                       sv.name.as_mut_ptr());
            COM_FixSlashes(name.as_mut_ptr());
            MSG_WriteString(&mut msg, name.as_mut_ptr());
            MSG_WriteByte(&mut msg, levelData.connectionCount);
            i = 0 as libc::c_int;
            while i < levelData.connectionCount {
                MSG_WriteString(&mut msg,
                                levelData.levelList[i as
                                                        usize].mapName.as_mut_ptr());
                i += 1
            }
            (*svgame.globals).pSaveData = 0 as *mut libc::c_void
        }
        // reset weaponanim
        MSG_WriteCmdExt(&mut msg, 35 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        MSG_WriteByte(&mut msg, 0 as libc::c_int);
        MSG_WriteByte(&mut msg, 0 as libc::c_int);
        sv.loadgame = false_0;
        sv.paused = false_0
    } else {
        if Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                                   b"hltv\x00" as *const u8 as
                                       *const libc::c_char)) != 0 {
            (*cl).flags =
                (*cl).flags | (1 as libc::c_uint) << 8 as libc::c_int
        }
        // reset pViewEntity
        SV_InitEdict(ent);
        if (*cl).flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            (*ent).v.flags =
                ((*ent).v.flags as libc::c_uint |
                     (1 as libc::c_uint) << 20 as libc::c_int) as libc::c_int
        } else { (*ent).v.flags = 0 as libc::c_int }
        (*ent).v.netname = SV_MakeString((*cl).name.as_mut_ptr());
        (*ent).v.colormap =
            ent.wrapping_offset_from(svgame.edicts) as libc::c_long as
                libc::c_int;
        (*svgame.globals).time = sv.time as libc::c_float;
        svgame.dllFuncs.pfnClientPutInServer.expect("non-null function pointer")(ent);
        if sv.background as u64 != 0 {
            // need to realloc private data for client
            // ???
            // fisrt entering
            // don't attack player in background mode
            (*ent).v.flags =
                ((*ent).v.flags as libc::c_uint |
                     ((1 as libc::c_uint) << 6 as libc::c_int |
                          (1 as libc::c_uint) << 7 as libc::c_int)) as
                    libc::c_int
        }
        (*cl).pViewEntity = 0 as *mut edict_t
    }
    if (*svgame.globals).cdAudioTrack != 0 {
        MSG_WriteCmdExt(&mut msg, 9 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        MSG_WriteString(&mut msg,
                        va(b"cd loop %3d\n\x00" as *const u8 as
                               *const libc::c_char,
                           (*svgame.globals).cdAudioTrack));
        (*svgame.globals).cdAudioTrack = 0 as libc::c_int
    }
    // enable dev-mode to prevent crash cheat-protecting from Invasion mod
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 6 as libc::c_int |
                (1 as libc::c_uint) << 7 as libc::c_int) != 0 &&
           Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                      b"invasion\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        SV_ExecuteClientCommand(cl,
                                b"test\n\x00" as *const u8 as
                                    *const libc::c_char);
    }
    // refresh the userinfo and movevars
	// NOTE: because movevars can be changed during the connection process
    (*cl).flags =
        (*cl).flags |
            ((1 as libc::c_uint) << 0 as libc::c_int |
                 (1 as libc::c_uint) << 1 as libc::c_int);
    // reset client times
    (*cl).connecttime = 0.0f64;
    (*cl).ignorecmdtime = 0.0f64;
    (*cl).cmdtime = 0.0f64;
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        let mut viewEnt: libc::c_int = 0;
        // NOTE: it's will be fragmented automatically in right ordering
        MSG_WriteBits(&mut msg,
                      MSG_GetData(&mut sv.signon) as *const libc::c_void,
                      MSG_GetNumBitsWritten(&mut sv.signon));
        if !(*cl).pViewEntity.is_null() {
            viewEnt =
                (*cl).pViewEntity.wrapping_offset_from(svgame.edicts) as
                    libc::c_long as libc::c_int
        } else {
            viewEnt =
                (*cl).edict.wrapping_offset_from(svgame.edicts) as
                    libc::c_long as libc::c_int
        }
        MSG_WriteCmdExt(&mut msg, 5 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        MSG_WriteWord(&mut msg, viewEnt);
        MSG_WriteCmdExt(&mut msg, 25 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        MSG_WriteByte(&mut msg, 1 as libc::c_int);
        if MSG_CheckOverflow(&mut msg) as u64 != 0 {
            if svs.maxclients == 1 as libc::c_int {
                Host_Error(b"spawn player: overflowed\n\x00" as *const u8 as
                               *const libc::c_char);
            } else { SV_DropClient(cl, false_0); }
        } else {
            // send initialization data
            Netchan_CreateFragments(&mut (*cl).netchan, &mut msg);
            Netchan_FragSend(&mut (*cl).netchan);
        }
    };
}
/*
===========
SV_UpdateClientView

Resend the client viewentity (used for demos)
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateClientView(mut cl: *mut sv_client_t) {
    let mut viewEnt: libc::c_int = 0;
    if !(*cl).pViewEntity.is_null() {
        viewEnt =
            (*cl).pViewEntity.wrapping_offset_from(svgame.edicts) as
                libc::c_long as libc::c_int
    } else {
        viewEnt =
            (*cl).edict.wrapping_offset_from(svgame.edicts) as libc::c_long as
                libc::c_int
    }
    MSG_WriteCmdExt(&mut (*cl).netchan.message, 5 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteWord(&mut (*cl).netchan.message, viewEnt);
}
/*
==================
SV_TogglePause
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TogglePause(mut msg: *const libc::c_char) {
    if sv.background as u64 != 0 { return }
    sv.paused =
        ::std::mem::transmute::<libc::c_uint,
                                qboolean>(sv.paused as libc::c_uint ^
                                              1 as libc::c_int as
                                                  libc::c_uint);
    if if msg.is_null() || *msg == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        SV_BroadcastPrintf(0 as *mut sv_client_s,
                           b"%s\x00" as *const u8 as *const libc::c_char,
                           msg);
    }
    // send notification to all clients
    MSG_WriteCmdExt(&mut sv.reliable_datagram, 24 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteOneBit(&mut sv.reliable_datagram, sv.paused as libc::c_int);
}
/*
================
SV_SendReconnect

Tell all the clients that the server is changing levels
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BuildReconnect(mut msg: *mut sizebuf_t) {
    MSG_WriteCmdExt(msg, 9 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteString(msg,
                    b"reconnect\n\x00" as *const u8 as *const libc::c_char);
}
/*
==================
SV_WriteDeltaDescriptionToClient

send delta communication encoding
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_WriteDeltaDescriptionToClient(mut msg:
                                                              *mut sizebuf_t) {
    let mut tableIndex: libc::c_int = 0;
    let mut fieldIndex: libc::c_int = 0;
    tableIndex = 0 as libc::c_int;
    while tableIndex < Delta_NumTables() {
        let mut dt: *mut delta_info_t = Delta_FindStructByIndex(tableIndex);
        fieldIndex = 0 as libc::c_int;
        while fieldIndex < (*dt).numFields {
            Delta_WriteTableField(msg, tableIndex,
                                  &mut *(*dt).pFields.offset(fieldIndex as
                                                                 isize));
            fieldIndex += 1
        }
        tableIndex += 1
    };
}
/*
================
SV_SendServerdata

Sends the first message from the server to a connected client.
This will be sent on the initial connection and upon each server load.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SendServerdata(mut msg: *mut sizebuf_t,
                                           mut cl: *mut sv_client_t) {
    let mut message: string = [0; 256];
    let mut i: libc::c_int = 0;
    // Only send this message to developer console, or multiplayer clients.
    if host_developer.value != 0. || svs.maxclients > 1 as libc::c_int {
        MSG_WriteCmdExt(msg, 8 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        Q_snprintf(message.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"\n^3BUILD %d SERVER (%i CRC)\nServer #%i\n\x00" as
                       *const u8 as *const libc::c_char, Q_buildnum(),
                   sv.progsCRC, svs.spawncount);
        MSG_WriteString(msg, message.as_mut_ptr());
    }
    // send the serverdata
    MSG_WriteCmdExt(msg, 11 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char); // Map Message
    MSG_WriteLong(msg, 49 as libc::c_int); // tell client about background map
    MSG_WriteLong(msg, svs.spawncount);
    MSG_WriteLong(msg, sv.worldmapCRC as libc::c_int);
    MSG_WriteByte(msg,
                  cl.wrapping_offset_from(svs.clients) as libc::c_long as
                      libc::c_int);
    MSG_WriteByte(msg, svs.maxclients);
    MSG_WriteWord(msg, (*SI.GameInfo).max_edicts);
    MSG_WriteWord(msg, 1024 as libc::c_int);
    MSG_WriteString(msg, sv.name.as_mut_ptr());
    MSG_WriteString(msg, SV_GetString((*svgame.edicts).v.message));
    MSG_WriteOneBit(msg, sv.background as libc::c_int);
    MSG_WriteString(msg, (*SI.GameInfo).gamefolder.as_mut_ptr());
    MSG_WriteLong(msg, host.features as libc::c_int);
    // send the player hulls
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int * 3 as libc::c_int {
        MSG_WriteChar(msg,
                      host.player_mins[(i / 3 as libc::c_int) as
                                           usize][(i % 3 as libc::c_int) as
                                                      usize] as libc::c_int);
        MSG_WriteChar(msg,
                      host.player_maxs[(i / 3 as libc::c_int) as
                                           usize][(i % 3 as libc::c_int) as
                                                      usize] as libc::c_int);
        i += 1
    }
    // send delta-encoding
    SV_WriteDeltaDescriptionToClient(msg);
    // now client know delta and can reading encoded messages
    SV_FullUpdateMovevars(cl, msg);
    // send the user messages registration
    i = 1 as libc::c_int; // unused style
    while i < 197 as libc::c_int &&
              svgame.msg[i as usize].name[0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        SV_SendUserReg(msg,
                       &mut *svgame.msg.as_mut_ptr().offset(i as
                                                                isize)); // stylenum
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !(sv.lightstyles[i as usize].pattern[0 as libc::c_int as usize] ==
                 0) {
            MSG_WriteCmdExt(msg, 12 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteByte(msg, i);
            MSG_WriteString(msg,
                            sv.lightstyles[i as usize].pattern.as_mut_ptr());
            MSG_WriteFloat(msg, sv.lightstyles[i as usize].time);
        }
        i += 1
    };
}
/*
============================================================

CLIENT COMMAND EXECUTION

============================================================
*/
/*
================
SV_New_f

Sends the first message from the server to a connected client.
This will be sent on the initial connection and upon each server load.
================
*/
unsafe extern "C" fn SV_New_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut msg_buf: [byte; 131072] = [0; 131072];
    let mut szRejectReason: [libc::c_char; 128] = [0; 128];
    let mut szAddress: [libc::c_char; 128] = [0; 128];
    let mut szName: [libc::c_char; 32] = [0; 32];
    let mut cur: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut i: libc::c_int = 0;
    MSG_InitExt(&mut msg, b"New\x00" as *const u8 as *const libc::c_char,
                msg_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    if (*cl).state as libc::c_uint !=
           cs_connected as libc::c_int as libc::c_uint {
        return false_0
    }
    // send the serverdata
    SV_SendServerdata(&mut msg, cl);
    // if the client was connected, tell the game .dll to disconnect him/her.
    if (*cl).state as libc::c_uint ==
           cs_spawned as libc::c_int as libc::c_uint && !(*cl).edict.is_null()
       {
        svgame.dllFuncs.pfnClientDisconnect.expect("non-null function pointer")((*cl).edict);
    }
    Q_strncpy(szName.as_mut_ptr(), (*cl).name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy(szAddress.as_mut_ptr(),
              NET_AdrToString((*cl).netchan.remote_address),
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong);
    Q_strncpy(szRejectReason.as_mut_ptr(),
              b"Connection rejected by game\n\x00" as *const u8 as
                  *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong);
    // Allow the game dll to reject this client.
    if svgame.dllFuncs.pfnClientConnect.expect("non-null function pointer")((*cl).edict,
                                                                            szName.as_mut_ptr(),
                                                                            szAddress.as_mut_ptr(),
                                                                            szRejectReason.as_mut_ptr())
           as u64 == 0 {
        // reject the connection and drop the client.
        SV_RejectConnection((*cl).netchan.remote_address,
                            b"%s\n\x00" as *const u8 as *const libc::c_char,
                            szRejectReason.as_mut_ptr());
        SV_DropClient(cl, false_0);
        return true_0
    }
    // server info string
    MSG_WriteCmdExt(&mut msg, 9 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut msg,
                    va(b"fullserverinfo \"%s\"\n\x00" as *const u8 as
                           *const libc::c_char, SV_Serverinfo()));
    // collect the info about all the players and send to me
    i = 0 as libc::c_int; // not in game yet
    cur = svs.clients;
    while i < svs.maxclients {
        if !((*cur).edict.is_null() ||
                 (*cur).state as libc::c_uint !=
                     cs_spawned as libc::c_int as libc::c_uint) {
            SV_FullClientUpdate(cur, &mut msg);
        }
        i += 1;
        cur = cur.offset(1)
    }
    // g-cont. why this is there?
    memset(&mut (*cl).lastcmd as *mut usercmd_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    Netchan_CreateFragments(&mut (*cl).netchan, &mut msg);
    Netchan_FragSend(&mut (*cl).netchan);
    return true_0;
}
/*
=================
SV_Disconnect_f

The client is going to disconnect, so remove the connection immediately
=================
*/
unsafe extern "C" fn SV_Disconnect_f(mut cl: *mut sv_client_t) -> qboolean {
    SV_DropClient(cl, false_0);
    return true_0;
}
/*
==================
SV_ShowServerinfo_f

Dumps the serverinfo info string
==================
*/
unsafe extern "C" fn SV_ShowServerinfo_f(mut cl: *mut sv_client_t)
 -> qboolean {
    Info_Print(svs.serverinfo.as_mut_ptr());
    return true_0;
}
/*
==================
SV_Pause_f
==================
*/
unsafe extern "C" fn SV_Pause_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut message: string = [0; 256];
    if UI_CreditsActive() as u64 != 0 { return true_0 }
    if (*sv_pausable).value == 0. {
        SV_ClientPrintf(cl,
                        b"Pause not allowed.\n\x00" as *const u8 as
                            *const libc::c_char);
        return true_0
    }
    if (*cl).flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
        SV_ClientPrintf(cl,
                        b"Spectators can not pause.\n\x00" as *const u8 as
                            *const libc::c_char);
        return true_0
    }
    if sv.paused as u64 == 0 {
        Q_snprintf(message.as_mut_ptr(), 256 as libc::c_int as size_t,
                   b"^2%s^7 paused the game\n\x00" as *const u8 as
                       *const libc::c_char, (*cl).name.as_mut_ptr());
    } else {
        Q_snprintf(message.as_mut_ptr(), 256 as libc::c_int as size_t,
                   b"^2%s^7 unpaused the game\n\x00" as *const u8 as
                       *const libc::c_char, (*cl).name.as_mut_ptr());
    }
    SV_TogglePause(message.as_mut_ptr());
    return true_0;
}
/*
=================
SV_UserinfoChanged

Pull specific info from a newly changed userinfo string
into a more C freindly form.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UserinfoChanged(mut cl: *mut sv_client_t) {
    let mut i: libc::c_int = 0;
    let mut dupc: libc::c_int = 1 as libc::c_int;
    let mut ent: *mut edict_t = (*cl).edict;
    let mut name1: string = [0; 256];
    let mut name2: string = [0; 256];
    let mut current: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    if if (*cl).userinfo.as_mut_ptr().is_null() ||
              *(*cl).userinfo.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"name\x00" as *const u8 as *const libc::c_char);
    Q_strncpy(name2.as_mut_ptr(), val,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_TrimSpace(name2.as_mut_ptr(), name1.as_mut_ptr());
    if Q_strnicmp(name1.as_mut_ptr(),
                  b"console\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                            b"name\x00" as *const u8 as *const libc::c_char,
                            b"unnamed\x00" as *const u8 as
                                *const libc::c_char, 256 as libc::c_int);
        val =
            Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                             b"name\x00" as *const u8 as *const libc::c_char)
    } else if Q_strncmp(name1.as_mut_ptr(), val, 99999 as libc::c_int) != 0 {
        Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                            b"name\x00" as *const u8 as *const libc::c_char,
                            name1.as_mut_ptr(), 256 as libc::c_int);
        val =
            Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                             b"name\x00" as *const u8 as *const libc::c_char)
    }
    if if *name1.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                            b"name\x00" as *const u8 as *const libc::c_char,
                            b"unnamed\x00" as *const u8 as
                                *const libc::c_char, 256 as libc::c_int);
        val =
            Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                             b"name\x00" as *const u8 as *const libc::c_char);
        Q_strncpy(name2.as_mut_ptr(),
                  b"unnamed\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        Q_strncpy(name1.as_mut_ptr(),
                  b"unnamed\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    }
    loop 
         // check to see if another user by the same name exists
         {
        i = 0 as libc::c_int;
        current = svs.clients;
        while i < svs.maxclients {
            if !(current == cl ||
                     (*current).state as libc::c_uint !=
                         cs_spawned as libc::c_int as libc::c_uint) {
                if Q_strnicmp((*current).name.as_mut_ptr(), val,
                              99999 as libc::c_int) == 0 {
                    break ;
                }
            }
            i += 1;
            current = current.offset(1)
        }
        if i != svs.maxclients {
            // dup name
            let fresh2 = dupc;
            dupc = dupc + 1;
            Q_snprintf(name2.as_mut_ptr(),
                       ::std::mem::size_of::<string>() as libc::c_ulong,
                       b"%s (%u)\x00" as *const u8 as *const libc::c_char,
                       name1.as_mut_ptr(), fresh2);
            Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                                b"name\x00" as *const u8 as
                                    *const libc::c_char, name2.as_mut_ptr(),
                                256 as libc::c_int);
            val =
                Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                                 b"name\x00" as *const u8 as
                                     *const libc::c_char);
            Q_strncpy((*cl).name.as_mut_ptr(), name2.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong);
        } else {
            if dupc == 1 as libc::c_int {
                // unchanged
                Q_strncpy((*cl).name.as_mut_ptr(), name1.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 32]>() as
                              libc::c_ulong);
            }
            break ;
        }
    }
    // rate command
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"rate\x00" as *const u8 as *const libc::c_char);
    if if val.is_null() || *val == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        (*cl).netchan.rate =
            if Q_atoi(val) as libc::c_float >= sv_minrate.value {
                if (Q_atoi(val) as libc::c_float) < sv_maxrate.value {
                    Q_atoi(val) as libc::c_float
                } else { sv_maxrate.value }
            } else { sv_minrate.value } as libc::c_double
    } else { (*cl).netchan.rate = 9999.0f32 as libc::c_double }
    // movement prediction
    if Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                               b"cl_nopred\x00" as *const u8 as
                                   *const libc::c_char)) != 0 {
        (*cl).flags = (*cl).flags & !((1 as libc::c_uint) << 4 as libc::c_int)
    } else {
        (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 4 as libc::c_int
    }
    // lag compensation
    if Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                               b"cl_lc\x00" as *const u8 as
                                   *const libc::c_char)) != 0 {
        (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 6 as libc::c_int
    } else {
        (*cl).flags = (*cl).flags & !((1 as libc::c_uint) << 6 as libc::c_int)
    }
    // weapon perdiction
    if Q_atoi(Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                               b"cl_lw\x00" as *const u8 as
                                   *const libc::c_char)) != 0 {
        (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 5 as libc::c_int
    } else {
        (*cl).flags = (*cl).flags & !((1 as libc::c_uint) << 5 as libc::c_int)
    }
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"cl_updaterate\x00" as *const u8 as
                             *const libc::c_char);
    if if val.is_null() || *val == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        if Q_atoi(val) != 0 as libc::c_int {
            (*cl).cl_updaterate =
                1.0f64 /
                    (if Q_atoi(val) as libc::c_float >= sv_minupdaterate.value
                        {
                         (if (Q_atoi(val) as libc::c_float) <
                                 sv_maxupdaterate.value {
                              Q_atoi(val) as libc::c_float
                          } else { sv_maxupdaterate.value })
                     } else { sv_minupdaterate.value }) as libc::c_double
        } else { (*cl).cl_updaterate = 0.0f64 }
    }
    // call prog code to allow overrides
    svgame.dllFuncs.pfnClientUserInfoChanged.expect("non-null function pointer")((*cl).edict,
                                                                                 (*cl).userinfo.as_mut_ptr());
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"name\x00" as *const u8 as *const libc::c_char);
    Q_strncpy((*cl).name.as_mut_ptr(), val,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    (*ent).v.netname = SV_MakeString((*cl).name.as_mut_ptr());
}
/*
==================
SV_UpdateUserinfo_f
==================
*/
unsafe extern "C" fn SV_UpdateUserinfo_f(mut cl: *mut sv_client_t)
 -> qboolean {
    Q_strncpy((*cl).userinfo.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<[libc::c_char; 256]>() as
                  libc::c_ulong); // needs for update client info
    if (*cl).state as libc::c_uint >=
           cs_connected as libc::c_int as libc::c_uint {
        (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 0 as libc::c_int
    }
    return true_0;
}
/*
==================
SV_SetInfo_f
==================
*/
unsafe extern "C" fn SV_SetInfo_f(mut cl: *mut sv_client_t) -> qboolean {
    Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                        Cmd_Argv(1 as libc::c_int),
                        Cmd_Argv(2 as libc::c_int),
                        256 as libc::c_int); // needs for update client info
    if (*cl).state as libc::c_uint >=
           cs_connected as libc::c_int as libc::c_uint {
        (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 0 as libc::c_int
    }
    return true_0;
}
/*
==================
SV_Noclip_f
==================
*/
unsafe extern "C" fn SV_Noclip_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut pEntity: *mut edict_t = (*cl).edict;
    if Cvar_VariableInteger(b"sv_cheats\x00" as *const u8 as
                                *const libc::c_char) == 0 ||
           sv.background as libc::c_uint != 0 {
        return true_0
    }
    if (*pEntity).v.movetype != 8 as libc::c_int {
        SV_ClientPrintf(cl,
                        b"noclip ON\n\x00" as *const u8 as
                            *const libc::c_char);
        (*pEntity).v.movetype = 8 as libc::c_int
    } else {
        SV_ClientPrintf(cl,
                        b"noclip OFF\n\x00" as *const u8 as
                            *const libc::c_char);
        (*pEntity).v.movetype = 3 as libc::c_int
    }
    return true_0;
}
/*
==================
SV_Godmode_f
==================
*/
unsafe extern "C" fn SV_Godmode_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut pEntity: *mut edict_t = (*cl).edict;
    if Cvar_VariableInteger(b"sv_cheats\x00" as *const u8 as
                                *const libc::c_char) == 0 ||
           sv.background as libc::c_uint != 0 {
        return true_0
    }
    (*pEntity).v.flags =
        ((*pEntity).v.flags as libc::c_uint ^
             (1 as libc::c_uint) << 6 as libc::c_int) as libc::c_int;
    if (*pEntity).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int == 0 {
        SV_ClientPrintf(cl,
                        b"godmode OFF\n\x00" as *const u8 as
                            *const libc::c_char);
    } else {
        SV_ClientPrintf(cl,
                        b"godmode ON\n\x00" as *const u8 as
                            *const libc::c_char);
    }
    return true_0;
}
/*
==================
SV_Notarget_f
==================
*/
unsafe extern "C" fn SV_Notarget_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut pEntity: *mut edict_t = (*cl).edict;
    if Cvar_VariableInteger(b"sv_cheats\x00" as *const u8 as
                                *const libc::c_char) == 0 ||
           sv.background as libc::c_uint != 0 {
        return true_0
    }
    (*pEntity).v.flags =
        ((*pEntity).v.flags as libc::c_uint ^
             (1 as libc::c_uint) << 7 as libc::c_int) as libc::c_int;
    if (*pEntity).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        SV_ClientPrintf(cl,
                        b"notarget OFF\n\x00" as *const u8 as
                            *const libc::c_char);
    } else {
        SV_ClientPrintf(cl,
                        b"notarget ON\n\x00" as *const u8 as
                            *const libc::c_char);
    }
    return true_0;
}
/*
==================
SV_Kill_f
==================
*/
unsafe extern "C" fn SV_Kill_f(mut cl: *mut sv_client_t) -> qboolean {
    if SV_CheckEdict((*cl).edict,
                     b"../engine/server/sv_client.c\x00" as *const u8 as
                         *const libc::c_char, 1894 as libc::c_int) as u64 == 0
       {
        return true_0
    }
    if (*cl).state as libc::c_uint !=
           cs_spawned as libc::c_int as libc::c_uint {
        SV_ClientPrintf(cl,
                        b"Can\'t suicide - not connected!\n\x00" as *const u8
                            as *const libc::c_char);
        return true_0
    }
    if (*(*cl).edict).v.health <= 0.0f32 {
        SV_ClientPrintf(cl,
                        b"Can\'t suicide - already dead!\n\x00" as *const u8
                            as *const libc::c_char);
        return true_0
    }
    svgame.dllFuncs.pfnClientKill.expect("non-null function pointer")((*cl).edict);
    return true_0;
}
/*
==================
SV_SendRes_f
==================
*/
unsafe extern "C" fn SV_SendRes_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut buffer: [byte; 131072] = [0; 131072];
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    if (*cl).state as libc::c_uint !=
           cs_connected as libc::c_int as libc::c_uint {
        return false_0
    }
    MSG_InitExt(&mut msg,
                b"SendResources\x00" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    if svs.maxclients > 1 as libc::c_int &&
           (*cl).flags & (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        return true_0
    }
    (*cl).flags = (*cl).flags | (1 as libc::c_uint) << 9 as libc::c_int;
    SV_SendResources(cl, &mut msg);
    Netchan_CreateFragments(&mut (*cl).netchan, &mut msg);
    Netchan_FragSend(&mut (*cl).netchan);
    return true_0;
}
/*
==================
SV_DownloadFile_f
==================
*/
unsafe extern "C" fn SV_DownloadFile_f(mut cl: *mut sv_client_t) -> qboolean {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if Cmd_Argc() < 2 as libc::c_int { return true_0 }
    name = Cmd_Argv(1 as libc::c_int);
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return true_0
    }
    if COM_IsSafeFileToDownload(name) as u64 == 0 ||
           sv_allow_download.value == 0. {
        SV_FailDownload(cl, name);
        return true_0
    }
    // g-cont. now we supports hot precache
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32 {
        if sv_send_resources.value != 0. {
            let mut i: libc::c_int = 0;
            // security: allow download only precached resources
            i = 0 as libc::c_int; // cut "sound/" off
            while i < sv.num_resources {
                let mut cmpname: *const libc::c_char = name;
                if sv.resources[i as usize].type_0 as libc::c_uint ==
                       t_sound as libc::c_int as libc::c_uint {
                    cmpname =
                        cmpname.offset((::std::mem::size_of::<[libc::c_char; 7]>()
                                            as
                                            libc::c_ulong).wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
                                           as isize)
                }
                if Q_strncmp(sv.resources[i as usize].szFileName.as_mut_ptr(),
                             cmpname, 64 as libc::c_int) == 0 {
                    break ;
                }
                i += 1
            }
            if i == sv.num_resources {
                SV_FailDownload(cl, name);
                return true_0
            }
            // also check the model textures
            if Q_strnicmp(COM_FileExtension(name),
                          b"mdl\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                if FS_FileExists(Mod_StudioTexName(name),
                                 false_0 as libc::c_int) > 0 as libc::c_int {
                    Netchan_CreateFileFragments(&mut (*cl).netchan,
                                                Mod_StudioTexName(name));
                }
            }
            if Netchan_CreateFileFragments(&mut (*cl).netchan, name) != 0 {
                Netchan_FragSend(&mut (*cl).netchan);
                return true_0
            }
        }
        SV_FailDownload(cl, name);
        return true_0
    }
    if Q_strlen(name) == 36 as libc::c_int as libc::c_ulong &&
           Q_strnicmp(name, b"!MD5\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int) == 0 && sv_send_logos.value != 0. {
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
        let mut md5: [byte; 32] = [0; 32];
        let mut pbuf: *mut byte = 0 as *mut byte;
        let mut size: libc::c_int = 0;
        memset(&mut custResource as *mut resource_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<resource_t>() as libc::c_ulong);
        COM_HexConvert(name.offset(4 as libc::c_int as isize),
                       32 as libc::c_int, md5.as_mut_ptr());
        if HPAK_ResourceForHash(b"custom.hpk\x00" as *const u8 as
                                    *const libc::c_char, md5.as_mut_ptr(),
                                &mut custResource) as u64 != 0 {
            if HPAK_GetDataPointer(b"custom.hpk\x00" as *const u8 as
                                       *const libc::c_char, &mut custResource,
                                   &mut pbuf, &mut size) as u64 != 0 {
                if size != 0 {
                    Netchan_CreateFileFragmentsFromBuffer(&mut (*cl).netchan,
                                                          name, pbuf, size);
                    Netchan_FragSend(&mut (*cl).netchan);
                    _Mem_Free(pbuf as *mut libc::c_void,
                              b"../engine/server/sv_client.c\x00" as *const u8
                                  as *const libc::c_char,
                              2025 as libc::c_int);
                }
            }
        }
    } else { SV_FailDownload(cl, name); }
    return true_0;
}
/*
==================
SV_Spawn_f
==================
*/
unsafe extern "C" fn SV_Spawn_f(mut cl: *mut sv_client_t) -> qboolean {
    if (*cl).state as libc::c_uint !=
           cs_connected as libc::c_int as libc::c_uint {
        return false_0
    }
    // handle the case of a level changing while a client was connecting
    if Q_atoi(Cmd_Argv(1 as libc::c_int)) != svs.spawncount {
        SV_New_f(cl);
        return true_0
    }
    SV_PutClientInServer(cl);
    // if we are paused, tell the clients
    if sv.paused as u64 != 0 {
        MSG_WriteCmdExt(&mut sv.reliable_datagram, 24 as libc::c_int,
                        NS_SERVER, 0 as *const libc::c_char);
        MSG_WriteByte(&mut sv.reliable_datagram, sv.paused as libc::c_int);
        SV_ClientPrintf(cl,
                        b"Server is paused.\n\x00" as *const u8 as
                            *const libc::c_char);
    }
    return true_0;
}
/*
==================
SV_Begin_f
==================
*/
unsafe extern "C" fn SV_Begin_f(mut cl: *mut sv_client_t) -> qboolean {
    if (*cl).state as libc::c_uint !=
           cs_connected as libc::c_int as libc::c_uint {
        return false_0
    }
    // now client is spawned
    (*cl).state = cs_spawned;
    return true_0;
}
/*
==================
SV_SendBuildInfo_f
==================
*/
unsafe extern "C" fn SV_SendBuildInfo_f(mut cl: *mut sv_client_t)
 -> qboolean {
    SV_ClientPrintf(cl,
                    b"Server running %s %s (build %i-%s, %s-%s)\n\x00" as
                        *const u8 as *const libc::c_char,
                    b"Xash3D FWGS\x00" as *const u8 as *const libc::c_char,
                    b"0.20\x00" as *const u8 as *const libc::c_char,
                    Q_buildnum(), Q_buildcommit(), Q_buildos(),
                    Q_buildarch());
    return true_0;
}
#[no_mangle]
pub static mut ucmds: [ucmd_t; 16] =
    unsafe {
        [{
             let mut init =
                 ucmd_s{name: b"new\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_New_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name: b"god\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_Godmode_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name: b"kill\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_Kill_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"begin\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_Begin_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"spawn\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_Spawn_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"pause\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_Pause_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"noclip\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_Noclip_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"setinfo\x00" as *const u8 as
                                *const libc::c_char,
                        func:
                            Some(SV_SetInfo_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"sendres\x00" as *const u8 as
                                *const libc::c_char,
                        func:
                            Some(SV_SendRes_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"notarget\x00" as *const u8 as
                                *const libc::c_char,
                        func:
                            Some(SV_Notarget_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name: b"info\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_ShowServerinfo_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"dlfile\x00" as *const u8 as *const libc::c_char,
                        func:
                            Some(SV_DownloadFile_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"disconnect\x00" as *const u8 as
                                *const libc::c_char,
                        func:
                            Some(SV_Disconnect_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"userinfo\x00" as *const u8 as
                                *const libc::c_char,
                        func:
                            Some(SV_UpdateUserinfo_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name:
                            b"_sv_build_info\x00" as *const u8 as
                                *const libc::c_char,
                        func:
                            Some(SV_SendBuildInfo_f as
                                     unsafe extern "C" fn(_: *mut sv_client_t)
                                         -> qboolean),};
             init
         },
         {
             let mut init =
                 ucmd_s{name: 0 as *const libc::c_char, func: None,};
             init
         }]
    };
/*
==================
SV_ExecuteUserCommand
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ExecuteClientCommand(mut cl: *mut sv_client_t,
                                                 mut s: *const libc::c_char) {
    let mut u: *mut ucmd_t = 0 as *mut ucmd_t;
    Cmd_TokenizeString(s);
    u = ucmds.as_mut_ptr();
    while !(*u).name.is_null() {
        if Q_strncmp(Cmd_Argv(0 as libc::c_int), (*u).name,
                     99999 as libc::c_int) == 0 {
            if (*u).func.expect("non-null function pointer")(cl) as u64 == 0 {
                Con_Printf(b"\'%s\' is not valid from the console\n\x00" as
                               *const u8 as *const libc::c_char, (*u).name);
            } else {
                Con_Reportf(b"ucmd->%s()\n\x00" as *const u8 as
                                *const libc::c_char, (*u).name);
            }
            break ;
        } else { u = u.offset(1) }
    }
    if (*u).name.is_null() &&
           sv.state as libc::c_uint ==
               ss_active as libc::c_int as libc::c_uint {
        // custom client commands
        svgame.dllFuncs.pfnClientCommand.expect("non-null function pointer")((*cl).edict);
        if Q_strncmp(Cmd_Argv(0 as libc::c_int),
                     b"fullupdate\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            // resend the ambient sounds for demo recording
            SV_RestartAmbientSounds();
            // resend all the decals for demo recording
            SV_RestartDecals();
            // resend all the static ents for demo recording
            SV_RestartStaticEnts();
            // resend the viewentity
            SV_UpdateClientView(cl);
        }
    };
}
/*
==================
SV_TSourceEngineQuery
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TSourceEngineQuery(mut from: netadr_t) {
    // A2S_INFO
    let mut answer: [libc::c_char; 1024] =
        *::std::mem::transmute::<&[u8; 1024],
                                 &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // Mark as connectionless
    let mut count: libc::c_int = 0 as libc::c_int; // Half-Life
    let mut bots: libc::c_int = 0 as libc::c_int; // mod
    let mut index: libc::c_int = 0;
    let mut buf: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    if !svs.clients.is_null() {
        index = 0 as libc::c_int;
        while index < svs.maxclients {
            if (*svs.clients.offset(index as isize)).state as libc::c_uint >=
                   cs_connected as libc::c_int as libc::c_uint {
                if (*svs.clients.offset(index as isize)).flags &
                       (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                    bots += 1
                } else { count += 1 }
            }
            index += 1
        }
    }
    MSG_InitExt(&mut buf,
                b"TSourceEngineQuery\x00" as *const u8 as *const libc::c_char,
                answer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int, -(1 as libc::c_int));
    MSG_WriteLong(&mut buf, -(1 as libc::c_int));
    MSG_WriteByte(&mut buf, 'm' as i32);
    MSG_WriteString(&mut buf, NET_AdrToString(net_local));
    MSG_WriteString(&mut buf, hostname.string);
    MSG_WriteString(&mut buf, sv.name.as_mut_ptr());
    MSG_WriteString(&mut buf, (*SI.GameInfo).gamefolder.as_mut_ptr());
    MSG_WriteString(&mut buf, (*SI.GameInfo).title.as_mut_ptr());
    MSG_WriteByte(&mut buf, count);
    MSG_WriteByte(&mut buf, svs.maxclients);
    MSG_WriteByte(&mut buf, 49 as libc::c_int);
    MSG_WriteByte(&mut buf,
                  if host.type_0 ==
                         HOST_DEDICATED as libc::c_int as libc::c_uint {
                      'D' as i32
                  } else { 'L' as i32 });
    MSG_WriteByte(&mut buf, 'L' as i32);
    if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                  b"valve\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        MSG_WriteByte(&mut buf, 1 as libc::c_int);
        MSG_WriteString(&mut buf, (*SI.GameInfo).game_url.as_mut_ptr());
        MSG_WriteString(&mut buf, (*SI.GameInfo).update_url.as_mut_ptr());
        MSG_WriteByte(&mut buf, 0 as libc::c_int);
        MSG_WriteLong(&mut buf, (*SI.GameInfo).version as libc::c_int);
        MSG_WriteLong(&mut buf, (*SI.GameInfo).size as libc::c_int);
        if (*SI.GameInfo).gamemode == 2 as libc::c_int {
            // Own DLL
            MSG_WriteByte(&mut buf, 1 as libc::c_int); // multiplayer_only
        } else {
            MSG_WriteByte(&mut buf, 0 as libc::c_int); // Half-Life DLL
        } // unsecure
        if !Q_strstr((*SI.GameInfo).game_dll.as_mut_ptr(),
                     b"hl.\x00" as *const u8 as *const libc::c_char).is_null()
           {
            MSG_WriteByte(&mut buf, 0 as libc::c_int);
        } else { MSG_WriteByte(&mut buf, 1 as libc::c_int); }
    } else { MSG_WriteByte(&mut buf, 0 as libc::c_int); }
    MSG_WriteByte(&mut buf, (*SI.GameInfo).secure as libc::c_int);
    MSG_WriteByte(&mut buf, bots);
    NET_SendPacket(NS_SERVER, MSG_GetNumBytesWritten(&mut buf) as size_t,
                   MSG_GetData(&mut buf) as *const libc::c_void, from);
}
/*
=================
SV_ConnectionlessPacket

A connectionless packet has four leading 0xff
characters to distinguish it from a game channel.
Clients that are in the game can still send
connectionless packets.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ConnectionlessPacket(mut from: netadr_t,
                                                 mut msg: *mut sizebuf_t) {
    let mut args: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pcmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut len: libc::c_int =
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as
            libc::c_int;
    // prevent flooding from banned address
    if SV_CheckIP(&mut from) as u64 != 0 { return } // skip the -1 marker
    MSG_Clear(msg); // A2A_PING
    MSG_ReadLong(msg);
    args = MSG_ReadStringExt(msg, true_0);
    Cmd_TokenizeString(args);
    pcmd = Cmd_Argv(0 as libc::c_int);
    Con_Reportf(b"SV_ConnectionlessPacket: %s : %s\n\x00" as *const u8 as
                    *const libc::c_char, NET_AdrToString(from), pcmd);
    if Q_strncmp(pcmd, b"ping\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        SV_Ping(from);
    } else if Q_strncmp(pcmd, b"ack\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_Ack(from);
    } else if Q_strncmp(pcmd, b"info\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_Info(from);
    } else if Q_strncmp(pcmd,
                        b"bandwidth\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_TestBandWidth(from);
    } else if Q_strncmp(pcmd,
                        b"getchallenge\x00" as *const u8 as
                            *const libc::c_char, 99999 as libc::c_int) == 0 {
        SV_GetChallenge(from);
    } else if Q_strncmp(pcmd,
                        b"connect\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_ConnectClient(from);
    } else if Q_strncmp(pcmd, b"rcon\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_RemoteCommand(from, msg);
    } else if Q_strncmp(pcmd,
                        b"netinfo\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_BuildNetAnswer(from);
    } else if Q_strncmp(pcmd, b"s\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_AddToMaster(from, msg);
    } else if Q_strncmp(pcmd,
                        b"TSource\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        SV_TSourceEngineQuery(from);
    } else if Q_strncmp(pcmd, b"i\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        NET_SendPacket(NS_SERVER, 5 as libc::c_int as size_t,
                       b"\xff\xff\xff\xffj\x00" as *const u8 as
                           *const libc::c_char as *const libc::c_void, from);
    } else if svgame.dllFuncs.pfnConnectionlessPacket.expect("non-null function pointer")(&mut from,
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
    };
}
/*
==================
SV_ParseClientMove

The message usually contains all the movement commands
that were in the last three packets, so that the information
in dropped packets can be recovered.

On very fast clients, there may be multiple usercmd packed into
each of the backup packets.
==================
*/
unsafe extern "C" fn SV_ParseClientMove(mut cl: *mut sv_client_t,
                                        mut msg: *mut sizebuf_t) {
    let mut frame: *mut client_frame_t =
        0 as
            *mut client_frame_t; // first cmd are starting from null-compressed usercmd_t
    let mut key: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut checksum1: libc::c_int = 0;
    let mut checksum2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut numbackup: libc::c_int = 0;
    let mut totalcmds: libc::c_int = 0;
    let mut numcmds: libc::c_int = 0;
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
    let mut to: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut from: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut cmds: [usercmd_t; 64] =
        [usercmd_t{lerp_msec: 0,
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
                   impact_position: [0.; 3],}; 64];
    let mut packet_loss: libc::c_float = 0.;
    let mut player: *mut edict_t = 0 as *mut edict_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    player = (*cl).edict;
    frame =
        &mut *(*cl).frames.offset(((*cl).netchan.incoming_acknowledged &
                                       (SV_UPDATE_BACKUP - 1 as libc::c_int)
                                           as libc::c_uint) as isize) as
            *mut client_frame_t;
    memset(&mut nullcmd as *mut usercmd_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    memset(cmds.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[usercmd_t; 64]>() as libc::c_ulong);
    key = MSG_GetRealBytesWritten(msg);
    checksum1 = MSG_ReadByte(msg);
    packet_loss = MSG_ReadByte(msg) as libc::c_float;
    numbackup = MSG_ReadByte(msg);
    numcmds = MSG_ReadByte(msg);
    totalcmds = numcmds + numbackup;
    net_drop -= numcmds - 1 as libc::c_int;
    if totalcmds < 0 as libc::c_int ||
           totalcmds >= 64 as libc::c_int - 1 as libc::c_int {
        Con_Reportf(b"^1Error:^7 SV_ParseClientMove: %s sending too many commands %i\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*cl).name.as_mut_ptr(), totalcmds);
        SV_DropClient(cl, false_0);
        return
    }
    from = &mut nullcmd;
    i = totalcmds - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        to = &mut *cmds.as_mut_ptr().offset(i as isize) as *mut usercmd_t;
        MSG_ReadDeltaUsercmd(msg, from, to);
        from = to;
        i -= 1
        // get new baseline
    }
    if (*cl).state as libc::c_uint !=
           cs_spawned as libc::c_int as libc::c_uint {
        return
    }
    // if the checksum fails, ignore the rest of the packet
    size = MSG_GetRealBytesWritten(msg) - key - 1 as libc::c_int;
    checksum2 =
        CRC32_BlockSequence((*msg).pData.offset(key as
                                                    isize).offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                            size,
                            (*cl).netchan.incoming_sequence as libc::c_int) as
            libc::c_int;
    if checksum2 != checksum1 {
        Con_Reportf(b"^1Error:^7 SV_UserMove: failed command checksum for %s (%d != %d)\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*cl).name.as_mut_ptr(), checksum2, checksum1);
        return
    }
    (*cl).packet_loss = packet_loss as libc::c_int;
    // freeze player for some reasons if loadgame was executed
    if host.game.loadGame as u64 != 0 { return }
    // check for pause or frozen
    if sv.paused as libc::c_uint != 0 || CL_IsInGame() as u64 == 0 ||
           SV_PlayerIsFrozen(player) as libc::c_uint != 0 {
        i = 0 as libc::c_int;
        while i < numcmds {
            cmds[i as usize].msec = 0 as libc::c_int as byte;
            cmds[i as usize].forwardmove = 0 as libc::c_int as libc::c_float;
            cmds[i as usize].sidemove = 0 as libc::c_int as libc::c_float;
            cmds[i as usize].upmove = 0 as libc::c_int as libc::c_float;
            cmds[i as usize].buttons = 0 as libc::c_int as libc::c_ushort;
            if SV_PlayerIsFrozen(player) as u64 != 0 {
                cmds[i as usize].impulse = 0 as libc::c_int as byte
            }
            (*player).v.v_angle[0 as libc::c_int as usize] =
                cmds[i as usize].viewangles[0 as libc::c_int as usize];
            (*player).v.v_angle[1 as libc::c_int as usize] =
                cmds[i as usize].viewangles[1 as libc::c_int as usize];
            (*player).v.v_angle[2 as libc::c_int as usize] =
                cmds[i as usize].viewangles[2 as libc::c_int as usize];
            i += 1
        }
        net_drop = 0 as libc::c_int
    } else if (*player).v.fixangle == 0 {
        (*player).v.v_angle[0 as libc::c_int as usize] =
            cmds[0 as libc::c_int as
                     usize].viewangles[0 as libc::c_int as usize];
        (*player).v.v_angle[1 as libc::c_int as usize] =
            cmds[0 as libc::c_int as
                     usize].viewangles[1 as libc::c_int as usize];
        (*player).v.v_angle[2 as libc::c_int as usize] =
            cmds[0 as libc::c_int as
                     usize].viewangles[2 as libc::c_int as usize]
    }
    SV_EstablishTimeBase(cl, cmds.as_mut_ptr(), net_drop, numbackup, numcmds);
    if net_drop < 24 as libc::c_int {
        while net_drop > numbackup {
            SV_RunCmd(cl, &mut (*cl).lastcmd, 0 as libc::c_int);
            net_drop -= 1
        }
        while net_drop > 0 as libc::c_int {
            i = numcmds + net_drop - 1 as libc::c_int;
            SV_RunCmd(cl, &mut *cmds.as_mut_ptr().offset(i as isize),
                      (*cl).netchan.incoming_sequence.wrapping_sub(i as
                                                                       libc::c_uint)
                          as libc::c_int);
            net_drop -= 1
        }
    }
    i = numcmds - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        SV_RunCmd(cl, &mut *cmds.as_mut_ptr().offset(i as isize),
                  (*cl).netchan.incoming_sequence.wrapping_sub(i as
                                                                   libc::c_uint)
                      as libc::c_int);
        i -= 1
    }
    (*cl).lastcmd = cmds[0 as libc::c_int as usize];
    // adjust latency time by 1/2 last client frame since
	// the message probably arrived 1/2 through client's frame loop
    (*frame).ping_time -=
        (*cl).lastcmd.msec as libc::c_int as libc::c_float * 0.5f32 /
            1000.0f32;
    (*frame).ping_time =
        if 0.0f32 > (*frame).ping_time { 0.0f32 } else { (*frame).ping_time };
    model = SV_ModelHandle((*player).v.modelindex);
    if !model.is_null() &&
           (*model).type_0 as libc::c_int == mod_studio as libc::c_int {
        // g-cont. yes we using svgame.globals->time instead of sv.time
        if (*player).v.animtime > (*svgame.globals).time + sv.frametime {
            (*player).v.animtime = (*svgame.globals).time + sv.frametime
        }
    };
}
/*
===================
SV_ParseResourceList

Parse resource list
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ParseResourceList(mut cl: *mut sv_client_t,
                                              mut msg: *mut sizebuf_t) {
    let mut totalsize: libc::c_int = 0;
    let mut resource: *mut resource_t = 0 as *mut resource_t;
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut ri: resourceinfo_t =
        resourceinfo_t{info: [_resourceinfo_t{size: 0,}; 8],};
    total = MSG_ReadShort(msg);
    SV_ClearResourceList(&mut (*cl).resourcesneeded);
    SV_ClearResourceList(&mut (*cl).resourcesonhand);
    i = 0 as libc::c_int;
    while i < total {
        resource =
            _Mem_Alloc(host.mempool,
                       ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/server/sv_client.c\x00" as *const u8 as
                           *const libc::c_char, 2432 as libc::c_int) as
                *mut resource_t;
        Q_strncpy((*resource).szFileName.as_mut_ptr(),
                  MSG_ReadStringExt(msg, false_0),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        (*resource).type_0 = MSG_ReadByte(msg) as resourcetype_t;
        (*resource).nIndex = MSG_ReadShort(msg);
        (*resource).nDownloadSize = MSG_ReadLong(msg);
        (*resource).ucFlags = MSG_ReadByte(msg) as libc::c_uchar;
        (*resource).pNext = 0 as *mut resource_s;
        (*resource).pPrev = 0 as *mut resource_s;
        (*resource).ucFlags =
            ((*resource).ucFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as libc::c_uchar;
        if (*resource).ucFlags as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int != 0 {
            MSG_ReadBytes(msg,
                          (*resource).rgucMD5_hash.as_mut_ptr() as
                              *mut libc::c_void, 16 as libc::c_int);
        }
        if (*resource).type_0 as libc::c_uint >
               t_world as libc::c_int as libc::c_uint ||
               (*resource).nDownloadSize >
                   1024 as libc::c_int * 1024 as libc::c_int *
                       1024 as libc::c_int {
            SV_ClearResourceList(&mut (*cl).resourcesneeded);
            SV_ClearResourceList(&mut (*cl).resourcesonhand);
            return
        }
        SV_AddToResourceList(resource, &mut (*cl).resourcesneeded);
        i += 1
    }
    totalsize = COM_SizeofResourceList(&mut (*cl).resourcesneeded, &mut ri);
    if totalsize != 0 as libc::c_int && sv_allow_upload.value != 0. {
        Con_DPrintf(b"Verifying and uploading resources...\n\x00" as *const u8
                        as *const libc::c_char);
        if totalsize != 0 as libc::c_int {
            Con_DPrintf(b"Custom resources total %.2fK\n\x00" as *const u8 as
                            *const libc::c_char,
                        totalsize as libc::c_double / 1024.0f64);
            if ri.info[t_model as libc::c_int as usize].size !=
                   0 as libc::c_int {
                Con_DPrintf(b"  Models:  %.2fK\n\x00" as *const u8 as
                                *const libc::c_char,
                            ri.info[t_model as libc::c_int as usize].size as
                                libc::c_double / 1024.0f64);
            }
            if ri.info[t_sound as libc::c_int as usize].size !=
                   0 as libc::c_int {
                Con_DPrintf(b"  Sounds:  %.2fK\n\x00" as *const u8 as
                                *const libc::c_char,
                            ri.info[t_sound as libc::c_int as usize].size as
                                libc::c_double / 1024.0f64);
            }
            if ri.info[t_decal as libc::c_int as usize].size !=
                   0 as libc::c_int {
                Con_DPrintf(b"  Decals:  %.2fK\n\x00" as *const u8 as
                                *const libc::c_char,
                            ri.info[t_decal as libc::c_int as usize].size as
                                libc::c_double / 1024.0f64);
            }
            if ri.info[t_skin as libc::c_int as usize].size !=
                   0 as libc::c_int {
                Con_DPrintf(b"  Skins :  %.2fK\n\x00" as *const u8 as
                                *const libc::c_char,
                            ri.info[t_skin as libc::c_int as usize].size as
                                libc::c_double / 1024.0f64);
            }
            if ri.info[t_generic as libc::c_int as usize].size !=
                   0 as libc::c_int {
                Con_DPrintf(b"  Generic :  %.2fK\n\x00" as *const u8 as
                                *const libc::c_char,
                            ri.info[t_generic as libc::c_int as usize].size as
                                libc::c_double / 1024.0f64);
            }
            if ri.info[t_eventscript as libc::c_int as usize].size !=
                   0 as libc::c_int {
                Con_DPrintf(b"  Events  :  %.2fK\n\x00" as *const u8 as
                                *const libc::c_char,
                            ri.info[t_eventscript as libc::c_int as
                                        usize].size as libc::c_double /
                                1024.0f64);
            }
            Con_DPrintf(b"----------------------\n\x00" as *const u8 as
                            *const libc::c_char);
        }
        totalsize = SV_EstimateNeededResources(cl);
        if totalsize as libc::c_float >
               sv_uploadmax.value * 1024 as libc::c_int as libc::c_float *
                   1024 as libc::c_int as libc::c_float {
            SV_ClearResourceList(&mut (*cl).resourcesneeded);
            SV_ClearResourceList(&mut (*cl).resourcesonhand);
            return
        }
        Con_DPrintf(b"resources to request: %s\n\x00" as *const u8 as
                        *const libc::c_char,
                    Q_pretifymem(totalsize as libc::c_float,
                                 2 as libc::c_int));
    }
    (*cl).upstate = us_processing;
    SV_BatchUploadRequest(cl);
}
/*
===================
SV_ParseCvarValue

Parse a requested value from client cvar
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ParseCvarValue(mut cl: *mut sv_client_t,
                                           mut msg: *mut sizebuf_t) {
    let mut value: *const libc::c_char = MSG_ReadStringExt(msg, false_0);
    if svgame.dllFuncs2.pfnCvarValue.is_some() {
        svgame.dllFuncs2.pfnCvarValue.expect("non-null function pointer")((*cl).edict,
                                                                          value);
    }
    Con_Reportf(b"Cvar query response: name:%s, value:%s\n\x00" as *const u8
                    as *const libc::c_char, (*cl).name.as_mut_ptr(), value);
}
/*
===================
SV_ParseCvarValue2

Parse a requested value from client cvar
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ParseCvarValue2(mut cl: *mut sv_client_t,
                                            mut msg: *mut sizebuf_t) {
    let mut name: string = [0; 256];
    let mut value: string = [0; 256];
    let mut requestID: libc::c_int = MSG_ReadLong(msg);
    Q_strncpy(name.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              99999 as libc::c_int as size_t);
    Q_strncpy(value.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              99999 as libc::c_int as size_t);
    if svgame.dllFuncs2.pfnCvarValue2.is_some() {
        svgame.dllFuncs2.pfnCvarValue2.expect("non-null function pointer")((*cl).edict,
                                                                           requestID,
                                                                           name.as_mut_ptr(),
                                                                           value.as_mut_ptr());
    }
    Con_Reportf(b"Cvar query response: name:%s, request ID %d, cvar:%s, value:%s\n\x00"
                    as *const u8 as *const libc::c_char,
                (*cl).name.as_mut_ptr(), requestID, name.as_mut_ptr(),
                value.as_mut_ptr());
}
/*
===================
SV_ExecuteClientMessage

Parse a client packet
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ExecuteClientMessage(mut cl: *mut sv_client_t,
                                                 mut msg: *mut sizebuf_t) {
    let mut move_issued: qboolean = false_0;
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut c: libc::c_int = 0;
    if (*cl).frames.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_client.c\x00" as *const u8 as
                      *const libc::c_char, 2549 as libc::c_int);
    }
    // calc ping time
    frame =
        &mut *(*cl).frames.offset(((*cl).netchan.incoming_acknowledged &
                                       (SV_UPDATE_BACKUP - 1 as libc::c_int)
                                           as libc::c_uint) as isize) as
            *mut client_frame_t;
    // ping time doesn't factor in message interval, either
    (*frame).ping_time =
        (host.realtime - (*frame).senttime - (*cl).cl_updaterate) as
            libc::c_float;
    // on first frame ( no senttime ) don't skew ping
    if (*frame).senttime == 0.0f32 as libc::c_double {
        (*frame).ping_time = 0.0f32
    }
    // don't skew ping based on signon stuff either
    if host.realtime - (*cl).connection_started < 2.0f32 as libc::c_double &&
           (*frame).ping_time > 0.0f32 {
        (*frame).ping_time = 0.0f32
    } // no delta unless requested
    (*cl).latency = SV_CalcClientTime(cl);
    (*cl).delta_sequence = -(1 as libc::c_int);
    // read optional clientCommand strings
    while (*cl).state as libc::c_uint !=
              cs_zombie as libc::c_int as libc::c_uint {
        if MSG_CheckOverflow(msg) as u64 != 0 {
            Con_DPrintf(b"^1Error:^7 incoming overflow for %s\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*cl).name.as_mut_ptr());
            SV_DropClient(cl, false_0);
            return
        }
        // end of message
        if MSG_GetNumBitsLeft(msg) < 8 as libc::c_int {
            break ; // someone is trying to cheat...
        } // disconnect command
        c = MSG_ReadCmd(msg, NS_CLIENT);
        match c {
            1 => { }
            4 => { (*cl).delta_sequence = MSG_ReadByte(msg) }
            2 => {
                if move_issued as u64 != 0 { return }
                move_issued = true_0;
                SV_ParseClientMove(cl, msg);
            }
            3 => {
                SV_ExecuteClientCommand(cl, MSG_ReadStringExt(msg, false_0));
                if (*cl).state as libc::c_uint ==
                       cs_zombie as libc::c_int as libc::c_uint {
                    return
                }
            }
            5 => { SV_ParseResourceList(cl, msg); }
            7 => { SV_ParseConsistencyResponse(cl, msg); }
            9 => { SV_ParseCvarValue(cl, msg); }
            10 => { SV_ParseCvarValue2(cl, msg); }
            _ => {
                Con_DPrintf(b"^1Error:^7 %s: clc_bad\n\x00" as *const u8 as
                                *const libc::c_char, (*cl).name.as_mut_ptr());
                SV_DropClient(cl, false_0);
                return
            }
        }
    };
}
