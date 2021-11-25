#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type file_s;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_stristr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Sleep(msec: libc::c_int);
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _Sys_GetParmFromCmdLine(parm: *const libc::c_char,
                               out: *mut libc::c_char, size: size_t)
     -> qboolean;
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut net_showpackets: *mut convar_t;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Write(file: *mut file_t, data: *const libc::c_void,
                datasize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn FS_Rename(oldname: *const libc::c_char, newname: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn CL_ProcessFile(successfully_received: qboolean,
                      filename: *const libc::c_char);
    #[no_mangle]
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn Cmd_AddCommand(cmd_name: *const libc::c_char, function: xcommand_t,
                      cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn LZSS_IsCompressed(source: *const byte) -> qboolean;
    #[no_mangle]
    fn LZSS_GetActualSize(source: *const byte) -> uint;
    #[no_mangle]
    fn LZSS_Decompress(pInput: *const byte, pOutput: *mut byte) -> uint;
    #[no_mangle]
    fn LZSS_Compress(pInput: *mut byte, inputLength: libc::c_int,
                     pOutputSize: *mut uint) -> *mut byte;
    #[no_mangle]
    fn CL_LegacyMode() -> qboolean;
    #[no_mangle]
    fn CL_GetSplitSize() -> libc::c_int;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut net_from: netadr_t;
    #[no_mangle]
    fn recvfrom(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                __flags: libc::c_int, __addr: *mut sockaddr,
                __addr_len: *mut socklen_t) -> ssize_t;
    #[no_mangle]
    fn sendto(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
              __flags: libc::c_int, __addr: *const sockaddr,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getsockname(__fd: libc::c_int, __addr: *mut sockaddr,
                   __len: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn send(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    #[no_mangle]
    fn getaddrinfo(__name: *const libc::c_char,
                   __service: *const libc::c_char, __req: *const addrinfo,
                   __pai: *mut *mut addrinfo) -> libc::c_int;
    #[no_mangle]
    fn freeaddrinfo(__ai: *mut addrinfo);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type socklen_t = __socklen_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct net_state_t {
    pub loopbacks: [net_loopback_t; 2],
    pub lagdata: [packetlag_t; 2],
    pub losscount: [libc::c_int; 2],
    pub fakelag: libc::c_float,
    pub split: LONGPACKET,
    pub split_flags: [libc::c_int; 131],
    pub sequence_number: libc::c_int,
    pub ip_sockets: [libc::c_int; 2],
    pub initialized: qboolean,
    pub threads_initialized: qboolean,
    pub configured: qboolean,
    pub allow_ip: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LONGPACKET {
    pub current_sequence: libc::c_int,
    pub split_count: libc::c_int,
    pub total_size: libc::c_int,
    pub buffer: [libc::c_char; 65535],
}
pub type packetlag_t = packetlag_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packetlag_s {
    pub data: *mut byte,
    pub size: libc::c_int,
    pub from: netadr_t,
    pub receivedtime: libc::c_float,
    pub next: *mut packetlag_s,
    pub prev: *mut packetlag_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_loopback_t {
    pub msgs: [net_loopmsg_t; 4],
    pub get: libc::c_int,
    pub send: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_loopmsg_t {
    pub data: [byte; 131120],
    pub datalen: libc::c_int,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type sa_family_t = libc::c_ushort;
pub type WSAsize_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
// Raw stream data is stored.
// WIN32
// !_WIN32
//  DEBUG_RESOLVE
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nsthread_s {
    pub mutexns: pthread_mutex_t,
    pub mutexres: pthread_mutex_t,
    pub thread: pthread_t,
    pub result: libc::c_int,
    pub hostname: string,
    pub busy: qboolean,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_1 = 0;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const SOCK_DGRAM: __socket_type = 2;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SPLITPACKET {
    pub net_id: libc::c_int,
    pub sequence_number: libc::c_int,
    pub packet_id: libc::c_short,
}
/*
=================================================

HTTP downloader

=================================================
*/
pub type httpserver_t = httpserver_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct httpserver_s {
    pub host: [libc::c_char; 256],
    pub port: libc::c_int,
    pub path: [libc::c_char; 1024],
    pub needfree: qboolean,
    pub next: *mut httpserver_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_static_s {
    pub first_file: *mut httpfile_t,
    pub last_file: *mut httpfile_t,
    pub first_server: *mut httpserver_t,
    pub last_server: *mut httpserver_t,
}
pub type httpfile_t = httpfile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct httpfile_s {
    pub next: *mut httpfile_s,
    pub server: *mut httpserver_t,
    pub path: [libc::c_char; 1024],
    pub file: *mut file_t,
    pub socket: libc::c_int,
    pub size: libc::c_int,
    pub downloaded: libc::c_int,
    pub lastchecksize: libc::c_int,
    pub checktime: libc::c_float,
    pub blocktime: libc::c_float,
    pub id: libc::c_int,
    pub state: connectionstate,
    pub process: qboolean,
    pub buf: [libc::c_char; 8193],
    pub header_size: libc::c_int,
    pub query_length: libc::c_int,
    pub bytes_sent: libc::c_int,
}
pub type connectionstate = libc::c_uint;
pub const HTTP_FREE: connectionstate = 8;
pub const HTTP_RESPONSE_RECEIVED: connectionstate = 7;
pub const HTTP_REQUEST_SENT: connectionstate = 6;
pub const HTTP_REQUEST: connectionstate = 5;
pub const HTTP_CONNECTED: connectionstate = 4;
pub const HTTP_NS_RESOLVED: connectionstate = 3;
pub const HTTP_SOCKET: connectionstate = 2;
pub const HTTP_OPENED: connectionstate = 1;
pub const HTTP_QUEUE: connectionstate = 0;
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const SOCK_STREAM: __socket_type = 1;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_1 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_1 = 1;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
                (__bsx as libc::c_int & 0xff as libc::c_int) <<
                    8 as libc::c_int) as __uint16_t;
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
static mut net: net_state_t =
    net_state_t{loopbacks:
                    [net_loopback_t{msgs:
                                        [net_loopmsg_t{data: [0; 131120],
                                                       datalen: 0,}; 4],
                                    get: 0,
                                    send: 0,}; 2],
                lagdata:
                    [packetlag_t{data: 0 as *const byte as *mut byte,
                                 size: 0,
                                 from:
                                     netadr_t{type_0: NA_UNUSED,
                                              ip: [0; 4],
                                              ipx: [0; 10],
                                              port: 0,},
                                 receivedtime: 0.,
                                 next:
                                     0 as *const packetlag_s as
                                         *mut packetlag_s,
                                 prev:
                                     0 as *const packetlag_s as
                                         *mut packetlag_s,}; 2],
                losscount: [0; 2],
                fakelag: 0.,
                split:
                    LONGPACKET{current_sequence: 0,
                               split_count: 0,
                               total_size: 0,
                               buffer: [0; 65535],},
                split_flags: [0; 131],
                sequence_number: 0,
                ip_sockets: [0; 2],
                initialized: false_0,
                threads_initialized: false_0,
                configured: false_0,
                allow_ip: false_0,};
static mut net_ipname: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut net_hostport: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut net_iphostport: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut net_clientport: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut net_ipclientport: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut net_fakelag: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut net_fakeloss: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut net_address: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut net_clockwindow: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut net_local: netadr_t =
    netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
// query or response
/*
====================
NET_ErrorString
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_ErrorString() -> *mut libc::c_char {
    return strerror(*__errno_location());
}
#[inline]
unsafe extern "C" fn NET_IsSocketError(mut retval: libc::c_int) -> qboolean {
    return if retval < 0 as libc::c_int {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
#[inline]
unsafe extern "C" fn NET_IsSocketValid(mut socket_0: libc::c_int)
 -> qboolean {
    return (socket_0 >= 0 as libc::c_int) as libc::c_int as qboolean;
}
/*
====================
NET_NetadrToSockadr
====================
*/
unsafe extern "C" fn NET_NetadrToSockadr(mut a: *mut netadr_t,
                                         mut s: *mut sockaddr) {
    memset(s as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sockaddr>() as libc::c_ulong);
    if (*a).type_0 as libc::c_uint ==
           NA_BROADCAST as libc::c_int as libc::c_uint {
        (*(s as *mut sockaddr_in)).sin_family =
            2 as libc::c_int as sa_family_t;
        (*(s as *mut sockaddr_in)).sin_port = (*a).port;
        (*(s as *mut sockaddr_in)).sin_addr.s_addr =
            0xffffffff as libc::c_uint
    } else if (*a).type_0 as libc::c_uint ==
                  NA_IP as libc::c_int as libc::c_uint {
        (*(s as *mut sockaddr_in)).sin_family =
            2 as libc::c_int as sa_family_t;
        (*(s as *mut sockaddr_in)).sin_addr.s_addr =
            *(&mut (*a).ip as *mut [libc::c_uchar; 4] as *mut libc::c_int) as
                in_addr_t;
        (*(s as *mut sockaddr_in)).sin_port = (*a).port
    };
}
/*
====================
NET_SockadrToNetAdr
====================
*/
unsafe extern "C" fn NET_SockadrToNetadr(mut s: *mut sockaddr,
                                         mut a: *mut netadr_t) {
    if (*s).sa_family as libc::c_int == 2 as libc::c_int {
        (*a).type_0 = NA_IP;
        *(&mut (*a).ip as *mut [libc::c_uchar; 4] as *mut libc::c_int) =
            (*(s as *mut sockaddr_in)).sin_addr.s_addr as libc::c_int;
        (*a).port = (*(s as *mut sockaddr_in)).sin_port
    };
}
/*
============
NET_GetHostByName
============
*/
#[no_mangle]
pub unsafe extern "C" fn NET_GetHostByName(mut hostname: *const libc::c_char)
 -> libc::c_int {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut cur: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut ip: libc::c_int = 0 as libc::c_int;
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_family = 2 as libc::c_int;
    if getaddrinfo(hostname, 0 as *const libc::c_char, &mut hints, &mut ai) ==
           0 {
        cur = ai;
        while !cur.is_null() {
            if (*cur).ai_family == 2 as libc::c_int {
                ip =
                    *(&mut (*((*cur).ai_addr as *mut sockaddr_in)).sin_addr as
                          *mut in_addr as *mut libc::c_int);
                break ;
            } else { cur = (*cur).ai_next }
        }
        if !ai.is_null() { freeaddrinfo(ai); }
    }
    return ip;
}
#[no_mangle]
pub unsafe extern "C" fn NET_ThreadStart(mut unused: *mut libc::c_void)
 -> *mut libc::c_void {
    NET_ResolveThread();
    return 0 as *mut libc::c_void;
}
static mut nsthread: nsthread_s =
    {
        let mut init =
            nsthread_s{mutexns:
                           pthread_mutex_t{__data:
                                               {
                                                   let mut init =
                                                       __pthread_mutex_s{__lock:
                                                                             0
                                                                                 as
                                                                                 libc::c_int,
                                                                         __count:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint,
                                                                         __owner:
                                                                             0
                                                                                 as
                                                                                 libc::c_int,
                                                                         __nusers:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint,
                                                                         __kind:
                                                                             PTHREAD_MUTEX_TIMED_NP
                                                                                 as
                                                                                 libc::c_int,
                                                                         __spins:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_short,
                                                                         __elision:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_short,
                                                                         __list:
                                                                             {
                                                                                 let mut init =
                                                                                     __pthread_internal_list{__prev:
                                                                                                                 0
                                                                                                                     as
                                                                                                                     *const __pthread_internal_list
                                                                                                                     as
                                                                                                                     *mut __pthread_internal_list,
                                                                                                             __next:
                                                                                                                 0
                                                                                                                     as
                                                                                                                     *const __pthread_internal_list
                                                                                                                     as
                                                                                                                     *mut __pthread_internal_list,};
                                                                                 init
                                                                             },};
                                                   init
                                               },},
                       mutexres:
                           pthread_mutex_t{__data:
                                               {
                                                   let mut init =
                                                       __pthread_mutex_s{__lock:
                                                                             0
                                                                                 as
                                                                                 libc::c_int,
                                                                         __count:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint,
                                                                         __owner:
                                                                             0
                                                                                 as
                                                                                 libc::c_int,
                                                                         __nusers:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint,
                                                                         __kind:
                                                                             PTHREAD_MUTEX_TIMED_NP
                                                                                 as
                                                                                 libc::c_int,
                                                                         __spins:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_short,
                                                                         __elision:
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_short,
                                                                         __list:
                                                                             {
                                                                                 let mut init =
                                                                                     __pthread_internal_list{__prev:
                                                                                                                 0
                                                                                                                     as
                                                                                                                     *const __pthread_internal_list
                                                                                                                     as
                                                                                                                     *mut __pthread_internal_list,
                                                                                                             __next:
                                                                                                                 0
                                                                                                                     as
                                                                                                                     *const __pthread_internal_list
                                                                                                                     as
                                                                                                                     *mut __pthread_internal_list,};
                                                                                 init
                                                                             },};
                                                   init
                                               },},
                       thread: 0,
                       result: 0,
                       hostname: [0; 256],
                       busy: false_0,};
        init
    };
unsafe extern "C" fn NET_ResolveThread() {
    let mut sin_addr: libc::c_int = 0 as libc::c_int;
    sin_addr = NET_GetHostByName(nsthread.hostname.as_mut_ptr());
    (sin_addr) != 0;
    pthread_mutex_lock(&mut nsthread.mutexres);
    nsthread.result = sin_addr;
    nsthread.busy = false_0;
    pthread_mutex_unlock(&mut nsthread.mutexres);
}
// CAN_ASYNC_NS_RESOLVE
/*
=============
NET_StringToAdr

localhost
idnewt
idnewt:28000
192.246.40.70
192.246.40.70:28000
=============
*/
unsafe extern "C" fn NET_StringToSockaddr(mut s: *const libc::c_char,
                                          mut sadr: *mut sockaddr,
                                          mut nonblocking: qboolean)
 -> libc::c_int {
    let mut ip: libc::c_int = 0 as libc::c_int;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: [libc::c_char; 128] = [0; 128];
    if net.initialized as u64 == 0 { return false_0 as libc::c_int }
    memset(sadr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sockaddr>() as libc::c_ulong);
    (*(sadr as *mut sockaddr_in)).sin_family =
        2 as libc::c_int as sa_family_t;
    (*(sadr as *mut sockaddr_in)).sin_port = 0 as libc::c_int as in_port_t;
    Q_strncpy(copy.as_mut_ptr(), s,
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong);
    // strip off a trailing :port if present
    colon = copy.as_mut_ptr();
    while *colon != 0 {
        if *colon as libc::c_int == ':' as i32 {
            *colon = 0 as libc::c_int as libc::c_char;
            (*(sadr as *mut sockaddr_in)).sin_port =
                __bswap_16(Q_atoi(colon.offset(1 as libc::c_int as isize)) as
                               libc::c_short as __uint16_t)
        }
        colon = colon.offset(1)
    }
    if copy[0 as libc::c_int as usize] as libc::c_int >= '0' as i32 &&
           copy[0 as libc::c_int as usize] as libc::c_int <= '9' as i32 {
        *(&mut (*(sadr as *mut sockaddr_in)).sin_addr as *mut in_addr as
              *mut libc::c_int) = inet_addr(copy.as_mut_ptr()) as libc::c_int
    } else {
        let mut asyncfailed: qboolean = true_0;
        if net.threads_initialized as libc::c_uint != 0 &&
               nonblocking as u64 == 0 {
            pthread_mutex_lock(&mut nsthread.mutexres);
            if nsthread.busy as u64 != 0 {
                pthread_mutex_unlock(&mut nsthread.mutexres);
                return 2 as libc::c_int
            }
            if Q_strncmp(copy.as_mut_ptr(), nsthread.hostname.as_mut_ptr(),
                         99999 as libc::c_int) == 0 {
                ip = nsthread.result;
                nsthread.hostname[0 as libc::c_int as usize] =
                    0 as libc::c_int as libc::c_char;
                pthread_detach(nsthread.thread);
            } else {
                Q_strncpy(nsthread.hostname.as_mut_ptr(), copy.as_mut_ptr(),
                          256 as libc::c_int as size_t);
                nsthread.busy = true_0;
                pthread_mutex_unlock(&mut nsthread.mutexres);
                if pthread_create(&mut nsthread.thread,
                                  0 as *const pthread_attr_t,
                                  Some(NET_ThreadStart as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> *mut libc::c_void),
                                  0 as *mut libc::c_void) == 0 {
                    asyncfailed = false_0;
                    return 2 as libc::c_int
                } else {
                    // failed to create thread
                    Con_Reportf(b"^1Error:^7 NET_StringToSockaddr: failed to create thread!\n\x00"
                                    as *const u8 as *const libc::c_char);
                    nsthread.busy = false_0
                }
            }
            pthread_mutex_unlock(&mut nsthread.mutexres);
        }
        // CAN_ASYNC_NS_RESOLVE
        if asyncfailed as u64 != 0 {
            ip = NET_GetHostByName(copy.as_mut_ptr())
        }
        if ip == 0 { return 0 as libc::c_int }
        *(&mut (*(sadr as *mut sockaddr_in)).sin_addr as *mut in_addr as
              *mut libc::c_int) = ip
    }
    return 1 as libc::c_int;
}
/*
====================
NET_AdrToString
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_AdrToString(a: netadr_t) -> *const libc::c_char {
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return b"loopback\x00" as *const u8 as *const libc::c_char
    }
    return va(b"%i.%i.%i.%i:%i\x00" as *const u8 as *const libc::c_char,
              a.ip[0 as libc::c_int as usize] as libc::c_int,
              a.ip[1 as libc::c_int as usize] as libc::c_int,
              a.ip[2 as libc::c_int as usize] as libc::c_int,
              a.ip[3 as libc::c_int as usize] as libc::c_int,
              __bswap_16(a.port) as libc::c_int);
}
/*
====================
NET_BaseAdrToString
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_BaseAdrToString(a: netadr_t)
 -> *const libc::c_char {
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return b"loopback\x00" as *const u8 as *const libc::c_char
    }
    return va(b"%i.%i.%i.%i\x00" as *const u8 as *const libc::c_char,
              a.ip[0 as libc::c_int as usize] as libc::c_int,
              a.ip[1 as libc::c_int as usize] as libc::c_int,
              a.ip[2 as libc::c_int as usize] as libc::c_int,
              a.ip[3 as libc::c_int as usize] as libc::c_int);
}
/*
===================
NET_CompareBaseAdr

Compares without the port
===================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_CompareBaseAdr(a: netadr_t, b: netadr_t)
 -> qboolean {
    if a.type_0 as libc::c_uint != b.type_0 as libc::c_uint { return false_0 }
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return true_0
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if memcmp(a.ip.as_ptr() as *const libc::c_void,
                  b.ip.as_ptr() as *const libc::c_void,
                  4 as libc::c_int as libc::c_ulong) == 0 {
            return true_0
        }
    }
    return false_0;
}
/*
====================
NET_CompareClassBAdr

Compare local masks
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_CompareClassBAdr(mut a: netadr_t,
                                              mut b: netadr_t) -> qboolean {
    if a.type_0 as libc::c_uint != b.type_0 as libc::c_uint { return false_0 }
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return true_0
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if a.ip[0 as libc::c_int as usize] as libc::c_int ==
               b.ip[0 as libc::c_int as usize] as libc::c_int &&
               a.ip[1 as libc::c_int as usize] as libc::c_int ==
                   b.ip[1 as libc::c_int as usize] as libc::c_int {
            return true_0
        }
    }
    return false_0;
}
/*
====================
NET_IsReservedAdr

Check for reserved ip's
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_IsReservedAdr(mut a: netadr_t) -> qboolean {
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return true_0
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if a.ip[0 as libc::c_int as usize] as libc::c_int == 10 as libc::c_int
               ||
               a.ip[0 as libc::c_int as usize] as libc::c_int ==
                   127 as libc::c_int {
            return true_0
        }
        if a.ip[0 as libc::c_int as usize] as libc::c_int ==
               172 as libc::c_int &&
               a.ip[1 as libc::c_int as usize] as libc::c_int >=
                   16 as libc::c_int {
            if a.ip[1 as libc::c_int as usize] as libc::c_int >=
                   32 as libc::c_int {
                return false_0
            }
            return true_0
        }
        if a.ip[0 as libc::c_int as usize] as libc::c_int ==
               192 as libc::c_int &&
               a.ip[1 as libc::c_int as usize] as libc::c_int >=
                   168 as libc::c_int {
            return true_0
        }
    }
    return false_0;
}
/*
====================
NET_CompareAdr

Compare full address
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_CompareAdr(a: netadr_t, b: netadr_t)
 -> qboolean {
    if a.type_0 as libc::c_uint != b.type_0 as libc::c_uint { return false_0 }
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return true_0
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if memcmp(a.ip.as_ptr() as *const libc::c_void,
                  b.ip.as_ptr() as *const libc::c_void,
                  4 as libc::c_int as libc::c_ulong) == 0 &&
               a.port as libc::c_int == b.port as libc::c_int {
            return true_0
        }
        return false_0
    }
    Con_DPrintf(b"^1Error:^7 NET_CompareAdr: bad address type\n\x00" as
                    *const u8 as *const libc::c_char);
    return false_0;
}
/*
====================
NET_IsLocalAddress
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_IsLocalAddress(mut adr: netadr_t) -> qboolean {
    return if adr.type_0 as libc::c_uint ==
                  NA_LOOPBACK as libc::c_int as libc::c_uint {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
=============
NET_StringToAdr

idnewt
192.246.40.70
=============
*/
#[no_mangle]
pub unsafe extern "C" fn NET_StringToAdr(mut string: *const libc::c_char,
                                         mut adr: *mut netadr_t) -> qboolean {
    let mut s: sockaddr = sockaddr{sa_family: 0, sa_data: [0; 14],};
    memset(adr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    if Q_strnicmp(string,
                  b"localhost\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 ||
           Q_strnicmp(string,
                      b"loopback\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        (*adr).type_0 = NA_LOOPBACK;
        return true_0
    }
    if NET_StringToSockaddr(string, &mut s, false_0) == 0 { return false_0 }
    NET_SockadrToNetadr(&mut s, adr);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn NET_StringToAdrNB(mut string: *const libc::c_char,
                                           mut adr: *mut netadr_t)
 -> libc::c_int {
    let mut s: sockaddr = sockaddr{sa_family: 0, sa_data: [0; 14],};
    let mut res: libc::c_int = 0;
    memset(adr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    if Q_strnicmp(string,
                  b"localhost\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 ||
           Q_strnicmp(string,
                      b"loopback\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        (*adr).type_0 = NA_LOOPBACK;
        return true_0 as libc::c_int
    }
    res = NET_StringToSockaddr(string, &mut s, true_0);
    if res == 0 as libc::c_int || res == 2 as libc::c_int { return res }
    NET_SockadrToNetadr(&mut s, adr);
    return true_0 as libc::c_int;
}
/*
=============================================================================

LOOPBACK BUFFERS FOR LOCAL PLAYER

=============================================================================
*/
/*
====================
NET_GetLoopPacket
====================
*/
unsafe extern "C" fn NET_GetLoopPacket(mut sock: netsrc_t,
                                       mut from: *mut netadr_t,
                                       mut data: *mut byte,
                                       mut length: *mut size_t) -> qboolean {
    let mut loop_0: *mut net_loopback_t = 0 as *mut net_loopback_t;
    let mut i: libc::c_int = 0;
    if data.is_null() || length.is_null() { return false_0 }
    loop_0 =
        &mut *net.loopbacks.as_mut_ptr().offset(sock as isize) as
            *mut net_loopback_t;
    if (*loop_0).send - (*loop_0).get > 4 as libc::c_int {
        (*loop_0).get = (*loop_0).send - 4 as libc::c_int
    }
    if (*loop_0).get >= (*loop_0).send { return false_0 }
    i = (*loop_0).get & 4 as libc::c_int - 1 as libc::c_int;
    (*loop_0).get += 1;
    memcpy(data as *mut libc::c_void,
           (*loop_0).msgs[i as usize].data.as_mut_ptr() as
               *const libc::c_void,
           (*loop_0).msgs[i as usize].datalen as libc::c_ulong);
    *length = (*loop_0).msgs[i as usize].datalen as size_t;
    memset(from as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    (*from).type_0 = NA_LOOPBACK;
    return true_0;
}
/*
====================
NET_SendLoopPacket
====================
*/
unsafe extern "C" fn NET_SendLoopPacket(mut sock: netsrc_t,
                                        mut length: size_t,
                                        mut data: *const libc::c_void,
                                        mut to: netadr_t) {
    let mut loop_0: *mut net_loopback_t = 0 as *mut net_loopback_t;
    let mut i: libc::c_int = 0;
    loop_0 =
        &mut *net.loopbacks.as_mut_ptr().offset((sock as libc::c_uint ^
                                                     1 as libc::c_int as
                                                         libc::c_uint) as
                                                    isize) as
            *mut net_loopback_t;
    i = (*loop_0).send & 4 as libc::c_int - 1 as libc::c_int;
    (*loop_0).send += 1;
    memcpy((*loop_0).msgs[i as usize].data.as_mut_ptr() as *mut libc::c_void,
           data, length);
    (*loop_0).msgs[i as usize].datalen = length as libc::c_int;
}
/*
====================
NET_ClearLoopback
====================
*/
unsafe extern "C" fn NET_ClearLoopback() {
    net.loopbacks[0 as libc::c_int as usize].get = 0 as libc::c_int;
    net.loopbacks[0 as libc::c_int as usize].send =
        net.loopbacks[0 as libc::c_int as usize].get;
    net.loopbacks[1 as libc::c_int as usize].get = 0 as libc::c_int;
    net.loopbacks[1 as libc::c_int as usize].send =
        net.loopbacks[1 as libc::c_int as usize].get;
}
/*
=============================================================================

LAG & LOSS SIMULATION SYSTEM (network debugging)

=============================================================================
*/
/*
==================
NET_RemoveFromPacketList

double linked list remove entry
==================
*/
unsafe extern "C" fn NET_RemoveFromPacketList(mut p: *mut packetlag_t) {
    (*(*p).prev).next = (*p).next;
    (*(*p).next).prev = (*p).prev;
    (*p).prev = 0 as *mut packetlag_s;
    (*p).next = 0 as *mut packetlag_s;
}
/*
==================
NET_ClearLaggedList

double linked list remove queue
==================
*/
unsafe extern "C" fn NET_ClearLaggedList(mut list: *mut packetlag_t) {
    let mut p: *mut packetlag_t = 0 as *mut packetlag_t;
    let mut n: *mut packetlag_t = 0 as *mut packetlag_t;
    p = (*list).next;
    while !p.is_null() && p != list {
        n = (*p).next;
        NET_RemoveFromPacketList(p);
        if !(*p).data.is_null() {
            _Mem_Free((*p).data as *mut libc::c_void,
                      b"../engine/common/net_ws.c\x00" as *const u8 as
                          *const libc::c_char, 832 as libc::c_int);
            (*p).data = 0 as *mut byte
        }
        _Mem_Free(p as *mut libc::c_void,
                  b"../engine/common/net_ws.c\x00" as *const u8 as
                      *const libc::c_char, 836 as libc::c_int);
        p = n
    }
    (*list).prev = list;
    (*list).next = list;
}
/*
==================
NET_AddToLagged

add lagged packet to stream
==================
*/
unsafe extern "C" fn NET_AddToLagged(mut sock: netsrc_t,
                                     mut list: *mut packetlag_t,
                                     mut packet: *mut packetlag_t,
                                     mut from: *mut netadr_t,
                                     mut length: size_t,
                                     mut data: *const libc::c_void,
                                     mut timestamp: libc::c_float) {
    let mut pStart: *mut byte = 0 as *mut byte;
    if !(*packet).prev.is_null() || !(*packet).next.is_null() { return }
    (*packet).prev = (*list).prev;
    (*(*list).prev).next = packet;
    (*list).prev = packet;
    (*packet).next = list;
    pStart =
        _Mem_Alloc(host.mempool, length, false_0,
                   b"../engine/common/net_ws.c\x00" as *const u8 as
                       *const libc::c_char, 863 as libc::c_int) as *mut byte;
    memcpy(pStart as *mut libc::c_void, data, length);
    (*packet).data = pStart;
    (*packet).size = length as libc::c_int;
    (*packet).receivedtime = timestamp;
    memcpy(&mut (*packet).from as *mut netadr_t as *mut libc::c_void,
           from as *const libc::c_void,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
}
/*
==================
NET_AdjustLag

adjust time to next fake lag
==================
*/
unsafe extern "C" fn NET_AdjustLag() {
    static mut lasttime: libc::c_double = 0.0f64;
    let mut diff: libc::c_float = 0.;
    let mut converge: libc::c_float = 0.;
    let mut dt: libc::c_double = 0.;
    dt = host.realtime - lasttime;
    dt =
        if dt >= 0.0f64 {
            if dt < 0.1f64 { dt } else { 0.1f64 }
        } else { 0.0f64 };
    lasttime = host.realtime;
    if host_developer.value != 0. || (*net_fakelag).value == 0. {
        if (*net_fakelag).value != net.fakelag {
            diff = (*net_fakelag).value - net.fakelag;
            converge = (dt * 200.0f32 as libc::c_double) as libc::c_float;
            if __tg_fabs(diff) < converge { converge = __tg_fabs(diff) }
            if diff < 0.0f32 { converge = -converge }
            net.fakelag += converge
        }
    } else {
        Con_Printf(b"Server must enable dev-mode to activate fakelag\n\x00" as
                       *const u8 as *const libc::c_char);
        Cvar_SetValue(b"fakelag\x00" as *const u8 as *const libc::c_char,
                      0.0f64 as libc::c_float);
        net.fakelag = 0.0f32
    };
}
/*
==================
NET_LagPacket

add fake lagged packet into rececived message
==================
*/
unsafe extern "C" fn NET_LagPacket(mut newdata: qboolean, mut sock: netsrc_t,
                                   mut from: *mut netadr_t,
                                   mut length: *mut size_t,
                                   mut data: *mut libc::c_void) -> qboolean {
    let mut pNewPacketLag: *mut packetlag_t = 0 as *mut packetlag_t;
    let mut pPacket: *mut packetlag_t = 0 as *mut packetlag_t;
    let mut ninterval: libc::c_int = 0;
    let mut curtime: libc::c_float = 0.;
    if net.fakelag <= 0.0f32 {
        NET_ClearLagData(true_0, true_0);
        return newdata
    }
    curtime = host.realtime as libc::c_float;
    if newdata as u64 != 0 {
        if (*net_fakeloss).value != 0.0f32 {
            if host_developer.value != 0. {
                net.losscount[sock as usize] += 1;
                if (*net_fakeloss).value <= 0.0f32 {
                    ninterval =
                        __tg_fabs((*net_fakeloss).value) as libc::c_int;
                    if ninterval < 2 as libc::c_int {
                        ninterval = 2 as libc::c_int
                    }
                    if net.losscount[sock as usize] % ninterval ==
                           0 as libc::c_int {
                        return false_0
                    }
                } else if COM_RandomLong(0 as libc::c_int, 100 as libc::c_int)
                              as libc::c_float <= (*net_fakeloss).value {
                    return false_0
                }
            } else {
                Cvar_SetValue(b"fakeloss\x00" as *const u8 as
                                  *const libc::c_char,
                              0.0f64 as libc::c_float);
            }
        }
        pNewPacketLag =
            _Mem_Alloc(host.mempool,
                       ::std::mem::size_of::<packetlag_t>() as libc::c_ulong,
                       false_0,
                       b"../engine/common/net_ws.c\x00" as *const u8 as
                           *const libc::c_char, 958 as libc::c_int) as
                *mut packetlag_t;
        // queue packet to simulate fake lag
        NET_AddToLagged(sock,
                        &mut *net.lagdata.as_mut_ptr().offset(sock as isize),
                        pNewPacketLag, from, *length, data, curtime);
    }
    pPacket = net.lagdata[sock as usize].next;
    while pPacket !=
              &mut *net.lagdata.as_mut_ptr().offset(sock as isize) as
                  *mut packetlag_t {
        if (*pPacket).receivedtime <= curtime - net.fakelag / 1000.0f32 {
            break ;
        }
        pPacket = (*pPacket).next
    }
    if pPacket ==
           &mut *net.lagdata.as_mut_ptr().offset(sock as isize) as
               *mut packetlag_t {
        return false_0
    }
    NET_RemoveFromPacketList(pPacket);
    // delivery packet from fake lag queue
    memcpy(data, (*pPacket).data as *const libc::c_void,
           (*pPacket).size as libc::c_ulong);
    memcpy(&mut net_from as *mut netadr_t as *mut libc::c_void,
           &mut (*pPacket).from as *mut netadr_t as *const libc::c_void,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    *length = (*pPacket).size as size_t;
    if !(*pPacket).data.is_null() {
        _Mem_Free((*pPacket).data as *mut libc::c_void,
                  b"../engine/common/net_ws.c\x00" as *const u8 as
                      *const libc::c_char, 984 as libc::c_int);
    }
    _Mem_Free(pPacket as *mut libc::c_void,
              b"../engine/common/net_ws.c\x00" as *const u8 as
                  *const libc::c_char, 986 as libc::c_int);
    return true_0;
}
/*
==================
NET_GetLong

receive long packet from network
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_GetLong(mut pData: *mut byte,
                                     mut size: libc::c_int,
                                     mut outSize: *mut size_t,
                                     mut splitsize: libc::c_int) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut sequence_number: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut pHeader: *mut SPLITPACKET = pData as *mut SPLITPACKET;
    let mut packet_number: libc::c_int = 0;
    let mut packet_count: libc::c_int = 0;
    let mut packet_id: libc::c_short = 0;
    let mut body_size: libc::c_int =
        (splitsize as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<SPLITPACKET>()
                                             as libc::c_ulong) as libc::c_int;
    if body_size < 0 as libc::c_int { return false_0 }
    if (size as libc::c_ulong) <
           ::std::mem::size_of::<SPLITPACKET>() as libc::c_ulong {
        Con_Printf(b"^1Error:^7 invalid split packet length %i\n\x00" as
                       *const u8 as *const libc::c_char, size);
        return false_0
    }
    sequence_number = (*pHeader).sequence_number;
    packet_id = (*pHeader).packet_id;
    packet_count = packet_id as libc::c_int & 0xff as libc::c_int;
    packet_number = packet_id as libc::c_int >> 8 as libc::c_int;
    if packet_number as libc::c_ulong >=
           (65535 as libc::c_int as
                libc::c_ulong).wrapping_div((508 as libc::c_int as
                                                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<SPLITPACKET>()
                                                                                 as
                                                                                 libc::c_ulong))
           ||
           packet_count as libc::c_ulong >
               (65535 as libc::c_int as
                    libc::c_ulong).wrapping_div((508 as libc::c_int as
                                                     libc::c_ulong).wrapping_sub(::std::mem::size_of::<SPLITPACKET>()
                                                                                     as
                                                                                     libc::c_ulong))
       {
        Con_Printf(b"^1Error:^7 malformed packet number (%i/%i)\n\x00" as
                       *const u8 as *const libc::c_char,
                   packet_number + 1 as libc::c_int, packet_count);
        return false_0
    }
    if net.split.current_sequence == -(1 as libc::c_int) ||
           sequence_number != net.split.current_sequence {
        net.split.current_sequence = sequence_number;
        net.split.split_count = packet_count;
        net.split.total_size = 0 as libc::c_int;
        // clear part's sequence
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) <
                  (65535 as libc::c_int as
                       libc::c_ulong).wrapping_div((508 as libc::c_int as
                                                        libc::c_ulong).wrapping_sub(::std::mem::size_of::<SPLITPACKET>()
                                                                                        as
                                                                                        libc::c_ulong))
              {
            net.split_flags[i as usize] = -(1 as libc::c_int);
            i += 1
        }
        if !net_showpackets.is_null() && (*net_showpackets).value == 4.0f32 {
            Con_Printf(b"<-- Split packet restart %i count %i seq\n\x00" as
                           *const u8 as *const libc::c_char,
                       net.split.split_count, sequence_number);
        }
    }
    size =
        (size as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<SPLITPACKET>()
                                             as libc::c_ulong) as libc::c_int
            as libc::c_int;
    if net.split_flags[packet_number as usize] != sequence_number {
        if packet_number == packet_count - 1 as libc::c_int {
            net.split.total_size =
                size + body_size * (packet_count - 1 as libc::c_int)
        }
        net.split.split_count -= 1;
        net.split_flags[packet_number as usize] = sequence_number;
        if !net_showpackets.is_null() && (*net_showpackets).value == 4.0f32 {
            Con_Printf(b"<-- Split packet %i of %i, %i bytes %i seq\n\x00" as
                           *const u8 as *const libc::c_char,
                       packet_number + 1 as libc::c_int, packet_count, size,
                       sequence_number);
        }
    } else {
        Con_DPrintf(b"NET_GetLong: Ignoring duplicated split packet %i of %i ( %i bytes )\n\x00"
                        as *const u8 as *const libc::c_char,
                    packet_number + 1 as libc::c_int, packet_count, size);
    }
    offset = packet_number * body_size;
    memcpy(net.split.buffer.as_mut_ptr().offset(offset as isize) as
               *mut libc::c_void,
           pData.offset(::std::mem::size_of::<SPLITPACKET>() as libc::c_ulong
                            as isize) as *const libc::c_void,
           size as libc::c_ulong);
    // have we received all of the pieces to the packet?
    if net.split.split_count <= 0 as libc::c_int {
        net.split.current_sequence = -(1 as libc::c_int); // Clear packet
        if net.split.total_size as libc::c_ulong >
               ::std::mem::size_of::<[libc::c_char; 65535]>() as libc::c_ulong
           {
            Con_Printf(b"Split packet too large! %d bytes\n\x00" as *const u8
                           as *const libc::c_char, net.split.total_size);
            return false_0
        }
        memcpy(pData as *mut libc::c_void,
               net.split.buffer.as_mut_ptr() as *const libc::c_void,
               net.split.total_size as libc::c_ulong);
        *outSize = net.split.total_size as size_t;
        return true_0
    }
    return false_0;
}
/*
==================
NET_QueuePacket

queue normal and lagged packets
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_QueuePacket(mut sock: netsrc_t,
                                         mut from: *mut netadr_t,
                                         mut data: *mut byte,
                                         mut length: *mut size_t)
 -> qboolean {
    let mut buf: [byte; 65535] = [0; 65535];
    let mut ret: libc::c_int = 0;
    let mut net_socket: libc::c_int = 0;
    let mut addr_len: WSAsize_t = 0;
    let mut addr: sockaddr = sockaddr{sa_family: 0, sa_data: [0; 14],};
    *length = 0 as libc::c_int as size_t;
    net_socket = net.ip_sockets[sock as usize];
    if NET_IsSocketValid(net_socket) as u64 != 0 {
        addr_len =
            ::std::mem::size_of::<sockaddr>() as libc::c_ulong as WSAsize_t;
        ret =
            recvfrom(net_socket, buf.as_mut_ptr() as *mut libc::c_void,
                     ::std::mem::size_of::<[byte; 65535]>() as libc::c_ulong,
                     0 as libc::c_int, &mut addr as *mut sockaddr,
                     &mut addr_len as *mut WSAsize_t as *mut socklen_t) as
                libc::c_int;
        if NET_IsSocketError(ret) as u64 == 0 {
            NET_SockadrToNetadr(&mut addr, from);
            if ret < 65535 as libc::c_int {
                // Transfer data
                memcpy(data as *mut libc::c_void,
                       buf.as_mut_ptr() as *const libc::c_void,
                       ret as libc::c_ulong);
                *length = ret as size_t;
                if CL_LegacyMode() as u64 != 0 {
                    return NET_LagPacket(true_0, sock, from, length,
                                         data as *mut libc::c_void)
                }
                // check for split message
                if sock as libc::c_uint ==
                       NS_CLIENT as libc::c_int as libc::c_uint &&
                       *(data as *mut libc::c_int) == -(2 as libc::c_int) {
                    return NET_GetLong(data, ret, length, CL_GetSplitSize())
                }
                // lag the packet, if needed
                return NET_LagPacket(true_0, sock, from, length,
                                     data as *mut libc::c_void)
            } else {
                Con_Reportf(b"NET_QueuePacket: oversize packet from %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            NET_AdrToString(*from));
            }
        } else {
            let mut err: libc::c_int = *__errno_location();
            match err {
                11 | 104 | 111 | 90 | 110 => { }
                _ => {
                    // let's continue even after errors
                    Con_DPrintf(b"^1Error:^7 NET_QueuePacket: %s from %s\n\x00"
                                    as *const u8 as *const libc::c_char,
                                NET_ErrorString(), NET_AdrToString(*from));
                }
            }
        }
    }
    return NET_LagPacket(false_0, sock, from, length,
                         data as *mut libc::c_void);
}
/*
==================
NET_GetPacket

Never called by the game logic, just the system event queing
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_GetPacket(mut sock: netsrc_t,
                                       mut from: *mut netadr_t,
                                       mut data: *mut byte,
                                       mut length: *mut size_t) -> qboolean {
    if data.is_null() || length.is_null() { return false_0 }
    NET_AdjustLag();
    if NET_GetLoopPacket(sock, from, data, length) as u64 != 0 {
        return NET_LagPacket(true_0, sock, from, length,
                             data as *mut libc::c_void)
    } else { return NET_QueuePacket(sock, from, data, length) };
}
/*
==================
NET_SendLong

Fragment long packets, send short directly
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_SendLong(mut sock: netsrc_t,
                                      mut net_socket: libc::c_int,
                                      mut buf: *const libc::c_char,
                                      mut len: size_t, mut flags: libc::c_int,
                                      mut to: *const sockaddr,
                                      mut tolen: size_t,
                                      mut splitsize: size_t) -> libc::c_int {
    // do we need to break this packet up?
    if splitsize > ::std::mem::size_of::<SPLITPACKET>() as libc::c_ulong &&
           sock as libc::c_uint == NS_SERVER as libc::c_int as libc::c_uint &&
           len > splitsize {
        let mut packet: [libc::c_char; 64000] = [0; 64000]; // error
        let mut total_sent: libc::c_int = 0;
        let mut size: libc::c_int = 0;
        let mut packet_count: libc::c_int = 0;
        let mut ret: libc::c_int = 0;
        let mut packet_number: libc::c_int = 0;
        let mut body_size: libc::c_int =
            splitsize.wrapping_sub(::std::mem::size_of::<SPLITPACKET>() as
                                       libc::c_ulong) as libc::c_int;
        let mut pPacket: *mut SPLITPACKET = 0 as *mut SPLITPACKET;
        net.sequence_number += 1;
        if net.sequence_number <= 0 as libc::c_int {
            net.sequence_number = 1 as libc::c_int
        }
        pPacket = packet.as_mut_ptr() as *mut SPLITPACKET;
        (*pPacket).sequence_number = net.sequence_number;
        (*pPacket).net_id = -(2 as libc::c_int);
        packet_number = 0 as libc::c_int;
        total_sent = 0 as libc::c_int;
        packet_count =
            len.wrapping_add(body_size as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_div(body_size
                                                                                                 as
                                                                                                 libc::c_ulong)
                as libc::c_int;
        while len > 0 as libc::c_int as libc::c_ulong {
            size =
                if (body_size as libc::c_ulong) < len {
                    body_size as libc::c_ulong
                } else { len } as libc::c_int;
            (*pPacket).packet_id =
                ((packet_number << 8 as libc::c_int) + packet_count) as
                    libc::c_short;
            memcpy(packet.as_mut_ptr().offset(::std::mem::size_of::<SPLITPACKET>()
                                                  as libc::c_ulong as isize)
                       as *mut libc::c_void,
                   buf.offset((packet_number * body_size) as isize) as
                       *const libc::c_void, size as libc::c_ulong);
            if !net_showpackets.is_null() &&
                   (*net_showpackets).value == 3.0f32 {
                let mut adr: netadr_t =
                    netadr_t{type_0: NA_UNUSED,
                             ip: [0; 4],
                             ipx: [0; 10],
                             port: 0,};
                memset(&mut adr as *mut netadr_t as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
                NET_SockadrToNetadr(to as *mut sockaddr, &mut adr);
                Con_Printf(b"Sending split %i of %i with %i bytes and seq %i to %s\n\x00"
                               as *const u8 as *const libc::c_char,
                           packet_number + 1 as libc::c_int, packet_count,
                           size, net.sequence_number, NET_AdrToString(adr));
            }
            ret =
                sendto(net_socket, packet.as_mut_ptr() as *const libc::c_void,
                       (size as
                            libc::c_ulong).wrapping_add(::std::mem::size_of::<SPLITPACKET>()
                                                            as libc::c_ulong),
                       flags, to, tolen as socklen_t) as libc::c_int;
            if ret < 0 as libc::c_int { return ret }
            if ret >= size { total_sent += size }
            len =
                (len as libc::c_ulong).wrapping_sub(size as libc::c_ulong) as
                    size_t as size_t;
            packet_number += 1;
            Sys_Sleep(1 as libc::c_int);
        }
        return total_sent
    } else {
        // no fragmenantion for client connection
        return sendto(net_socket, buf as *const libc::c_void, len, flags, to,
                      tolen as socklen_t) as libc::c_int
    };
}
/*
==================
NET_SendPacketEx
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_SendPacketEx(mut sock: netsrc_t,
                                          mut length: size_t,
                                          mut data: *const libc::c_void,
                                          mut to: netadr_t,
                                          mut splitsize: size_t) {
    let mut ret: libc::c_int = 0;
    let mut addr: sockaddr = sockaddr{sa_family: 0, sa_data: [0; 14],};
    let mut net_socket: libc::c_int = 0 as libc::c_int;
    if net.initialized as u64 == 0 ||
           to.type_0 as libc::c_uint ==
               NA_LOOPBACK as libc::c_int as libc::c_uint {
        NET_SendLoopPacket(sock, length, data, to);
        return
    } else {
        if to.type_0 as libc::c_uint ==
               NA_BROADCAST as libc::c_int as libc::c_uint {
            net_socket = net.ip_sockets[sock as usize];
            if NET_IsSocketValid(net_socket) as u64 == 0 { return }
        } else if to.type_0 as libc::c_uint ==
                      NA_IP as libc::c_int as libc::c_uint {
            net_socket = net.ip_sockets[sock as usize];
            if NET_IsSocketValid(net_socket) as u64 == 0 { return }
        } else {
            Host_Error(b"NET_SendPacket: bad address type %i\n\x00" as
                           *const u8 as *const libc::c_char,
                       to.type_0 as libc::c_uint);
        }
    }
    NET_NetadrToSockadr(&mut to, &mut addr);
    ret =
        NET_SendLong(sock, net_socket, data as *const libc::c_char, length,
                     0 as libc::c_int, &mut addr,
                     ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
                     splitsize);
    if NET_IsSocketError(ret) as u64 != 0 {
        let mut err: libc::c_int = *__errno_location();
        // WSAEWOULDBLOCK is silent
        if err == 11 as libc::c_int { return }
        // some PPP links don't allow broadcasts
        if err == 99 as libc::c_int &&
               to.type_0 as libc::c_uint ==
                   NA_BROADCAST as libc::c_int as libc::c_uint {
            return
        }
        if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
            Con_DPrintf(b"^1Error:^7 NET_SendPacket: %s to %s\n\x00" as
                            *const u8 as *const libc::c_char,
                        NET_ErrorString(), NET_AdrToString(to));
        } else if err == 99 as libc::c_int || err == 105 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 NET_SendPacket: %s to %s\n\x00" as
                            *const u8 as *const libc::c_char,
                        NET_ErrorString(), NET_AdrToString(to));
        } else {
            Con_Printf(b"^1Error:^7 NET_SendPacket: %s to %s\n\x00" as
                           *const u8 as *const libc::c_char,
                       NET_ErrorString(), NET_AdrToString(to));
        }
    };
}
/*
==================
NET_SendPacket
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_SendPacket(mut sock: netsrc_t,
                                        mut length: size_t,
                                        mut data: *const libc::c_void,
                                        mut to: netadr_t) {
    NET_SendPacketEx(sock, length, data, to, 0 as libc::c_int as size_t);
}
/*
====================
NET_BufferToBufferCompress

generic fast compression
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_BufferToBufferCompress(mut dest: *mut byte,
                                                    mut destLen: *mut uint,
                                                    mut source: *mut byte,
                                                    mut sourceLen: uint)
 -> qboolean {
    let mut uCompressedLen: uint = 0 as libc::c_int as uint;
    let mut pbOut: *mut byte = 0 as *mut byte;
    memcpy(dest as *mut libc::c_void, source as *const libc::c_void,
           sourceLen as libc::c_ulong);
    pbOut =
        LZSS_Compress(source, sourceLen as libc::c_int, &mut uCompressedLen);
    if !pbOut.is_null() && uCompressedLen > 0 as libc::c_int as libc::c_uint
           && uCompressedLen <= *destLen {
        memcpy(dest as *mut libc::c_void, pbOut as *const libc::c_void,
               uCompressedLen as libc::c_ulong);
        *destLen = uCompressedLen;
        free(pbOut as *mut libc::c_void);
        return true_0
    } else {
        if !pbOut.is_null() { free(pbOut as *mut libc::c_void); }
        memcpy(dest as *mut libc::c_void, source as *const libc::c_void,
               sourceLen as libc::c_ulong);
        *destLen = sourceLen;
        return false_0
    };
}
/*
====================
NET_BufferToBufferDecompress

generic fast decompression
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_BufferToBufferDecompress(mut dest: *mut byte,
                                                      mut destLen: *mut uint,
                                                      mut source: *mut byte,
                                                      mut sourceLen: uint)
 -> qboolean {
    if LZSS_IsCompressed(source) as u64 != 0 {
        let mut uDecompressedLen: uint = LZSS_GetActualSize(source);
        if uDecompressedLen <= *destLen {
            *destLen = LZSS_Decompress(source, dest)
        } else { return false_0 }
    } else {
        memcpy(dest as *mut libc::c_void, source as *const libc::c_void,
               sourceLen as libc::c_ulong);
        *destLen = sourceLen
    }
    return true_0;
}
/*
====================
NET_Isocket
====================
*/
unsafe extern "C" fn NET_Isocket(mut net_interface: *const libc::c_char,
                                 mut port: libc::c_int,
                                 mut multicast: qboolean) -> libc::c_int {
    let mut addr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut err: libc::c_int = 0;
    let mut net_socket: libc::c_int = 0;
    let mut optval: uint = 1 as libc::c_int as uint;
    let mut _true: dword = 1 as libc::c_int as dword;
    net_socket =
        socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int,
               IPPROTO_UDP as libc::c_int);
    if NET_IsSocketError(net_socket) as u64 != 0 {
        err = *__errno_location();
        if err != 97 as libc::c_int {
            Con_DPrintf(b"^3Warning:^7 NET_UDsocket: port: %d socket: %s\n\x00"
                            as *const u8 as *const libc::c_char, port,
                        NET_ErrorString());
        }
        return -(1 as libc::c_int)
    }
    if NET_IsSocketError(ioctl(net_socket,
                               0x5421 as libc::c_int as libc::c_ulong,
                               &mut _true as *mut dword as *mut libc::c_void))
           as u64 != 0 {
        let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
        Con_DPrintf(b"^3Warning:^7 NET_UDsocket: port: %d ioctl FIONBIO: %s\n\x00"
                        as *const u8 as *const libc::c_char, port,
                    NET_ErrorString());
        // try timeout instead of NBIO
        timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
        timeout.tv_sec = timeout.tv_usec;
        setsockopt(net_socket, 1 as libc::c_int, 20 as libc::c_int,
                   &mut timeout as *mut timeval as *mut libc::c_char as
                       *const libc::c_void,
                   ::std::mem::size_of::<timeval>() as libc::c_ulong as
                       socklen_t);
    }
    // make it broadcast capable
    if NET_IsSocketError(setsockopt(net_socket, 1 as libc::c_int,
                                    6 as libc::c_int,
                                    &mut _true as *mut dword as
                                        *mut libc::c_char as
                                        *const libc::c_void,
                                    ::std::mem::size_of::<dword>() as
                                        libc::c_ulong as socklen_t)) as u64 !=
           0 {
        Con_DPrintf(b"^3Warning:^7 NET_UDsocket: port: %d setsockopt SO_BROADCAST: %s\n\x00"
                        as *const u8 as *const libc::c_char, port,
                    NET_ErrorString());
    }
    if Sys_CheckParm(b"-reuse\x00" as *const u8 as *const libc::c_char) != 0
           || multicast as libc::c_uint != 0 {
        if NET_IsSocketError(setsockopt(net_socket, 1 as libc::c_int,
                                        2 as libc::c_int,
                                        &mut optval as *mut uint as
                                            *const libc::c_char as
                                            *const libc::c_void,
                                        ::std::mem::size_of::<uint>() as
                                            libc::c_ulong as socklen_t)) as
               u64 != 0 {
            Con_DPrintf(b"^3Warning:^7 NET_UDsocket: port: %d setsockopt SO_REUSEADDR: %s\n\x00"
                            as *const u8 as *const libc::c_char, port,
                        NET_ErrorString());
            close(net_socket);
            return -(1 as libc::c_int)
        }
    }
    if Sys_CheckParm(b"-tos\x00" as *const u8 as *const libc::c_char) != 0 {
        optval = 16 as libc::c_int as uint;
        Con_Printf(b"Enabling LOWDELAY TOS option\n\x00" as *const u8 as
                       *const libc::c_char);
        if NET_IsSocketError(setsockopt(net_socket, IPPROTO_IP as libc::c_int,
                                        1 as libc::c_int,
                                        &mut optval as *mut uint as
                                            *const libc::c_char as
                                            *const libc::c_void,
                                        ::std::mem::size_of::<uint>() as
                                            libc::c_ulong as socklen_t)) as
               u64 != 0 {
            err = *__errno_location();
            if err != 92 as libc::c_int {
                Con_Printf(b"^3Warning:^7 NET_UDsocket: port: %d  setsockopt IP_TOS: %s\n\x00"
                               as *const u8 as *const libc::c_char, port,
                           NET_ErrorString());
            }
            close(net_socket);
            return -(1 as libc::c_int)
        }
    }
    if (if *net_interface == 0 { 0 as libc::c_int } else { 1 as libc::c_int })
           == 0 ||
           Q_strnicmp(net_interface,
                      b"localhost\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        addr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t
    } else {
        NET_StringToSockaddr(net_interface,
                             &mut addr as *mut sockaddr_in as *mut sockaddr,
                             false_0);
    }
    if port == -(1 as libc::c_int) {
        addr.sin_port = 0 as libc::c_int as in_port_t
    } else { addr.sin_port = __bswap_16(port as libc::c_short as __uint16_t) }
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    if NET_IsSocketError(bind(net_socket,
                              &mut addr as *mut sockaddr_in as
                                  *mut libc::c_void as *const sockaddr,
                              ::std::mem::size_of::<sockaddr_in>() as
                                  libc::c_ulong as socklen_t)) as u64 != 0 {
        Con_DPrintf(b"^3Warning:^7 NET_UDsocket: port: %d bind: %s\n\x00" as
                        *const u8 as *const libc::c_char, port,
                    NET_ErrorString());
        close(net_socket);
        return -(1 as libc::c_int)
    }
    if Sys_CheckParm(b"-loopback\x00" as *const u8 as *const libc::c_char) !=
           0 {
        optval = 1 as libc::c_int as uint;
        if NET_IsSocketError(setsockopt(net_socket, IPPROTO_IP as libc::c_int,
                                        34 as libc::c_int,
                                        &mut optval as *mut uint as
                                            *const libc::c_char as
                                            *const libc::c_void,
                                        ::std::mem::size_of::<uint>() as
                                            libc::c_ulong as socklen_t)) as
               u64 != 0 {
            Con_DPrintf(b"^3Warning:^7 NET_UDsocket: port %d setsockopt IP_MULTICAST_LOOP: %s\n\x00"
                            as *const u8 as *const libc::c_char, port,
                        NET_ErrorString());
        }
    }
    return net_socket;
}
/*
====================
NET_OpenIP
====================
*/
unsafe extern "C" fn NET_OpenIP() {
    let mut port: libc::c_int = 0; // forcing to default
    let mut sv_port: libc::c_int = 0 as libc::c_int;
    let mut cl_port: libc::c_int = 0 as libc::c_int;
    if NET_IsSocketValid(net.ip_sockets[NS_SERVER as libc::c_int as usize]) as
           u64 == 0 {
        port = (*net_iphostport).value as libc::c_int;
        if port == 0 { port = (*net_hostport).value as libc::c_int }
        if port == 0 { port = 27015 as libc::c_int }
        net.ip_sockets[NS_SERVER as libc::c_int as usize] =
            NET_Isocket((*net_ipname).string, port, false_0);
        if NET_IsSocketValid(net.ip_sockets[NS_SERVER as libc::c_int as
                                                usize]) as u64 == 0 &&
               host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
            Host_Error(b"Couldn\'t allocate dedicated server IP port %d.\n\x00"
                           as *const u8 as *const libc::c_char, port);
        }
        sv_port = port
    }
    // dedicated servers don't need client ports
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        return
    } // forcing to default
    if NET_IsSocketValid(net.ip_sockets[NS_CLIENT as libc::c_int as usize]) as
           u64 == 0 {
        port = (*net_ipclientport).value as libc::c_int;
        if port == 0 { port = (*net_clientport).value as libc::c_int }
        if port == 0 { port = -(1 as libc::c_int) }
        net.ip_sockets[NS_CLIENT as libc::c_int as usize] =
            NET_Isocket((*net_ipname).string, port, false_0);
        if NET_IsSocketValid(net.ip_sockets[NS_CLIENT as libc::c_int as
                                                usize]) as u64 == 0 {
            net.ip_sockets[NS_CLIENT as libc::c_int as usize] =
                NET_Isocket((*net_ipname).string, -(1 as libc::c_int),
                            false_0)
        }
        cl_port = port
    };
}
/*
================
NET_GetLocalAddress

Returns the servers' ip address as a string.
================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_GetLocalAddress() {
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut address: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut namelen: WSAsize_t = 0;
    memset(&mut net_local as *mut netadr_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    buff[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if net.allow_ip as u64 != 0 {
        // If we have changed the ip var from the command line, use that instead.
        if Q_strncmp((*net_ipname).string,
                     b"localhost\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) != 0 {
            Q_strncpy(buff.as_mut_ptr(), (*net_ipname).string,
                      ::std::mem::size_of::<[libc::c_char; 512]>() as
                          libc::c_ulong);
        } else {
            gethostname(buff.as_mut_ptr(), 512 as libc::c_int as size_t);
            // ensure that it doesn't overrun the buffer
            buff[511 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_char
        }
        if NET_StringToAdr(buff.as_mut_ptr(), &mut net_local) as u64 != 0 {
            namelen =
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    WSAsize_t;
            if NET_IsSocketError(getsockname(net.ip_sockets[NS_SERVER as
                                                                libc::c_int as
                                                                usize],
                                             &mut address as *mut sockaddr_in
                                                 as *mut sockaddr,
                                             &mut namelen as *mut WSAsize_t as
                                                 *mut socklen_t)) as u64 != 0
               {
                // this may happens if multiple clients running on single machine
                Con_DPrintf(b"^1Error:^7 Could not get TCP/IP address. Reason:  %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            NET_ErrorString());
                //				net.allow_ip = false;
            } else {
                net_local.port = address.sin_port;
                Con_Printf(b"Server IP address %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           NET_AdrToString(net_local));
                Cvar_FullSet(b"net_address\x00" as *const u8 as
                                 *const libc::c_char,
                             va(b"%s\x00" as *const u8 as *const libc::c_char,
                                NET_AdrToString(net_local)),
                             (1 as libc::c_int) << 17 as libc::c_int);
            }
        } else {
            Con_DPrintf(b"^1Error:^7 Could not get TCP/IP address, Invalid hostname: \'%s\'\n\x00"
                            as *const u8 as *const libc::c_char,
                        buff.as_mut_ptr());
        }
    } else {
        Con_Printf(b"TCP/IP Disabled.\n\x00" as *const u8 as
                       *const libc::c_char);
    };
}
/*
====================
NET_Config

A single player game will only use the loopback code
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_Config(mut multiplayer: qboolean) {
    static mut bFirst: qboolean = true_0;
    static mut old_config: qboolean = false_0;
    if net.initialized as u64 == 0 { return }
    if old_config as libc::c_uint == multiplayer as libc::c_uint { return }
    old_config = multiplayer;
    if multiplayer as u64 != 0 {
        // open sockets
        if net.allow_ip as u64 != 0 { NET_OpenIP(); }
        // get our local address, if possible
        if bFirst as u64 != 0 { NET_GetLocalAddress(); bFirst = false_0 }
    } else {
        let mut i: libc::c_int = 0;
        // shut down any existing sockets
        i = 0 as libc::c_int;
        while i < NS_COUNT as libc::c_int {
            if net.ip_sockets[i as usize] != -(1 as libc::c_int) {
                close(net.ip_sockets[i as usize]);
                net.ip_sockets[i as usize] = -(1 as libc::c_int)
            }
            i += 1
        }
    }
    NET_ClearLoopback();
    net.configured =
        if multiplayer as libc::c_uint != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
}
/*
====================
NET_IsConfigured

Is winsock ip initialized?
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_IsConfigured() -> qboolean {
    return net.configured;
}
/*
====================
NET_IsActive
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_IsActive() -> qboolean {
    return net.initialized;
}
/*
====================
NET_Sleep

sleeps msec or until net socket is ready
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_Sleep(mut msec: libc::c_int) {
    let mut timeout: timeval =
        timeval{tv_sec: 0,
                tv_usec:
                    0,}; // we're not a dedicated server, just run full speed
    let mut fdset: fd_set = fd_set{__fds_bits: [0; 16],}; // network socket
    let mut i: libc::c_int = 0 as libc::c_int;
    if net.initialized as u64 == 0 ||
           host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        return
    }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = &mut __d1;
    let fresh3;
    let fresh4 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh5 =
        &mut *fdset.__fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
    if net.ip_sockets[NS_SERVER as libc::c_int as usize] !=
           -(1 as libc::c_int) {
        fdset.__fds_bits[(net.ip_sockets[NS_SERVER as libc::c_int as usize] /
                              (8 as libc::c_int *
                                   ::std::mem::size_of::<__fd_mask>() as
                                       libc::c_ulong as libc::c_int)) as
                             usize] |=
            ((1 as libc::c_ulong) <<
                 net.ip_sockets[NS_SERVER as libc::c_int as usize] %
                     (8 as libc::c_int *
                          ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                              as libc::c_int)) as __fd_mask;
        i = net.ip_sockets[NS_SERVER as libc::c_int as usize]
    }
    timeout.tv_sec = (msec / 1000 as libc::c_int) as __time_t;
    timeout.tv_usec =
        (msec % 1000 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
    select(i + 1 as libc::c_int, &mut fdset, 0 as *mut fd_set,
           0 as *mut fd_set, &mut timeout);
}
/*
====================
NET_ClearLagData

clear fakelag list
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_ClearLagData(mut bClient: qboolean,
                                          mut bServer: qboolean) {
    if bClient as u64 != 0 {
        NET_ClearLaggedList(&mut *net.lagdata.as_mut_ptr().offset(NS_CLIENT as
                                                                      libc::c_int
                                                                      as
                                                                      isize));
    }
    if bServer as u64 != 0 {
        NET_ClearLaggedList(&mut *net.lagdata.as_mut_ptr().offset(NS_SERVER as
                                                                      libc::c_int
                                                                      as
                                                                      isize));
    };
}
/*
====================
NET_Init
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_Init() {
    let mut cmd: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 1 as libc::c_int;
    if net.initialized as u64 != 0 { return }
    net_clockwindow =
        Cvar_Get(b"clockwindow\x00" as *const u8 as *const libc::c_char,
                 b"0.5\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 10 as libc::c_int,
                 b"timewindow to execute client moves\x00" as *const u8 as
                     *const libc::c_char);
    net_address =
        Cvar_Get(b"net_address\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"contain local address of current client\x00" as *const u8
                     as *const libc::c_char);
    net_ipname =
        Cvar_Get(b"ip\x00" as *const u8 as *const libc::c_char,
                 b"localhost\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"network ip address\x00" as *const u8 as
                     *const libc::c_char);
    net_iphostport =
        Cvar_Get(b"ip_hostport\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"network ip host port\x00" as *const u8 as
                     *const libc::c_char);
    net_hostport =
        Cvar_Get(b"hostport\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char,
                    27015 as libc::c_int),
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"network default host port\x00" as *const u8 as
                     *const libc::c_char);
    net_ipclientport =
        Cvar_Get(b"ip_clientport\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"network ip client port\x00" as *const u8 as
                     *const libc::c_char);
    net_clientport =
        Cvar_Get(b"clientport\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char,
                    27005 as libc::c_int),
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"network default client port\x00" as *const u8 as
                     *const libc::c_char);
    net_fakelag =
        Cvar_Get(b"fakelag\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 10 as libc::c_int,
                 b"lag all incoming network data (including loopback) by xxx ms.\x00"
                     as *const u8 as *const libc::c_char);
    net_fakeloss =
        Cvar_Get(b"fakeloss\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 10 as libc::c_int,
                 b"act like we dropped the packet this % of the time.\x00" as
                     *const u8 as *const libc::c_char);
    // prepare some network data
    i = 0 as libc::c_int;
    while i < NS_COUNT as libc::c_int {
        net.lagdata[i as usize].prev =
            &mut *net.lagdata.as_mut_ptr().offset(i as isize) as
                *mut packetlag_t;
        net.lagdata[i as usize].next =
            &mut *net.lagdata.as_mut_ptr().offset(i as isize) as
                *mut packetlag_t;
        net.ip_sockets[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    // we have pthreads by default
    net.threads_initialized = true_0;
    if Sys_CheckParm(b"-noip\x00" as *const u8 as *const libc::c_char) != 0 {
        net.allow_ip = false_0
    } else { net.allow_ip = true_0 }
    // specify custom host port
    if _Sys_GetParmFromCmdLine(b"-port\x00" as *const u8 as
                                   *const libc::c_char, cmd.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 64]>() as
                                   libc::c_ulong) as libc::c_uint != 0 &&
           Q_isdigit(cmd.as_mut_ptr()) as libc::c_uint != 0 {
        Cvar_FullSet(b"hostport\x00" as *const u8 as *const libc::c_char,
                     cmd.as_mut_ptr(),
                     (1 as libc::c_int) << 17 as libc::c_int);
    }
    // specify custom ip
    if _Sys_GetParmFromCmdLine(b"-ip\x00" as *const u8 as *const libc::c_char,
                               cmd.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 64]>() as
                                   libc::c_ulong) as u64 != 0 {
        Cvar_FullSet(b"ip\x00" as *const u8 as *const libc::c_char,
                     cmd.as_mut_ptr(),
                     (1 as libc::c_int) << 17 as libc::c_int);
    }
    // adjust clockwindow
    if _Sys_GetParmFromCmdLine(b"-clockwindow\x00" as *const u8 as
                                   *const libc::c_char, cmd.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 64]>() as
                                   libc::c_ulong) as u64 != 0 {
        Cvar_SetValue(b"clockwindow\x00" as *const u8 as *const libc::c_char,
                      Q_atof(cmd.as_mut_ptr()));
    }
    net.sequence_number = 1 as libc::c_int;
    net.initialized = true_0;
    Con_Reportf(b"Base networking initialized.\n\x00" as *const u8 as
                    *const libc::c_char);
}
/*
====================
NET_Shutdown
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_Shutdown() {
    if net.initialized as u64 == 0 { return }
    NET_ClearLagData(true_0, true_0);
    NET_Config(false_0);
    net.initialized = false_0;
}
static mut http: http_static_s =
    http_static_s{first_file: 0 as *const httpfile_t as *mut httpfile_t,
                  last_file: 0 as *const httpfile_t as *mut httpfile_t,
                  first_server: 0 as *const httpserver_t as *mut httpserver_t,
                  last_server:
                      0 as *const httpserver_t as *mut httpserver_t,};
static mut http_useragent: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut http_autoremove: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut http_timeout: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut http_maxconnections: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
/*
========================
HTTP_ClearCustomServers
========================
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_ClearCustomServers() {
    if !http.first_file.is_null() { return } // may be referenced
    while !http.first_server.is_null() &&
              (*http.first_server).needfree as libc::c_uint != 0 {
        let mut tmp: *mut httpserver_t = http.first_server;
        http.first_server = (*http.first_server).next;
        _Mem_Free(tmp as *mut libc::c_void,
                  b"../engine/common/net_ws.c\x00" as *const u8 as
                      *const libc::c_char, 1847 as libc::c_int);
    };
}
/*
==============
HTTP_FreeFile

Skip to next server/file
==============
*/
unsafe extern "C" fn HTTP_FreeFile(mut file: *mut httpfile_t,
                                   mut error: qboolean) {
    let mut incname: [libc::c_char; 256] = [0; 256];
    // Allways close file and socket
    if !(*file).file.is_null() { FS_Close((*file).file); }
    (*file).file = 0 as *mut file_t;
    if (*file).socket != -(1 as libc::c_int) { close((*file).socket); }
    (*file).socket = -(1 as libc::c_int);
    Q_snprintf(incname.as_mut_ptr(), 256 as libc::c_int as size_t,
               b"downloaded/%s.incomplete\x00" as *const u8 as
                   *const libc::c_char, (*file).path.as_mut_ptr());
    if error as u64 != 0 {
        // Switch to next fastdl server if present
        if !(*file).server.is_null() &&
               (*file).state as libc::c_uint >
                   HTTP_QUEUE as libc::c_int as libc::c_uint &&
               (*file).state as libc::c_uint !=
                   HTTP_FREE as libc::c_int as libc::c_uint {
            (*file).server =
                (*(*file).server).next; // Reset download state, HTTP_Run() will open file again
            (*file).state = HTTP_QUEUE;
            return
        }
        // Process file, increase counter
        if (*http_autoremove).value == 1 as libc::c_int as libc::c_float {
            // Called because there was no servers to download, free file now
            // Warn about trash file
            // remove broken file
            FS_Delete(incname.as_mut_ptr());
        } else {
            // autoremove disabled, keep file
            Con_Printf(b"cannot download %s from any server. You may remove %s now\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*file).path.as_mut_ptr(), incname.as_mut_ptr());
        }
        if (*file).process as u64 != 0 {
            CL_ProcessFile(false_0, (*file).path.as_mut_ptr());
        }
    } else {
        // Success, rename and process file
        let mut name: [libc::c_char; 256] = [0; 256];
        Q_snprintf(name.as_mut_ptr(), 256 as libc::c_int as size_t,
                   b"downloaded/%s\x00" as *const u8 as *const libc::c_char,
                   (*file).path.as_mut_ptr());
        FS_Rename(incname.as_mut_ptr(), name.as_mut_ptr());
        if (*file).process as u64 != 0 {
            CL_ProcessFile(true_0, name.as_mut_ptr());
        } else {
            Con_Printf(b"successfully downloaded %s, processing disabled!\n\x00"
                           as *const u8 as *const libc::c_char,
                       name.as_mut_ptr());
        }
    }
    (*file).state = HTTP_FREE;
}
/*
===================
HTTP_AutoClean

remove files with HTTP_FREE state from list
===================
*/
unsafe extern "C" fn HTTP_AutoClean() {
    let mut curfile: *mut httpfile_t = 0 as *mut httpfile_t;
    let mut prevfile: *mut httpfile_t = 0 as *mut httpfile_t;
    // clean all files marked to free
    curfile = http.first_file;
    while !curfile.is_null() {
        if (*curfile).state as libc::c_uint !=
               HTTP_FREE as libc::c_int as libc::c_uint {
            prevfile = curfile
        } else if curfile == http.first_file {
            http.first_file = (*http.first_file).next;
            _Mem_Free(curfile as *mut libc::c_void,
                      b"../engine/common/net_ws.c\x00" as *const u8 as
                          *const libc::c_char, 1934 as libc::c_int);
            curfile = http.first_file;
            if curfile.is_null() { break ; }
        } else {
            if !prevfile.is_null() { (*prevfile).next = (*curfile).next }
            _Mem_Free(curfile as *mut libc::c_void,
                      b"../engine/common/net_ws.c\x00" as *const u8 as
                          *const libc::c_char, 1943 as libc::c_int);
            curfile = prevfile;
            if curfile.is_null() { break ; }
        }
        curfile = (*curfile).next
    }
    http.last_file = prevfile;
}
/*
===================
HTTP_ProcessStream

process incoming data
===================
*/
unsafe extern "C" fn HTTP_ProcessStream(mut curfile: *mut httpfile_t)
 -> qboolean {
    let mut buf: [libc::c_char; 8193] = [0; 8193];
    let mut begin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: libc::c_int = 0;
    if (*curfile).header_size >= 8192 as libc::c_int {
        Con_Reportf(b"^1Error:^7 Header to big\n\x00" as *const u8 as
                        *const libc::c_char);
        HTTP_FreeFile(curfile, true_0);
        return false_0
    }
    loop  {
        res =
            recv((*curfile).socket, buf.as_mut_ptr() as *mut libc::c_void,
                 (8192 as libc::c_int - (*curfile).header_size) as size_t,
                 0 as libc::c_int) as libc::c_int;
        if !(res > 0 as libc::c_int) { break ; }
        // if we got there, we are receiving data
        (*curfile).blocktime = 0 as libc::c_int as libc::c_float;
        if ((*curfile).state as libc::c_uint) <
               HTTP_RESPONSE_RECEIVED as libc::c_int as libc::c_uint {
            // Response still not received
            memcpy((*curfile).buf.as_mut_ptr().offset((*curfile).header_size
                                                          as isize) as
                       *mut libc::c_void,
                   buf.as_mut_ptr() as *const libc::c_void,
                   res as libc::c_ulong);
            (*curfile).buf[((*curfile).header_size + res) as usize] =
                0 as libc::c_int as libc::c_char;
            begin =
                Q_strstr((*curfile).buf.as_mut_ptr(),
                         b"\r\n\r\n\x00" as *const u8 as *const libc::c_char);
            if !begin.is_null() {
                // Got full header
                let mut cutheadersize: libc::c_int =
                    (begin.wrapping_offset_from((*curfile).buf.as_mut_ptr())
                         as libc::c_long + 4 as libc::c_int as libc::c_long)
                        as libc::c_int; // after that begin of data
                let mut length: *mut libc::c_char =
                    0 as
                        *mut libc::c_char; // cut string to print out response
                Con_Reportf(b"HTTP: Got response!\n\x00" as *const u8 as
                                *const libc::c_char);
                if Q_strstr((*curfile).buf.as_mut_ptr(),
                            b"200 OK\x00" as *const u8 as
                                *const libc::c_char).is_null() {
                    *begin = 0 as libc::c_int as libc::c_char;
                    begin =
                        Q_strchr((*curfile).buf.as_mut_ptr(),
                                 '\r' as i32 as libc::c_char);
                    if begin.is_null() {
                        begin =
                            Q_strchr((*curfile).buf.as_mut_ptr(),
                                     '\n' as i32 as libc::c_char)
                    }
                    if !begin.is_null() {
                        *begin = 0 as libc::c_int as libc::c_char
                    }
                    Con_Printf(b"^1Error:^7 %s: bad response: %s\n\x00" as
                                   *const u8 as *const libc::c_char,
                               (*curfile).path.as_mut_ptr(),
                               (*curfile).buf.as_mut_ptr());
                    HTTP_FreeFile(curfile, true_0);
                    return false_0
                }
                length =
                    Q_stristr((*curfile).buf.as_mut_ptr(),
                              b"Content-Length: \x00" as *const u8 as
                                  *const libc::c_char);
                if !length.is_null() {
                    length = length.offset(16 as libc::c_int as isize);
                    let mut size: libc::c_int = Q_atoi(length);
                    Con_Reportf(b"HTTP: File size is %d\n\x00" as *const u8 as
                                    *const libc::c_char, size);
                    if (*curfile).size != -(1 as libc::c_int) &&
                           (*curfile).size != size {
                        // print size
                        // check size if specified, not used
                        Con_Reportf(b"^3Warning:^7 Server reports wrong file size!\n\x00"
                                        as *const u8 as *const libc::c_char);
                    }
                    (*curfile).size = size;
                    (*curfile).header_size = 0 as libc::c_int
                }
                if (*curfile).size == -(1 as libc::c_int) {
                    // Usually fastdl's reports file size if link is correct
                    Con_Printf(b"^1Error:^7 file size is unknown, refusing download!\n\x00"
                                   as *const u8 as
                                   *const libc::c_char); // got response, let's start download
                    HTTP_FreeFile(curfile, true_0);
                    return false_0
                }
                (*curfile).state = HTTP_RESPONSE_RECEIVED;
                begin = begin.offset(4 as libc::c_int as isize);
                if res - cutheadersize - (*curfile).header_size >
                       0 as libc::c_int {
                    let mut ret: libc::c_int =
                        FS_Write((*curfile).file,
                                 begin as *const libc::c_void,
                                 (res - cutheadersize -
                                      (*curfile).header_size) as size_t) as
                            libc::c_int;
                    if ret != res - cutheadersize - (*curfile).header_size {
                        // could not write file
                        // close it and go to next
                        Con_Printf(b"^1Error:^7 write failed for %s!\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*curfile).path.as_mut_ptr());
                        HTTP_FreeFile(curfile, true_0);
                        return false_0
                    }
                    (*curfile).downloaded += ret
                }
            } else { (*curfile).header_size += res }
        } else if res > 0 as libc::c_int {
            // Write remaining message part
            // data download
            let mut ret_0: libc::c_int =
                FS_Write((*curfile).file,
                         buf.as_mut_ptr() as *const libc::c_void,
                         res as size_t) as libc::c_int;
            if ret_0 != res {
                // close it and go to next
                Con_Printf(b"^1Error:^7 write failed for %s!\n\x00" as
                               *const u8 as *const libc::c_char,
                           (*curfile).path.as_mut_ptr());
                (*curfile).state = HTTP_FREE;
                HTTP_FreeFile(curfile, true_0);
                return false_0
            }
            (*curfile).downloaded += ret_0;
            (*curfile).lastchecksize += ret_0;
            // as after it will run in same frame
            if (*curfile).checktime > 5 as libc::c_int as libc::c_float {
                let mut speed: libc::c_float =
                    (*curfile).lastchecksize as libc::c_float /
                        (5.0f32 * 1024 as libc::c_int as libc::c_float);
                (*curfile).checktime = 0 as libc::c_int as libc::c_float;
                Con_Reportf(b"download speed %f KB/s\n\x00" as *const u8 as
                                *const libc::c_char, speed as libc::c_double);
                (*curfile).lastchecksize = 0 as libc::c_int
            }
        }
    }
    (*curfile).checktime =
        ((*curfile).checktime as libc::c_double + host.frametime) as
            libc::c_float;
    return true_0;
}
/*
==============
HTTP_Run

Download next file block of each active file
Call every frame
==============
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_Run() {
    let mut curfile: *mut httpfile_t = 0 as *mut httpfile_t;
    let mut iActiveCount: libc::c_int = 0 as libc::c_int;
    let mut iProgressCount: libc::c_int = 0 as libc::c_int;
    let mut flProgress: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut fResolving: qboolean = false_0;
    let mut current_block_61: u64;
    curfile = http.first_file;
    while !curfile.is_null() {
        let mut res: libc::c_int = 0;
        let mut addr: sockaddr = sockaddr{sa_family: 0, sa_data: [0; 14],};
        if !((*curfile).state as libc::c_uint ==
                 HTTP_FREE as libc::c_int as libc::c_uint) {
            if (*curfile).state as libc::c_uint ==
                   HTTP_QUEUE as libc::c_int as libc::c_uint {
                let mut name: [libc::c_char; 1024] = [0; 1024];
                if iActiveCount as libc::c_float >
                       (*http_maxconnections).value {
                    current_block_61 = 4644295000439058019;
                } else {
                    if (*curfile).server.is_null() {
                        Con_Printf(b"^1Error:^7 no servers to download %s!\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curfile).path.as_mut_ptr());
                        HTTP_FreeFile(curfile, true_0);
                        break ;
                    } else {
                        Con_Reportf(b"HTTP: Starting download %s from %s\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    (*curfile).path.as_mut_ptr(),
                                    (*(*curfile).server).host.as_mut_ptr());
                        Q_snprintf(name.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 1024]>()
                                       as libc::c_ulong,
                                   b"downloaded/%s.incomplete\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*curfile).path.as_mut_ptr());
                        (*curfile).file =
                            FS_Open(name.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const libc::c_char, true_0);
                        if (*curfile).file.is_null() {
                            Con_Printf(b"^1Error:^7 cannot open %s!\n\x00" as
                                           *const u8 as *const libc::c_char,
                                       name.as_mut_ptr());
                            HTTP_FreeFile(curfile, true_0);
                            break ;
                        } else {
                            (*curfile).state = HTTP_OPENED;
                            (*curfile).blocktime =
                                0 as libc::c_int as libc::c_float;
                            (*curfile).downloaded = 0 as libc::c_int;
                            (*curfile).lastchecksize = 0 as libc::c_int;
                            (*curfile).checktime =
                                0 as libc::c_int as libc::c_float
                        }
                    }
                    current_block_61 = 10652014663920648156;
                }
            } else { current_block_61 = 10652014663920648156; }
            match current_block_61 {
                4644295000439058019 => { }
                _ => {
                    iActiveCount += 1;
                    if ((*curfile).state as libc::c_uint) <
                           HTTP_SOCKET as libc::c_int as libc::c_uint {
                        // Socket is not created
                        let mut mode: dword = 0;
                        (*curfile).socket =
                            socket(2 as libc::c_int,
                                   SOCK_STREAM as libc::c_int,
                                   IPPROTO_TCP as libc::c_int);
                        // Now set non-blocking mode
			// You may skip this if not supported by system,
			// but download will lock engine, maybe you will need to add manual returns
                        mode = 1 as libc::c_int as dword;
                        ioctl((*curfile).socket,
                              0x5421 as libc::c_int as libc::c_ulong,
                              &mut mode as *mut dword as *mut libc::c_void);
                        // SOCK_NONBLOCK is not portable, so use fcntl
                        fcntl((*curfile).socket, 4 as libc::c_int,
                              fcntl((*curfile).socket, 3 as libc::c_int,
                                    0 as libc::c_int) |
                                  0o4000 as libc::c_int); // Cannot connect
                        (*curfile).state = HTTP_SOCKET
                    }
                    if ((*curfile).state as libc::c_uint) <
                           HTTP_NS_RESOLVED as libc::c_int as libc::c_uint {
                        if fResolving as u64 != 0 {
                            current_block_61 = 4644295000439058019;
                        } else {
                            res =
                                NET_StringToSockaddr(va(b"%s:%d\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        (*(*curfile).server).host.as_mut_ptr(),
                                                        (*(*curfile).server).port),
                                                     &mut addr, true_0);
                            if res == 2 as libc::c_int {
                                fResolving = true_0;
                                current_block_61 = 4644295000439058019;
                            } else {
                                if res == 0 {
                                    Con_Printf(b"^1Error:^7 failed to resolve server address for %s!\n\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               (*(*curfile).server).host.as_mut_ptr());
                                    HTTP_FreeFile(curfile, true_0);
                                    break ;
                                } else { (*curfile).state = HTTP_NS_RESOLVED }
                                current_block_61 = 18435049525520518667;
                            }
                        }
                    } else { current_block_61 = 18435049525520518667; }
                    match current_block_61 {
                        4644295000439058019 => { }
                        _ => {
                            if ((*curfile).state as libc::c_uint) <
                                   HTTP_CONNECTED as libc::c_int as
                                       libc::c_uint {
                                // Connection not enstabilished
                                res =
                                    connect((*curfile).socket, &mut addr,
                                            ::std::mem::size_of::<sockaddr>()
                                                as libc::c_ulong as
                                                socklen_t);
                                if res != 0 {
                                    if *__errno_location() ==
                                           115 as libc::c_int ||
                                           *__errno_location() ==
                                               11 as libc::c_int {
                                        // Should give EWOOLDBLOCK if try recv too soon
                                        (*curfile).state = HTTP_CONNECTED
                                    } else {
                                        Con_Printf(b"^1Error:^7 cannot connect to server: %s\n\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   NET_ErrorString()); // Cannot connect
                                        HTTP_FreeFile(curfile, true_0);
                                        break ;
                                    }
                                    current_block_61 = 4644295000439058019;
                                    // skip to next file
                                } else {
                                    (*curfile).state = HTTP_CONNECTED;
                                    current_block_61 = 11793792312832361944;
                                }
                            } else {
                                current_block_61 = 11793792312832361944;
                            }
                            match current_block_61 {
                                4644295000439058019 => { }
                                _ => {
                                    if ((*curfile).state as libc::c_uint) <
                                           HTTP_REQUEST as libc::c_int as
                                               libc::c_uint {
                                        // Request not formatted
                                        (*curfile).query_length =
                                            Q_snprintf((*curfile).buf.as_mut_ptr(),
                                                       ::std::mem::size_of::<[libc::c_char; 8193]>()
                                                           as libc::c_ulong,
                                                       b"GET %s%s HTTP/1.0\r\nHost: %s\r\nUser-Agent: %s\r\n\r\n\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       (*(*curfile).server).path.as_mut_ptr(),
                                                       (*curfile).path.as_mut_ptr(),
                                                       (*(*curfile).server).host.as_mut_ptr(),
                                                       (*http_useragent).string);
                                        (*curfile).header_size =
                                            0 as libc::c_int;
                                        (*curfile).bytes_sent =
                                            0 as libc::c_int;
                                        (*curfile).state = HTTP_REQUEST
                                    }
                                    if ((*curfile).state as libc::c_uint) <
                                           HTTP_REQUEST_SENT as libc::c_int as
                                               libc::c_uint {
                                        // Request not sent
                                        let mut wait: qboolean = false_0;
                                        while (*curfile).bytes_sent <
                                                  (*curfile).query_length {
                                            res =
                                                send((*curfile).socket,
                                                     (*curfile).buf.as_mut_ptr().offset((*curfile).bytes_sent
                                                                                            as
                                                                                            isize)
                                                         as
                                                         *const libc::c_void,
                                                     ((*curfile).query_length
                                                          -
                                                          (*curfile).bytes_sent)
                                                         as size_t,
                                                     0 as libc::c_int) as
                                                    libc::c_int;
                                            if res < 0 as libc::c_int {
                                                if *__errno_location() !=
                                                       11 as libc::c_int &&
                                                       *__errno_location() !=
                                                           107 as libc::c_int
                                                   {
                                                    Con_Printf(b"^1Error:^7 failed to send request: %s\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               NET_ErrorString());
                                                    HTTP_FreeFile(curfile,
                                                                  true_0);
                                                    wait = true_0;
                                                    break ;
                                                } else {
                                                    // blocking while waiting connection
					// increase counter when blocking
                                                    (*curfile).blocktime =
                                                        ((*curfile).blocktime
                                                             as libc::c_double
                                                             + host.frametime)
                                                            as
                                                            libc::c_float; // success
                                                    wait = true_0;
                                                    if !((*curfile).blocktime
                                                             >
                                                             (*http_timeout).value)
                                                       {
                                                        break ;
                                                    }
                                                    Con_Printf(b"^1Error:^7 timeout on request send:\n%s\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (*curfile).buf.as_mut_ptr());
                                                    HTTP_FreeFile(curfile,
                                                                  true_0);
                                                    break ;
                                                }
                                            } else {
                                                (*curfile).bytes_sent += res;
                                                (*curfile).blocktime =
                                                    0 as libc::c_int as
                                                        libc::c_float
                                            }
                                        }
                                        if wait as u64 != 0 {
                                            current_block_61 =
                                                4644295000439058019;
                                        } else {
                                            Con_Reportf(b"HTTP: Request sent!\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            memset((*curfile).buf.as_mut_ptr()
                                                       as *mut libc::c_void,
                                                   0 as libc::c_int,
                                                   ::std::mem::size_of::<[libc::c_char; 8193]>()
                                                       as libc::c_ulong);
                                            (*curfile).state =
                                                HTTP_REQUEST_SENT;
                                            current_block_61 =
                                                479107131381816815;
                                        }
                                    } else {
                                        current_block_61 = 479107131381816815;
                                    }
                                    match current_block_61 {
                                        4644295000439058019 => { }
                                        _ => {
                                            if HTTP_ProcessStream(curfile) as
                                                   u64 == 0 {
                                                break ;
                                            }
                                            if (*curfile).size >
                                                   0 as libc::c_int {
                                                flProgress +=
                                                    (*curfile).downloaded as
                                                        libc::c_float /
                                                        (*curfile).size as
                                                            libc::c_float;
                                                iProgressCount += 1
                                            }
                                            if (*curfile).size >
                                                   0 as libc::c_int &&
                                                   (*curfile).downloaded >=
                                                       (*curfile).size {
                                                HTTP_FreeFile(curfile,
                                                              false_0);
                                                break ;
                                            } else {
                                                if *__errno_location() !=
                                                       11 as libc::c_int &&
                                                       *__errno_location() !=
                                                           115 as libc::c_int
                                                   {
                                                    Con_Reportf(b"problem downloading %s:\n%s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                (*curfile).path.as_mut_ptr(),
                                                                NET_ErrorString());
                                                } else {
                                                    (*curfile).blocktime =
                                                        ((*curfile).blocktime
                                                             as libc::c_double
                                                             + host.frametime)
                                                            as libc::c_float
                                                }
                                                if (*curfile).blocktime >
                                                       (*http_timeout).value {
                                                    Con_Printf(b"^1Error:^7 timeout on receiving data!\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                                                    HTTP_FreeFile(curfile,
                                                                  true_0);
                                                    break ;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        curfile = (*curfile).next
    }
    // update progress
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        Cvar_SetValue(b"scr_download\x00" as *const u8 as *const libc::c_char,
                      flProgress / iProgressCount as libc::c_float *
                          100 as libc::c_int as libc::c_float);
    }
    HTTP_AutoClean();
}
/*
===================
HTTP_AddDownload

Add new download to end of queue
===================
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_AddDownload(mut path: *const libc::c_char,
                                          mut size: libc::c_int,
                                          mut process: qboolean) {
    let mut httpfile: *mut httpfile_t =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<httpfile_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/net_ws.c\x00" as *const u8 as
                       *const libc::c_char, 2298 as libc::c_int) as
            *mut httpfile_t;
    Con_Reportf(b"File %s queued to download\n\x00" as *const u8 as
                    *const libc::c_char, path);
    (*httpfile).size = size;
    (*httpfile).downloaded = 0 as libc::c_int;
    (*httpfile).socket = -(1 as libc::c_int);
    Q_strncpy((*httpfile).path.as_mut_ptr(), path,
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong);
    if !http.last_file.is_null() {
        // Add next to last download
        (*httpfile).id = (*http.last_file).id + 1 as libc::c_int;
        (*http.last_file).next = httpfile;
        http.last_file = httpfile
    } else {
        // It will be the only download
        (*httpfile).id = 0 as libc::c_int;
        http.first_file = httpfile;
        http.last_file = http.first_file
    }
    (*httpfile).file = 0 as *mut file_t;
    (*httpfile).next = 0 as *mut httpfile_s;
    (*httpfile).state = HTTP_QUEUE;
    (*httpfile).server = http.first_server;
    (*httpfile).process = process;
}
/*
===============
HTTP_Download_f

Console wrapper
===============
*/
unsafe extern "C" fn HTTP_Download_f() {
    if Cmd_Argc() < 2 as libc::c_int {
        Con_Printf(b"Usage: download <gamedir_path>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    HTTP_AddDownload(Cmd_Argv(1 as libc::c_int), -(1 as libc::c_int),
                     false_0);
}
/*
==============
HTTP_ParseURL
==============
*/
unsafe extern "C" fn HTTP_ParseURL(mut url: *const libc::c_char)
 -> *mut httpserver_t {
    let mut server: *mut httpserver_t = 0 as *mut httpserver_t;
    let mut i: libc::c_int = 0;
    url = Q_strstr(url, b"http://\x00" as *const u8 as *const libc::c_char);
    if url.is_null() { return 0 as *mut httpserver_t }
    url = url.offset(7 as libc::c_int as isize);
    server =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<httpserver_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/net_ws.c\x00" as *const u8 as
                       *const libc::c_char, 2362 as libc::c_int) as
            *mut httpserver_t;
    i = 0 as libc::c_int;
    while *url as libc::c_int != 0 && *url as libc::c_int != ':' as i32 &&
              *url as libc::c_int != '/' as i32 &&
              *url as libc::c_int != '\r' as i32 &&
              *url as libc::c_int != '\n' as i32 {
        if i as libc::c_ulong >
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
            return 0 as *mut httpserver_t
        }
        let fresh6 = url;
        url = url.offset(1);
        let fresh7 = i;
        i = i + 1;
        (*server).host[fresh7 as usize] = *fresh6
    }
    (*server).host[i as usize] = 0 as libc::c_int as libc::c_char;
    if *url as libc::c_int == ':' as i32 {
        url = url.offset(1);
        (*server).port = Q_atoi(url);
        while *url as libc::c_int != 0 && *url as libc::c_int != '/' as i32 &&
                  *url as libc::c_int != '\r' as i32 &&
                  *url as libc::c_int != '\n' as i32 {
            url = url.offset(1)
        }
    } else { (*server).port = 80 as libc::c_int }
    i = 0 as libc::c_int;
    while *url as libc::c_int != 0 && *url as libc::c_int != '\r' as i32 &&
              *url as libc::c_int != '\n' as i32 {
        if i as libc::c_ulong >
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
           {
            return 0 as *mut httpserver_t
        }
        let fresh8 = url;
        url = url.offset(1);
        let fresh9 = i;
        i = i + 1;
        (*server).path[fresh9 as usize] = *fresh8
    }
    (*server).path[i as usize] = 0 as libc::c_int as libc::c_char;
    (*server).next = 0 as *mut httpserver_s;
    (*server).needfree = false_0;
    return server;
}
/*
=======================
HTTP_AddCustomServer
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_AddCustomServer(mut url: *const libc::c_char) {
    let mut server: *mut httpserver_t = HTTP_ParseURL(url);
    if server.is_null() {
        Con_Printf(b"^1Error:^7 \"%s\" is not valid url!\n\x00" as *const u8
                       as *const libc::c_char, url);
        return
    }
    (*server).needfree = true_0;
    (*server).next = http.first_server;
    http.first_server = server;
}
/*
=======================
HTTP_AddCustomServer_f
=======================
*/
unsafe extern "C" fn HTTP_AddCustomServer_f() {
    if Cmd_Argc() == 2 as libc::c_int {
        HTTP_AddCustomServer(Cmd_Argv(1 as libc::c_int));
    };
}
/*
============
HTTP_Clear_f

Clear all queue
============
*/
unsafe extern "C" fn HTTP_Clear_f() {
    http.last_file = 0 as *mut httpfile_t;
    while !http.first_file.is_null() {
        let mut file: *mut httpfile_t = http.first_file;
        http.first_file = (*http.first_file).next;
        if !(*file).file.is_null() { FS_Close((*file).file); }
        if (*file).socket != -(1 as libc::c_int) { close((*file).socket); }
        _Mem_Free(file as *mut libc::c_void,
                  b"../engine/common/net_ws.c\x00" as *const u8 as
                      *const libc::c_char, 2458 as libc::c_int);
    };
}
/*
==============
HTTP_Cancel_f

Stop current download, skip to next file
==============
*/
unsafe extern "C" fn HTTP_Cancel_f() {
    if http.first_file.is_null() { return }
    (*http.first_file).state = HTTP_FREE;
    HTTP_FreeFile(http.first_file, true_0);
}
/*
=============
HTTP_Skip_f

Stop current download, skip to next server
=============
*/
unsafe extern "C" fn HTTP_Skip_f() {
    if !http.first_file.is_null() { HTTP_FreeFile(http.first_file, true_0); };
}
/*
=============
HTTP_List_f

Print all pending downloads to console
=============
*/
unsafe extern "C" fn HTTP_List_f() {
    let mut file: *mut httpfile_t = http.first_file;
    while !file.is_null() {
        if !(*file).server.is_null() {
            Con_Printf(b"\t%d %d http://%s:%d/%s%s %d\n\x00" as *const u8 as
                           *const libc::c_char, (*file).id,
                       (*file).state as libc::c_uint,
                       (*(*file).server).host.as_mut_ptr(),
                       (*(*file).server).port,
                       (*(*file).server).path.as_mut_ptr(),
                       (*file).path.as_mut_ptr(), (*file).downloaded);
        } else {
            Con_Printf(b"\t%d %d (no server) %s\n\x00" as *const u8 as
                           *const libc::c_char, (*file).id,
                       (*file).state as libc::c_uint,
                       (*file).path.as_mut_ptr());
        }
        file = (*file).next
    };
}
/*
================
HTTP_ResetProcessState

When connected to new server, all old files should not increase counter
================
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_ResetProcessState() {
    let mut file: *mut httpfile_t = http.first_file;
    while !file.is_null() { (*file).process = false_0; file = (*file).next };
}
/*
=============
HTTP_Init
=============
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_Init() {
    let mut serverfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: [libc::c_char; 1024] = [0; 1024];
    http.last_server = 0 as *mut httpserver_t;
    http.last_file = 0 as *mut httpfile_t;
    http.first_file = http.last_file;
    Cmd_AddRestrictedCommand(b"http_download\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HTTP_Download_f as
                                      unsafe extern "C" fn() -> ()),
                             b"add file to download queue\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"http_skip\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HTTP_Skip_f as
                                      unsafe extern "C" fn() -> ()),
                             b"skip current download server\x00" as *const u8
                                 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"http_cancel\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HTTP_Cancel_f as
                                      unsafe extern "C" fn() -> ()),
                             b"cancel current download\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"http_clear\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HTTP_Clear_f as
                                      unsafe extern "C" fn() -> ()),
                             b"cancel all downloads\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddCommand(b"http_list\x00" as *const u8 as *const libc::c_char,
                   Some(HTTP_List_f as unsafe extern "C" fn() -> ()),
                   b"list all queued downloads\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"http_addcustomserver\x00" as *const u8 as
                       *const libc::c_char,
                   Some(HTTP_AddCustomServer_f as
                            unsafe extern "C" fn() -> ()),
                   b"add custom fastdl server\x00" as *const u8 as
                       *const libc::c_char);
    http_useragent =
        Cvar_Get(b"http_useragent\x00" as *const u8 as *const libc::c_char,
                 b"xash3d\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"User-Agent string\x00" as *const u8 as
                     *const libc::c_char);
    http_autoremove =
        Cvar_Get(b"http_autoremove\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"remove broken files\x00" as *const u8 as
                     *const libc::c_char);
    http_timeout =
        Cvar_Get(b"http_timeout\x00" as *const u8 as *const libc::c_char,
                 b"45\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"timeout for http downloader\x00" as *const u8 as
                     *const libc::c_char);
    http_maxconnections =
        Cvar_Get(b"http_maxconnections\x00" as *const u8 as
                     *const libc::c_char,
                 b"4\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"maximum http connection number\x00" as *const u8 as
                     *const libc::c_char);
    // Read servers from fastdl.txt
    serverfile =
        FS_LoadFile(b"fastdl.txt\x00" as *const u8 as *const libc::c_char,
                    0 as *mut fs_offset_t, false_0) as *mut libc::c_char;
    line = serverfile;
    if !serverfile.is_null() {
        loop  {
            line =
                _COM_ParseFileSafe(line, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 1024]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if line.is_null() { break ; }
            let mut server: *mut httpserver_t =
                HTTP_ParseURL(token.as_mut_ptr());
            if server.is_null() { continue ; }
            if http.last_server.is_null() {
                http.first_server = server;
                http.last_server = http.first_server
            } else {
                (*http.last_server).next = server;
                http.last_server = server
            }
        }
        _Mem_Free(serverfile as *mut libc::c_void,
                  b"../engine/common/net_ws.c\x00" as *const u8 as
                      *const libc::c_char, 2578 as libc::c_int);
    };
}
/*
====================
HTTP_Shutdown
====================
*/
#[no_mangle]
pub unsafe extern "C" fn HTTP_Shutdown() {
    HTTP_Clear_f();
    while !http.first_server.is_null() {
        let mut tmp: *mut httpserver_t = http.first_server;
        http.first_server = (*http.first_server).next;
        _Mem_Free(tmp as *mut libc::c_void,
                  b"../engine/common/net_ws.c\x00" as *const u8 as
                      *const libc::c_char, 2596 as libc::c_int);
    }
    http.last_server = 0 as *mut httpserver_t;
}
