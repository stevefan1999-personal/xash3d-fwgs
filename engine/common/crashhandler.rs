#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type SDL_Window;
    pub type decallist_s;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn SDL_SetWindowGrab(window: *mut SDL_Window, grabbed: SDL_bool);
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Platform_MessageBox(title: *const libc::c_char,
                           message: *const libc::c_char,
                           parentMainWindow: qboolean);
    #[no_mangle]
    fn Sys_Quit() -> !;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn CL_Crashed();
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Sys_LogFileNo() -> libc::c_int;
    #[no_mangle]
    fn Q_buildarch() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildos() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildcommit() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    fn dladdr(__address: *const libc::c_void, __info: *mut Dl_info)
     -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn mprotect(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int)
     -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dl_info {
    pub dli_fname: *const libc::c_char,
    pub dli_fbase: *mut libc::c_void,
    pub dli_sname: *const libc::c_char,
    pub dli_saddr: *mut libc::c_void,
}
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_1,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_11,
    pub _timer: C2RustUnnamed_10,
    pub _rt: C2RustUnnamed_9,
    pub _sigchld: C2RustUnnamed_8,
    pub _sigfault: C2RustUnnamed_5,
    pub _sigpoll: C2RustUnnamed_4,
    pub _sigsys: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _addr_bnd: C2RustUnnamed_7,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
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
pub type greg_t = libc::c_longlong;
pub const REG_RSP: C2RustUnnamed_12 = 15;
pub type gregset_t = [greg_t; 23];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
pub type fpregset_t = *mut _libc_fpstate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    pub __glibc_reserved1: [__uint32_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: libc::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
    pub __ssp: [libc::c_ulonglong; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub const REG_RBP: C2RustUnnamed_12 = 10;
pub const REG_RIP: C2RustUnnamed_12 = 16;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const REG_CR2: C2RustUnnamed_12 = 22;
pub const REG_OLDMASK: C2RustUnnamed_12 = 21;
pub const REG_TRAPNO: C2RustUnnamed_12 = 20;
pub const REG_ERR: C2RustUnnamed_12 = 19;
pub const REG_CSGSFS: C2RustUnnamed_12 = 18;
pub const REG_EFL: C2RustUnnamed_12 = 17;
pub const REG_RCX: C2RustUnnamed_12 = 14;
pub const REG_RAX: C2RustUnnamed_12 = 13;
pub const REG_RDX: C2RustUnnamed_12 = 12;
pub const REG_RBX: C2RustUnnamed_12 = 11;
pub const REG_RSI: C2RustUnnamed_12 = 9;
pub const REG_RDI: C2RustUnnamed_12 = 8;
pub const REG_R15: C2RustUnnamed_12 = 7;
pub const REG_R14: C2RustUnnamed_12 = 6;
pub const REG_R13: C2RustUnnamed_12 = 5;
pub const REG_R12: C2RustUnnamed_12 = 4;
pub const REG_R11: C2RustUnnamed_12 = 3;
pub const REG_R10: C2RustUnnamed_12 = 2;
pub const REG_R9: C2RustUnnamed_12 = 1;
pub const REG_R8: C2RustUnnamed_12 = 0;
/*
crashhandler.c - advanced crashhandler
Copyright (C) 2016 Mittorn

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
/*
================
Sys_Crash

Crash handler, called from system
================
*/
// Posix signal handler
#[no_mangle]
pub unsafe extern "C" fn printframe(mut buf: *mut libc::c_char,
                                    mut len: libc::c_int, mut i: libc::c_int,
                                    mut addr: *mut libc::c_void)
 -> libc::c_int {
    let mut dlinfo: Dl_info =
        Dl_info{dli_fname: 0 as *const libc::c_char,
                dli_fbase: 0 as *mut libc::c_void,
                dli_sname: 0 as *const libc::c_char,
                dli_saddr: 0 as *mut libc::c_void,}; // overflow
    if len <= 0 as libc::c_int {
        return 0 as libc::c_int
    } // print symbol, module and address
    if dladdr(addr, &mut dlinfo) != 0 {
        if !dlinfo.dli_sname.is_null() {
            return Q_snprintf(buf, len as size_t,
                              b"%2d: %p <%s+%lu> (%s)\n\x00" as *const u8 as
                                  *const libc::c_char, i, addr,
                              dlinfo.dli_sname,
                              (addr as
                                   libc::c_ulong).wrapping_sub(dlinfo.dli_saddr
                                                                   as
                                                                   libc::c_ulong),
                              dlinfo.dli_fname)
        } else {
            return Q_snprintf(buf, len as size_t,
                              b"%2d: %p (%s)\n\x00" as *const u8 as
                                  *const libc::c_char, i, addr,
                              dlinfo.dli_fname)
        }
        // print module and address
    } else {
        return Q_snprintf(buf, len as size_t,
                          b"%2d: %p\n\x00" as *const u8 as
                              *const libc::c_char, i, addr)
    };
    // print only address
}
#[no_mangle]
pub static mut oldFilter: sigaction =
    sigaction{__sigaction_handler: C2RustUnnamed_1{sa_handler: None,},
              sa_mask: __sigset_t{__val: [0; 16],},
              sa_flags: 0,
              sa_restorer: None,};
unsafe extern "C" fn Sys_Crash(mut signal: libc::c_int,
                               mut si: *mut siginfo_t,
                               mut context: *mut libc::c_void) {
    let mut pc: *mut libc::c_void =
        0 as *mut libc::c_void; // this must be set for every OS!
    let mut bp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut sp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut message: [libc::c_char; 8192] = [0; 8192];
    let mut len: libc::c_int = 0;
    let mut logfd: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut pagesize: size_t = 0;
    let mut ucontext: *mut ucontext_t = context as *mut ucontext_t;
    pc =
        (*ucontext).uc_mcontext.gregs[REG_RIP as libc::c_int as usize] as
            *mut libc::c_void;
    bp =
        (*ucontext).uc_mcontext.gregs[REG_RBP as libc::c_int as usize] as
            *mut *mut libc::c_void;
    sp =
        (*ucontext).uc_mcontext.gregs[REG_RSP as libc::c_int as usize] as
            *mut *mut libc::c_void;
    // safe actions first, stack and memory may be corrupted
    len =
        Q_snprintf(message.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 8192]>() as
                       libc::c_ulong,
                   b"Ver: %s %s (build %i-%s, %s-%s)\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"Xash3D FWGS\x00" as *const u8 as *const libc::c_char,
                   b"0.20\x00" as *const u8 as *const libc::c_char,
                   Q_buildnum(), Q_buildcommit(), Q_buildos(), Q_buildarch());
    len +=
        Q_snprintf(message.as_mut_ptr().offset(len as isize),
                   (::std::mem::size_of::<[libc::c_char; 8192]>() as
                        libc::c_ulong).wrapping_sub(len as libc::c_ulong),
                   b"Crash: signal %d errno %d with code %d at %p %p\n\x00" as
                       *const u8 as *const libc::c_char, signal,
                   (*si).si_errno, (*si).si_code,
                   (*si)._sifields._sigfault.si_addr,
                   (*si)._sifields._rt.si_sigval.sival_ptr);
    write(2 as libc::c_int, message.as_mut_ptr() as *const libc::c_void,
          len as size_t);
    // flush buffers before writing directly to descriptors
    fflush(stdout);
    fflush(stderr);
    // now get log fd and write trace directly to log
    logfd = Sys_LogFileNo();
    write(logfd, message.as_mut_ptr() as *const libc::c_void, len as size_t);
    if !pc.is_null() && !bp.is_null() && !sp.is_null() {
        let mut pagesize_0: size_t =
            sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
        // try to print backtrace
        write(2 as libc::c_int,
              b"Stack backtrace:\n\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void, 17 as libc::c_int as size_t);
        write(logfd,
              b"Stack backtrace:\n\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void, 17 as libc::c_int as size_t);
        strncpy(message.as_mut_ptr().offset(len as isize),
                b"Stack backtrace:\n\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong));
        len += 17 as libc::c_int;
        loop 
             // false on success, true on failure
             {
            i += 1;
            let mut line: libc::c_int =
                printframe(message.as_mut_ptr().offset(len as isize),
                           (::std::mem::size_of::<[libc::c_char; 8192]>() as
                                libc::c_ulong).wrapping_sub(len as
                                                                libc::c_ulong)
                               as libc::c_int, i, pc);
            write(2 as libc::c_int,
                  message.as_mut_ptr().offset(len as isize) as
                      *const libc::c_void, line as size_t);
            write(logfd,
                  message.as_mut_ptr().offset(len as isize) as
                      *const libc::c_void, line as size_t);
            len += line;
            //if( !dladdr(bp,0) ) break; // only when bp is in module
            if mprotect(((bp as
                              uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong))
                             &
                             !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong)) as
                            *mut libc::c_char as *mut libc::c_void,
                        pagesize_0,
                        0x1 as libc::c_int | 0x2 as libc::c_int |
                            0x4 as libc::c_int) == -(1 as libc::c_int) &&
                   mprotect(((bp as
                                  uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                                 &
                                 !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong))
                                as *mut libc::c_char as *mut libc::c_void,
                            pagesize_0,
                            0x1 as libc::c_int | 0x4 as libc::c_int) ==
                       -(1 as libc::c_int) &&
                   mprotect(((bp as
                                  uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                                 &
                                 !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong))
                                as *mut libc::c_char as *mut libc::c_void,
                            pagesize_0,
                            0x1 as libc::c_int | 0x2 as libc::c_int) ==
                       -(1 as libc::c_int) &&
                   mprotect(((bp as
                                  uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                                 &
                                 !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong))
                                as *mut libc::c_char as *mut libc::c_void,
                            pagesize_0, 0x1 as libc::c_int) ==
                       -(1 as libc::c_int) {
                break ;
            }
            if mprotect(((*bp.offset(0 as libc::c_int as isize) as
                              uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong))
                             &
                             !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong)) as
                            *mut libc::c_char as *mut libc::c_void,
                        pagesize_0,
                        0x1 as libc::c_int | 0x2 as libc::c_int |
                            0x4 as libc::c_int) == -(1 as libc::c_int) &&
                   mprotect(((*bp.offset(0 as libc::c_int as isize) as
                                  uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                                 &
                                 !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong))
                                as *mut libc::c_char as *mut libc::c_void,
                            pagesize_0,
                            0x1 as libc::c_int | 0x4 as libc::c_int) ==
                       -(1 as libc::c_int) &&
                   mprotect(((*bp.offset(0 as libc::c_int as isize) as
                                  uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                                 &
                                 !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong))
                                as *mut libc::c_char as *mut libc::c_void,
                            pagesize_0,
                            0x1 as libc::c_int | 0x2 as libc::c_int) ==
                       -(1 as libc::c_int) &&
                   mprotect(((*bp.offset(0 as libc::c_int as isize) as
                                  uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                                 &
                                 !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong))
                                as *mut libc::c_char as *mut libc::c_void,
                            pagesize_0, 0x1 as libc::c_int) ==
                       -(1 as libc::c_int) {
                break ;
            }
            pc = *bp.offset(1 as libc::c_int as isize);
            bp =
                *bp.offset(0 as libc::c_int as isize) as
                    *mut *mut libc::c_void;
            if !(!bp.is_null() && i < 128 as libc::c_int) { break ; }
        }
        // try to print stack
        write(2 as libc::c_int,
              b"Stack dump:\n\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void, 12 as libc::c_int as size_t);
        write(logfd,
              b"Stack dump:\n\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void, 12 as libc::c_int as size_t);
        strncpy(message.as_mut_ptr().offset(len as isize),
                b"Stack dump:\n\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong));
        len += 12 as libc::c_int;
        if !(mprotect(((sp as
                            uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong))
                           &
                           !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong)) as
                          *mut libc::c_char as *mut libc::c_void, pagesize_0,
                      0x1 as libc::c_int | 0x2 as libc::c_int |
                          0x4 as libc::c_int) == -(1 as libc::c_int) &&
                 mprotect(((sp as
                                uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                               &
                               !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong)) as
                              *mut libc::c_char as *mut libc::c_void,
                          pagesize_0, 0x1 as libc::c_int | 0x4 as libc::c_int)
                     == -(1 as libc::c_int) &&
                 mprotect(((sp as
                                uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                               &
                               !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong)) as
                              *mut libc::c_char as *mut libc::c_void,
                          pagesize_0, 0x1 as libc::c_int | 0x2 as libc::c_int)
                     == -(1 as libc::c_int) &&
                 mprotect(((sp as
                                uintptr_t).wrapping_add(pagesize_0.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                               &
                               !pagesize_0.wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong)) as
                              *mut libc::c_char as *mut libc::c_void,
                          pagesize_0, 0x1 as libc::c_int) ==
                     -(1 as libc::c_int)) {
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                let mut line_0: libc::c_int =
                    printframe(message.as_mut_ptr().offset(len as isize),
                               (::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as
                                    libc::c_ulong).wrapping_sub(len as
                                                                    libc::c_ulong)
                                   as libc::c_int, i, *sp.offset(i as isize));
                write(2 as libc::c_int,
                      message.as_mut_ptr().offset(len as isize) as
                          *const libc::c_void, line_0 as size_t);
                write(logfd,
                      message.as_mut_ptr().offset(len as isize) as
                          *const libc::c_void, line_0 as size_t);
                len += line_0;
                i += 1
            }
        }
    }
    // put MessageBox as Sys_Error
    Con_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
               message.as_mut_ptr());
    SDL_SetWindowGrab(host.hWnd as *mut SDL_Window, SDL_FALSE);
    Platform_MessageBox(b"Xash Error\x00" as *const u8 as *const libc::c_char,
                        message.as_mut_ptr(), false_0);
    // log saved, now we can try to save configs and close log correctly, it may crash
    if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        CL_Crashed();
    }
    host.status = HOST_CRASHED;
    host.crashed = true_0;
    Sys_Quit();
}
#[no_mangle]
pub unsafe extern "C" fn Sys_SetupCrashHandler() {
    let mut act: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_1{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    act.__sigaction_handler.sa_sigaction =
        Some(Sys_Crash as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t,
                                      _: *mut libc::c_void) -> ());
    act.sa_flags = 4 as libc::c_int | 0x8000000 as libc::c_int;
    sigaction(11 as libc::c_int, &mut act, &mut oldFilter);
    sigaction(6 as libc::c_int, &mut act, &mut oldFilter);
    sigaction(7 as libc::c_int, &mut act, &mut oldFilter);
    sigaction(4 as libc::c_int, &mut act, &mut oldFilter);
}
#[no_mangle]
pub unsafe extern "C" fn Sys_RestoreCrashHandler() {
    sigaction(11 as libc::c_int, &mut oldFilter, 0 as *mut sigaction);
    sigaction(6 as libc::c_int, &mut oldFilter, 0 as *mut sigaction);
    sigaction(7 as libc::c_int, &mut oldFilter, 0 as *mut sigaction);
    sigaction(4 as libc::c_int, &mut oldFilter, 0 as *mut sigaction);
}
