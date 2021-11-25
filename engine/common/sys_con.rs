#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type decallist_s;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_timestamp(format: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_Print(pMsg: *const libc::c_char);
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type time_t = __time_t;
pub type uint = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uint32_t = __uint32_t;
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
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type C2RustUnnamed_1 = libc::c_uint;
pub const TIME_FILENAME: C2RustUnnamed_1 = 5;
pub const TIME_YEAR_ONLY: C2RustUnnamed_1 = 4;
pub const TIME_NO_SECONDS: C2RustUnnamed_1 = 3;
pub const TIME_TIME_ONLY: C2RustUnnamed_1 = 2;
pub const TIME_DATE_ONLY: C2RustUnnamed_1 = 1;
pub const TIME_FULL: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LogData {
    pub title: [libc::c_char; 64],
    pub log_active: qboolean,
    pub log_path: [libc::c_char; 1024],
    pub logfile: *mut FILE,
    pub logfileno: libc::c_int,
}
pub type convar_t = convar_s;
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
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
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
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
static mut s_ld: LogData =
    LogData{title: [0; 64],
            log_active: false_0,
            log_path: [0; 1024],
            logfile: 0 as *const FILE as *mut FILE,
            logfileno: 0,};
#[no_mangle]
pub unsafe extern "C" fn Sys_Input() -> *mut libc::c_char {
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_DestroyConsole() {
    // last text message into console or log
    Con_Reportf(b"Sys_DestroyConsole: Exiting!\n\x00" as *const u8 as
                    *const libc::c_char);
}
/*
===============================================================================

SYSTEM LOG

===============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_LogFileNo() -> libc::c_int {
    return s_ld.logfileno;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_InitLog() {
    let mut mode: *const libc::c_char = 0 as *const libc::c_char;
    if Sys_CheckParm(b"-log\x00" as *const u8 as *const libc::c_char) != 0 &&
           host.allow_console as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
        s_ld.log_active = true_0;
        Q_strncpy(s_ld.log_path.as_mut_ptr(),
                  b"engine.log\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 1024]>() as
                      libc::c_ulong);
    }
    if host.change_game as libc::c_uint != 0 &&
           host.type_0 != HOST_DEDICATED as libc::c_int as libc::c_uint {
        mode = b"a\x00" as *const u8 as *const libc::c_char
    } else { mode = b"w\x00" as *const u8 as *const libc::c_char }
    // create log if needed
    if s_ld.log_active as u64 != 0 {
        s_ld.logfile = fopen(s_ld.log_path.as_mut_ptr(), mode);
        if s_ld.logfile.is_null() {
            Con_Reportf(b"^1Error:^7 Sys_InitLog: can\'t create log file %s\n\x00"
                            as *const u8 as *const libc::c_char,
                        s_ld.log_path.as_mut_ptr());
        }
        fprintf(s_ld.logfile,
                b"=================================================================================\n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(s_ld.logfile,
                b"\t%s (build %i) started at %s\n\x00" as *const u8 as
                    *const libc::c_char, s_ld.title.as_mut_ptr(),
                Q_buildnum(), Q_timestamp(TIME_FULL as libc::c_int));
        fprintf(s_ld.logfile,
                b"=================================================================================\n\x00"
                    as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sys_CloseLog() {
    let mut event_name: [libc::c_char; 64] = [0; 64];
    // continue logged
    match host.status as libc::c_uint {
        6 => {
            Q_strncpy(event_name.as_mut_ptr(),
                      b"crashed\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong); //short time
        }
        3 => {
            Q_strncpy(event_name.as_mut_ptr(),
                      b"stopped with error\x00" as *const u8 as
                          *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong);
        }
        _ => {
            if host.change_game as u64 == 0 {
                Q_strncpy(event_name.as_mut_ptr(),
                          b"stopped\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            } else {
                Q_strncpy(event_name.as_mut_ptr(), host.finalmsg.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            }
        }
    }
    if !s_ld.logfile.is_null() {
        fprintf(s_ld.logfile, b"\n\x00" as *const u8 as *const libc::c_char);
        fprintf(s_ld.logfile,
                b"=================================================================================\x00"
                    as *const u8 as *const libc::c_char);
        if host.change_game as u64 != 0 {
            fprintf(s_ld.logfile,
                    b"\n\t%s (build %i) %s\n\x00" as *const u8 as
                        *const libc::c_char, s_ld.title.as_mut_ptr(),
                    Q_buildnum(), event_name.as_mut_ptr());
        } else {
            fprintf(s_ld.logfile,
                    b"\n\t%s (build %i) %s at %s\n\x00" as *const u8 as
                        *const libc::c_char, s_ld.title.as_mut_ptr(),
                    Q_buildnum(), event_name.as_mut_ptr(),
                    Q_timestamp(TIME_FULL as libc::c_int));
        }
        fprintf(s_ld.logfile,
                b"=================================================================================\n\x00"
                    as *const u8 as *const libc::c_char);
        fclose(s_ld.logfile);
        s_ld.logfile = 0 as *mut FILE
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sys_PrintLog(mut pMsg: *const libc::c_char) {
    let mut crt_time: time_t = 0;
    let mut crt_tm: *const tm = 0 as *const tm;
    let mut logtime: [libc::c_char; 32] =
        *::std::mem::transmute::<&[u8; 32],
                                 &mut [libc::c_char; 32]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    static mut lastchar: libc::c_char = 0;
    time(&mut crt_time);
    crt_tm = localtime(&mut crt_time);
    if lastchar == 0 || lastchar as libc::c_int == '\n' as i32 {
        strftime(logtime.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                 b"[%H:%M:%S] \x00" as *const u8 as *const libc::c_char,
                 crt_tm);
    }
    let mut colored: [libc::c_char; 4096] = [0; 4096];
    let mut msg: *const libc::c_char = pMsg;
    let mut len: libc::c_int = 0 as libc::c_int;
    while *msg as libc::c_int != 0 && len < 4090 as libc::c_int {
        static mut q3ToAnsi: [libc::c_char; 8] =
            ['0' as i32 as libc::c_char, '1' as i32 as libc::c_char,
             '2' as i32 as libc::c_char, '3' as i32 as libc::c_char,
             '4' as i32 as libc::c_char, '6' as i32 as libc::c_char,
             '5' as i32 as libc::c_char, 0 as libc::c_int as libc::c_char];
        if !msg.is_null() && *msg as libc::c_int == '^' as i32 &&
               *msg.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
               *msg.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
               *msg.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            let mut color: libc::c_int = 0;
            msg = msg.offset(1);
            let fresh0 = msg;
            msg = msg.offset(1);
            color =
                q3ToAnsi[(*fresh0 as libc::c_int % 8 as libc::c_int) as usize]
                    as libc::c_int;
            let fresh1 = len;
            len = len + 1;
            colored[fresh1 as usize] = '\u{1b}' as i32 as libc::c_char;
            let fresh2 = len;
            len = len + 1;
            colored[fresh2 as usize] = '[' as i32 as libc::c_char;
            if color != 0 {
                let fresh3 = len;
                len = len + 1;
                colored[fresh3 as usize] = '3' as i32 as libc::c_char;
                let fresh4 = len;
                len = len + 1;
                colored[fresh4 as usize] = color as libc::c_char
            } else {
                let fresh5 = len;
                len = len + 1;
                colored[fresh5 as usize] = '0' as i32 as libc::c_char
            }
            let fresh6 = len;
            len = len + 1;
            colored[fresh6 as usize] = 'm' as i32 as libc::c_char
        } else {
            let fresh7 = msg;
            msg = msg.offset(1);
            let fresh8 = len;
            len = len + 1;
            colored[fresh8 as usize] = *fresh7
        }
    }
    colored[len as usize] = 0 as libc::c_int as libc::c_char;
    printf(b"\x1b[34m%s\x1b[0m%s\x1b[0m\x00" as *const u8 as
               *const libc::c_char, logtime.as_mut_ptr(),
           colored.as_mut_ptr());
    // save last char to detect when line was not ended
    lastchar =
        *pMsg.offset(strlen(pMsg).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) as
                         isize); //full time
    if s_ld.logfile.is_null() { return }
    if lastchar == 0 || lastchar as libc::c_int == '\n' as i32 {
        strftime(logtime.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                 b"[%Y:%m:%d|%H:%M:%S]\x00" as *const u8 as
                     *const libc::c_char, crt_tm);
    }
    fprintf(s_ld.logfile, b"%s %s\x00" as *const u8 as *const libc::c_char,
            logtime.as_mut_ptr(), pMsg);
    fflush(s_ld.logfile);
}
/*
=============================================================================

CONSOLE PRINT

=============================================================================
*/
/*
=============
Con_Printf

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Printf(mut szFmt: *const libc::c_char,
                                    mut args: ...) {
    static mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut args_0: ::std::ffi::VaListImpl;
    if host.allow_console as u64 == 0 { return }
    args_0 = args.clone();
    Q_vsnprintf(buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as
                    libc::c_ulong, szFmt, args_0.as_va_list());
    Sys_Print(buffer.as_mut_ptr());
}
/*
=============
Con_DPrintf

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DPrintf(mut szFmt: *const libc::c_char,
                                     mut args: ...) {
    static mut buffer: [libc::c_char; 8192] = [0; 8192]; // hlrally spam
    let mut args_0: ::std::ffi::VaListImpl;
    if host_developer.value < DEV_NORMAL as libc::c_int as libc::c_float {
        return
    }
    args_0 = args.clone();
    Q_vsnprintf(buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as
                    libc::c_ulong, szFmt, args_0.as_va_list());
    if buffer[0 as libc::c_int as usize] as libc::c_int == '0' as i32 &&
           buffer[1 as libc::c_int as usize] as libc::c_int == '\n' as i32 &&
           buffer[2 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32
       {
        return
    }
    Sys_Print(buffer.as_mut_ptr());
}
/*
=============
Con_Reportf

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Reportf(mut szFmt: *const libc::c_char,
                                     mut args: ...) {
    static mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut args_0: ::std::ffi::VaListImpl;
    if host_developer.value < DEV_EXTENDED as libc::c_int as libc::c_float {
        return
    }
    args_0 = args.clone();
    Q_vsnprintf(buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as
                    libc::c_ulong, szFmt, args_0.as_va_list());
    Sys_Print(buffer.as_mut_ptr());
}
