#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, const_transmute,
           extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn UI_ShowConnectionWarning();
    #[no_mangle]
    fn LZSS_Decompress(pInput: *const byte, pOutput: *mut byte) -> uint;
    #[no_mangle]
    fn LZSS_Compress(pInput: *mut byte, inputLength: libc::c_int,
                     pOutputSize: *mut uint) -> *mut byte;
    #[no_mangle]
    fn LZSS_GetActualSize(source: *const byte) -> uint;
    #[no_mangle]
    fn LZSS_IsCompressed(source: *const byte) -> qboolean;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_ReplaceExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut scr_download: *mut convar_t;
    #[no_mangle]
    fn NET_IsActive() -> qboolean;
    #[no_mangle]
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_SendPacket(sock: netsrc_t, length: size_t,
                      data: *const libc::c_void, to: netadr_t);
    #[no_mangle]
    fn NET_SendPacketEx(sock: netsrc_t, length: size_t,
                        data: *const libc::c_void, to: netadr_t,
                        splitsize: size_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
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
    fn FS_WriteFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: fs_offset_t) -> qboolean;
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
    fn FS_FileSize(filename: *const libc::c_char, gamedironly: qboolean)
     -> fs_offset_t;
    #[no_mangle]
    fn FS_FileTime(filename: *const libc::c_char, gamedironly: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn COM_IsSafeFileToDownload(filename: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_IsPlaybackDemo() -> qboolean;
    #[no_mangle]
    static mut sv_lan: convar_t;
    #[no_mangle]
    static mut sv_lan_rate: convar_t;
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn MSG_InitMasks();
    #[no_mangle]
    fn MSG_SeekToBit(sb: *mut sizebuf_t, bitPos: libc::c_int,
                     whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn MSG_ExciseBits(sb: *mut sizebuf_t, startbit: libc::c_int,
                      bitstoremove: libc::c_int);
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_StartWriting(sb: *mut sizebuf_t, pData: *mut libc::c_void,
                        nBytes: libc::c_int, iStartBit: libc::c_int,
                        nBits: libc::c_int);
    #[no_mangle]
    fn MSG_Clear(sb: *mut sizebuf_t);
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
    fn MSG_WriteBytes(sb: *mut sizebuf_t, pBuf: *const libc::c_void,
                      nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_WriteLong(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteWord(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_ReadBits(sb: *mut sizebuf_t, pOutData: *mut libc::c_void,
                    nBits: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
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
pub type host_parm_t = host_parm_s;
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
pub struct netsplit_packet_s {
    pub signature: uint32_t,
    pub length: uint32_t,
    pub part: uint32_t,
    pub id: uint32_t,
    pub count: byte,
    pub index: byte,
    pub data: [byte; 131054],
}
pub type netsplit_packet_t = netsplit_packet_s;
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
#[inline]
unsafe extern "C" fn BitByte(mut bits: libc::c_int) -> libc::c_int {
    return (bits + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int *
               8 as libc::c_int >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return BitByte((*sb).iCurBit);
}
#[inline]
unsafe extern "C" fn MSG_GetMaxBytes(mut sb: *mut sizebuf_t) -> libc::c_int {
    return (*sb).nDataBits >> 3 as libc::c_int;
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
packet header ( size in bits )
-------------
31	sequence
1	does this message contain a reliable payload
31	acknowledge sequence
1	acknowledge receipt of even/odd message
16	qport

The remote connection never knows if it missed a reliable message, the
local side detects that it has been dropped by seeing a sequence acknowledge
higher thatn the last reliable sequence, but without the correct evon/odd
bit for the reliable set.

If the sender notices that a reliable message has been dropped, it will be
retransmitted.  It will not be retransmitted again until a message after
the retransmit has been acknowledged and the reliable still failed to get there.

if the sequence number is -1, the packet should be handled without a netcon

The reliable message can be added to at any time by doing
MSG_Write* (&netchan->message, <data>).

If the message buffer is overflowed, either by a single message, or by
multiple frames worth piling up while the last reliable transmit goes
unacknowledged, the netchan signals a fatal error.

Reliable messages are allways placed first in a packet, then the unreliable
message is included if there is sufficient room.

To the receiver, there is no distinction between the reliable and unreliable
parts of the message, they are just processed out as a single larger message.

Illogical packet sequence numbers cause the packet to be dropped, but do
not kill the connection.  This, combined with the tight window of valid
reliable acknowledgement numbers provides protection against malicious
address spoofing.

The qport field is a workaround for bad address translating routers that
sometimes remap the client's source port on a packet during gameplay.

If the base part of the net address matches and the qport matches, then the
channel matches even if the IP port differs.  The IP port should be updated
to the new value before sending out any replies.


If there is no information that needs to be transfered on a given frame,
such as during the connection stage while waiting for the client to load,
then a packet only needs to be delivered if there is something in the
unacknowledged reliable
*/
#[no_mangle]
pub static mut net_showpackets: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut net_chokeloopback: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut net_showdrop: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut net_qport: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut net_drop: libc::c_int = 0;
#[no_mangle]
pub static mut net_from: netadr_t =
    netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
#[no_mangle]
pub static mut net_message: sizebuf_t =
    sizebuf_t{bOverflow: false_0,
              pDebugName: 0 as *const libc::c_char,
              pData: 0 as *const byte as *mut byte,
              iCurBit: 0,
              nDataBits: 0,};
static mut net_mempool: poolhandle_t = 0;
#[no_mangle]
pub static mut net_message_buffer: [byte; 131120] = [0; 131120];
#[no_mangle]
pub static mut ns_strings: [*const libc::c_char; 2] =
    [b"Client\x00" as *const u8 as *const libc::c_char,
     b"Server\x00" as *const u8 as *const libc::c_char];
/*
=================================

NETWORK PACKET SPLIT

=================================
*/
/*
======================
NetSplit_GetLong

Collect fragmrnts with signature 0xFFFFFFFE to single packet
return true when got full packet
======================
*/
#[no_mangle]
pub unsafe extern "C" fn NetSplit_GetLong(mut ns: *mut netsplit_t,
                                          mut from: *mut netadr_t,
                                          mut data: *mut byte,
                                          mut length: *mut size_t)
 -> qboolean {
    let mut packet: *mut netsplit_packet_t = data as *mut netsplit_packet_t;
    let mut p: *mut netsplit_chain_packet_t =
        0 as *mut netsplit_chain_packet_t;
    //ASSERT( *length > NETSPLIT_HEADER_SIZE );
    if *length <= 18 as libc::c_int as libc::c_ulong { return false_0 }
    p =
        &mut *(*ns).packets.as_mut_ptr().offset(((*packet).id &
                                                     (8 as libc::c_int -
                                                          1 as libc::c_int) as
                                                         libc::c_uint) as
                                                    isize) as
            *mut netsplit_chain_packet_t;
    // Con_Reportf( S_NOTE "NetSplit_GetLong: packet from %s, id %d, index %d length %d\n", NET_AdrToString( *from ), (int)packet->id, (int)packet->index, (int)*length );
    // no packets with this id received
    if (*packet).id != (*p).id {
        // warn if previous packet not received
        if ((*p).received as libc::c_int) < (*p).count as libc::c_int {
            UI_ShowConnectionWarning();
            Con_Reportf(b"^3Warning:^7 NetSplit_GetLong: lost packet %d\n\x00"
                            as *const u8 as *const libc::c_char, (*p).id);
        }
        (*p).id = (*packet).id;
        (*p).count = (*packet).count;
        (*p).received = 0 as libc::c_int as byte;
        memset((*p).recieved_v.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int, 32 as libc::c_int as libc::c_ulong);
    }
    // use bool vector to detect dup packets
    if (*p).recieved_v[((*packet).index as libc::c_int >> 5 as libc::c_int) as
                           usize] &
           ((1 as libc::c_int) <<
                ((*packet).index as libc::c_int & 31 as libc::c_int)) as
               libc::c_uint != 0 {
        Con_Reportf(b"^3Warning:^7 NetSplit_GetLong: dup packet from %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(*from));
        return false_0
    }
    (*p).received = (*p).received.wrapping_add(1);
    // mark as received
    (*p).recieved_v[((*packet).index as libc::c_int >> 5 as libc::c_int) as
                        usize] |=
        ((1 as libc::c_int) <<
             ((*packet).index as libc::c_int & 31 as libc::c_int)) as
            libc::c_uint;
    // prevent overflow
    if (*packet).part.wrapping_mul((*packet).index as libc::c_uint) >
           0x20000 as libc::c_int as libc::c_uint {
        Con_Reportf(b"^3Warning:^7 NetSplit_GetLong: packet out fo bounds from %s (part %d index %d)\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(*from), (*packet).part,
                    (*packet).index as libc::c_int);
        return false_0
    }
    if (*packet).length > 0x20000 as libc::c_int as libc::c_uint {
        Con_Reportf(b"^3Warning:^7 NetSplit_GetLong: packet out fo bounds from %s (length %d)\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(*from), (*packet).length);
        return false_0
    }
    memcpy((*p).data.as_mut_ptr().offset((*packet).part.wrapping_mul((*packet).index
                                                                         as
                                                                         libc::c_uint)
                                             as isize) as *mut libc::c_void,
           (*packet).data.as_mut_ptr() as *const libc::c_void,
           (*length).wrapping_sub(18 as libc::c_int as libc::c_ulong));
    // rewrite results of NET_GetPacket
    if (*p).received as libc::c_int == (*packet).count as libc::c_int {
        //ASSERT( packet->length % packet->part == (*length - NETSPLIT_HEADER_SIZE) % packet->part );
        let mut len: size_t = (*packet).length as size_t;
        (*ns).total_received =
            ((*ns).total_received as libc::c_ulong).wrapping_add(len) as
                uint64_t as uint64_t;
        (*ns).total_received_uncompressed =
            ((*ns).total_received_uncompressed as
                 libc::c_ulong).wrapping_add(len) as uint64_t as uint64_t;
        *length = len;
        // Con_Reportf( S_NOTE "NetSplit_GetLong: packet from %s, id %d received %d length %d\n", NET_AdrToString( *from ), (int)packet->id, (int)p->received, (int)packet->length );
        memcpy(data as *mut libc::c_void,
               (*p).data.as_mut_ptr() as *const libc::c_void, len);
        return true_0
    } else {
        *length =
            (18 as libc::c_int as libc::c_uint).wrapping_add((*packet).part)
                as size_t
    }
    return false_0;
}
/*
======================
NetSplit_SendLong

Send parts that are less or equal maxpacket
======================
*/
#[no_mangle]
pub unsafe extern "C" fn NetSplit_SendLong(mut sock: netsrc_t,
                                           mut length: size_t,
                                           mut data: *mut libc::c_void,
                                           mut to: netadr_t,
                                           mut maxpacket: libc::c_uint,
                                           mut id: libc::c_uint) {
    let mut packet: netsplit_packet_t =
        {
            let mut init =
                netsplit_packet_s{signature: 0 as libc::c_int as uint32_t,
                                  length: 0,
                                  part: 0,
                                  id: 0,
                                  count: 0,
                                  index: 0,
                                  data: [0; 131054],};
            init
        };
    let mut part: libc::c_uint =
        maxpacket.wrapping_sub(18 as libc::c_int as libc::c_uint);
    packet.signature = 0xfffffffe as libc::c_uint;
    packet.id = id;
    packet.length = length as uint32_t;
    packet.part = part;
    packet.count =
        length.wrapping_sub(1 as libc::c_int as
                                libc::c_ulong).wrapping_div(part as
                                                                libc::c_ulong).wrapping_add(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
            as byte;
    //Con_Reportf( S_NOTE "NetSplit_SendLong: packet to %s, count %d, length %d\n", NET_AdrToString( to ), (int)packet.count, (int)packet.length );
    while (packet.index as libc::c_int) < packet.count as libc::c_int {
        let mut size: libc::c_uint = part;
        if size as libc::c_ulong > length { size = length as libc::c_uint }
        length =
            (length as libc::c_ulong).wrapping_sub(size as libc::c_ulong) as
                size_t as size_t;
        memcpy(packet.data.as_mut_ptr() as *mut libc::c_void,
               (data as
                    *const byte).offset((packet.index as
                                             libc::c_uint).wrapping_mul(part)
                                            as isize) as *const libc::c_void,
               size as libc::c_ulong);
        //Con_Reportf( S_NOTE "NetSplit_SendLong: packet to %s, id %d, index %d\n", NET_AdrToString( to ), (int)packet.id, (int)packet.index );
        NET_SendPacket(sock,
                       size.wrapping_add(18 as libc::c_int as libc::c_uint) as
                           size_t,
                       &mut packet as *mut netsplit_packet_t as
                           *const libc::c_void, to);
        packet.index = packet.index.wrapping_add(1)
    };
}
/*
===============
Netchan_Init
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_Init() {
    let mut port: libc::c_int = 0;
    // pick a port value that should be nice and random
    port = COM_RandomLong(1 as libc::c_int, 65535 as libc::c_int);
    net_showpackets =
        Cvar_Get(b"net_showpackets\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"show network packets\x00" as *const u8 as
                     *const libc::c_char);
    net_chokeloopback =
        Cvar_Get(b"net_chokeloop\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"apply bandwidth choke to loopback packets\x00" as *const u8
                     as *const libc::c_char);
    net_showdrop =
        Cvar_Get(b"net_showdrop\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"show packets that are dropped\x00" as *const u8 as
                     *const libc::c_char);
    net_qport =
        Cvar_Get(b"net_qport\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char, port),
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"current quake netport\x00" as *const u8 as
                     *const libc::c_char);
    net_mempool =
        _Mem_AllocPool(b"Network Pool\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/common/net_chan.c\x00" as *const u8 as
                           *const libc::c_char, 256 as libc::c_int);
    MSG_InitMasks();
    // initialize bit-masks
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Shutdown() {
    _Mem_FreePool(&mut net_mempool,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 263 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_ReportFlow(mut chan: *mut netchan_t) {
    let mut incoming: [libc::c_char; 64] = [0; 64];
    let mut outgoing: [libc::c_char; 64] = [0; 64];
    if CL_IsPlaybackDemo() as u64 != 0 { return }
    Q_strncpy(incoming.as_mut_ptr(),
              Q_pretifymem((*chan).flow[1 as libc::c_int as usize].totalbytes
                               as libc::c_float, 3 as libc::c_int),
              99999 as libc::c_int as size_t);
    Q_strncpy(outgoing.as_mut_ptr(),
              Q_pretifymem((*chan).flow[0 as libc::c_int as usize].totalbytes
                               as libc::c_float, 3 as libc::c_int),
              99999 as libc::c_int as size_t);
    Con_DPrintf(b"Signon network traffic:  %s from server, %s to server\n\x00"
                    as *const u8 as *const libc::c_char,
                incoming.as_mut_ptr(), outgoing.as_mut_ptr());
}
/*
==============
Netchan_IsLocal

detect a loopback message
==============
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_IsLocal(mut chan: *mut netchan_t)
 -> qboolean {
    if NET_IsActive() as u64 == 0 ||
           NET_IsLocalAddress((*chan).remote_address) as libc::c_uint != 0 {
        return true_0
    }
    return false_0;
}
/*
==============
Netchan_Setup

called to open a channel to a remote system
==============
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_Setup(mut sock: netsrc_t,
                                       mut chan: *mut netchan_t,
                                       mut adr: netadr_t,
                                       mut qport: libc::c_int,
                                       mut client: *mut libc::c_void,
                                       mut pfnBlockSize:
                                           Option<unsafe extern "C" fn(_:
                                                                           *mut libc::c_void,
                                                                       _:
                                                                           fragsize_t)
                                                      -> libc::c_int>) {
    Netchan_Clear(chan);
    memset(chan as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<netchan_t>() as libc::c_ulong);
    (*chan).sock = sock;
    (*chan).remote_address = adr;
    (*chan).last_received = host.realtime;
    (*chan).connect_time = host.realtime;
    (*chan).incoming_sequence = 0 as libc::c_int as libc::c_uint;
    (*chan).outgoing_sequence = 1 as libc::c_int as libc::c_uint;
    (*chan).rate = 9999.0f32 as libc::c_double;
    (*chan).qport = qport;
    (*chan).client = client;
    (*chan).pfnBlockSize = pfnBlockSize;
    MSG_InitExt(&mut (*chan).message,
                b"NetData\x00" as *const u8 as *const libc::c_char,
                (*chan).message_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131120]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
}
/*
==============================
Netchan_IncomingReady

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_IncomingReady(mut chan: *mut netchan_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if (*chan).incomingready[i as usize] as u64 != 0 { return true_0 }
        i += 1
    }
    return false_0;
}
/*
===============
Netchan_CanPacket

Returns true if the bandwidth choke isn't active
================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CanPacket(mut chan: *mut netchan_t,
                                           mut choke: qboolean) -> qboolean {
    // never choke loopback packets.
    if choke as u64 == 0 ||
           (*net_chokeloopback).value == 0. &&
               NET_IsLocalAddress((*chan).remote_address) as libc::c_uint != 0
       {
        (*chan).cleartime = host.realtime;
        return true_0
    }
    return if (*chan).cleartime < host.realtime {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
==============================
Netchan_UnlinkFragment

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_UnlinkFragment(mut buf: *mut fragbuf_t,
                                                mut list:
                                                    *mut *mut fragbuf_t) {
    let mut search: *mut fragbuf_t = 0 as *mut fragbuf_t;
    if list.is_null() { return }
    // at head of list
    if buf == *list {
        // remove first element
        *list = (*buf).next;
        // destroy remnant
        _Mem_Free(buf as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 380 as libc::c_int);
        return
    }
    search = *list;
    while !(*search).next.is_null() {
        if (*search).next == buf {
            (*search).next = (*buf).next;
            // destroy remnant
            _Mem_Free(buf as *mut libc::c_void,
                      b"../engine/common/net_chan.c\x00" as *const u8 as
                          *const libc::c_char, 392 as libc::c_int);
            return
        }
        search = (*search).next
    };
}
/*
==============================
Netchan_ClearFragbufs

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_ClearFragbufs(mut ppbuf:
                                                   *mut *mut fragbuf_t) {
    let mut buf: *mut fragbuf_t = 0 as *mut fragbuf_t;
    let mut n: *mut fragbuf_t = 0 as *mut fragbuf_t;
    if ppbuf.is_null() { return }
    // Throw away any that are sitting around
    buf = *ppbuf;
    while !buf.is_null() {
        n = (*buf).next;
        _Mem_Free(buf as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 417 as libc::c_int);
        buf = n
    }
    *ppbuf = 0 as *mut fragbuf_t;
}
/*
==============================
Netchan_ClearFragments

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_ClearFragments(mut chan: *mut netchan_t) {
    let mut wait: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut next: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        wait = (*chan).waitlist[i as usize];
        while !wait.is_null() {
            next = (*wait).next;
            Netchan_ClearFragbufs(&mut (*wait).fragbufs);
            _Mem_Free(wait as *mut libc::c_void,
                      b"../engine/common/net_chan.c\x00" as *const u8 as
                          *const libc::c_char, 443 as libc::c_int);
            wait = next
        }
        (*chan).waitlist[i as usize] = 0 as *mut fragbufwaiting_t;
        Netchan_ClearFragbufs(&mut *(*chan).fragbufs.as_mut_ptr().offset(i as
                                                                             isize));
        Netchan_FlushIncoming(chan, i);
        i += 1
    };
}
/*
==============================
Netchan_Clear

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_Clear(mut chan: *mut netchan_t) {
    let mut i: libc::c_int = 0;
    Netchan_ClearFragments(chan);
    (*chan).cleartime = 0.0f64;
    (*chan).reliable_length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        (*chan).reliable_fragid[i as usize] = 0 as libc::c_int as uint;
        (*chan).reliable_fragment[i as usize] = 0 as libc::c_int;
        (*chan).fragbufcount[i as usize] = 0 as libc::c_int;
        (*chan).frag_startpos[i as usize] = 0 as libc::c_int;
        (*chan).frag_length[i as usize] = 0 as libc::c_int;
        (*chan).incomingready[i as usize] = false_0;
        i += 1
    }
    if !(*chan).tempbuffer.is_null() {
        _Mem_Free((*chan).tempbuffer,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 480 as libc::c_int);
        (*chan).tempbuffer = 0 as *mut libc::c_void
    }
    (*chan).tempbuffersize = 0 as libc::c_int;
    memset((*chan).flow.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[flow_t; 2]>() as libc::c_ulong);
}
/*
===============
Netchan_OutOfBand

Sends an out-of-band datagram
================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_OutOfBand(mut net_socket: libc::c_int,
                                           mut adr: netadr_t,
                                           mut length: libc::c_int,
                                           mut data: *mut byte) {
    let mut send_buf: [byte; 8192] = [0; 8192];
    let mut send: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    // write the packet header
    MSG_InitExt(&mut send,
                b"SequencePacket\x00" as *const u8 as *const libc::c_char,
                send_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as
                    libc::c_int,
                -(1 as libc::c_int)); // -1 sequence means out of band
    MSG_WriteLong(&mut send, -(1 as libc::c_int));
    MSG_WriteBytes(&mut send, data as *const libc::c_void, length);
    if CL_IsPlaybackDemo() as u64 == 0 {
        // send the datagram
        NET_SendPacket(net_socket as netsrc_t,
                       MSG_GetNumBytesWritten(&mut send) as size_t,
                       MSG_GetData(&mut send) as *const libc::c_void, adr);
    };
}
/*
===============
Netchan_OutOfBandPrint

Sends a text message in an out-of-band datagram
================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_OutOfBandPrint(mut net_socket: libc::c_int,
                                                mut adr: netadr_t,
                                                mut format:
                                                    *const libc::c_char,
                                                mut args: ...) {
    let mut string: [libc::c_char; 8192] = [0; 8192];
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    Q_vsnprintf(string.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong), format,
                argptr.as_va_list());
    Netchan_OutOfBand(net_socket, adr,
                      Q_strlen(string.as_mut_ptr()) as libc::c_int,
                      string.as_mut_ptr() as *mut byte);
}
/*
==============================
Netchan_AllocFragbuf

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_AllocFragbuf() -> *mut fragbuf_t {
    let mut buf: *mut fragbuf_t = 0 as *mut fragbuf_t;
    buf =
        _Mem_Alloc(net_mempool,
                   ::std::mem::size_of::<fragbuf_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/net_chan.c\x00" as *const u8 as
                       *const libc::c_char, 542 as libc::c_int) as
            *mut fragbuf_t;
    MSG_InitExt(&mut (*buf).frag_message,
                b"Frag Message\x00" as *const u8 as *const libc::c_char,
                (*buf).frag_message_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 65535]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    return buf;
}
/*
==============================
Netchan_AddFragbufToTail

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_AddFragbufToTail(mut wait:
                                                      *mut fragbufwaiting_t,
                                                  mut buf: *mut fragbuf_t) {
    let mut p: *mut fragbuf_t = 0 as *mut fragbuf_t;
    (*buf).next = 0 as *mut fragbuf_s;
    (*wait).fragbufcount += 1;
    p = (*wait).fragbufs;
    if !p.is_null() {
        while !(*p).next.is_null() { p = (*p).next }
        (*p).next = buf
    } else { (*wait).fragbufs = buf };
}
/*
==============================
Netchan_UpdateFlow

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_UpdateFlow(mut chan: *mut netchan_t) {
    let mut faccumulatedtime: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    let mut flow: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    if chan.is_null() { return }
    flow = 0 as libc::c_int;
    while flow < 2 as libc::c_int {
        let mut pflow: *mut flow_t =
            &mut *(*chan).flow.as_mut_ptr().offset(flow as isize) as
                *mut flow_t;
        if !(host.realtime - (*pflow).nextcompute < 0.1f64) {
            (*pflow).nextcompute = host.realtime + 0.1f64;
            start = (*pflow).current - 1 as libc::c_int;
            // compute data flow rate
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int - 1 as libc::c_int {
                let mut pprev: *mut flowstats_t =
                    &mut *(*pflow).stats.as_mut_ptr().offset((start - i &
                                                                  32 as
                                                                      libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                 as isize) as
                        *mut flowstats_t;
                let mut pstat: *mut flowstats_t =
                    &mut *(*pflow).stats.as_mut_ptr().offset((start - i -
                                                                  1 as
                                                                      libc::c_int
                                                                  &
                                                                  32 as
                                                                      libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                 as isize) as
                        *mut flowstats_t;
                faccumulatedtime =
                    (faccumulatedtime as libc::c_double +
                         ((*pprev).time - (*pstat).time)) as libc::c_float;
                bytes += (*pstat).size;
                i += 1
            }
            (*pflow).kbytespersec =
                if faccumulatedtime == 0.0f32 {
                    0.0f32
                } else {
                    (bytes as libc::c_float / faccumulatedtime) / 1024.0f32
                };
            (*pflow).avgkbytespersec =
                (*pflow).avgkbytespersec * (2.0f32 / 3.0f32) +
                    (*pflow).kbytespersec * (1.0f32 - 2.0f32 / 3.0f32)
        }
        flow += 1
    };
}
/*
==============================
Netchan_FragSend

Fragmentation buffer is full and user is prepared to send
==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_FragSend(mut chan: *mut netchan_t) {
    let mut wait: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut i: libc::c_int = 0;
    if chan.is_null() { return }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        // already something queued up, just leave in waitlist
        if (*chan).fragbufs[i as usize].is_null() {
            wait = (*chan).waitlist[i as usize];
            // nothing to queue?
            if !wait.is_null() {
                (*chan).waitlist[i as usize] = (*wait).next;
                (*wait).next = 0 as *mut fbufqueue_s;
                // copy in to fragbuf
                (*chan).fragbufs[i as usize] = (*wait).fragbufs;
                (*chan).fragbufcount[i as usize] = (*wait).fragbufcount;
                // throw away wait list
                _Mem_Free(wait as *mut libc::c_void,
                          b"../engine/common/net_chan.c\x00" as *const u8 as
                              *const libc::c_char, 643 as libc::c_int);
            }
        }
        i += 1
    };
}
/*
==============================
Netchan_AddBufferToList

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_AddBufferToList(mut pplist:
                                                     *mut *mut fragbuf_t,
                                                 mut pbuf: *mut fragbuf_t) {
    // Find best slot
    let mut pprev: *mut fragbuf_t = 0 as *mut fragbuf_t; // next item in list
    let mut n: *mut fragbuf_t = 0 as *mut fragbuf_t;
    let mut id1: libc::c_int = 0;
    let mut id2: libc::c_int = 0;
    (*pbuf).next = 0 as *mut fragbuf_s;
    if pplist.is_null() { return }
    if (*pplist).is_null() { (*pbuf).next = *pplist; *pplist = pbuf; return }
    pprev = *pplist;
    while !(*pprev).next.is_null() {
        n = (*pprev).next;
        id1 = (*n).bufferid >> 16 as libc::c_int & 0xffff as libc::c_int;
        id2 = (*pbuf).bufferid >> 16 as libc::c_int & 0xffff as libc::c_int;
        if id1 > id2 {
            // insert here
            (*pbuf).next = (*n).next;
            (*pprev).next = pbuf;
            return
        }
        pprev = (*pprev).next
    }
    // insert at end
    (*pprev).next = pbuf;
}
/*
==============================
Netchan_CreateFragments_

==============================
*/
unsafe extern "C" fn Netchan_CreateFragments_(mut chan: *mut netchan_t,
                                              mut msg: *mut sizebuf_t) {
    let mut buf: *mut fragbuf_t = 0 as *mut fragbuf_t; // fallback
    let mut chunksize: libc::c_int = 0; // current position in bytes
    let mut remaining: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut bufferid: libc::c_int = 1 as libc::c_int;
    let mut wait: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut p: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    if MSG_GetNumBytesWritten(msg) == 0 as libc::c_int { return }
    if (*chan).pfnBlockSize.is_some() {
        chunksize =
            (*chan).pfnBlockSize.expect("non-null function pointer")((*chan).client,
                                                                     FRAGSIZE_FRAG)
    } else { chunksize = 64000 as libc::c_int }
    wait =
        _Mem_Alloc(net_mempool,
                   ::std::mem::size_of::<fragbufwaiting_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/net_chan.c\x00" as *const u8 as
                       *const libc::c_char, 714 as libc::c_int) as
            *mut fragbufwaiting_t;
    if LZSS_IsCompressed(MSG_GetData(msg)) as u64 == 0 {
        let mut uCompressedSize: uint = 0 as libc::c_int as uint;
        let mut uSourceSize: uint = MSG_GetNumBytesWritten(msg) as uint;
        let mut pbOut: *mut byte =
            LZSS_Compress((*msg).pData, uSourceSize as libc::c_int,
                          &mut uCompressedSize);
        if !pbOut.is_null() &&
               uCompressedSize > 0 as libc::c_int as libc::c_uint &&
               uCompressedSize < uSourceSize {
            Con_Reportf(b"Compressing split packet (%d -> %d bytes)\n\x00" as
                            *const u8 as *const libc::c_char, uSourceSize,
                        uCompressedSize);
            memcpy((*msg).pData as *mut libc::c_void,
                   pbOut as *const libc::c_void,
                   uCompressedSize as libc::c_ulong);
            MSG_SeekToBit(msg,
                          (uCompressedSize << 3 as libc::c_int) as
                              libc::c_int, 0 as libc::c_int);
        }
        if !pbOut.is_null() { free(pbOut as *mut libc::c_void); }
    }
    remaining = MSG_GetNumBytesWritten(msg);
    pos = 0 as libc::c_int;
    while remaining > 0 as libc::c_int {
        bytes = if remaining < chunksize { remaining } else { chunksize };
        remaining -= bytes;
        buf = Netchan_AllocFragbuf();
        let fresh0 = bufferid;
        bufferid = bufferid + 1;
        (*buf).bufferid = fresh0;
        // Copy in data
        MSG_Clear(&mut (*buf).frag_message);
        MSG_WriteBits(&mut (*buf).frag_message,
                      &mut *(*msg).pData.offset(pos as isize) as *mut byte as
                          *const libc::c_void, bytes << 3 as libc::c_int);
        Netchan_AddFragbufToTail(wait, buf);
        pos += bytes
    }
    // now add waiting list item to end of buffer queue
    if (*chan).waitlist[0 as libc::c_int as usize].is_null() {
        (*chan).waitlist[0 as libc::c_int as usize] = wait
    } else {
        p = (*chan).waitlist[0 as libc::c_int as usize];
        while !(*p).next.is_null() { p = (*p).next }
        (*p).next = wait
    };
}
/*
==============================
Netchan_CreateFragments

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CreateFragments(mut chan: *mut netchan_t,
                                                 mut msg: *mut sizebuf_t) {
    // always queue any pending reliable data ahead of the fragmentation buffer
    if MSG_GetNumBytesWritten(&mut (*chan).message) > 0 as libc::c_int {
        Netchan_CreateFragments_(chan, &mut (*chan).message);
        MSG_Clear(&mut (*chan).message);
    }
    Netchan_CreateFragments_(chan, msg);
}
/*
==============================
Netchan_FindBufferById

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_FindBufferById(mut pplist:
                                                    *mut *mut fragbuf_t,
                                                mut id: libc::c_int,
                                                mut allocate: qboolean)
 -> *mut fragbuf_t {
    let mut list: *mut fragbuf_t = *pplist;
    let mut pnewbuf: *mut fragbuf_t = 0 as *mut fragbuf_t;
    while !list.is_null() {
        if (*list).bufferid == id { return list }
        list = (*list).next
    }
    if allocate as u64 == 0 { return 0 as *mut fragbuf_t }
    // create new entry
    pnewbuf = Netchan_AllocFragbuf();
    (*pnewbuf).bufferid = id;
    Netchan_AddBufferToList(pplist, pnewbuf);
    return pnewbuf;
}
/*
==============================
Netchan_CheckForCompletion

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CheckForCompletion(mut chan: *mut netchan_t,
                                                    mut stream: libc::c_int,
                                                    mut intotalbuffers:
                                                        libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut p: *mut fragbuf_t = 0 as *mut fragbuf_t;
    size = 0 as libc::c_int;
    c = 0 as libc::c_int;
    p = (*chan).incomingbufs[stream as usize];
    if p.is_null() { return }
    while !p.is_null() {
        size += MSG_GetNumBytesWritten(&mut (*p).frag_message);
        c += 1;
        id = (*p).bufferid >> 16 as libc::c_int & 0xffff as libc::c_int;
        if id != c {
            if (*chan).sock as libc::c_uint ==
                   NS_CLIENT as libc::c_int as libc::c_uint {
                Con_DPrintf(b"^1Error:^7 Lost/dropped fragment would cause stall, retrying connection\n\x00"
                                as *const u8 as *const libc::c_char);
                Cbuf_AddText(b"reconnect\n\x00" as *const u8 as
                                 *const libc::c_char);
            }
        }
        p = (*p).next
    }
    // received final message
    if c == intotalbuffers {
        (*chan).incomingready[stream as usize] = true_0
    };
}
/*
==============================
Netchan_CreateFileFragmentsFromBuffer

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CreateFileFragmentsFromBuffer(mut chan:
                                                                   *mut netchan_t,
                                                               mut filename:
                                                                   *const libc::c_char,
                                                               mut pbuf:
                                                                   *mut byte,
                                                               mut size:
                                                                   libc::c_int) {
    let mut chunksize: libc::c_int = 0; // fallback
    let mut send: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut bufferid: libc::c_int = 1 as libc::c_int;
    let mut firstfragment: qboolean = true_0;
    let mut wait: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut p: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut buf: *mut fragbuf_t = 0 as *mut fragbuf_t;
    if size == 0 { return }
    if (*chan).pfnBlockSize.is_some() {
        chunksize =
            (*chan).pfnBlockSize.expect("non-null function pointer")((*chan).client,
                                                                     FRAGSIZE_FRAG)
    } else { chunksize = 64000 as libc::c_int }
    if LZSS_IsCompressed(pbuf) as u64 == 0 {
        let mut uCompressedSize: uint = 0 as libc::c_int as uint;
        let mut pbOut: *mut byte =
            LZSS_Compress(pbuf, size, &mut uCompressedSize);
        if !pbOut.is_null() &&
               uCompressedSize > 0 as libc::c_int as libc::c_uint &&
               uCompressedSize < size as libc::c_uint {
            Con_DPrintf(b"Compressing filebuffer (%s -> %s)\n\x00" as
                            *const u8 as *const libc::c_char,
                        Q_pretifymem(size as libc::c_float, 2 as libc::c_int),
                        Q_pretifymem(uCompressedSize as libc::c_float,
                                     2 as libc::c_int));
            memcpy(pbuf as *mut libc::c_void, pbOut as *const libc::c_void,
                   uCompressedSize as libc::c_ulong);
            size = uCompressedSize as libc::c_int
        }
        if !pbOut.is_null() { free(pbOut as *mut libc::c_void); }
    }
    wait =
        _Mem_Alloc(net_mempool,
                   ::std::mem::size_of::<fragbufwaiting_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/net_chan.c\x00" as *const u8 as
                       *const libc::c_char, 889 as libc::c_int) as
            *mut fragbufwaiting_t;
    remaining = size;
    pos = 0 as libc::c_int;
    while remaining > 0 as libc::c_int {
        send = if remaining < chunksize { remaining } else { chunksize };
        buf = Netchan_AllocFragbuf();
        let fresh1 = bufferid;
        bufferid = bufferid + 1;
        (*buf).bufferid = fresh1;
        // copy in data
        MSG_Clear(&mut (*buf).frag_message);
        if firstfragment as u64 != 0 {
            // write filename
            MSG_WriteString(&mut (*buf).frag_message, filename);
            // send a bit less on first package
            send -= MSG_GetNumBytesWritten(&mut (*buf).frag_message);
            firstfragment = false_0
        }
        (*buf).isbuffer = true_0;
        (*buf).isfile = true_0;
        (*buf).size = send;
        (*buf).foffset = pos;
        MSG_WriteBits(&mut (*buf).frag_message,
                      pbuf.offset(pos as isize) as *const libc::c_void,
                      send << 3 as libc::c_int);
        remaining -= send;
        pos += send;
        Netchan_AddFragbufToTail(wait, buf);
    }
    // now add waiting list item to end of buffer queue
    if (*chan).waitlist[1 as libc::c_int as usize].is_null() {
        (*chan).waitlist[1 as libc::c_int as usize] = wait
    } else {
        p = (*chan).waitlist[1 as libc::c_int as usize];
        while !(*p).next.is_null() { p = (*p).next }
        (*p).next = wait
    };
}
/*
==============================
Netchan_CreateFileFragments

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CreateFileFragments(mut chan: *mut netchan_t,
                                                     mut filename:
                                                         *const libc::c_char)
 -> libc::c_int {
    let mut chunksize: libc::c_int = 0; // fallback
    let mut send: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut bufferid: libc::c_int = 1 as libc::c_int;
    let mut filesize: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    let mut compressedfilename: [libc::c_char; 260] = [0; 260];
    let mut compressedFileTime: libc::c_int = 0;
    let mut fileTime: libc::c_int = 0;
    let mut firstfragment: qboolean = true_0;
    let mut bCompressed: qboolean = false_0;
    let mut wait: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut p: *mut fragbufwaiting_t = 0 as *mut fragbufwaiting_t;
    let mut buf: *mut fragbuf_t = 0 as *mut fragbuf_t;
    filesize = FS_FileSize(filename, false_0);
    if filesize <= 0 as libc::c_int as libc::c_long {
        Con_Printf(b"^3Warning:^7 Unable to open %s for transfer\n\x00" as
                       *const u8 as *const libc::c_char, filename);
        return 0 as libc::c_int
    }
    if (*chan).pfnBlockSize.is_some() {
        chunksize =
            (*chan).pfnBlockSize.expect("non-null function pointer")((*chan).client,
                                                                     FRAGSIZE_FRAG)
    } else { chunksize = 64000 as libc::c_int }
    Q_strncpy(compressedfilename.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong);
    COM_ReplaceExtension(compressedfilename.as_mut_ptr(),
                         b".ztmp\x00" as *const u8 as *const libc::c_char);
    compressedFileTime =
        FS_FileTime(compressedfilename.as_mut_ptr(), false_0);
    fileTime = FS_FileTime(filename, false_0);
    if compressedFileTime >= fileTime {
        // if compressed file already created and newer than source
        if FS_FileSize(compressedfilename.as_mut_ptr(), false_0) !=
               -(1 as libc::c_int) as libc::c_long {
            bCompressed = true_0
        }
    } else {
        let mut uCompressedSize: uint = 0;
        let mut uncompressed: *mut byte = 0 as *mut byte;
        let mut compressed: *mut byte = 0 as *mut byte;
        uncompressed = FS_LoadFile(filename, &mut filesize, false_0);
        compressed =
            LZSS_Compress(uncompressed, filesize as libc::c_int,
                          &mut uCompressedSize);
        if !compressed.is_null() {
            Con_DPrintf(b"compressed file %s (%s -> %s)\n\x00" as *const u8 as
                            *const libc::c_char, filename,
                        Q_pretifymem(filesize as libc::c_float,
                                     2 as libc::c_int),
                        Q_pretifymem(uCompressedSize as libc::c_float,
                                     2 as libc::c_int));
            FS_WriteFile(compressedfilename.as_mut_ptr(),
                         compressed as *const libc::c_void,
                         uCompressedSize as fs_offset_t);
            filesize = uCompressedSize as fs_offset_t;
            bCompressed = true_0;
            free(compressed as *mut libc::c_void);
        }
        _Mem_Free(uncompressed as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 1001 as libc::c_int);
    }
    wait =
        _Mem_Alloc(net_mempool,
                   ::std::mem::size_of::<fragbufwaiting_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/net_chan.c\x00" as *const u8 as
                       *const libc::c_char, 1004 as libc::c_int) as
            *mut fragbufwaiting_t;
    remaining = filesize as libc::c_int;
    pos = 0 as libc::c_int;
    while remaining > 0 as libc::c_int {
        send = if remaining < chunksize { remaining } else { chunksize };
        buf = Netchan_AllocFragbuf();
        let fresh2 = bufferid;
        bufferid = bufferid + 1;
        (*buf).bufferid = fresh2;
        // copy in data
        MSG_Clear(&mut (*buf).frag_message);
        if firstfragment as u64 != 0 {
            // Write filename
            MSG_WriteString(&mut (*buf).frag_message, filename);
            // Send a bit less on first package
            send -= MSG_GetNumBytesWritten(&mut (*buf).frag_message);
            firstfragment = false_0
        }
        (*buf).isfile = true_0;
        (*buf).size = send;
        (*buf).foffset = pos;
        (*buf).iscompressed = bCompressed;
        Q_strncpy((*buf).filename.as_mut_ptr(), filename,
                  ::std::mem::size_of::<[libc::c_char; 260]>() as
                      libc::c_ulong);
        pos += send;
        remaining -= send;
        Netchan_AddFragbufToTail(wait, buf);
    }
    // now add waiting list item to end of buffer queue
    if (*chan).waitlist[1 as libc::c_int as usize].is_null() {
        (*chan).waitlist[1 as libc::c_int as usize] = wait
    } else {
        p = (*chan).waitlist[1 as libc::c_int as usize];
        while !(*p).next.is_null() { p = (*p).next }
        (*p).next = wait
    }
    return 1 as libc::c_int;
}
// biggest packet that has frag and or reliable data
// forward declarations
/*
==============================
Netchan_FlushIncoming

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_FlushIncoming(mut chan: *mut netchan_t,
                                               mut stream: libc::c_int) {
    let mut p: *mut fragbuf_t = 0 as *mut fragbuf_t;
    let mut n: *mut fragbuf_t = 0 as *mut fragbuf_t;
    MSG_Clear(&mut net_message);
    p = (*chan).incomingbufs[stream as usize];
    while !p.is_null() {
        n = (*p).next;
        _Mem_Free(p as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 1074 as libc::c_int);
        p = n
    }
    (*chan).incomingbufs[stream as usize] = 0 as *mut fragbuf_t;
    (*chan).incomingready[stream as usize] = false_0;
}
/*
==============================
Netchan_CopyNormalFragments

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CopyNormalFragments(mut chan: *mut netchan_t,
                                                     mut msg: *mut sizebuf_t,
                                                     mut length: *mut size_t)
 -> qboolean {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut fragbuf_t = 0 as *mut fragbuf_t;
    let mut n: *mut fragbuf_t = 0 as *mut fragbuf_t;
    if (*chan).incomingready[0 as libc::c_int as usize] as u64 == 0 {
        return false_0
    }
    if (*chan).incomingbufs[0 as libc::c_int as usize].is_null() {
        (*chan).incomingready[0 as libc::c_int as usize] = false_0;
        return false_0
    }
    p = (*chan).incomingbufs[0 as libc::c_int as usize];
    MSG_InitExt(msg, b"NetMessage\x00" as *const u8 as *const libc::c_char,
                net_message_buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131120]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    while !p.is_null() {
        n = (*p).next;
        // copy it in
        MSG_WriteBytes(msg,
                       MSG_GetData(&mut (*p).frag_message) as
                           *const libc::c_void,
                       MSG_GetNumBytesWritten(&mut (*p).frag_message));
        size =
            (size as
                 libc::c_ulong).wrapping_add(MSG_GetNumBytesWritten(&mut (*p).frag_message)
                                                 as libc::c_ulong) as size_t
                as size_t;
        _Mem_Free(p as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 1113 as libc::c_int);
        p = n
    }
    if LZSS_IsCompressed(MSG_GetData(msg)) as u64 != 0 {
        let mut uDecompressedLen: uint = LZSS_GetActualSize(MSG_GetData(msg));
        let mut buf: [byte; 131120] = [0; 131120];
        if uDecompressedLen as libc::c_ulong <=
               ::std::mem::size_of::<[byte; 131120]>() as libc::c_ulong {
            size =
                LZSS_Decompress(MSG_GetData(msg), buf.as_mut_ptr()) as size_t;
            memcpy((*msg).pData as *mut libc::c_void,
                   buf.as_mut_ptr() as *const libc::c_void, size);
        } else {
            // g-cont. this should not happens
            Con_Printf(b"^1Error:^7 buffer to small to decompress message\n\x00"
                           as *const u8 as *const libc::c_char);
            return false_0
        }
    }
    (*chan).incomingbufs[0 as libc::c_int as usize] = 0 as *mut fragbuf_t;
    // reset flag
    (*chan).incomingready[0 as libc::c_int as usize] = false_0;
    // tell about message size
    if !length.is_null() { *length = size }
    return true_0;
}
/*
==============================
Netchan_CopyFileFragments

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_CopyFileFragments(mut chan: *mut netchan_t,
                                                   mut msg: *mut sizebuf_t)
 -> qboolean {
    let mut filename: [libc::c_char; 260] = [0; 260];
    let mut nsize: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut p: *mut fragbuf_t = 0 as *mut fragbuf_t;
    let mut n: *mut fragbuf_t = 0 as *mut fragbuf_t;
    if (*chan).incomingready[1 as libc::c_int as usize] as u64 == 0 {
        return false_0
    }
    if (*chan).incomingbufs[1 as libc::c_int as usize].is_null() {
        (*chan).incomingready[1 as libc::c_int as usize] = false_0;
        return false_0
    }
    p = (*chan).incomingbufs[1 as libc::c_int as usize];
    MSG_InitExt(msg, b"NetMessage\x00" as *const u8 as *const libc::c_char,
                net_message_buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131120]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // copy in first chunk so we can get filename out
    MSG_WriteBytes(msg,
                   MSG_GetData(&mut (*p).frag_message) as *const libc::c_void,
                   MSG_GetNumBytesWritten(&mut (*p).frag_message));
    MSG_Clear(msg);
    Q_strncpy(filename.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong);
    if if filename.as_mut_ptr().is_null() || *filename.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Printf(b"^1Error:^7 file fragment received with no filename\nFlushing input queue\n\x00"
                       as *const u8 as *const libc::c_char);
        Netchan_FlushIncoming(chan, 1 as libc::c_int);
        return false_0
    } else {
        if filename[0 as libc::c_int as usize] as libc::c_int != '!' as i32 &&
               COM_IsSafeFileToDownload(filename.as_mut_ptr()) as u64 == 0 {
            Con_Printf(b"^1Error:^7 file fragment received with bad path, ignoring\n\x00"
                           as *const u8 as *const libc::c_char);
            Netchan_FlushIncoming(chan, 1 as libc::c_int);
            return false_0
        }
    }
    Q_strncpy((*chan).incomingfilename.as_mut_ptr(), filename.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong);
    if filename[0 as libc::c_int as usize] as libc::c_int != '!' as i32 &&
           FS_FileExists(filename.as_mut_ptr(), false_0 as libc::c_int) != 0 {
        Con_Printf(b"^1Error:^7 can\'t download %s, already exists\n\x00" as
                       *const u8 as *const libc::c_char,
                   filename.as_mut_ptr());
        Netchan_FlushIncoming(chan, 1 as libc::c_int);
        return true_0
    }
    // create file from buffers
    nsize = 0 as libc::c_int; // Size will include a bit of slop, oh well
    while !p.is_null() {
        nsize += MSG_GetNumBytesWritten(&mut (*p).frag_message);
        if p == (*chan).incomingbufs[1 as libc::c_int as usize] {
            nsize -= MSG_GetNumBytesWritten(msg)
        }
        p = (*p).next
    }
    buffer =
        _Mem_Alloc(net_mempool, (nsize + 1 as libc::c_int) as size_t, true_0,
                   b"../engine/common/net_chan.c\x00" as *const u8 as
                       *const libc::c_char, 1210 as libc::c_int) as *mut byte;
    p = (*chan).incomingbufs[1 as libc::c_int as usize];
    pos = 0 as libc::c_int;
    while !p.is_null() {
        let mut cursize: libc::c_int = 0;
        n = (*p).next;
        cursize = MSG_GetNumBytesWritten(&mut (*p).frag_message);
        // first message has the file name, don't write that into the data stream,
		// just write the rest of the actual data
        if p == (*chan).incomingbufs[1 as libc::c_int as usize] {
            // copy it in
            cursize -= MSG_GetNumBytesWritten(msg);
            memcpy(&mut *buffer.offset(pos as isize) as *mut byte as
                       *mut libc::c_void,
                   &mut *(*p).frag_message.pData.offset((MSG_GetNumBytesWritten
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut sizebuf_t)
                                                                 ->
                                                                     libc::c_int)(msg)
                                                            as isize) as
                       *mut byte as *const libc::c_void,
                   cursize as libc::c_ulong);
        } else {
            memcpy(&mut *buffer.offset(pos as isize) as *mut byte as
                       *mut libc::c_void,
                   (*p).frag_message.pData as *const libc::c_void,
                   cursize as libc::c_ulong);
        }
        pos += cursize;
        _Mem_Free(p as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 1236 as libc::c_int);
        p = n
    }
    if LZSS_IsCompressed(buffer) as u64 != 0 {
        let mut uncompressedSize: uint =
            LZSS_GetActualSize(buffer).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint);
        let mut uncompressedBuffer: *mut byte =
            _Mem_Alloc(net_mempool, uncompressedSize as size_t, true_0,
                       b"../engine/common/net_chan.c\x00" as *const u8 as
                           *const libc::c_char, 1243 as libc::c_int) as
                *mut byte;
        nsize = LZSS_Decompress(buffer, uncompressedBuffer) as libc::c_int;
        _Mem_Free(buffer as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 1246 as libc::c_int);
        buffer = uncompressedBuffer
    }
    // customization files goes int tempbuffer
    if filename[0 as libc::c_int as usize] as libc::c_int == '!' as i32 {
        if !(*chan).tempbuffer.is_null() {
            _Mem_Free((*chan).tempbuffer,
                      b"../engine/common/net_chan.c\x00" as *const u8 as
                          *const libc::c_char, 1254 as libc::c_int);
        }
        (*chan).tempbuffer = buffer as *mut libc::c_void;
        (*chan).tempbuffersize = nsize
    } else {
        // g-cont. it's will be stored downloaded files directly into game folder
        FS_WriteFile(filename.as_mut_ptr(), buffer as *const libc::c_void,
                     nsize as fs_offset_t);
        _Mem_Free(buffer as *mut libc::c_void,
                  b"../engine/common/net_chan.c\x00" as *const u8 as
                      *const libc::c_char, 1262 as libc::c_int);
    }
    // clear remnants
    MSG_Clear(msg);
    (*chan).incomingbufs[1 as libc::c_int as usize] = 0 as *mut fragbuf_t;
    (*chan).incomingready[1 as libc::c_int as usize] = false_0;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Validate(mut chan: *mut netchan_t,
                                          mut sb: *mut sizebuf_t,
                                          mut frag_message: *mut qboolean,
                                          mut fragid: *mut uint,
                                          mut frag_offset: *mut libc::c_int,
                                          mut frag_length: *mut libc::c_int)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut buffer: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if !(*frag_message.offset(i as isize) as u64 == 0) {
            buffer =
                (*fragid.offset(i as isize) >> 16 as libc::c_int &
                     0xffff as libc::c_int as libc::c_uint) as libc::c_int;
            count =
                (*fragid.offset(i as isize) &
                     0xffff as libc::c_int as libc::c_uint) as libc::c_int;
            offset = BitByte(*frag_offset.offset(i as isize));
            length = BitByte(*frag_length.offset(i as isize));
            if buffer < 0 as libc::c_int || buffer > 32767 as libc::c_int {
                return false_0
            }
            if count < 0 as libc::c_int || count > 32767 as libc::c_int {
                return false_0
            }
            if length < 0 as libc::c_int ||
                   length > (64000 as libc::c_int) << 3 as libc::c_int {
                return false_0
            }
            if offset < 0 as libc::c_int ||
                   offset > (64000 as libc::c_int) << 3 as libc::c_int {
                return false_0
            }
        }
        i += 1
    }
    return true_0;
}
/*
==============================
Netchan_UpdateProgress

==============================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_UpdateProgress(mut chan: *mut netchan_t) {
    let mut p: *mut fragbuf_t = 0 as *mut fragbuf_t;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut bestpercent: libc::c_float = 0.0f64 as libc::c_float;
    if host.downloadcount == 0 as libc::c_int {
        (*scr_download).value = -1.0f32;
        host.downloadfile[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    // do show slider for file downloads.
    if (*chan).incomingbufs[1 as libc::c_int as usize].is_null() { return }
    i = 2 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        // receiving data
        if !(*chan).incomingbufs[i as usize].is_null() {
            p = (*chan).incomingbufs[i as usize];
            total = (*p).bufferid & 0xffff as libc::c_int;
            while !p.is_null() { c += 1; p = (*p).next }
            if total != 0 {
                let mut percent: libc::c_float =
                    100.0f32 * c as libc::c_float / total as libc::c_float;
                if percent > bestpercent { bestpercent = percent }
            }
            p = (*chan).incomingbufs[i as usize];
            if i == 1 as libc::c_int {
                let mut sz: [libc::c_char; 1024] = [0; 1024];
                let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut len: libc::c_int = 0 as libc::c_int;
                in_0 =
                    MSG_GetData(&mut (*p).frag_message) as *mut libc::c_char;
                out = sz.as_mut_ptr();
                while *in_0 != 0 {
                    let fresh3 = in_0;
                    in_0 = in_0.offset(1);
                    let fresh4 = out;
                    out = out.offset(1);
                    *fresh4 = *fresh3;
                    len += 1;
                    if len > 128 as libc::c_int { break ; }
                }
                *out = '\u{0}' as i32 as libc::c_char;
                if (if *sz.as_mut_ptr() == 0 {
                        0 as libc::c_int
                    } else { 1 as libc::c_int }) != 0 &&
                       sz[0 as libc::c_int as usize] as libc::c_int !=
                           '!' as i32 {
                    Q_strncpy(host.downloadfile.as_mut_ptr(), sz.as_mut_ptr(),
                              ::std::mem::size_of::<string>() as
                                  libc::c_ulong);
                }
            }
        } else if !(*chan).fragbufs[i as usize].is_null() {
            // Sending data
            if (*chan).fragbufcount[i as usize] != 0 {
                let mut percent_0: libc::c_float =
                    100.0f32 *
                        (*(*chan).fragbufs[i as usize]).bufferid as
                            libc::c_float /
                        (*chan).fragbufcount[i as usize] as libc::c_float;
                if percent_0 > bestpercent { bestpercent = percent_0 }
            }
        }
        i -= 1
    }
    (*scr_download).value = bestpercent;
    // XASH_DEDICATED
}
/*
===============
Netchan_TransmitBits

tries to send an unreliable message to a connection, and handles the
transmition / retransmition of the reliable messages.

A 0 length will still generate a packet and deal with the reliable messages.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_TransmitBits(mut chan: *mut netchan_t,
                                              mut length: libc::c_int,
                                              mut data: *mut byte) {
    let mut send_buf: [byte; 131120] = [0; 131120];
    let mut send_reliable_fragment: qboolean = false_0;
    let mut w1: uint = 0;
    let mut w2: uint = 0;
    let mut statId: uint = 0;
    let mut send_reliable: qboolean = false_0;
    let mut send: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut fRate: libc::c_float = 0.;
    // check for message overflow
    if MSG_CheckOverflow(&mut (*chan).message) as u64 != 0 {
        Con_Printf(b"^1Error:^7 %s:outgoing message overflow\n\x00" as
                       *const u8 as *const libc::c_char,
                   NET_AdrToString((*chan).remote_address));
        return
    }
    // if the remote side dropped the last reliable message, resend it
    send_reliable = false_0;
    if (*chan).incoming_acknowledged > (*chan).last_reliable_sequence &&
           (*chan).incoming_reliable_acknowledged != (*chan).reliable_sequence
       {
        send_reliable = true_0
    }
    // A packet can have "reliable payload + frag payload + unreliable payload
	// frag payload can be a file chunk, if so, it needs to be parsed on the receiving end and reliable payload + unreliable payload need
	// to be passed on to the message queue.  The processing routine needs to be able to handle the case where a message comes in and a file
	// transfer completes
    // if the reliable transmit buffer is empty, copy the current message out
    if (*chan).reliable_length == 0 {
        let mut send_frag: qboolean = false_0;
        let mut pbuf: *mut fragbuf_t = 0 as *mut fragbuf_t;
        // will be true if we are active and should let chan->message get some bandwidth
        let mut send_from_frag: [libc::c_int; 2] =
            [0 as libc::c_int, 0 as libc::c_int];
        let mut send_from_regular: libc::c_int = 0 as libc::c_int;
        // if we have data in the waiting list(s) and we have cleared the current queue(s), then
		// push the waitlist(s) into the current queue(s)
        Netchan_FragSend(chan);
        // sending regular payload
        send_from_regular =
            if MSG_GetNumBytesWritten(&mut (*chan).message) != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        // check to see if we are sending a frag payload
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if !(*chan).fragbufs[i as usize].is_null() {
                send_from_frag[i as usize] = 1 as libc::c_int
            }
            i += 1
        }
        // stall reliable payloads if sending from frag buffer
        if send_from_regular != 0 &&
               send_from_frag[0 as libc::c_int as usize] != 0 {
            let mut maxsize: libc::c_int = 1400 as libc::c_int;
            send_from_regular = false_0 as libc::c_int;
            if (*chan).pfnBlockSize.is_some() {
                maxsize =
                    (*chan).pfnBlockSize.expect("non-null function pointer")((*chan).client,
                                                                             FRAGSIZE_SPLIT)
            }
            if maxsize == 0 as libc::c_int { maxsize = 1400 as libc::c_int }
            // if the reliable buffer has gotten too big, queue it at the end of everything and clear out buffer
            if (MSG_GetNumBytesWritten(&mut (*chan).message) as
                    libc::c_uint).wrapping_add(length as uint >>
                                                   3 as libc::c_int) >
                   maxsize as libc::c_uint {
                Netchan_CreateFragments_(chan, &mut (*chan).message);
                MSG_Clear(&mut (*chan).message);
            }
        }
        // startpos will be zero if there is no regular payload
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            (*chan).frag_startpos[i as usize] = 0 as libc::c_int;
            // assume no fragment is being sent
            (*chan).reliable_fragment[i as usize] = 0 as libc::c_int;
            (*chan).reliable_fragid[i as usize] = 0 as libc::c_int as uint;
            (*chan).frag_length[i as usize] = 0 as libc::c_int;
            if send_from_frag[i as usize] != 0 { send_frag = true_0 }
            i += 1
        }
        if send_from_regular != 0 || send_frag as libc::c_uint != 0 {
            (*chan).reliable_sequence ^= 1 as libc::c_int as libc::c_uint;
            send_reliable = true_0
        }
        if send_from_regular != 0 {
            memcpy((*chan).reliable_buf.as_mut_ptr() as *mut libc::c_void,
                   (*chan).message_buf.as_mut_ptr() as *const libc::c_void,
                   MSG_GetNumBytesWritten(&mut (*chan).message) as
                       libc::c_ulong);
            (*chan).reliable_length =
                MSG_GetNumBitsWritten(&mut (*chan).message);
            MSG_Clear(&mut (*chan).message);
            // if we send fragments, this is where they'll start
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                (*chan).frag_startpos[i as usize] = (*chan).reliable_length;
                i += 1
            }
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut newpayloadsize: libc::c_int = 0;
            let mut fragment_size: libc::c_int = 0;
            // is there someting in the fragbuf?
            pbuf = (*chan).fragbufs[i as usize];
            fragment_size = 0 as libc::c_int;
            if !pbuf.is_null() {
                fragment_size =
                    MSG_GetNumBytesWritten(&mut (*pbuf).frag_message);
                // files set size a bit differently.
                if (*pbuf).isfile as libc::c_uint != 0 &&
                       (*pbuf).isbuffer as u64 == 0 {
                    fragment_size = (*pbuf).size
                }
            }
            newpayloadsize =
                (*chan).reliable_length + (fragment_size << 3 as libc::c_int)
                    + 7 as libc::c_int >> 3 as libc::c_int;
            // make sure we have enought space left
            if send_from_frag[i as usize] != 0 && !pbuf.is_null() &&
                   newpayloadsize < 65535 as libc::c_int {
                let mut temp: sizebuf_t =
                    sizebuf_t{bOverflow: false_0,
                              pDebugName: 0 as *const libc::c_char,
                              pData: 0 as *const byte as *mut byte,
                              iCurBit: 0,
                              nDataBits: 0,};
                // which buffer are we sending ?
                (*chan).reliable_fragid[i as usize] =
                    (((*pbuf).bufferid & 0xffff as libc::c_int) <<
                         16 as libc::c_int |
                         (*chan).fragbufcount[i as usize] &
                             0xffff as libc::c_int) as uint;
                // if it's not in-memory, then we'll need to copy it in frame the file handle.
                if (*pbuf).isfile as libc::c_uint != 0 &&
                       (*pbuf).isbuffer as u64 == 0 {
                    let mut filebuffer: [byte; 65535] = [0; 65535];
                    let mut file: *mut file_t = 0 as *mut file_t;
                    if (*pbuf).iscompressed as u64 != 0 {
                        let mut compressedfilename: [libc::c_char; 260] =
                            [0; 260];
                        Q_strncpy(compressedfilename.as_mut_ptr(),
                                  (*pbuf).filename.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 260]>()
                                      as libc::c_ulong);
                        COM_ReplaceExtension(compressedfilename.as_mut_ptr(),
                                             b".ztmp\x00" as *const u8 as
                                                 *const libc::c_char);
                        file =
                            FS_Open(compressedfilename.as_mut_ptr(),
                                    b"rb\x00" as *const u8 as
                                        *const libc::c_char, false_0)
                    } else {
                        file =
                            FS_Open((*pbuf).filename.as_mut_ptr(),
                                    b"rb\x00" as *const u8 as
                                        *const libc::c_char, false_0)
                    }
                    FS_Seek(file, (*pbuf).foffset as fs_offset_t,
                            0 as libc::c_int);
                    FS_Read(file,
                            filebuffer.as_mut_ptr() as *mut libc::c_void,
                            (*pbuf).size as size_t);
                    MSG_WriteBits(&mut (*pbuf).frag_message,
                                  filebuffer.as_mut_ptr() as
                                      *const libc::c_void,
                                  (*pbuf).size << 3 as libc::c_int);
                    FS_Close(file);
                }
                // copy frag stuff on top of current buffer
                MSG_StartWriting(&mut temp,
                                 (*chan).reliable_buf.as_mut_ptr() as
                                     *mut libc::c_void,
                                 ::std::mem::size_of::<[byte; 131120]>() as
                                     libc::c_ulong as libc::c_int,
                                 (*chan).reliable_length,
                                 -(1 as libc::c_int));
                MSG_WriteBits(&mut temp,
                              MSG_GetData(&mut (*pbuf).frag_message) as
                                  *const libc::c_void,
                              MSG_GetNumBitsWritten(&mut (*pbuf).frag_message));
                (*chan).reliable_length +=
                    MSG_GetNumBitsWritten(&mut (*pbuf).frag_message);
                (*chan).frag_length[i as usize] =
                    MSG_GetNumBitsWritten(&mut (*pbuf).frag_message);
                // unlink pbuf
                Netchan_UnlinkFragment(pbuf,
                                       &mut *(*chan).fragbufs.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize));
                (*chan).reliable_fragment[i as usize] = 1 as libc::c_int;
                // offset the rest of the starting positions
                j = i + 1 as libc::c_int;
                while j < 2 as libc::c_int {
                    (*chan).frag_startpos[j as usize] +=
                        (*chan).frag_length[i as usize];
                    j += 1
                }
            }
            i += 1
        }
    }
    MSG_InitExt(&mut send, b"NetSend\x00" as *const u8 as *const libc::c_char,
                send_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131120]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // prepare the packet header
    w1 =
        (*chan).outgoing_sequence |
            (send_reliable as libc::c_uint) << 31 as libc::c_int;
    w2 =
        (*chan).incoming_sequence |
            (*chan).incoming_reliable_sequence << 31 as libc::c_int;
    send_reliable_fragment = false_0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if (*chan).reliable_fragment[i as usize] != 0 {
            send_reliable_fragment = true_0;
            break ;
        } else { i += 1 }
    }
    if send_reliable as libc::c_uint != 0 &&
           send_reliable_fragment as libc::c_uint != 0 {
        w1 = w1 | (1 as libc::c_uint) << 30 as libc::c_int
    }
    (*chan).outgoing_sequence = (*chan).outgoing_sequence.wrapping_add(1);
    MSG_WriteLong(&mut send, w1 as libc::c_int);
    MSG_WriteLong(&mut send, w2 as libc::c_int);
    // send the qport if we are a client
    if (*chan).sock as libc::c_uint ==
           NS_CLIENT as libc::c_int as libc::c_uint {
        MSG_WriteWord(&mut send,
                      Cvar_VariableInteger(b"net_qport\x00" as *const u8 as
                                               *const libc::c_char));
    }
    if send_reliable as libc::c_uint != 0 &&
           send_reliable_fragment as libc::c_uint != 0 {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if (*chan).reliable_fragment[i as usize] != 0 {
                MSG_WriteByte(&mut send, 1 as libc::c_int);
                MSG_WriteLong(&mut send,
                              (*chan).reliable_fragid[i as usize] as
                                  libc::c_int);
                MSG_WriteLong(&mut send, (*chan).frag_startpos[i as usize]);
                MSG_WriteLong(&mut send, (*chan).frag_length[i as usize]);
            } else { MSG_WriteByte(&mut send, 0 as libc::c_int); }
            i += 1
        }
    }
    // copy the reliable message to the packet first
    if send_reliable as u64 != 0 {
        MSG_WriteBits(&mut send,
                      (*chan).reliable_buf.as_mut_ptr() as
                          *const libc::c_void, (*chan).reliable_length);
        (*chan).last_reliable_sequence =
            (*chan).outgoing_sequence.wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint)
    }
    if length != 0 {
        let mut maxsize_0: libc::c_int =
            (0x20000 as libc::c_int +
                 (8 as libc::c_int + 2 as libc::c_int * 13 as libc::c_int) +
                 (16 as libc::c_int - 1 as libc::c_int)) / 16 as libc::c_int *
                16 as libc::c_int;
        if (*chan).pfnBlockSize.is_some() {
            maxsize_0 =
                (*chan).pfnBlockSize.expect("non-null function pointer")((*chan).client,
                                                                         FRAGSIZE_UNRELIABLE)
        }
        if MSG_GetNumBytesWritten(&mut send) + length >> 3 as libc::c_int <=
               maxsize_0 {
            MSG_WriteBits(&mut send, data as *const libc::c_void, length);
        } else {
            Con_Printf(b"^3Warning:^7 Netchan_Transmit: unreliable message overflow: %d\n\x00"
                           as *const u8 as *const libc::c_char,
                       MSG_GetNumBytesWritten(&mut send));
        }
    }
    // deal with packets that are too small for some networks
    if MSG_GetNumBytesWritten(&mut send) < 16 as libc::c_int &&
           NET_IsLocalAddress((*chan).remote_address) as u64 == 0 {
        // packet too small for some networks
        // go ahead and pad a full 16 extra bytes -- this only happens during authentication / signon
        i = MSG_GetNumBytesWritten(&mut send);
        while i < 16 as libc::c_int {
            if (*chan).sock as libc::c_uint ==
                   NS_CLIENT as libc::c_int as libc::c_uint {
                MSG_WriteCmdExt(&mut send, 1 as libc::c_int, NS_CLIENT,
                                0 as *const libc::c_char);
            } else {
                if !((*chan).sock as libc::c_uint ==
                         NS_SERVER as libc::c_int as libc::c_uint) {
                    break ;
                }
                MSG_WriteCmdExt(&mut send, 1 as libc::c_int, NS_SERVER,
                                0 as *const libc::c_char);
            }
            i += 1
        }
    }
    statId =
        ((*chan).flow[0 as libc::c_int as usize].current &
             32 as libc::c_int - 1 as libc::c_int) as uint;
    (*chan).flow[0 as libc::c_int as usize].stats[statId as usize].size =
        MSG_GetNumBytesWritten(&mut send) + 28 as libc::c_int;
    (*chan).flow[0 as libc::c_int as usize].stats[statId as usize].time =
        host.realtime;
    (*chan).flow[0 as libc::c_int as usize].totalbytes +=
        (*chan).flow[0 as libc::c_int as usize].stats[statId as usize].size;
    (*chan).flow[0 as libc::c_int as usize].current += 1;
    Netchan_UpdateFlow(chan);
    (*chan).total_sended =
        ((*chan).total_sended as
             libc::c_ulong).wrapping_add(MSG_GetNumBytesWritten(&mut send) as
                                             libc::c_ulong) as size_t as
            size_t;
    // send the datagram
    if CL_IsPlaybackDemo() as u64 == 0 {
        let mut splitsize: libc::c_int = 0 as libc::c_int;
        if (*chan).pfnBlockSize.is_some() {
            splitsize =
                (*chan).pfnBlockSize.expect("non-null function pointer")((*chan).client,
                                                                         FRAGSIZE_SPLIT)
        }
        NET_SendPacketEx((*chan).sock,
                         MSG_GetNumBytesWritten(&mut send) as size_t,
                         MSG_GetData(&mut send) as *const libc::c_void,
                         (*chan).remote_address, splitsize as size_t);
    }
    if SV_Active() as libc::c_uint != 0 && sv_lan.value != 0. &&
           sv_lan_rate.value > 1000.0f32 {
        fRate = 1.0f32 / sv_lan_rate.value
    } else {
        fRate = (1.0f32 as libc::c_double / (*chan).rate) as libc::c_float
    }
    if (*chan).cleartime < host.realtime { (*chan).cleartime = host.realtime }
    (*chan).cleartime +=
        ((MSG_GetNumBytesWritten(&mut send) + 28 as libc::c_int) as
             libc::c_float * fRate) as libc::c_double;
    if (*net_showpackets).value != 0. && (*net_showpackets).value != 2.0f32 {
        Con_Printf(b" %s --> sz=%i seq=%i ack=%i rel=%i tm=%f\n\x00" as
                       *const u8 as *const libc::c_char,
                   ns_strings[(*chan).sock as usize],
                   MSG_GetNumBytesWritten(&mut send),
                   (*chan).outgoing_sequence.wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint) &
                       63 as libc::c_int as libc::c_uint,
                   (*chan).incoming_sequence &
                       63 as libc::c_int as libc::c_uint,
                   if send_reliable as libc::c_uint != 0 {
                       1 as libc::c_int
                   } else { 0 as libc::c_int },
                   host.realtime as libc::c_float as libc::c_double);
    };
}
/*
===============
Netchan_Transmit

tries to send an unreliable message to a connection, and handles the
transmition / retransmition of the reliable messages.

A 0 length will still generate a packet and deal with the reliable messages.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_Transmit(mut chan: *mut netchan_t,
                                          mut lengthInBytes: libc::c_int,
                                          mut data: *mut byte) {
    Netchan_TransmitBits(chan, lengthInBytes << 3 as libc::c_int, data);
}
/*
=================
Netchan_Process

called when the current net_message is from remote_address
modifies net_message so that it points to the packet payload
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Netchan_Process(mut chan: *mut netchan_t,
                                         mut msg: *mut sizebuf_t)
 -> qboolean {
    let mut sequence: uint = 0;
    let mut sequence_ack: uint = 0;
    let mut reliable_ack: uint = 0;
    let mut reliable_message: uint = 0;
    let mut fragid: [uint; 2] =
        [0 as libc::c_int as uint, 0 as libc::c_int as uint];
    let mut frag_message: [qboolean; 2] = [false_0, false_0];
    let mut frag_offset: [libc::c_int; 2] =
        [0 as libc::c_int, 0 as libc::c_int];
    let mut frag_length: [libc::c_int; 2] =
        [0 as libc::c_int, 0 as libc::c_int];
    let mut message_contains_fragments: qboolean = false_0;
    let mut i: libc::c_int = 0;
    let mut qport: libc::c_int = 0;
    let mut statId: libc::c_int = 0;
    if CL_IsPlaybackDemo() as u64 == 0 &&
           NET_CompareAdr(net_from, (*chan).remote_address) as u64 == 0 {
        return false_0
    }
    // get sequence numbers
    MSG_Clear(msg);
    sequence = MSG_ReadLong(msg) as uint;
    sequence_ack = MSG_ReadLong(msg) as uint;
    // read the qport if we are a server
    if (*chan).sock as libc::c_uint ==
           NS_SERVER as libc::c_int as libc::c_uint {
        qport = MSG_ReadShort(msg)
    }
    reliable_message = sequence >> 31 as libc::c_int;
    reliable_ack = sequence_ack >> 31 as libc::c_int;
    message_contains_fragments =
        if sequence & (1 as libc::c_uint) << 30 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    if message_contains_fragments as u64 != 0 {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if MSG_ReadByte(msg) != 0 {
                frag_message[i as usize] = true_0;
                fragid[i as usize] = MSG_ReadLong(msg) as uint;
                frag_offset[i as usize] = MSG_ReadLong(msg);
                frag_length[i as usize] = MSG_ReadLong(msg)
            }
            i += 1
        }
        if Netchan_Validate(chan, msg, frag_message.as_mut_ptr(),
                            fragid.as_mut_ptr(), frag_offset.as_mut_ptr(),
                            frag_length.as_mut_ptr()) as u64 == 0 {
            return false_0
        }
    }
    sequence &= !((1 as libc::c_uint) << 31 as libc::c_int);
    sequence &= !((1 as libc::c_uint) << 30 as libc::c_int);
    sequence_ack &= !((1 as libc::c_uint) << 30 as libc::c_int);
    sequence_ack &= !((1 as libc::c_uint) << 31 as libc::c_int);
    if (*net_showpackets).value != 0. && (*net_showpackets).value != 3.0f32 {
        Con_Printf(b" %s <-- sz=%i seq=%i ack=%i rel=%i tm=%f\n\x00" as
                       *const u8 as *const libc::c_char,
                   ns_strings[(*chan).sock as usize], MSG_GetMaxBytes(msg),
                   sequence & 63 as libc::c_int as libc::c_uint,
                   sequence_ack & 63 as libc::c_int as libc::c_uint,
                   reliable_message, host.realtime);
    }
    // discard stale or duplicated packets
    if sequence <= (*chan).incoming_sequence {
        if (*net_showdrop).value != 0. {
            let mut adr: *const libc::c_char =
                NET_AdrToString((*chan).remote_address);
            if sequence == (*chan).incoming_sequence {
                Con_Printf(b"%s:duplicate packet %i at %i\n\x00" as *const u8
                               as *const libc::c_char, adr, sequence,
                           (*chan).incoming_sequence);
            } else {
                Con_Printf(b"%s:out of order packet %i at %i\n\x00" as
                               *const u8 as *const libc::c_char, adr,
                           sequence, (*chan).incoming_sequence);
            }
        }
        return false_0
    }
    // dropped packets don't keep the message from being used
    net_drop =
        sequence.wrapping_sub((*chan).incoming_sequence.wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint))
            as libc::c_int;
    if net_drop > 0 as libc::c_int && (*net_showdrop).value != 0. {
        Con_Printf(b"%s:dropped %i packets at %i\n\x00" as *const u8 as
                       *const libc::c_char,
                   NET_AdrToString((*chan).remote_address), net_drop,
                   sequence);
    }
    // if the current outgoing reliable message has been acknowledged
	// clear the buffer to make way for the next
    if reliable_ack == (*chan).reliable_sequence {
        // make sure we actually could have ack'd this message
        if sequence_ack >= (*chan).last_reliable_sequence {
            (*chan).reliable_length = 0 as libc::c_int
            // it has been received
        }
    }
    // if this message contains a reliable message, bump incoming_reliable_sequence
    (*chan).incoming_sequence = sequence;
    (*chan).incoming_acknowledged = sequence_ack;
    (*chan).incoming_reliable_acknowledged = reliable_ack;
    if reliable_message != 0 {
        (*chan).incoming_reliable_sequence ^= 1 as libc::c_int as libc::c_uint
    }
    (*chan).last_received = host.realtime;
    // Update data flow stats
    statId =
        (*chan).flow[1 as libc::c_int as usize].current &
            32 as libc::c_int - 1 as libc::c_int;
    (*chan).flow[1 as libc::c_int as usize].stats[statId as usize].size =
        MSG_GetMaxBytes(msg) + 28 as libc::c_int;
    (*chan).flow[1 as libc::c_int as usize].stats[statId as usize].time =
        host.realtime;
    (*chan).flow[1 as libc::c_int as usize].totalbytes +=
        (*chan).flow[1 as libc::c_int as usize].stats[statId as usize].size;
    (*chan).flow[1 as libc::c_int as usize].current += 1;
    Netchan_UpdateFlow(chan);
    (*chan).total_received =
        ((*chan).total_received as
             libc::c_ulong).wrapping_add(MSG_GetMaxBytes(msg) as
                                             libc::c_ulong) as size_t as
            size_t;
    if message_contains_fragments as u64 != 0 {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut j: libc::c_int = 0;
            let mut inbufferid: libc::c_int = 0;
            let mut intotalbuffers: libc::c_int = 0;
            let mut oldpos: libc::c_int = 0;
            let mut curbit: libc::c_int = 0;
            let mut numbitstoremove: libc::c_int = 0;
            let mut pbuf: *mut fragbuf_t = 0 as *mut fragbuf_t;
            if !(frag_message[i as usize] as u64 == 0) {
                inbufferid =
                    (fragid[i as usize] >> 16 as libc::c_int &
                         0xffff as libc::c_int as libc::c_uint) as
                        libc::c_int;
                intotalbuffers =
                    (fragid[i as usize] &
                         0xffff as libc::c_int as libc::c_uint) as
                        libc::c_int;
                if fragid[i as usize] != 0 as libc::c_int as libc::c_uint {
                    pbuf =
                        Netchan_FindBufferById(&mut *(*chan).incomingbufs.as_mut_ptr().offset(i
                                                                                                  as
                                                                                                  isize),
                                               fragid[i as usize] as
                                                   libc::c_int, true_0);
                    if !pbuf.is_null() {
                        let mut buffer: [byte; 65535] = [0; 65535];
                        let mut bits: libc::c_int = 0;
                        let mut size: libc::c_int = 0;
                        let mut temp: sizebuf_t =
                            sizebuf_t{bOverflow: false_0,
                                      pDebugName: 0 as *const libc::c_char,
                                      pData: 0 as *const byte as *mut byte,
                                      iCurBit: 0,
                                      nDataBits: 0,};
                        size =
                            MSG_GetNumBitsWritten(msg) +
                                frag_offset[i as usize];
                        bits = frag_length[i as usize];
                        // copy in data
                        MSG_Clear(&mut (*pbuf).frag_message);
                        MSG_StartWriting(&mut temp,
                                         (*msg).pData as *mut libc::c_void,
                                         MSG_GetMaxBytes(msg), size,
                                         -(1 as libc::c_int));
                        MSG_ReadBits(&mut temp,
                                     buffer.as_mut_ptr() as *mut libc::c_void,
                                     bits);
                        MSG_WriteBits(&mut (*pbuf).frag_message,
                                      buffer.as_mut_ptr() as
                                          *const libc::c_void, bits);
                    }
                    // count # of incoming bufs we've queued? are we done?
                    Netchan_CheckForCompletion(chan, i, intotalbuffers);
                }
                // rearrange incoming data to not have the frag stuff in the middle of it
                oldpos = MSG_GetNumBitsWritten(msg);
                curbit = MSG_GetNumBitsWritten(msg) + frag_offset[i as usize];
                numbitstoremove = frag_length[i as usize];
                MSG_ExciseBits(msg, curbit, numbitstoremove);
                MSG_SeekToBit(msg, oldpos, 0 as libc::c_int);
                j = i + 1 as libc::c_int;
                while j < 2 as libc::c_int {
                    frag_offset[j as usize] -= frag_length[i as usize];
                    j += 1
                }
            }
            i += 1
        }
        // is there anything left to process?
        if MSG_GetNumBitsLeft(msg) <= 0 as libc::c_int { return false_0 }
    }
    return true_0;
}
