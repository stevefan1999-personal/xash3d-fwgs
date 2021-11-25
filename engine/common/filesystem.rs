#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, const_transmute,
           extern_types, label_break_value, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    pub type __dirstream;
    pub type mz_internal_state;
    pub type decallist_s;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnlwr(in_0: *const libc::c_char, out: *mut libc::c_char,
                 size_out: size_t);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_tolower(in_0: libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strrchr(s: *const libc::c_char, c: libc::c_char)
     -> *mut libc::c_char;
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
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn COM_FileExtension(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn COM_DefaultExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn COM_ExtractFilePath(path: *const libc::c_char,
                           dest: *mut libc::c_char);
    #[no_mangle]
    fn COM_FileWithoutPath(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn matchpattern(in_0: *const libc::c_char, pattern: *const libc::c_char,
                    caseinsensitive: qboolean) -> libc::c_int;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Sys_GetParmFromCmdLine(parm: *const libc::c_char,
                               out: *mut libc::c_char, size: size_t)
     -> qboolean;
    #[no_mangle]
    fn CRC32_Init(pulCRC: *mut dword);
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
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
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
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int)
     -> __off_t;
    #[no_mangle]
    fn Q_buildarch() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildos() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildcommit() -> *const libc::c_char;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_InitDecals();
    #[no_mangle]
    fn Image_CheckPaletteQ1();
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type mz_ulong = libc::c_ulong;
pub type mz_uint32 = libc::c_uint;
pub type mz_uint8 = libc::c_uchar;
pub type C2RustUnnamed = libc::c_uint;
pub const MZ_FIXED: C2RustUnnamed = 4;
pub const MZ_RLE: C2RustUnnamed = 3;
pub const MZ_HUFFMAN_ONLY: C2RustUnnamed = 2;
pub const MZ_FILTERED: C2RustUnnamed = 1;
pub const MZ_DEFAULT_STRATEGY: C2RustUnnamed = 0;
pub type mz_alloc_func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t, _: size_t)
               -> *mut libc::c_void>;
pub type mz_free_func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub type C2RustUnnamed_0 = libc::c_int;
pub const MZ_DEFAULT_COMPRESSION: C2RustUnnamed_0 = -1;
pub const MZ_DEFAULT_LEVEL: C2RustUnnamed_0 = 6;
pub const MZ_UBER_COMPRESSION: C2RustUnnamed_0 = 10;
pub const MZ_BEST_COMPRESSION: C2RustUnnamed_0 = 9;
pub const MZ_BEST_SPEED: C2RustUnnamed_0 = 1;
pub const MZ_NO_COMPRESSION: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MZ_BLOCK: C2RustUnnamed_1 = 5;
pub const MZ_FINISH: C2RustUnnamed_1 = 4;
pub const MZ_FULL_FLUSH: C2RustUnnamed_1 = 3;
pub const MZ_SYNC_FLUSH: C2RustUnnamed_1 = 2;
pub const MZ_PARTIAL_FLUSH: C2RustUnnamed_1 = 1;
pub const MZ_NO_FLUSH: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_int;
pub const MZ_PARAM_ERROR: C2RustUnnamed_2 = -10000;
pub const MZ_VERSION_ERROR: C2RustUnnamed_2 = -6;
pub const MZ_BUF_ERROR: C2RustUnnamed_2 = -5;
pub const MZ_MEM_ERROR: C2RustUnnamed_2 = -4;
pub const MZ_DATA_ERROR: C2RustUnnamed_2 = -3;
pub const MZ_STREAM_ERROR: C2RustUnnamed_2 = -2;
pub const MZ_ERRNO: C2RustUnnamed_2 = -1;
pub const MZ_NEED_DICT: C2RustUnnamed_2 = 2;
pub const MZ_STREAM_END: C2RustUnnamed_2 = 1;
pub const MZ_OK: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mz_stream_s {
    pub next_in: *const libc::c_uchar,
    pub avail_in: libc::c_uint,
    pub total_in: mz_ulong,
    pub next_out: *mut libc::c_uchar,
    pub avail_out: libc::c_uint,
    pub total_out: mz_ulong,
    pub msg: *mut libc::c_char,
    pub state: *mut mz_internal_state,
    pub zalloc: mz_alloc_func,
    pub zfree: mz_free_func,
    pub opaque: *mut libc::c_void,
    pub data_type: libc::c_int,
    pub adler: mz_ulong,
    pub reserved: mz_ulong,
}
pub type mz_stream = mz_stream_s;
pub type mz_streamp = *mut mz_stream;
pub const TDEFL_STATUS_OKAY: tdefl_status = 0;
pub type tdefl_status = libc::c_int;
pub const TDEFL_STATUS_DONE: tdefl_status = 1;
pub const TDEFL_STATUS_PUT_BUF_FAILED: tdefl_status = -1;
pub const TDEFL_STATUS_BAD_PARAM: tdefl_status = -2;
pub type mz_uint = libc::c_uint;
pub const TDEFL_GREEDY_PARSING_FLAG: C2RustUnnamed_5 = 16384;
pub const TDEFL_RLE_MATCHES: C2RustUnnamed_5 = 65536;
pub const TDEFL_FORCE_ALL_STATIC_BLOCKS: C2RustUnnamed_5 = 262144;
pub const TDEFL_MAX_PROBES_MASK: C2RustUnnamed_4 = 4095;
pub const TDEFL_FILTER_MATCHES: C2RustUnnamed_5 = 131072;
pub const TDEFL_FORCE_ALL_RAW_BLOCKS: C2RustUnnamed_5 = 524288;
pub const TDEFL_WRITE_ZLIB_HEADER: C2RustUnnamed_5 = 4096;
pub const TDEFL_COMPUTE_ADLER32: C2RustUnnamed_5 = 8192;
pub type tdefl_put_buf_func_ptr
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: libc::c_int,
                                _: *mut libc::c_void) -> mz_bool>;
pub type mz_bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdefl_compressor {
    pub m_pPut_buf_func: tdefl_put_buf_func_ptr,
    pub m_pPut_buf_user: *mut libc::c_void,
    pub m_flags: mz_uint,
    pub m_max_probes: [mz_uint; 2],
    pub m_greedy_parsing: libc::c_int,
    pub m_adler32: mz_uint,
    pub m_lookahead_pos: mz_uint,
    pub m_lookahead_size: mz_uint,
    pub m_dict_size: mz_uint,
    pub m_pLZ_code_buf: *mut mz_uint8,
    pub m_pLZ_flags: *mut mz_uint8,
    pub m_pOutput_buf: *mut mz_uint8,
    pub m_pOutput_buf_end: *mut mz_uint8,
    pub m_num_flags_left: mz_uint,
    pub m_total_lz_bytes: mz_uint,
    pub m_lz_code_buf_dict_pos: mz_uint,
    pub m_bits_in: mz_uint,
    pub m_bit_buffer: mz_uint,
    pub m_saved_match_dist: mz_uint,
    pub m_saved_match_len: mz_uint,
    pub m_saved_lit: mz_uint,
    pub m_output_flush_ofs: mz_uint,
    pub m_output_flush_remaining: mz_uint,
    pub m_finished: mz_uint,
    pub m_block_index: mz_uint,
    pub m_wants_to_finish: mz_uint,
    pub m_prev_return_status: tdefl_status,
    pub m_pIn_buf: *const libc::c_void,
    pub m_pOut_buf: *mut libc::c_void,
    pub m_pIn_buf_size: *mut size_t,
    pub m_pOut_buf_size: *mut size_t,
    pub m_flush: tdefl_flush,
    pub m_pSrc: *const mz_uint8,
    pub m_src_buf_left: size_t,
    pub m_out_buf_ofs: size_t,
    pub m_dict: [mz_uint8; 33025],
    pub m_huff_count: [[mz_uint16; 288]; 3],
    pub m_huff_codes: [[mz_uint16; 288]; 3],
    pub m_huff_code_sizes: [[mz_uint8; 288]; 3],
    pub m_lz_code_buf: [mz_uint8; 65536],
    pub m_next: [mz_uint16; 32768],
    pub m_hash: [mz_uint16; 32768],
    pub m_output_buf: [mz_uint8; 85196],
}
pub type mz_uint16 = libc::c_ushort;
pub type tdefl_flush = libc::c_uint;
pub const TDEFL_FINISH: tdefl_flush = 4;
pub const TDEFL_FULL_FLUSH: tdefl_flush = 3;
pub const TDEFL_SYNC_FLUSH: tdefl_flush = 2;
pub const TDEFL_NO_FLUSH: tdefl_flush = 0;
pub const TDEFL_MAX_HUFF_SYMBOLS_1: C2RustUnnamed_6 = 32;
pub const TDEFL_MAX_HUFF_SYMBOLS_0: C2RustUnnamed_6 = 288;
pub const TDEFL_NONDETERMINISTIC_PARSING_FLAG: C2RustUnnamed_5 = 32768;
pub const TDEFL_OUT_BUF_SIZE: C2RustUnnamed_7 = 85196;
pub type mz_uint64 = uint64_t;
pub type uint64_t = __uint64_t;
pub const TDEFL_MAX_HUFF_SYMBOLS_2: C2RustUnnamed_6 = 19;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdefl_sym_freq {
    pub m_key: mz_uint16,
    pub m_sym_index: mz_uint16,
}
pub const TDEFL_MAX_SUPPORTED_HUFF_CODESIZE: C2RustUnnamed_10 = 32;
pub const TDEFL_LZ_DICT_SIZE_MASK: C2RustUnnamed_6 = 32767;
pub const TDEFL_LZ_CODE_BUF_SIZE: C2RustUnnamed_7 = 65536;
pub const TDEFL_LZ_DICT_SIZE: C2RustUnnamed_6 = 32768;
pub const TDEFL_MIN_MATCH_LEN: C2RustUnnamed_6 = 3;
pub const TDEFL_MAX_MATCH_LEN: C2RustUnnamed_6 = 258;
pub const TDEFL_LZ_HASH_SIZE: C2RustUnnamed_7 = 32768;
pub const TDEFL_LZ_HASH_SHIFT: C2RustUnnamed_7 = 5;
pub const TDEFL_LEVEL1_HASH_SIZE_MASK: C2RustUnnamed_7 = 4095;
pub const TDEFL_LZ_HASH_BITS: C2RustUnnamed_7 = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub m_decomp: tinfl_decompressor,
    pub m_dict_ofs: mz_uint,
    pub m_dict_avail: mz_uint,
    pub m_first_call: mz_uint,
    pub m_has_flushed: mz_uint,
    pub m_window_bits: libc::c_int,
    pub m_dict: [mz_uint8; 32768],
    pub m_last_status: tinfl_status,
}
pub type tinfl_status = libc::c_int;
pub const TINFL_STATUS_HAS_MORE_OUTPUT: tinfl_status = 2;
pub const TINFL_STATUS_NEEDS_MORE_INPUT: tinfl_status = 1;
pub const TINFL_STATUS_DONE: tinfl_status = 0;
pub const TINFL_STATUS_FAILED: tinfl_status = -1;
pub const TINFL_STATUS_ADLER32_MISMATCH: tinfl_status = -2;
pub const TINFL_STATUS_BAD_PARAM: tinfl_status = -3;
pub const TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS: tinfl_status = -4;
pub type tinfl_decompressor = tinfl_decompressor_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinfl_decompressor_tag {
    pub m_state: mz_uint32,
    pub m_num_bits: mz_uint32,
    pub m_zhdr0: mz_uint32,
    pub m_zhdr1: mz_uint32,
    pub m_z_adler32: mz_uint32,
    pub m_final: mz_uint32,
    pub m_type: mz_uint32,
    pub m_check_adler32: mz_uint32,
    pub m_dist: mz_uint32,
    pub m_counter: mz_uint32,
    pub m_num_extra: mz_uint32,
    pub m_table_sizes: [mz_uint32; 3],
    pub m_bit_buf: tinfl_bit_buf_t,
    pub m_dist_from_out_buf_start: size_t,
    pub m_tables: [tinfl_huff_table; 3],
    pub m_raw_header: [mz_uint8; 4],
    pub m_len_codes: [mz_uint8; 457],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinfl_huff_table {
    pub m_code_size: [mz_uint8; 288],
    pub m_look_up: [mz_int16; 1024],
    pub m_tree: [mz_int16; 576],
}
pub type mz_int16 = libc::c_short;
pub type tinfl_bit_buf_t = mz_uint64;
pub const TINFL_FLAG_COMPUTE_ADLER32: C2RustUnnamed_8 = 8;
pub const TINFL_FLAG_PARSE_ZLIB_HEADER: C2RustUnnamed_8 = 1;
pub const TINFL_FLAG_HAS_MORE_INPUT: C2RustUnnamed_8 = 2;
pub const TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF: C2RustUnnamed_8 = 4;
pub const TINFL_FAST_LOOKUP_BITS: C2RustUnnamed_9 = 10;
pub const TINFL_FAST_LOOKUP_SIZE: C2RustUnnamed_9 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub m_err: libc::c_int,
    pub m_pDesc: *const libc::c_char,
}
pub type Byte = libc::c_uchar;
pub type Bytef = Byte;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const TDEFL_DEFAULT_MAX_PROBES: C2RustUnnamed_4 = 128;
pub const TDEFL_HUFFMAN_ONLY: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdefl_output_buffer {
    pub m_size: size_t,
    pub m_capacity: size_t,
    pub m_pBuf: *mut mz_uint8,
    pub m_expandable: mz_bool,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const TDEFL_MAX_HUFF_TABLES: C2RustUnnamed_6 = 3;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const TDEFL_MAX_HUFF_SYMBOLS: C2RustUnnamed_7 = 288;
pub type C2RustUnnamed_8 = libc::c_uint;
pub type tinfl_put_buf_func_ptr
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: libc::c_int,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const TINFL_MAX_HUFF_SYMBOLS_2: C2RustUnnamed_9 = 19;
pub const TINFL_MAX_HUFF_SYMBOLS_1: C2RustUnnamed_9 = 32;
pub const TINFL_MAX_HUFF_SYMBOLS_0: C2RustUnnamed_9 = 288;
pub const TINFL_MAX_HUFF_TABLES: C2RustUnnamed_9 = 3;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub numfilenames: libc::c_int,
    pub filenames: *mut *mut libc::c_char,
    pub filenamesbuffer: *mut libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_s {
    pub handle: libc::c_int,
    pub ungetc: libc::c_int,
    pub real_length: fs_offset_t,
    pub position: fs_offset_t,
    pub offset: fs_offset_t,
    pub filetime: time_t,
    pub buff_ind: fs_offset_t,
    pub buff_len: fs_offset_t,
    pub buff: [byte; 2048],
}
pub type fs_offset_t = off_t;
pub type file_t = file_s;
// intermediate buffer
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wfile_s {
    pub filename: string,
    pub infotableofs: libc::c_int,
    pub numlumps: libc::c_int,
    pub mempool: poolhandle_t,
    pub handle: *mut file_t,
    pub lumps: *mut dlumpinfo_t,
    pub filetime: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlumpinfo_t {
    pub filepos: libc::c_int,
    pub disksize: libc::c_int,
    pub size: libc::c_int,
    pub type_0: libc::c_schar,
    pub attribs: libc::c_schar,
    pub pad0: libc::c_schar,
    pub pad1: libc::c_schar,
    pub name: [libc::c_char; 16],
}
pub type wfile_t = wfile_s;
pub type word = libc::c_ushort;
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
pub type C2RustUnnamed_11 = libc::c_uint;
pub const GAME_MULTIPLAYER_ONLY: C2RustUnnamed_11 = 2;
pub const GAME_SINGLEPLAYER_ONLY: C2RustUnnamed_11 = 1;
pub const GAME_NORMAL: C2RustUnnamed_11 = 0;
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
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
pub type stringlist_t = stringlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringlist_s {
    pub maxstrings: libc::c_int,
    pub numstrings: libc::c_int,
    pub strings: *mut *mut libc::c_char,
}
pub type zip_t = zip_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_s {
    pub filename: string,
    pub handle: libc::c_int,
    pub numfiles: libc::c_int,
    pub filetime: time_t,
    pub files: *mut zipfile_t,
}
pub type zipfile_t = zipfile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zipfile_s {
    pub name: [libc::c_char; 1024],
    pub offset: fs_offset_t,
    pub size: fs_offset_t,
    pub compressed_size: fs_offset_t,
    pub flags: libc::c_ushort,
}
pub type searchpath_t = searchpath_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct searchpath_s {
    pub filename: string,
    pub pack: *mut pack_t,
    pub wad: *mut wfile_t,
    pub zip: *mut zip_t,
    pub flags: libc::c_int,
    pub next: *mut searchpath_s,
}
pub type pack_t = pack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pack_s {
    pub filename: string,
    pub handle: libc::c_int,
    pub numfiles: libc::c_int,
    pub filetime: time_t,
    pub files: *mut dpackfile_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpackfile_t {
    pub name: [libc::c_char; 56],
    pub filepos: libc::c_int,
    pub filelen: libc::c_int,
}
pub type wadtype_t = wadtype_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wadtype_s {
    pub ext: *const libc::c_char,
    pub type_0: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwadinfo_t {
    pub ident: libc::c_int,
    pub numlumps: libc::c_int,
    pub infotableofs: libc::c_int,
}
pub type zip_header_t = zip_header_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct zip_header_s {
    pub signature: libc::c_uint,
    pub version: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub compression_flags: libc::c_ushort,
    pub dos_date: libc::c_uint,
    pub mz_crc32: libc::c_uint,
    pub compressed_size: libc::c_uint,
    pub uncompressed_size: libc::c_uint,
    pub filename_len: libc::c_ushort,
    pub extrafield_len: libc::c_ushort,
}
pub type zip_cdf_header_t = zip_cdf_header_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct zip_cdf_header_s {
    pub signature: libc::c_uint,
    pub version: libc::c_ushort,
    pub version_need: libc::c_ushort,
    pub generalPurposeBitFlag: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub modification_time: libc::c_ushort,
    pub modification_date: libc::c_ushort,
    pub mz_crc32: libc::c_uint,
    pub compressed_size: libc::c_uint,
    pub uncompressed_size: libc::c_uint,
    pub filename_len: libc::c_ushort,
    pub extrafield_len: libc::c_ushort,
    pub file_commentary_len: libc::c_ushort,
    pub disk_start: libc::c_ushort,
    pub internal_attr: libc::c_ushort,
    pub external_attr: libc::c_uint,
    pub local_header_offset: libc::c_uint,
}
pub type zip_header_eocd_t = zip_header_eocd_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct zip_header_eocd_s {
    pub disk_number: libc::c_ushort,
    pub start_disk_number: libc::c_ushort,
    pub number_central_directory_record: libc::c_ushort,
    pub total_central_directory_record: libc::c_ushort,
    pub size_of_central_directory: libc::c_uint,
    pub central_directory_offset: libc::c_uint,
    pub commentary_len: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpackheader_t {
    pub ident: libc::c_int,
    pub dirofs: libc::c_int,
    pub dirlen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dll_user_s {
    pub hInstance: *mut libc::c_void,
    pub custom_loader: qboolean,
    pub encrypted: qboolean,
    pub dllName: [libc::c_char; 32],
    pub fullPath: string,
    pub shortPath: string,
    pub ordinals: *mut word,
    pub funcs: *mut dword,
    pub names: [*mut libc::c_char; 4096],
    pub num_ordinals: libc::c_int,
    pub funcBase: uintptr_t,
}
pub type dll_user_t = dll_user_s;
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline(always)]
unsafe extern "C" fn tdefl_record_match(mut d: *mut tdefl_compressor,
                                        mut match_len: mz_uint,
                                        mut match_dist: mz_uint) {
    let mut s0: mz_uint32 = 0;
    let mut s1: mz_uint32 = 0;
    if match_len >= TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint &&
           match_dist >= 1 as libc::c_int as libc::c_uint &&
           match_dist <= TDEFL_LZ_DICT_SIZE as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"(match_len >= TDEFL_MIN_MATCH_LEN) && (match_dist >= 1) && (match_dist <= TDEFL_LZ_DICT_SIZE)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2589 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"void tdefl_record_match(tdefl_compressor *, mz_uint, mz_uint)\x00")).as_ptr());
    }
    (*d).m_total_lz_bytes =
        ((*d).m_total_lz_bytes as libc::c_uint).wrapping_add(match_len) as
            mz_uint as mz_uint;
    *(*d).m_pLZ_code_buf.offset(0 as libc::c_int as isize) =
        match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN as libc::c_int as
                                   libc::c_uint) as mz_uint8;
    match_dist =
        (match_dist as
             libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint) as
            mz_uint as mz_uint;
    *(*d).m_pLZ_code_buf.offset(1 as libc::c_int as isize) =
        (match_dist & 0xff as libc::c_int as libc::c_uint) as mz_uint8;
    *(*d).m_pLZ_code_buf.offset(2 as libc::c_int as isize) =
        (match_dist >> 8 as libc::c_int) as mz_uint8;
    (*d).m_pLZ_code_buf =
        (*d).m_pLZ_code_buf.offset(3 as libc::c_int as isize);
    *(*d).m_pLZ_flags =
        (*(*d).m_pLZ_flags as libc::c_int >> 1 as libc::c_int |
             0x80 as libc::c_int) as mz_uint8;
    (*d).m_num_flags_left = (*d).m_num_flags_left.wrapping_sub(1);
    if (*d).m_num_flags_left == 0 as libc::c_int as libc::c_uint {
        (*d).m_num_flags_left = 8 as libc::c_int as mz_uint;
        let fresh0 = (*d).m_pLZ_code_buf;
        (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(1);
        (*d).m_pLZ_flags = fresh0
    }
    s0 =
        s_tdefl_small_dist_sym[(match_dist &
                                    511 as libc::c_int as libc::c_uint) as
                                   usize] as mz_uint32;
    s1 =
        s_tdefl_large_dist_sym[(match_dist >> 8 as libc::c_int &
                                    127 as libc::c_int as libc::c_uint) as
                                   usize] as mz_uint32;
    (*d).m_huff_count[1 as libc::c_int as
                          usize][if match_dist <
                                        512 as libc::c_int as libc::c_uint {
                                     s0
                                 } else { s1 } as usize] =
        (*d).m_huff_count[1 as libc::c_int as
                              usize][if match_dist <
                                            512 as libc::c_int as libc::c_uint
                                        {
                                         s0
                                     } else { s1 } as usize].wrapping_add(1);
    if match_len >= TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint {
        (*d).m_huff_count[0 as libc::c_int as
                              usize][s_tdefl_len_sym[match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                         as usize] as usize] =
            (*d).m_huff_count[0 as libc::c_int as
                                  usize][s_tdefl_len_sym[match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                             as usize] as
                                             usize].wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mz_inflateReset(mut pStream: mz_streamp)
 -> libc::c_int {
    let mut pDecomp: *mut inflate_state = 0 as *mut inflate_state;
    if pStream.is_null() { return MZ_STREAM_ERROR as libc::c_int }
    (*pStream).data_type = 0 as libc::c_int;
    (*pStream).adler = 0 as libc::c_int as mz_ulong;
    (*pStream).msg = 0 as *mut libc::c_char;
    (*pStream).total_in = 0 as libc::c_int as mz_ulong;
    (*pStream).total_out = 0 as libc::c_int as mz_ulong;
    (*pStream).reserved = 0 as libc::c_int as mz_ulong;
    pDecomp = (*pStream).state as *mut inflate_state;
    (*pDecomp).m_decomp.m_state = 0 as libc::c_int as mz_uint32;
    (*pDecomp).m_dict_ofs = 0 as libc::c_int as mz_uint;
    (*pDecomp).m_dict_avail = 0 as libc::c_int as mz_uint;
    (*pDecomp).m_last_status = TINFL_STATUS_NEEDS_MORE_INPUT;
    (*pDecomp).m_first_call = 1 as libc::c_int as mz_uint;
    (*pDecomp).m_has_flushed = 0 as libc::c_int as mz_uint;
    return MZ_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mz_inflate(mut pStream: mz_streamp,
                                    mut flush: libc::c_int) -> libc::c_int {
    let mut pState: *mut inflate_state = 0 as *mut inflate_state;
    let mut n: mz_uint = 0;
    let mut first_call: mz_uint = 0;
    let mut decomp_flags: mz_uint =
        TINFL_FLAG_COMPUTE_ADLER32 as libc::c_int as mz_uint;
    let mut in_bytes: size_t = 0;
    let mut out_bytes: size_t = 0;
    let mut orig_avail_in: size_t = 0;
    let mut status: tinfl_status = TINFL_STATUS_DONE;
    if pStream.is_null() || (*pStream).state.is_null() {
        return MZ_STREAM_ERROR as libc::c_int
    }
    if flush == MZ_PARTIAL_FLUSH as libc::c_int {
        flush = MZ_SYNC_FLUSH as libc::c_int
    }
    if flush != 0 && flush != MZ_SYNC_FLUSH as libc::c_int &&
           flush != MZ_FINISH as libc::c_int {
        return MZ_STREAM_ERROR as libc::c_int
    }
    pState = (*pStream).state as *mut inflate_state;
    if (*pState).m_window_bits > 0 as libc::c_int {
        decomp_flags |=
            TINFL_FLAG_PARSE_ZLIB_HEADER as libc::c_int as libc::c_uint
    }
    orig_avail_in = (*pStream).avail_in as size_t;
    first_call = (*pState).m_first_call;
    (*pState).m_first_call = 0 as libc::c_int as mz_uint;
    if ((*pState).m_last_status as libc::c_int) < 0 as libc::c_int {
        return MZ_DATA_ERROR as libc::c_int
    }
    if (*pState).m_has_flushed != 0 && flush != MZ_FINISH as libc::c_int {
        return MZ_STREAM_ERROR as libc::c_int
    }
    (*pState).m_has_flushed |=
        (flush == MZ_FINISH as libc::c_int) as libc::c_int as libc::c_uint;
    if flush == MZ_FINISH as libc::c_int && first_call != 0 {
        decomp_flags |=
            TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as libc::c_int as
                libc::c_uint;
        in_bytes = (*pStream).avail_in as size_t;
        out_bytes = (*pStream).avail_out as size_t;
        status =
            tinfl_decompress(&mut (*pState).m_decomp, (*pStream).next_in,
                             &mut in_bytes, (*pStream).next_out,
                             (*pStream).next_out, &mut out_bytes,
                             decomp_flags);
        (*pState).m_last_status = status;
        (*pStream).next_in =
            (*pStream).next_in.offset(in_bytes as mz_uint as isize);
        (*pStream).avail_in =
            (*pStream).avail_in.wrapping_sub(in_bytes as mz_uint);
        (*pStream).total_in =
            ((*pStream).total_in as
                 libc::c_ulong).wrapping_add(in_bytes as mz_uint as
                                                 libc::c_ulong) as mz_ulong as
                mz_ulong;
        (*pStream).adler = (*pState).m_decomp.m_check_adler32 as mz_ulong;
        (*pStream).next_out =
            (*pStream).next_out.offset(out_bytes as mz_uint as isize);
        (*pStream).avail_out =
            (*pStream).avail_out.wrapping_sub(out_bytes as mz_uint);
        (*pStream).total_out =
            ((*pStream).total_out as
                 libc::c_ulong).wrapping_add(out_bytes as mz_uint as
                                                 libc::c_ulong) as mz_ulong as
                mz_ulong;
        if (status as libc::c_int) < 0 as libc::c_int {
            return MZ_DATA_ERROR as libc::c_int
        } else {
            if status as libc::c_int != TINFL_STATUS_DONE as libc::c_int {
                (*pState).m_last_status = TINFL_STATUS_FAILED;
                return MZ_BUF_ERROR as libc::c_int
            }
        }
        return MZ_STREAM_END as libc::c_int
    }
    if flush != MZ_FINISH as libc::c_int {
        decomp_flags |=
            TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as libc::c_uint
    }
    if (*pState).m_dict_avail != 0 {
        n =
            if (*pState).m_dict_avail < (*pStream).avail_out {
                (*pState).m_dict_avail
            } else { (*pStream).avail_out };
        memcpy((*pStream).next_out as *mut libc::c_void,
               (*pState).m_dict.as_mut_ptr().offset((*pState).m_dict_ofs as
                                                        isize) as
                   *const libc::c_void, n as libc::c_ulong);
        (*pStream).next_out = (*pStream).next_out.offset(n as isize);
        (*pStream).avail_out = (*pStream).avail_out.wrapping_sub(n);
        (*pStream).total_out =
            ((*pStream).total_out as
                 libc::c_ulong).wrapping_add(n as libc::c_ulong) as mz_ulong
                as mz_ulong;
        (*pState).m_dict_avail =
            ((*pState).m_dict_avail as libc::c_uint).wrapping_sub(n) as
                mz_uint as mz_uint;
        (*pState).m_dict_ofs =
            (*pState).m_dict_ofs.wrapping_add(n) &
                (32768 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        return if (*pState).m_last_status as libc::c_int ==
                      TINFL_STATUS_DONE as libc::c_int &&
                      (*pState).m_dict_avail == 0 {
                   MZ_STREAM_END as libc::c_int
               } else { MZ_OK as libc::c_int }
    }
    loop  {
        in_bytes = (*pStream).avail_in as size_t;
        out_bytes =
            (32768 as libc::c_int as
                 libc::c_uint).wrapping_sub((*pState).m_dict_ofs) as size_t;
        status =
            tinfl_decompress(&mut (*pState).m_decomp, (*pStream).next_in,
                             &mut in_bytes, (*pState).m_dict.as_mut_ptr(),
                             (*pState).m_dict.as_mut_ptr().offset((*pState).m_dict_ofs
                                                                      as
                                                                      isize),
                             &mut out_bytes, decomp_flags);
        (*pState).m_last_status = status;
        (*pStream).next_in =
            (*pStream).next_in.offset(in_bytes as mz_uint as isize);
        (*pStream).avail_in =
            (*pStream).avail_in.wrapping_sub(in_bytes as mz_uint);
        (*pStream).total_in =
            ((*pStream).total_in as
                 libc::c_ulong).wrapping_add(in_bytes as mz_uint as
                                                 libc::c_ulong) as mz_ulong as
                mz_ulong;
        (*pStream).adler = (*pState).m_decomp.m_check_adler32 as mz_ulong;
        (*pState).m_dict_avail = out_bytes as mz_uint;
        n =
            if (*pState).m_dict_avail < (*pStream).avail_out {
                (*pState).m_dict_avail
            } else { (*pStream).avail_out };
        memcpy((*pStream).next_out as *mut libc::c_void,
               (*pState).m_dict.as_mut_ptr().offset((*pState).m_dict_ofs as
                                                        isize) as
                   *const libc::c_void, n as libc::c_ulong);
        (*pStream).next_out = (*pStream).next_out.offset(n as isize);
        (*pStream).avail_out = (*pStream).avail_out.wrapping_sub(n);
        (*pStream).total_out =
            ((*pStream).total_out as
                 libc::c_ulong).wrapping_add(n as libc::c_ulong) as mz_ulong
                as mz_ulong;
        (*pState).m_dict_avail =
            ((*pState).m_dict_avail as libc::c_uint).wrapping_sub(n) as
                mz_uint as mz_uint;
        (*pState).m_dict_ofs =
            (*pState).m_dict_ofs.wrapping_add(n) &
                (32768 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        if (status as libc::c_int) < 0 as libc::c_int {
            return MZ_DATA_ERROR as libc::c_int
        } else if status as libc::c_int ==
                      TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int &&
                      orig_avail_in == 0 {
            return MZ_BUF_ERROR as libc::c_int
        } else if flush == MZ_FINISH as libc::c_int {
            if status as libc::c_int == TINFL_STATUS_DONE as libc::c_int {
                return if (*pState).m_dict_avail != 0 {
                           MZ_BUF_ERROR as libc::c_int
                       } else { MZ_STREAM_END as libc::c_int }
            } else {
                if (*pStream).avail_out == 0 {
                    return MZ_BUF_ERROR as libc::c_int
                }
            }
        } else if status as libc::c_int == TINFL_STATUS_DONE as libc::c_int ||
                      (*pStream).avail_in == 0 || (*pStream).avail_out == 0 ||
                      (*pState).m_dict_avail != 0 {
            break ;
        }
    }
    return if status as libc::c_int == TINFL_STATUS_DONE as libc::c_int &&
                  (*pState).m_dict_avail == 0 {
               MZ_STREAM_END as libc::c_int
           } else { MZ_OK as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn mz_inflateEnd(mut pStream: mz_streamp)
 -> libc::c_int {
    if pStream.is_null() { return MZ_STREAM_ERROR as libc::c_int }
    if !(*pStream).state.is_null() {
        (*pStream).zfree.expect("non-null function pointer")((*pStream).opaque,
                                                             (*pStream).state
                                                                 as
                                                                 *mut libc::c_void);
        (*pStream).state = 0 as *mut mz_internal_state
    }
    return MZ_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mz_uncompress(mut pDest: *mut libc::c_uchar,
                                       mut pDest_len: *mut mz_ulong,
                                       mut pSource: *const libc::c_uchar,
                                       mut source_len: mz_ulong)
 -> libc::c_int {
    let mut stream: mz_stream =
        mz_stream{next_in: 0 as *const libc::c_uchar,
                  avail_in: 0,
                  total_in: 0,
                  next_out: 0 as *mut libc::c_uchar,
                  avail_out: 0,
                  total_out: 0,
                  msg: 0 as *mut libc::c_char,
                  state: 0 as *mut mz_internal_state,
                  zalloc: None,
                  zfree: None,
                  opaque: 0 as *mut libc::c_void,
                  data_type: 0,
                  adler: 0,
                  reserved: 0,};
    let mut status: libc::c_int = 0;
    memset(&mut stream as *mut mz_stream as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mz_stream>() as libc::c_ulong);
    if source_len | *pDest_len > 0xffffffff as libc::c_uint as libc::c_ulong {
        return MZ_PARAM_ERROR as libc::c_int
    }
    stream.next_in = pSource;
    stream.avail_in = source_len as mz_uint32;
    stream.next_out = pDest;
    stream.avail_out = *pDest_len as mz_uint32;
    status = mz_inflateInit(&mut stream);
    if status != MZ_OK as libc::c_int { return status }
    status = mz_inflate(&mut stream, MZ_FINISH as libc::c_int);
    if status != MZ_STREAM_END as libc::c_int {
        mz_inflateEnd(&mut stream);
        return if status == MZ_BUF_ERROR as libc::c_int &&
                      stream.avail_in == 0 {
                   MZ_DATA_ERROR as libc::c_int
               } else { status }
    }
    *pDest_len = stream.total_out;
    return mz_inflateEnd(&mut stream);
}
#[no_mangle]
pub unsafe extern "C" fn mz_error(mut err: libc::c_int)
 -> *const libc::c_char {
    static mut s_error_descs: [C2RustUnnamed_3; 10] =
        [{
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_OK as libc::c_int,
                                 m_pDesc:
                                     b"\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_STREAM_END as libc::c_int,
                                 m_pDesc:
                                     b"stream end\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_NEED_DICT as libc::c_int,
                                 m_pDesc:
                                     b"need dictionary\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_ERRNO as libc::c_int,
                                 m_pDesc:
                                     b"file error\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_STREAM_ERROR as libc::c_int,
                                 m_pDesc:
                                     b"stream error\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_DATA_ERROR as libc::c_int,
                                 m_pDesc:
                                     b"data error\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_MEM_ERROR as libc::c_int,
                                 m_pDesc:
                                     b"out of memory\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_BUF_ERROR as libc::c_int,
                                 m_pDesc:
                                     b"buf error\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_VERSION_ERROR as libc::c_int,
                                 m_pDesc:
                                     b"version error\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_3{m_err: MZ_PARAM_ERROR as libc::c_int,
                                 m_pDesc:
                                     b"parameter error\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         }];
    let mut i: mz_uint = 0;
    i = 0 as libc::c_int as mz_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[C2RustUnnamed_3; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>()
                                                   as libc::c_ulong) {
        if s_error_descs[i as usize].m_err == err {
            return s_error_descs[i as usize].m_pDesc
        }
        i = i.wrapping_add(1)
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mz_inflateInit2(mut pStream: mz_streamp,
                                         mut window_bits: libc::c_int)
 -> libc::c_int {
    let mut pDecomp: *mut inflate_state = 0 as *mut inflate_state;
    if pStream.is_null() { return MZ_STREAM_ERROR as libc::c_int }
    if window_bits != 15 as libc::c_int && -window_bits != 15 as libc::c_int {
        return MZ_PARAM_ERROR as libc::c_int
    }
    (*pStream).data_type = 0 as libc::c_int;
    (*pStream).adler = 0 as libc::c_int as mz_ulong;
    (*pStream).msg = 0 as *mut libc::c_char;
    (*pStream).total_in = 0 as libc::c_int as mz_ulong;
    (*pStream).total_out = 0 as libc::c_int as mz_ulong;
    (*pStream).reserved = 0 as libc::c_int as mz_ulong;
    if (*pStream).zalloc.is_none() {
        (*pStream).zalloc =
            Some(miniz_def_alloc_func as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                          _: size_t) -> *mut libc::c_void)
    }
    if (*pStream).zfree.is_none() {
        (*pStream).zfree =
            Some(miniz_def_free_func as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut libc::c_void) -> ())
    }
    pDecomp =
        (*pStream).zalloc.expect("non-null function pointer")((*pStream).opaque,
                                                              1 as libc::c_int
                                                                  as size_t,
                                                              ::std::mem::size_of::<inflate_state>()
                                                                  as
                                                                  libc::c_ulong)
            as *mut inflate_state;
    if pDecomp.is_null() { return MZ_MEM_ERROR as libc::c_int }
    (*pStream).state = pDecomp as *mut mz_internal_state;
    (*pDecomp).m_decomp.m_state = 0 as libc::c_int as mz_uint32;
    (*pDecomp).m_dict_ofs = 0 as libc::c_int as mz_uint;
    (*pDecomp).m_dict_avail = 0 as libc::c_int as mz_uint;
    (*pDecomp).m_last_status = TINFL_STATUS_NEEDS_MORE_INPUT;
    (*pDecomp).m_first_call = 1 as libc::c_int as mz_uint;
    (*pDecomp).m_has_flushed = 0 as libc::c_int as mz_uint;
    (*pDecomp).m_window_bits = window_bits;
    return MZ_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mz_inflateInit(mut pStream: mz_streamp)
 -> libc::c_int {
    return mz_inflateInit2(pStream, 15 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mz_compressBound(mut source_len: mz_ulong)
 -> mz_ulong {
    return mz_deflateBound(0 as mz_streamp, source_len);
}
#[no_mangle]
pub unsafe extern "C" fn mz_compress2(mut pDest: *mut libc::c_uchar,
                                      mut pDest_len: *mut mz_ulong,
                                      mut pSource: *const libc::c_uchar,
                                      mut source_len: mz_ulong,
                                      mut level: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut stream: mz_stream =
        mz_stream{next_in: 0 as *const libc::c_uchar,
                  avail_in: 0,
                  total_in: 0,
                  next_out: 0 as *mut libc::c_uchar,
                  avail_out: 0,
                  total_out: 0,
                  msg: 0 as *mut libc::c_char,
                  state: 0 as *mut mz_internal_state,
                  zalloc: None,
                  zfree: None,
                  opaque: 0 as *mut libc::c_void,
                  data_type: 0,
                  adler: 0,
                  reserved: 0,};
    memset(&mut stream as *mut mz_stream as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mz_stream>() as libc::c_ulong);
    if source_len | *pDest_len > 0xffffffff as libc::c_uint as libc::c_ulong {
        return MZ_PARAM_ERROR as libc::c_int
    }
    stream.next_in = pSource;
    stream.avail_in = source_len as mz_uint32;
    stream.next_out = pDest;
    stream.avail_out = *pDest_len as mz_uint32;
    status = mz_deflateInit(&mut stream, level);
    if status != MZ_OK as libc::c_int { return status }
    status = mz_deflate(&mut stream, MZ_FINISH as libc::c_int);
    if status != MZ_STREAM_END as libc::c_int {
        mz_deflateEnd(&mut stream);
        return if status == MZ_OK as libc::c_int {
                   MZ_BUF_ERROR as libc::c_int
               } else { status }
    }
    *pDest_len = stream.total_out;
    return mz_deflateEnd(&mut stream);
}
#[no_mangle]
pub unsafe extern "C" fn miniz_def_realloc_func(mut opaque: *mut libc::c_void,
                                                mut address:
                                                    *mut libc::c_void,
                                                mut items: size_t,
                                                mut size: size_t)
 -> *mut libc::c_void {
    return realloc(address, items.wrapping_mul(size));
}
#[no_mangle]
pub unsafe extern "C" fn mz_deflateInit2(mut pStream: mz_streamp,
                                         mut level: libc::c_int,
                                         mut method: libc::c_int,
                                         mut window_bits: libc::c_int,
                                         mut mem_level: libc::c_int,
                                         mut strategy: libc::c_int)
 -> libc::c_int {
    let mut pComp: *mut tdefl_compressor = 0 as *mut tdefl_compressor;
    let mut comp_flags: mz_uint =
        TDEFL_COMPUTE_ADLER32 as libc::c_int as libc::c_uint |
            tdefl_create_comp_flags_from_zip_params(level, window_bits,
                                                    strategy);
    if pStream.is_null() { return MZ_STREAM_ERROR as libc::c_int }
    if method != 8 as libc::c_int ||
           (mem_level < 1 as libc::c_int || mem_level > 9 as libc::c_int) ||
           window_bits != 15 as libc::c_int &&
               -window_bits != 15 as libc::c_int {
        return MZ_PARAM_ERROR as libc::c_int
    }
    (*pStream).data_type = 0 as libc::c_int;
    (*pStream).adler = 1 as libc::c_int as mz_ulong;
    (*pStream).msg = 0 as *mut libc::c_char;
    (*pStream).reserved = 0 as libc::c_int as mz_ulong;
    (*pStream).total_in = 0 as libc::c_int as mz_ulong;
    (*pStream).total_out = 0 as libc::c_int as mz_ulong;
    if (*pStream).zalloc.is_none() {
        (*pStream).zalloc =
            Some(miniz_def_alloc_func as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                          _: size_t) -> *mut libc::c_void)
    }
    if (*pStream).zfree.is_none() {
        (*pStream).zfree =
            Some(miniz_def_free_func as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut libc::c_void) -> ())
    }
    pComp =
        (*pStream).zalloc.expect("non-null function pointer")((*pStream).opaque,
                                                              1 as libc::c_int
                                                                  as size_t,
                                                              ::std::mem::size_of::<tdefl_compressor>()
                                                                  as
                                                                  libc::c_ulong)
            as *mut tdefl_compressor;
    if pComp.is_null() { return MZ_MEM_ERROR as libc::c_int }
    (*pStream).state = pComp as *mut mz_internal_state;
    if tdefl_init(pComp, None, 0 as *mut libc::c_void,
                  comp_flags as libc::c_int) as libc::c_int !=
           TDEFL_STATUS_OKAY as libc::c_int {
        mz_deflateEnd(pStream);
        return MZ_PARAM_ERROR as libc::c_int
    }
    return MZ_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mz_free(mut p: *mut libc::c_void) { free(p); }
#[no_mangle]
pub unsafe extern "C" fn mz_adler32(mut adler: mz_ulong,
                                    mut ptr: *const libc::c_uchar,
                                    mut buf_len: size_t) -> mz_ulong {
    let mut i: mz_uint32 = 0;
    let mut s1: mz_uint32 =
        (adler & 0xffff as libc::c_int as libc::c_ulong) as mz_uint32;
    let mut s2: mz_uint32 = (adler >> 16 as libc::c_int) as mz_uint32;
    let mut block_len: size_t =
        buf_len.wrapping_rem(5552 as libc::c_int as libc::c_ulong);
    if ptr.is_null() { return 1 as libc::c_int as mz_ulong }
    while buf_len != 0 {
        i = 0 as libc::c_int as mz_uint32;
        while (i.wrapping_add(7 as libc::c_int as libc::c_uint) as
                   libc::c_ulong) < block_len {
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(0 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(1 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(2 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(3 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(4 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(5 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(6 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            s1 =
                (s1 as
                     libc::c_uint).wrapping_add(*ptr.offset(7 as libc::c_int
                                                                as isize) as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            i =
                (i as
                     libc::c_uint).wrapping_add(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint32
                    as mz_uint32;
            ptr = ptr.offset(8 as libc::c_int as isize)
        }
        while (i as libc::c_ulong) < block_len {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            s1 =
                (s1 as libc::c_uint).wrapping_add(*fresh1 as libc::c_uint) as
                    mz_uint32 as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                    mz_uint32;
            i = i.wrapping_add(1)
        }
        s1 =
            (s1 as libc::c_uint).wrapping_rem(65521 as libc::c_uint) as
                mz_uint32 as mz_uint32;
        s2 =
            (s2 as libc::c_uint).wrapping_rem(65521 as libc::c_uint) as
                mz_uint32 as mz_uint32;
        buf_len =
            (buf_len as libc::c_ulong).wrapping_sub(block_len) as size_t as
                size_t;
        block_len = 5552 as libc::c_int as size_t
    }
    return (s2 << 16 as libc::c_int).wrapping_add(s1) as mz_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn mz_compress(mut pDest: *mut libc::c_uchar,
                                     mut pDest_len: *mut mz_ulong,
                                     mut pSource: *const libc::c_uchar,
                                     mut source_len: mz_ulong)
 -> libc::c_int {
    return mz_compress2(pDest, pDest_len, pSource, source_len,
                        MZ_DEFAULT_COMPRESSION as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn miniz_def_free_func(mut opaque: *mut libc::c_void,
                                             mut address: *mut libc::c_void) {
    free(address);
}
#[no_mangle]
pub unsafe extern "C" fn mz_crc32(mut crc: mz_ulong, mut ptr: *const mz_uint8,
                                  mut buf_len: size_t) -> mz_ulong {
    static mut s_crc_table: [mz_uint32; 256] =
        [0 as libc::c_int as mz_uint32,
         0x77073096 as libc::c_int as mz_uint32, 0xee0e612c as libc::c_uint,
         0x990951ba as libc::c_uint, 0x76dc419 as libc::c_int as mz_uint32,
         0x706af48f as libc::c_int as mz_uint32, 0xe963a535 as libc::c_uint,
         0x9e6495a3 as libc::c_uint, 0xedb8832 as libc::c_int as mz_uint32,
         0x79dcb8a4 as libc::c_int as mz_uint32, 0xe0d5e91e as libc::c_uint,
         0x97d2d988 as libc::c_uint, 0x9b64c2b as libc::c_int as mz_uint32,
         0x7eb17cbd as libc::c_int as mz_uint32, 0xe7b82d07 as libc::c_uint,
         0x90bf1d91 as libc::c_uint, 0x1db71064 as libc::c_int as mz_uint32,
         0x6ab020f2 as libc::c_int as mz_uint32, 0xf3b97148 as libc::c_uint,
         0x84be41de as libc::c_uint, 0x1adad47d as libc::c_int as mz_uint32,
         0x6ddde4eb as libc::c_int as mz_uint32, 0xf4d4b551 as libc::c_uint,
         0x83d385c7 as libc::c_uint, 0x136c9856 as libc::c_int as mz_uint32,
         0x646ba8c0 as libc::c_int as mz_uint32, 0xfd62f97a as libc::c_uint,
         0x8a65c9ec as libc::c_uint, 0x14015c4f as libc::c_int as mz_uint32,
         0x63066cd9 as libc::c_int as mz_uint32, 0xfa0f3d63 as libc::c_uint,
         0x8d080df5 as libc::c_uint, 0x3b6e20c8 as libc::c_int as mz_uint32,
         0x4c69105e as libc::c_int as mz_uint32, 0xd56041e4 as libc::c_uint,
         0xa2677172 as libc::c_uint, 0x3c03e4d1 as libc::c_int as mz_uint32,
         0x4b04d447 as libc::c_int as mz_uint32, 0xd20d85fd as libc::c_uint,
         0xa50ab56b as libc::c_uint, 0x35b5a8fa as libc::c_int as mz_uint32,
         0x42b2986c as libc::c_int as mz_uint32, 0xdbbbc9d6 as libc::c_uint,
         0xacbcf940 as libc::c_uint, 0x32d86ce3 as libc::c_int as mz_uint32,
         0x45df5c75 as libc::c_int as mz_uint32, 0xdcd60dcf as libc::c_uint,
         0xabd13d59 as libc::c_uint, 0x26d930ac as libc::c_int as mz_uint32,
         0x51de003a as libc::c_int as mz_uint32, 0xc8d75180 as libc::c_uint,
         0xbfd06116 as libc::c_uint, 0x21b4f4b5 as libc::c_int as mz_uint32,
         0x56b3c423 as libc::c_int as mz_uint32, 0xcfba9599 as libc::c_uint,
         0xb8bda50f as libc::c_uint, 0x2802b89e as libc::c_int as mz_uint32,
         0x5f058808 as libc::c_int as mz_uint32, 0xc60cd9b2 as libc::c_uint,
         0xb10be924 as libc::c_uint, 0x2f6f7c87 as libc::c_int as mz_uint32,
         0x58684c11 as libc::c_int as mz_uint32, 0xc1611dab as libc::c_uint,
         0xb6662d3d as libc::c_uint, 0x76dc4190 as libc::c_int as mz_uint32,
         0x1db7106 as libc::c_int as mz_uint32, 0x98d220bc as libc::c_uint,
         0xefd5102a as libc::c_uint, 0x71b18589 as libc::c_int as mz_uint32,
         0x6b6b51f as libc::c_int as mz_uint32, 0x9fbfe4a5 as libc::c_uint,
         0xe8b8d433 as libc::c_uint, 0x7807c9a2 as libc::c_int as mz_uint32,
         0xf00f934 as libc::c_int as mz_uint32, 0x9609a88e as libc::c_uint,
         0xe10e9818 as libc::c_uint, 0x7f6a0dbb as libc::c_int as mz_uint32,
         0x86d3d2d as libc::c_int as mz_uint32, 0x91646c97 as libc::c_uint,
         0xe6635c01 as libc::c_uint, 0x6b6b51f4 as libc::c_int as mz_uint32,
         0x1c6c6162 as libc::c_int as mz_uint32, 0x856530d8 as libc::c_uint,
         0xf262004e as libc::c_uint, 0x6c0695ed as libc::c_int as mz_uint32,
         0x1b01a57b as libc::c_int as mz_uint32, 0x8208f4c1 as libc::c_uint,
         0xf50fc457 as libc::c_uint, 0x65b0d9c6 as libc::c_int as mz_uint32,
         0x12b7e950 as libc::c_int as mz_uint32, 0x8bbeb8ea as libc::c_uint,
         0xfcb9887c as libc::c_uint, 0x62dd1ddf as libc::c_int as mz_uint32,
         0x15da2d49 as libc::c_int as mz_uint32, 0x8cd37cf3 as libc::c_uint,
         0xfbd44c65 as libc::c_uint, 0x4db26158 as libc::c_int as mz_uint32,
         0x3ab551ce as libc::c_int as mz_uint32, 0xa3bc0074 as libc::c_uint,
         0xd4bb30e2 as libc::c_uint, 0x4adfa541 as libc::c_int as mz_uint32,
         0x3dd895d7 as libc::c_int as mz_uint32, 0xa4d1c46d as libc::c_uint,
         0xd3d6f4fb as libc::c_uint, 0x4369e96a as libc::c_int as mz_uint32,
         0x346ed9fc as libc::c_int as mz_uint32, 0xad678846 as libc::c_uint,
         0xda60b8d0 as libc::c_uint, 0x44042d73 as libc::c_int as mz_uint32,
         0x33031de5 as libc::c_int as mz_uint32, 0xaa0a4c5f as libc::c_uint,
         0xdd0d7cc9 as libc::c_uint, 0x5005713c as libc::c_int as mz_uint32,
         0x270241aa as libc::c_int as mz_uint32, 0xbe0b1010 as libc::c_uint,
         0xc90c2086 as libc::c_uint, 0x5768b525 as libc::c_int as mz_uint32,
         0x206f85b3 as libc::c_int as mz_uint32, 0xb966d409 as libc::c_uint,
         0xce61e49f as libc::c_uint, 0x5edef90e as libc::c_int as mz_uint32,
         0x29d9c998 as libc::c_int as mz_uint32, 0xb0d09822 as libc::c_uint,
         0xc7d7a8b4 as libc::c_uint, 0x59b33d17 as libc::c_int as mz_uint32,
         0x2eb40d81 as libc::c_int as mz_uint32, 0xb7bd5c3b as libc::c_uint,
         0xc0ba6cad as libc::c_uint, 0xedb88320 as libc::c_uint,
         0x9abfb3b6 as libc::c_uint, 0x3b6e20c as libc::c_int as mz_uint32,
         0x74b1d29a as libc::c_int as mz_uint32, 0xead54739 as libc::c_uint,
         0x9dd277af as libc::c_uint, 0x4db2615 as libc::c_int as mz_uint32,
         0x73dc1683 as libc::c_int as mz_uint32, 0xe3630b12 as libc::c_uint,
         0x94643b84 as libc::c_uint, 0xd6d6a3e as libc::c_int as mz_uint32,
         0x7a6a5aa8 as libc::c_int as mz_uint32, 0xe40ecf0b as libc::c_uint,
         0x9309ff9d as libc::c_uint, 0xa00ae27 as libc::c_int as mz_uint32,
         0x7d079eb1 as libc::c_int as mz_uint32, 0xf00f9344 as libc::c_uint,
         0x8708a3d2 as libc::c_uint, 0x1e01f268 as libc::c_int as mz_uint32,
         0x6906c2fe as libc::c_int as mz_uint32, 0xf762575d as libc::c_uint,
         0x806567cb as libc::c_uint, 0x196c3671 as libc::c_int as mz_uint32,
         0x6e6b06e7 as libc::c_int as mz_uint32, 0xfed41b76 as libc::c_uint,
         0x89d32be0 as libc::c_uint, 0x10da7a5a as libc::c_int as mz_uint32,
         0x67dd4acc as libc::c_int as mz_uint32, 0xf9b9df6f as libc::c_uint,
         0x8ebeeff9 as libc::c_uint, 0x17b7be43 as libc::c_int as mz_uint32,
         0x60b08ed5 as libc::c_int as mz_uint32, 0xd6d6a3e8 as libc::c_uint,
         0xa1d1937e as libc::c_uint, 0x38d8c2c4 as libc::c_int as mz_uint32,
         0x4fdff252 as libc::c_int as mz_uint32, 0xd1bb67f1 as libc::c_uint,
         0xa6bc5767 as libc::c_uint, 0x3fb506dd as libc::c_int as mz_uint32,
         0x48b2364b as libc::c_int as mz_uint32, 0xd80d2bda as libc::c_uint,
         0xaf0a1b4c as libc::c_uint, 0x36034af6 as libc::c_int as mz_uint32,
         0x41047a60 as libc::c_int as mz_uint32, 0xdf60efc3 as libc::c_uint,
         0xa867df55 as libc::c_uint, 0x316e8eef as libc::c_int as mz_uint32,
         0x4669be79 as libc::c_int as mz_uint32, 0xcb61b38c as libc::c_uint,
         0xbc66831a as libc::c_uint, 0x256fd2a0 as libc::c_int as mz_uint32,
         0x5268e236 as libc::c_int as mz_uint32, 0xcc0c7795 as libc::c_uint,
         0xbb0b4703 as libc::c_uint, 0x220216b9 as libc::c_int as mz_uint32,
         0x5505262f as libc::c_int as mz_uint32, 0xc5ba3bbe as libc::c_uint,
         0xb2bd0b28 as libc::c_uint, 0x2bb45a92 as libc::c_int as mz_uint32,
         0x5cb36a04 as libc::c_int as mz_uint32, 0xc2d7ffa7 as libc::c_uint,
         0xb5d0cf31 as libc::c_uint, 0x2cd99e8b as libc::c_int as mz_uint32,
         0x5bdeae1d as libc::c_int as mz_uint32, 0x9b64c2b0 as libc::c_uint,
         0xec63f226 as libc::c_uint, 0x756aa39c as libc::c_int as mz_uint32,
         0x26d930a as libc::c_int as mz_uint32, 0x9c0906a9 as libc::c_uint,
         0xeb0e363f as libc::c_uint, 0x72076785 as libc::c_int as mz_uint32,
         0x5005713 as libc::c_int as mz_uint32, 0x95bf4a82 as libc::c_uint,
         0xe2b87a14 as libc::c_uint, 0x7bb12bae as libc::c_int as mz_uint32,
         0xcb61b38 as libc::c_int as mz_uint32, 0x92d28e9b as libc::c_uint,
         0xe5d5be0d as libc::c_uint, 0x7cdcefb7 as libc::c_int as mz_uint32,
         0xbdbdf21 as libc::c_int as mz_uint32, 0x86d3d2d4 as libc::c_uint,
         0xf1d4e242 as libc::c_uint, 0x68ddb3f8 as libc::c_int as mz_uint32,
         0x1fda836e as libc::c_int as mz_uint32, 0x81be16cd as libc::c_uint,
         0xf6b9265b as libc::c_uint, 0x6fb077e1 as libc::c_int as mz_uint32,
         0x18b74777 as libc::c_int as mz_uint32, 0x88085ae6 as libc::c_uint,
         0xff0f6a70 as libc::c_uint, 0x66063bca as libc::c_int as mz_uint32,
         0x11010b5c as libc::c_int as mz_uint32, 0x8f659eff as libc::c_uint,
         0xf862ae69 as libc::c_uint, 0x616bffd3 as libc::c_int as mz_uint32,
         0x166ccf45 as libc::c_int as mz_uint32, 0xa00ae278 as libc::c_uint,
         0xd70dd2ee as libc::c_uint, 0x4e048354 as libc::c_int as mz_uint32,
         0x3903b3c2 as libc::c_int as mz_uint32, 0xa7672661 as libc::c_uint,
         0xd06016f7 as libc::c_uint, 0x4969474d as libc::c_int as mz_uint32,
         0x3e6e77db as libc::c_int as mz_uint32, 0xaed16a4a as libc::c_uint,
         0xd9d65adc as libc::c_uint, 0x40df0b66 as libc::c_int as mz_uint32,
         0x37d83bf0 as libc::c_int as mz_uint32, 0xa9bcae53 as libc::c_uint,
         0xdebb9ec5 as libc::c_uint, 0x47b2cf7f as libc::c_int as mz_uint32,
         0x30b5ffe9 as libc::c_int as mz_uint32, 0xbdbdf21c as libc::c_uint,
         0xcabac28a as libc::c_uint, 0x53b39330 as libc::c_int as mz_uint32,
         0x24b4a3a6 as libc::c_int as mz_uint32, 0xbad03605 as libc::c_uint,
         0xcdd70693 as libc::c_uint, 0x54de5729 as libc::c_int as mz_uint32,
         0x23d967bf as libc::c_int as mz_uint32, 0xb3667a2e as libc::c_uint,
         0xc4614ab8 as libc::c_uint, 0x5d681b02 as libc::c_int as mz_uint32,
         0x2a6f2b94 as libc::c_int as mz_uint32, 0xb40bbe37 as libc::c_uint,
         0xc30c8ea1 as libc::c_uint, 0x5a05df1b as libc::c_int as mz_uint32,
         0x2d02ef8d as libc::c_int as mz_uint32];
    let mut mz_crc32_0: mz_uint32 =
        crc as mz_uint32 ^ 0xffffffff as libc::c_uint;
    let mut pByte_buf: *const mz_uint8 = ptr;
    while buf_len >= 4 as libc::c_int as libc::c_ulong {
        mz_crc32_0 =
            mz_crc32_0 >> 8 as libc::c_int ^
                s_crc_table[((mz_crc32_0 ^
                                  *pByte_buf.offset(0 as libc::c_int as isize)
                                      as libc::c_uint) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
        mz_crc32_0 =
            mz_crc32_0 >> 8 as libc::c_int ^
                s_crc_table[((mz_crc32_0 ^
                                  *pByte_buf.offset(1 as libc::c_int as isize)
                                      as libc::c_uint) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
        mz_crc32_0 =
            mz_crc32_0 >> 8 as libc::c_int ^
                s_crc_table[((mz_crc32_0 ^
                                  *pByte_buf.offset(2 as libc::c_int as isize)
                                      as libc::c_uint) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
        mz_crc32_0 =
            mz_crc32_0 >> 8 as libc::c_int ^
                s_crc_table[((mz_crc32_0 ^
                                  *pByte_buf.offset(3 as libc::c_int as isize)
                                      as libc::c_uint) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
        pByte_buf = pByte_buf.offset(4 as libc::c_int as isize);
        buf_len =
            (buf_len as
                 libc::c_ulong).wrapping_sub(4 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    while buf_len != 0 {
        mz_crc32_0 =
            mz_crc32_0 >> 8 as libc::c_int ^
                s_crc_table[((mz_crc32_0 ^
                                  *pByte_buf.offset(0 as libc::c_int as isize)
                                      as libc::c_uint) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
        pByte_buf = pByte_buf.offset(1);
        buf_len = buf_len.wrapping_sub(1)
    }
    return !mz_crc32_0 as mz_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn miniz_def_alloc_func(mut opaque: *mut libc::c_void,
                                              mut items: size_t,
                                              mut size: size_t)
 -> *mut libc::c_void {
    return malloc(items.wrapping_mul(size));
}
#[no_mangle]
pub unsafe extern "C" fn mz_deflateReset(mut pStream: mz_streamp)
 -> libc::c_int {
    if pStream.is_null() || (*pStream).state.is_null() ||
           (*pStream).zalloc.is_none() || (*pStream).zfree.is_none() {
        return MZ_STREAM_ERROR as libc::c_int
    }
    (*pStream).total_out = 0 as libc::c_int as mz_ulong;
    (*pStream).total_in = (*pStream).total_out;
    tdefl_init((*pStream).state as *mut tdefl_compressor, None,
               0 as *mut libc::c_void,
               (*((*pStream).state as *mut tdefl_compressor)).m_flags as
                   libc::c_int);
    return MZ_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mz_deflate(mut pStream: mz_streamp,
                                    mut flush: libc::c_int) -> libc::c_int {
    let mut in_bytes: size_t = 0;
    let mut out_bytes: size_t = 0;
    let mut orig_total_in: mz_ulong = 0;
    let mut orig_total_out: mz_ulong = 0;
    let mut mz_status: libc::c_int = MZ_OK as libc::c_int;
    if pStream.is_null() || (*pStream).state.is_null() ||
           flush < 0 as libc::c_int || flush > MZ_FINISH as libc::c_int ||
           (*pStream).next_out.is_null() {
        return MZ_STREAM_ERROR as libc::c_int
    }
    if (*pStream).avail_out == 0 { return MZ_BUF_ERROR as libc::c_int }
    if flush == MZ_PARTIAL_FLUSH as libc::c_int {
        flush = MZ_SYNC_FLUSH as libc::c_int
    }
    if (*((*pStream).state as *mut tdefl_compressor)).m_prev_return_status as
           libc::c_int == TDEFL_STATUS_DONE as libc::c_int {
        return if flush == MZ_FINISH as libc::c_int {
                   MZ_STREAM_END as libc::c_int
               } else { MZ_BUF_ERROR as libc::c_int }
    }
    orig_total_in = (*pStream).total_in;
    orig_total_out = (*pStream).total_out;
    loop  {
        let mut defl_status: tdefl_status = TDEFL_STATUS_OKAY;
        in_bytes = (*pStream).avail_in as size_t;
        out_bytes = (*pStream).avail_out as size_t;
        defl_status =
            tdefl_compress((*pStream).state as *mut tdefl_compressor,
                           (*pStream).next_in as *const libc::c_void,
                           &mut in_bytes,
                           (*pStream).next_out as *mut libc::c_void,
                           &mut out_bytes, flush as tdefl_flush);
        (*pStream).next_in =
            (*pStream).next_in.offset(in_bytes as mz_uint as isize);
        (*pStream).avail_in =
            (*pStream).avail_in.wrapping_sub(in_bytes as mz_uint);
        (*pStream).total_in =
            ((*pStream).total_in as
                 libc::c_ulong).wrapping_add(in_bytes as mz_uint as
                                                 libc::c_ulong) as mz_ulong as
                mz_ulong;
        (*pStream).adler =
            tdefl_get_adler32((*pStream).state as *mut tdefl_compressor) as
                mz_ulong;
        (*pStream).next_out =
            (*pStream).next_out.offset(out_bytes as mz_uint as isize);
        (*pStream).avail_out =
            (*pStream).avail_out.wrapping_sub(out_bytes as mz_uint);
        (*pStream).total_out =
            ((*pStream).total_out as
                 libc::c_ulong).wrapping_add(out_bytes as mz_uint as
                                                 libc::c_ulong) as mz_ulong as
                mz_ulong;
        if (defl_status as libc::c_int) < 0 as libc::c_int {
            mz_status = MZ_STREAM_ERROR as libc::c_int;
            break ;
        } else if defl_status as libc::c_int ==
                      TDEFL_STATUS_DONE as libc::c_int {
            mz_status = MZ_STREAM_END as libc::c_int;
            break ;
        } else {
            if (*pStream).avail_out == 0 { break ; }
            if !((*pStream).avail_in == 0 &&
                     flush != MZ_FINISH as libc::c_int) {
                continue ;
            }
            if flush != 0 || (*pStream).total_in != orig_total_in ||
                   (*pStream).total_out != orig_total_out {
                break ;
            }
            return MZ_BUF_ERROR as libc::c_int
        }
    }
    return mz_status;
}
unsafe extern "C" fn tdefl_flush_block(mut d: *mut tdefl_compressor,
                                       mut flush: libc::c_int)
 -> libc::c_int {
    let mut saved_bit_buf: mz_uint = 0;
    let mut saved_bits_in: mz_uint = 0;
    let mut pSaved_output_buf: *mut mz_uint8 = 0 as *mut mz_uint8;
    let mut comp_block_succeeded: mz_bool = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut use_raw_block: libc::c_int =
        ((*d).m_flags &
             TDEFL_FORCE_ALL_RAW_BLOCKS as libc::c_int as libc::c_uint !=
             0 as libc::c_int as libc::c_uint &&
             (*d).m_lookahead_pos.wrapping_sub((*d).m_lz_code_buf_dict_pos) <=
                 (*d).m_dict_size) as libc::c_int;
    let mut pOutput_buf_start: *mut mz_uint8 =
        if (*d).m_pPut_buf_func.is_none() &&
               (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs) >=
                   TDEFL_OUT_BUF_SIZE as libc::c_int as libc::c_ulong {
            ((*d).m_pOut_buf as
                 *mut mz_uint8).offset((*d).m_out_buf_ofs as isize)
        } else { (*d).m_output_buf.as_mut_ptr() };
    (*d).m_pOutput_buf = pOutput_buf_start;
    (*d).m_pOutput_buf_end =
        (*d).m_pOutput_buf.offset(TDEFL_OUT_BUF_SIZE as libc::c_int as
                                      isize).offset(-(16 as libc::c_int as
                                                          isize));
    if (*d).m_output_flush_remaining == 0 {
    } else {
        __assert_fail(b"!d->m_output_flush_remaining\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2147 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
    }
    (*d).m_output_flush_ofs = 0 as libc::c_int as mz_uint;
    (*d).m_output_flush_remaining = 0 as libc::c_int as mz_uint;
    *(*d).m_pLZ_flags =
        (*(*d).m_pLZ_flags as libc::c_int >> (*d).m_num_flags_left) as
            mz_uint8;
    (*d).m_pLZ_code_buf =
        (*d).m_pLZ_code_buf.offset(-(((*d).m_num_flags_left ==
                                          8 as libc::c_int as libc::c_uint) as
                                         libc::c_int as isize));
    if (*d).m_flags & TDEFL_WRITE_ZLIB_HEADER as libc::c_int as libc::c_uint
           != 0 && (*d).m_block_index == 0 {
        let mut bits: mz_uint = 0x78 as libc::c_int as mz_uint;
        let mut len: mz_uint = 8 as libc::c_int as mz_uint;
        if bits <=
               ((1 as libc::c_uint) << len).wrapping_sub(1 as libc::c_uint) {
        } else {
            __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          2156 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
        }
        (*d).m_bit_buffer |= bits << (*d).m_bits_in;
        (*d).m_bits_in =
            ((*d).m_bits_in as libc::c_uint).wrapping_add(len) as mz_uint as
                mz_uint;
        while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh2 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh2 = (*d).m_bit_buffer as mz_uint8
            }
            (*d).m_bit_buffer >>= 8 as libc::c_int;
            (*d).m_bits_in =
                ((*d).m_bits_in as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint
                    as mz_uint
        }
        let mut bits_0: mz_uint = 0x1 as libc::c_int as mz_uint;
        let mut len_0: mz_uint = 8 as libc::c_int as mz_uint;
        if bits_0 <=
               ((1 as libc::c_uint) << len_0).wrapping_sub(1 as libc::c_uint)
           {
        } else {
            __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          2157 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
        }
        (*d).m_bit_buffer |= bits_0 << (*d).m_bits_in;
        (*d).m_bits_in =
            ((*d).m_bits_in as libc::c_uint).wrapping_add(len_0) as mz_uint as
                mz_uint;
        while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh3 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh3 = (*d).m_bit_buffer as mz_uint8
            }
            (*d).m_bit_buffer >>= 8 as libc::c_int;
            (*d).m_bits_in =
                ((*d).m_bits_in as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint
                    as mz_uint
        }
    }
    let mut bits_1: mz_uint =
        (flush == TDEFL_FINISH as libc::c_int) as libc::c_int as mz_uint;
    let mut len_1: mz_uint = 1 as libc::c_int as mz_uint;
    if bits_1 <=
           ((1 as libc::c_uint) << len_1).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2160 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits_1 << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len_1) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh4 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh4 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    }
    pSaved_output_buf = (*d).m_pOutput_buf;
    saved_bit_buf = (*d).m_bit_buffer;
    saved_bits_in = (*d).m_bits_in;
    if use_raw_block == 0 {
        comp_block_succeeded =
            tdefl_compress_block(d,
                                 ((*d).m_flags &
                                      TDEFL_FORCE_ALL_STATIC_BLOCKS as
                                          libc::c_int as libc::c_uint != 0 ||
                                      (*d).m_total_lz_bytes <
                                          48 as libc::c_int as libc::c_uint)
                                     as libc::c_int)
    }
    if (use_raw_block != 0 ||
            (*d).m_total_lz_bytes != 0 &&
                (*d).m_pOutput_buf.wrapping_offset_from(pSaved_output_buf) as
                    libc::c_long + 1 as libc::c_uint as libc::c_long >=
                    (*d).m_total_lz_bytes as libc::c_long) &&
           (*d).m_lookahead_pos.wrapping_sub((*d).m_lz_code_buf_dict_pos) <=
               (*d).m_dict_size {
        let mut i: mz_uint = 0;
        (*d).m_pOutput_buf = pSaved_output_buf;
        (*d).m_bit_buffer = saved_bit_buf;
        (*d).m_bits_in = saved_bits_in;
        let mut bits_2: mz_uint = 0 as libc::c_int as mz_uint;
        let mut len_2: mz_uint = 2 as libc::c_int as mz_uint;
        if bits_2 <=
               ((1 as libc::c_uint) << len_2).wrapping_sub(1 as libc::c_uint)
           {
        } else {
            __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          2176 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
        }
        (*d).m_bit_buffer |= bits_2 << (*d).m_bits_in;
        (*d).m_bits_in =
            ((*d).m_bits_in as libc::c_uint).wrapping_add(len_2) as mz_uint as
                mz_uint;
        while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh5 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh5 = (*d).m_bit_buffer as mz_uint8
            }
            (*d).m_bit_buffer >>= 8 as libc::c_int;
            (*d).m_bits_in =
                ((*d).m_bits_in as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint
                    as mz_uint
        }
        if (*d).m_bits_in != 0 {
            let mut bits_3: mz_uint = 0 as libc::c_int as mz_uint;
            let mut len_3: mz_uint =
                (8 as libc::c_int as
                     libc::c_uint).wrapping_sub((*d).m_bits_in);
            if bits_3 <=
                   ((1 as libc::c_uint) <<
                        len_3).wrapping_sub(1 as libc::c_uint) {
            } else {
                __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2179 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
            }
            (*d).m_bit_buffer |= bits_3 << (*d).m_bits_in;
            (*d).m_bits_in =
                ((*d).m_bits_in as libc::c_uint).wrapping_add(len_3) as
                    mz_uint as mz_uint;
            while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh6 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh6 = (*d).m_bit_buffer as mz_uint8
                }
                (*d).m_bit_buffer >>= 8 as libc::c_int;
                (*d).m_bits_in =
                    ((*d).m_bits_in as
                         libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint as mz_uint
            }
        }
        i = 2 as libc::c_int as mz_uint;
        while i != 0 {
            let mut bits_4: mz_uint =
                (*d).m_total_lz_bytes & 0xffff as libc::c_int as libc::c_uint;
            let mut len_4: mz_uint = 16 as libc::c_int as mz_uint;
            if bits_4 <=
                   ((1 as libc::c_uint) <<
                        len_4).wrapping_sub(1 as libc::c_uint) {
            } else {
                __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2183 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
            }
            (*d).m_bit_buffer |= bits_4 << (*d).m_bits_in;
            (*d).m_bits_in =
                ((*d).m_bits_in as libc::c_uint).wrapping_add(len_4) as
                    mz_uint as mz_uint;
            while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh7 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh7 = (*d).m_bit_buffer as mz_uint8
                }
                (*d).m_bit_buffer >>= 8 as libc::c_int;
                (*d).m_bits_in =
                    ((*d).m_bits_in as
                         libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint as mz_uint
            }
            i = i.wrapping_sub(1);
            (*d).m_total_lz_bytes ^= 0xffff as libc::c_int as libc::c_uint
        }
        i = 0 as libc::c_int as mz_uint;
        while i < (*d).m_total_lz_bytes {
            let mut bits_5: mz_uint =
                (*d).m_dict[((*d).m_lz_code_buf_dict_pos.wrapping_add(i) &
                                 TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                                     libc::c_uint) as usize] as mz_uint;
            let mut len_5: mz_uint = 8 as libc::c_int as mz_uint;
            if bits_5 <=
                   ((1 as libc::c_uint) <<
                        len_5).wrapping_sub(1 as libc::c_uint) {
            } else {
                __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2187 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
            }
            (*d).m_bit_buffer |= bits_5 << (*d).m_bits_in;
            (*d).m_bits_in =
                ((*d).m_bits_in as libc::c_uint).wrapping_add(len_5) as
                    mz_uint as mz_uint;
            while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh8 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh8 = (*d).m_bit_buffer as mz_uint8
                }
                (*d).m_bit_buffer >>= 8 as libc::c_int;
                (*d).m_bits_in =
                    ((*d).m_bits_in as
                         libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint as mz_uint
            }
            i = i.wrapping_add(1)
        }
    } else if comp_block_succeeded == 0 {
        (*d).m_pOutput_buf = pSaved_output_buf;
        (*d).m_bit_buffer = saved_bit_buf;
        (*d).m_bits_in = saved_bits_in;
        tdefl_compress_block(d, 1 as libc::c_int);
    }
    if flush != 0 {
        if flush == TDEFL_FINISH as libc::c_int {
            if (*d).m_bits_in != 0 {
                let mut bits_6: mz_uint = 0 as libc::c_int as mz_uint;
                let mut len_6: mz_uint =
                    (8 as libc::c_int as
                         libc::c_uint).wrapping_sub((*d).m_bits_in);
                if bits_6 <=
                       ((1 as libc::c_uint) <<
                            len_6).wrapping_sub(1 as libc::c_uint) {
                } else {
                    __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"../engine/common/miniz.h\x00" as *const u8
                                      as *const libc::c_char,
                                  2204 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
                }
                (*d).m_bit_buffer |= bits_6 << (*d).m_bits_in;
                (*d).m_bits_in =
                    ((*d).m_bits_in as libc::c_uint).wrapping_add(len_6) as
                        mz_uint as mz_uint;
                while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                        let fresh9 = (*d).m_pOutput_buf;
                        (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                        *fresh9 = (*d).m_bit_buffer as mz_uint8
                    }
                    (*d).m_bit_buffer >>= 8 as libc::c_int;
                    (*d).m_bits_in =
                        ((*d).m_bits_in as
                             libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint as mz_uint
                }
            }
            if (*d).m_flags &
                   TDEFL_WRITE_ZLIB_HEADER as libc::c_int as libc::c_uint != 0
               {
                let mut i_0: mz_uint = 0;
                let mut a: mz_uint = (*d).m_adler32;
                i_0 = 0 as libc::c_int as mz_uint;
                while i_0 < 4 as libc::c_int as libc::c_uint {
                    let mut bits_7: mz_uint =
                        a >> 24 as libc::c_int &
                            0xff as libc::c_int as libc::c_uint;
                    let mut len_7: mz_uint = 8 as libc::c_int as mz_uint;
                    if bits_7 <=
                           ((1 as libc::c_uint) <<
                                len_7).wrapping_sub(1 as libc::c_uint) {
                    } else {
                        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"../engine/common/miniz.h\x00" as
                                          *const u8 as *const libc::c_char,
                                      2211 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 47],
                                                                &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
                    }
                    (*d).m_bit_buffer |= bits_7 << (*d).m_bits_in;
                    (*d).m_bits_in =
                        ((*d).m_bits_in as libc::c_uint).wrapping_add(len_7)
                            as mz_uint as mz_uint;
                    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                            let fresh10 = (*d).m_pOutput_buf;
                            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                            *fresh10 = (*d).m_bit_buffer as mz_uint8
                        }
                        (*d).m_bit_buffer >>= 8 as libc::c_int;
                        (*d).m_bits_in =
                            ((*d).m_bits_in as
                                 libc::c_uint).wrapping_sub(8 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as mz_uint as mz_uint
                    }
                    a <<= 8 as libc::c_int;
                    i_0 = i_0.wrapping_add(1)
                }
            }
        } else {
            let mut i_1: mz_uint = 0;
            let mut z: mz_uint = 0 as libc::c_int as mz_uint;
            let mut bits_8: mz_uint = 0 as libc::c_int as mz_uint;
            let mut len_8: mz_uint = 3 as libc::c_int as mz_uint;
            if bits_8 <=
                   ((1 as libc::c_uint) <<
                        len_8).wrapping_sub(1 as libc::c_uint) {
            } else {
                __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2219 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
            }
            (*d).m_bit_buffer |= bits_8 << (*d).m_bits_in;
            (*d).m_bits_in =
                ((*d).m_bits_in as libc::c_uint).wrapping_add(len_8) as
                    mz_uint as mz_uint;
            while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh11 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh11 = (*d).m_bit_buffer as mz_uint8
                }
                (*d).m_bit_buffer >>= 8 as libc::c_int;
                (*d).m_bits_in =
                    ((*d).m_bits_in as
                         libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint as mz_uint
            }
            if (*d).m_bits_in != 0 {
                let mut bits_9: mz_uint = 0 as libc::c_int as mz_uint;
                let mut len_9: mz_uint =
                    (8 as libc::c_int as
                         libc::c_uint).wrapping_sub((*d).m_bits_in);
                if bits_9 <=
                       ((1 as libc::c_uint) <<
                            len_9).wrapping_sub(1 as libc::c_uint) {
                } else {
                    __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"../engine/common/miniz.h\x00" as *const u8
                                      as *const libc::c_char,
                                  2222 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
                }
                (*d).m_bit_buffer |= bits_9 << (*d).m_bits_in;
                (*d).m_bits_in =
                    ((*d).m_bits_in as libc::c_uint).wrapping_add(len_9) as
                        mz_uint as mz_uint;
                while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                        let fresh12 = (*d).m_pOutput_buf;
                        (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                        *fresh12 = (*d).m_bit_buffer as mz_uint8
                    }
                    (*d).m_bit_buffer >>= 8 as libc::c_int;
                    (*d).m_bits_in =
                        ((*d).m_bits_in as
                             libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint as mz_uint
                }
            }
            i_1 = 2 as libc::c_int as mz_uint;
            while i_1 != 0 {
                let mut bits_10: mz_uint =
                    z & 0xffff as libc::c_int as libc::c_uint;
                let mut len_10: mz_uint = 16 as libc::c_int as mz_uint;
                if bits_10 <=
                       ((1 as libc::c_uint) <<
                            len_10).wrapping_sub(1 as libc::c_uint) {
                } else {
                    __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"../engine/common/miniz.h\x00" as *const u8
                                      as *const libc::c_char,
                                  2226 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
                }
                (*d).m_bit_buffer |= bits_10 << (*d).m_bits_in;
                (*d).m_bits_in =
                    ((*d).m_bits_in as libc::c_uint).wrapping_add(len_10) as
                        mz_uint as mz_uint;
                while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                        let fresh13 = (*d).m_pOutput_buf;
                        (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                        *fresh13 = (*d).m_bit_buffer as mz_uint8
                    }
                    (*d).m_bit_buffer >>= 8 as libc::c_int;
                    (*d).m_bits_in =
                        ((*d).m_bits_in as
                             libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint as mz_uint
                }
                i_1 = i_1.wrapping_sub(1);
                z ^= 0xffff as libc::c_int as libc::c_uint
            }
        }
    }
    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
    } else {
        __assert_fail(b"d->m_pOutput_buf < d->m_pOutput_buf_end\x00" as
                          *const u8 as *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2231 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int tdefl_flush_block(tdefl_compressor *, int)\x00")).as_ptr());
    }
    memset(&mut *(*(*d).m_huff_count.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize)).as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
               as *mut mz_uint16 as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<mz_uint16>() as
                libc::c_ulong).wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_0 as
                                                libc::c_int as
                                                libc::c_ulong));
    memset(&mut *(*(*d).m_huff_count.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
               as *mut mz_uint16 as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<mz_uint16>() as
                libc::c_ulong).wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_1 as
                                                libc::c_int as
                                                libc::c_ulong));
    (*d).m_pLZ_code_buf =
        (*d).m_lz_code_buf.as_mut_ptr().offset(1 as libc::c_int as isize);
    (*d).m_pLZ_flags = (*d).m_lz_code_buf.as_mut_ptr();
    (*d).m_num_flags_left = 8 as libc::c_int as mz_uint;
    (*d).m_lz_code_buf_dict_pos =
        ((*d).m_lz_code_buf_dict_pos as
             libc::c_uint).wrapping_add((*d).m_total_lz_bytes) as mz_uint as
            mz_uint;
    (*d).m_total_lz_bytes = 0 as libc::c_int as mz_uint;
    (*d).m_block_index = (*d).m_block_index.wrapping_add(1);
    n =
        (*d).m_pOutput_buf.wrapping_offset_from(pOutput_buf_start) as
            libc::c_long as libc::c_int;
    if n != 0 as libc::c_int {
        if (*d).m_pPut_buf_func.is_some() {
            *(*d).m_pIn_buf_size =
                (*d).m_pSrc.wrapping_offset_from((*d).m_pIn_buf as
                                                     *const mz_uint8) as
                    libc::c_long as size_t;
            if Some((*d).m_pPut_buf_func.expect("non-null function pointer")).expect("non-null function pointer")((*d).m_output_buf.as_mut_ptr()
                                                                                                                      as
                                                                                                                      *const libc::c_void,
                                                                                                                  n,
                                                                                                                  (*d).m_pPut_buf_user)
                   == 0 {
                (*d).m_prev_return_status = TDEFL_STATUS_PUT_BUF_FAILED;
                return (*d).m_prev_return_status as libc::c_int
            }
        } else if pOutput_buf_start == (*d).m_output_buf.as_mut_ptr() {
            let mut bytes_to_copy: libc::c_int =
                if (n as size_t) <
                       (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
                   {
                    n as size_t
                } else {
                    (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
                } as libc::c_int;
            memcpy(((*d).m_pOut_buf as
                        *mut mz_uint8).offset((*d).m_out_buf_ofs as isize) as
                       *mut libc::c_void,
                   (*d).m_output_buf.as_mut_ptr() as *const libc::c_void,
                   bytes_to_copy as libc::c_ulong);
            (*d).m_out_buf_ofs =
                ((*d).m_out_buf_ofs as
                     libc::c_ulong).wrapping_add(bytes_to_copy as
                                                     libc::c_ulong) as size_t
                    as size_t;
            n -= bytes_to_copy;
            if n != 0 as libc::c_int {
                (*d).m_output_flush_ofs = bytes_to_copy as mz_uint;
                (*d).m_output_flush_remaining = n as mz_uint
            }
        } else {
            (*d).m_out_buf_ofs =
                ((*d).m_out_buf_ofs as
                     libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                    as size_t
        }
    }
    return (*d).m_output_flush_remaining as libc::c_int;
}
unsafe extern "C" fn tdefl_compress_block(mut d: *mut tdefl_compressor,
                                          mut static_block: mz_bool)
 -> mz_bool {
    if static_block != 0 {
        tdefl_start_static_block(d);
    } else { tdefl_start_dynamic_block(d); }
    return tdefl_compress_lz_codes(d);
}
#[no_mangle]
pub unsafe extern "C" fn mz_deflateBound(mut pStream: mz_streamp,
                                         mut source_len: mz_ulong)
 -> mz_ulong {
    return if (128 as libc::c_int as
                   libc::c_ulong).wrapping_add(source_len.wrapping_mul(110 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong).wrapping_div(100
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                  >
                  (128 as libc::c_int as
                       libc::c_ulong).wrapping_add(source_len).wrapping_add(source_len.wrapping_div((31
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         *
                                                                                                         1024
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(1
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_mul(5
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong))
              {
               (128 as libc::c_int as
                    libc::c_ulong).wrapping_add(source_len.wrapping_mul(110 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
           } else {
               (128 as libc::c_int as
                    libc::c_ulong).wrapping_add(source_len).wrapping_add(source_len.wrapping_div((31
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      *
                                                                                                      1024
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_add(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong).wrapping_mul(5
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_ulong))
           };
}
#[no_mangle]
pub unsafe extern "C" fn mz_deflateEnd(mut pStream: mz_streamp)
 -> libc::c_int {
    if pStream.is_null() { return MZ_STREAM_ERROR as libc::c_int }
    if !(*pStream).state.is_null() {
        (*pStream).zfree.expect("non-null function pointer")((*pStream).opaque,
                                                             (*pStream).state
                                                                 as
                                                                 *mut libc::c_void);
        (*pStream).state = 0 as *mut mz_internal_state
    }
    return MZ_OK as libc::c_int;
}
unsafe extern "C" fn TDEFL_READ_UNALIGNED_WORD32(mut p: *const mz_uint8)
 -> mz_uint32 {
    let mut ret: mz_uint32 = 0;
    memcpy(&mut ret as *mut mz_uint32 as *mut libc::c_void,
           p as *const libc::c_void,
           ::std::mem::size_of::<mz_uint32>() as libc::c_ulong);
    return ret;
}
unsafe extern "C" fn tdefl_compress_fast(mut d: *mut tdefl_compressor)
 -> mz_bool {
    let mut lookahead_pos: mz_uint = (*d).m_lookahead_pos;
    let mut lookahead_size: mz_uint = (*d).m_lookahead_size;
    let mut dict_size: mz_uint = (*d).m_dict_size;
    let mut total_lz_bytes: mz_uint = (*d).m_total_lz_bytes;
    let mut num_flags_left: mz_uint = (*d).m_num_flags_left;
    let mut pLZ_code_buf: *mut mz_uint8 = (*d).m_pLZ_code_buf;
    let mut pLZ_flags: *mut mz_uint8 = (*d).m_pLZ_flags;
    let mut cur_pos: mz_uint =
        lookahead_pos &
            TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
    while (*d).m_src_buf_left != 0 ||
              (*d).m_flush as libc::c_uint != 0 && lookahead_size != 0 {
        let TDEFL_COMP_FAST_LOOKAHEAD_SIZE: mz_uint =
            4096 as libc::c_int as mz_uint;
        let mut dst_pos: mz_uint =
            lookahead_pos.wrapping_add(lookahead_size) &
                TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
        let mut num_bytes_to_process: mz_uint =
            if (*d).m_src_buf_left <
                   TDEFL_COMP_FAST_LOOKAHEAD_SIZE.wrapping_sub(lookahead_size)
                       as libc::c_ulong {
                (*d).m_src_buf_left
            } else {
                TDEFL_COMP_FAST_LOOKAHEAD_SIZE.wrapping_sub(lookahead_size) as
                    libc::c_ulong
            } as mz_uint;
        (*d).m_src_buf_left =
            ((*d).m_src_buf_left as
                 libc::c_ulong).wrapping_sub(num_bytes_to_process as
                                                 libc::c_ulong) as size_t as
                size_t;
        lookahead_size =
            (lookahead_size as
                 libc::c_uint).wrapping_add(num_bytes_to_process) as mz_uint
                as mz_uint;
        while num_bytes_to_process != 0 {
            let mut n: mz_uint32 =
                if (TDEFL_LZ_DICT_SIZE as libc::c_int as
                        libc::c_uint).wrapping_sub(dst_pos) <
                       num_bytes_to_process {
                    (TDEFL_LZ_DICT_SIZE as libc::c_int as
                         libc::c_uint).wrapping_sub(dst_pos)
                } else { num_bytes_to_process };
            memcpy((*d).m_dict.as_mut_ptr().offset(dst_pos as isize) as
                       *mut libc::c_void, (*d).m_pSrc as *const libc::c_void,
                   n as libc::c_ulong);
            if dst_pos <
                   (TDEFL_MAX_MATCH_LEN as libc::c_int - 1 as libc::c_int) as
                       libc::c_uint {
                memcpy((*d).m_dict.as_mut_ptr().offset(TDEFL_LZ_DICT_SIZE as
                                                           libc::c_int as
                                                           isize).offset(dst_pos
                                                                             as
                                                                             isize)
                           as *mut libc::c_void,
                       (*d).m_pSrc as *const libc::c_void,
                       if n <
                              ((TDEFL_MAX_MATCH_LEN as libc::c_int -
                                    1 as libc::c_int) as
                                   libc::c_uint).wrapping_sub(dst_pos) {
                           n
                       } else {
                           ((TDEFL_MAX_MATCH_LEN as libc::c_int -
                                 1 as libc::c_int) as
                                libc::c_uint).wrapping_sub(dst_pos)
                       } as libc::c_ulong);
            }
            (*d).m_pSrc = (*d).m_pSrc.offset(n as isize);
            dst_pos =
                dst_pos.wrapping_add(n) &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            num_bytes_to_process =
                (num_bytes_to_process as libc::c_uint).wrapping_sub(n) as
                    mz_uint as mz_uint
        }
        dict_size =
            if (TDEFL_LZ_DICT_SIZE as libc::c_int as
                    libc::c_uint).wrapping_sub(lookahead_size) < dict_size {
                (TDEFL_LZ_DICT_SIZE as libc::c_int as
                     libc::c_uint).wrapping_sub(lookahead_size)
            } else { dict_size };
        if (*d).m_flush as u64 == 0 &&
               lookahead_size < TDEFL_COMP_FAST_LOOKAHEAD_SIZE {
            break ;
        }
        while lookahead_size >= 4 as libc::c_int as libc::c_uint {
            let mut cur_match_dist: mz_uint = 0;
            let mut cur_match_len: mz_uint = 1 as libc::c_int as mz_uint;
            let mut pCur_dict: *mut mz_uint8 =
                (*d).m_dict.as_mut_ptr().offset(cur_pos as isize);
            let mut first_trigram: mz_uint =
                TDEFL_READ_UNALIGNED_WORD32(pCur_dict) &
                    0xffffff as libc::c_int as libc::c_uint;
            let mut hash: mz_uint =
                (first_trigram ^
                     first_trigram >>
                         24 as libc::c_int -
                             (TDEFL_LZ_HASH_BITS as libc::c_int -
                                  8 as libc::c_int)) &
                    TDEFL_LEVEL1_HASH_SIZE_MASK as libc::c_int as
                        libc::c_uint;
            let mut probe_pos: mz_uint =
                (*d).m_hash[hash as usize] as mz_uint;
            (*d).m_hash[hash as usize] = lookahead_pos as mz_uint16;
            cur_match_dist =
                lookahead_pos.wrapping_sub(probe_pos) as mz_uint16 as mz_uint;
            if cur_match_dist <= dict_size &&
                   {
                       probe_pos &=
                           TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                               libc::c_uint;
                       (TDEFL_READ_UNALIGNED_WORD32((*d).m_dict.as_mut_ptr().offset(probe_pos
                                                                                        as
                                                                                        isize))
                            & 0xffffff as libc::c_int as libc::c_uint) ==
                           first_trigram
                   } {
                let mut p: *const mz_uint16 = pCur_dict as *const mz_uint16;
                let mut q: *const mz_uint16 =
                    (*d).m_dict.as_mut_ptr().offset(probe_pos as isize) as
                        *const mz_uint16;
                let mut probe_len: mz_uint32 = 32 as libc::c_int as mz_uint32;
                loop  {
                    p = p.offset(1);
                    q = q.offset(1);
                    if !(TDEFL_READ_UNALIGNED_WORD2(p) as libc::c_int ==
                             TDEFL_READ_UNALIGNED_WORD2(q) as libc::c_int &&
                             {
                                 p = p.offset(1);
                                 q = q.offset(1);
                                 (TDEFL_READ_UNALIGNED_WORD2(p) as
                                      libc::c_int) ==
                                     TDEFL_READ_UNALIGNED_WORD2(q) as
                                         libc::c_int
                             } &&
                             {
                                 p = p.offset(1);
                                 q = q.offset(1);
                                 (TDEFL_READ_UNALIGNED_WORD2(p) as
                                      libc::c_int) ==
                                     TDEFL_READ_UNALIGNED_WORD2(q) as
                                         libc::c_int
                             } &&
                             {
                                 p = p.offset(1);
                                 q = q.offset(1);
                                 (TDEFL_READ_UNALIGNED_WORD2(p) as
                                      libc::c_int) ==
                                     TDEFL_READ_UNALIGNED_WORD2(q) as
                                         libc::c_int
                             } &&
                             {
                                 probe_len = probe_len.wrapping_sub(1);
                                 (probe_len) >
                                     0 as libc::c_int as libc::c_uint
                             }) {
                        break ;
                    }
                }
                cur_match_len =
                    (p.wrapping_offset_from(pCur_dict as *const mz_uint16) as
                         libc::c_long as
                         mz_uint).wrapping_mul(2 as libc::c_int as
                                                   libc::c_uint).wrapping_add((*(p
                                                                                     as
                                                                                     *const mz_uint8)
                                                                                   as
                                                                                   libc::c_int
                                                                                   ==
                                                                                   *(q
                                                                                         as
                                                                                         *const mz_uint8)
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  mz_uint);
                if probe_len == 0 {
                    cur_match_len =
                        if cur_match_dist != 0 {
                            TDEFL_MAX_MATCH_LEN as libc::c_int
                        } else { 0 as libc::c_int } as mz_uint
                }
                if cur_match_len <
                       TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint ||
                       cur_match_len ==
                           TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint
                           &&
                           cur_match_dist >=
                               (8 as
                                    libc::c_uint).wrapping_mul(1024 as
                                                                   libc::c_uint)
                   {
                    cur_match_len = 1 as libc::c_int as mz_uint;
                    let fresh14 = pLZ_code_buf;
                    pLZ_code_buf = pLZ_code_buf.offset(1);
                    *fresh14 = first_trigram as mz_uint8;
                    *pLZ_flags =
                        (*pLZ_flags as libc::c_int >> 1 as libc::c_int) as
                            mz_uint8;
                    (*d).m_huff_count[0 as libc::c_int as
                                          usize][first_trigram as mz_uint8 as
                                                     usize] =
                        (*d).m_huff_count[0 as libc::c_int as
                                              usize][first_trigram as mz_uint8
                                                         as
                                                         usize].wrapping_add(1)
                } else {
                    let mut s0: mz_uint32 = 0;
                    let mut s1: mz_uint32 = 0;
                    cur_match_len =
                        if cur_match_len < lookahead_size {
                            cur_match_len
                        } else { lookahead_size };
                    if cur_match_len >=
                           TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint
                           &&
                           cur_match_dist >= 1 as libc::c_int as libc::c_uint
                           &&
                           cur_match_dist <=
                               TDEFL_LZ_DICT_SIZE as libc::c_int as
                                   libc::c_uint {
                    } else {
                        __assert_fail(b"(cur_match_len >= TDEFL_MIN_MATCH_LEN) && (cur_match_dist >= 1) && (cur_match_dist <= TDEFL_LZ_DICT_SIZE)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"../engine/common/miniz.h\x00" as
                                          *const u8 as *const libc::c_char,
                                      2462 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 48],
                                                                &[libc::c_char; 48]>(b"mz_bool tdefl_compress_fast(tdefl_compressor *)\x00")).as_ptr());
                    }
                    cur_match_dist = cur_match_dist.wrapping_sub(1);
                    *pLZ_code_buf.offset(0 as libc::c_int as isize) =
                        cur_match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN as
                                                       libc::c_int as
                                                       libc::c_uint) as
                            mz_uint8;
                    memcpy(&mut *pLZ_code_buf.offset(1 as libc::c_int as
                                                         isize) as
                               *mut mz_uint8 as *mut libc::c_void,
                           &mut cur_match_dist as *mut mz_uint as
                               *const libc::c_void,
                           ::std::mem::size_of::<mz_uint>() as libc::c_ulong);
                    pLZ_code_buf =
                        pLZ_code_buf.offset(3 as libc::c_int as isize);
                    *pLZ_flags =
                        (*pLZ_flags as libc::c_int >> 1 as libc::c_int |
                             0x80 as libc::c_int) as mz_uint8;
                    s0 =
                        s_tdefl_small_dist_sym[(cur_match_dist &
                                                    511 as libc::c_int as
                                                        libc::c_uint) as
                                                   usize] as mz_uint32;
                    s1 =
                        s_tdefl_large_dist_sym[(cur_match_dist >>
                                                    8 as libc::c_int) as
                                                   usize] as mz_uint32;
                    (*d).m_huff_count[1 as libc::c_int as
                                          usize][if cur_match_dist <
                                                        512 as libc::c_int as
                                                            libc::c_uint {
                                                     s0
                                                 } else { s1 } as usize] =
                        (*d).m_huff_count[1 as libc::c_int as
                                              usize][if cur_match_dist <
                                                            512 as libc::c_int
                                                                as
                                                                libc::c_uint {
                                                         s0
                                                     } else { s1 } as
                                                         usize].wrapping_add(1);
                    (*d).m_huff_count[0 as libc::c_int as
                                          usize][s_tdefl_len_sym[cur_match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                                     as usize]
                                                     as usize] =
                        (*d).m_huff_count[0 as libc::c_int as
                                              usize][s_tdefl_len_sym[cur_match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                         as
                                                                         usize]
                                                         as
                                                         usize].wrapping_add(1)
                }
            } else {
                let fresh15 = pLZ_code_buf;
                pLZ_code_buf = pLZ_code_buf.offset(1);
                *fresh15 = first_trigram as mz_uint8;
                *pLZ_flags =
                    (*pLZ_flags as libc::c_int >> 1 as libc::c_int) as
                        mz_uint8;
                (*d).m_huff_count[0 as libc::c_int as
                                      usize][first_trigram as mz_uint8 as
                                                 usize] =
                    (*d).m_huff_count[0 as libc::c_int as
                                          usize][first_trigram as mz_uint8 as
                                                     usize].wrapping_add(1)
            }
            num_flags_left = num_flags_left.wrapping_sub(1);
            if num_flags_left == 0 as libc::c_int as libc::c_uint {
                num_flags_left = 8 as libc::c_int as mz_uint;
                let fresh16 = pLZ_code_buf;
                pLZ_code_buf = pLZ_code_buf.offset(1);
                pLZ_flags = fresh16
            }
            total_lz_bytes =
                (total_lz_bytes as libc::c_uint).wrapping_add(cur_match_len)
                    as mz_uint as mz_uint;
            lookahead_pos =
                (lookahead_pos as libc::c_uint).wrapping_add(cur_match_len) as
                    mz_uint as mz_uint;
            dict_size =
                if dict_size.wrapping_add(cur_match_len) <
                       TDEFL_LZ_DICT_SIZE as libc::c_int as mz_uint {
                    dict_size.wrapping_add(cur_match_len)
                } else { TDEFL_LZ_DICT_SIZE as libc::c_int as mz_uint };
            cur_pos =
                cur_pos.wrapping_add(cur_match_len) &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            if lookahead_size >= cur_match_len {
            } else {
                __assert_fail(b"lookahead_size >= cur_match_len\x00" as
                                  *const u8 as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2499 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"mz_bool tdefl_compress_fast(tdefl_compressor *)\x00")).as_ptr());
            }
            lookahead_size =
                (lookahead_size as libc::c_uint).wrapping_sub(cur_match_len)
                    as mz_uint as mz_uint;
            if pLZ_code_buf >
                   &mut *(*d).m_lz_code_buf.as_mut_ptr().offset((TDEFL_LZ_CODE_BUF_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     8 as
                                                                         libc::c_int)
                                                                    as isize)
                       as *mut mz_uint8 {
                let mut n_0: libc::c_int = 0;
                (*d).m_lookahead_pos = lookahead_pos;
                (*d).m_lookahead_size = lookahead_size;
                (*d).m_dict_size = dict_size;
                (*d).m_total_lz_bytes = total_lz_bytes;
                (*d).m_pLZ_code_buf = pLZ_code_buf;
                (*d).m_pLZ_flags = pLZ_flags;
                (*d).m_num_flags_left = num_flags_left;
                n_0 = tdefl_flush_block(d, 0 as libc::c_int);
                if n_0 != 0 as libc::c_int {
                    return if n_0 < 0 as libc::c_int {
                               0 as libc::c_int
                           } else { 1 as libc::c_int }
                }
                total_lz_bytes = (*d).m_total_lz_bytes;
                pLZ_code_buf = (*d).m_pLZ_code_buf;
                pLZ_flags = (*d).m_pLZ_flags;
                num_flags_left = (*d).m_num_flags_left
            }
        }
        while lookahead_size != 0 {
            let mut lit: mz_uint8 = (*d).m_dict[cur_pos as usize];
            total_lz_bytes = total_lz_bytes.wrapping_add(1);
            let fresh17 = pLZ_code_buf;
            pLZ_code_buf = pLZ_code_buf.offset(1);
            *fresh17 = lit;
            *pLZ_flags =
                (*pLZ_flags as libc::c_int >> 1 as libc::c_int) as mz_uint8;
            num_flags_left = num_flags_left.wrapping_sub(1);
            if num_flags_left == 0 as libc::c_int as libc::c_uint {
                num_flags_left = 8 as libc::c_int as mz_uint;
                let fresh18 = pLZ_code_buf;
                pLZ_code_buf = pLZ_code_buf.offset(1);
                pLZ_flags = fresh18
            }
            (*d).m_huff_count[0 as libc::c_int as usize][lit as usize] =
                (*d).m_huff_count[0 as libc::c_int as
                                      usize][lit as usize].wrapping_add(1);
            lookahead_pos = lookahead_pos.wrapping_add(1);
            dict_size =
                if dict_size.wrapping_add(1 as libc::c_int as libc::c_uint) <
                       TDEFL_LZ_DICT_SIZE as libc::c_int as mz_uint {
                    dict_size.wrapping_add(1 as libc::c_int as libc::c_uint)
                } else { TDEFL_LZ_DICT_SIZE as libc::c_int as mz_uint };
            cur_pos =
                cur_pos.wrapping_add(1 as libc::c_int as libc::c_uint) &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            lookahead_size = lookahead_size.wrapping_sub(1);
            if pLZ_code_buf >
                   &mut *(*d).m_lz_code_buf.as_mut_ptr().offset((TDEFL_LZ_CODE_BUF_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     8 as
                                                                         libc::c_int)
                                                                    as isize)
                       as *mut mz_uint8 {
                let mut n_1: libc::c_int = 0;
                (*d).m_lookahead_pos = lookahead_pos;
                (*d).m_lookahead_size = lookahead_size;
                (*d).m_dict_size = dict_size;
                (*d).m_total_lz_bytes = total_lz_bytes;
                (*d).m_pLZ_code_buf = pLZ_code_buf;
                (*d).m_pLZ_flags = pLZ_flags;
                (*d).m_num_flags_left = num_flags_left;
                n_1 = tdefl_flush_block(d, 0 as libc::c_int);
                if n_1 != 0 as libc::c_int {
                    return if n_1 < 0 as libc::c_int {
                               0 as libc::c_int
                           } else { 1 as libc::c_int }
                }
                total_lz_bytes = (*d).m_total_lz_bytes;
                pLZ_code_buf = (*d).m_pLZ_code_buf;
                pLZ_flags = (*d).m_pLZ_flags;
                num_flags_left = (*d).m_num_flags_left
            }
        }
    }
    (*d).m_lookahead_pos = lookahead_pos;
    (*d).m_lookahead_size = lookahead_size;
    (*d).m_dict_size = dict_size;
    (*d).m_total_lz_bytes = total_lz_bytes;
    (*d).m_pLZ_code_buf = pLZ_code_buf;
    (*d).m_pLZ_flags = pLZ_flags;
    (*d).m_num_flags_left = num_flags_left;
    return 1 as libc::c_int;
}
unsafe extern "C" fn TDEFL_READ_UNALIGNED_WORD2(mut p: *const mz_uint16)
 -> mz_uint16 {
    let mut ret: mz_uint16 = 0;
    memcpy(&mut ret as *mut mz_uint16 as *mut libc::c_void,
           p as *const libc::c_void,
           ::std::mem::size_of::<mz_uint16>() as libc::c_ulong);
    return ret;
}
unsafe extern "C" fn TDEFL_READ_UNALIGNED_WORD(mut p: *const mz_uint8)
 -> mz_uint16 {
    let mut ret: mz_uint16 = 0;
    memcpy(&mut ret as *mut mz_uint16 as *mut libc::c_void,
           p as *const libc::c_void,
           ::std::mem::size_of::<mz_uint16>() as libc::c_ulong);
    return ret;
}
#[inline(always)]
unsafe extern "C" fn tdefl_find_match(mut d: *mut tdefl_compressor,
                                      mut lookahead_pos: mz_uint,
                                      mut max_dist: mz_uint,
                                      mut max_match_len: mz_uint,
                                      mut pMatch_dist: *mut mz_uint,
                                      mut pMatch_len: *mut mz_uint) {
    let mut dist: mz_uint = 0;
    let mut pos: mz_uint =
        lookahead_pos &
            TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
    let mut match_len: mz_uint = *pMatch_len;
    let mut probe_pos: mz_uint = pos;
    let mut next_probe_pos: mz_uint = 0;
    let mut probe_len: mz_uint = 0;
    let mut num_probes_left: mz_uint =
        (*d).m_max_probes[(match_len >= 32 as libc::c_int as libc::c_uint) as
                              libc::c_int as usize];
    let mut s: *const mz_uint16 =
        (*d).m_dict.as_mut_ptr().offset(pos as isize) as *const mz_uint16;
    let mut p: *const mz_uint16 = 0 as *const mz_uint16;
    let mut q: *const mz_uint16 = 0 as *const mz_uint16;
    let mut c01: mz_uint16 =
        TDEFL_READ_UNALIGNED_WORD(&mut *(*d).m_dict.as_mut_ptr().offset(pos.wrapping_add(match_len).wrapping_sub(1
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint)
                                                                            as
                                                                            isize));
    let mut s01: mz_uint16 = TDEFL_READ_UNALIGNED_WORD2(s);
    if max_match_len <= TDEFL_MAX_MATCH_LEN as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"max_match_len <= TDEFL_MAX_MATCH_LEN\x00" as *const u8
                          as *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2295 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 91],
                                                &[libc::c_char; 91]>(b"void tdefl_find_match(tdefl_compressor *, mz_uint, mz_uint, mz_uint, mz_uint *, mz_uint *)\x00")).as_ptr());
    }
    if max_match_len <= match_len { return }
    loop  {
        loop  {
            num_probes_left = num_probes_left.wrapping_sub(1);
            if num_probes_left == 0 as libc::c_int as libc::c_uint { return }
            next_probe_pos = (*d).m_next[probe_pos as usize] as mz_uint;
            if next_probe_pos == 0 ||
                   {
                       dist =
                           lookahead_pos.wrapping_sub(next_probe_pos) as
                               mz_uint16 as mz_uint;
                       (dist) > max_dist
                   } {
                return
            }
            probe_pos =
                next_probe_pos &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            if TDEFL_READ_UNALIGNED_WORD(&mut *(*d).m_dict.as_mut_ptr().offset(probe_pos.wrapping_add(match_len).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint)
                                                                                   as
                                                                                   isize))
                   as libc::c_int == c01 as libc::c_int {
                break ;
            }
            next_probe_pos = (*d).m_next[probe_pos as usize] as mz_uint;
            if next_probe_pos == 0 ||
                   {
                       dist =
                           lookahead_pos.wrapping_sub(next_probe_pos) as
                               mz_uint16 as mz_uint;
                       (dist) > max_dist
                   } {
                return
            }
            probe_pos =
                next_probe_pos &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            if TDEFL_READ_UNALIGNED_WORD(&mut *(*d).m_dict.as_mut_ptr().offset(probe_pos.wrapping_add(match_len).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint)
                                                                                   as
                                                                                   isize))
                   as libc::c_int == c01 as libc::c_int {
                break ;
            }
            next_probe_pos = (*d).m_next[probe_pos as usize] as mz_uint;
            if next_probe_pos == 0 ||
                   {
                       dist =
                           lookahead_pos.wrapping_sub(next_probe_pos) as
                               mz_uint16 as mz_uint;
                       (dist) > max_dist
                   } {
                return
            }
            probe_pos =
                next_probe_pos &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            if TDEFL_READ_UNALIGNED_WORD(&mut *(*d).m_dict.as_mut_ptr().offset(probe_pos.wrapping_add(match_len).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint)
                                                                                   as
                                                                                   isize))
                   as libc::c_int == c01 as libc::c_int {
                break ;
            }
        }
        if dist == 0 { break ; }
        q =
            (*d).m_dict.as_mut_ptr().offset(probe_pos as isize) as
                *const mz_uint16;
        if TDEFL_READ_UNALIGNED_WORD2(q) as libc::c_int != s01 as libc::c_int
           {
            continue ;
        }
        p = s;
        probe_len = 32 as libc::c_int as mz_uint;
        loop  {
            p = p.offset(1);
            q = q.offset(1);
            if !(TDEFL_READ_UNALIGNED_WORD2(p) as libc::c_int ==
                     TDEFL_READ_UNALIGNED_WORD2(q) as libc::c_int &&
                     {
                         p = p.offset(1);
                         q = q.offset(1);
                         (TDEFL_READ_UNALIGNED_WORD2(p) as libc::c_int) ==
                             TDEFL_READ_UNALIGNED_WORD2(q) as libc::c_int
                     } &&
                     {
                         p = p.offset(1);
                         q = q.offset(1);
                         (TDEFL_READ_UNALIGNED_WORD2(p) as libc::c_int) ==
                             TDEFL_READ_UNALIGNED_WORD2(q) as libc::c_int
                     } &&
                     {
                         p = p.offset(1);
                         q = q.offset(1);
                         (TDEFL_READ_UNALIGNED_WORD2(p) as libc::c_int) ==
                             TDEFL_READ_UNALIGNED_WORD2(q) as libc::c_int
                     } &&
                     {
                         probe_len = probe_len.wrapping_sub(1);
                         (probe_len) > 0 as libc::c_int as libc::c_uint
                     }) {
                break ;
            }
        }
        if probe_len == 0 {
            *pMatch_dist = dist;
            *pMatch_len =
                if max_match_len <
                       TDEFL_MAX_MATCH_LEN as libc::c_int as mz_uint {
                    max_match_len
                } else { TDEFL_MAX_MATCH_LEN as libc::c_int as mz_uint };
            break ;
        } else {
            probe_len =
                (p.wrapping_offset_from(s) as libc::c_long as
                     mz_uint).wrapping_mul(2 as libc::c_int as
                                               libc::c_uint).wrapping_add((*(p
                                                                                 as
                                                                                 *const mz_uint8)
                                                                               as
                                                                               libc::c_int
                                                                               ==
                                                                               *(q
                                                                                     as
                                                                                     *const mz_uint8)
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              mz_uint);
            if !(probe_len > match_len) { continue ; }
            *pMatch_dist = dist;
            match_len =
                (if max_match_len < probe_len {
                     max_match_len
                 } else { probe_len });
            *pMatch_len = match_len;
            if *pMatch_len == max_match_len { break ; }
            c01 =
                TDEFL_READ_UNALIGNED_WORD(&mut *(*d).m_dict.as_mut_ptr().offset(pos.wrapping_add(match_len).wrapping_sub(1
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             libc::c_uint)
                                                                                    as
                                                                                    isize))
        }
    };
}
#[inline(always)]
unsafe extern "C" fn tdefl_record_literal(mut d: *mut tdefl_compressor,
                                          mut lit: mz_uint8) {
    (*d).m_total_lz_bytes = (*d).m_total_lz_bytes.wrapping_add(1);
    let fresh19 = (*d).m_pLZ_code_buf;
    (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(1);
    *fresh19 = lit;
    *(*d).m_pLZ_flags =
        (*(*d).m_pLZ_flags as libc::c_int >> 1 as libc::c_int) as mz_uint8;
    (*d).m_num_flags_left = (*d).m_num_flags_left.wrapping_sub(1);
    if (*d).m_num_flags_left == 0 as libc::c_int as libc::c_uint {
        (*d).m_num_flags_left = 8 as libc::c_int as mz_uint;
        let fresh20 = (*d).m_pLZ_code_buf;
        (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(1);
        (*d).m_pLZ_flags = fresh20
    }
    (*d).m_huff_count[0 as libc::c_int as usize][lit as usize] =
        (*d).m_huff_count[0 as libc::c_int as
                              usize][lit as usize].wrapping_add(1);
}
unsafe extern "C" fn tdefl_compress_lz_codes(mut d: *mut tdefl_compressor)
 -> mz_bool {
    let mut flags: mz_uint = 0;
    let mut pLZ_codes: *mut mz_uint8 = 0 as *mut mz_uint8;
    let mut pOutput_buf: *mut mz_uint8 = (*d).m_pOutput_buf;
    let mut pLZ_code_buf_end: *mut mz_uint8 = (*d).m_pLZ_code_buf;
    let mut bit_buffer: mz_uint64 = (*d).m_bit_buffer as mz_uint64;
    let mut bits_in: mz_uint = (*d).m_bits_in;
    flags = 1 as libc::c_int as mz_uint;
    pLZ_codes = (*d).m_lz_code_buf.as_mut_ptr();
    while pLZ_codes < pLZ_code_buf_end {
        if flags == 1 as libc::c_int as libc::c_uint {
            let fresh21 = pLZ_codes;
            pLZ_codes = pLZ_codes.offset(1);
            flags =
                (*fresh21 as libc::c_int | 0x100 as libc::c_int) as mz_uint
        }
        if flags & 1 as libc::c_int as libc::c_uint != 0 {
            let mut s0: mz_uint = 0;
            let mut s1: mz_uint = 0;
            let mut n0: mz_uint = 0;
            let mut n1: mz_uint = 0;
            let mut sym: mz_uint = 0;
            let mut num_extra_bits: mz_uint = 0;
            let mut match_len: mz_uint =
                *pLZ_codes.offset(0 as libc::c_int as isize) as mz_uint;
            let mut match_dist: mz_uint =
                *(pLZ_codes.offset(1 as libc::c_int as isize) as
                      *const mz_uint16) as mz_uint;
            pLZ_codes = pLZ_codes.offset(3 as libc::c_int as isize);
            if (*d).m_huff_code_sizes[0 as libc::c_int as
                                          usize][s_tdefl_len_sym[match_len as
                                                                     usize] as
                                                     usize] != 0 {
            } else {
                __assert_fail(b"d->m_huff_code_sizes[0][s_tdefl_len_sym[match_len]]\x00"
                                  as *const u8 as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2012 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
            }
            bit_buffer |=
                ((*d).m_huff_codes[0 as libc::c_int as
                                       usize][s_tdefl_len_sym[match_len as
                                                                  usize] as
                                                  usize] as mz_uint64) <<
                    bits_in;
            bits_in =
                (bits_in as
                     libc::c_uint).wrapping_add((*d).m_huff_code_sizes[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize][s_tdefl_len_sym[match_len
                                                                                                      as
                                                                                                      usize]
                                                                                      as
                                                                                      usize]
                                                    as libc::c_uint) as
                    mz_uint as mz_uint;
            bit_buffer |=
                ((match_len &
                      mz_bitmasks[s_tdefl_len_extra[match_len as usize] as
                                      usize]) as mz_uint64) << bits_in;
            bits_in =
                (bits_in as
                     libc::c_uint).wrapping_add(s_tdefl_len_extra[match_len as
                                                                      usize]
                                                    as libc::c_uint) as
                    mz_uint as mz_uint;
            s0 =
                s_tdefl_small_dist_sym[(match_dist &
                                            511 as libc::c_int as
                                                libc::c_uint) as usize] as
                    mz_uint;
            n0 =
                s_tdefl_small_dist_extra[(match_dist &
                                              511 as libc::c_int as
                                                  libc::c_uint) as usize] as
                    mz_uint;
            s1 =
                s_tdefl_large_dist_sym[(match_dist >> 8 as libc::c_int) as
                                           usize] as mz_uint;
            n1 =
                s_tdefl_large_dist_extra[(match_dist >> 8 as libc::c_int) as
                                             usize] as mz_uint;
            sym =
                if match_dist < 512 as libc::c_int as libc::c_uint {
                    s0
                } else { s1 };
            num_extra_bits =
                if match_dist < 512 as libc::c_int as libc::c_uint {
                    n0
                } else { n1 };
            if (*d).m_huff_code_sizes[1 as libc::c_int as usize][sym as usize]
                   != 0 {
            } else {
                __assert_fail(b"d->m_huff_code_sizes[1][sym]\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2024 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
            }
            bit_buffer |=
                ((*d).m_huff_codes[1 as libc::c_int as usize][sym as usize] as
                     mz_uint64) << bits_in;
            bits_in =
                (bits_in as
                     libc::c_uint).wrapping_add((*d).m_huff_code_sizes[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize][sym
                                                                                      as
                                                                                      usize]
                                                    as libc::c_uint) as
                    mz_uint as mz_uint;
            bit_buffer |=
                ((match_dist & mz_bitmasks[num_extra_bits as usize]) as
                     mz_uint64) << bits_in;
            bits_in =
                (bits_in as libc::c_uint).wrapping_add(num_extra_bits) as
                    mz_uint as mz_uint
        } else {
            let fresh22 = pLZ_codes;
            pLZ_codes = pLZ_codes.offset(1);
            let mut lit: mz_uint = *fresh22 as mz_uint;
            if (*d).m_huff_code_sizes[0 as libc::c_int as usize][lit as usize]
                   != 0 {
            } else {
                __assert_fail(b"d->m_huff_code_sizes[0][lit]\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              2031 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
            }
            bit_buffer |=
                ((*d).m_huff_codes[0 as libc::c_int as usize][lit as usize] as
                     mz_uint64) << bits_in;
            bits_in =
                (bits_in as
                     libc::c_uint).wrapping_add((*d).m_huff_code_sizes[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize][lit
                                                                                      as
                                                                                      usize]
                                                    as libc::c_uint) as
                    mz_uint as mz_uint;
            if flags & 2 as libc::c_int as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint &&
                   pLZ_codes < pLZ_code_buf_end {
                flags >>= 1 as libc::c_int;
                let fresh23 = pLZ_codes;
                pLZ_codes = pLZ_codes.offset(1);
                lit = *fresh23 as mz_uint;
                if (*d).m_huff_code_sizes[0 as libc::c_int as
                                              usize][lit as usize] != 0 {
                } else {
                    __assert_fail(b"d->m_huff_code_sizes[0][lit]\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"../engine/common/miniz.h\x00" as *const u8
                                      as *const libc::c_char,
                                  2038 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 52],
                                                            &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
                }
                bit_buffer |=
                    ((*d).m_huff_codes[0 as libc::c_int as
                                           usize][lit as usize] as mz_uint64)
                        << bits_in;
                bits_in =
                    (bits_in as
                         libc::c_uint).wrapping_add((*d).m_huff_code_sizes[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize][lit
                                                                                          as
                                                                                          usize]
                                                        as libc::c_uint) as
                        mz_uint as mz_uint;
                if flags & 2 as libc::c_int as libc::c_uint ==
                       0 as libc::c_int as libc::c_uint &&
                       pLZ_codes < pLZ_code_buf_end {
                    flags >>= 1 as libc::c_int;
                    let fresh24 = pLZ_codes;
                    pLZ_codes = pLZ_codes.offset(1);
                    lit = *fresh24 as mz_uint;
                    if (*d).m_huff_code_sizes[0 as libc::c_int as
                                                  usize][lit as usize] != 0 {
                    } else {
                        __assert_fail(b"d->m_huff_code_sizes[0][lit]\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"../engine/common/miniz.h\x00" as
                                          *const u8 as *const libc::c_char,
                                      2045 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 52],
                                                                &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
                    }
                    bit_buffer |=
                        ((*d).m_huff_codes[0 as libc::c_int as
                                               usize][lit as usize] as
                             mz_uint64) << bits_in;
                    bits_in =
                        (bits_in as
                             libc::c_uint).wrapping_add((*d).m_huff_code_sizes[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize][lit
                                                                                              as
                                                                                              usize]
                                                            as libc::c_uint)
                            as mz_uint as mz_uint
                }
            }
        }
        if pOutput_buf >= (*d).m_pOutput_buf_end { return 0 as libc::c_int }
        *(pOutput_buf as *mut mz_uint64) = bit_buffer;
        pOutput_buf =
            pOutput_buf.offset((bits_in >> 3 as libc::c_int) as isize);
        bit_buffer >>= bits_in & !(7 as libc::c_int) as libc::c_uint;
        bits_in &= 7 as libc::c_int as libc::c_uint;
        flags >>= 1 as libc::c_int
    }
    (*d).m_pOutput_buf = pOutput_buf;
    (*d).m_bits_in = 0 as libc::c_int as mz_uint;
    (*d).m_bit_buffer = 0 as libc::c_int as mz_uint;
    while bits_in != 0 {
        let mut n: mz_uint32 =
            if bits_in < 16 as libc::c_int as libc::c_uint {
                bits_in
            } else { 16 as libc::c_int as libc::c_uint };
        let mut bits: mz_uint =
            bit_buffer as mz_uint & mz_bitmasks[n as usize];
        let mut len: mz_uint = n;
        if bits <=
               ((1 as libc::c_uint) << len).wrapping_sub(1 as libc::c_uint) {
        } else {
            __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          2069 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 52],
                                                    &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
        }
        (*d).m_bit_buffer |= bits << (*d).m_bits_in;
        (*d).m_bits_in =
            ((*d).m_bits_in as libc::c_uint).wrapping_add(len) as mz_uint as
                mz_uint;
        while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh25 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh25 = (*d).m_bit_buffer as mz_uint8
            }
            (*d).m_bit_buffer >>= 8 as libc::c_int;
            (*d).m_bits_in =
                ((*d).m_bits_in as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint
                    as mz_uint
        }
        bit_buffer >>= n;
        bits_in =
            (bits_in as libc::c_uint).wrapping_sub(n) as mz_uint as mz_uint
    }
    let mut bits_0: mz_uint =
        (*d).m_huff_codes[0 as libc::c_int as
                              usize][256 as libc::c_int as usize] as mz_uint;
    let mut len_0: mz_uint =
        (*d).m_huff_code_sizes[0 as libc::c_int as
                                   usize][256 as libc::c_int as usize] as
            mz_uint;
    if bits_0 <=
           ((1 as libc::c_uint) << len_0).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2074 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 52],
                                                &[libc::c_char; 52]>(b"mz_bool tdefl_compress_lz_codes(tdefl_compressor *)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits_0 << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len_0) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh26 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh26 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    }
    return ((*d).m_pOutput_buf < (*d).m_pOutput_buf_end) as libc::c_int;
}
unsafe extern "C" fn tdefl_start_static_block(mut d: *mut tdefl_compressor) {
    let mut i: mz_uint = 0;
    let mut p: *mut mz_uint8 =
        &mut *(*(*d).m_huff_code_sizes.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize)).as_mut_ptr().offset(0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)
            as *mut mz_uint8;
    i = 0 as libc::c_int as mz_uint;
    while i <= 143 as libc::c_int as libc::c_uint {
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = 8 as libc::c_int as mz_uint8;
        i = i.wrapping_add(1)
    }
    while i <= 255 as libc::c_int as libc::c_uint {
        let fresh28 = p;
        p = p.offset(1);
        *fresh28 = 9 as libc::c_int as mz_uint8;
        i = i.wrapping_add(1)
    }
    while i <= 279 as libc::c_int as libc::c_uint {
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = 7 as libc::c_int as mz_uint8;
        i = i.wrapping_add(1)
    }
    while i <= 287 as libc::c_int as libc::c_uint {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = 8 as libc::c_int as mz_uint8;
        i = i.wrapping_add(1)
    }
    memset((*d).m_huff_code_sizes[1 as libc::c_int as usize].as_mut_ptr() as
               *mut libc::c_void, 5 as libc::c_int,
           32 as libc::c_int as libc::c_ulong);
    tdefl_optimize_huffman_table(d, 0 as libc::c_int, 288 as libc::c_int,
                                 15 as libc::c_int, 1 as libc::c_int);
    tdefl_optimize_huffman_table(d, 1 as libc::c_int, 32 as libc::c_int,
                                 15 as libc::c_int, 1 as libc::c_int);
    let mut bits: mz_uint = 1 as libc::c_int as mz_uint;
    let mut len: mz_uint = 2 as libc::c_int as mz_uint;
    if bits <= ((1 as libc::c_uint) << len).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      1979 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"void tdefl_start_static_block(tdefl_compressor *)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh31 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh31 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    };
}
unsafe extern "C" fn tdefl_radix_sort_syms(mut num_syms: mz_uint,
                                           mut pSyms0: *mut tdefl_sym_freq,
                                           mut pSyms1: *mut tdefl_sym_freq)
 -> *mut tdefl_sym_freq {
    let mut total_passes: mz_uint32 = 2 as libc::c_int as mz_uint32;
    let mut pass_shift: mz_uint32 = 0;
    let mut pass: mz_uint32 = 0;
    let mut i: mz_uint32 = 0;
    let mut hist: [mz_uint32; 512] = [0; 512];
    let mut pCur_syms: *mut tdefl_sym_freq = pSyms0;
    let mut pNew_syms: *mut tdefl_sym_freq = pSyms1;
    memset(&mut hist as *mut [mz_uint32; 512] as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[mz_uint32; 512]>() as libc::c_ulong);
    i = 0 as libc::c_int as mz_uint32;
    while i < num_syms {
        let mut freq: mz_uint = (*pSyms0.offset(i as isize)).m_key as mz_uint;
        hist[(freq & 0xff as libc::c_int as libc::c_uint) as usize] =
            hist[(freq & 0xff as libc::c_int as libc::c_uint) as
                     usize].wrapping_add(1);
        hist[(256 as libc::c_int as
                  libc::c_uint).wrapping_add(freq >> 8 as libc::c_int &
                                                 0xff as libc::c_int as
                                                     libc::c_uint) as usize] =
            hist[(256 as libc::c_int as
                      libc::c_uint).wrapping_add(freq >> 8 as libc::c_int &
                                                     0xff as libc::c_int as
                                                         libc::c_uint) as
                     usize].wrapping_add(1);
        i = i.wrapping_add(1)
    }
    while total_passes > 1 as libc::c_int as libc::c_uint &&
              num_syms ==
                  hist[total_passes.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(256
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                           as usize] {
        total_passes = total_passes.wrapping_sub(1)
    }
    pass_shift = 0 as libc::c_int as mz_uint32;
    pass = 0 as libc::c_int as mz_uint32;
    while pass < total_passes {
        let mut pHist: *const mz_uint32 =
            &mut *hist.as_mut_ptr().offset((pass << 8 as libc::c_int) as
                                               isize) as *mut mz_uint32;
        let mut offsets: [mz_uint; 256] = [0; 256];
        let mut cur_ofs: mz_uint = 0 as libc::c_int as mz_uint;
        i = 0 as libc::c_int as mz_uint32;
        while i < 256 as libc::c_int as libc::c_uint {
            offsets[i as usize] = cur_ofs;
            cur_ofs =
                (cur_ofs as
                     libc::c_uint).wrapping_add(*pHist.offset(i as isize)) as
                    mz_uint as mz_uint;
            i = i.wrapping_add(1)
        }
        i = 0 as libc::c_int as mz_uint32;
        while i < num_syms {
            let fresh32 =
                offsets[((*pCur_syms.offset(i as isize)).m_key as libc::c_int
                             >> pass_shift & 0xff as libc::c_int) as usize];
            offsets[((*pCur_syms.offset(i as isize)).m_key as libc::c_int >>
                         pass_shift & 0xff as libc::c_int) as usize] =
                offsets[((*pCur_syms.offset(i as isize)).m_key as libc::c_int
                             >> pass_shift & 0xff as libc::c_int) as
                            usize].wrapping_add(1);
            *pNew_syms.offset(fresh32 as isize) =
                *pCur_syms.offset(i as isize);
            i = i.wrapping_add(1)
        }
        let mut t: *mut tdefl_sym_freq = pCur_syms;
        pCur_syms = pNew_syms;
        pNew_syms = t;
        pass = pass.wrapping_add(1);
        pass_shift =
            (pass_shift as
                 libc::c_uint).wrapping_add(8 as libc::c_int as libc::c_uint)
                as mz_uint32 as mz_uint32
    }
    return pCur_syms;
}
unsafe extern "C" fn tdefl_calculate_minimum_redundancy(mut A:
                                                            *mut tdefl_sym_freq,
                                                        mut n: libc::c_int) {
    let mut root: libc::c_int = 0;
    let mut leaf: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut avbl: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut dpth: libc::c_int = 0;
    if n == 0 as libc::c_int {
        return
    } else {
        if n == 1 as libc::c_int {
            (*A.offset(0 as libc::c_int as isize)).m_key =
                1 as libc::c_int as mz_uint16;
            return
        }
    }
    let ref mut fresh33 = (*A.offset(0 as libc::c_int as isize)).m_key;
    *fresh33 =
        (*fresh33 as libc::c_int +
             (*A.offset(1 as libc::c_int as isize)).m_key as libc::c_int) as
            mz_uint16;
    root = 0 as libc::c_int;
    leaf = 2 as libc::c_int;
    next = 1 as libc::c_int;
    while next < n - 1 as libc::c_int {
        if leaf >= n ||
               ((*A.offset(root as isize)).m_key as libc::c_int) <
                   (*A.offset(leaf as isize)).m_key as libc::c_int {
            (*A.offset(next as isize)).m_key =
                (*A.offset(root as isize)).m_key;
            let fresh34 = root;
            root = root + 1;
            (*A.offset(fresh34 as isize)).m_key = next as mz_uint16
        } else {
            let fresh35 = leaf;
            leaf = leaf + 1;
            (*A.offset(next as isize)).m_key =
                (*A.offset(fresh35 as isize)).m_key
        }
        if leaf >= n ||
               root < next &&
                   ((*A.offset(root as isize)).m_key as libc::c_int) <
                       (*A.offset(leaf as isize)).m_key as libc::c_int {
            (*A.offset(next as isize)).m_key =
                ((*A.offset(next as isize)).m_key as libc::c_int +
                     (*A.offset(root as isize)).m_key as libc::c_int) as
                    mz_uint16;
            let fresh36 = root;
            root = root + 1;
            (*A.offset(fresh36 as isize)).m_key = next as mz_uint16
        } else {
            let fresh37 = leaf;
            leaf = leaf + 1;
            (*A.offset(next as isize)).m_key =
                ((*A.offset(next as isize)).m_key as libc::c_int +
                     (*A.offset(fresh37 as isize)).m_key as libc::c_int) as
                    mz_uint16
        }
        next += 1
    }
    (*A.offset((n - 2 as libc::c_int) as isize)).m_key =
        0 as libc::c_int as mz_uint16;
    next = n - 3 as libc::c_int;
    while next >= 0 as libc::c_int {
        (*A.offset(next as isize)).m_key =
            ((*A.offset((*A.offset(next as isize)).m_key as isize)).m_key as
                 libc::c_int + 1 as libc::c_int) as mz_uint16;
        next -= 1
    }
    avbl = 1 as libc::c_int;
    dpth = 0 as libc::c_int;
    used = dpth;
    root = n - 2 as libc::c_int;
    next = n - 1 as libc::c_int;
    while avbl > 0 as libc::c_int {
        while root >= 0 as libc::c_int &&
                  (*A.offset(root as isize)).m_key as libc::c_int == dpth {
            used += 1;
            root -= 1
        }
        while avbl > used {
            let fresh38 = next;
            next = next - 1;
            (*A.offset(fresh38 as isize)).m_key = dpth as mz_uint16;
            avbl -= 1
        }
        avbl = 2 as libc::c_int * used;
        dpth += 1;
        used = 0 as libc::c_int
    };
}
unsafe extern "C" fn tdefl_huffman_enforce_max_code_size(mut pNum_codes:
                                                             *mut libc::c_int,
                                                         mut code_list_len:
                                                             libc::c_int,
                                                         mut max_code_size:
                                                             libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut total: mz_uint32 = 0 as libc::c_int as mz_uint32;
    if code_list_len <= 1 as libc::c_int { return }
    i = max_code_size + 1 as libc::c_int;
    while i <= TDEFL_MAX_SUPPORTED_HUFF_CODESIZE as libc::c_int {
        *pNum_codes.offset(max_code_size as isize) +=
            *pNum_codes.offset(i as isize);
        i += 1
    }
    i = max_code_size;
    while i > 0 as libc::c_int {
        total =
            (total as
                 libc::c_uint).wrapping_add((*pNum_codes.offset(i as isize) as
                                                 mz_uint32) <<
                                                max_code_size - i) as
                mz_uint32 as mz_uint32;
        i -= 1
    }
    while total as libc::c_ulong != (1 as libc::c_ulong) << max_code_size {
        let ref mut fresh39 = *pNum_codes.offset(max_code_size as isize);
        *fresh39 -= 1;
        i = max_code_size - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            if *pNum_codes.offset(i as isize) != 0 {
                let ref mut fresh40 = *pNum_codes.offset(i as isize);
                *fresh40 -= 1;
                *pNum_codes.offset((i + 1 as libc::c_int) as isize) +=
                    2 as libc::c_int;
                break ;
            } else { i -= 1 }
        }
        total = total.wrapping_sub(1)
    };
}
unsafe extern "C" fn tdefl_optimize_huffman_table(mut d:
                                                      *mut tdefl_compressor,
                                                  mut table_num: libc::c_int,
                                                  mut table_len: libc::c_int,
                                                  mut code_size_limit:
                                                      libc::c_int,
                                                  mut static_table:
                                                      libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut num_codes: [libc::c_int; 33] = [0; 33];
    let mut next_code: [mz_uint; 33] = [0; 33];
    memset(&mut num_codes as *mut [libc::c_int; 33] as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 33]>() as libc::c_ulong);
    if static_table != 0 {
        i = 0 as libc::c_int;
        while i < table_len {
            num_codes[(*d).m_huff_code_sizes[table_num as usize][i as usize]
                          as usize] += 1;
            i += 1
        }
    } else {
        let mut syms0: [tdefl_sym_freq; 288] =
            [tdefl_sym_freq{m_key: 0, m_sym_index: 0,}; 288];
        let mut syms1: [tdefl_sym_freq; 288] =
            [tdefl_sym_freq{m_key: 0, m_sym_index: 0,}; 288];
        let mut pSyms: *mut tdefl_sym_freq = 0 as *mut tdefl_sym_freq;
        let mut num_used_syms: libc::c_int = 0 as libc::c_int;
        let mut pSym_count: *const mz_uint16 =
            &mut *(*(*d).m_huff_count.as_mut_ptr().offset(table_num as
                                                              isize)).as_mut_ptr().offset(0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)
                as *mut mz_uint16;
        i = 0 as libc::c_int;
        while i < table_len {
            if *pSym_count.offset(i as isize) != 0 {
                syms0[num_used_syms as usize].m_key =
                    *pSym_count.offset(i as isize);
                let fresh41 = num_used_syms;
                num_used_syms = num_used_syms + 1;
                syms0[fresh41 as usize].m_sym_index = i as mz_uint16
            }
            i += 1
        }
        pSyms =
            tdefl_radix_sort_syms(num_used_syms as mz_uint,
                                  syms0.as_mut_ptr(), syms1.as_mut_ptr());
        tdefl_calculate_minimum_redundancy(pSyms, num_used_syms);
        i = 0 as libc::c_int;
        while i < num_used_syms {
            num_codes[(*pSyms.offset(i as isize)).m_key as usize] += 1;
            i += 1
        }
        tdefl_huffman_enforce_max_code_size(num_codes.as_mut_ptr(),
                                            num_used_syms, code_size_limit);
        memset(&mut *(*d).m_huff_code_sizes.as_mut_ptr().offset(table_num as
                                                                    isize) as
                   *mut [mz_uint8; 288] as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<[mz_uint8; 288]>() as libc::c_ulong);
        memset(&mut *(*d).m_huff_codes.as_mut_ptr().offset(table_num as isize)
                   as *mut [mz_uint16; 288] as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<[mz_uint16; 288]>() as libc::c_ulong);
        i = 1 as libc::c_int;
        j = num_used_syms;
        while i <= code_size_limit {
            l = num_codes[i as usize];
            while l > 0 as libc::c_int {
                j -= 1;
                (*d).m_huff_code_sizes[table_num as
                                           usize][(*pSyms.offset(j as
                                                                     isize)).m_sym_index
                                                      as usize] =
                    i as mz_uint8;
                l -= 1
            }
            i += 1
        }
    }
    next_code[1 as libc::c_int as usize] = 0 as libc::c_int as mz_uint;
    j = 0 as libc::c_int;
    i = 2 as libc::c_int;
    while i <= code_size_limit {
        j =
            j + num_codes[(i - 1 as libc::c_int) as usize] <<
                1 as libc::c_int;
        next_code[i as usize] = j as mz_uint;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < table_len {
        let mut rev_code: mz_uint = 0 as libc::c_int as mz_uint;
        let mut code: mz_uint = 0;
        let mut code_size: mz_uint = 0;
        code_size =
            (*d).m_huff_code_sizes[table_num as usize][i as usize] as mz_uint;
        if !(code_size == 0 as libc::c_int as libc::c_uint) {
            let fresh42 = next_code[code_size as usize];
            next_code[code_size as usize] =
                next_code[code_size as usize].wrapping_add(1);
            code = fresh42;
            l = code_size as libc::c_int;
            while l > 0 as libc::c_int {
                rev_code =
                    rev_code << 1 as libc::c_int |
                        code & 1 as libc::c_int as libc::c_uint;
                l -= 1;
                code >>= 1 as libc::c_int
            }
            (*d).m_huff_codes[table_num as usize][i as usize] =
                rev_code as mz_uint16
        }
        i += 1
    };
}
static mut s_tdefl_packed_code_size_syms_swizzle: [mz_uint8; 19] =
    [16 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     18 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     8 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8];
unsafe extern "C" fn tdefl_start_dynamic_block(mut d: *mut tdefl_compressor) {
    let mut num_lit_codes: libc::c_int = 0;
    let mut num_dist_codes: libc::c_int = 0;
    let mut num_bit_lengths: libc::c_int = 0;
    let mut i: mz_uint = 0;
    let mut total_code_sizes_to_pack: mz_uint = 0;
    let mut num_packed_code_sizes: mz_uint = 0;
    let mut rle_z_count: mz_uint = 0;
    let mut rle_repeat_count: mz_uint = 0;
    let mut packed_code_sizes_index: mz_uint = 0;
    let mut code_sizes_to_pack: [mz_uint8; 320] = [0; 320];
    let mut packed_code_sizes: [mz_uint8; 320] = [0; 320];
    let mut prev_code_size: mz_uint8 = 0xff as libc::c_int as mz_uint8;
    (*d).m_huff_count[0 as libc::c_int as usize][256 as libc::c_int as usize]
        = 1 as libc::c_int as mz_uint16;
    tdefl_optimize_huffman_table(d, 0 as libc::c_int,
                                 TDEFL_MAX_HUFF_SYMBOLS_0 as libc::c_int,
                                 15 as libc::c_int, 0 as libc::c_int);
    tdefl_optimize_huffman_table(d, 1 as libc::c_int,
                                 TDEFL_MAX_HUFF_SYMBOLS_1 as libc::c_int,
                                 15 as libc::c_int, 0 as libc::c_int);
    num_lit_codes = 286 as libc::c_int;
    while num_lit_codes > 257 as libc::c_int {
        if (*d).m_huff_code_sizes[0 as libc::c_int as
                                      usize][(num_lit_codes -
                                                  1 as libc::c_int) as usize]
               != 0 {
            break ;
        }
        num_lit_codes -= 1
    }
    num_dist_codes = 30 as libc::c_int;
    while num_dist_codes > 1 as libc::c_int {
        if (*d).m_huff_code_sizes[1 as libc::c_int as
                                      usize][(num_dist_codes -
                                                  1 as libc::c_int) as usize]
               != 0 {
            break ;
        }
        num_dist_codes -= 1
    }
    memcpy(code_sizes_to_pack.as_mut_ptr() as *mut libc::c_void,
           &mut *(*(*d).m_huff_code_sizes.as_mut_ptr().offset(0 as libc::c_int
                                                                  as
                                                                  isize)).as_mut_ptr().offset(0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize)
               as *mut mz_uint8 as *const libc::c_void,
           num_lit_codes as libc::c_ulong);
    memcpy(code_sizes_to_pack.as_mut_ptr().offset(num_lit_codes as isize) as
               *mut libc::c_void,
           &mut *(*(*d).m_huff_code_sizes.as_mut_ptr().offset(1 as libc::c_int
                                                                  as
                                                                  isize)).as_mut_ptr().offset(0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize)
               as *mut mz_uint8 as *const libc::c_void,
           num_dist_codes as libc::c_ulong);
    total_code_sizes_to_pack = (num_lit_codes + num_dist_codes) as mz_uint;
    num_packed_code_sizes = 0 as libc::c_int as mz_uint;
    rle_z_count = 0 as libc::c_int as mz_uint;
    rle_repeat_count = 0 as libc::c_int as mz_uint;
    memset(&mut *(*(*d).m_huff_count.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize)).as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
               as *mut mz_uint16 as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<mz_uint16>() as
                libc::c_ulong).wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_2 as
                                                libc::c_int as
                                                libc::c_ulong));
    i = 0 as libc::c_int as mz_uint;
    while i < total_code_sizes_to_pack {
        let mut code_size: mz_uint8 = code_sizes_to_pack[i as usize];
        if code_size == 0 {
            if rle_repeat_count != 0 {
                if rle_repeat_count < 3 as libc::c_int as libc::c_uint {
                    (*d).m_huff_count[2 as libc::c_int as
                                          usize][prev_code_size as usize] =
                        ((*d).m_huff_count[2 as libc::c_int as
                                               usize][prev_code_size as usize]
                             as libc::c_uint).wrapping_add(rle_repeat_count)
                            as mz_uint16;
                    loop  {
                        let fresh43 = rle_repeat_count;
                        rle_repeat_count = rle_repeat_count.wrapping_sub(1);
                        if !(fresh43 != 0) { break ; }
                        let fresh44 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh44 as usize] = prev_code_size
                    }
                } else {
                    (*d).m_huff_count[2 as libc::c_int as
                                          usize][16 as libc::c_int as usize] =
                        ((*d).m_huff_count[2 as libc::c_int as
                                               usize][16 as libc::c_int as
                                                          usize] as
                             libc::c_int + 1 as libc::c_int) as mz_uint16;
                    let fresh45 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh45 as usize] =
                        16 as libc::c_int as mz_uint8;
                    let fresh46 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh46 as usize] =
                        rle_repeat_count.wrapping_sub(3 as libc::c_int as
                                                          libc::c_uint) as
                            mz_uint8
                }
                rle_repeat_count = 0 as libc::c_int as mz_uint
            }
            rle_z_count = rle_z_count.wrapping_add(1);
            if rle_z_count == 138 as libc::c_int as libc::c_uint {
                if rle_z_count != 0 {
                    if rle_z_count < 3 as libc::c_int as libc::c_uint {
                        (*d).m_huff_count[2 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] =
                            ((*d).m_huff_count[2 as libc::c_int as
                                                   usize][0 as libc::c_int as
                                                              usize] as
                                 libc::c_uint).wrapping_add(rle_z_count) as
                                mz_uint16;
                        loop  {
                            let fresh47 = rle_z_count;
                            rle_z_count = rle_z_count.wrapping_sub(1);
                            if !(fresh47 != 0) { break ; }
                            let fresh48 = num_packed_code_sizes;
                            num_packed_code_sizes =
                                num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh48 as usize] =
                                0 as libc::c_int as mz_uint8
                        }
                    } else if rle_z_count <= 10 as libc::c_int as libc::c_uint
                     {
                        (*d).m_huff_count[2 as libc::c_int as
                                              usize][17 as libc::c_int as
                                                         usize] =
                            ((*d).m_huff_count[2 as libc::c_int as
                                                   usize][17 as libc::c_int as
                                                              usize] as
                                 libc::c_int + 1 as libc::c_int) as mz_uint16;
                        let fresh49 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh49 as usize] =
                            17 as libc::c_int as mz_uint8;
                        let fresh50 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh50 as usize] =
                            rle_z_count.wrapping_sub(3 as libc::c_int as
                                                         libc::c_uint) as
                                mz_uint8
                    } else {
                        (*d).m_huff_count[2 as libc::c_int as
                                              usize][18 as libc::c_int as
                                                         usize] =
                            ((*d).m_huff_count[2 as libc::c_int as
                                                   usize][18 as libc::c_int as
                                                              usize] as
                                 libc::c_int + 1 as libc::c_int) as mz_uint16;
                        let fresh51 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh51 as usize] =
                            18 as libc::c_int as mz_uint8;
                        let fresh52 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh52 as usize] =
                            rle_z_count.wrapping_sub(11 as libc::c_int as
                                                         libc::c_uint) as
                                mz_uint8
                    }
                    rle_z_count = 0 as libc::c_int as mz_uint
                }
            }
        } else {
            if rle_z_count != 0 {
                if rle_z_count < 3 as libc::c_int as libc::c_uint {
                    (*d).m_huff_count[2 as libc::c_int as
                                          usize][0 as libc::c_int as usize] =
                        ((*d).m_huff_count[2 as libc::c_int as
                                               usize][0 as libc::c_int as
                                                          usize] as
                             libc::c_uint).wrapping_add(rle_z_count) as
                            mz_uint16;
                    loop  {
                        let fresh53 = rle_z_count;
                        rle_z_count = rle_z_count.wrapping_sub(1);
                        if !(fresh53 != 0) { break ; }
                        let fresh54 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh54 as usize] =
                            0 as libc::c_int as mz_uint8
                    }
                } else if rle_z_count <= 10 as libc::c_int as libc::c_uint {
                    (*d).m_huff_count[2 as libc::c_int as
                                          usize][17 as libc::c_int as usize] =
                        ((*d).m_huff_count[2 as libc::c_int as
                                               usize][17 as libc::c_int as
                                                          usize] as
                             libc::c_int + 1 as libc::c_int) as mz_uint16;
                    let fresh55 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh55 as usize] =
                        17 as libc::c_int as mz_uint8;
                    let fresh56 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh56 as usize] =
                        rle_z_count.wrapping_sub(3 as libc::c_int as
                                                     libc::c_uint) as mz_uint8
                } else {
                    (*d).m_huff_count[2 as libc::c_int as
                                          usize][18 as libc::c_int as usize] =
                        ((*d).m_huff_count[2 as libc::c_int as
                                               usize][18 as libc::c_int as
                                                          usize] as
                             libc::c_int + 1 as libc::c_int) as mz_uint16;
                    let fresh57 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh57 as usize] =
                        18 as libc::c_int as mz_uint8;
                    let fresh58 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh58 as usize] =
                        rle_z_count.wrapping_sub(11 as libc::c_int as
                                                     libc::c_uint) as mz_uint8
                }
                rle_z_count = 0 as libc::c_int as mz_uint
            }
            if code_size as libc::c_int != prev_code_size as libc::c_int {
                if rle_repeat_count != 0 {
                    if rle_repeat_count < 3 as libc::c_int as libc::c_uint {
                        (*d).m_huff_count[2 as libc::c_int as
                                              usize][prev_code_size as usize]
                            =
                            ((*d).m_huff_count[2 as libc::c_int as
                                                   usize][prev_code_size as
                                                              usize] as
                                 libc::c_uint).wrapping_add(rle_repeat_count)
                                as mz_uint16;
                        loop  {
                            let fresh59 = rle_repeat_count;
                            rle_repeat_count =
                                rle_repeat_count.wrapping_sub(1);
                            if !(fresh59 != 0) { break ; }
                            let fresh60 = num_packed_code_sizes;
                            num_packed_code_sizes =
                                num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh60 as usize] =
                                prev_code_size
                        }
                    } else {
                        (*d).m_huff_count[2 as libc::c_int as
                                              usize][16 as libc::c_int as
                                                         usize] =
                            ((*d).m_huff_count[2 as libc::c_int as
                                                   usize][16 as libc::c_int as
                                                              usize] as
                                 libc::c_int + 1 as libc::c_int) as mz_uint16;
                        let fresh61 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh61 as usize] =
                            16 as libc::c_int as mz_uint8;
                        let fresh62 = num_packed_code_sizes;
                        num_packed_code_sizes =
                            num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh62 as usize] =
                            rle_repeat_count.wrapping_sub(3 as libc::c_int as
                                                              libc::c_uint) as
                                mz_uint8
                    }
                    rle_repeat_count = 0 as libc::c_int as mz_uint
                }
                (*d).m_huff_count[2 as libc::c_int as
                                      usize][code_size as usize] =
                    ((*d).m_huff_count[2 as libc::c_int as
                                           usize][code_size as usize] as
                         libc::c_int + 1 as libc::c_int) as mz_uint16;
                let fresh63 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh63 as usize] = code_size
            } else {
                rle_repeat_count = rle_repeat_count.wrapping_add(1);
                if rle_repeat_count == 6 as libc::c_int as libc::c_uint {
                    if rle_repeat_count != 0 {
                        if rle_repeat_count < 3 as libc::c_int as libc::c_uint
                           {
                            (*d).m_huff_count[2 as libc::c_int as
                                                  usize][prev_code_size as
                                                             usize] =
                                ((*d).m_huff_count[2 as libc::c_int as
                                                       usize][prev_code_size
                                                                  as usize] as
                                     libc::c_uint).wrapping_add(rle_repeat_count)
                                    as mz_uint16;
                            loop  {
                                let fresh64 = rle_repeat_count;
                                rle_repeat_count =
                                    rle_repeat_count.wrapping_sub(1);
                                if !(fresh64 != 0) { break ; }
                                let fresh65 = num_packed_code_sizes;
                                num_packed_code_sizes =
                                    num_packed_code_sizes.wrapping_add(1);
                                packed_code_sizes[fresh65 as usize] =
                                    prev_code_size
                            }
                        } else {
                            (*d).m_huff_count[2 as libc::c_int as
                                                  usize][16 as libc::c_int as
                                                             usize] =
                                ((*d).m_huff_count[2 as libc::c_int as
                                                       usize][16 as
                                                                  libc::c_int
                                                                  as usize] as
                                     libc::c_int + 1 as libc::c_int) as
                                    mz_uint16;
                            let fresh66 = num_packed_code_sizes;
                            num_packed_code_sizes =
                                num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh66 as usize] =
                                16 as libc::c_int as mz_uint8;
                            let fresh67 = num_packed_code_sizes;
                            num_packed_code_sizes =
                                num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh67 as usize] =
                                rle_repeat_count.wrapping_sub(3 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                    as mz_uint8
                        }
                        rle_repeat_count = 0 as libc::c_int as mz_uint
                    }
                }
            }
        }
        prev_code_size = code_size;
        i = i.wrapping_add(1)
    }
    if rle_repeat_count != 0 {
        if rle_repeat_count != 0 {
            if rle_repeat_count < 3 as libc::c_int as libc::c_uint {
                (*d).m_huff_count[2 as libc::c_int as
                                      usize][prev_code_size as usize] =
                    ((*d).m_huff_count[2 as libc::c_int as
                                           usize][prev_code_size as usize] as
                         libc::c_uint).wrapping_add(rle_repeat_count) as
                        mz_uint16;
                loop  {
                    let fresh68 = rle_repeat_count;
                    rle_repeat_count = rle_repeat_count.wrapping_sub(1);
                    if !(fresh68 != 0) { break ; }
                    let fresh69 = num_packed_code_sizes;
                    num_packed_code_sizes =
                        num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh69 as usize] = prev_code_size
                }
            } else {
                (*d).m_huff_count[2 as libc::c_int as
                                      usize][16 as libc::c_int as usize] =
                    ((*d).m_huff_count[2 as libc::c_int as
                                           usize][16 as libc::c_int as usize]
                         as libc::c_int + 1 as libc::c_int) as mz_uint16;
                let fresh70 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh70 as usize] =
                    16 as libc::c_int as mz_uint8;
                let fresh71 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh71 as usize] =
                    rle_repeat_count.wrapping_sub(3 as libc::c_int as
                                                      libc::c_uint) as
                        mz_uint8
            }
            rle_repeat_count = 0 as libc::c_int as mz_uint
        }
    } else if rle_z_count != 0 {
        if rle_z_count < 3 as libc::c_int as libc::c_uint {
            (*d).m_huff_count[2 as libc::c_int as
                                  usize][0 as libc::c_int as usize] =
                ((*d).m_huff_count[2 as libc::c_int as
                                       usize][0 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add(rle_z_count) as mz_uint16;
            loop  {
                let fresh72 = rle_z_count;
                rle_z_count = rle_z_count.wrapping_sub(1);
                if !(fresh72 != 0) { break ; }
                let fresh73 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh73 as usize] =
                    0 as libc::c_int as mz_uint8
            }
        } else if rle_z_count <= 10 as libc::c_int as libc::c_uint {
            (*d).m_huff_count[2 as libc::c_int as
                                  usize][17 as libc::c_int as usize] =
                ((*d).m_huff_count[2 as libc::c_int as
                                       usize][17 as libc::c_int as usize] as
                     libc::c_int + 1 as libc::c_int) as mz_uint16;
            let fresh74 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh74 as usize] =
                17 as libc::c_int as mz_uint8;
            let fresh75 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh75 as usize] =
                rle_z_count.wrapping_sub(3 as libc::c_int as libc::c_uint) as
                    mz_uint8
        } else {
            (*d).m_huff_count[2 as libc::c_int as
                                  usize][18 as libc::c_int as usize] =
                ((*d).m_huff_count[2 as libc::c_int as
                                       usize][18 as libc::c_int as usize] as
                     libc::c_int + 1 as libc::c_int) as mz_uint16;
            let fresh76 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh76 as usize] =
                18 as libc::c_int as mz_uint8;
            let fresh77 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh77 as usize] =
                rle_z_count.wrapping_sub(11 as libc::c_int as libc::c_uint) as
                    mz_uint8
        }
        rle_z_count = 0 as libc::c_int as mz_uint
    }
    tdefl_optimize_huffman_table(d, 2 as libc::c_int,
                                 TDEFL_MAX_HUFF_SYMBOLS_2 as libc::c_int,
                                 7 as libc::c_int, 0 as libc::c_int);
    let mut bits: mz_uint = 2 as libc::c_int as mz_uint;
    let mut len: mz_uint = 2 as libc::c_int as mz_uint;
    if bits <= ((1 as libc::c_uint) << len).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      1937 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh78 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh78 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    }
    let mut bits_0: mz_uint = (num_lit_codes - 257 as libc::c_int) as mz_uint;
    let mut len_0: mz_uint = 5 as libc::c_int as mz_uint;
    if bits_0 <=
           ((1 as libc::c_uint) << len_0).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      1939 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits_0 << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len_0) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh79 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh79 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    }
    let mut bits_1: mz_uint = (num_dist_codes - 1 as libc::c_int) as mz_uint;
    let mut len_1: mz_uint = 5 as libc::c_int as mz_uint;
    if bits_1 <=
           ((1 as libc::c_uint) << len_1).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      1940 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits_1 << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len_1) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh80 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh80 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    }
    num_bit_lengths = 18 as libc::c_int;
    while num_bit_lengths >= 0 as libc::c_int {
        if (*d).m_huff_code_sizes[2 as libc::c_int as
                                      usize][s_tdefl_packed_code_size_syms_swizzle[num_bit_lengths
                                                                                       as
                                                                                       usize]
                                                 as usize] != 0 {
            break ;
        }
        num_bit_lengths -= 1
    }
    num_bit_lengths =
        if 4 as libc::c_int > num_bit_lengths + 1 as libc::c_int {
            4 as libc::c_int
        } else { (num_bit_lengths) + 1 as libc::c_int };
    let mut bits_2: mz_uint = (num_bit_lengths - 4 as libc::c_int) as mz_uint;
    let mut len_2: mz_uint = 4 as libc::c_int as mz_uint;
    if bits_2 <=
           ((1 as libc::c_uint) << len_2).wrapping_sub(1 as libc::c_uint) {
    } else {
        __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      1946 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
    }
    (*d).m_bit_buffer |= bits_2 << (*d).m_bits_in;
    (*d).m_bits_in =
        ((*d).m_bits_in as libc::c_uint).wrapping_add(len_2) as mz_uint as
            mz_uint;
    while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh81 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh81 = (*d).m_bit_buffer as mz_uint8
        }
        (*d).m_bit_buffer >>= 8 as libc::c_int;
        (*d).m_bits_in =
            ((*d).m_bits_in as
                 libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
                as mz_uint as mz_uint
    }
    i = 0 as libc::c_int as mz_uint;
    while (i as libc::c_int) < num_bit_lengths {
        let mut bits_3: mz_uint =
            (*d).m_huff_code_sizes[2 as libc::c_int as
                                       usize][s_tdefl_packed_code_size_syms_swizzle[i
                                                                                        as
                                                                                        usize]
                                                  as usize] as mz_uint;
        let mut len_3: mz_uint = 3 as libc::c_int as mz_uint;
        if bits_3 <=
               ((1 as libc::c_uint) << len_3).wrapping_sub(1 as libc::c_uint)
           {
        } else {
            __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          1948 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 51],
                                                    &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
        }
        (*d).m_bit_buffer |= bits_3 << (*d).m_bits_in;
        (*d).m_bits_in =
            ((*d).m_bits_in as libc::c_uint).wrapping_add(len_3) as mz_uint as
                mz_uint;
        while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh82 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh82 = (*d).m_bit_buffer as mz_uint8
            }
            (*d).m_bit_buffer >>= 8 as libc::c_int;
            (*d).m_bits_in =
                ((*d).m_bits_in as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint
                    as mz_uint
        }
        i = i.wrapping_add(1)
    }
    packed_code_sizes_index = 0 as libc::c_int as mz_uint;
    while packed_code_sizes_index < num_packed_code_sizes {
        let fresh83 = packed_code_sizes_index;
        packed_code_sizes_index = packed_code_sizes_index.wrapping_add(1);
        let mut code: mz_uint =
            packed_code_sizes[fresh83 as usize] as mz_uint;
        if code < TDEFL_MAX_HUFF_SYMBOLS_2 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"code < TDEFL_MAX_HUFF_SYMBOLS_2\x00" as *const u8
                              as *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          1953 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 51],
                                                    &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
        }
        let mut bits_4: mz_uint =
            (*d).m_huff_codes[2 as libc::c_int as usize][code as usize] as
                mz_uint;
        let mut len_4: mz_uint =
            (*d).m_huff_code_sizes[2 as libc::c_int as usize][code as usize]
                as mz_uint;
        if bits_4 <=
               ((1 as libc::c_uint) << len_4).wrapping_sub(1 as libc::c_uint)
           {
        } else {
            __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          1954 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 51],
                                                    &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
        }
        (*d).m_bit_buffer |= bits_4 << (*d).m_bits_in;
        (*d).m_bits_in =
            ((*d).m_bits_in as libc::c_uint).wrapping_add(len_4) as mz_uint as
                mz_uint;
        while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh84 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh84 = (*d).m_bit_buffer as mz_uint8
            }
            (*d).m_bit_buffer >>= 8 as libc::c_int;
            (*d).m_bits_in =
                ((*d).m_bits_in as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as mz_uint
                    as mz_uint
        }
        if code >= 16 as libc::c_int as libc::c_uint {
            let fresh85 = packed_code_sizes_index;
            packed_code_sizes_index = packed_code_sizes_index.wrapping_add(1);
            let mut bits_5: mz_uint =
                packed_code_sizes[fresh85 as usize] as mz_uint;
            let mut len_5: mz_uint =
                (*::std::mem::transmute::<&[u8; 4],
                                          &[libc::c_char; 4]>(b"\x02\x03\x07\x00"))[code.wrapping_sub(16
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint)
                                                                                        as
                                                                                        usize]
                    as mz_uint;
            if bits_5 <=
                   ((1 as libc::c_uint) <<
                        len_5).wrapping_sub(1 as libc::c_uint) {
            } else {
                __assert_fail(b"bits <= ((1U << len) - 1U)\x00" as *const u8
                                  as *const libc::c_char,
                              b"../engine/common/miniz.h\x00" as *const u8 as
                                  *const libc::c_char,
                              1956 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 51],
                                                        &[libc::c_char; 51]>(b"void tdefl_start_dynamic_block(tdefl_compressor *)\x00")).as_ptr());
            }
            (*d).m_bit_buffer |= bits_5 << (*d).m_bits_in;
            (*d).m_bits_in =
                ((*d).m_bits_in as libc::c_uint).wrapping_add(len_5) as
                    mz_uint as mz_uint;
            while (*d).m_bits_in >= 8 as libc::c_int as libc::c_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh86 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh86 = (*d).m_bit_buffer as mz_uint8
                }
                (*d).m_bit_buffer >>= 8 as libc::c_int;
                (*d).m_bits_in =
                    ((*d).m_bits_in as
                         libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint as mz_uint
            }
        }
    };
}
static mut s_tdefl_len_sym: [mz_uint16; 256] =
    [257 as libc::c_int as mz_uint16, 258 as libc::c_int as mz_uint16,
     259 as libc::c_int as mz_uint16, 260 as libc::c_int as mz_uint16,
     261 as libc::c_int as mz_uint16, 262 as libc::c_int as mz_uint16,
     263 as libc::c_int as mz_uint16, 264 as libc::c_int as mz_uint16,
     265 as libc::c_int as mz_uint16, 265 as libc::c_int as mz_uint16,
     266 as libc::c_int as mz_uint16, 266 as libc::c_int as mz_uint16,
     267 as libc::c_int as mz_uint16, 267 as libc::c_int as mz_uint16,
     268 as libc::c_int as mz_uint16, 268 as libc::c_int as mz_uint16,
     269 as libc::c_int as mz_uint16, 269 as libc::c_int as mz_uint16,
     269 as libc::c_int as mz_uint16, 269 as libc::c_int as mz_uint16,
     270 as libc::c_int as mz_uint16, 270 as libc::c_int as mz_uint16,
     270 as libc::c_int as mz_uint16, 270 as libc::c_int as mz_uint16,
     271 as libc::c_int as mz_uint16, 271 as libc::c_int as mz_uint16,
     271 as libc::c_int as mz_uint16, 271 as libc::c_int as mz_uint16,
     272 as libc::c_int as mz_uint16, 272 as libc::c_int as mz_uint16,
     272 as libc::c_int as mz_uint16, 272 as libc::c_int as mz_uint16,
     273 as libc::c_int as mz_uint16, 273 as libc::c_int as mz_uint16,
     273 as libc::c_int as mz_uint16, 273 as libc::c_int as mz_uint16,
     273 as libc::c_int as mz_uint16, 273 as libc::c_int as mz_uint16,
     273 as libc::c_int as mz_uint16, 273 as libc::c_int as mz_uint16,
     274 as libc::c_int as mz_uint16, 274 as libc::c_int as mz_uint16,
     274 as libc::c_int as mz_uint16, 274 as libc::c_int as mz_uint16,
     274 as libc::c_int as mz_uint16, 274 as libc::c_int as mz_uint16,
     274 as libc::c_int as mz_uint16, 274 as libc::c_int as mz_uint16,
     275 as libc::c_int as mz_uint16, 275 as libc::c_int as mz_uint16,
     275 as libc::c_int as mz_uint16, 275 as libc::c_int as mz_uint16,
     275 as libc::c_int as mz_uint16, 275 as libc::c_int as mz_uint16,
     275 as libc::c_int as mz_uint16, 275 as libc::c_int as mz_uint16,
     276 as libc::c_int as mz_uint16, 276 as libc::c_int as mz_uint16,
     276 as libc::c_int as mz_uint16, 276 as libc::c_int as mz_uint16,
     276 as libc::c_int as mz_uint16, 276 as libc::c_int as mz_uint16,
     276 as libc::c_int as mz_uint16, 276 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     277 as libc::c_int as mz_uint16, 277 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     278 as libc::c_int as mz_uint16, 278 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     279 as libc::c_int as mz_uint16, 279 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     280 as libc::c_int as mz_uint16, 280 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     281 as libc::c_int as mz_uint16, 281 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     282 as libc::c_int as mz_uint16, 282 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     283 as libc::c_int as mz_uint16, 283 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 284 as libc::c_int as mz_uint16,
     284 as libc::c_int as mz_uint16, 285 as libc::c_int as mz_uint16];
static mut s_tdefl_len_extra: [mz_uint8; 256] =
    [0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     1 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     1 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     1 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     1 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8];
static mut s_tdefl_small_dist_sym: [mz_uint8; 512] =
    [0 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     8 as libc::c_int as mz_uint8, 8 as libc::c_int as mz_uint8,
     8 as libc::c_int as mz_uint8, 8 as libc::c_int as mz_uint8,
     8 as libc::c_int as mz_uint8, 8 as libc::c_int as mz_uint8,
     8 as libc::c_int as mz_uint8, 8 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 9 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 9 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 9 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 9 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     14 as libc::c_int as mz_uint8, 14 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     15 as libc::c_int as mz_uint8, 15 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     16 as libc::c_int as mz_uint8, 16 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
     17 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8];
#[no_mangle]
pub unsafe extern "C" fn mz_deflateInit(mut pStream: mz_streamp,
                                        mut level: libc::c_int)
 -> libc::c_int {
    return mz_deflateInit2(pStream, level, 8 as libc::c_int,
                           15 as libc::c_int, 9 as libc::c_int,
                           MZ_DEFAULT_STRATEGY as libc::c_int);
}
static mut s_tdefl_small_dist_extra: [mz_uint8; 512] =
    [0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     1 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     1 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     2 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     3 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     4 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     5 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     6 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
     7 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8];
#[no_mangle]
pub unsafe extern "C" fn mz_version() -> *const libc::c_char {
    return b"10.1.0\x00" as *const u8 as *const libc::c_char;
}
static mut s_tdefl_large_dist_sym: [mz_uint8; 128] =
    [0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     18 as libc::c_int as mz_uint8, 19 as libc::c_int as mz_uint8,
     20 as libc::c_int as mz_uint8, 20 as libc::c_int as mz_uint8,
     21 as libc::c_int as mz_uint8, 21 as libc::c_int as mz_uint8,
     22 as libc::c_int as mz_uint8, 22 as libc::c_int as mz_uint8,
     22 as libc::c_int as mz_uint8, 22 as libc::c_int as mz_uint8,
     23 as libc::c_int as mz_uint8, 23 as libc::c_int as mz_uint8,
     23 as libc::c_int as mz_uint8, 23 as libc::c_int as mz_uint8,
     24 as libc::c_int as mz_uint8, 24 as libc::c_int as mz_uint8,
     24 as libc::c_int as mz_uint8, 24 as libc::c_int as mz_uint8,
     24 as libc::c_int as mz_uint8, 24 as libc::c_int as mz_uint8,
     24 as libc::c_int as mz_uint8, 24 as libc::c_int as mz_uint8,
     25 as libc::c_int as mz_uint8, 25 as libc::c_int as mz_uint8,
     25 as libc::c_int as mz_uint8, 25 as libc::c_int as mz_uint8,
     25 as libc::c_int as mz_uint8, 25 as libc::c_int as mz_uint8,
     25 as libc::c_int as mz_uint8, 25 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     26 as libc::c_int as mz_uint8, 26 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     27 as libc::c_int as mz_uint8, 27 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     28 as libc::c_int as mz_uint8, 28 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8,
     29 as libc::c_int as mz_uint8, 29 as libc::c_int as mz_uint8];
static mut s_tdefl_large_dist_extra: [mz_uint8; 128] =
    [0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
     8 as libc::c_int as mz_uint8, 8 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 9 as libc::c_int as mz_uint8,
     9 as libc::c_int as mz_uint8, 9 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     10 as libc::c_int as mz_uint8, 10 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     11 as libc::c_int as mz_uint8, 11 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     12 as libc::c_int as mz_uint8, 12 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8,
     13 as libc::c_int as mz_uint8, 13 as libc::c_int as mz_uint8];
static mut mz_bitmasks: [mz_uint; 17] =
    [0 as libc::c_int as mz_uint, 0x1 as libc::c_int as mz_uint,
     0x3 as libc::c_int as mz_uint, 0x7 as libc::c_int as mz_uint,
     0xf as libc::c_int as mz_uint, 0x1f as libc::c_int as mz_uint,
     0x3f as libc::c_int as mz_uint, 0x7f as libc::c_int as mz_uint,
     0xff as libc::c_int as mz_uint, 0x1ff as libc::c_int as mz_uint,
     0x3ff as libc::c_int as mz_uint, 0x7ff as libc::c_int as mz_uint,
     0xfff as libc::c_int as mz_uint, 0x1fff as libc::c_int as mz_uint,
     0x3fff as libc::c_int as mz_uint, 0x7fff as libc::c_int as mz_uint,
     0xffff as libc::c_int as mz_uint];
#[no_mangle]
pub unsafe extern "C" fn tdefl_write_image_to_png_file_in_memory_ex(mut pImage:
                                                                        *const libc::c_void,
                                                                    mut w:
                                                                        libc::c_int,
                                                                    mut h:
                                                                        libc::c_int,
                                                                    mut num_chans:
                                                                        libc::c_int,
                                                                    mut pLen_out:
                                                                        *mut size_t,
                                                                    mut level:
                                                                        mz_uint,
                                                                    mut flip:
                                                                        mz_bool)
 -> *mut libc::c_void {
    static mut s_tdefl_png_num_probes: [mz_uint; 11] =
        [0 as libc::c_int as mz_uint, 1 as libc::c_int as mz_uint,
         6 as libc::c_int as mz_uint, 32 as libc::c_int as mz_uint,
         16 as libc::c_int as mz_uint, 32 as libc::c_int as mz_uint,
         128 as libc::c_int as mz_uint, 256 as libc::c_int as mz_uint,
         512 as libc::c_int as mz_uint, 768 as libc::c_int as mz_uint,
         1500 as libc::c_int as mz_uint];
    let mut pComp: *mut tdefl_compressor =
        malloc(::std::mem::size_of::<tdefl_compressor>() as libc::c_ulong) as
            *mut tdefl_compressor;
    let mut out_buf: tdefl_output_buffer =
        tdefl_output_buffer{m_size: 0,
                            m_capacity: 0,
                            m_pBuf: 0 as *mut mz_uint8,
                            m_expandable: 0,};
    let mut i: libc::c_int = 0;
    let mut bpl: libc::c_int = w * num_chans;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut c: mz_uint32 = 0;
    *pLen_out = 0 as libc::c_int as size_t;
    if pComp.is_null() { return 0 as *mut libc::c_void }
    memset(&mut out_buf as *mut tdefl_output_buffer as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<tdefl_output_buffer>() as libc::c_ulong);
    out_buf.m_expandable = 1 as libc::c_int;
    out_buf.m_capacity =
        (57 as libc::c_int +
             (if 64 as libc::c_int > (1 as libc::c_int + bpl) * h {
                  64 as libc::c_int
              } else { (1 as libc::c_int + bpl) * h })) as size_t;
    out_buf.m_pBuf = malloc(out_buf.m_capacity) as *mut mz_uint8;
    if out_buf.m_pBuf.is_null() {
        free(pComp as *mut libc::c_void);
        return 0 as *mut libc::c_void
    }
    z = 41 as libc::c_int;
    while z != 0 {
        tdefl_output_buffer_putter(&mut z as *mut libc::c_int as
                                       *const libc::c_void, 1 as libc::c_int,
                                   &mut out_buf as *mut tdefl_output_buffer as
                                       *mut libc::c_void);
        z -= 1
    }
    tdefl_init(pComp,
               Some(tdefl_output_buffer_putter as
                        unsafe extern "C" fn(_: *const libc::c_void,
                                             _: libc::c_int,
                                             _: *mut libc::c_void)
                            -> mz_bool),
               &mut out_buf as *mut tdefl_output_buffer as *mut libc::c_void,
               (s_tdefl_png_num_probes[(if (10 as libc::c_int as libc::c_uint)
                                               < level {
                                            10 as libc::c_int as libc::c_uint
                                        } else { level }) as usize] |
                    TDEFL_WRITE_ZLIB_HEADER as libc::c_int as libc::c_uint) as
                   libc::c_int);
    y = 0 as libc::c_int;
    while y < h {
        tdefl_compress_buffer(pComp,
                              &mut z as *mut libc::c_int as
                                  *const libc::c_void,
                              1 as libc::c_int as size_t, TDEFL_NO_FLUSH);
        tdefl_compress_buffer(pComp,
                              (pImage as
                                   *mut mz_uint8).offset(((if flip != 0 {
                                                               (h -
                                                                    1 as
                                                                        libc::c_int)
                                                                   - y
                                                           } else { y }) *
                                                              bpl) as isize)
                                  as *const libc::c_void, bpl as size_t,
                              TDEFL_NO_FLUSH);
        y += 1
    }
    if tdefl_compress_buffer(pComp, 0 as *const libc::c_void,
                             0 as libc::c_int as size_t, TDEFL_FINISH) as
           libc::c_int != TDEFL_STATUS_DONE as libc::c_int {
        free(pComp as *mut libc::c_void);
        free(out_buf.m_pBuf as *mut libc::c_void);
        return 0 as *mut libc::c_void
    }
    *pLen_out =
        out_buf.m_size.wrapping_sub(41 as libc::c_int as libc::c_ulong);
    static mut chans: [mz_uint8; 5] =
        [0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0x4 as libc::c_int as mz_uint8, 0x2 as libc::c_int as mz_uint8,
         0x6 as libc::c_int as mz_uint8];
    let mut pnghdr: [mz_uint8; 41] =
        [0x89 as libc::c_int as mz_uint8, 0x50 as libc::c_int as mz_uint8,
         0x4e as libc::c_int as mz_uint8, 0x47 as libc::c_int as mz_uint8,
         0xd as libc::c_int as mz_uint8, 0xa as libc::c_int as mz_uint8,
         0x1a as libc::c_int as mz_uint8, 0xa as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0xd as libc::c_int as mz_uint8,
         0x49 as libc::c_int as mz_uint8, 0x48 as libc::c_int as mz_uint8,
         0x44 as libc::c_int as mz_uint8, 0x52 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0x8 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         0 as libc::c_int as mz_uint8, 0x49 as libc::c_int as mz_uint8,
         0x44 as libc::c_int as mz_uint8, 0x41 as libc::c_int as mz_uint8,
         0x54 as libc::c_int as mz_uint8];
    pnghdr[18 as libc::c_int as usize] = (w >> 8 as libc::c_int) as mz_uint8;
    pnghdr[19 as libc::c_int as usize] = w as mz_uint8;
    pnghdr[22 as libc::c_int as usize] = (h >> 8 as libc::c_int) as mz_uint8;
    pnghdr[23 as libc::c_int as usize] = h as mz_uint8;
    pnghdr[25 as libc::c_int as usize] = chans[num_chans as usize];
    pnghdr[33 as libc::c_int as usize] =
        (*pLen_out >> 24 as libc::c_int) as mz_uint8;
    pnghdr[34 as libc::c_int as usize] =
        (*pLen_out >> 16 as libc::c_int) as mz_uint8;
    pnghdr[35 as libc::c_int as usize] =
        (*pLen_out >> 8 as libc::c_int) as mz_uint8;
    pnghdr[36 as libc::c_int as usize] = *pLen_out as mz_uint8;
    c =
        mz_crc32(0 as libc::c_int as mz_ulong,
                 pnghdr.as_mut_ptr().offset(12 as libc::c_int as isize),
                 17 as libc::c_int as size_t) as mz_uint32;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *pnghdr.as_mut_ptr().offset(29 as libc::c_int as
                                        isize).offset(i as isize) =
            (c >> 24 as libc::c_int) as mz_uint8;
        i += 1;
        c <<= 8 as libc::c_int
    }
    memcpy(out_buf.m_pBuf as *mut libc::c_void,
           pnghdr.as_mut_ptr() as *const libc::c_void,
           41 as libc::c_int as libc::c_ulong);
    if tdefl_output_buffer_putter(b"\x00\x00\x00\x00\x00\x00\x00\x00IEND\xaeB`\x82\x00"
                                      as *const u8 as *const libc::c_char as
                                      *const libc::c_void, 16 as libc::c_int,
                                  &mut out_buf as *mut tdefl_output_buffer as
                                      *mut libc::c_void) == 0 {
        *pLen_out = 0 as libc::c_int as size_t;
        free(pComp as *mut libc::c_void);
        free(out_buf.m_pBuf as *mut libc::c_void);
        return 0 as *mut libc::c_void
    }
    c =
        mz_crc32(0 as libc::c_int as mz_ulong,
                 out_buf.m_pBuf.offset(41 as libc::c_int as
                                           isize).offset(-(4 as libc::c_int as
                                                               isize)),
                 (*pLen_out).wrapping_add(4 as libc::c_int as libc::c_ulong))
            as mz_uint32;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *out_buf.m_pBuf.offset(out_buf.m_size as
                                   isize).offset(-(16 as libc::c_int as
                                                       isize)).offset(i as
                                                                          isize)
            = (c >> 24 as libc::c_int) as mz_uint8;
        i += 1;
        c <<= 8 as libc::c_int
    }
    *pLen_out =
        (*pLen_out as
             libc::c_ulong).wrapping_add(57 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    free(pComp as *mut libc::c_void);
    return out_buf.m_pBuf as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_mem_to_heap(mut pSrc_buf:
                                                        *const libc::c_void,
                                                    mut src_buf_len: size_t,
                                                    mut pOut_len: *mut size_t,
                                                    mut flags: libc::c_int)
 -> *mut libc::c_void {
    let mut out_buf: tdefl_output_buffer =
        tdefl_output_buffer{m_size: 0,
                            m_capacity: 0,
                            m_pBuf: 0 as *mut mz_uint8,
                            m_expandable: 0,};
    memset(&mut out_buf as *mut tdefl_output_buffer as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<tdefl_output_buffer>() as libc::c_ulong);
    if pOut_len.is_null() {
        return 0 as *mut libc::c_void
    } else { *pOut_len = 0 as libc::c_int as size_t }
    out_buf.m_expandable = 1 as libc::c_int;
    if tdefl_compress_mem_to_output(pSrc_buf, src_buf_len,
                                    Some(tdefl_output_buffer_putter as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_void,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> mz_bool),
                                    &mut out_buf as *mut tdefl_output_buffer
                                        as *mut libc::c_void, flags) == 0 {
        return 0 as *mut libc::c_void
    }
    *pOut_len = out_buf.m_size;
    return out_buf.m_pBuf as *mut libc::c_void;
}
unsafe extern "C" fn tdefl_flush_output_buffer(mut d: *mut tdefl_compressor)
 -> tdefl_status {
    if !(*d).m_pIn_buf_size.is_null() {
        *(*d).m_pIn_buf_size =
            (*d).m_pSrc.wrapping_offset_from((*d).m_pIn_buf as
                                                 *const mz_uint8) as
                libc::c_long as size_t
    }
    if !(*d).m_pOut_buf_size.is_null() {
        let mut n: size_t =
            if (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs) <
                   (*d).m_output_flush_remaining as libc::c_ulong {
                (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
            } else { (*d).m_output_flush_remaining as libc::c_ulong };
        memcpy(((*d).m_pOut_buf as
                    *mut mz_uint8).offset((*d).m_out_buf_ofs as isize) as
                   *mut libc::c_void,
               (*d).m_output_buf.as_mut_ptr().offset((*d).m_output_flush_ofs
                                                         as isize) as
                   *const libc::c_void, n);
        (*d).m_output_flush_ofs =
            ((*d).m_output_flush_ofs as
                 libc::c_uint).wrapping_add(n as mz_uint) as mz_uint as
                mz_uint;
        (*d).m_output_flush_remaining =
            ((*d).m_output_flush_remaining as
                 libc::c_uint).wrapping_sub(n as mz_uint) as mz_uint as
                mz_uint;
        (*d).m_out_buf_ofs =
            ((*d).m_out_buf_ofs as libc::c_ulong).wrapping_add(n) as size_t as
                size_t;
        *(*d).m_pOut_buf_size = (*d).m_out_buf_ofs
    }
    return if (*d).m_finished != 0 && (*d).m_output_flush_remaining == 0 {
               TDEFL_STATUS_DONE as libc::c_int
           } else { TDEFL_STATUS_OKAY as libc::c_int } as tdefl_status;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress(mut d: *mut tdefl_compressor,
                                        mut pIn_buf: *const libc::c_void,
                                        mut pIn_buf_size: *mut size_t,
                                        mut pOut_buf: *mut libc::c_void,
                                        mut pOut_buf_size: *mut size_t,
                                        mut flush: tdefl_flush)
 -> tdefl_status {
    if d.is_null() {
        if !pIn_buf_size.is_null() {
            *pIn_buf_size = 0 as libc::c_int as size_t
        }
        if !pOut_buf_size.is_null() {
            *pOut_buf_size = 0 as libc::c_int as size_t
        }
        return TDEFL_STATUS_BAD_PARAM
    }
    (*d).m_pIn_buf = pIn_buf;
    (*d).m_pIn_buf_size = pIn_buf_size;
    (*d).m_pOut_buf = pOut_buf;
    (*d).m_pOut_buf_size = pOut_buf_size;
    (*d).m_pSrc = pIn_buf as *const mz_uint8;
    (*d).m_src_buf_left =
        if !pIn_buf_size.is_null() {
            *pIn_buf_size
        } else { 0 as libc::c_int as libc::c_ulong };
    (*d).m_out_buf_ofs = 0 as libc::c_int as size_t;
    (*d).m_flush = flush;
    if (*d).m_pPut_buf_func.is_some() as libc::c_int ==
           (!pOut_buf.is_null() || !pOut_buf_size.is_null()) as libc::c_int ||
           (*d).m_prev_return_status as libc::c_int !=
               TDEFL_STATUS_OKAY as libc::c_int ||
           (*d).m_wants_to_finish != 0 &&
               flush as libc::c_uint !=
                   TDEFL_FINISH as libc::c_int as libc::c_uint ||
           !pIn_buf_size.is_null() && *pIn_buf_size != 0 && pIn_buf.is_null()
           ||
           !pOut_buf_size.is_null() && *pOut_buf_size != 0 &&
               pOut_buf.is_null() {
        if !pIn_buf_size.is_null() {
            *pIn_buf_size = 0 as libc::c_int as size_t
        }
        if !pOut_buf_size.is_null() {
            *pOut_buf_size = 0 as libc::c_int as size_t
        }
        (*d).m_prev_return_status = TDEFL_STATUS_BAD_PARAM;
        return (*d).m_prev_return_status
    }
    (*d).m_wants_to_finish |=
        (flush as libc::c_uint == TDEFL_FINISH as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_uint;
    if (*d).m_output_flush_remaining != 0 || (*d).m_finished != 0 {
        (*d).m_prev_return_status = tdefl_flush_output_buffer(d);
        return (*d).m_prev_return_status
    }
    if (*d).m_flags & TDEFL_MAX_PROBES_MASK as libc::c_int as libc::c_uint ==
           1 as libc::c_int as libc::c_uint &&
           (*d).m_flags &
               TDEFL_GREEDY_PARSING_FLAG as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint &&
           (*d).m_flags &
               (TDEFL_FILTER_MATCHES as libc::c_int |
                    TDEFL_FORCE_ALL_RAW_BLOCKS as libc::c_int |
                    TDEFL_RLE_MATCHES as libc::c_int) as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
        if tdefl_compress_fast(d) == 0 { return (*d).m_prev_return_status }
    } else if tdefl_compress_normal(d) == 0 {
        return (*d).m_prev_return_status
    }
    if (*d).m_flags &
           (TDEFL_WRITE_ZLIB_HEADER as libc::c_int |
                TDEFL_COMPUTE_ADLER32 as libc::c_int) as libc::c_uint != 0 &&
           !pIn_buf.is_null() {
        (*d).m_adler32 =
            mz_adler32((*d).m_adler32 as mz_ulong, pIn_buf as *const mz_uint8,
                       (*d).m_pSrc.wrapping_offset_from(pIn_buf as
                                                            *const mz_uint8)
                           as libc::c_long as size_t) as mz_uint32
    }
    if flush as libc::c_uint != 0 && (*d).m_lookahead_size == 0 &&
           (*d).m_src_buf_left == 0 && (*d).m_output_flush_remaining == 0 {
        if tdefl_flush_block(d, flush as libc::c_int) < 0 as libc::c_int {
            return (*d).m_prev_return_status
        }
        (*d).m_finished =
            (flush as libc::c_uint ==
                 TDEFL_FINISH as libc::c_int as libc::c_uint) as libc::c_int
                as mz_uint;
        if flush as libc::c_uint ==
               TDEFL_FULL_FLUSH as libc::c_int as libc::c_uint {
            memset(&mut (*d).m_hash as *mut [mz_uint16; 32768] as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<[mz_uint16; 32768]>() as
                       libc::c_ulong);
            memset(&mut (*d).m_next as *mut [mz_uint16; 32768] as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<[mz_uint16; 32768]>() as
                       libc::c_ulong);
            (*d).m_dict_size = 0 as libc::c_int as mz_uint
        }
    }
    (*d).m_prev_return_status = tdefl_flush_output_buffer(d);
    return (*d).m_prev_return_status;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_get_adler32(mut d: *mut tdefl_compressor)
 -> mz_uint32 {
    return (*d).m_adler32;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_create_comp_flags_from_zip_params(mut level:
                                                                     libc::c_int,
                                                                 mut window_bits:
                                                                     libc::c_int,
                                                                 mut strategy:
                                                                     libc::c_int)
 -> mz_uint {
    let mut comp_flags: mz_uint =
        s_tdefl_num_probes[(if level >= 0 as libc::c_int {
                                (if (10 as libc::c_int) < level {
                                     10 as libc::c_int
                                 } else { level })
                            } else { MZ_DEFAULT_LEVEL as libc::c_int }) as
                               usize] |
            (if level <= 3 as libc::c_int {
                 TDEFL_GREEDY_PARSING_FLAG as libc::c_int
             } else { 0 as libc::c_int }) as libc::c_uint;
    if window_bits > 0 as libc::c_int {
        comp_flags |= TDEFL_WRITE_ZLIB_HEADER as libc::c_int as libc::c_uint
    }
    if level == 0 {
        comp_flags |=
            TDEFL_FORCE_ALL_RAW_BLOCKS as libc::c_int as libc::c_uint
    } else if strategy == MZ_FILTERED as libc::c_int {
        comp_flags |= TDEFL_FILTER_MATCHES as libc::c_int as libc::c_uint
    } else if strategy == MZ_HUFFMAN_ONLY as libc::c_int {
        comp_flags &= !(TDEFL_MAX_PROBES_MASK as libc::c_int) as libc::c_uint
    } else if strategy == MZ_FIXED as libc::c_int {
        comp_flags |=
            TDEFL_FORCE_ALL_STATIC_BLOCKS as libc::c_int as libc::c_uint
    } else if strategy == MZ_RLE as libc::c_int {
        comp_flags |= TDEFL_RLE_MATCHES as libc::c_int as libc::c_uint
    }
    return comp_flags;
}
unsafe extern "C" fn tdefl_output_buffer_putter(mut pBuf: *const libc::c_void,
                                                mut len: libc::c_int,
                                                mut pUser: *mut libc::c_void)
 -> mz_bool {
    let mut p: *mut tdefl_output_buffer = pUser as *mut tdefl_output_buffer;
    let mut new_size: size_t = (*p).m_size.wrapping_add(len as libc::c_ulong);
    if new_size > (*p).m_capacity {
        let mut new_capacity: size_t = (*p).m_capacity;
        let mut pNew_buf: *mut mz_uint8 = 0 as *mut mz_uint8;
        if (*p).m_expandable == 0 { return 0 as libc::c_int }
        loop  {
            new_capacity =
                if 128 as libc::c_uint as libc::c_ulong >
                       new_capacity << 1 as libc::c_uint {
                    128 as libc::c_uint as libc::c_ulong
                } else { (new_capacity) << 1 as libc::c_uint };
            if !(new_size > new_capacity) { break ; }
        }
        pNew_buf =
            realloc((*p).m_pBuf as *mut libc::c_void, new_capacity) as
                *mut mz_uint8;
        if pNew_buf.is_null() { return 0 as libc::c_int }
        (*p).m_pBuf = pNew_buf;
        (*p).m_capacity = new_capacity
    }
    memcpy((*p).m_pBuf.offset((*p).m_size as isize) as *mut libc::c_void,
           pBuf, len as libc::c_ulong);
    (*p).m_size = new_size;
    return 1 as libc::c_int;
}
static mut s_tdefl_num_probes: [mz_uint; 11] =
    [0 as libc::c_int as mz_uint, 1 as libc::c_int as mz_uint,
     6 as libc::c_int as mz_uint, 32 as libc::c_int as mz_uint,
     16 as libc::c_int as mz_uint, 32 as libc::c_int as mz_uint,
     128 as libc::c_int as mz_uint, 256 as libc::c_int as mz_uint,
     512 as libc::c_int as mz_uint, 768 as libc::c_int as mz_uint,
     1500 as libc::c_int as mz_uint];
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_mem_to_output(mut pBuf:
                                                          *const libc::c_void,
                                                      mut buf_len: size_t,
                                                      mut pPut_buf_func:
                                                          tdefl_put_buf_func_ptr,
                                                      mut pPut_buf_user:
                                                          *mut libc::c_void,
                                                      mut flags: libc::c_int)
 -> mz_bool {
    let mut pComp: *mut tdefl_compressor = 0 as *mut tdefl_compressor;
    let mut succeeded: mz_bool = 0;
    if buf_len != 0 && pBuf.is_null() || pPut_buf_func.is_none() {
        return 0 as libc::c_int
    }
    pComp =
        malloc(::std::mem::size_of::<tdefl_compressor>() as libc::c_ulong) as
            *mut tdefl_compressor;
    if pComp.is_null() { return 0 as libc::c_int }
    succeeded =
        (tdefl_init(pComp, pPut_buf_func, pPut_buf_user, flags) as libc::c_int
             == TDEFL_STATUS_OKAY as libc::c_int) as libc::c_int;
    succeeded =
        (succeeded != 0 &&
             tdefl_compress_buffer(pComp, pBuf, buf_len, TDEFL_FINISH) as
                 libc::c_int == TDEFL_STATUS_DONE as libc::c_int) as
            libc::c_int;
    free(pComp as *mut libc::c_void);
    return succeeded;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_get_prev_return_status(mut d:
                                                          *mut tdefl_compressor)
 -> tdefl_status {
    return (*d).m_prev_return_status;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_init(mut d: *mut tdefl_compressor,
                                    mut pPut_buf_func: tdefl_put_buf_func_ptr,
                                    mut pPut_buf_user: *mut libc::c_void,
                                    mut flags: libc::c_int) -> tdefl_status {
    (*d).m_pPut_buf_func = pPut_buf_func;
    (*d).m_pPut_buf_user = pPut_buf_user;
    (*d).m_flags = flags as mz_uint;
    (*d).m_max_probes[0 as libc::c_int as usize] =
        (1 as libc::c_int +
             ((flags & 0xfff as libc::c_int) + 2 as libc::c_int) /
                 3 as libc::c_int) as mz_uint;
    (*d).m_greedy_parsing =
        (flags & TDEFL_GREEDY_PARSING_FLAG as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
    (*d).m_max_probes[1 as libc::c_int as usize] =
        (1 as libc::c_int +
             (((flags & 0xfff as libc::c_int) >> 2 as libc::c_int) +
                  2 as libc::c_int) / 3 as libc::c_int) as mz_uint;
    if flags & TDEFL_NONDETERMINISTIC_PARSING_FLAG as libc::c_int == 0 {
        memset(&mut (*d).m_hash as *mut [mz_uint16; 32768] as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[mz_uint16; 32768]>() as libc::c_ulong);
    }
    (*d).m_bits_in = 0 as libc::c_int as mz_uint;
    (*d).m_lz_code_buf_dict_pos = (*d).m_bits_in;
    (*d).m_total_lz_bytes = (*d).m_lz_code_buf_dict_pos;
    (*d).m_dict_size = (*d).m_total_lz_bytes;
    (*d).m_lookahead_size = (*d).m_dict_size;
    (*d).m_lookahead_pos = (*d).m_lookahead_size;
    (*d).m_wants_to_finish = 0 as libc::c_int as mz_uint;
    (*d).m_bit_buffer = (*d).m_wants_to_finish;
    (*d).m_block_index = (*d).m_bit_buffer;
    (*d).m_finished = (*d).m_block_index;
    (*d).m_output_flush_remaining = (*d).m_finished;
    (*d).m_output_flush_ofs = (*d).m_output_flush_remaining;
    (*d).m_pLZ_code_buf =
        (*d).m_lz_code_buf.as_mut_ptr().offset(1 as libc::c_int as isize);
    (*d).m_pLZ_flags = (*d).m_lz_code_buf.as_mut_ptr();
    (*d).m_num_flags_left = 8 as libc::c_int as mz_uint;
    (*d).m_pOutput_buf = (*d).m_output_buf.as_mut_ptr();
    (*d).m_pOutput_buf_end = (*d).m_output_buf.as_mut_ptr();
    (*d).m_prev_return_status = TDEFL_STATUS_OKAY;
    (*d).m_saved_lit = 0 as libc::c_int as mz_uint;
    (*d).m_saved_match_len = (*d).m_saved_lit;
    (*d).m_saved_match_dist = (*d).m_saved_match_len;
    (*d).m_adler32 = 1 as libc::c_int as mz_uint;
    (*d).m_pIn_buf = 0 as *const libc::c_void;
    (*d).m_pOut_buf = 0 as *mut libc::c_void;
    (*d).m_pIn_buf_size = 0 as *mut size_t;
    (*d).m_pOut_buf_size = 0 as *mut size_t;
    (*d).m_flush = TDEFL_NO_FLUSH;
    (*d).m_pSrc = 0 as *const mz_uint8;
    (*d).m_src_buf_left = 0 as libc::c_int as size_t;
    (*d).m_out_buf_ofs = 0 as libc::c_int as size_t;
    if flags & TDEFL_NONDETERMINISTIC_PARSING_FLAG as libc::c_int == 0 {
        memset(&mut (*d).m_dict as *mut [mz_uint8; 33025] as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[mz_uint8; 33025]>() as libc::c_ulong);
    }
    memset(&mut *(*(*d).m_huff_count.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize)).as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
               as *mut mz_uint16 as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<mz_uint16>() as
                libc::c_ulong).wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_0 as
                                                libc::c_int as
                                                libc::c_ulong));
    memset(&mut *(*(*d).m_huff_count.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
               as *mut mz_uint16 as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<mz_uint16>() as
                libc::c_ulong).wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_1 as
                                                libc::c_int as
                                                libc::c_ulong));
    return TDEFL_STATUS_OKAY;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_buffer(mut d: *mut tdefl_compressor,
                                               mut pIn_buf:
                                                   *const libc::c_void,
                                               mut in_buf_size: size_t,
                                               mut flush: tdefl_flush)
 -> tdefl_status {
    if (*d).m_pPut_buf_func.is_some() {
    } else {
        __assert_fail(b"d->m_pPut_buf_func\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/miniz.h\x00" as *const u8 as
                          *const libc::c_char,
                      2851 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"tdefl_status tdefl_compress_buffer(tdefl_compressor *, const void *, size_t, tdefl_flush)\x00")).as_ptr());
    }
    return tdefl_compress(d, pIn_buf, &mut in_buf_size,
                          0 as *mut libc::c_void, 0 as *mut size_t, flush);
}
unsafe extern "C" fn tdefl_compress_normal(mut d: *mut tdefl_compressor)
 -> mz_bool {
    let mut pSrc: *const mz_uint8 = (*d).m_pSrc;
    let mut src_buf_left: size_t = (*d).m_src_buf_left;
    let mut flush: tdefl_flush = (*d).m_flush;
    while src_buf_left != 0 ||
              flush as libc::c_uint != 0 && (*d).m_lookahead_size != 0 {
        let mut len_to_move: mz_uint = 0;
        let mut cur_match_dist: mz_uint = 0;
        let mut cur_match_len: mz_uint = 0;
        let mut cur_pos: mz_uint = 0;
        if (*d).m_lookahead_size.wrapping_add((*d).m_dict_size) >=
               (TDEFL_MIN_MATCH_LEN as libc::c_int - 1 as libc::c_int) as
                   libc::c_uint {
            let mut dst_pos: mz_uint =
                (*d).m_lookahead_pos.wrapping_add((*d).m_lookahead_size) &
                    TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
            let mut ins_pos: mz_uint =
                (*d).m_lookahead_pos.wrapping_add((*d).m_lookahead_size).wrapping_sub(2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint);
            let mut hash: mz_uint =
                (((*d).m_dict[(ins_pos &
                                   TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                                       libc::c_uint) as usize] as libc::c_int)
                     << TDEFL_LZ_HASH_SHIFT as libc::c_int ^
                     (*d).m_dict[(ins_pos.wrapping_add(1 as libc::c_int as
                                                           libc::c_uint) &
                                      TDEFL_LZ_DICT_SIZE_MASK as libc::c_int
                                          as libc::c_uint) as usize] as
                         libc::c_int) as mz_uint;
            let mut num_bytes_to_process: mz_uint =
                if src_buf_left <
                       (TDEFL_MAX_MATCH_LEN as libc::c_int as
                            libc::c_uint).wrapping_sub((*d).m_lookahead_size)
                           as libc::c_ulong {
                    src_buf_left
                } else {
                    (TDEFL_MAX_MATCH_LEN as libc::c_int as
                         libc::c_uint).wrapping_sub((*d).m_lookahead_size) as
                        libc::c_ulong
                } as mz_uint;
            let mut pSrc_end: *const mz_uint8 =
                pSrc.offset(num_bytes_to_process as isize);
            src_buf_left =
                (src_buf_left as
                     libc::c_ulong).wrapping_sub(num_bytes_to_process as
                                                     libc::c_ulong) as size_t
                    as size_t;
            (*d).m_lookahead_size =
                ((*d).m_lookahead_size as
                     libc::c_uint).wrapping_add(num_bytes_to_process) as
                    mz_uint as mz_uint;
            while pSrc != pSrc_end {
                let fresh87 = pSrc;
                pSrc = pSrc.offset(1);
                let mut c: mz_uint8 = *fresh87;
                (*d).m_dict[dst_pos as usize] = c;
                if dst_pos <
                       (TDEFL_MAX_MATCH_LEN as libc::c_int - 1 as libc::c_int)
                           as libc::c_uint {
                    (*d).m_dict[(TDEFL_LZ_DICT_SIZE as libc::c_int as
                                     libc::c_uint).wrapping_add(dst_pos) as
                                    usize] = c
                }
                hash =
                    (hash << TDEFL_LZ_HASH_SHIFT as libc::c_int ^
                         c as libc::c_uint) &
                        (TDEFL_LZ_HASH_SIZE as libc::c_int - 1 as libc::c_int)
                            as libc::c_uint;
                (*d).m_next[(ins_pos &
                                 TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                                     libc::c_uint) as usize] =
                    (*d).m_hash[hash as usize];
                (*d).m_hash[hash as usize] = ins_pos as mz_uint16;
                dst_pos =
                    dst_pos.wrapping_add(1 as libc::c_int as libc::c_uint) &
                        TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                            libc::c_uint;
                ins_pos = ins_pos.wrapping_add(1)
            }
        } else {
            while src_buf_left != 0 &&
                      (*d).m_lookahead_size <
                          TDEFL_MAX_MATCH_LEN as libc::c_int as libc::c_uint {
                let fresh88 = pSrc;
                pSrc = pSrc.offset(1);
                let mut c_0: mz_uint8 = *fresh88;
                let mut dst_pos_0: mz_uint =
                    (*d).m_lookahead_pos.wrapping_add((*d).m_lookahead_size) &
                        TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                            libc::c_uint;
                src_buf_left = src_buf_left.wrapping_sub(1);
                (*d).m_dict[dst_pos_0 as usize] = c_0;
                if dst_pos_0 <
                       (TDEFL_MAX_MATCH_LEN as libc::c_int - 1 as libc::c_int)
                           as libc::c_uint {
                    (*d).m_dict[(TDEFL_LZ_DICT_SIZE as libc::c_int as
                                     libc::c_uint).wrapping_add(dst_pos_0) as
                                    usize] = c_0
                }
                (*d).m_lookahead_size = (*d).m_lookahead_size.wrapping_add(1);
                if (*d).m_lookahead_size.wrapping_add((*d).m_dict_size) >=
                       TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint {
                    let mut ins_pos_0: mz_uint =
                        (*d).m_lookahead_pos.wrapping_add((*d).m_lookahead_size.wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_sub(2
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint);
                    let mut hash_0: mz_uint =
                        ((((*d).m_dict[(ins_pos_0 &
                                            TDEFL_LZ_DICT_SIZE_MASK as
                                                libc::c_int as libc::c_uint)
                                           as usize] as libc::c_int) <<
                              TDEFL_LZ_HASH_SHIFT as libc::c_int *
                                  2 as libc::c_int ^
                              ((*d).m_dict[(ins_pos_0.wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                                &
                                                TDEFL_LZ_DICT_SIZE_MASK as
                                                    libc::c_int as
                                                    libc::c_uint) as usize] as
                                   libc::c_int) <<
                                  TDEFL_LZ_HASH_SHIFT as libc::c_int ^
                              c_0 as libc::c_int) &
                             TDEFL_LZ_HASH_SIZE as libc::c_int -
                                 1 as libc::c_int) as mz_uint;
                    (*d).m_next[(ins_pos_0 &
                                     TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                                         libc::c_uint) as usize] =
                        (*d).m_hash[hash_0 as usize];
                    (*d).m_hash[hash_0 as usize] = ins_pos_0 as mz_uint16
                }
            }
        }
        (*d).m_dict_size =
            if (TDEFL_LZ_DICT_SIZE as libc::c_int as
                    libc::c_uint).wrapping_sub((*d).m_lookahead_size) <
                   (*d).m_dict_size {
                (TDEFL_LZ_DICT_SIZE as libc::c_int as
                     libc::c_uint).wrapping_sub((*d).m_lookahead_size)
            } else { (*d).m_dict_size };
        if flush as u64 == 0 &&
               (*d).m_lookahead_size <
                   TDEFL_MAX_MATCH_LEN as libc::c_int as libc::c_uint {
            break ;
        }
        len_to_move = 1 as libc::c_int as mz_uint;
        cur_match_dist = 0 as libc::c_int as mz_uint;
        cur_match_len =
            if (*d).m_saved_match_len != 0 {
                (*d).m_saved_match_len
            } else {
                (TDEFL_MIN_MATCH_LEN as libc::c_int - 1 as libc::c_int) as
                    libc::c_uint
            };
        cur_pos =
            (*d).m_lookahead_pos &
                TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as libc::c_uint;
        if (*d).m_flags &
               (TDEFL_RLE_MATCHES as libc::c_int |
                    TDEFL_FORCE_ALL_RAW_BLOCKS as libc::c_int) as libc::c_uint
               != 0 {
            if (*d).m_dict_size != 0 &&
                   (*d).m_flags &
                       TDEFL_FORCE_ALL_RAW_BLOCKS as libc::c_int as
                           libc::c_uint == 0 {
                let mut c_1: mz_uint8 =
                    (*d).m_dict[(cur_pos.wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) &
                                     TDEFL_LZ_DICT_SIZE_MASK as libc::c_int as
                                         libc::c_uint) as usize];
                cur_match_len = 0 as libc::c_int as mz_uint;
                while cur_match_len < (*d).m_lookahead_size {
                    if (*d).m_dict[cur_pos.wrapping_add(cur_match_len) as
                                       usize] as libc::c_int !=
                           c_1 as libc::c_int {
                        break ;
                    }
                    cur_match_len = cur_match_len.wrapping_add(1)
                }
                if cur_match_len <
                       TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint {
                    cur_match_len = 0 as libc::c_int as mz_uint
                } else { cur_match_dist = 1 as libc::c_int as mz_uint }
            }
        } else {
            tdefl_find_match(d, (*d).m_lookahead_pos, (*d).m_dict_size,
                             (*d).m_lookahead_size, &mut cur_match_dist,
                             &mut cur_match_len);
        }
        if cur_match_len == TDEFL_MIN_MATCH_LEN as libc::c_int as libc::c_uint
               &&
               cur_match_dist >=
                   (8 as libc::c_uint).wrapping_mul(1024 as libc::c_uint) ||
               cur_pos == cur_match_dist ||
               (*d).m_flags &
                   TDEFL_FILTER_MATCHES as libc::c_int as libc::c_uint != 0 &&
                   cur_match_len <= 5 as libc::c_int as libc::c_uint {
            cur_match_len = 0 as libc::c_int as mz_uint;
            cur_match_dist = cur_match_len
        }
        if (*d).m_saved_match_len != 0 {
            if cur_match_len > (*d).m_saved_match_len {
                tdefl_record_literal(d, (*d).m_saved_lit as mz_uint8);
                if cur_match_len >= 128 as libc::c_int as libc::c_uint {
                    tdefl_record_match(d, cur_match_len, cur_match_dist);
                    (*d).m_saved_match_len = 0 as libc::c_int as mz_uint;
                    len_to_move = cur_match_len
                } else {
                    (*d).m_saved_lit =
                        (*d).m_dict[cur_pos as usize] as mz_uint;
                    (*d).m_saved_match_dist = cur_match_dist;
                    (*d).m_saved_match_len = cur_match_len
                }
            } else {
                tdefl_record_match(d, (*d).m_saved_match_len,
                                   (*d).m_saved_match_dist);
                len_to_move =
                    (*d).m_saved_match_len.wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint);
                (*d).m_saved_match_len = 0 as libc::c_int as mz_uint
            }
        } else if cur_match_dist == 0 {
            tdefl_record_literal(d,
                                 (*d).m_dict[if (cur_pos as libc::c_ulong) <
                                                    (::std::mem::size_of::<[mz_uint8; 33025]>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                {
                                                 cur_pos as libc::c_ulong
                                             } else {
                                                 (::std::mem::size_of::<[mz_uint8; 33025]>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                                             } as usize]);
        } else if (*d).m_greedy_parsing != 0 ||
                      (*d).m_flags &
                          TDEFL_RLE_MATCHES as libc::c_int as libc::c_uint !=
                          0 ||
                      cur_match_len >= 128 as libc::c_int as libc::c_uint {
            tdefl_record_match(d, cur_match_len, cur_match_dist);
            len_to_move = cur_match_len
        } else {
            (*d).m_saved_lit =
                (*d).m_dict[if (cur_pos as libc::c_ulong) <
                                   (::std::mem::size_of::<[mz_uint8; 33025]>()
                                        as
                                        libc::c_ulong).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                               {
                                cur_pos as libc::c_ulong
                            } else {
                                (::std::mem::size_of::<[mz_uint8; 33025]>() as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                            } as usize] as mz_uint;
            (*d).m_saved_match_dist = cur_match_dist;
            (*d).m_saved_match_len = cur_match_len
        }
        (*d).m_lookahead_pos =
            ((*d).m_lookahead_pos as libc::c_uint).wrapping_add(len_to_move)
                as mz_uint as mz_uint;
        if (*d).m_lookahead_size >= len_to_move {
        } else {
            __assert_fail(b"d->m_lookahead_size >= len_to_move\x00" as
                              *const u8 as *const libc::c_char,
                          b"../engine/common/miniz.h\x00" as *const u8 as
                              *const libc::c_char,
                          2740 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"mz_bool tdefl_compress_normal(tdefl_compressor *)\x00")).as_ptr());
        }
        (*d).m_lookahead_size =
            ((*d).m_lookahead_size as libc::c_uint).wrapping_sub(len_to_move)
                as mz_uint as mz_uint;
        (*d).m_dict_size =
            if (*d).m_dict_size.wrapping_add(len_to_move) <
                   TDEFL_LZ_DICT_SIZE as libc::c_int as mz_uint {
                (*d).m_dict_size.wrapping_add(len_to_move)
            } else { TDEFL_LZ_DICT_SIZE as libc::c_int as mz_uint };
        if (*d).m_pLZ_code_buf >
               &mut *(*d).m_lz_code_buf.as_mut_ptr().offset((TDEFL_LZ_CODE_BUF_SIZE
                                                                 as
                                                                 libc::c_int -
                                                                 8 as
                                                                     libc::c_int)
                                                                as isize) as
                   *mut mz_uint8 ||
               (*d).m_total_lz_bytes >
                   (31 as libc::c_int * 1024 as libc::c_int) as libc::c_uint
                   &&
                   (((*d).m_pLZ_code_buf.wrapping_offset_from((*d).m_lz_code_buf.as_mut_ptr())
                         as libc::c_long as
                         mz_uint).wrapping_mul(115 as libc::c_int as
                                                   libc::c_uint) >>
                        7 as libc::c_int >= (*d).m_total_lz_bytes ||
                        (*d).m_flags &
                            TDEFL_FORCE_ALL_RAW_BLOCKS as libc::c_int as
                                libc::c_uint != 0) {
            let mut n: libc::c_int = 0;
            (*d).m_pSrc = pSrc;
            (*d).m_src_buf_left = src_buf_left;
            n = tdefl_flush_block(d, 0 as libc::c_int);
            if n != 0 as libc::c_int {
                return if n < 0 as libc::c_int {
                           0 as libc::c_int
                       } else { 1 as libc::c_int }
            }
        }
    }
    (*d).m_pSrc = pSrc;
    (*d).m_src_buf_left = src_buf_left;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_mem_to_mem(mut pOut_buf:
                                                       *mut libc::c_void,
                                                   mut out_buf_len: size_t,
                                                   mut pSrc_buf:
                                                       *const libc::c_void,
                                                   mut src_buf_len: size_t,
                                                   mut flags: libc::c_int)
 -> size_t {
    let mut out_buf: tdefl_output_buffer =
        tdefl_output_buffer{m_size: 0,
                            m_capacity: 0,
                            m_pBuf: 0 as *mut mz_uint8,
                            m_expandable: 0,};
    memset(&mut out_buf as *mut tdefl_output_buffer as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<tdefl_output_buffer>() as libc::c_ulong);
    if pOut_buf.is_null() { return 0 as libc::c_int as size_t }
    out_buf.m_pBuf = pOut_buf as *mut mz_uint8;
    out_buf.m_capacity = out_buf_len;
    if tdefl_compress_mem_to_output(pSrc_buf, src_buf_len,
                                    Some(tdefl_output_buffer_putter as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_void,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> mz_bool),
                                    &mut out_buf as *mut tdefl_output_buffer
                                        as *mut libc::c_void, flags) == 0 {
        return 0 as libc::c_int as size_t
    }
    return out_buf.m_size;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_write_image_to_png_file_in_memory(mut pImage:
                                                                     *const libc::c_void,
                                                                 mut w:
                                                                     libc::c_int,
                                                                 mut h:
                                                                     libc::c_int,
                                                                 mut num_chans:
                                                                     libc::c_int,
                                                                 mut pLen_out:
                                                                     *mut size_t)
 -> *mut libc::c_void {
    return tdefl_write_image_to_png_file_in_memory_ex(pImage, w, h, num_chans,
                                                      pLen_out,
                                                      6 as libc::c_int as
                                                          mz_uint,
                                                      0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compressor_alloc() -> *mut tdefl_compressor {
    return malloc(::std::mem::size_of::<tdefl_compressor>() as libc::c_ulong)
               as *mut tdefl_compressor;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compressor_free(mut pComp:
                                                   *mut tdefl_compressor) {
    free(pComp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress(mut r: *mut tinfl_decompressor,
                                          mut pIn_buf_next: *const mz_uint8,
                                          mut pIn_buf_size: *mut size_t,
                                          mut pOut_buf_start: *mut mz_uint8,
                                          mut pOut_buf_next: *mut mz_uint8,
                                          mut pOut_buf_size: *mut size_t,
                                          decomp_flags: mz_uint32)
 -> tinfl_status {
    let mut s_1: mz_uint = 0;
    let mut c_12: mz_uint = 0;
    let mut c_11: mz_uint = 0;
    let mut pSrc: *mut mz_uint8 = 0 as *mut mz_uint8;
    let mut extra_bits_0: mz_uint = 0;
    let mut c_10: mz_uint = 0;
    let mut temp_1: libc::c_int = 0;
    let mut code_len_2: mz_uint = 0;
    let mut c_9: mz_uint = 0;
    let mut extra_bits: mz_uint = 0;
    let mut c_8: mz_uint = 0;
    let mut temp_0: libc::c_int = 0;
    let mut code_len_0: mz_uint = 0;
    let mut c_7: mz_uint = 0;
    let mut tree_next: libc::c_int = 0;
    let mut tree_cur: libc::c_int = 0;
    let mut pTable: *mut tinfl_huff_table = 0 as *mut tinfl_huff_table;
    let mut i_0: mz_uint = 0;
    let mut j: mz_uint = 0;
    let mut used_syms: mz_uint = 0;
    let mut total: mz_uint = 0;
    let mut sym_index: mz_uint = 0;
    let mut next_code: [mz_uint; 17] = [0; 17];
    let mut total_syms: [mz_uint; 16] = [0; 16];
    let mut s_0: mz_uint = 0;
    let mut c_6: mz_uint = 0;
    let mut temp: libc::c_int = 0;
    let mut code_len: mz_uint = 0;
    let mut c_5: mz_uint = 0;
    let mut s: mz_uint = 0;
    let mut c_4: mz_uint = 0;
    let mut c_3: mz_uint = 0;
    let mut n: size_t = 0;
    let mut c_2: mz_uint = 0;
    let mut c_1: mz_uint = 0;
    let mut c_0: mz_uint = 0;
    let mut c: mz_uint = 0;
    let mut current_block: u64;
    static mut s_length_base: [libc::c_int; 31] =
        [3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int,
         6 as libc::c_int, 7 as libc::c_int, 8 as libc::c_int,
         9 as libc::c_int, 10 as libc::c_int, 11 as libc::c_int,
         13 as libc::c_int, 15 as libc::c_int, 17 as libc::c_int,
         19 as libc::c_int, 23 as libc::c_int, 27 as libc::c_int,
         31 as libc::c_int, 35 as libc::c_int, 43 as libc::c_int,
         51 as libc::c_int, 59 as libc::c_int, 67 as libc::c_int,
         83 as libc::c_int, 99 as libc::c_int, 115 as libc::c_int,
         131 as libc::c_int, 163 as libc::c_int, 195 as libc::c_int,
         227 as libc::c_int, 258 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int];
    static mut s_length_extra: [libc::c_int; 31] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int];
    static mut s_dist_base: [libc::c_int; 32] =
        [1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 7 as libc::c_int,
         9 as libc::c_int, 13 as libc::c_int, 17 as libc::c_int,
         25 as libc::c_int, 33 as libc::c_int, 49 as libc::c_int,
         65 as libc::c_int, 97 as libc::c_int, 129 as libc::c_int,
         193 as libc::c_int, 257 as libc::c_int, 385 as libc::c_int,
         513 as libc::c_int, 769 as libc::c_int, 1025 as libc::c_int,
         1537 as libc::c_int, 2049 as libc::c_int, 3073 as libc::c_int,
         4097 as libc::c_int, 6145 as libc::c_int, 8193 as libc::c_int,
         12289 as libc::c_int, 16385 as libc::c_int, 24577 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int];
    static mut s_dist_extra: [libc::c_int; 32] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         6 as libc::c_int, 7 as libc::c_int, 7 as libc::c_int,
         8 as libc::c_int, 8 as libc::c_int, 9 as libc::c_int,
         9 as libc::c_int, 10 as libc::c_int, 10 as libc::c_int,
         11 as libc::c_int, 11 as libc::c_int, 12 as libc::c_int,
         12 as libc::c_int, 13 as libc::c_int, 13 as libc::c_int, 0, 0];
    static mut s_length_dezigzag: [mz_uint8; 19] =
        [16 as libc::c_int as mz_uint8, 17 as libc::c_int as mz_uint8,
         18 as libc::c_int as mz_uint8, 0 as libc::c_int as mz_uint8,
         8 as libc::c_int as mz_uint8, 7 as libc::c_int as mz_uint8,
         9 as libc::c_int as mz_uint8, 6 as libc::c_int as mz_uint8,
         10 as libc::c_int as mz_uint8, 5 as libc::c_int as mz_uint8,
         11 as libc::c_int as mz_uint8, 4 as libc::c_int as mz_uint8,
         12 as libc::c_int as mz_uint8, 3 as libc::c_int as mz_uint8,
         13 as libc::c_int as mz_uint8, 2 as libc::c_int as mz_uint8,
         14 as libc::c_int as mz_uint8, 1 as libc::c_int as mz_uint8,
         15 as libc::c_int as mz_uint8];
    static mut s_min_table_sizes: [libc::c_int; 3] =
        [257 as libc::c_int, 1 as libc::c_int, 4 as libc::c_int];
    let mut status: tinfl_status = TINFL_STATUS_FAILED;
    let mut num_bits: mz_uint32 = 0;
    let mut dist: mz_uint32 = 0;
    let mut counter: mz_uint32 = 0;
    let mut num_extra: mz_uint32 = 0;
    let mut bit_buf: tinfl_bit_buf_t = 0;
    let mut pIn_buf_cur: *const mz_uint8 = pIn_buf_next;
    let pIn_buf_end: *const mz_uint8 =
        pIn_buf_next.offset(*pIn_buf_size as isize);
    let mut pOut_buf_cur: *mut mz_uint8 = pOut_buf_next;
    let pOut_buf_end: *mut mz_uint8 =
        pOut_buf_next.offset(*pOut_buf_size as isize);
    let mut out_buf_size_mask: size_t =
        if decomp_flags &
               TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as libc::c_int as
                   libc::c_uint != 0 {
            -(1 as libc::c_int) as size_t
        } else {
            (pOut_buf_next.wrapping_offset_from(pOut_buf_start) as
                 libc::c_long as
                 libc::c_ulong).wrapping_add(*pOut_buf_size).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
        };
    let mut dist_from_out_buf_start: size_t = 0;
    if out_buf_size_mask.wrapping_add(1 as libc::c_int as libc::c_ulong) &
           out_buf_size_mask != 0 || pOut_buf_next < pOut_buf_start {
        *pOut_buf_size = 0 as libc::c_int as size_t;
        *pIn_buf_size = *pOut_buf_size;
        return TINFL_STATUS_BAD_PARAM
    }
    num_bits = (*r).m_num_bits;
    bit_buf = (*r).m_bit_buf;
    dist = (*r).m_dist;
    counter = (*r).m_counter;
    num_extra = (*r).m_num_extra;
    dist_from_out_buf_start = (*r).m_dist_from_out_buf_start;
    match (*r).m_state {
        0 => {
            (*r).m_zhdr1 = 0 as libc::c_int as mz_uint32;
            (*r).m_zhdr0 = (*r).m_zhdr1;
            num_extra = (*r).m_zhdr0;
            counter = num_extra;
            dist = counter;
            num_bits = dist;
            bit_buf = num_bits as tinfl_bit_buf_t;
            (*r).m_check_adler32 = 1 as libc::c_int as mz_uint32;
            (*r).m_z_adler32 = (*r).m_check_adler32;
            if decomp_flags &
                   TINFL_FLAG_PARSE_ZLIB_HEADER as libc::c_int as libc::c_uint
                   != 0 {
                current_block = 13797916685926291137;
            } else { current_block = 8151474771948790331; }
        }
        1 => { current_block = 13797916685926291137; }
        2 => { current_block = 4761528863920922185; }
        36 => { current_block = 3938820862080741272; }
        3 => { current_block = 6072622540298447352; }
        5 => { current_block = 479107131381816815; }
        6 => { current_block = 1428307939028130064; }
        7 => { current_block = 8304106758420804164; }
        39 => { current_block = 17336970397495664729; }
        51 => { current_block = 9838996637140935403; }
        52 => { current_block = 10784681114964964746; }
        9 => { current_block = 16580259026179177070; }
        38 => { current_block = 17648591037158480576; }
        10 => { current_block = 5131379528150656272; }
        11 => { current_block = 8288085890650723895; }
        14 => { current_block = 5565703735569783978; }
        35 => { current_block = 9914851455145855695; }
        16 => { current_block = 14785121481331406365; }
        17 => { current_block = 11975512581081481720; }
        18 => { current_block = 8577262330622874318; }
        21 => { current_block = 3178910365268327865; }
        23 => { current_block = 14852891895508065921; }
        24 => { current_block = 7054538615515007887; }
        25 => { current_block = 5330158124161540915; }
        26 => { current_block = 2030331543845570311; }
        27 => { current_block = 9945048139610397779; }
        37 => { current_block = 17083204083870333287; }
        53 => { current_block = 9997576841122142810; }
        32 => { current_block = 8078296886873617068; }
        41 => { current_block = 10368141810877879434; }
        42 => { current_block = 2644446111302234773; }
        34 => { current_block = 6415062399687569957; }
        _ => { current_block = 17988355554207322903; }
    }
    match current_block {
        13797916685926291137 => {
            if pIn_buf_cur >= pIn_buf_end {
                status =
                    if decomp_flags &
                           TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                               libc::c_uint != 0 {
                        TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                            libc::c_int
                    } as tinfl_status;
                (*r).m_state = 1 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
            } else {
                let fresh89 = pIn_buf_cur;
                pIn_buf_cur = pIn_buf_cur.offset(1);
                (*r).m_zhdr0 = *fresh89 as mz_uint32;
                current_block = 4761528863920922185;
            }
        }
        _ => { }
    }
    match current_block {
        4761528863920922185 => {
            if pIn_buf_cur >= pIn_buf_end {
                status =
                    if decomp_flags &
                           TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                               libc::c_uint != 0 {
                        TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                            libc::c_int
                    } as tinfl_status;
                (*r).m_state = 2 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
            } else {
                let fresh90 = pIn_buf_cur;
                pIn_buf_cur = pIn_buf_cur.offset(1);
                (*r).m_zhdr1 = *fresh90 as mz_uint32;
                counter =
                    ((*r).m_zhdr0.wrapping_mul(256 as libc::c_int as
                                                   libc::c_uint).wrapping_add((*r).m_zhdr1).wrapping_rem(31
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                         != 0 as libc::c_int as libc::c_uint ||
                         (*r).m_zhdr1 & 32 as libc::c_int as libc::c_uint != 0
                         ||
                         (*r).m_zhdr0 & 15 as libc::c_int as libc::c_uint !=
                             8 as libc::c_int as libc::c_uint) as libc::c_int
                        as mz_uint32;
                if decomp_flags &
                       TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as libc::c_int
                           as libc::c_uint == 0 {
                    counter |=
                        ((1 as libc::c_uint) <<
                             (8 as
                                  libc::c_uint).wrapping_add((*r).m_zhdr0 >>
                                                                 4 as
                                                                     libc::c_int)
                             > 32768 as libc::c_uint ||
                             out_buf_size_mask.wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                                 <
                                 ((1 as libc::c_uint) <<
                                      (8 as
                                           libc::c_uint).wrapping_add((*r).m_zhdr0
                                                                          >>
                                                                          4 as
                                                                              libc::c_int))
                                     as size_t) as libc::c_int as libc::c_uint
                }
                if counter != 0 {
                    current_block = 3938820862080741272;
                } else { current_block = 8151474771948790331; }
            }
        }
        _ => { }
    }
    match current_block {
        3938820862080741272 => {
            status = TINFL_STATUS_FAILED;
            (*r).m_state = 36 as libc::c_int as mz_uint32;
            current_block = 17988355554207322903;
        }
        _ => { }
    }
    loop  {
        match current_block {
            8151474771948790331 => {
                if num_bits < 3 as libc::c_int as mz_uint {
                    current_block = 17233182392562552756;
                } else { current_block = 12264624100856317061; }
            }
            8078296886873617068 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 32 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh123 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_11 = *fresh123 as mz_uint;
                    bit_buf |= (c_11 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < num_bits & 7 as libc::c_int as libc::c_uint
                       {
                        current_block = 17582560547174494307;
                    } else { current_block = 15251772801061689047; }
                }
            }
            9997576841122142810 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 53 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh121 = dist_from_out_buf_start;
                    dist_from_out_buf_start =
                        dist_from_out_buf_start.wrapping_add(1);
                    let fresh122 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh122 =
                        *pOut_buf_start.offset((fresh121.wrapping_sub(dist as
                                                                          libc::c_ulong)
                                                    & out_buf_size_mask) as
                                                   isize)
                }
                current_block = 16251542583832361733;
            }
            17083204083870333287 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 37 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            9945048139610397779 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 27 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh119 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_10 = *fresh119 as mz_uint;
                    bit_buf |= (c_10 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < num_extra {
                        current_block = 12303096423297627778;
                    } else { current_block = 12860518208867661037; }
                }
            }
            2030331543845570311 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 26 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh117 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_9 = *fresh117 as mz_uint;
                    bit_buf |= (c_9 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 15 as libc::c_int as libc::c_uint {
                        current_block = 15235053155526978372;
                    } else { current_block = 13191700919457454999; }
                }
            }
            5330158124161540915 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 25 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh115 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_8 = *fresh115 as mz_uint;
                    bit_buf |= (c_8 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < num_extra {
                        current_block = 16708736497050535312;
                    } else { current_block = 12189673838652692395; }
                }
            }
            7054538615515007887 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 24 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh112 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh112 = counter as mz_uint8
                }
                current_block = 15980792246487107818;
            }
            14852891895508065921 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 23 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh110 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_7 = *fresh110 as mz_uint;
                    bit_buf |= (c_7 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 15 as libc::c_int as libc::c_uint {
                        current_block = 8592218650707875588;
                    } else { current_block = 2358954804400455004; }
                }
            }
            3178910365268327865 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 21 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            8577262330622874318 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 18 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh108 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_6 = *fresh108 as mz_uint;
                    bit_buf |= (c_6 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < num_extra {
                        current_block = 11426676793787244633;
                    } else { current_block = 2365107296380899847; }
                }
            }
            11975512581081481720 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 17 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            14785121481331406365 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 16 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh105 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_5 = *fresh105 as mz_uint;
                    bit_buf |= (c_5 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 15 as libc::c_int as libc::c_uint {
                        current_block = 14898553815918780345;
                    } else { current_block = 1387597052038031995; }
                }
            }
            9914851455145855695 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 35 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            5565703735569783978 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 14 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh102 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_4 = *fresh102 as mz_uint;
                    bit_buf |= (c_4 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 3 as libc::c_int as mz_uint {
                        current_block = 622347125454405425;
                    } else { current_block = 7591354809551635686; }
                }
            }
            8288085890650723895 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 11 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh101 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_3 = *fresh101 as mz_uint;
                    bit_buf |= (c_3 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits <
                           (*::std::mem::transmute::<&[u8; 4],
                                                     &[libc::c_char; 4]>(b"\x05\x05\x04\x00"))[counter
                                                                                                   as
                                                                                                   usize]
                               as mz_uint {
                        current_block = 15513414128719341844;
                    } else { current_block = 14723615986260991866; }
                }
            }
            5131379528150656272 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 10 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            17648591037158480576 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 38 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    n =
                        if (if (pOut_buf_end.wrapping_offset_from(pOut_buf_cur)
                                    as libc::c_long as size_t) <
                                   pIn_buf_end.wrapping_offset_from(pIn_buf_cur)
                                       as libc::c_long as size_t {
                                pOut_buf_end.wrapping_offset_from(pOut_buf_cur)
                                    as libc::c_long as size_t
                            } else {
                                pIn_buf_end.wrapping_offset_from(pIn_buf_cur)
                                    as libc::c_long as size_t
                            }) < counter as libc::c_ulong {
                            if (pOut_buf_end.wrapping_offset_from(pOut_buf_cur)
                                    as libc::c_long as size_t) <
                                   pIn_buf_end.wrapping_offset_from(pIn_buf_cur)
                                       as libc::c_long as size_t {
                                pOut_buf_end.wrapping_offset_from(pOut_buf_cur)
                                    as libc::c_long as size_t
                            } else {
                                pIn_buf_end.wrapping_offset_from(pIn_buf_cur)
                                    as libc::c_long as size_t
                            }
                        } else { counter as libc::c_ulong };
                    memcpy(pOut_buf_cur as *mut libc::c_void,
                           pIn_buf_cur as *const libc::c_void, n);
                    pIn_buf_cur = pIn_buf_cur.offset(n as isize);
                    pOut_buf_cur = pOut_buf_cur.offset(n as isize);
                    counter =
                        (counter as libc::c_uint).wrapping_sub(n as mz_uint)
                            as mz_uint32 as mz_uint32
                }
                current_block = 2089914658669629659;
            }
            16580259026179177070 => {
                if !(pOut_buf_cur >= pOut_buf_end) {
                    current_block = 17648591037158480576;
                    continue ;
                }
                status = TINFL_STATUS_HAS_MORE_OUTPUT;
                (*r).m_state = 9 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            10784681114964964746 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 52 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh96 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh96 = dist as mz_uint8;
                    counter = counter.wrapping_sub(1)
                }
                current_block = 10938659635288570931;
            }
            9838996637140935403 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 51 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh95 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_2 = *fresh95 as mz_uint;
                    bit_buf |= (c_2 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 8 as libc::c_int as mz_uint {
                        current_block = 16937825661756021828;
                    } else { current_block = 10783567741412653655; }
                }
            }
            17336970397495664729 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 39 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            8304106758420804164 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 7 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh94 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    (*r).m_raw_header[counter as usize] = *fresh94
                }
                current_block = 17769492591016358583;
            }
            1428307939028130064 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 6 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh93 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_1 = *fresh93 as mz_uint;
                    bit_buf |= (c_1 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 8 as libc::c_int as mz_uint {
                        current_block = 15734707049249739970;
                    } else { current_block = 13201766686570145889; }
                }
            }
            479107131381816815 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 5 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh92 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_0 = *fresh92 as mz_uint;
                    bit_buf |= (c_0 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < num_bits & 7 as libc::c_int as libc::c_uint
                       {
                        current_block = 11739054925370445424;
                    } else { current_block = 10261677128829721533; }
                }
            }
            6072622540298447352 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 3 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh91 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c = *fresh91 as mz_uint;
                    bit_buf |= (c as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 3 as libc::c_int as mz_uint {
                        current_block = 17233182392562552756;
                    } else { current_block = 12264624100856317061; }
                }
            }
            2644446111302234773 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 42 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh125 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    s_1 = *fresh125 as mz_uint
                }
                current_block = 13238870711695007221;
            }
            6415062399687569957 => {
                status = TINFL_STATUS_DONE;
                (*r).m_state = 34 as libc::c_int as mz_uint32;
                current_block = 17988355554207322903;
                continue ;
            }
            17988355554207322903 => {
                if status as libc::c_int !=
                       TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int &&
                       status as libc::c_int !=
                           TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                               libc::c_int {
                    while pIn_buf_cur > pIn_buf_next &&
                              num_bits >= 8 as libc::c_int as libc::c_uint {
                        pIn_buf_cur = pIn_buf_cur.offset(-1);
                        num_bits =
                            (num_bits as
                                 libc::c_uint).wrapping_sub(8 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as mz_uint32 as mz_uint32
                    }
                }
                break ;
            }
            _ => {
                if pIn_buf_cur >= pIn_buf_end {
                    status =
                        if decomp_flags &
                               TINFL_FLAG_HAS_MORE_INPUT as libc::c_int as
                                   libc::c_uint != 0 {
                            TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int
                        } else {
                            TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as
                                libc::c_int
                        } as tinfl_status;
                    (*r).m_state = 41 as libc::c_int as mz_uint32;
                    current_block = 17988355554207322903;
                    continue ;
                } else {
                    let fresh124 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_12 = *fresh124 as mz_uint;
                    bit_buf |= (c_12 as tinfl_bit_buf_t) << num_bits;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    if num_bits < 8 as libc::c_int as mz_uint {
                        current_block = 4847010591044017726;
                    } else { current_block = 5335649609539064255; }
                }
            }
        }
        match current_block {
            12264624100856317061 => {
                (*r).m_final =
                    (bit_buf &
                         (((1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_ulong) as
                        mz_uint32;
                bit_buf >>= 3 as libc::c_int;
                num_bits =
                    (num_bits as
                         libc::c_uint).wrapping_sub(3 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint32 as mz_uint32;
                (*r).m_type = (*r).m_final >> 1 as libc::c_int;
                if (*r).m_type == 0 as libc::c_int as libc::c_uint {
                    if num_bits < num_bits & 7 as libc::c_int as libc::c_uint
                       {
                        current_block = 11739054925370445424;
                    } else { current_block = 10261677128829721533; }
                } else {
                    if (*r).m_type == 3 as libc::c_int as libc::c_uint {
                        current_block = 5131379528150656272;
                        continue ;
                    }
                    if (*r).m_type == 1 as libc::c_int as libc::c_uint {
                        let mut p: *mut mz_uint8 =
                            (*r).m_tables[0 as libc::c_int as
                                              usize].m_code_size.as_mut_ptr();
                        let mut i: mz_uint = 0;
                        (*r).m_table_sizes[0 as libc::c_int as usize] =
                            288 as libc::c_int as mz_uint32;
                        (*r).m_table_sizes[1 as libc::c_int as usize] =
                            32 as libc::c_int as mz_uint32;
                        memset((*r).m_tables[1 as libc::c_int as
                                                 usize].m_code_size.as_mut_ptr()
                                   as *mut libc::c_void, 5 as libc::c_int,
                               32 as libc::c_int as libc::c_ulong);
                        i = 0 as libc::c_int as mz_uint;
                        while i <= 143 as libc::c_int as libc::c_uint {
                            let fresh97 = p;
                            p = p.offset(1);
                            *fresh97 = 8 as libc::c_int as mz_uint8;
                            i = i.wrapping_add(1)
                        }
                        while i <= 255 as libc::c_int as libc::c_uint {
                            let fresh98 = p;
                            p = p.offset(1);
                            *fresh98 = 9 as libc::c_int as mz_uint8;
                            i = i.wrapping_add(1)
                        }
                        while i <= 279 as libc::c_int as libc::c_uint {
                            let fresh99 = p;
                            p = p.offset(1);
                            *fresh99 = 7 as libc::c_int as mz_uint8;
                            i = i.wrapping_add(1)
                        }
                        while i <= 287 as libc::c_int as libc::c_uint {
                            let fresh100 = p;
                            p = p.offset(1);
                            *fresh100 = 8 as libc::c_int as mz_uint8;
                            i = i.wrapping_add(1)
                        }
                        current_block = 13419727033115386574;
                    } else {
                        counter = 0 as libc::c_int as mz_uint32;
                        current_block = 11978944022089568707;
                    }
                }
            }
            17233182392562552756 => {
                c = 0;
                current_block = 6072622540298447352;
                continue ;
            }
            _ => { }
        }
        match current_block {
            10261677128829721533 => {
                bit_buf >>= num_bits & 7 as libc::c_int as libc::c_uint;
                num_bits =
                    (num_bits as
                         libc::c_uint).wrapping_sub(num_bits &
                                                        7 as libc::c_int as
                                                            libc::c_uint) as
                        mz_uint32 as mz_uint32;
                counter = 0 as libc::c_int as mz_uint32;
                current_block = 178030534879405462;
            }
            11739054925370445424 => {
                c_0 = 0;
                current_block = 479107131381816815;
                continue ;
            }
            _ => { }
        }
        loop  {
            match current_block {
                15734707049249739970 => {
                    c_1 = 0;
                    current_block = 1428307939028130064;
                    break ;
                }
                5335649609539064255 => {
                    s_1 =
                        (bit_buf &
                             (((1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint;
                    bit_buf >>= 8 as libc::c_int;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    current_block = 13238870711695007221;
                    continue ;
                }
                15251772801061689047 => {
                    bit_buf >>= num_bits & 7 as libc::c_int as libc::c_uint;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_sub(num_bits &
                                                            7 as libc::c_int
                                                                as
                                                                libc::c_uint)
                            as mz_uint32 as mz_uint32;
                    while pIn_buf_cur > pIn_buf_next &&
                              num_bits >= 8 as libc::c_int as libc::c_uint {
                        pIn_buf_cur = pIn_buf_cur.offset(-1);
                        num_bits =
                            (num_bits as
                                 libc::c_uint).wrapping_sub(8 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as mz_uint32 as mz_uint32
                    }
                    bit_buf &=
                        ((1 as libc::c_int as mz_uint64) <<
                             num_bits).wrapping_sub(1 as libc::c_int as
                                                        mz_uint64);
                    if num_bits == 0 {
                    } else {
                        __assert_fail(b"!num_bits\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"../engine/common/miniz.h\x00" as
                                          *const u8 as *const libc::c_char,
                                      3687 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 131],
                                                                &[libc::c_char; 131]>(b"tinfl_status tinfl_decompress(tinfl_decompressor *, const mz_uint8 *, size_t *, mz_uint8 *, mz_uint8 *, size_t *, const mz_uint32)\x00")).as_ptr());
                    }
                    if !(decomp_flags &
                             TINFL_FLAG_PARSE_ZLIB_HEADER as libc::c_int as
                                 libc::c_uint != 0) {
                        current_block = 6415062399687569957;
                        break ;
                    }
                    counter = 0 as libc::c_int as mz_uint32;
                    current_block = 13895028675981758206;
                }
                12860518208867661037 => {
                    extra_bits_0 =
                        (bit_buf &
                             (((1 as libc::c_int) << num_extra) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint;
                    bit_buf >>= num_extra;
                    num_bits =
                        (num_bits as libc::c_uint).wrapping_sub(num_extra) as
                            mz_uint32 as mz_uint32;
                    dist =
                        (dist as libc::c_uint).wrapping_add(extra_bits_0) as
                            mz_uint32 as mz_uint32;
                    current_block = 15826704491191897766;
                }
                13191700919457454999 => {
                    temp_1 =
                        (*r).m_tables[1 as libc::c_int as
                                          usize].m_look_up[(bit_buf &
                                                                (TINFL_FAST_LOOKUP_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong)
                                                               as usize] as
                            libc::c_int;
                    if temp_1 >= 0 as libc::c_int {
                        code_len_2 = (temp_1 >> 9 as libc::c_int) as mz_uint;
                        temp_1 &= 511 as libc::c_int
                    } else {
                        code_len_2 =
                            TINFL_FAST_LOOKUP_BITS as libc::c_int as mz_uint;
                        loop  {
                            let fresh118 = code_len_2;
                            code_len_2 = code_len_2.wrapping_add(1);
                            temp_1 =
                                (*r).m_tables[1 as libc::c_int as
                                                  usize].m_tree[(!temp_1 as
                                                                     libc::c_ulong).wrapping_add(bit_buf
                                                                                                     >>
                                                                                                     fresh118
                                                                                                     &
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                    as usize]
                                    as libc::c_int;
                            if !(temp_1 < 0 as libc::c_int) { break ; }
                        }
                    }
                    dist = temp_1 as mz_uint32;
                    bit_buf >>= code_len_2;
                    num_bits =
                        (num_bits as libc::c_uint).wrapping_sub(code_len_2) as
                            mz_uint32 as mz_uint32;
                    num_extra = s_dist_extra[dist as usize] as mz_uint32;
                    dist = s_dist_base[dist as usize] as mz_uint32;
                    if num_extra != 0 {
                        extra_bits_0 = 0;
                        if num_bits < num_extra {
                            current_block = 12303096423297627778;
                            continue ;
                        } else {
                            current_block = 12860518208867661037;
                            continue ;
                        }
                    } else { current_block = 15826704491191897766; }
                }
                15235053155526978372 => {
                    temp_1 =
                        (*r).m_tables[1 as libc::c_int as
                                          usize].m_look_up[(bit_buf &
                                                                (TINFL_FAST_LOOKUP_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong)
                                                               as usize] as
                            libc::c_int;
                    if temp_1 >= 0 as libc::c_int {
                        code_len_2 = (temp_1 >> 9 as libc::c_int) as mz_uint;
                        if code_len_2 != 0 && num_bits >= code_len_2 {
                            current_block = 13191700919457454999;
                            continue ;
                        } else {
                            current_block = 2030331543845570311;
                            break ;
                        }
                    } else {
                        if !(num_bits >
                                 TINFL_FAST_LOOKUP_BITS as libc::c_int as
                                     libc::c_uint) {
                            current_block = 2030331543845570311;
                            break ;
                        }
                        code_len_2 =
                            TINFL_FAST_LOOKUP_BITS as libc::c_int as mz_uint;
                        loop  {
                            let fresh116 = code_len_2;
                            code_len_2 = code_len_2.wrapping_add(1);
                            temp_1 =
                                (*r).m_tables[1 as libc::c_int as
                                                  usize].m_tree[(!temp_1 as
                                                                     libc::c_ulong).wrapping_add(bit_buf
                                                                                                     >>
                                                                                                     fresh116
                                                                                                     &
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                    as usize]
                                    as libc::c_int;
                            if !(temp_1 < 0 as libc::c_int &&
                                     num_bits >=
                                         code_len_2.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
                               {
                                break ;
                            }
                        }
                        if temp_1 >= 0 as libc::c_int {
                            current_block = 13191700919457454999;
                            continue ;
                        } else {
                            current_block = 2030331543845570311;
                            break ;
                        }
                    }
                }
                12189673838652692395 => {
                    extra_bits =
                        (bit_buf &
                             (((1 as libc::c_int) << num_extra) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint;
                    bit_buf >>= num_extra;
                    num_bits =
                        (num_bits as libc::c_uint).wrapping_sub(num_extra) as
                            mz_uint32 as mz_uint32;
                    counter =
                        (counter as libc::c_uint).wrapping_add(extra_bits) as
                            mz_uint32 as mz_uint32;
                    current_block = 15736258377632409300;
                }
                15980792246487107818 => {
                    if (pIn_buf_end.wrapping_offset_from(pIn_buf_cur) as
                            libc::c_long) < 4 as libc::c_int as libc::c_long
                           ||
                           (pOut_buf_end.wrapping_offset_from(pOut_buf_cur) as
                                libc::c_long) <
                               2 as libc::c_int as libc::c_long {
                        temp_0 = 0;
                        code_len_0 = 0;
                        c_7 = 0;
                        if !(num_bits < 15 as libc::c_int as libc::c_uint) {
                            current_block = 2358954804400455004;
                            continue ;
                        }
                        if (pIn_buf_end.wrapping_offset_from(pIn_buf_cur) as
                                libc::c_long) <
                               2 as libc::c_int as libc::c_long {
                            current_block = 8592218650707875588;
                            continue ;
                        }
                        bit_buf |=
                            (*pIn_buf_cur.offset(0 as libc::c_int as isize) as
                                 tinfl_bit_buf_t) << num_bits |
                                (*pIn_buf_cur.offset(1 as libc::c_int as
                                                         isize) as
                                     tinfl_bit_buf_t) <<
                                    num_bits.wrapping_add(8 as libc::c_int as
                                                              libc::c_uint);
                        pIn_buf_cur =
                            pIn_buf_cur.offset(2 as libc::c_int as isize);
                        num_bits =
                            (num_bits as
                                 libc::c_uint).wrapping_add(16 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as mz_uint32 as mz_uint32;
                        current_block = 2358954804400455004;
                        continue ;
                    } else {
                        let mut sym2: libc::c_int = 0;
                        let mut code_len_1: mz_uint = 0;
                        if num_bits < 30 as libc::c_int as libc::c_uint {
                            bit_buf |=
                                (*(pIn_buf_cur as *const mz_uint32) as
                                     tinfl_bit_buf_t) << num_bits;
                            pIn_buf_cur =
                                pIn_buf_cur.offset(4 as libc::c_int as isize);
                            num_bits =
                                (num_bits as
                                     libc::c_uint).wrapping_add(32 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as mz_uint32 as mz_uint32
                        }
                        sym2 =
                            (*r).m_tables[0 as libc::c_int as
                                              usize].m_look_up[(bit_buf &
                                                                    (TINFL_FAST_LOOKUP_SIZE
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as usize]
                                as libc::c_int;
                        if sym2 >= 0 as libc::c_int {
                            code_len_1 = (sym2 >> 9 as libc::c_int) as mz_uint
                        } else {
                            code_len_1 =
                                TINFL_FAST_LOOKUP_BITS as libc::c_int as
                                    mz_uint;
                            loop  {
                                let fresh113 = code_len_1;
                                code_len_1 = code_len_1.wrapping_add(1);
                                sym2 =
                                    (*r).m_tables[0 as libc::c_int as
                                                      usize].m_tree[(!sym2 as
                                                                         libc::c_ulong).wrapping_add(bit_buf
                                                                                                         >>
                                                                                                         fresh113
                                                                                                         &
                                                                                                         1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                        as
                                                                        usize]
                                        as libc::c_int;
                                if !(sym2 < 0 as libc::c_int) { break ; }
                            }
                        }
                        counter = sym2 as mz_uint32;
                        bit_buf >>= code_len_1;
                        num_bits =
                            (num_bits as
                                 libc::c_uint).wrapping_sub(code_len_1) as
                                mz_uint32 as mz_uint32;
                        if !(counter & 256 as libc::c_int as libc::c_uint !=
                                 0) {
                            sym2 =
                                (*r).m_tables[0 as libc::c_int as
                                                  usize].m_look_up[(bit_buf &
                                                                        (TINFL_FAST_LOOKUP_SIZE
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            libc::c_ulong)
                                                                       as
                                                                       usize]
                                    as libc::c_int;
                            if sym2 >= 0 as libc::c_int {
                                code_len_1 =
                                    (sym2 >> 9 as libc::c_int) as mz_uint
                            } else {
                                code_len_1 =
                                    TINFL_FAST_LOOKUP_BITS as libc::c_int as
                                        mz_uint;
                                loop  {
                                    let fresh114 = code_len_1;
                                    code_len_1 = code_len_1.wrapping_add(1);
                                    sym2 =
                                        (*r).m_tables[0 as libc::c_int as
                                                          usize].m_tree[(!sym2
                                                                             as
                                                                             libc::c_ulong).wrapping_add(bit_buf
                                                                                                             >>
                                                                                                             fresh114
                                                                                                             &
                                                                                                             1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                                            as
                                                                            usize]
                                            as libc::c_int;
                                    if !(sym2 < 0 as libc::c_int) { break ; }
                                }
                            }
                            bit_buf >>= code_len_1;
                            num_bits =
                                (num_bits as
                                     libc::c_uint).wrapping_sub(code_len_1) as
                                    mz_uint32 as mz_uint32;
                            *pOut_buf_cur.offset(0 as libc::c_int as isize) =
                                counter as mz_uint8;
                            if sym2 & 256 as libc::c_int != 0 {
                                pOut_buf_cur = pOut_buf_cur.offset(1);
                                counter = sym2 as mz_uint32
                            } else {
                                *pOut_buf_cur.offset(1 as libc::c_int as
                                                         isize) =
                                    sym2 as mz_uint8;
                                pOut_buf_cur =
                                    pOut_buf_cur.offset(2 as libc::c_int as
                                                            isize);
                                current_block = 15980792246487107818;
                                continue ;
                            }
                        }
                    }
                    current_block = 18421180869108055851;
                }
                2358954804400455004 => {
                    temp_0 =
                        (*r).m_tables[0 as libc::c_int as
                                          usize].m_look_up[(bit_buf &
                                                                (TINFL_FAST_LOOKUP_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong)
                                                               as usize] as
                            libc::c_int;
                    if temp_0 >= 0 as libc::c_int {
                        code_len_0 = (temp_0 >> 9 as libc::c_int) as mz_uint;
                        temp_0 &= 511 as libc::c_int
                    } else {
                        code_len_0 =
                            TINFL_FAST_LOOKUP_BITS as libc::c_int as mz_uint;
                        loop  {
                            let fresh111 = code_len_0;
                            code_len_0 = code_len_0.wrapping_add(1);
                            temp_0 =
                                (*r).m_tables[0 as libc::c_int as
                                                  usize].m_tree[(!temp_0 as
                                                                     libc::c_ulong).wrapping_add(bit_buf
                                                                                                     >>
                                                                                                     fresh111
                                                                                                     &
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                    as usize]
                                    as libc::c_int;
                            if !(temp_0 < 0 as libc::c_int) { break ; }
                        }
                    }
                    counter = temp_0 as mz_uint32;
                    bit_buf >>= code_len_0;
                    num_bits =
                        (num_bits as libc::c_uint).wrapping_sub(code_len_0) as
                            mz_uint32 as mz_uint32;
                    if !(counter >= 256 as libc::c_int as libc::c_uint) {
                        current_block = 7054538615515007887;
                        break ;
                    }
                    current_block = 18421180869108055851;
                }
                8592218650707875588 => {
                    temp_0 =
                        (*r).m_tables[0 as libc::c_int as
                                          usize].m_look_up[(bit_buf &
                                                                (TINFL_FAST_LOOKUP_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong)
                                                               as usize] as
                            libc::c_int;
                    if temp_0 >= 0 as libc::c_int {
                        code_len_0 = (temp_0 >> 9 as libc::c_int) as mz_uint;
                        if code_len_0 != 0 && num_bits >= code_len_0 {
                            current_block = 2358954804400455004;
                            continue ;
                        } else {
                            current_block = 14852891895508065921;
                            break ;
                        }
                    } else {
                        if !(num_bits >
                                 TINFL_FAST_LOOKUP_BITS as libc::c_int as
                                     libc::c_uint) {
                            current_block = 14852891895508065921;
                            break ;
                        }
                        code_len_0 =
                            TINFL_FAST_LOOKUP_BITS as libc::c_int as mz_uint;
                        loop  {
                            let fresh109 = code_len_0;
                            code_len_0 = code_len_0.wrapping_add(1);
                            temp_0 =
                                (*r).m_tables[0 as libc::c_int as
                                                  usize].m_tree[(!temp_0 as
                                                                     libc::c_ulong).wrapping_add(bit_buf
                                                                                                     >>
                                                                                                     fresh109
                                                                                                     &
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                    as usize]
                                    as libc::c_int;
                            if !(temp_0 < 0 as libc::c_int &&
                                     num_bits >=
                                         code_len_0.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
                               {
                                break ;
                            }
                        }
                        if temp_0 >= 0 as libc::c_int {
                            current_block = 2358954804400455004;
                            continue ;
                        } else {
                            current_block = 14852891895508065921;
                            break ;
                        }
                    }
                }
                2365107296380899847 => {
                    s_0 =
                        (bit_buf &
                             (((1 as libc::c_int) << num_extra) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint;
                    bit_buf >>= num_extra;
                    num_bits =
                        (num_bits as libc::c_uint).wrapping_sub(num_extra) as
                            mz_uint32 as mz_uint32;
                    s_0 =
                        (s_0 as
                             libc::c_uint).wrapping_add((*::std::mem::transmute::<&[u8; 4],
                                                                                  &[libc::c_char; 4]>(b"\x03\x03\x0b\x00"))[dist.wrapping_sub(16
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint)
                                                                                                                                as
                                                                                                                                usize]
                                                            as libc::c_uint)
                            as mz_uint as mz_uint;
                    memset((*r).m_len_codes.as_mut_ptr().offset(counter as
                                                                    isize) as
                               *mut libc::c_void,
                           if dist == 16 as libc::c_int as libc::c_uint {
                               (*r).m_len_codes[counter.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                    as usize] as libc::c_int
                           } else { 0 as libc::c_int }, s_0 as libc::c_ulong);
                    counter =
                        (counter as libc::c_uint).wrapping_add(s_0) as
                            mz_uint32 as mz_uint32;
                    current_block = 11366682102430008042;
                }
                1387597052038031995 => {
                    temp =
                        (*r).m_tables[2 as libc::c_int as
                                          usize].m_look_up[(bit_buf &
                                                                (TINFL_FAST_LOOKUP_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong)
                                                               as usize] as
                            libc::c_int;
                    if temp >= 0 as libc::c_int {
                        code_len = (temp >> 9 as libc::c_int) as mz_uint;
                        temp &= 511 as libc::c_int
                    } else {
                        code_len =
                            TINFL_FAST_LOOKUP_BITS as libc::c_int as mz_uint;
                        loop  {
                            let fresh106 = code_len;
                            code_len = code_len.wrapping_add(1);
                            temp =
                                (*r).m_tables[2 as libc::c_int as
                                                  usize].m_tree[(!temp as
                                                                     libc::c_ulong).wrapping_add(bit_buf
                                                                                                     >>
                                                                                                     fresh106
                                                                                                     &
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                    as usize]
                                    as libc::c_int;
                            if !(temp < 0 as libc::c_int) { break ; }
                        }
                    }
                    dist = temp as mz_uint32;
                    bit_buf >>= code_len;
                    num_bits =
                        (num_bits as libc::c_uint).wrapping_sub(code_len) as
                            mz_uint32 as mz_uint32;
                    if dist < 16 as libc::c_int as libc::c_uint {
                        let fresh107 = counter;
                        counter = counter.wrapping_add(1);
                        (*r).m_len_codes[fresh107 as usize] = dist as mz_uint8
                    } else {
                        if dist == 16 as libc::c_int as libc::c_uint &&
                               counter == 0 {
                            current_block = 11975512581081481720;
                            break ;
                        }
                        num_extra =
                            (*::std::mem::transmute::<&[u8; 4],
                                                      &[libc::c_char; 4]>(b"\x02\x03\x07\x00"))[dist.wrapping_sub(16
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint)
                                                                                                    as
                                                                                                    usize]
                                as mz_uint32;
                        if num_bits < num_extra {
                            current_block = 11426676793787244633;
                            continue ;
                        } else {
                            current_block = 2365107296380899847;
                            continue ;
                        }
                    }
                    current_block = 11366682102430008042;
                }
                14898553815918780345 => {
                    temp =
                        (*r).m_tables[2 as libc::c_int as
                                          usize].m_look_up[(bit_buf &
                                                                (TINFL_FAST_LOOKUP_SIZE
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong)
                                                               as usize] as
                            libc::c_int;
                    if temp >= 0 as libc::c_int {
                        code_len = (temp >> 9 as libc::c_int) as mz_uint;
                        if code_len != 0 && num_bits >= code_len {
                            current_block = 1387597052038031995;
                            continue ;
                        } else {
                            current_block = 14785121481331406365;
                            break ;
                        }
                    } else {
                        if !(num_bits >
                                 TINFL_FAST_LOOKUP_BITS as libc::c_int as
                                     libc::c_uint) {
                            current_block = 14785121481331406365;
                            break ;
                        }
                        code_len =
                            TINFL_FAST_LOOKUP_BITS as libc::c_int as mz_uint;
                        loop  {
                            let fresh104 = code_len;
                            code_len = code_len.wrapping_add(1);
                            temp =
                                (*r).m_tables[2 as libc::c_int as
                                                  usize].m_tree[(!temp as
                                                                     libc::c_ulong).wrapping_add(bit_buf
                                                                                                     >>
                                                                                                     fresh104
                                                                                                     &
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                    as usize]
                                    as libc::c_int;
                            if !(temp < 0 as libc::c_int &&
                                     num_bits >=
                                         code_len.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint))
                               {
                                break ;
                            }
                        }
                        if temp >= 0 as libc::c_int {
                            current_block = 1387597052038031995;
                            continue ;
                        } else {
                            current_block = 14785121481331406365;
                            break ;
                        }
                    }
                }
                13419727033115386574 => {
                    if (*r).m_type as libc::c_int >= 0 as libc::c_int {
                        tree_next = 0;
                        tree_cur = 0;
                        pTable = 0 as *mut tinfl_huff_table;
                        i_0 = 0;
                        j = 0;
                        used_syms = 0;
                        total = 0;
                        sym_index = 0;
                        next_code = [0; 17];
                        total_syms = [0; 16];
                        pTable =
                            &mut *(*r).m_tables.as_mut_ptr().offset((*r).m_type
                                                                        as
                                                                        isize)
                                as *mut tinfl_huff_table;
                        memset(&mut total_syms as *mut [mz_uint; 16] as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<[mz_uint; 16]>() as
                                   libc::c_ulong);
                        memset(&mut (*pTable).m_look_up as
                                   *mut [mz_int16; 1024] as *mut libc::c_void,
                               0 as libc::c_int,
                               ::std::mem::size_of::<[mz_int16; 1024]>() as
                                   libc::c_ulong);
                        memset(&mut (*pTable).m_tree as *mut [mz_int16; 576]
                                   as *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<[mz_int16; 576]>() as
                                   libc::c_ulong);
                        i_0 = 0 as libc::c_int as mz_uint;
                        while i_0 < (*r).m_table_sizes[(*r).m_type as usize] {
                            total_syms[(*pTable).m_code_size[i_0 as usize] as
                                           usize] =
                                total_syms[(*pTable).m_code_size[i_0 as usize]
                                               as usize].wrapping_add(1);
                            i_0 = i_0.wrapping_add(1)
                        }
                        used_syms = 0 as libc::c_int as mz_uint;
                        total = 0 as libc::c_int as mz_uint;
                        next_code[1 as libc::c_int as usize] =
                            0 as libc::c_int as mz_uint;
                        next_code[0 as libc::c_int as usize] =
                            next_code[1 as libc::c_int as usize];
                        i_0 = 1 as libc::c_int as mz_uint;
                        while i_0 <= 15 as libc::c_int as libc::c_uint {
                            used_syms =
                                (used_syms as
                                     libc::c_uint).wrapping_add(total_syms[i_0
                                                                               as
                                                                               usize])
                                    as mz_uint as mz_uint;
                            total =
                                total.wrapping_add(total_syms[i_0 as usize])
                                    << 1 as libc::c_int;
                            next_code[i_0.wrapping_add(1 as libc::c_int as
                                                           libc::c_uint) as
                                          usize] = total;
                            i_0 = i_0.wrapping_add(1)
                        }
                        if 65536 as libc::c_int as libc::c_uint != total &&
                               used_syms > 1 as libc::c_int as libc::c_uint {
                            current_block = 9914851455145855695;
                            break ;
                        }
                        tree_next = -(1 as libc::c_int);
                        sym_index = 0 as libc::c_int as mz_uint;
                        while sym_index <
                                  (*r).m_table_sizes[(*r).m_type as usize] {
                            let mut rev_code: mz_uint =
                                0 as libc::c_int as mz_uint;
                            let mut l: mz_uint = 0;
                            let mut cur_code: mz_uint = 0;
                            let mut code_size: mz_uint =
                                (*pTable).m_code_size[sym_index as usize] as
                                    mz_uint;
                            if !(code_size == 0) {
                                let fresh103 = next_code[code_size as usize];
                                next_code[code_size as usize] =
                                    next_code[code_size as
                                                  usize].wrapping_add(1);
                                cur_code = fresh103;
                                l = code_size;
                                while l > 0 as libc::c_int as libc::c_uint {
                                    rev_code =
                                        rev_code << 1 as libc::c_int |
                                            cur_code &
                                                1 as libc::c_int as
                                                    libc::c_uint;
                                    l = l.wrapping_sub(1);
                                    cur_code >>= 1 as libc::c_int
                                }
                                if code_size <=
                                       TINFL_FAST_LOOKUP_BITS as libc::c_int
                                           as libc::c_uint {
                                    let mut k: mz_int16 =
                                        (code_size << 9 as libc::c_int |
                                             sym_index) as mz_int16;
                                    while rev_code <
                                              TINFL_FAST_LOOKUP_SIZE as
                                                  libc::c_int as libc::c_uint
                                          {
                                        (*pTable).m_look_up[rev_code as usize]
                                            = k;
                                        rev_code =
                                            (rev_code as
                                                 libc::c_uint).wrapping_add(((1
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 code_size)
                                                                                as
                                                                                libc::c_uint)
                                                as mz_uint as mz_uint
                                    }
                                } else {
                                    tree_cur =
                                        (*pTable).m_look_up[(rev_code &
                                                                 (TINFL_FAST_LOOKUP_SIZE
                                                                      as
                                                                      libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_uint)
                                                                as usize] as
                                            libc::c_int;
                                    if 0 as libc::c_int == tree_cur {
                                        (*pTable).m_look_up[(rev_code &
                                                                 (TINFL_FAST_LOOKUP_SIZE
                                                                      as
                                                                      libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_uint)
                                                                as usize] =
                                            tree_next as mz_int16;
                                        tree_cur = tree_next;
                                        tree_next -= 2 as libc::c_int
                                    }
                                    rev_code >>=
                                        TINFL_FAST_LOOKUP_BITS as libc::c_int
                                            - 1 as libc::c_int;
                                    j = code_size;
                                    while j >
                                              (TINFL_FAST_LOOKUP_BITS as
                                                   libc::c_int +
                                                   1 as libc::c_int) as
                                                  libc::c_uint {
                                        rev_code >>= 1 as libc::c_int;
                                        tree_cur =
                                            (tree_cur as
                                                 libc::c_uint).wrapping_sub(rev_code
                                                                                &
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                as libc::c_int as libc::c_int;
                                        if (*pTable).m_tree[(-tree_cur -
                                                                 1 as
                                                                     libc::c_int)
                                                                as usize] == 0
                                           {
                                            (*pTable).m_tree[(-tree_cur -
                                                                  1 as
                                                                      libc::c_int)
                                                                 as usize] =
                                                tree_next as mz_int16;
                                            tree_cur = tree_next;
                                            tree_next -= 2 as libc::c_int
                                        } else {
                                            tree_cur =
                                                (*pTable).m_tree[(-tree_cur -
                                                                      1 as
                                                                          libc::c_int)
                                                                     as usize]
                                                    as libc::c_int
                                        }
                                        j = j.wrapping_sub(1)
                                    }
                                    rev_code >>= 1 as libc::c_int;
                                    tree_cur =
                                        (tree_cur as
                                             libc::c_uint).wrapping_sub(rev_code
                                                                            &
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                            as libc::c_int as libc::c_int;
                                    (*pTable).m_tree[(-tree_cur -
                                                          1 as libc::c_int) as
                                                         usize] =
                                        sym_index as mz_int16
                                }
                            }
                            sym_index = sym_index.wrapping_add(1)
                        }
                        if (*r).m_type == 2 as libc::c_int as libc::c_uint {
                            counter = 0 as libc::c_int as mz_uint32;
                            current_block = 11366682102430008042;
                        } else { current_block = 17836586256320518600; }
                    } else { current_block = 4396507302695188767; }
                }
                7591354809551635686 => {
                    s =
                        (bit_buf &
                             (((1 as libc::c_int) << 3 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint;
                    bit_buf >>= 3 as libc::c_int;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_sub(3 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    (*r).m_tables[2 as libc::c_int as
                                      usize].m_code_size[s_length_dezigzag[counter
                                                                               as
                                                                               usize]
                                                             as usize] =
                        s as mz_uint8;
                    counter = counter.wrapping_add(1);
                    current_block = 4533671380017093834;
                }
                11978944022089568707 => {
                    if counter < 3 as libc::c_int as libc::c_uint {
                        if num_bits <
                               (*::std::mem::transmute::<&[u8; 4],
                                                         &[libc::c_char; 4]>(b"\x05\x05\x04\x00"))[counter
                                                                                                       as
                                                                                                       usize]
                                   as mz_uint {
                            current_block = 15513414128719341844;
                            continue ;
                        } else {
                            current_block = 14723615986260991866;
                            continue ;
                        }
                    } else {
                        memset(&mut (*(*r).m_tables.as_mut_ptr().offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).m_code_size
                                   as *mut [mz_uint8; 288] as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<[mz_uint8; 288]>() as
                                   libc::c_ulong);
                        counter = 0 as libc::c_int as mz_uint32
                    }
                    current_block = 4533671380017093834;
                }
                14723615986260991866 => {
                    (*r).m_table_sizes[counter as usize] =
                        (bit_buf &
                             (((1 as libc::c_int) <<
                                   (*::std::mem::transmute::<&[u8; 4],
                                                             &[libc::c_char; 4]>(b"\x05\x05\x04\x00"))[counter
                                                                                                           as
                                                                                                           usize]
                                       as libc::c_int) - 1 as libc::c_int) as
                                 libc::c_ulong) as mz_uint32;
                    bit_buf >>=
                        (*::std::mem::transmute::<&[u8; 4],
                                                  &[libc::c_char; 4]>(b"\x05\x05\x04\x00"))[counter
                                                                                                as
                                                                                                usize]
                            as libc::c_int;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_sub((*::std::mem::transmute::<&[u8; 4],
                                                                                  &[libc::c_char; 4]>(b"\x05\x05\x04\x00"))[counter
                                                                                                                                as
                                                                                                                                usize]
                                                            as libc::c_uint)
                            as mz_uint32 as mz_uint32;
                    (*r).m_table_sizes[counter as usize] =
                        ((*r).m_table_sizes[counter as usize] as
                             libc::c_uint).wrapping_add(s_min_table_sizes[counter
                                                                              as
                                                                              usize]
                                                            as libc::c_uint)
                            as mz_uint32 as mz_uint32;
                    counter = counter.wrapping_add(1);
                    current_block = 11978944022089568707;
                    continue ;
                }
                2089914658669629659 => {
                    if counter != 0 {
                        n = 0;
                        current_block = 16580259026179177070;
                        break ;
                    } else { current_block = 12556861819962772176; }
                }
                10938659635288570931 => {
                    if !(counter != 0 && num_bits != 0) {
                        current_block = 2089914658669629659;
                        continue ;
                    }
                    if num_bits < 8 as libc::c_int as mz_uint {
                        current_block = 16937825661756021828;
                        continue ;
                    } else {
                        current_block = 10783567741412653655;
                        continue ;
                    }
                }
                13201766686570145889 => {
                    (*r).m_raw_header[counter as usize] =
                        (bit_buf &
                             (((1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint8;
                    bit_buf >>= 8 as libc::c_int;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    current_block = 17769492591016358583;
                    continue ;
                }
                178030534879405462 => {
                    if counter < 4 as libc::c_int as libc::c_uint {
                        if !(num_bits != 0) {
                            current_block = 8304106758420804164;
                            break ;
                        }
                        if num_bits < 8 as libc::c_int as mz_uint {
                            current_block = 15734707049249739970;
                            continue ;
                        } else {
                            current_block = 13201766686570145889;
                            continue ;
                        }
                    } else {
                        counter =
                            ((*r).m_raw_header[0 as libc::c_int as usize] as
                                 libc::c_int |
                                 ((*r).m_raw_header[1 as libc::c_int as usize]
                                      as libc::c_int) << 8 as libc::c_int) as
                                mz_uint32;
                        if counter !=
                               (0xffff as libc::c_int ^
                                    ((*r).m_raw_header[2 as libc::c_int as
                                                           usize] as
                                         libc::c_int |
                                         ((*r).m_raw_header[3 as libc::c_int
                                                                as usize] as
                                              libc::c_int) <<
                                             8 as libc::c_int)) as mz_uint {
                            current_block = 17336970397495664729;
                            break ;
                        } else {
                            current_block = 10938659635288570931;
                            continue ;
                        }
                    }
                }
                13238870711695007221 => {
                    (*r).m_z_adler32 =
                        (*r).m_z_adler32 << 8 as libc::c_int | s_1;
                    counter = counter.wrapping_add(1);
                    current_block = 13895028675981758206;
                }
                4847010591044017726 => {
                    c_12 = 0;
                    current_block = 10368141810877879434;
                    break ;
                }
                17582560547174494307 => {
                    c_11 = 0;
                    current_block = 8078296886873617068;
                    break ;
                }
                16251542583832361733 => {
                    let fresh120 = counter;
                    counter = counter.wrapping_sub(1);
                    if fresh120 != 0 {
                        current_block = 9997576841122142810;
                        break ;
                    }
                    current_block = 4396507302695188767;
                }
                12303096423297627778 => {
                    c_10 = 0;
                    current_block = 9945048139610397779;
                    break ;
                }
                16708736497050535312 => {
                    c_8 = 0;
                    current_block = 5330158124161540915;
                    break ;
                }
                11426676793787244633 => {
                    c_6 = 0;
                    current_block = 8577262330622874318;
                    break ;
                }
                622347125454405425 => {
                    c_4 = 0;
                    current_block = 5565703735569783978;
                    break ;
                }
                15513414128719341844 => {
                    c_3 = 0;
                    current_block = 8288085890650723895;
                    break ;
                }
                16937825661756021828 => {
                    c_2 = 0;
                    current_block = 9838996637140935403;
                    break ;
                }
                17769492591016358583 => {
                    counter = counter.wrapping_add(1);
                    current_block = 178030534879405462;
                    continue ;
                }
                _ => {
                    dist =
                        (bit_buf &
                             (((1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_ulong) as
                            mz_uint32;
                    bit_buf >>= 8 as libc::c_int;
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    current_block = 10784681114964964746;
                    break ;
                }
            }
            match current_block {
                13895028675981758206 => {
                    if !(counter < 4 as libc::c_int as libc::c_uint) {
                        current_block = 6415062399687569957;
                        break ;
                    }
                    s_1 = 0;
                    if !(num_bits != 0) {
                        current_block = 2644446111302234773;
                        break ;
                    }
                    if num_bits < 8 as libc::c_int as mz_uint {
                        current_block = 4847010591044017726;
                        continue ;
                    } else { current_block = 5335649609539064255; continue ; }
                }
                15826704491191897766 => {
                    dist_from_out_buf_start =
                        pOut_buf_cur.wrapping_offset_from(pOut_buf_start) as
                            libc::c_long as size_t;
                    if dist as libc::c_ulong > dist_from_out_buf_start &&
                           decomp_flags &
                               TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as
                                   libc::c_int as libc::c_uint != 0 {
                        current_block = 17083204083870333287;
                        break ;
                    }
                    pSrc =
                        pOut_buf_start.offset((dist_from_out_buf_start.wrapping_sub(dist
                                                                                        as
                                                                                        libc::c_ulong)
                                                   & out_buf_size_mask) as
                                                  isize);
                    if (if pOut_buf_cur > pSrc {
                            pOut_buf_cur
                        } else { pSrc }).offset(counter as isize) >
                           pOut_buf_end {
                        current_block = 16251542583832361733;
                        continue ;
                    }
                    if counter >= 9 as libc::c_int as libc::c_uint &&
                           counter <= dist {
                        let mut pSrc_end: *const mz_uint8 =
                            pSrc.offset((counter &
                                             !(7 as libc::c_int) as
                                                 libc::c_uint) as isize);
                        loop  {
                            memcpy(pOut_buf_cur as *mut libc::c_void,
                                   pSrc as *const libc::c_void,
                                   (::std::mem::size_of::<mz_uint32>() as
                                        libc::c_ulong).wrapping_mul(2 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong));
                            pOut_buf_cur =
                                pOut_buf_cur.offset(8 as libc::c_int as
                                                        isize);
                            pSrc = pSrc.offset(8 as libc::c_int as isize);
                            if !(pSrc < pSrc_end as *mut mz_uint8) { break ; }
                        }
                        counter &= 7 as libc::c_int as libc::c_uint;
                        if counter < 3 as libc::c_int as libc::c_uint {
                            if counter != 0 {
                                *pOut_buf_cur.offset(0 as libc::c_int as
                                                         isize) =
                                    *pSrc.offset(0 as libc::c_int as isize);
                                if counter > 1 as libc::c_int as libc::c_uint
                                   {
                                    *pOut_buf_cur.offset(1 as libc::c_int as
                                                             isize) =
                                        *pSrc.offset(1 as libc::c_int as
                                                         isize)
                                }
                                pOut_buf_cur =
                                    pOut_buf_cur.offset(counter as isize)
                            }
                            current_block = 4396507302695188767;
                        } else { current_block = 12350459713597596760; }
                    } else { current_block = 12350459713597596760; }
                    match current_block {
                        4396507302695188767 => { }
                        _ => {
                            while counter > 2 as libc::c_int as libc::c_uint {
                                *pOut_buf_cur.offset(0 as libc::c_int as
                                                         isize) =
                                    *pSrc.offset(0 as libc::c_int as isize);
                                *pOut_buf_cur.offset(1 as libc::c_int as
                                                         isize) =
                                    *pSrc.offset(1 as libc::c_int as isize);
                                *pOut_buf_cur.offset(2 as libc::c_int as
                                                         isize) =
                                    *pSrc.offset(2 as libc::c_int as isize);
                                pOut_buf_cur =
                                    pOut_buf_cur.offset(3 as libc::c_int as
                                                            isize);
                                pSrc = pSrc.offset(3 as libc::c_int as isize);
                                counter =
                                    (counter as
                                         libc::c_uint).wrapping_sub(3 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                        as mz_uint32 as mz_uint32
                            }
                            if counter > 0 as libc::c_int as libc::c_uint {
                                *pOut_buf_cur.offset(0 as libc::c_int as
                                                         isize) =
                                    *pSrc.offset(0 as libc::c_int as isize);
                                if counter > 1 as libc::c_int as libc::c_uint
                                   {
                                    *pOut_buf_cur.offset(1 as libc::c_int as
                                                             isize) =
                                        *pSrc.offset(1 as libc::c_int as
                                                         isize)
                                }
                                pOut_buf_cur =
                                    pOut_buf_cur.offset(counter as isize)
                            }
                            current_block = 4396507302695188767;
                        }
                    }
                }
                18421180869108055851 => {
                    counter &= 511 as libc::c_int as libc::c_uint;
                    if counter == 256 as libc::c_int as libc::c_uint {
                        current_block = 12556861819962772176;
                    } else {
                        num_extra =
                            s_length_extra[counter.wrapping_sub(257 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                               as usize] as mz_uint32;
                        counter =
                            s_length_base[counter.wrapping_sub(257 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                              as usize] as mz_uint32;
                        if num_extra != 0 {
                            extra_bits = 0;
                            if num_bits < num_extra {
                                current_block = 16708736497050535312;
                                continue ;
                            } else {
                                current_block = 12189673838652692395;
                                continue ;
                            }
                        } else { current_block = 15736258377632409300; }
                    }
                }
                11366682102430008042 => {
                    if counter <
                           (*r).m_table_sizes[0 as libc::c_int as
                                                  usize].wrapping_add((*r).m_table_sizes[1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize])
                       {
                        s_0 = 0;
                        temp = 0;
                        code_len = 0;
                        c_5 = 0;
                        if !(num_bits < 15 as libc::c_int as libc::c_uint) {
                            current_block = 1387597052038031995;
                            continue ;
                        }
                        if (pIn_buf_end.wrapping_offset_from(pIn_buf_cur) as
                                libc::c_long) <
                               2 as libc::c_int as libc::c_long {
                            current_block = 14898553815918780345;
                            continue ;
                        }
                        bit_buf |=
                            (*pIn_buf_cur.offset(0 as libc::c_int as isize) as
                                 tinfl_bit_buf_t) << num_bits |
                                (*pIn_buf_cur.offset(1 as libc::c_int as
                                                         isize) as
                                     tinfl_bit_buf_t) <<
                                    num_bits.wrapping_add(8 as libc::c_int as
                                                              libc::c_uint);
                        pIn_buf_cur =
                            pIn_buf_cur.offset(2 as libc::c_int as isize);
                        num_bits =
                            (num_bits as
                                 libc::c_uint).wrapping_add(16 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as mz_uint32 as mz_uint32;
                        current_block = 1387597052038031995;
                        continue ;
                    } else {
                        if (*r).m_table_sizes[0 as libc::c_int as
                                                  usize].wrapping_add((*r).m_table_sizes[1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize])
                               != counter {
                            current_block = 3178910365268327865;
                            break ;
                        }
                        memcpy((*r).m_tables[0 as libc::c_int as
                                                 usize].m_code_size.as_mut_ptr()
                                   as *mut libc::c_void,
                               (*r).m_len_codes.as_mut_ptr() as
                                   *const libc::c_void,
                               (*r).m_table_sizes[0 as libc::c_int as usize]
                                   as libc::c_ulong);
                        memcpy((*r).m_tables[1 as libc::c_int as
                                                 usize].m_code_size.as_mut_ptr()
                                   as *mut libc::c_void,
                               (*r).m_len_codes.as_mut_ptr().offset((*r).m_table_sizes[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                                                                        as
                                                                        isize)
                                   as *const libc::c_void,
                               (*r).m_table_sizes[1 as libc::c_int as usize]
                                   as libc::c_ulong);
                    }
                    current_block = 17836586256320518600;
                }
                4533671380017093834 => {
                    if counter < (*r).m_table_sizes[2 as libc::c_int as usize]
                       {
                        s = 0;
                        if num_bits < 3 as libc::c_int as mz_uint {
                            current_block = 622347125454405425;
                            continue ;
                        } else {
                            current_block = 7591354809551635686;
                            continue ;
                        }
                    } else {
                        (*r).m_table_sizes[2 as libc::c_int as usize] =
                            19 as libc::c_int as mz_uint32;
                        current_block = 13419727033115386574;
                        continue ;
                    }
                }
                _ => { }
            }
            match current_block {
                17836586256320518600 => {
                    (*r).m_type = (*r).m_type.wrapping_sub(1);
                    current_block = 13419727033115386574;
                }
                15736258377632409300 => {
                    temp_1 = 0;
                    code_len_2 = 0;
                    c_9 = 0;
                    if !(num_bits < 15 as libc::c_int as libc::c_uint) {
                        current_block = 13191700919457454999;
                        continue ;
                    }
                    if (pIn_buf_end.wrapping_offset_from(pIn_buf_cur) as
                            libc::c_long) < 2 as libc::c_int as libc::c_long {
                        current_block = 15235053155526978372;
                        continue ;
                    }
                    bit_buf |=
                        (*pIn_buf_cur.offset(0 as libc::c_int as isize) as
                             tinfl_bit_buf_t) << num_bits |
                            (*pIn_buf_cur.offset(1 as libc::c_int as isize) as
                                 tinfl_bit_buf_t) <<
                                num_bits.wrapping_add(8 as libc::c_int as
                                                          libc::c_uint);
                    pIn_buf_cur =
                        pIn_buf_cur.offset(2 as libc::c_int as isize);
                    num_bits =
                        (num_bits as
                             libc::c_uint).wrapping_add(16 as libc::c_int as
                                                            libc::c_uint) as
                            mz_uint32 as mz_uint32;
                    current_block = 13191700919457454999;
                }
                4396507302695188767 => {
                    pSrc = 0 as *mut mz_uint8;
                    current_block = 15980792246487107818;
                }
                _ => {
                    if (*r).m_final & 1 as libc::c_int as libc::c_uint == 0 {
                        current_block = 8151474771948790331;
                        break ;
                    }
                    if num_bits < num_bits & 7 as libc::c_int as libc::c_uint
                       {
                        current_block = 17582560547174494307;
                    } else { current_block = 15251772801061689047; }
                }
            }
        }
    }
    (*r).m_num_bits = num_bits;
    (*r).m_bit_buf =
        bit_buf &
            ((1 as libc::c_int as mz_uint64) <<
                 num_bits).wrapping_sub(1 as libc::c_int as mz_uint64);
    (*r).m_dist = dist;
    (*r).m_counter = counter;
    (*r).m_num_extra = num_extra;
    (*r).m_dist_from_out_buf_start = dist_from_out_buf_start;
    *pIn_buf_size =
        pIn_buf_cur.wrapping_offset_from(pIn_buf_next) as libc::c_long as
            size_t;
    *pOut_buf_size =
        pOut_buf_cur.wrapping_offset_from(pOut_buf_next) as libc::c_long as
            size_t;
    if decomp_flags &
           (TINFL_FLAG_PARSE_ZLIB_HEADER as libc::c_int |
                TINFL_FLAG_COMPUTE_ADLER32 as libc::c_int) as libc::c_uint !=
           0 && status as libc::c_int >= 0 as libc::c_int {
        let mut ptr: *const mz_uint8 = pOut_buf_next;
        let mut buf_len: size_t = *pOut_buf_size;
        let mut i_1: mz_uint32 = 0;
        let mut s1: mz_uint32 =
            (*r).m_check_adler32 & 0xffff as libc::c_int as libc::c_uint;
        let mut s2: mz_uint32 = (*r).m_check_adler32 >> 16 as libc::c_int;
        let mut block_len: size_t =
            buf_len.wrapping_rem(5552 as libc::c_int as libc::c_ulong);
        while buf_len != 0 {
            i_1 = 0 as libc::c_int as mz_uint32;
            while (i_1.wrapping_add(7 as libc::c_int as libc::c_uint) as
                       libc::c_ulong) < block_len {
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(5 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(6 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*ptr.offset(7 as
                                                                    libc::c_int
                                                                    as isize)
                                                        as libc::c_uint) as
                        mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                i_1 =
                    (i_1 as
                         libc::c_uint).wrapping_add(8 as libc::c_int as
                                                        libc::c_uint) as
                        mz_uint32 as mz_uint32;
                ptr = ptr.offset(8 as libc::c_int as isize)
            }
            while (i_1 as libc::c_ulong) < block_len {
                let fresh126 = ptr;
                ptr = ptr.offset(1);
                s1 =
                    (s1 as
                         libc::c_uint).wrapping_add(*fresh126 as libc::c_uint)
                        as mz_uint32 as mz_uint32;
                s2 =
                    (s2 as libc::c_uint).wrapping_add(s1) as mz_uint32 as
                        mz_uint32;
                i_1 = i_1.wrapping_add(1)
            }
            s1 =
                (s1 as libc::c_uint).wrapping_rem(65521 as libc::c_uint) as
                    mz_uint32 as mz_uint32;
            s2 =
                (s2 as libc::c_uint).wrapping_rem(65521 as libc::c_uint) as
                    mz_uint32 as mz_uint32;
            buf_len =
                (buf_len as libc::c_ulong).wrapping_sub(block_len) as size_t
                    as size_t;
            block_len = 5552 as libc::c_int as size_t
        }
        (*r).m_check_adler32 = (s2 << 16 as libc::c_int).wrapping_add(s1);
        if status as libc::c_int == TINFL_STATUS_DONE as libc::c_int &&
               decomp_flags &
                   TINFL_FLAG_PARSE_ZLIB_HEADER as libc::c_int as libc::c_uint
                   != 0 && (*r).m_check_adler32 != (*r).m_z_adler32 {
            status = TINFL_STATUS_ADLER32_MISMATCH
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress_mem_to_heap(mut pSrc_buf:
                                                          *const libc::c_void,
                                                      mut src_buf_len: size_t,
                                                      mut pOut_len:
                                                          *mut size_t,
                                                      mut flags: libc::c_int)
 -> *mut libc::c_void {
    let mut decomp: tinfl_decompressor =
        tinfl_decompressor{m_state: 0,
                           m_num_bits: 0,
                           m_zhdr0: 0,
                           m_zhdr1: 0,
                           m_z_adler32: 0,
                           m_final: 0,
                           m_type: 0,
                           m_check_adler32: 0,
                           m_dist: 0,
                           m_counter: 0,
                           m_num_extra: 0,
                           m_table_sizes: [0; 3],
                           m_bit_buf: 0,
                           m_dist_from_out_buf_start: 0,
                           m_tables:
                               [tinfl_huff_table{m_code_size: [0; 288],
                                                 m_look_up: [0; 1024],
                                                 m_tree: [0; 576],}; 3],
                           m_raw_header: [0; 4],
                           m_len_codes: [0; 457],};
    let mut pBuf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pNew_buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut src_buf_ofs: size_t = 0 as libc::c_int as size_t;
    let mut out_buf_capacity: size_t = 0 as libc::c_int as size_t;
    *pOut_len = 0 as libc::c_int as size_t;
    decomp.m_state = 0 as libc::c_int as mz_uint32;
    loop  {
        let mut src_buf_size: size_t = src_buf_len.wrapping_sub(src_buf_ofs);
        let mut dst_buf_size: size_t =
            out_buf_capacity.wrapping_sub(*pOut_len);
        let mut new_out_buf_capacity: size_t = 0;
        let mut status: tinfl_status =
            tinfl_decompress(&mut decomp,
                             (pSrc_buf as
                                  *const mz_uint8).offset(src_buf_ofs as
                                                              isize),
                             &mut src_buf_size, pBuf as *mut mz_uint8,
                             if !pBuf.is_null() {
                                 (pBuf as
                                      *mut mz_uint8).offset(*pOut_len as
                                                                isize)
                             } else { 0 as *mut mz_uint8 }, &mut dst_buf_size,
                             (flags &
                                  !(TINFL_FLAG_HAS_MORE_INPUT as libc::c_int)
                                  |
                                  TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as
                                      libc::c_int) as mz_uint32);
        if (status as libc::c_int) < 0 as libc::c_int ||
               status as libc::c_int ==
                   TINFL_STATUS_NEEDS_MORE_INPUT as libc::c_int {
            free(pBuf);
            *pOut_len = 0 as libc::c_int as size_t;
            return 0 as *mut libc::c_void
        }
        src_buf_ofs =
            (src_buf_ofs as libc::c_ulong).wrapping_add(src_buf_size) as
                size_t as size_t;
        *pOut_len =
            (*pOut_len as libc::c_ulong).wrapping_add(dst_buf_size) as size_t
                as size_t;
        if status as libc::c_int == TINFL_STATUS_DONE as libc::c_int {
            break ;
        }
        new_out_buf_capacity =
            out_buf_capacity.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        if new_out_buf_capacity < 128 as libc::c_int as libc::c_ulong {
            new_out_buf_capacity = 128 as libc::c_int as size_t
        }
        pNew_buf = realloc(pBuf, new_out_buf_capacity);
        if pNew_buf.is_null() {
            free(pBuf);
            *pOut_len = 0 as libc::c_int as size_t;
            return 0 as *mut libc::c_void
        }
        pBuf = pNew_buf;
        out_buf_capacity = new_out_buf_capacity
    }
    return pBuf;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress_mem_to_mem(mut pOut_buf:
                                                         *mut libc::c_void,
                                                     mut out_buf_len: size_t,
                                                     mut pSrc_buf:
                                                         *const libc::c_void,
                                                     mut src_buf_len: size_t,
                                                     mut flags: libc::c_int)
 -> size_t {
    let mut decomp: tinfl_decompressor =
        tinfl_decompressor{m_state: 0,
                           m_num_bits: 0,
                           m_zhdr0: 0,
                           m_zhdr1: 0,
                           m_z_adler32: 0,
                           m_final: 0,
                           m_type: 0,
                           m_check_adler32: 0,
                           m_dist: 0,
                           m_counter: 0,
                           m_num_extra: 0,
                           m_table_sizes: [0; 3],
                           m_bit_buf: 0,
                           m_dist_from_out_buf_start: 0,
                           m_tables:
                               [tinfl_huff_table{m_code_size: [0; 288],
                                                 m_look_up: [0; 1024],
                                                 m_tree: [0; 576],}; 3],
                           m_raw_header: [0; 4],
                           m_len_codes: [0; 457],};
    let mut status: tinfl_status = TINFL_STATUS_DONE;
    decomp.m_state = 0 as libc::c_int as mz_uint32;
    status =
        tinfl_decompress(&mut decomp, pSrc_buf as *const mz_uint8,
                         &mut src_buf_len, pOut_buf as *mut mz_uint8,
                         pOut_buf as *mut mz_uint8, &mut out_buf_len,
                         (flags & !(TINFL_FLAG_HAS_MORE_INPUT as libc::c_int)
                              |
                              TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as
                                  libc::c_int) as mz_uint32);
    return if status as libc::c_int != TINFL_STATUS_DONE as libc::c_int {
               -(1 as libc::c_int) as size_t
           } else { out_buf_len };
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress_mem_to_callback(mut pIn_buf:
                                                              *const libc::c_void,
                                                          mut pIn_buf_size:
                                                              *mut size_t,
                                                          mut pPut_buf_func:
                                                              tinfl_put_buf_func_ptr,
                                                          mut pPut_buf_user:
                                                              *mut libc::c_void,
                                                          mut flags:
                                                              libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut decomp: tinfl_decompressor =
        tinfl_decompressor{m_state: 0,
                           m_num_bits: 0,
                           m_zhdr0: 0,
                           m_zhdr1: 0,
                           m_z_adler32: 0,
                           m_final: 0,
                           m_type: 0,
                           m_check_adler32: 0,
                           m_dist: 0,
                           m_counter: 0,
                           m_num_extra: 0,
                           m_table_sizes: [0; 3],
                           m_bit_buf: 0,
                           m_dist_from_out_buf_start: 0,
                           m_tables:
                               [tinfl_huff_table{m_code_size: [0; 288],
                                                 m_look_up: [0; 1024],
                                                 m_tree: [0; 576],}; 3],
                           m_raw_header: [0; 4],
                           m_len_codes: [0; 457],};
    let mut pDict: *mut mz_uint8 =
        malloc(32768 as libc::c_int as libc::c_ulong) as *mut mz_uint8;
    let mut in_buf_ofs: size_t = 0 as libc::c_int as size_t;
    let mut dict_ofs: size_t = 0 as libc::c_int as size_t;
    if pDict.is_null() { return TINFL_STATUS_FAILED as libc::c_int }
    decomp.m_state = 0 as libc::c_int as mz_uint32;
    loop  {
        let mut in_buf_size: size_t =
            (*pIn_buf_size).wrapping_sub(in_buf_ofs);
        let mut dst_buf_size: size_t =
            (32768 as libc::c_int as libc::c_ulong).wrapping_sub(dict_ofs);
        let mut status: tinfl_status =
            tinfl_decompress(&mut decomp,
                             (pIn_buf as
                                  *const mz_uint8).offset(in_buf_ofs as
                                                              isize),
                             &mut in_buf_size, pDict,
                             pDict.offset(dict_ofs as isize),
                             &mut dst_buf_size,
                             (flags &
                                  !(TINFL_FLAG_HAS_MORE_INPUT as libc::c_int |
                                        TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF
                                            as libc::c_int)) as mz_uint32);
        in_buf_ofs =
            (in_buf_ofs as libc::c_ulong).wrapping_add(in_buf_size) as size_t
                as size_t;
        if dst_buf_size != 0 &&
               Some(pPut_buf_func.expect("non-null function pointer")).expect("non-null function pointer")(pDict.offset(dict_ofs
                                                                                                                            as
                                                                                                                            isize)
                                                                                                               as
                                                                                                               *const libc::c_void,
                                                                                                           dst_buf_size
                                                                                                               as
                                                                                                               libc::c_int,
                                                                                                           pPut_buf_user)
                   == 0 {
            break ;
        }
        if status as libc::c_int !=
               TINFL_STATUS_HAS_MORE_OUTPUT as libc::c_int {
            result =
                (status as libc::c_int == TINFL_STATUS_DONE as libc::c_int) as
                    libc::c_int;
            break ;
        } else {
            dict_ofs =
                dict_ofs.wrapping_add(dst_buf_size) &
                    (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        }
    }
    free(pDict as *mut libc::c_void);
    *pIn_buf_size = in_buf_ofs;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompressor_alloc()
 -> *mut tinfl_decompressor {
    let mut pDecomp: *mut tinfl_decompressor =
        malloc(::std::mem::size_of::<tinfl_decompressor>() as libc::c_ulong)
            as *mut tinfl_decompressor;
    if !pDecomp.is_null() {
        (*pDecomp).m_state = 0 as libc::c_int as mz_uint32
    }
    return pDecomp;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompressor_free(mut pDecomp:
                                                     *mut tinfl_decompressor) {
    free(pDecomp as *mut libc::c_void);
}
static mut fs_mempool: poolhandle_t = 0;
static mut fs_searchpaths: *mut searchpath_t =
    0 as *const searchpath_t as *mut searchpath_t;
// maxstrings changes as needed, causing reallocation of strings[] array
// offset of local file header
//original file size
// compressed file size
// common for all packed files
// chain
static mut fs_directpath: searchpath_t =
    searchpath_t{filename: [0; 256],
                 pack: 0 as *const pack_t as *mut pack_t,
                 wad: 0 as *const wfile_t as *mut wfile_t,
                 zip: 0 as *const zip_t as *mut zip_t,
                 flags: 0,
                 next: 0 as *const searchpath_s as *mut searchpath_s,};
// static direct path
static mut fs_basedir: [libc::c_char; 1024] = [0; 1024];
// base game directory
static mut fs_gamedir: [libc::c_char; 1024] = [0; 1024];
// game current directory
static mut fs_writedir: [libc::c_char; 1024] = [0; 1024];
// path that game allows to overwrite, delete and rename files (and create new of course)
static mut fs_ext_path: qboolean = false_0;
// attempt to read\write from ./ or ../ pathes
static mut fs_caseinsensitive: qboolean = true_0;
// try to search missing files
unsafe extern "C" fn FS_EnsureOpenFile(mut file: *mut file_t) { }
unsafe extern "C" fn FS_EnsureOpenZip(mut zip: *mut zip_t) { }
unsafe extern "C" fn FS_BackupFileName(mut file: *mut file_t,
                                       mut path: *const libc::c_char,
                                       mut options: uint) {
}
/*
=============================================================================

FILEMATCH COMMON SYSTEM

=============================================================================
*/
unsafe extern "C" fn stringlistinit(mut list: *mut stringlist_t) {
    memset(list as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<stringlist_t>() as
               libc::c_ulong); // ignore the virtual directories
}
unsafe extern "C" fn stringlistfreecontents(mut list: *mut stringlist_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*list).numstrings {
        if !(*(*list).strings.offset(i as isize)).is_null() {
            _Mem_Free(*(*list).strings.offset(i as isize) as
                          *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 256 as libc::c_int);
        }
        let ref mut fresh127 = *(*list).strings.offset(i as isize);
        *fresh127 = 0 as *mut libc::c_char;
        i += 1
    }
    if !(*list).strings.is_null() {
        _Mem_Free((*list).strings as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 261 as libc::c_int);
    }
    (*list).numstrings = 0 as libc::c_int;
    (*list).maxstrings = 0 as libc::c_int;
    (*list).strings = 0 as *mut *mut libc::c_char;
}
unsafe extern "C" fn stringlistappend(mut list: *mut stringlist_t,
                                      mut text: *mut libc::c_char) {
    let mut textlen: size_t = 0;
    if Q_strncmp(text, b".\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 ||
           Q_strncmp(text, b"..\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
        return
    }
    if (*list).numstrings >= (*list).maxstrings {
        (*list).maxstrings += 4096 as libc::c_int;
        (*list).strings =
            _Mem_Realloc(fs_mempool, (*list).strings as *mut libc::c_void,
                         ((*list).maxstrings as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                              as
                                                              libc::c_ulong),
                         true_0,
                         b"../engine/common/filesystem.c\x00" as *const u8 as
                             *const libc::c_char, 278 as libc::c_int) as
                *mut *mut libc::c_char
    }
    textlen = Q_strlen(text).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let ref mut fresh128 =
        *(*list).strings.offset((*list).numstrings as isize);
    *fresh128 =
        _Mem_Alloc(fs_mempool, textlen, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 282 as libc::c_int) as
            *mut libc::c_char;
    memcpy(*(*list).strings.offset((*list).numstrings as isize) as
               *mut libc::c_void, text as *const libc::c_void, textlen);
    (*list).numstrings += 1;
}
unsafe extern "C" fn stringlistsort(mut list: *mut stringlist_t) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // this is a selection sort (finds the best entry for each slot)
    i = 0 as libc::c_int;
    while i < (*list).numstrings - 1 as libc::c_int {
        j = i + 1 as libc::c_int;
        while j < (*list).numstrings {
            if Q_strncmp(*(*list).strings.offset(i as isize),
                         *(*list).strings.offset(j as isize),
                         99999 as libc::c_int) > 0 as libc::c_int {
                temp = *(*list).strings.offset(i as isize);
                let ref mut fresh129 = *(*list).strings.offset(i as isize);
                *fresh129 = *(*list).strings.offset(j as isize);
                let ref mut fresh130 = *(*list).strings.offset(j as isize);
                *fresh130 = temp
            }
            j += 1
        }
        i += 1
    };
}
unsafe extern "C" fn listdirectory(mut list: *mut stringlist_t,
                                   mut path: *const libc::c_char,
                                   mut lowercase: qboolean) {
    let mut i: libc::c_int = 0;
    let mut c: *mut libc::c_schar = 0 as *mut libc::c_schar;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    dir = opendir(path);
    if dir.is_null() { return }
    loop 
         // iterate through the directory
         {
        entry = readdir(dir);
        if entry.is_null() { break ; }
        stringlistappend(list, (*entry).d_name.as_mut_ptr());
    }
    closedir(dir);
}
/*
=============================================================================

OTHER PRIVATE FUNCTIONS

=============================================================================
*/
/*
==================
FS_FixFileCase

emulate WIN32 FS behaviour when opening local file
==================
*/
unsafe extern "C" fn FS_FixFileCase(mut path: *const libc::c_char)
 -> *const libc::c_char {
    // assume case insensitive
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut path2: [libc::c_char; 4096] = [0; 4096];
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_caseinsensitive as u64 == 0 { return path }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        Q_snprintf(path2.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong,
                   b"./%s\x00" as *const u8 as *const libc::c_char, path);
    } else {
        Q_strncpy(path2.as_mut_ptr(), path, 4096 as libc::c_int as size_t);
    }
    fname = Q_strrchr(path2.as_mut_ptr(), '/' as i32 as libc::c_char);
    if !fname.is_null() {
        let fresh131 = fname;
        fname = fname.offset(1);
        *fresh131 = 0 as libc::c_int as libc::c_char
    } else {
        fname = path as *mut libc::c_char;
        Q_strncpy(path2.as_mut_ptr(),
                  b".\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
    }
    /* android has too slow directory scanning,
	   so drop out some not useful cases */
    if fname.wrapping_offset_from(path2.as_mut_ptr()) as libc::c_long >
           4 as libc::c_int as libc::c_long {
        let mut point: *mut libc::c_char = 0 as *mut libc::c_char;
        // too many wad textures
        if Q_strnicmp(fname.offset(-(5 as libc::c_int as isize)),
                      b".wad\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            return path
        }
        point = Q_strchr(fname, '.' as i32 as libc::c_char);
        if !point.is_null() {
            if Q_strncmp(point,
                         b".mip\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 ||
                   Q_strncmp(point,
                             b".dds\x00" as *const u8 as *const libc::c_char,
                             99999 as libc::c_int) == 0 ||
                   Q_strncmp(point,
                             b".ent\x00" as *const u8 as *const libc::c_char,
                             99999 as libc::c_int) == 0 {
                return path
            }
            if *fname.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '{' as i32 {
                return path
            }
        }
    }
    //Con_Reportf( "FS_FixFileCase: %s\n", path );
    dir = opendir(path2.as_mut_ptr());
    if dir.is_null() { return path }
    loop  {
        entry = readdir(dir);
        if entry.is_null() { break ; }
        if Q_strnicmp((*entry).d_name.as_mut_ptr(), fname,
                      99999 as libc::c_int) != 0 {
            continue ;
        }
        path =
            va(b"%s/%s\x00" as *const u8 as *const libc::c_char,
               path2.as_mut_ptr(), (*entry).d_name.as_mut_ptr());
        break ;
    }
    closedir(dir);
    return path;
}
/*
====================
FS_AddFileToPack

Add a file to the list of files contained into a package
====================
*/
unsafe extern "C" fn FS_AddFileToPack(mut name: *const libc::c_char,
                                      mut pack: *mut pack_t,
                                      mut offset: fs_offset_t,
                                      mut size: fs_offset_t)
 -> *mut dpackfile_t {
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut middle: libc::c_int = 0;
    let mut pfile: *mut dpackfile_t = 0 as *mut dpackfile_t;
    // look for the slot we should put that file into (binary search)
    left = 0 as libc::c_int;
    right = (*pack).numfiles - 1 as libc::c_int;
    while left <= right {
        let mut diff: libc::c_int = 0;
        middle = (left + right) / 2 as libc::c_int;
        diff =
            Q_strnicmp((*(*pack).files.offset(middle as
                                                  isize)).name.as_mut_ptr(),
                       name, 99999 as libc::c_int);
        // If we found the file, there's a problem
        if diff == 0 {
            Con_Reportf(b"^3Warning:^7 package %s contains the file %s several times\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*pack).filename.as_mut_ptr(), name);
        }
        // If we're too far in the list
        if diff > 0 as libc::c_int {
            right = middle - 1 as libc::c_int
        } else { left = middle + 1 as libc::c_int }
    }
    // We have to move the right of the list by one slot to free the one we need
    pfile = &mut *(*pack).files.offset(left as isize) as *mut dpackfile_t;
    memmove(pfile.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            pfile as *const libc::c_void,
            (((*pack).numfiles - left) as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<dpackfile_t>()
                                                 as libc::c_ulong));
    (*pack).numfiles += 1;
    Q_strncpy((*pfile).name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong);
    (*pfile).filepos = offset as libc::c_int;
    (*pfile).filelen = size as libc::c_int;
    return pfile;
}
/*
============
FS_CreatePath

Only used for FS_Open.
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_CreatePath(mut path: *mut libc::c_char) {
    let mut ofs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    ofs = path.offset(1 as libc::c_int as isize);
    while *ofs != 0 {
        if *ofs as libc::c_int == '/' as i32 ||
               *ofs as libc::c_int == '\\' as i32 {
            // create the directory
            save = *ofs;
            *ofs = 0 as libc::c_int as libc::c_char;
            mkdir(path,
                  (0o400 as libc::c_int | 0o200 as libc::c_int |
                       0o100 as libc::c_int |
                       (0o400 as libc::c_int | 0o200 as libc::c_int |
                            0o100 as libc::c_int) >> 3 as libc::c_int |
                       0o400 as libc::c_int >> 3 as libc::c_int >>
                           3 as libc::c_int |
                       0o100 as libc::c_int >> 3 as libc::c_int >>
                           3 as libc::c_int) as __mode_t);
            *ofs = save
        }
        ofs = ofs.offset(1)
    };
}
/*
============
FS_Path_f

debug info
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Path_f() {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t;
    Con_Printf(b"Current search path:\n\x00" as *const u8 as
                   *const libc::c_char);
    s = fs_searchpaths;
    while !s.is_null() {
        if !(*s).pack.is_null() {
            Con_Printf(b"%s (%i files)\x00" as *const u8 as
                           *const libc::c_char,
                       (*(*s).pack).filename.as_mut_ptr(),
                       (*(*s).pack).numfiles);
        } else if !(*s).wad.is_null() {
            Con_Printf(b"%s (%i files)\x00" as *const u8 as
                           *const libc::c_char,
                       (*(*s).wad).filename.as_mut_ptr(),
                       (*(*s).wad).numlumps);
        } else if !(*s).zip.is_null() {
            Con_Printf(b"%s (%i files)\x00" as *const u8 as
                           *const libc::c_char,
                       (*(*s).zip).filename.as_mut_ptr(),
                       (*(*s).zip).numfiles);
        } else {
            Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       (*s).filename.as_mut_ptr());
        }
        if (*s).flags as libc::c_uint &
               (1 as libc::c_uint) << 4 as libc::c_int != 0 {
            Con_Printf(b" ^2rodir^7\x00" as *const u8 as *const libc::c_char);
        }
        if (*s).flags as libc::c_uint &
               (1 as libc::c_uint) << 2 as libc::c_int != 0 {
            Con_Printf(b" ^2gamedir^7\x00" as *const u8 as
                           *const libc::c_char);
        }
        if (*s).flags as libc::c_uint &
               (1 as libc::c_uint) << 3 as libc::c_int != 0 {
            Con_Printf(b" ^2custom^7\x00" as *const u8 as
                           *const libc::c_char);
        }
        if (*s).flags as libc::c_uint &
               (1 as libc::c_uint) << 1 as libc::c_int != 0 {
            Con_Printf(b" ^2nowrite^7\x00" as *const u8 as
                           *const libc::c_char);
        }
        if (*s).flags as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int != 0 {
            Con_Printf(b" ^2static^7\x00" as *const u8 as
                           *const libc::c_char);
        }
        Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        s = (*s).next
    };
}
/*
============
FS_ClearPath_f

only for debug targets
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_ClearPaths_f() { FS_ClearSearchPath(); }
/*
=================
FS_LoadPackPAK

Takes an explicit (not game tree related) path to a pak file.

Loads the header and directory, adding the files at the beginning
of the list so they override previous pack files.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_LoadPackPAK(mut packfile: *const libc::c_char,
                                        mut error: *mut libc::c_int)
 -> *mut pack_t {
    let mut header: dpackheader_t =
        dpackheader_t{ident: 0, dirofs: 0, dirlen: 0,};
    let mut packhandle: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut numpackfiles: libc::c_int = 0;
    let mut pack: *mut pack_t = 0 as *mut pack_t;
    let mut info: *mut dpackfile_t = 0 as *mut dpackfile_t;
    packhandle = open(packfile, 0 as libc::c_int | 0 as libc::c_int);
    if packhandle < 0 as libc::c_int {
        let mut fpackfile: *const libc::c_char = FS_FixFileCase(packfile);
        if fpackfile != packfile {
            packhandle = open(fpackfile, 0 as libc::c_int | 0 as libc::c_int)
        }
    }
    if packhandle < 0 as libc::c_int {
        Con_Reportf(b"%s couldn\'t open\n\x00" as *const u8 as
                        *const libc::c_char, packfile);
        if !error.is_null() { *error = 1 as libc::c_int }
        return 0 as *mut pack_t
    }
    read(packhandle, &mut header as *mut dpackheader_t as *mut libc::c_void,
         ::std::mem::size_of::<dpackheader_t>() as libc::c_ulong);
    if header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('C' as i32) << 16 as libc::c_int) +
               (('A' as i32) << 8 as libc::c_int) + 'P' as i32 {
        Con_Reportf(b"%s is not a packfile. Ignored.\n\x00" as *const u8 as
                        *const libc::c_char, packfile);
        if !error.is_null() { *error = 2 as libc::c_int }
        close(packhandle);
        return 0 as *mut pack_t
    }
    if (header.dirlen as
            libc::c_ulong).wrapping_rem(::std::mem::size_of::<dpackfile_t>()
                                            as libc::c_ulong) != 0 {
        Con_Reportf(b"^1Error:^7 %s has an invalid directory size. Ignored.\n\x00"
                        as *const u8 as *const libc::c_char, packfile);
        if !error.is_null() { *error = 3 as libc::c_int }
        close(packhandle);
        return 0 as *mut pack_t
    }
    numpackfiles =
        (header.dirlen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dpackfile_t>()
                                             as libc::c_ulong) as libc::c_int;
    if numpackfiles > 65536 as libc::c_int {
        Con_Reportf(b"^1Error:^7 %s has too many files ( %i ). Ignored.\n\x00"
                        as *const u8 as *const libc::c_char, packfile,
                    numpackfiles);
        if !error.is_null() { *error = 4 as libc::c_int }
        close(packhandle);
        return 0 as *mut pack_t
    }
    if numpackfiles <= 0 as libc::c_int {
        Con_Reportf(b"%s has no files. Ignored.\n\x00" as *const u8 as
                        *const libc::c_char, packfile);
        if !error.is_null() { *error = 5 as libc::c_int }
        close(packhandle);
        return 0 as *mut pack_t
    }
    info =
        _Mem_Alloc(fs_mempool,
                   (::std::mem::size_of::<dpackfile_t>() as
                        libc::c_ulong).wrapping_mul(numpackfiles as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 663 as libc::c_int) as
            *mut dpackfile_t;
    lseek(packhandle, header.dirofs as __off_t, 0 as libc::c_int);
    if header.dirlen as libc::c_long !=
           read(packhandle, info as *mut libc::c_void,
                header.dirlen as size_t) {
        Con_Reportf(b"%s is an incomplete PAK, not loading\n\x00" as *const u8
                        as *const libc::c_char, packfile);
        if !error.is_null() { *error = 6 as libc::c_int }
        close(packhandle);
        _Mem_Free(info as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 671 as libc::c_int);
        return 0 as *mut pack_t
    }
    pack =
        _Mem_Alloc(fs_mempool,
                   ::std::mem::size_of::<pack_t>() as libc::c_ulong, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 675 as libc::c_int) as
            *mut pack_t;
    Q_strncpy((*pack).filename.as_mut_ptr(), packfile,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    (*pack).files =
        _Mem_Alloc(fs_mempool,
                   (numpackfiles as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<dpackfile_t>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 677 as libc::c_int) as
            *mut dpackfile_t;
    (*pack).filetime = FS_SysFileTime(packfile) as time_t;
    (*pack).handle = packhandle;
    (*pack).numfiles = 0 as libc::c_int;
    // parse the directory
    i = 0 as libc::c_int;
    while i < numpackfiles {
        FS_AddFileToPack((*info.offset(i as isize)).name.as_mut_ptr(), pack,
                         (*info.offset(i as isize)).filepos as fs_offset_t,
                         (*info.offset(i as isize)).filelen as fs_offset_t);
        i += 1
    }
    if !error.is_null() { *error = 0 as libc::c_int }
    _Mem_Free(info as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 693 as libc::c_int);
    return pack;
}
/*
============
FS_SortZip
============
*/
unsafe extern "C" fn FS_SortZip(mut a: *const libc::c_void,
                                mut b: *const libc::c_void) -> libc::c_int {
    return Q_strnicmp((*(a as *mut zipfile_t)).name.as_mut_ptr(),
                      (*(b as *mut zipfile_t)).name.as_mut_ptr(),
                      99999 as libc::c_int);
}
/*
============
FS_LoadZip
============
*/
unsafe extern "C" fn FS_LoadZip(mut zipfile: *const libc::c_char,
                                mut error: *mut libc::c_int) -> *mut zip_t {
    let mut numpackfiles: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut header_cdf: zip_cdf_header_t =
        zip_cdf_header_t{signature: 0,
                         version: 0,
                         version_need: 0,
                         generalPurposeBitFlag: 0,
                         flags: 0,
                         modification_time: 0,
                         modification_date: 0,
                         mz_crc32: 0,
                         compressed_size: 0,
                         uncompressed_size: 0,
                         filename_len: 0,
                         extrafield_len: 0,
                         file_commentary_len: 0,
                         disk_start: 0,
                         internal_attr: 0,
                         external_attr: 0,
                         local_header_offset: 0,};
    let mut header_eocd: zip_header_eocd_t =
        zip_header_eocd_t{disk_number: 0,
                          start_disk_number: 0,
                          number_central_directory_record: 0,
                          total_central_directory_record: 0,
                          size_of_central_directory: 0,
                          central_directory_offset: 0,
                          commentary_len: 0,};
    let mut signature: uint = 0;
    let mut filepos: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    let mut length: fs_offset_t = 0;
    let mut info: *mut zipfile_t = 0 as *mut zipfile_t;
    let mut filename_buffer: [libc::c_char; 1024] = [0; 1024];
    let mut zip: *mut zip_t =
        _Mem_Alloc(fs_mempool,
                   ::std::mem::size_of::<zip_t>() as libc::c_ulong, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 722 as libc::c_int) as *mut zip_t;
    (*zip).handle = open(zipfile, 0 as libc::c_int | 0 as libc::c_int);
    if (*zip).handle < 0 as libc::c_int {
        let mut fzipfile: *const libc::c_char = FS_FixFileCase(zipfile);
        if fzipfile != zipfile {
            (*zip).handle =
                open(fzipfile, 0 as libc::c_int | 0 as libc::c_int)
        }
    }
    if (*zip).handle < 0 as libc::c_int {
        Con_Reportf(b"^1Error:^7 %s couldn\'t open\n\x00" as *const u8 as
                        *const libc::c_char, zipfile);
        if !error.is_null() { *error = 1 as libc::c_int }
        Zip_Close(zip);
        return 0 as *mut zip_t
    }
    length =
        lseek((*zip).handle, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    if length >
           (2147483647 as libc::c_int as
                libc::c_uint).wrapping_mul(2 as
                                               libc::c_uint).wrapping_add(1 as
                                                                              libc::c_uint)
               as libc::c_long {
        Con_Reportf(b"^1Error:^7 %s bigger than 4GB.\n\x00" as *const u8 as
                        *const libc::c_char, zipfile);
        if !error.is_null() { *error = 1 as libc::c_int }
        Zip_Close(zip);
        return 0 as *mut zip_t
    }
    lseek((*zip).handle, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    read((*zip).handle, &mut signature as *mut uint as *mut libc::c_void,
         ::std::mem::size_of::<uint>() as libc::c_ulong);
    if signature ==
           (((0x6 as libc::c_int) << 24 as libc::c_int) +
                ((0x5 as libc::c_int) << 16 as libc::c_int) +
                (('K' as i32) << 8 as libc::c_int) + 'P' as i32) as
               libc::c_uint {
        Con_Reportf(b"^3Warning:^7 %s has no files. Ignored.\n\x00" as
                        *const u8 as *const libc::c_char, zipfile);
        if !error.is_null() { *error = 5 as libc::c_int }
        Zip_Close(zip);
        return 0 as *mut zip_t
    }
    if signature !=
           ((('K' as i32) << 8 as libc::c_int) + 'P' as i32 +
                ((0x3 as libc::c_int) << 16 as libc::c_int) +
                ((0x4 as libc::c_int) << 24 as libc::c_int)) as libc::c_uint {
        Con_Reportf(b"^1Error:^7 %s is not a zip file. Ignored.\n\x00" as
                        *const u8 as *const libc::c_char, zipfile);
        if !error.is_null() { *error = 2 as libc::c_int }
        Zip_Close(zip);
        return 0 as *mut zip_t
    }
    // Find oecd
    lseek((*zip).handle, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    filepos = length;
    while filepos > 0 as libc::c_int as libc::c_long {
        lseek((*zip).handle, filepos, 0 as libc::c_int);
        read((*zip).handle, &mut signature as *mut uint as *mut libc::c_void,
             ::std::mem::size_of::<uint>() as libc::c_ulong);
        if signature ==
               (((0x6 as libc::c_int) << 24 as libc::c_int) +
                    ((0x5 as libc::c_int) << 16 as libc::c_int) +
                    (('K' as i32) << 8 as libc::c_int) + 'P' as i32) as
                   libc::c_uint {
            break ;
        }
        filepos =
            (filepos as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_char>()
                                                 as libc::c_ulong) as
                fs_offset_t as fs_offset_t
        // step back one byte
    }
    if (((0x6 as libc::c_int) << 24 as libc::c_int) +
            ((0x5 as libc::c_int) << 16 as libc::c_int) +
            (('K' as i32) << 8 as libc::c_int) + 'P' as i32) as libc::c_uint
           != signature {
        Con_Reportf(b"^1Error:^7 cannot find EOCD in %s. Zip file corrupted.\n\x00"
                        as *const u8 as *const libc::c_char, zipfile);
        if !error.is_null() { *error = 2 as libc::c_int }
        Zip_Close(zip);
        return 0 as *mut zip_t
    }
    read((*zip).handle,
         &mut header_eocd as *mut zip_header_eocd_t as *mut libc::c_void,
         ::std::mem::size_of::<zip_header_eocd_t>() as libc::c_ulong);
    // Move to CDF start
    lseek((*zip).handle, header_eocd.central_directory_offset as __off_t,
          0 as libc::c_int);
    // Calc count of files in archive
    info =
        _Mem_Alloc(fs_mempool,
                   (::std::mem::size_of::<zipfile_t>() as
                        libc::c_ulong).wrapping_mul(header_eocd.total_central_directory_record
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 817 as libc::c_int) as
            *mut zipfile_t;
    i = 0 as libc::c_int;
    while i < header_eocd.total_central_directory_record as libc::c_int {
        read((*zip).handle,
             &mut header_cdf as *mut zip_cdf_header_t as *mut libc::c_void,
             ::std::mem::size_of::<zip_cdf_header_t>() as libc::c_ulong);
        if header_cdf.signature !=
               (((0x2 as libc::c_int) << 24 as libc::c_int) +
                    ((0x1 as libc::c_int) << 16 as libc::c_int) +
                    (('K' as i32) << 8 as libc::c_int) + 'P' as i32) as
                   libc::c_uint {
            Con_Reportf(b"^1Error:^7 CDF signature mismatch in %s. Zip file corrupted.\n\x00"
                            as *const u8 as *const libc::c_char, zipfile);
            if !error.is_null() { *error = 2 as libc::c_int }
            _Mem_Free(info as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 830 as libc::c_int);
            Zip_Close(zip);
            return 0 as *mut zip_t
        }
        if header_cdf.uncompressed_size != 0 &&
               header_cdf.filename_len as libc::c_int != 0 &&
               (header_cdf.filename_len as libc::c_int) < 1024 as libc::c_int
           {
            memset(&mut filename_buffer as *mut [libc::c_char; 1024] as
                       *mut libc::c_void, '\u{0}' as i32,
                   1024 as libc::c_int as libc::c_ulong);
            read((*zip).handle,
                 &mut filename_buffer as *mut [libc::c_char; 1024] as
                     *mut libc::c_void, header_cdf.filename_len as size_t);
            Q_strncpy((*info.offset(numpackfiles as isize)).name.as_mut_ptr(),
                      filename_buffer.as_mut_ptr(),
                      1024 as libc::c_int as size_t);
            (*info.offset(numpackfiles as isize)).size =
                header_cdf.uncompressed_size as fs_offset_t;
            (*info.offset(numpackfiles as isize)).compressed_size =
                header_cdf.compressed_size as fs_offset_t;
            (*info.offset(numpackfiles as isize)).offset =
                header_cdf.local_header_offset as fs_offset_t;
            numpackfiles += 1
        } else {
            lseek((*zip).handle, header_cdf.filename_len as __off_t,
                  1 as libc::c_int);
        }
        if header_cdf.extrafield_len != 0 {
            lseek((*zip).handle, header_cdf.extrafield_len as __off_t,
                  1 as libc::c_int);
        }
        if header_cdf.file_commentary_len != 0 {
            lseek((*zip).handle, header_cdf.file_commentary_len as __off_t,
                  1 as libc::c_int);
        }
        i += 1
    }
    // recalculate offsets
    i = 0 as libc::c_int;
    while i < numpackfiles {
        let mut header: zip_header_t =
            zip_header_t{signature: 0,
                         version: 0,
                         flags: 0,
                         compression_flags: 0,
                         dos_date: 0,
                         mz_crc32: 0,
                         compressed_size: 0,
                         uncompressed_size: 0,
                         filename_len: 0,
                         extrafield_len: 0,};
        lseek((*zip).handle, (*info.offset(i as isize)).offset,
              0 as libc::c_int);
        read((*zip).handle,
             &mut header as *mut zip_header_t as *mut libc::c_void,
             ::std::mem::size_of::<zip_header_t>() as libc::c_ulong);
        (*info.offset(i as isize)).flags = header.compression_flags;
        (*info.offset(i as isize)).offset =
            (((*info.offset(i as isize)).offset +
                  header.filename_len as libc::c_long +
                  header.extrafield_len as libc::c_long) as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<zip_header_t>()
                                                 as libc::c_ulong) as
                fs_offset_t;
        i += 1
    }
    Q_strncpy((*zip).filename.as_mut_ptr(), zipfile,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    (*zip).filetime = FS_SysFileTime(zipfile) as time_t;
    (*zip).numfiles = numpackfiles;
    (*zip).files = info;
    qsort((*zip).files as *mut libc::c_void, (*zip).numfiles as size_t,
          ::std::mem::size_of::<zipfile_t>() as libc::c_ulong,
          Some(FS_SortZip as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    if !error.is_null() { *error = 0 as libc::c_int }
    return zip;
}
#[no_mangle]
pub unsafe extern "C" fn Zip_Close(mut zip: *mut zip_t) {
    if zip.is_null() { return }
    _Mem_Free((*zip).files as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 891 as libc::c_int);
    FS_EnsureOpenZip(0 as *mut zip_t);
    if (*zip).handle >= 0 as libc::c_int { close((*zip).handle); }
    _Mem_Free(zip as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 898 as libc::c_int);
}
unsafe extern "C" fn Zip_LoadFile(mut path: *const libc::c_char,
                                  mut sizeptr: *mut fs_offset_t,
                                  mut gamedironly: qboolean) -> *mut byte {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut index: libc::c_int = 0;
    let mut file: *mut zipfile_t = 0 as *mut zipfile_t;
    let mut compressed_buffer: *mut byte = 0 as *mut byte;
    let mut decompressed_buffer: *mut byte = 0 as *mut byte;
    let mut zlib_result: libc::c_int = 0 as libc::c_int;
    let mut test_crc: dword = 0;
    let mut final_crc: dword = 0;
    let mut decompress_stream: mz_stream =
        mz_stream{next_in: 0 as *const libc::c_uchar,
                  avail_in: 0,
                  total_in: 0,
                  next_out: 0 as *mut libc::c_uchar,
                  avail_out: 0,
                  total_out: 0,
                  msg: 0 as *mut libc::c_char,
                  state: 0 as *mut mz_internal_state,
                  zalloc: None,
                  zfree: None,
                  opaque: 0 as *mut libc::c_void,
                  data_type: 0,
                  adler: 0,
                  reserved: 0,};
    if !sizeptr.is_null() { *sizeptr = 0 as libc::c_int as fs_offset_t }
    search = FS_FindFile(path, &mut index, gamedironly);
    if search.is_null() || (*search).zip.is_null() { return 0 as *mut byte }
    file =
        &mut *(*(*search).zip).files.offset(index as isize) as *mut zipfile_t;
    FS_EnsureOpenZip((*search).zip);
    if lseek((*(*search).zip).handle, (*file).offset, 0 as libc::c_int) ==
           -(1 as libc::c_int) as libc::c_long {
        return 0 as *mut byte
    }
    /*if( read( search->zip->handle, &header, sizeof( header ) ) < 0 )
		return NULL;

	if( header.signature != ZIP_HEADER_LF )
	{
		Con_Reportf( S_ERROR "Zip_LoadFile: %s signature error\n", file->name );
		return NULL;
	}*/
    if (*file).flags as libc::c_int == 0 as libc::c_int {
        decompressed_buffer =
            _Mem_Alloc(fs_mempool,
                       ((*file).size + 1 as libc::c_int as libc::c_long) as
                           size_t, false_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 936 as libc::c_int) as
                *mut byte; // finaly free compressed buffer
        *decompressed_buffer.offset((*file).size as isize) =
            '\u{0}' as i32 as byte;
        read((*(*search).zip).handle,
             decompressed_buffer as *mut libc::c_void,
             (*file).size as size_t);
        if !sizeptr.is_null() { *sizeptr = (*file).size }
        FS_EnsureOpenZip(0 as *mut zip_t);
        return decompressed_buffer
    } else if (*file).flags as libc::c_int == 8 as libc::c_int {
        compressed_buffer =
            _Mem_Alloc(fs_mempool,
                       ((*file).compressed_size +
                            1 as libc::c_int as libc::c_long) as size_t,
                       false_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 960 as libc::c_int) as
                *mut byte;
        decompressed_buffer =
            _Mem_Alloc(fs_mempool,
                       ((*file).size + 1 as libc::c_int as libc::c_long) as
                           size_t, false_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 961 as libc::c_int) as
                *mut byte;
        *decompressed_buffer.offset((*file).size as isize) =
            '\u{0}' as i32 as byte;
        read((*(*search).zip).handle, compressed_buffer as *mut libc::c_void,
             (*file).compressed_size as size_t);
        memset(&mut decompress_stream as *mut mz_stream as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mz_stream>() as libc::c_ulong);
        decompress_stream.avail_in = (*file).compressed_size as libc::c_uint;
        decompress_stream.total_in = decompress_stream.avail_in as mz_ulong;
        decompress_stream.next_in = compressed_buffer as *mut Bytef;
        decompress_stream.avail_out = (*file).size as libc::c_uint;
        decompress_stream.total_out = decompress_stream.avail_out as mz_ulong;
        decompress_stream.next_out = decompressed_buffer as *mut Bytef;
        decompress_stream.zalloc = None;
        decompress_stream.zfree = None;
        decompress_stream.opaque = 0 as *mut libc::c_void;
        if mz_inflateInit2(&mut decompress_stream, -(15 as libc::c_int)) !=
               MZ_OK as libc::c_int {
            Con_Printf(b"^1Error:^7 Zip_LoadFile: inflateInit2 failed\n\x00"
                           as *const u8 as *const libc::c_char);
            _Mem_Free(compressed_buffer as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 980 as libc::c_int);
            _Mem_Free(decompressed_buffer as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 981 as libc::c_int);
            return 0 as *mut byte
        }
        zlib_result =
            mz_inflate(&mut decompress_stream, MZ_NO_FLUSH as libc::c_int);
        mz_inflateEnd(&mut decompress_stream);
        if zlib_result == MZ_OK as libc::c_int ||
               zlib_result == MZ_STREAM_END as libc::c_int {
            _Mem_Free(compressed_buffer as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 990 as libc::c_int);
            if !sizeptr.is_null() { *sizeptr = (*file).size }
            FS_EnsureOpenZip(0 as *mut zip_t);
            return decompressed_buffer
        } else {
            Con_Reportf(b"^1Error:^7 Zip_LoadFile: %s : error while file decompressing. Zlib return code %d.\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*file).name.as_mut_ptr(), zlib_result);
            _Mem_Free(compressed_buffer as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 1012 as libc::c_int);
            _Mem_Free(decompressed_buffer as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 1013 as libc::c_int);
            return 0 as *mut byte
        }
    } else {
        Con_Reportf(b"^1Error:^7 Zip_LoadFile: %s : file compressed with unknown algorithm.\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*file).name.as_mut_ptr());
        return 0 as *mut byte
    };
}
/*
====================
FS_AddWad_Fullpath
====================
*/
unsafe extern "C" fn FS_AddWad_Fullpath(mut wadfile: *const libc::c_char,
                                        mut already_loaded: *mut qboolean,
                                        mut flags: libc::c_int) -> qboolean {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut wad: *mut wfile_t = 0 as *mut wfile_t;
    let mut ext: *const libc::c_char = COM_FileExtension(wadfile);
    let mut errorcode: libc::c_int = 1 as libc::c_int;
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).wad.is_null() &&
               Q_strnicmp((*(*search).wad).filename.as_mut_ptr(), wadfile,
                          99999 as libc::c_int) == 0 {
            if !already_loaded.is_null() { *already_loaded = true_0 }
            return true_0
            // already loaded
        }
        search = (*search).next
    }
    if !already_loaded.is_null() { *already_loaded = false_0 }
    if Q_strnicmp(ext, b"wad\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        wad = W_Open(wadfile, &mut errorcode)
    }
    if !wad.is_null() {
        search =
            _Mem_Alloc(fs_mempool,
                       ::std::mem::size_of::<searchpath_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 1057 as libc::c_int) as
                *mut searchpath_t;
        (*search).wad = wad;
        (*search).next = fs_searchpaths;
        (*search).flags |= flags;
        fs_searchpaths = search;
        Con_Reportf(b"Adding wadfile: %s (%i files)\n\x00" as *const u8 as
                        *const libc::c_char, wadfile, (*wad).numlumps);
        return true_0
    } else {
        if errorcode != 5 as libc::c_int {
            Con_Reportf(b"^1Error:^7 FS_AddWad_Fullpath: unable to load wad \"%s\"\n\x00"
                            as *const u8 as *const libc::c_char, wadfile);
        }
        return false_0
    };
}
/*
================
FS_AddPak_Fullpath

Adds the given pack to the search path.
The pack type is autodetected by the file extension.

Returns true if the file was successfully added to the
search path or if it was already included.

If keep_plain_dirs is set, the pack will be added AFTER the first sequence of
plain directories.
================
*/
unsafe extern "C" fn FS_AddPak_Fullpath(mut pakfile: *const libc::c_char,
                                        mut already_loaded: *mut qboolean,
                                        mut flags: libc::c_int) -> qboolean {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut ext: *const libc::c_char = COM_FileExtension(pakfile);
    let mut i: libc::c_int = 0;
    let mut errorcode: libc::c_int = 1 as libc::c_int;
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).pack.is_null() &&
               Q_strnicmp((*(*search).pack).filename.as_mut_ptr(), pakfile,
                          99999 as libc::c_int) == 0 {
            if !already_loaded.is_null() { *already_loaded = true_0 }
            return true_0
            // already loaded
        }
        search = (*search).next
    }
    if !already_loaded.is_null() { *already_loaded = false_0 }
    if Q_strnicmp(ext, b"pak\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        pak = FS_LoadPackPAK(pakfile, &mut errorcode)
    }
    if !pak.is_null() {
        let mut fullpath: string = [0; 256];
        search =
            _Mem_Alloc(fs_mempool,
                       ::std::mem::size_of::<searchpath_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 1114 as libc::c_int) as
                *mut searchpath_t;
        (*search).pack = pak;
        (*search).next = fs_searchpaths;
        (*search).flags |= flags;
        fs_searchpaths = search;
        Con_Reportf(b"Adding pakfile: %s (%i files)\n\x00" as *const u8 as
                        *const libc::c_char, pakfile, (*pak).numfiles);
        // time to add in search list all the wads that contains in current pakfile (if do)
        i = 0 as libc::c_int;
        while i < (*pak).numfiles {
            if Q_strnicmp(COM_FileExtension((*(*pak).files.offset(i as
                                                                      isize)).name.as_mut_ptr()),
                          b"wad\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                Q_snprintf(fullpath.as_mut_ptr(),
                           256 as libc::c_int as size_t,
                           b"%s/%s\x00" as *const u8 as *const libc::c_char,
                           pakfile,
                           (*(*pak).files.offset(i as
                                                     isize)).name.as_mut_ptr());
                FS_AddWad_Fullpath(fullpath.as_mut_ptr(), 0 as *mut qboolean,
                                   flags);
            }
            i += 1
        }
        return true_0
    } else {
        if errorcode != 5 as libc::c_int {
            Con_Reportf(b"^1Error:^7 FS_AddPak_Fullpath: unable to load pak \"%s\"\n\x00"
                            as *const u8 as *const libc::c_char, pakfile);
        }
        return false_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn FS_AddZip_Fullpath(mut zipfile: *const libc::c_char,
                                            mut already_loaded: *mut qboolean,
                                            mut flags: libc::c_int)
 -> qboolean {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut zip: *mut zip_t = 0 as *mut zip_t;
    let mut ext: *const libc::c_char = COM_FileExtension(zipfile);
    let mut errorcode: libc::c_int = 1 as libc::c_int;
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).pack.is_null() &&
               Q_strnicmp((*(*search).pack).filename.as_mut_ptr(), zipfile,
                          99999 as libc::c_int) == 0 {
            if !already_loaded.is_null() { *already_loaded = true_0 }
            return true_0
            // already loaded
        }
        search = (*search).next
    }
    if !already_loaded.is_null() { *already_loaded = false_0 }
    if Q_strnicmp(ext, b"pk3\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        zip = FS_LoadZip(zipfile, &mut errorcode)
    }
    if !zip.is_null() {
        let mut fullpath: string = [0; 256];
        let mut i: libc::c_int = 0;
        search =
            _Mem_Alloc(fs_mempool,
                       ::std::mem::size_of::<searchpath_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 1168 as libc::c_int) as
                *mut searchpath_t;
        (*search).zip = zip;
        (*search).next = fs_searchpaths;
        (*search).flags |= flags;
        fs_searchpaths = search;
        Con_Reportf(b"Adding zipfile: %s (%i files)\n\x00" as *const u8 as
                        *const libc::c_char, zipfile, (*zip).numfiles);
        // time to add in search list all the wads that contains in current pakfile (if do)
        i = 0 as libc::c_int;
        while i < (*zip).numfiles {
            if Q_strnicmp(COM_FileExtension((*(*zip).files.offset(i as
                                                                      isize)).name.as_mut_ptr()),
                          b"wad\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                Q_snprintf(fullpath.as_mut_ptr(),
                           256 as libc::c_int as size_t,
                           b"%s/%s\x00" as *const u8 as *const libc::c_char,
                           zipfile,
                           (*(*zip).files.offset(i as
                                                     isize)).name.as_mut_ptr());
                FS_AddWad_Fullpath(fullpath.as_mut_ptr(), 0 as *mut qboolean,
                                   flags);
            }
            i += 1
        }
        return true_0
    } else {
        if errorcode != 5 as libc::c_int {
            Con_Reportf(b"^1Error:^7 FS_AddZip_Fullpath: unable to load zip \"%s\"\n\x00"
                            as *const u8 as *const libc::c_char, zipfile);
        }
        return false_0
    };
}
/*
================
FS_AddGameDirectory

Sets fs_writedir, adds the directory to the head of the path,
then loads and adds pak1.pak pak2.pak ...
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_AddGameDirectory(mut dir: *const libc::c_char,
                                             mut flags: uint) {
    let mut list: stringlist_t =
        stringlist_t{maxstrings: 0,
                     numstrings: 0,
                     strings: 0 as *mut *mut libc::c_char,};
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut fullpath: string = [0; 256];
    let mut i: libc::c_int = 0;
    if flags & (1 as libc::c_uint) << 1 as libc::c_int == 0 {
        Q_strncpy(fs_writedir.as_mut_ptr(), dir,
                  ::std::mem::size_of::<[libc::c_char; 1024]>() as
                      libc::c_ulong);
    }
    stringlistinit(&mut list);
    listdirectory(&mut list, dir, false_0);
    stringlistsort(&mut list);
    // add any PAK package in the directory
    i = 0 as libc::c_int;
    while i < list.numstrings {
        if Q_strnicmp(COM_FileExtension(*list.strings.offset(i as isize)),
                      b"pak\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            Q_sprintf(fullpath.as_mut_ptr(),
                      b"%s%s\x00" as *const u8 as *const libc::c_char, dir,
                      *list.strings.offset(i as isize));
            FS_AddPak_Fullpath(fullpath.as_mut_ptr(), 0 as *mut qboolean,
                               flags as libc::c_int);
        }
        i += 1
    }
    // add any Zip package in the directory
    i = 0 as libc::c_int;
    while i < list.numstrings {
        if Q_strnicmp(COM_FileExtension(*list.strings.offset(i as isize)),
                      b"zip\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 ||
               Q_strnicmp(COM_FileExtension(*list.strings.offset(i as isize)),
                          b"pk3\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
            Q_sprintf(fullpath.as_mut_ptr(),
                      b"%s%s\x00" as *const u8 as *const libc::c_char, dir,
                      *list.strings.offset(i as isize));
            FS_AddZip_Fullpath(fullpath.as_mut_ptr(), 0 as *mut qboolean,
                               flags as libc::c_int);
        }
        i += 1
    }
    FS_AllowDirectPaths(true_0);
    // add any WAD package in the directory
    i = 0 as libc::c_int;
    while i < list.numstrings {
        if Q_strnicmp(COM_FileExtension(*list.strings.offset(i as isize)),
                      b"wad\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            Q_sprintf(fullpath.as_mut_ptr(),
                      b"%s%s\x00" as *const u8 as *const libc::c_char, dir,
                      *list.strings.offset(i as isize));
            FS_AddWad_Fullpath(fullpath.as_mut_ptr(), 0 as *mut qboolean,
                               flags as libc::c_int);
        }
        i += 1
    }
    stringlistfreecontents(&mut list);
    FS_AllowDirectPaths(false_0);
    // add the directory to the search path
	// (unpacked files have the priority over packed files)
    search =
        _Mem_Alloc(fs_mempool,
                   ::std::mem::size_of::<searchpath_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 1253 as libc::c_int) as
            *mut searchpath_t;
    Q_strncpy((*search).filename.as_mut_ptr(), dir,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    (*search).next = fs_searchpaths;
    (*search).flags = flags as libc::c_int;
    fs_searchpaths = search;
}
/*
================
FS_AddGameHierarchy
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_AddGameHierarchy(mut dir: *const libc::c_char,
                                             mut flags: uint) {
    let mut i: libc::c_int = 0;
    let mut isGameDir: qboolean =
        (flags & (1 as libc::c_uint) << 2 as libc::c_int) as qboolean;
    (*SI.GameInfo).added = true_0;
    if if dir.is_null() || *dir == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    // add the common game directory
    // recursive gamedirs
	// for example, czeror->czero->cstrike->valve
    i = 0 as libc::c_int;
    while i < SI.numgames {
        if Q_strnicmp((*SI.games[i as usize]).gamefolder.as_mut_ptr(), dir,
                      64 as libc::c_int) == 0 {
            Con_Reportf(b"FS_AddGameHierarchy: %d %s %s\n\x00" as *const u8 as
                            *const libc::c_char, i,
                        (*SI.games[i as usize]).gamefolder.as_mut_ptr(),
                        (*SI.games[i as usize]).basedir.as_mut_ptr());
            if (*SI.games[i as usize]).added as u64 == 0 &&
                   Q_strnicmp((*SI.games[i as usize]).gamefolder.as_mut_ptr(),
                              (*SI.games[i as usize]).basedir.as_mut_ptr(),
                              99999 as libc::c_int) != 0 {
                (*SI.games[i as usize]).added = true_0;
                FS_AddGameHierarchy((*SI.games[i as
                                                   usize]).basedir.as_mut_ptr(),
                                    flags &
                                        !((1 as libc::c_uint) <<
                                              2 as libc::c_int));
            }
            break ;
        } else { i += 1 }
    }
    if if *host.rodir.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        // append new flags to rodir, except FS_GAMEDIR_PATH and FS_CUSTOM_PATH
        let mut newFlags: uint =
            (1 as libc::c_uint) << 1 as libc::c_int |
                flags &
                    (!((1 as libc::c_uint) << 2 as libc::c_int) |
                         (1 as libc::c_uint) << 3 as libc::c_int);
        if isGameDir as u64 != 0 {
            newFlags |= (1 as libc::c_uint) << 4 as libc::c_int
        }
        FS_AllowDirectPaths(true_0);
        FS_AddGameDirectory(va(b"%s/%s/\x00" as *const u8 as
                                   *const libc::c_char,
                               host.rodir.as_mut_ptr(), dir), newFlags);
        FS_AllowDirectPaths(false_0);
    }
    if isGameDir as u64 != 0 {
        FS_AddGameDirectory(va(b"%s/downloaded/\x00" as *const u8 as
                                   *const libc::c_char, dir),
                            (1 as libc::c_uint) << 1 as libc::c_int |
                                (1 as libc::c_uint) << 3 as libc::c_int);
    }
    FS_AddGameDirectory(va(b"%s/\x00" as *const u8 as *const libc::c_char,
                           dir), flags);
    if isGameDir as u64 != 0 {
        FS_AddGameDirectory(va(b"%s/custom/\x00" as *const u8 as
                                   *const libc::c_char, dir),
                            (1 as libc::c_uint) << 1 as libc::c_int |
                                (1 as libc::c_uint) << 3 as libc::c_int);
    };
}
/*
================
FS_ClearSearchPath
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_ClearSearchPath() {
    while !fs_searchpaths.is_null() {
        let mut search: *mut searchpath_t = fs_searchpaths;
        if search.is_null() { break ; }
        if (*search).flags as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int != 0 {
            // skip read-only pathes
            if (*search).next.is_null() { break ; }
            fs_searchpaths = (*(*search).next).next
        } else { fs_searchpaths = (*search).next }
        if !(*search).pack.is_null() {
            if !(*(*search).pack).files.is_null() {
                _Mem_Free((*(*search).pack).files as *mut libc::c_void,
                          b"../engine/common/filesystem.c\x00" as *const u8 as
                              *const libc::c_char, 1337 as libc::c_int);
            }
            if (*(*search).pack).handle >= 0 as libc::c_int {
                close((*(*search).pack).handle);
            }
            _Mem_Free((*search).pack as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 1340 as libc::c_int);
        }
        if !(*search).wad.is_null() { W_Close((*search).wad); }
        if !(*search).zip.is_null() { Zip_Close((*search).zip); }
        _Mem_Free(search as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 1353 as libc::c_int);
    };
}
/*
====================
FS_CheckNastyPath

Return true if the path should be rejected due to one of the following:
1: path elements that are non-portable
2: path elements that would allow access to files outside the game directory,
   or are just not a good idea for a mod to be using.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_CheckNastyPath(mut path: *const libc::c_char,
                                           mut isgamedir: qboolean)
 -> libc::c_int {
    // all: never allow an empty path, as for gamedir it would access the parent directory and a non-gamedir path it is just useless
    if if path.is_null() || *path == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 2 as libc::c_int
    } // allow any path
    if fs_ext_path as u64 != 0 { return 0 as libc::c_int }
    // Mac: don't allow Mac-only filenames - : is a directory separator
	// instead of /, but we rely on / working already, so there's no reason to
	// support a Mac-only path
	// Amiga and Windows: : tries to go to root of drive
    if !Q_strchr(path, ':' as i32 as libc::c_char).is_null() {
        return 1 as libc::c_int
    } // non-portable attempt to go to root of drive
    // Amiga: // is parent directory
    if !Q_strstr(path,
                 b"//\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int
    } // non-portable attempt to go to parent directory
    // all: don't allow going to parent directory (../ or /../)
    if !Q_strstr(path,
                 b"..\x00" as *const u8 as *const libc::c_char).is_null() {
        return 2 as libc::c_int
    } // attempt to go outside the game directory
    // Windows and UNIXes: don't allow absolute paths
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return 2 as libc::c_int
    } // attempt to go outside the game directory
    // all: forbid trailing slash on gamedir
    if isgamedir as libc::c_uint != 0 &&
           *path.offset(Q_strlen(path).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) as
                            isize) as libc::c_int == '/' as i32 {
        return 2 as libc::c_int
    }
    // all: forbid leading dot on any filename for any reason
    if !Q_strstr(path,
                 b"/.\x00" as *const u8 as *const libc::c_char).is_null() {
        return 2 as libc::c_int
    } // attempt to go outside the game directory
    // after all these checks we're pretty sure it's a / separated filename
	// and won't do much if any harm
    return 0 as libc::c_int;
}
/*
================
FS_Rescan
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Rescan() {
    let mut str: *const libc::c_char =
        0 as *const libc::c_char; // e.g. valve.rc
    let extrasFlags: libc::c_int =
        ((1 as libc::c_uint) << 1 as libc::c_int |
             (1 as libc::c_uint) << 3 as libc::c_int) as libc::c_int;
    Con_Reportf(b"FS_Rescan( %s )\n\x00" as *const u8 as *const libc::c_char,
                (*SI.GameInfo).title.as_mut_ptr());
    FS_ClearSearchPath();
    str =
        getenv(b"XASH3D_EXTRAS_PAK1\x00" as *const u8 as *const libc::c_char);
    if if str.is_null() || *str == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_AddPak_Fullpath(str, 0 as *mut qboolean, extrasFlags);
    }
    str =
        getenv(b"XASH3D_EXTRAS_PAK2\x00" as *const u8 as *const libc::c_char);
    if if str.is_null() || *str == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_AddPak_Fullpath(str, 0 as *mut qboolean, extrasFlags);
    }
    if Q_strnicmp((*SI.GameInfo).basedir.as_mut_ptr(),
                  (*SI.GameInfo).gamefolder.as_mut_ptr(),
                  99999 as libc::c_int) != 0 {
        FS_AddGameHierarchy((*SI.GameInfo).basedir.as_mut_ptr(),
                            0 as libc::c_int as uint);
    }
    if Q_strnicmp((*SI.GameInfo).basedir.as_mut_ptr(),
                  (*SI.GameInfo).falldir.as_mut_ptr(), 99999 as libc::c_int)
           != 0 &&
           Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                      (*SI.GameInfo).falldir.as_mut_ptr(),
                      99999 as libc::c_int) != 0 {
        FS_AddGameHierarchy((*SI.GameInfo).falldir.as_mut_ptr(),
                            0 as libc::c_int as uint);
    }
    FS_AddGameHierarchy((*SI.GameInfo).gamefolder.as_mut_ptr(),
                        (1 as libc::c_uint) << 2 as libc::c_int);
    if FS_FileExists(va(b"%s.rc\x00" as *const u8 as *const libc::c_char,
                        SI.basedirName.as_mut_ptr()), false_0 as libc::c_int)
           != 0 {
        Q_strncpy(SI.rcName.as_mut_ptr(), SI.basedirName.as_mut_ptr(),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    } else {
        Q_strncpy(SI.rcName.as_mut_ptr(), SI.exeName.as_mut_ptr(),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    };
    // e.g. quake.rc
}
/*
================
FS_Rescan_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Rescan_f() { FS_Rescan(); }
/*
================
FS_WriteGameInfo

assume GameInfo is valid
================
*/
unsafe extern "C" fn FS_WriteGameInfo(mut filepath: *const libc::c_char,
                                      mut GameInfo: *mut gameinfo_t) {
    let mut f: *mut file_t =
        FS_Open(filepath, b"w\x00" as *const u8 as *const libc::c_char,
                false_0); // we in binary-mode
    let mut i: libc::c_int = 0; // may be disk-space is out?
    let mut write_ambients: libc::c_int = false_0 as libc::c_int;
    if f.is_null() {
        Sys_Error(b"FS_WriteGameInfo: can\'t write %s\n\x00" as *const u8 as
                      *const libc::c_char, filepath);
    }
    FS_Printf(f,
              b"// generated by %s %s-%s (%s-%s)\n\n\n\x00" as *const u8 as
                  *const libc::c_char,
              b"Xash3D FWGS\x00" as *const u8 as *const libc::c_char,
              b"0.20\x00" as *const u8 as *const libc::c_char,
              Q_buildcommit(), Q_buildos(), Q_buildarch());
    if if *(*GameInfo).basedir.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"basedir\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).basedir.as_mut_ptr());
    }
    // DEPRECATED: gamedir key isn't supported by FWGS fork
	// but write it anyway to keep compability with original Xash3D
    if if *(*GameInfo).gamefolder.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"gamedir\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).gamefolder.as_mut_ptr());
    }
    if if *(*GameInfo).falldir.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"fallback_dir\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).falldir.as_mut_ptr());
    }
    if if *(*GameInfo).title.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"title\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).title.as_mut_ptr());
    }
    if if *(*GameInfo).startmap.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"startmap\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).startmap.as_mut_ptr());
    }
    if if *(*GameInfo).trainmap.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"trainmap\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).trainmap.as_mut_ptr());
    }
    if (*GameInfo).version != 0.0f32 {
        FS_Printf(f,
                  b"version\t\t%g\n\x00" as *const u8 as *const libc::c_char,
                  (*GameInfo).version as libc::c_double);
    }
    if (*GameInfo).size != 0 as libc::c_int as libc::c_ulong {
        FS_Printf(f, b"size\t\t%lu\n\x00" as *const u8 as *const libc::c_char,
                  (*GameInfo).size);
    }
    if if *(*GameInfo).game_url.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"url_info\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).game_url.as_mut_ptr());
    }
    if if *(*GameInfo).update_url.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"url_update\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).update_url.as_mut_ptr());
    }
    if if *(*GameInfo).type_0.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"type\t\t\"%s\"\n\x00" as *const u8 as *const libc::c_char,
                  (*GameInfo).type_0.as_mut_ptr());
    }
    if if *(*GameInfo).date.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"date\t\t\"%s\"\n\x00" as *const u8 as *const libc::c_char,
                  (*GameInfo).date.as_mut_ptr());
    }
    if if *(*GameInfo).dll_path.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"dllpath\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).dll_path.as_mut_ptr());
    }
    if if *(*GameInfo).game_dll.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"gamedll\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).game_dll.as_mut_ptr());
    }
    if if *(*GameInfo).game_dll_linux.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"gamedll_linux\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).game_dll_linux.as_mut_ptr());
    }
    if if *(*GameInfo).game_dll_osx.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"gamedll_osx\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).game_dll_osx.as_mut_ptr());
    }
    if if *(*GameInfo).iconpath.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"icon\t\t\"%s\"\n\x00" as *const u8 as *const libc::c_char,
                  (*GameInfo).iconpath.as_mut_ptr());
    }
    match (*GameInfo).gamemode {
        1 => {
            FS_Print(f,
                     b"gamemode\t\t\"singleplayer_only\"\n\x00" as *const u8
                         as *const libc::c_char);
        }
        2 => {
            FS_Print(f,
                     b"gamemode\t\t\"multiplayer_only\"\n\x00" as *const u8 as
                         *const libc::c_char);
        }
        _ => { }
    }
    if if *(*GameInfo).sp_entity.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"sp_entity\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).sp_entity.as_mut_ptr());
    }
    if if *(*GameInfo).mp_entity.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"mp_entity\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).mp_entity.as_mut_ptr());
    }
    if if *(*GameInfo).mp_filter.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        FS_Printf(f,
                  b"mp_filter\t\t\"%s\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).mp_filter.as_mut_ptr());
    }
    if (*GameInfo).secure as u64 != 0 {
        FS_Printf(f,
                  b"secure\t\t\"%i\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).secure as libc::c_uint);
    }
    if (*GameInfo).nomodels as u64 != 0 {
        FS_Printf(f,
                  b"nomodels\t\t\"%i\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*GameInfo).nomodels as libc::c_uint);
    }
    if (*GameInfo).max_edicts > 0 as libc::c_int {
        FS_Printf(f,
                  b"max_edicts\t%i\n\x00" as *const u8 as *const libc::c_char,
                  (*GameInfo).max_edicts);
    }
    if (*GameInfo).max_tents > 0 as libc::c_int {
        FS_Printf(f,
                  b"max_tempents\t%i\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).max_tents);
    }
    if (*GameInfo).max_beams > 0 as libc::c_int {
        FS_Printf(f,
                  b"max_beams\t\t%i\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).max_beams);
    }
    if (*GameInfo).max_particles > 0 as libc::c_int {
        FS_Printf(f,
                  b"max_particles\t%i\n\x00" as *const u8 as
                      *const libc::c_char, (*GameInfo).max_particles);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if *(*GameInfo).ambientsound[i as usize].as_mut_ptr() != 0 {
            if write_ambients == 0 {
                FS_Print(f, b"\n\x00" as *const u8 as *const libc::c_char);
                write_ambients = true_0 as libc::c_int
            }
            FS_Printf(f,
                      b"ambient%i\t\t%s\n\x00" as *const u8 as
                          *const libc::c_char, i,
                      (*GameInfo).ambientsound[i as usize].as_mut_ptr());
        }
        i += 1
    }
    FS_Print(f, b"\n\n\n\x00" as *const u8 as *const libc::c_char);
    FS_Close(f);
    // all done
}
#[no_mangle]
pub unsafe extern "C" fn FS_InitGameInfo(mut GameInfo: *mut gameinfo_t,
                                         mut gamedir: *const libc::c_char) {
    memset(GameInfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gameinfo_t>() as libc::c_ulong);
    // filesystem info
    Q_strncpy((*GameInfo).gamefolder.as_mut_ptr(), gamedir,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).basedir.as_mut_ptr(),
              b"valve\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*GameInfo).falldir[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    Q_strncpy((*GameInfo).startmap.as_mut_ptr(),
              b"c0a0\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).trainmap.as_mut_ptr(),
              b"t0a0\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).title.as_mut_ptr(),
              b"New Game\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*GameInfo).version = 1.0f32;
    // .dll pathes
    Q_strncpy((*GameInfo).dll_path.as_mut_ptr(),
              b"cl_dlls\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).game_dll.as_mut_ptr(),
              b"dlls/hl.dll\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).game_dll_linux.as_mut_ptr(),
              b"dlls/hl.so\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).game_dll_osx.as_mut_ptr(),
              b"dlls/hl.dylib\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // .ico path
    Q_strncpy((*GameInfo).iconpath.as_mut_ptr(),
              b"game.ico\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as
                  libc::c_ulong); // default value if not specified
    Q_strncpy((*GameInfo).sp_entity.as_mut_ptr(),
              b"info_player_start\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy((*GameInfo).mp_entity.as_mut_ptr(),
              b"info_player_deathmatch\x00" as *const u8 as
                  *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    (*GameInfo).max_edicts = 900 as libc::c_int;
    (*GameInfo).max_tents = 500 as libc::c_int;
    (*GameInfo).max_beams = 128 as libc::c_int;
    (*GameInfo).max_particles = 4096 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FS_ParseGenericGameInfo(mut GameInfo:
                                                     *mut gameinfo_t,
                                                 mut buf: *const libc::c_char,
                                                 isGameInfo: qboolean) {
    let mut pfile: *mut libc::c_char = buf as *mut libc::c_char;
    let mut found_linux: qboolean = false_0;
    let mut found_osx: qboolean = false_0;
    let mut token: string = [0; 256];
    loop  {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        // different names in liblist/gameinfo
        if Q_strnicmp(token.as_mut_ptr(),
                      if isGameInfo as libc::c_uint != 0 {
                          b"title\x00" as *const u8 as *const libc::c_char
                      } else {
                          b"game\x00" as *const u8 as *const libc::c_char
                      }, 99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).title.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"fallback_dir\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).falldir.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"startmap\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).startmap.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            COM_StripExtension((*GameInfo).startmap.as_mut_ptr());
            // valid for both
            // valid for both
            // HQ2:Amen has extension .bsp
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"trainmap\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 ||
                      isGameInfo as u64 == 0 &&
                          Q_strnicmp(token.as_mut_ptr(),
                                     b"trainingmap\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).trainmap.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            COM_StripExtension((*GameInfo).trainmap.as_mut_ptr());
            // only trainmap is valid for gameinfo
            // HQ2:Amen has extension .bsp
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"url_info\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).game_url.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        } else if Q_strnicmp(token.as_mut_ptr(),
                             if isGameInfo as libc::c_uint != 0 {
                                 b"url_update\x00" as *const u8 as
                                     *const libc::c_char
                             } else {
                                 b"url_dl\x00" as *const u8 as
                                     *const libc::c_char
                             }, 99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).update_url.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"gamedll\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).game_dll.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            COM_FixSlashes((*GameInfo).game_dll.as_mut_ptr());
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"gamedll_linux\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile,
                                   (*GameInfo).game_dll_linux.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            found_linux = true_0
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"gamedll_osx\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile,
                                   (*GameInfo).game_dll_osx.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            found_osx = true_0
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"icon\x00" as *const u8 as *const libc::c_char,
                             99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).iconpath.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            COM_FixSlashes((*GameInfo).iconpath.as_mut_ptr());
            COM_DefaultExtension((*GameInfo).iconpath.as_mut_ptr(),
                                 b".ico\x00" as *const u8 as
                                     *const libc::c_char);
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"type\x00" as *const u8 as *const libc::c_char,
                             99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if isGameInfo as u64 == 0 &&
                   Q_strnicmp(token.as_mut_ptr(),
                              b"singleplayer_only\x00" as *const u8 as
                                  *const libc::c_char, 99999 as libc::c_int)
                       == 0 {
                // valid for both
                // different names
                // valid for both
                // valid for both
                // valid for both
                // valid for both
                // TODO: Remove this ugly hack too.
				// This was made because Half-Life has multiplayer,
				// but for some reason it's marked as singleplayer_only.
				// Old WON version is fine.
                if Q_strnicmp((*GameInfo).gamefolder.as_mut_ptr(),
                              b"valve\x00" as *const u8 as
                                  *const libc::c_char, 99999 as libc::c_int)
                       == 0 {
                    (*GameInfo).gamemode = GAME_NORMAL as libc::c_int
                } else {
                    (*GameInfo).gamemode =
                        GAME_SINGLEPLAYER_ONLY as libc::c_int
                }
                Q_strncpy((*GameInfo).type_0.as_mut_ptr(),
                          b"Single\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            } else if isGameInfo as u64 == 0 &&
                          Q_strnicmp(token.as_mut_ptr(),
                                     b"multiplayer_only\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
                (*GameInfo).gamemode = GAME_MULTIPLAYER_ONLY as libc::c_int;
                Q_strncpy((*GameInfo).type_0.as_mut_ptr(),
                          b"Multiplayer\x00" as *const u8 as
                              *const libc::c_char,
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            } else {
                // pass type without changes
                if isGameInfo as u64 == 0 {
                    (*GameInfo).gamemode = GAME_NORMAL as libc::c_int
                }
                Q_strncpy((*GameInfo).type_0.as_mut_ptr(), token.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
            }
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"version\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*GameInfo).version = Q_atof(token.as_mut_ptr())
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"size\x00" as *const u8 as *const libc::c_char,
                             99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*GameInfo).size = Q_atoi(token.as_mut_ptr()) as size_t
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"edicts\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*GameInfo).max_edicts = Q_atoi(token.as_mut_ptr())
        } else if Q_strnicmp(token.as_mut_ptr(),
                             if isGameInfo as libc::c_uint != 0 {
                                 b"mp_entity\x00" as *const u8 as
                                     *const libc::c_char
                             } else {
                                 b"mpentity\x00" as *const u8 as
                                     *const libc::c_char
                             }, 99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).mp_entity.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        } else if Q_strnicmp(token.as_mut_ptr(),
                             if isGameInfo as libc::c_uint != 0 {
                                 b"mp_filter\x00" as *const u8 as
                                     *const libc::c_char
                             } else {
                                 b"mpfilter\x00" as *const u8 as
                                     *const libc::c_char
                             }, 99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, (*GameInfo).mp_filter.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"secure\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*GameInfo).secure = Q_atoi(token.as_mut_ptr()) as qboolean
        } else if Q_strnicmp(token.as_mut_ptr(),
                             b"nomodels\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*GameInfo).nomodels = Q_atoi(token.as_mut_ptr()) as qboolean
        } else if Q_strnicmp(token.as_mut_ptr(),
                             if isGameInfo as libc::c_uint != 0 {
                                 b"max_edicts\x00" as *const u8 as
                                     *const libc::c_char
                             } else {
                                 b"edicts\x00" as *const u8 as
                                     *const libc::c_char
                             }, 99999 as libc::c_int) == 0 {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*GameInfo).max_edicts =
                if Q_atoi(token.as_mut_ptr()) >= 64 as libc::c_int {
                    if Q_atoi(token.as_mut_ptr()) <
                           (1 as libc::c_int) << 13 as libc::c_int {
                        Q_atoi(token.as_mut_ptr())
                    } else { ((1 as libc::c_int)) << 13 as libc::c_int }
                } else { 64 as libc::c_int }
        } else if isGameInfo as u64 != 0 {
            if Q_strnicmp(token.as_mut_ptr(),
                          b"basedir\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                let mut fs_path: string = [0; 256];
                pfile =
                    _COM_ParseFileSafe(pfile, fs_path.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                if Q_strnicmp(fs_path.as_mut_ptr(),
                              (*GameInfo).basedir.as_mut_ptr(),
                              99999 as libc::c_int) != 0 ||
                       Q_strnicmp(fs_path.as_mut_ptr(),
                                  (*GameInfo).gamefolder.as_mut_ptr(),
                                  99999 as libc::c_int) != 0 {
                    Q_strncpy((*GameInfo).basedir.as_mut_ptr(),
                              fs_path.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong);
                }
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"sp_entity\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile,
                                       (*GameInfo).sp_entity.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_char; 32]>()
                                           as libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int)
            } else if isGameInfo as libc::c_uint != 0 &&
                          Q_strnicmp(token.as_mut_ptr(),
                                     b"dllpath\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile,
                                       (*GameInfo).dll_path.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_char; 64]>()
                                           as libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int)
            } else if isGameInfo as libc::c_uint != 0 &&
                          Q_strnicmp(token.as_mut_ptr(),
                                     b"date\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile, (*GameInfo).date.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_char; 64]>()
                                           as libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int)
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"max_tempents\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                (*GameInfo).max_tents =
                    if Q_atoi(token.as_mut_ptr()) >= 300 as libc::c_int {
                        if Q_atoi(token.as_mut_ptr()) < 2048 as libc::c_int {
                            Q_atoi(token.as_mut_ptr())
                        } else { 2048 as libc::c_int }
                    } else { 300 as libc::c_int }
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"max_beams\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                (*GameInfo).max_beams =
                    if Q_atoi(token.as_mut_ptr()) >= 64 as libc::c_int {
                        if Q_atoi(token.as_mut_ptr()) < 512 as libc::c_int {
                            Q_atoi(token.as_mut_ptr())
                        } else { 512 as libc::c_int }
                    } else { 64 as libc::c_int }
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"max_particles\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                (*GameInfo).max_particles =
                    if Q_atoi(token.as_mut_ptr()) >= 4096 as libc::c_int {
                        if Q_atoi(token.as_mut_ptr()) < 32768 as libc::c_int {
                            Q_atoi(token.as_mut_ptr())
                        } else { 32768 as libc::c_int }
                    } else { 4096 as libc::c_int }
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"gamemode\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                // valid for both
                // valid for both
                // valid for both
                // valid for both
                // only for gameinfo
                // TODO: Remove this ugly hack too.
				// This was made because Half-Life has multiplayer,
				// but for some reason it's marked as singleplayer_only.
				// Old WON version is fine.
                if Q_strnicmp(token.as_mut_ptr(),
                              b"singleplayer_only\x00" as *const u8 as
                                  *const libc::c_char, 99999 as libc::c_int)
                       == 0 &&
                       Q_strnicmp((*GameInfo).gamefolder.as_mut_ptr(),
                                  b"valve\x00" as *const u8 as
                                      *const libc::c_char,
                                  99999 as libc::c_int) != 0 {
                    (*GameInfo).gamemode =
                        GAME_SINGLEPLAYER_ONLY as libc::c_int
                } else if Q_strnicmp(token.as_mut_ptr(),
                                     b"multiplayer_only\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
                    (*GameInfo).gamemode =
                        GAME_MULTIPLAYER_ONLY as libc::c_int
                }
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"ambient\x00" as *const u8 as
                                     *const libc::c_char, 7 as libc::c_int) ==
                          0 {
                let mut ambientNum: libc::c_int =
                    Q_atoi(token.as_mut_ptr().offset(7 as libc::c_int as
                                                         isize));
                if ambientNum < 0 as libc::c_int ||
                       ambientNum > 4 as libc::c_int - 1 as libc::c_int {
                    ambientNum = 0 as libc::c_int
                }
                pfile =
                    _COM_ParseFileSafe(pfile,
                                       (*GameInfo).ambientsound[ambientNum as
                                                                    usize].as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_char; 64]>()
                                           as libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int)
            } else if Q_strnicmp(token.as_mut_ptr(),
                                 b"noskills\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                pfile =
                    _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                       ::std::mem::size_of::<string>() as
                                           libc::c_ulong as libc::c_int,
                                       0 as libc::c_int as libc::c_uint,
                                       0 as *mut libc::c_int);
                (*GameInfo).noskills = Q_atoi(token.as_mut_ptr()) as qboolean
            }
        }
    }
    if found_linux as u64 == 0 || found_osx as u64 == 0 {
        // just replace extension from dll to so/dylib
        let mut gamedll: [libc::c_char; 64] = [0; 64];
        Q_strncpy(gamedll.as_mut_ptr(), (*GameInfo).game_dll.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        COM_StripExtension(gamedll.as_mut_ptr());
        if found_linux as u64 == 0 {
            Q_snprintf((*GameInfo).game_dll_linux.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong,
                       b"%s.so\x00" as *const u8 as *const libc::c_char,
                       gamedll.as_mut_ptr());
        }
        if found_osx as u64 == 0 {
            Q_snprintf((*GameInfo).game_dll_osx.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong,
                       b"%s.dylib\x00" as *const u8 as *const libc::c_char,
                       gamedll.as_mut_ptr());
        }
    }
    // make sure what gamedir is really exist
    if FS_SysFolderExists(va(b"%s/%s\x00" as *const u8 as *const libc::c_char,
                             host.rootdir.as_mut_ptr(),
                             (*GameInfo).falldir.as_mut_ptr())) as u64 == 0 {
        (*GameInfo).falldir[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    };
}
/*
================
FS_CreateDefaultGameInfo
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_CreateDefaultGameInfo(mut filename:
                                                      *const libc::c_char) {
    let mut defGI: gameinfo_t =
        gameinfo_t{gamefolder: [0; 64],
                   basedir: [0; 64],
                   falldir: [0; 64],
                   startmap: [0; 64],
                   trainmap: [0; 64],
                   title: [0; 64],
                   version: 0.,
                   dll_path: [0; 64],
                   game_dll: [0; 64],
                   iconpath: [0; 64],
                   game_url: [0; 256],
                   update_url: [0; 256],
                   type_0: [0; 64],
                   date: [0; 64],
                   size: 0,
                   gamemode: 0,
                   secure: false_0,
                   nomodels: false_0,
                   noskills: false_0,
                   sp_entity: [0; 32],
                   mp_entity: [0; 32],
                   mp_filter: [0; 32],
                   ambientsound: [[0; 64]; 4],
                   max_edicts: 0,
                   max_tents: 0,
                   max_beams: 0,
                   max_particles: 0,
                   game_dll_linux: [0; 64],
                   game_dll_osx: [0; 64],
                   added: false_0,};
    FS_InitGameInfo(&mut defGI, fs_basedir.as_mut_ptr());
    // make simple gameinfo.txt
    FS_WriteGameInfo(filename, &mut defGI);
}
/*
================
FS_ParseLiblistGam
================
*/
unsafe extern "C" fn FS_ParseLiblistGam(mut filename: *const libc::c_char,
                                        mut gamedir: *const libc::c_char,
                                        mut GameInfo: *mut gameinfo_t)
 -> qboolean {
    let mut afile: *mut libc::c_char = 0 as *mut libc::c_char;
    if GameInfo.is_null() { return false_0 }
    afile =
        FS_LoadDirectFile(filename, 0 as *mut fs_offset_t) as
            *mut libc::c_char;
    if afile.is_null() { return false_0 }
    FS_InitGameInfo(GameInfo, gamedir);
    FS_ParseGenericGameInfo(GameInfo, afile, false_0);
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 1848 as libc::c_int);
    return true_0;
}
/*
================
FS_ConvertGameInfo
================
*/
unsafe extern "C" fn FS_ConvertGameInfo(mut gamedir: *const libc::c_char,
                                        mut gameinfo_path:
                                            *const libc::c_char,
                                        mut liblist_path: *const libc::c_char)
 -> qboolean {
    let mut GameInfo: gameinfo_t =
        gameinfo_t{gamefolder: [0; 64],
                   basedir: [0; 64],
                   falldir: [0; 64],
                   startmap: [0; 64],
                   trainmap: [0; 64],
                   title: [0; 64],
                   version: 0.,
                   dll_path: [0; 64],
                   game_dll: [0; 64],
                   iconpath: [0; 64],
                   game_url: [0; 256],
                   update_url: [0; 256],
                   type_0: [0; 64],
                   date: [0; 64],
                   size: 0,
                   gamemode: 0,
                   secure: false_0,
                   nomodels: false_0,
                   noskills: false_0,
                   sp_entity: [0; 32],
                   mp_entity: [0; 32],
                   mp_filter: [0; 32],
                   ambientsound: [[0; 64]; 4],
                   max_edicts: 0,
                   max_tents: 0,
                   max_beams: 0,
                   max_particles: 0,
                   game_dll_linux: [0; 64],
                   game_dll_osx: [0; 64],
                   added: false_0,};
    memset(&mut GameInfo as *mut gameinfo_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gameinfo_t>() as libc::c_ulong);
    if FS_ParseLiblistGam(liblist_path, gamedir, &mut GameInfo) as u64 != 0 {
        Con_DPrintf(b"Convert %s to %s\n\x00" as *const u8 as
                        *const libc::c_char, liblist_path, gameinfo_path);
        FS_WriteGameInfo(gameinfo_path, &mut GameInfo);
        return true_0
    }
    return false_0;
}
/*
================
FS_ReadGameInfo
================
*/
unsafe extern "C" fn FS_ReadGameInfo(mut filepath: *const libc::c_char,
                                     mut gamedir: *const libc::c_char,
                                     mut GameInfo: *mut gameinfo_t)
 -> qboolean {
    let mut afile: *mut libc::c_char = 0 as *mut libc::c_char;
    afile =
        FS_LoadFile(filepath, 0 as *mut fs_offset_t, false_0) as
            *mut libc::c_char;
    if afile.is_null() { return false_0 }
    FS_InitGameInfo(GameInfo, gamedir);
    FS_ParseGenericGameInfo(GameInfo, afile, true_0);
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 1891 as libc::c_int);
    return true_0;
}
/*
================
FS_CheckForGameDir
================
*/
unsafe extern "C" fn FS_CheckForGameDir(mut gamedir: *const libc::c_char)
 -> qboolean {
    // if directory contain config.cfg it's 100% gamedir
    if FS_FileExists(va(b"%s/config.cfg\x00" as *const u8 as
                            *const libc::c_char, gamedir),
                     false_0 as libc::c_int) != 0 {
        return true_0
    }
    // if directory contain progs.dat it's 100% gamedir
    if FS_FileExists(va(b"%s/progs.dat\x00" as *const u8 as
                            *const libc::c_char, gamedir),
                     false_0 as libc::c_int) != 0 {
        return true_0
    }
    // quake mods probably always archived but can missed config.cfg before first running
    if FS_FileExists(va(b"%s/pak0.pak\x00" as *const u8 as
                            *const libc::c_char, gamedir),
                     false_0 as libc::c_int) != 0 {
        return true_0
    }
    // NOTE; adds here some additional checks if you wished
    return false_0;
}
/*
================
FS_ParseGameInfo
================
*/
unsafe extern "C" fn FS_ParseGameInfo(mut gamedir: *const libc::c_char,
                                      mut GameInfo: *mut gameinfo_t)
 -> qboolean {
    let mut liblist_path: string = [0; 256];
    let mut gameinfo_path: string = [0; 256];
    let mut default_gameinfo_path: string = [0; 256];
    let mut tmpGameInfo: gameinfo_t =
        gameinfo_t{gamefolder: [0; 64],
                   basedir: [0; 64],
                   falldir: [0; 64],
                   startmap: [0; 64],
                   trainmap: [0; 64],
                   title: [0; 64],
                   version: 0.,
                   dll_path: [0; 64],
                   game_dll: [0; 64],
                   iconpath: [0; 64],
                   game_url: [0; 256],
                   update_url: [0; 256],
                   type_0: [0; 64],
                   date: [0; 64],
                   size: 0,
                   gamemode: 0,
                   secure: false_0,
                   nomodels: false_0,
                   noskills: false_0,
                   sp_entity: [0; 32],
                   mp_entity: [0; 32],
                   mp_filter: [0; 32],
                   ambientsound: [[0; 64]; 4],
                   max_edicts: 0,
                   max_tents: 0,
                   max_beams: 0,
                   max_particles: 0,
                   game_dll_linux: [0; 64],
                   game_dll_osx: [0; 64],
                   added: false_0,};
    let mut haveUpdate: qboolean = false_0;
    Q_snprintf(default_gameinfo_path.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s/gameinfo.txt\x00" as *const u8 as *const libc::c_char,
               fs_basedir.as_mut_ptr());
    Q_snprintf(gameinfo_path.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s/gameinfo.txt\x00" as *const u8 as *const libc::c_char,
               gamedir);
    Q_snprintf(liblist_path.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s/liblist.gam\x00" as *const u8 as *const libc::c_char,
               gamedir);
    // here goes some RoDir magic...
    if if *host.rodir.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        let mut filepath_ro: string = [0; 256];
        let mut liblist_ro: string = [0; 256];
        let mut roLibListTime: fs_offset_t = 0;
        let mut roGameInfoTime: fs_offset_t = 0;
        let mut rwGameInfoTime: fs_offset_t = 0;
        FS_AllowDirectPaths(true_0);
        Q_snprintf(filepath_ro.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s/%s/gameinfo.txt\x00" as *const u8 as
                       *const libc::c_char, host.rodir.as_mut_ptr(), gamedir);
        Q_snprintf(liblist_ro.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s/%s/liblist.gam\x00" as *const u8 as
                       *const libc::c_char, host.rodir.as_mut_ptr(), gamedir);
        roLibListTime =
            FS_SysFileTime(liblist_ro.as_mut_ptr()) as fs_offset_t;
        roGameInfoTime =
            FS_SysFileTime(filepath_ro.as_mut_ptr()) as fs_offset_t;
        rwGameInfoTime =
            FS_SysFileTime(gameinfo_path.as_mut_ptr()) as fs_offset_t;
        if roLibListTime > rwGameInfoTime {
            haveUpdate =
                FS_ConvertGameInfo(gamedir, gameinfo_path.as_mut_ptr(),
                                   liblist_ro.as_mut_ptr())
        } else if roGameInfoTime > rwGameInfoTime {
            let mut afile_ro: *mut libc::c_char =
                FS_LoadDirectFile(filepath_ro.as_mut_ptr(),
                                  0 as *mut fs_offset_t) as *mut libc::c_char;
            if !afile_ro.is_null() {
                let mut gi: gameinfo_t =
                    gameinfo_t{gamefolder: [0; 64],
                               basedir: [0; 64],
                               falldir: [0; 64],
                               startmap: [0; 64],
                               trainmap: [0; 64],
                               title: [0; 64],
                               version: 0.,
                               dll_path: [0; 64],
                               game_dll: [0; 64],
                               iconpath: [0; 64],
                               game_url: [0; 256],
                               update_url: [0; 256],
                               type_0: [0; 64],
                               date: [0; 64],
                               size: 0,
                               gamemode: 0,
                               secure: false_0,
                               nomodels: false_0,
                               noskills: false_0,
                               sp_entity: [0; 32],
                               mp_entity: [0; 32],
                               mp_filter: [0; 32],
                               ambientsound: [[0; 64]; 4],
                               max_edicts: 0,
                               max_tents: 0,
                               max_beams: 0,
                               max_particles: 0,
                               game_dll_linux: [0; 64],
                               game_dll_osx: [0; 64],
                               added: false_0,};
                haveUpdate = true_0;
                FS_InitGameInfo(&mut gi, gamedir);
                FS_ParseGenericGameInfo(&mut gi, afile_ro, true_0);
                FS_WriteGameInfo(gameinfo_path.as_mut_ptr(), &mut gi);
                _Mem_Free(afile_ro as *mut libc::c_void,
                          b"../engine/common/filesystem.c\x00" as *const u8 as
                              *const libc::c_char, 1968 as libc::c_int);
            }
        }
        FS_AllowDirectPaths(false_0);
    }
    // if user change liblist.gam update the gameinfo.txt
    if FS_FileTime(liblist_path.as_mut_ptr(), false_0) >
           FS_FileTime(gameinfo_path.as_mut_ptr(), false_0) {
        FS_ConvertGameInfo(gamedir, gameinfo_path.as_mut_ptr(),
                           liblist_path.as_mut_ptr());
    }
    // force to create gameinfo for specified game if missing
    if (FS_CheckForGameDir(gamedir) as libc::c_uint != 0 ||
            Q_strnicmp(fs_gamedir.as_mut_ptr(), gamedir, 99999 as libc::c_int)
                == 0) &&
           FS_FileExists(gameinfo_path.as_mut_ptr(), false_0 as libc::c_int)
               == 0 {
        memset(&mut tmpGameInfo as *mut gameinfo_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<gameinfo_t>() as libc::c_ulong);
        if FS_ReadGameInfo(default_gameinfo_path.as_mut_ptr(), gamedir,
                           &mut tmpGameInfo) as u64 != 0 {
            // now we have copy of game info from basedir but needs to change gamedir
            Con_DPrintf(b"Convert %s to %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        default_gameinfo_path.as_mut_ptr(),
                        gameinfo_path.as_mut_ptr()); // no dest
            Q_strncpy(tmpGameInfo.gamefolder.as_mut_ptr(), gamedir,
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong);
            FS_WriteGameInfo(gameinfo_path.as_mut_ptr(), &mut tmpGameInfo);
        } else { FS_CreateDefaultGameInfo(gameinfo_path.as_mut_ptr()); }
    }
    if GameInfo.is_null() ||
           FS_FileExists(gameinfo_path.as_mut_ptr(), false_0 as libc::c_int)
               == 0 {
        return false_0
    }
    if FS_ReadGameInfo(gameinfo_path.as_mut_ptr(), gamedir, GameInfo) as u64
           != 0 {
        return true_0
    }
    return false_0;
}
/*
================
FS_LoadGameInfo

can be passed null arg
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_LoadGameInfo(mut rootfolder:
                                             *const libc::c_char) {
    let mut i: libc::c_int = 0;
    // lock uplevel of gamedir for read\write
    fs_ext_path = false_0;
    if !rootfolder.is_null() {
        Q_strncpy(fs_gamedir.as_mut_ptr(), rootfolder,
                  99999 as libc::c_int as size_t);
    }
    Con_Reportf(b"FS_LoadGameInfo( %s )\n\x00" as *const u8 as
                    *const libc::c_char, fs_gamedir.as_mut_ptr());
    // clear any old pathes
    FS_ClearSearchPath();
    // validate gamedir
    i = 0 as libc::c_int; // create new filesystem
    while i < SI.numgames {
        if Q_strnicmp((*SI.games[i as usize]).gamefolder.as_mut_ptr(),
                      fs_gamedir.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            break ;
        }
        i += 1
    }
    if i == SI.numgames {
        Sys_Error(b"Couldn\'t find game directory \'%s\'\n\x00" as *const u8
                      as *const libc::c_char, fs_gamedir.as_mut_ptr());
    }
    SI.GameInfo = SI.games[i as usize];
    if _Sys_GetParmFromCmdLine(b"-dll\x00" as *const u8 as
                                   *const libc::c_char,
                               SI.gamedll.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong) as u64 == 0 {
        SI.gamedll[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    }
    if _Sys_GetParmFromCmdLine(b"-clientlib\x00" as *const u8 as
                                   *const libc::c_char,
                               SI.clientlib.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong) as u64 == 0 {
        SI.clientlib[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    }
    FS_Rescan();
    Image_CheckPaletteQ1();
    Host_InitDecals();
    // reload decals
}
/*
================
FS_Init
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Init() {
    let mut dirs: stringlist_t =
        stringlist_t{maxstrings: 0,
                     numstrings: 0,
                     strings: 0 as *mut *mut libc::c_char,};
    let mut hasBaseDir: qboolean = false_0;
    let mut hasGameDir: qboolean = false_0;
    let mut i: libc::c_int = 0;
    FS_InitMemory();
    Cmd_AddRestrictedCommand(b"fs_rescan\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(FS_Rescan_f as
                                      unsafe extern "C" fn() -> ()),
                             b"rescan filesystem search pathes\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"fs_path\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(FS_Path_f as unsafe extern "C" fn() -> ()),
                             b"show filesystem search pathes\x00" as *const u8
                                 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"fs_clearpaths\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(FS_ClearPaths_f as
                                      unsafe extern "C" fn() -> ()),
                             b"clear filesystem search pathes\x00" as
                                 *const u8 as *const libc::c_char);
    if Sys_CheckParm(b"-casesensitive\x00" as *const u8 as
                         *const libc::c_char) != 0 {
        fs_caseinsensitive = false_0
    }
    if fs_caseinsensitive as u64 == 0 {
        if (if *host.rodir.as_mut_ptr() == 0 {
                0 as libc::c_int
            } else { 1 as libc::c_int }) != 0 &&
               Q_strncmp(host.rodir.as_mut_ptr(), host.rootdir.as_mut_ptr(),
                         99999 as libc::c_int) == 0 {
            Sys_Error(b"RoDir and default rootdir can\'t point to same directory!\x00"
                          as *const u8 as *const libc::c_char);
        }
    } else if (if *host.rodir.as_mut_ptr() == 0 {
                   0 as libc::c_int
               } else { 1 as libc::c_int }) != 0 &&
                  Q_strnicmp(host.rodir.as_mut_ptr(),
                             host.rootdir.as_mut_ptr(), 99999 as libc::c_int)
                      == 0 {
        Sys_Error(b"RoDir and default rootdir can\'t point to same directory!\x00"
                      as *const u8 as *const libc::c_char);
    }
    // ignore commandlineoption "-game" for other stuff
    SI.numgames = 0 as libc::c_int; // default dir
    Q_strncpy(fs_basedir.as_mut_ptr(), SI.basedirName.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as
                  libc::c_ulong); // gamedir == basedir
    if _Sys_GetParmFromCmdLine(b"-game\x00" as *const u8 as
                                   *const libc::c_char,
                               fs_gamedir.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 1024]>()
                                   as libc::c_ulong) as u64 == 0 {
        Q_strncpy(fs_gamedir.as_mut_ptr(), fs_basedir.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 1024]>() as
                      libc::c_ulong);
    }
    if FS_CheckNastyPath(fs_basedir.as_mut_ptr(), true_0) != 0 {
        // this is completely fatal...
        Sys_Error(b"invalid base directory \"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, fs_basedir.as_mut_ptr());
    }
    if FS_CheckNastyPath(fs_gamedir.as_mut_ptr(), true_0) != 0 {
        Con_Printf(b"^1Error:^7 invalid game directory \"%s\"\n\x00" as
                       *const u8 as *const libc::c_char,
                   fs_gamedir.as_mut_ptr());
        Q_strncpy(fs_gamedir.as_mut_ptr(), fs_basedir.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 1024]>() as
                      libc::c_ulong);
        // default dir
    }
    // add readonly directories first
    if if *host.rodir.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        stringlistinit(&mut dirs);
        listdirectory(&mut dirs, host.rodir.as_mut_ptr(), false_0);
        stringlistsort(&mut dirs);
        i = 0 as libc::c_int;
        while i < dirs.numstrings {
            // no need to check folders here, FS_CreatePath will not fail if path exists
			// and listdirectory returns only really existing directories
            FS_CreatePath(va(b"%s/%s/\x00" as *const u8 as
                                 *const libc::c_char,
                             host.rootdir.as_mut_ptr(),
                             *dirs.strings.offset(i as isize)));
            i += 1
        }
        stringlistfreecontents(&mut dirs);
    }
    // validate directories
    stringlistinit(&mut dirs);
    listdirectory(&mut dirs, b"./\x00" as *const u8 as *const libc::c_char,
                  false_0);
    stringlistsort(&mut dirs);
    i = 0 as libc::c_int;
    while i < dirs.numstrings {
        if Q_strnicmp(fs_basedir.as_mut_ptr(),
                      *dirs.strings.offset(i as isize), 99999 as libc::c_int)
               == 0 {
            hasBaseDir = true_0
        }
        if Q_strnicmp(fs_gamedir.as_mut_ptr(),
                      *dirs.strings.offset(i as isize), 99999 as libc::c_int)
               == 0 {
            hasGameDir = true_0
        }
        i += 1
    }
    if hasGameDir as u64 == 0 {
        Con_Printf(b"^1Error:^7 game directory \"%s\" not exist\n\x00" as
                       *const u8 as *const libc::c_char,
                   fs_gamedir.as_mut_ptr());
        if hasBaseDir as u64 != 0 {
            Q_strncpy(fs_gamedir.as_mut_ptr(), fs_basedir.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 1024]>() as
                          libc::c_ulong);
        }
    }
    // build list of game directories here
    FS_AddGameDirectory(b"./\x00" as *const u8 as *const libc::c_char,
                        0 as libc::c_int as uint);
    i = 0 as libc::c_int;
    while i < dirs.numstrings {
        if !(FS_SysFolderExists(*dirs.strings.offset(i as isize)) as u64 == 0
                 ||
                 Q_strncmp(*dirs.strings.offset(i as isize),
                           b"..\x00" as *const u8 as *const libc::c_char,
                           99999 as libc::c_int) == 0 &&
                     fs_ext_path as u64 == 0) {
            if SI.games[SI.numgames as usize].is_null() {
                SI.games[SI.numgames as usize] =
                    _Mem_Alloc(fs_mempool,
                               ::std::mem::size_of::<gameinfo_t>() as
                                   libc::c_ulong, true_0,
                               b"../engine/common/filesystem.c\x00" as
                                   *const u8 as *const libc::c_char,
                               2154 as libc::c_int) as *mut gameinfo_t
            }
            if FS_ParseGameInfo(*dirs.strings.offset(i as isize),
                                SI.games[SI.numgames as usize]) as u64 != 0 {
                SI.numgames += 1
            }
        }
        i += 1
        // added
    }
    stringlistfreecontents(&mut dirs);
    Con_Reportf(b"FS_Init: done\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn FS_AllowDirectPaths(mut enable: qboolean) {
    fs_ext_path = enable;
}
/*
================
FS_Shutdown
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Shutdown() {
    let mut i: libc::c_int = 0;
    // release gamedirs
    i = 0 as libc::c_int; // release all wad files too
    while i < SI.numgames {
        if !SI.games[i as usize].is_null() {
            _Mem_Free(SI.games[i as usize] as *mut libc::c_void,
                      b"../engine/common/filesystem.c\x00" as *const u8 as
                          *const libc::c_char, 2180 as libc::c_int);
        }
        i += 1
    }
    memset(&mut SI as *mut sysinfo_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sysinfo_t>() as libc::c_ulong);
    FS_ClearSearchPath();
    _Mem_FreePool(&mut fs_mempool,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 2185 as libc::c_int);
}
/*
====================
FS_SysFileTime

Internal function used to determine filetime
====================
*/
unsafe extern "C" fn FS_SysFileTime(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(filename, &mut buf) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    return buf.st_mtim.tv_sec as libc::c_int;
}
/*
====================
FS_SysOpen

Internal function used to create a file_t and open the relevant non-packed file on disk
====================
*/
unsafe extern "C" fn FS_SysOpen(mut filepath: *const libc::c_char,
                                mut mode: *const libc::c_char)
 -> *mut file_t {
    let mut file: *mut file_t = 0 as *mut file_t;
    let mut mod_0: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut ind: uint = 0;
    // Parse the mode string
    match *mode.offset(0 as libc::c_int as isize) as libc::c_int {
        114 => {
            // read
            mod_0 = 0 as libc::c_int;
            opt = 0 as libc::c_int
        }
        119 => {
            // write
            mod_0 = 0o1 as libc::c_int;
            opt = 0o100 as libc::c_int | 0o1000 as libc::c_int
        }
        97 => {
            // append
            mod_0 = 0o1 as libc::c_int;
            opt = 0o100 as libc::c_int | 0o2000 as libc::c_int
        }
        101 => {
            // edit
            mod_0 = 0o1 as libc::c_int;
            opt = 0o100 as libc::c_int
        }
        _ => { return 0 as *mut file_t }
    }
    ind = 1 as libc::c_int as uint;
    while *mode.offset(ind as isize) as libc::c_int != '\u{0}' as i32 {
        match *mode.offset(ind as isize) as libc::c_int {
            43 => { mod_0 = 0o2 as libc::c_int }
            98 => { opt |= 0 as libc::c_int }
            _ => { }
        }
        ind = ind.wrapping_add(1)
    }
    file =
        _Mem_Alloc(fs_mempool,
                   ::std::mem::size_of::<file_t>() as libc::c_ulong, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 2256 as libc::c_int) as
            *mut file_t;
    (*file).filetime = FS_SysFileTime(filepath) as time_t;
    (*file).ungetc = -(1 as libc::c_int);
    (*file).handle = open(filepath, mod_0 | opt, 0o666 as libc::c_int);
    if (*file).handle < 0 as libc::c_int {
        let mut ffilepath: *const libc::c_char = FS_FixFileCase(filepath);
        if ffilepath != filepath {
            (*file).handle =
                open(ffilepath, mod_0 | opt, 0o666 as libc::c_int)
        }
        if (*file).handle >= 0 as libc::c_int {
            FS_BackupFileName(file, ffilepath, (mod_0 | opt) as uint);
        }
    } else { FS_BackupFileName(file, filepath, (mod_0 | opt) as uint); }
    if (*file).handle < 0 as libc::c_int {
        _Mem_Free(file as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 2281 as libc::c_int);
        return 0 as *mut file_t
    }
    (*file).real_length =
        lseek((*file).handle, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    // uncomment do disable write
	//if( opt & O_CREAT )
	//	return NULL;
    // For files opened in append mode, we start at the end of the file
    if opt & 0o2000 as libc::c_int != 0 {
        (*file).position = (*file).real_length
    } else {
        lseek((*file).handle, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    }
    return file;
}
/*
static int FS_DuplicateHandle( const char *filename, int handle, fs_offset_t pos )
{
#ifdef HAVE_DUP
	return dup( handle );
#else
	int newhandle = open( filename, O_RDONLY|O_BINARY );
	lseek( newhandle, pos, SEEK_SET );
	return newhandle;
#endif
}
*/
unsafe extern "C" fn FS_OpenHandle(mut syspath: *const libc::c_char,
                                   mut handle: libc::c_int,
                                   mut offset: fs_offset_t,
                                   mut len: fs_offset_t) -> *mut file_t {
    let mut file: *mut file_t =
        _Mem_Alloc(fs_mempool,
                   ::std::mem::size_of::<file_t>() as libc::c_ulong, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 2313 as libc::c_int) as
            *mut file_t;
    (*file).handle = dup(handle);
    if lseek((*file).handle, offset, 0 as libc::c_int) ==
           -(1 as libc::c_int) as libc::c_long {
        _Mem_Free(file as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 2323 as libc::c_int);
        return 0 as *mut file_t
    }
    (*file).real_length = len;
    (*file).offset = offset;
    (*file).position = 0 as libc::c_int as fs_offset_t;
    (*file).ungetc = -(1 as libc::c_int);
    return file;
}
/*
===========
FS_OpenPackedFile

Open a packed file using its package file descriptor
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_OpenPackedFile(mut pack: *mut pack_t,
                                           mut pack_ind: libc::c_int)
 -> *mut file_t {
    let mut pfile: *mut dpackfile_t = 0 as *mut dpackfile_t;
    pfile = &mut *(*pack).files.offset(pack_ind as isize) as *mut dpackfile_t;
    return FS_OpenHandle((*pack).filename.as_mut_ptr(), (*pack).handle,
                         (*pfile).filepos as fs_offset_t,
                         (*pfile).filelen as fs_offset_t);
}
/*
===========
FS_OpenZipFile

Open a packed file using its package file descriptor
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_OpenZipFile(mut zip: *mut zip_t,
                                        mut pack_ind: libc::c_int)
 -> *mut file_t {
    let mut pfile: *mut zipfile_t = 0 as *mut zipfile_t;
    pfile = &mut *(*zip).files.offset(pack_ind as isize) as *mut zipfile_t;
    // compressed files handled in Zip_LoadFile
    if (*pfile).flags as libc::c_int != 0 as libc::c_int {
        Con_Printf(b"^1Error:^7 %s: can\'t open compressed file %s\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*::std::mem::transmute::<&[u8; 15],
                                             &[libc::c_char; 15]>(b"FS_OpenZipFile\x00")).as_ptr(),
                   (*pfile).name.as_mut_ptr());
        return 0 as *mut file_t
    }
    return FS_OpenHandle((*zip).filename.as_mut_ptr(), (*zip).handle,
                         (*pfile).offset, (*pfile).size);
}
/*
==================
FS_SysFileExists

Look for a file in the filesystem only
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_SysFileExists(mut path: *const libc::c_char,
                                          mut caseinsensitive: qboolean)
 -> qboolean {
    let mut ret: libc::c_int = 0;
    let mut buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    ret = stat(path, &mut buf);
    // speedup custom path search
    if caseinsensitive as libc::c_uint != 0 && ret < 0 as libc::c_int {
        let mut fpath: *const libc::c_char = FS_FixFileCase(path);
        if fpath != path { ret = stat(fpath, &mut buf) }
    }
    if ret < 0 as libc::c_int { return false_0 }
    return (buf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                0o100000 as libc::c_int as libc::c_uint) as libc::c_int as
               qboolean;
}
/*
==================
FS_SetCurrentDirectory

Sets current directory, path should be in UTF-8 encoding
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_SetCurrentDirectory(mut path: *const libc::c_char)
 -> libc::c_int {
    return (chdir(path) == 0) as libc::c_int;
}
/*
==================
FS_SysFolderExists

Look for a existing folder
==================
*/
unsafe extern "C" fn FS_SysFolderExists(mut path: *const libc::c_char)
 -> qboolean {
    let mut dir: *mut DIR = opendir(path);
    if !dir.is_null() {
        closedir(dir);
        return true_0
    } else if *__errno_location() == 2 as libc::c_int ||
                  *__errno_location() == 20 as libc::c_int {
        return false_0
    } else {
        Con_Reportf(b"^1Error:^7 FS_SysFolderExists: problem while opening dir: %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()));
        return false_0
    };
}
/*
====================
FS_FindFile

Look for a file in the packages and in the filesystem

Return the searchpath where the file was found (or NULL)
and the file index in the package if relevant
====================
*/
unsafe extern "C" fn FS_FindFile(mut name: *const libc::c_char,
                                 mut index: *mut libc::c_int,
                                 mut gamedironly: qboolean)
 -> *mut searchpath_t {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pEnvPath: *mut libc::c_char = 0 as *mut libc::c_char;
    // search through the path, one element at a time
    search = fs_searchpaths;
    while !search.is_null() {
        if !(gamedironly as libc::c_uint &
                 ((*search).flags as libc::c_uint &
                      ((1 as libc::c_uint) << 2 as libc::c_int |
                           (1 as libc::c_uint) << 3 as libc::c_int |
                           (1 as libc::c_uint) << 4 as libc::c_int) == 0) as
                     libc::c_int as libc::c_uint != 0) {
            // is the element a pak file?
            if !(*search).pack.is_null() {
                let mut left: libc::c_int = 0;
                let mut right: libc::c_int = 0;
                let mut middle: libc::c_int = 0;
                let mut pak: *mut pack_t = 0 as *mut pack_t;
                pak = (*search).pack;
                // look for the file (binary search)
                left = 0 as libc::c_int;
                right = (*pak).numfiles - 1 as libc::c_int;
                while left <= right {
                    let mut diff: libc::c_int = 0;
                    middle = (left + right) / 2 as libc::c_int;
                    diff =
                        Q_strnicmp((*(*pak).files.offset(middle as
                                                             isize)).name.as_mut_ptr(),
                                   name, 99999 as libc::c_int);
                    // Found it
                    if diff == 0 {
                        if !index.is_null() { *index = middle }
                        return search
                    }
                    // if we're too far in the list
                    if diff > 0 as libc::c_int {
                        right = middle - 1 as libc::c_int
                    } else { left = middle + 1 as libc::c_int }
                }
            } else if !(*search).wad.is_null() {
                let mut lump: *mut dlumpinfo_t = 0 as *mut dlumpinfo_t;
                let mut type_0: libc::c_schar = W_TypeFromExt(name);
                let mut anywadname: qboolean = true_0;
                let mut wadname: string = [0; 256];
                let mut wadfolder: string = [0; 256];
                let mut shortname: string = [0; 256];
                // quick reject by filetype
                if !(type_0 as libc::c_int == 0 as libc::c_int) {
                    COM_ExtractFilePath(name, wadname.as_mut_ptr());
                    wadfolder[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    if if *wadname.as_mut_ptr() == 0 {
                           0 as libc::c_int
                       } else { 1 as libc::c_int } != 0 {
                        COM_FileBase(wadname.as_mut_ptr(),
                                     wadname.as_mut_ptr());
                        Q_strncpy(wadfolder.as_mut_ptr(),
                                  wadname.as_mut_ptr(),
                                  ::std::mem::size_of::<string>() as
                                      libc::c_ulong);
                        COM_DefaultExtension(wadname.as_mut_ptr(),
                                             b".wad\x00" as *const u8 as
                                                 *const libc::c_char);
                        anywadname = false_0
                    }
                    // make wadname from wad fullpath
                    COM_FileBase((*(*search).wad).filename.as_mut_ptr(),
                                 shortname.as_mut_ptr());
                    COM_DefaultExtension(shortname.as_mut_ptr(),
                                         b".wad\x00" as *const u8 as
                                             *const libc::c_char);
                    // quick reject by wadname
                    if !(anywadname as u64 == 0 &&
                             Q_strnicmp(wadname.as_mut_ptr(),
                                        shortname.as_mut_ptr(),
                                        99999 as libc::c_int) != 0) {
                        // NOTE: we can't using long names for wad,
			// because we using original wad names[16];
                        COM_FileBase(name, shortname.as_mut_ptr());
                        lump =
                            W_FindLump((*search).wad, shortname.as_mut_ptr(),
                                       type_0);
                        if !lump.is_null() {
                            if !index.is_null() {
                                *index =
                                    lump.wrapping_offset_from((*(*search).wad).lumps)
                                        as libc::c_long as libc::c_int
                            }
                            return search
                        }
                    }
                }
            } else if !(*search).zip.is_null() {
                let mut left_0: libc::c_int = 0;
                let mut right_0: libc::c_int = 0;
                let mut middle_0: libc::c_int = 0;
                let mut zip: *mut zip_t = 0 as *mut zip_t;
                zip = (*search).zip;
                // look for the file (binary search)
                left_0 = 0 as libc::c_int;
                right_0 = (*zip).numfiles - 1 as libc::c_int;
                while left_0 <= right_0 {
                    let mut diff_0: libc::c_int = 0;
                    middle_0 = (left_0 + right_0) / 2 as libc::c_int;
                    diff_0 =
                        Q_strnicmp((*(*zip).files.offset(middle_0 as
                                                             isize)).name.as_mut_ptr(),
                                   name, 99999 as libc::c_int);
                    // Found it
                    if diff_0 == 0 {
                        if !index.is_null() { *index = middle_0 }
                        return search
                    }
                    // if we're too far in the list
                    if diff_0 > 0 as libc::c_int {
                        right_0 = middle_0 - 1 as libc::c_int
                    } else { left_0 = middle_0 + 1 as libc::c_int }
                }
            } else {
                let mut netpath: [libc::c_char; 1024] = [0; 1024];
                Q_sprintf(netpath.as_mut_ptr(),
                          b"%s%s\x00" as *const u8 as *const libc::c_char,
                          (*search).filename.as_mut_ptr(), name);
                if FS_SysFileExists(netpath.as_mut_ptr(),
                                    ((*search).flags as libc::c_uint &
                                         (1 as libc::c_uint) <<
                                             3 as libc::c_int == 0) as
                                        libc::c_int as qboolean) as u64 != 0 {
                    if !index.is_null() { *index = -(1 as libc::c_int) }
                    return search
                }
            }
        }
        search = (*search).next
    }
    if fs_ext_path as u64 != 0 {
        let mut netpath_0: [libc::c_char; 1024] = [0; 1024];
        // 0
        search = &mut fs_directpath;
        memset(search as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<searchpath_t>() as libc::c_ulong);
        Q_strncpy((*search).filename.as_mut_ptr(), host.rootdir.as_mut_ptr(),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        Q_strncat((*search).filename.as_mut_ptr(),
                  b"/\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
        Q_snprintf(netpath_0.as_mut_ptr(), 1024 as libc::c_int as size_t,
                   b"%s%s\x00" as *const u8 as *const libc::c_char,
                   (*search).filename.as_mut_ptr(), name);
        if FS_SysFileExists(netpath_0.as_mut_ptr(),
                            ((*search).flags as libc::c_uint &
                                 (1 as libc::c_uint) << 3 as libc::c_int == 0)
                                as libc::c_int as qboolean) as u64 != 0 {
            if !index.is_null() { *index = -(1 as libc::c_int) }
            return search
        }
    }
    if !index.is_null() { *index = -(1 as libc::c_int) }
    return 0 as *mut searchpath_t;
}
// clear searchpath
// root folder has a more priority than netpath
/*
===========
FS_OpenReadFile

Look for a file in the search paths and open it in read-only mode
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_OpenReadFile(mut filename: *const libc::c_char,
                                         mut mode: *const libc::c_char,
                                         mut gamedironly: qboolean)
 -> *mut file_t {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pack_ind: libc::c_int = 0;
    search = FS_FindFile(filename, &mut pack_ind, gamedironly);
    // not found?
    if search.is_null() {
        return 0 as *mut file_t
    } // let W_LoadFile get lump correctly
    if !(*search).pack.is_null() {
        return FS_OpenPackedFile((*search).pack, pack_ind)
    } else {
        if !(*search).wad.is_null() {
            return 0 as *mut file_t
        } else {
            if !(*search).zip.is_null() {
                return FS_OpenZipFile((*search).zip, pack_ind)
            } else {
                if pack_ind < 0 as libc::c_int {
                    let mut path: [libc::c_char; 1024] = [0; 1024];
                    // found in the filesystem?
                    Q_sprintf(path.as_mut_ptr(),
                              b"%s%s\x00" as *const u8 as *const libc::c_char,
                              (*search).filename.as_mut_ptr(), filename);
                    return FS_SysOpen(path.as_mut_ptr(), mode)
                }
            }
        }
    }
    return 0 as *mut file_t;
}
/*
=============================================================================

MAIN PUBLIC FUNCTIONS

=============================================================================
*/
/*
====================
FS_Open

Open a file. The syntax is the same as fopen
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Open(mut filepath: *const libc::c_char,
                                 mut mode: *const libc::c_char,
                                 mut gamedironly: qboolean) -> *mut file_t {
    // some stupid mappers used leading '/' or '\' in path to models or sounds
    if *filepath.offset(0 as libc::c_int as isize) as libc::c_int ==
           '/' as i32 ||
           *filepath.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\\' as i32 {
        filepath = filepath.offset(1)
    }
    if *filepath.offset(0 as libc::c_int as isize) as libc::c_int ==
           '/' as i32 ||
           *filepath.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\\' as i32 {
        filepath = filepath.offset(1)
    }
    if FS_CheckNastyPath(filepath, false_0) != 0 { return 0 as *mut file_t }
    // if the file is opened in "write", "append", or "read/write" mode
    if *mode.offset(0 as libc::c_int as isize) as libc::c_int == 'w' as i32 ||
           *mode.offset(0 as libc::c_int as isize) as libc::c_int ==
               'a' as i32 ||
           *mode.offset(0 as libc::c_int as isize) as libc::c_int ==
               'e' as i32 ||
           !Q_strchr(mode, '+' as i32 as libc::c_char).is_null() {
        let mut real_path: [libc::c_char; 1024] = [0; 1024];
        // open the file on disk directly
        Q_sprintf(real_path.as_mut_ptr(),
                  b"%s/%s\x00" as *const u8 as *const libc::c_char,
                  fs_writedir.as_mut_ptr(),
                  filepath); // Create directories up to the file
        FS_CreatePath(real_path.as_mut_ptr());
        return FS_SysOpen(real_path.as_mut_ptr(), mode)
    }
    // else, we look at the various search paths and open the file in read-only mode
    return FS_OpenReadFile(filepath, mode, gamedironly);
}
/*
====================
FS_Close

Close a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Close(mut file: *mut file_t) -> libc::c_int {
    if file.is_null() { return 0 as libc::c_int }
    FS_BackupFileName(file, 0 as *const libc::c_char,
                      0 as libc::c_int as uint);
    if (*file).handle >= 0 as libc::c_int {
        if close((*file).handle) != 0 { return -(1 as libc::c_int) }
    }
    _Mem_Free(file as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 2750 as libc::c_int);
    return 0 as libc::c_int;
}
/*
====================
FS_Write

Write "datasize" bytes into a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Write(mut file: *mut file_t,
                                  mut data: *const libc::c_void,
                                  mut datasize: size_t) -> fs_offset_t {
    let mut result: fs_offset_t = 0;
    if file.is_null() { return 0 as libc::c_int as fs_offset_t }
    // if necessary, seek to the exact file position we're supposed to be
    if (*file).buff_ind != (*file).buff_len {
        lseek((*file).handle, (*file).buff_ind - (*file).buff_len,
              1 as libc::c_int);
    }
    // purge cached data
    FS_Purge(file);
    // write the buffer and update the position
    result = write((*file).handle, data, datasize as fs_offset_t as size_t);
    (*file).position =
        lseek((*file).handle, 0 as libc::c_int as __off_t, 1 as libc::c_int);
    if (*file).real_length < (*file).position {
        (*file).real_length = (*file).position
    }
    if result < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as fs_offset_t
    }
    return result;
}
/*
====================
FS_Read

Read up to "buffersize" bytes from a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Read(mut file: *mut file_t,
                                 mut buffer: *mut libc::c_void,
                                 mut buffersize: size_t) -> fs_offset_t {
    let mut count: fs_offset_t = 0;
    let mut done: fs_offset_t = 0;
    let mut nb: fs_offset_t = 0;
    // nothing to copy
    if buffersize == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int as fs_offset_t
    }
    // Get rid of the ungetc character
    if (*file).ungetc != -(1 as libc::c_int) {
        *(buffer as *mut libc::c_char).offset(0 as libc::c_int as isize) =
            (*file).ungetc as libc::c_char;
        buffersize = buffersize.wrapping_sub(1);
        (*file).ungetc = -(1 as libc::c_int);
        done = 1 as libc::c_int as fs_offset_t
    } else { done = 0 as libc::c_int as fs_offset_t }
    // first, we copy as many bytes as we can from "buff"
    if (*file).buff_ind < (*file).buff_len {
        count = (*file).buff_len - (*file).buff_ind;
        done +=
            if buffersize as fs_offset_t > count {
                count
            } else { buffersize as fs_offset_t };
        memcpy(buffer,
               &mut *(*file).buff.as_mut_ptr().offset((*file).buff_ind as
                                                          isize) as *mut byte
                   as *const libc::c_void, done as libc::c_ulong);
        (*file).buff_ind += done;
        buffersize =
            (buffersize as libc::c_ulong).wrapping_sub(done as libc::c_ulong)
                as size_t as size_t;
        if buffersize == 0 as libc::c_int as libc::c_ulong { return done }
    }
    // NOTE: at this point, the read buffer is always empty
    FS_EnsureOpenFile(file);
    // we must take care to not read after the end of the file
    count = (*file).real_length - (*file).position;
    // if we have a lot of data to get, put them directly into "buffer"
    if buffersize >
           (::std::mem::size_of::<[byte; 2048]>() as
                libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
       {
        if count > buffersize as fs_offset_t {
            count = buffersize as fs_offset_t
        }
        lseek((*file).handle, (*file).offset + (*file).position,
              0 as libc::c_int);
        nb =
            read((*file).handle,
                 &mut *(buffer as *mut byte).offset(done as isize) as
                     *mut byte as *mut libc::c_void, count as size_t);
        if nb > 0 as libc::c_int as libc::c_long {
            done += nb;
            (*file).position += nb;
            // purge cached data
            FS_Purge(file);
        }
    } else {
        if count >
               ::std::mem::size_of::<[byte; 2048]>() as libc::c_ulong as
                   fs_offset_t {
            count =
                ::std::mem::size_of::<[byte; 2048]>() as libc::c_ulong as
                    fs_offset_t
        }
        lseek((*file).handle, (*file).offset + (*file).position,
              0 as libc::c_int);
        nb =
            read((*file).handle,
                 (*file).buff.as_mut_ptr() as *mut libc::c_void,
                 count as size_t);
        if nb > 0 as libc::c_int as libc::c_long {
            (*file).buff_len = nb;
            (*file).position += nb;
            // copy the requested data in "buffer" (as much as we can)
            count =
                if buffersize as fs_offset_t > (*file).buff_len {
                    (*file).buff_len
                } else { buffersize as fs_offset_t };
            memcpy(&mut *(buffer as *mut byte).offset(done as isize) as
                       *mut byte as *mut libc::c_void,
                   (*file).buff.as_mut_ptr() as *const libc::c_void,
                   count as libc::c_ulong);
            (*file).buff_ind = count;
            done += count
        }
    }
    return done;
}
/*
====================
FS_Print

Print a string into a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Print(mut file: *mut file_t,
                                  mut msg: *const libc::c_char)
 -> libc::c_int {
    return FS_Write(file, msg as *const libc::c_void, Q_strlen(msg)) as
               libc::c_int;
}
/*
====================
FS_Printf

Print a string into a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Printf(mut file: *mut file_t,
                                   mut format: *const libc::c_char,
                                   mut args: ...) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    result = FS_VPrintf(file, format, args_0.as_va_list());
    return result;
}
/*
====================
FS_VPrintf

Print a string into a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_VPrintf(mut file: *mut file_t,
                                    mut format: *const libc::c_char,
                                    mut ap: ::std::ffi::VaList)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut buff_size: fs_offset_t = 1024 as libc::c_int as fs_offset_t;
    let mut tempbuff: *mut libc::c_char = 0 as *mut libc::c_char;
    if file.is_null() { return 0 as libc::c_int }
    loop  {
        tempbuff =
            _Mem_Alloc(fs_mempool, buff_size as size_t, false_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 2918 as libc::c_int) as
                *mut libc::c_char;
        len =
            Q_vsnprintf(tempbuff, 99999 as libc::c_int as size_t, format,
                        ap.as_va_list());
        if len >= 0 as libc::c_int && (len as libc::c_long) < buff_size {
            break ;
        }
        _Mem_Free(tempbuff as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 2924 as libc::c_int);
        buff_size *= 2 as libc::c_int as libc::c_long
    }
    len =
        write((*file).handle, tempbuff as *const libc::c_void, len as size_t)
            as libc::c_int;
    _Mem_Free(tempbuff as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 2929 as libc::c_int);
    return len;
}
/*
====================
FS_Getc

Get the next character of a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Getc(mut file: *mut file_t) -> libc::c_int {
    let mut c: libc::c_char = 0;
    if FS_Read(file, &mut c as *mut libc::c_char as *mut libc::c_void,
               1 as libc::c_int as size_t) != 1 as libc::c_int as libc::c_long
       {
        return -(1 as libc::c_int)
    }
    return c as libc::c_int;
}
/*
====================
FS_UnGetc

Put a character back into the read buffer (only supports one character!)
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_UnGetc(mut file: *mut file_t, mut c: byte)
 -> libc::c_int {
    // If there's already a character waiting to be read
    if (*file).ungetc != -(1 as libc::c_int) { return -(1 as libc::c_int) }
    (*file).ungetc = c as libc::c_int;
    return c as libc::c_int;
}
/*
====================
FS_Gets

Same as fgets
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Gets(mut file: *mut file_t, mut string: *mut byte,
                                 mut bufsize: size_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut end: libc::c_int = 0 as libc::c_int;
    loop  {
        c = FS_Getc(file);
        if c == '\r' as i32 || c == '\n' as i32 || c < 0 as libc::c_int {
            break ;
        }
        if (end as libc::c_ulong) <
               bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let fresh132 = end;
            end = end + 1;
            *string.offset(fresh132 as isize) = c as byte
        }
    }
    *string.offset(end as isize) = 0 as libc::c_int as byte;
    // remove \n following \r
    if c == '\r' as i32 {
        c = FS_Getc(file);
        if c != '\n' as i32 { FS_UnGetc(file, c as byte); }
    }
    return c;
}
/*
====================
FS_Seek

Move the position index in a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Seek(mut file: *mut file_t,
                                 mut offset: fs_offset_t,
                                 mut whence: libc::c_int) -> libc::c_int {
    // compute the file offset
    match whence {
        1 => {
            offset += (*file).position - (*file).buff_len + (*file).buff_ind
        }
        0 => { }
        2 => { offset += (*file).real_length }
        _ => { return -(1 as libc::c_int) }
    }
    if offset < 0 as libc::c_int as libc::c_long ||
           offset > (*file).real_length {
        return -(1 as libc::c_int)
    }
    // if we have the data in our read buffer, we don't need to actually seek
    if (*file).position - (*file).buff_len <= offset &&
           offset <= (*file).position {
        (*file).buff_ind = offset + (*file).buff_len - (*file).position;
        return 0 as libc::c_int
    }
    FS_EnsureOpenFile(file);
    // Purge cached data
    FS_Purge(file);
    if lseek((*file).handle, (*file).offset + offset, 0 as libc::c_int) ==
           -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int)
    }
    (*file).position = offset;
    return 0 as libc::c_int;
}
/*
====================
FS_Tell

Give the current position in a file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Tell(mut file: *mut file_t) -> fs_offset_t {
    if file.is_null() { return 0 as libc::c_int as fs_offset_t }
    return (*file).position - (*file).buff_len + (*file).buff_ind;
}
/*
====================
FS_Eof

indicates at reached end of file
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Eof(mut file: *mut file_t) -> qboolean {
    if file.is_null() { return true_0 }
    return if (*file).position - (*file).buff_len + (*file).buff_ind ==
                  (*file).real_length {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
====================
FS_Purge

Erases any buffered input or output data
====================
*/
unsafe extern "C" fn FS_Purge(mut file: *mut file_t) {
    (*file).buff_len = 0 as libc::c_int as fs_offset_t;
    (*file).buff_ind = 0 as libc::c_int as fs_offset_t;
    (*file).ungetc = -(1 as libc::c_int);
}
/*
============
FS_LoadFile

Filename are relative to the xash directory.
Always appends a 0 byte.
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_LoadFile(mut path: *const libc::c_char,
                                     mut filesizeptr: *mut fs_offset_t,
                                     mut gamedironly: qboolean) -> *mut byte {
    let mut file: *mut file_t = 0 as *mut file_t;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut filesize: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    file =
        FS_Open(path, b"rb\x00" as *const u8 as *const libc::c_char,
                gamedironly);
    if !file.is_null() {
        filesize = (*file).real_length;
        buf =
            _Mem_Alloc(fs_mempool,
                       (filesize + 1 as libc::c_int as libc::c_long) as
                           size_t, false_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 3108 as libc::c_int) as
                *mut byte;
        *buf.offset(filesize as isize) = '\u{0}' as i32 as byte;
        FS_Read(file, buf as *mut libc::c_void, filesize as size_t);
        FS_Close(file);
    } else {
        buf = W_LoadFile(path, &mut filesize, gamedironly);
        if buf.is_null() {
            buf = Zip_LoadFile(path, &mut filesize, gamedironly)
        }
    }
    if !filesizeptr.is_null() { *filesizeptr = filesize }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn CRC32_File(mut crcvalue: *mut dword,
                                    mut filename: *const libc::c_char)
 -> qboolean {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut num_bytes: libc::c_int = 0;
    let mut f: *mut file_t = 0 as *mut file_t;
    f =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if f.is_null() { return false_0 }
    CRC32_Init(crcvalue);
    loop  {
        num_bytes =
            FS_Read(f, buffer.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong) as libc::c_int;
        if num_bytes > 0 as libc::c_int {
            CRC32_ProcessBuffer(crcvalue,
                                buffer.as_mut_ptr() as *const libc::c_void,
                                num_bytes);
        }
        if FS_Eof(f) as u64 != 0 { break ; }
    }
    FS_Close(f);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn MD5_HashFile(mut digest: *mut byte,
                                      mut pszFileName: *const libc::c_char,
                                      mut seed: *mut uint) -> qboolean {
    let mut file: *mut file_t = 0 as *mut file_t;
    let mut buffer: [byte; 1024] = [0; 1024];
    let mut MD5_Hash: MD5Context_t =
        MD5Context_t{buf: [0; 4], bits: [0; 2], in_0: [0; 16],};
    let mut bytes: libc::c_int = 0;
    file =
        FS_Open(pszFileName, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if file.is_null() { return false_0 }
    memset(&mut MD5_Hash as *mut MD5Context_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<MD5Context_t>() as libc::c_ulong);
    MD5Init(&mut MD5_Hash);
    if !seed.is_null() {
        MD5Update(&mut MD5_Hash, seed as *const byte,
                  16 as libc::c_int as uint);
    }
    loop  {
        bytes =
            FS_Read(file, buffer.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong) as
                libc::c_int;
        if bytes > 0 as libc::c_int {
            MD5Update(&mut MD5_Hash, buffer.as_mut_ptr(), bytes as uint);
        }
        if FS_Eof(file) as u64 != 0 { break ; }
    }
    FS_Close(file);
    MD5Final(digest, &mut MD5_Hash);
    return true_0;
}
/*
============
FS_LoadFile

Filename are relative to the xash directory.
Always appends a 0 byte.
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_LoadDirectFile(mut path: *const libc::c_char,
                                           mut filesizeptr: *mut fs_offset_t)
 -> *mut byte {
    let mut file: *mut file_t = 0 as *mut file_t;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut filesize: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    file = FS_SysOpen(path, b"rb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() { return 0 as *mut byte }
    // Try to load
    filesize = (*file).real_length;
    buf =
        _Mem_Alloc(fs_mempool,
                   (filesize + 1 as libc::c_int as libc::c_long) as size_t,
                   false_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 3213 as libc::c_int) as *mut byte;
    *buf.offset(filesize as isize) = '\u{0}' as i32 as byte;
    FS_Read(file, buf as *mut libc::c_void, filesize as size_t);
    FS_Close(file);
    if !filesizeptr.is_null() { *filesizeptr = filesize }
    return buf;
}
/*
============
FS_WriteFile

The filename will be prefixed by the current game directory
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_WriteFile(mut filename: *const libc::c_char,
                                      mut data: *const libc::c_void,
                                      mut len: fs_offset_t) -> qboolean {
    let mut file: *mut file_t = 0 as *mut file_t;
    file =
        FS_Open(filename, b"wb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if file.is_null() {
        Con_Reportf(b"^1Error:^7 FS_WriteFile: failed on %s\n\x00" as
                        *const u8 as *const libc::c_char, filename);
        return false_0
    }
    FS_Write(file, data, len as size_t);
    FS_Close(file);
    return true_0;
}
/*
=============================================================================

OTHERS PUBLIC FUNCTIONS

=============================================================================
*/
/*
==================
FS_FileExists

Look for a file in the packages and in the filesystem
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FileExists(mut filename: *const libc::c_char,
                                       mut gamedironly: libc::c_int)
 -> libc::c_int {
    if !FS_FindFile(filename, 0 as *mut libc::c_int,
                    gamedironly as qboolean).is_null() {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
/*
==================
FS_GetDiskPath

Build direct path for file in the filesystem
return NULL for file in pack
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_GetDiskPath(mut name: *const libc::c_char,
                                        mut gamedironly: qboolean)
 -> *const libc::c_char {
    let mut index: libc::c_int = 0;
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    search = FS_FindFile(name, &mut index, gamedironly);
    if !search.is_null() {
        if index != -(1 as libc::c_int) {
            // file in pack or wad
            return 0 as *const libc::c_char
        }
        return va(b"%s%s\x00" as *const u8 as *const libc::c_char,
                  (*search).filename.as_mut_ptr(), name)
    }
    return 0 as *const libc::c_char;
}
/*
==================
FS_CheckForCrypt

return true if library is crypted
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_CheckForCrypt(mut dllname: *const libc::c_char)
 -> qboolean {
    let mut f: *mut file_t = 0 as *mut file_t; // skip first 64 bytes
    let mut key: libc::c_int = 0;
    f =
        FS_Open(dllname, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if f.is_null() { return false_0 }
    FS_Seek(f, 64 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut key as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Close(f);
    return if key == 0x12345678 as libc::c_int {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
==================
FS_FindLibrary

search for library, assume index is valid
only for internal use
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FindLibrary(mut dllname: *const libc::c_char,
                                        mut directpath: qboolean)
 -> *mut dll_user_t {
    let mut dllpath: string = [0; 256];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut hInst: *mut dll_user_t = 0 as *mut dll_user_t;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    // check for bad exports
    if if dllname.is_null() || *dllname == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as *mut dll_user_t
    }
    fs_ext_path = directpath;
    // HACKHACK remove absoulte path to valve folder
    if Q_strnicmp(dllname,
                  b"..\\valve\\\x00" as *const u8 as *const libc::c_char,
                  9 as libc::c_int) == 0 ||
           Q_strnicmp(dllname,
                      b"../valve/\x00" as *const u8 as *const libc::c_char,
                      9 as libc::c_int) == 0 {
        start += 9 as libc::c_int
    }
    // replace all backward slashes
    len = Q_strlen(dllname) as libc::c_int; // apply ext if forget
    i = 0 as libc::c_int;
    while i < len {
        if *dllname.offset((i + start) as isize) as libc::c_int == '\\' as i32
           {
            dllpath[i as usize] = '/' as i32 as libc::c_char
        } else {
            dllpath[i as usize] =
                Q_tolower(*dllname.offset((i + start) as isize))
        }
        i += 1
    }
    dllpath[i as usize] = '\u{0}' as i32 as libc::c_char;
    COM_DefaultExtension(dllpath.as_mut_ptr(),
                         b".so\x00" as *const u8 as *const libc::c_char);
    search = FS_FindFile(dllpath.as_mut_ptr(), &mut index, false_0);
    if search.is_null() && directpath as u64 == 0 {
        fs_ext_path = false_0;
        // unable to find
        Q_strncpy(dllpath.as_mut_ptr(), dllname,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        search = FS_FindFile(dllpath.as_mut_ptr(), &mut index, false_0);
        if search.is_null() { return 0 as *mut dll_user_t }
    }
    // trying check also 'bin' folder for indirect paths
    // NOTE: for libraries we not fail even if search is NULL
	// let the OS find library himself
    hInst =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<dll_user_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 3370 as libc::c_int) as
            *mut dll_user_t;
    // save dllname for debug purposes
    Q_strncpy((*hInst).dllName.as_mut_ptr(), dllname,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    // shortPath is used for LibraryLoadSymbols only
    Q_strncpy((*hInst).shortPath.as_mut_ptr(), dllpath.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    (*hInst).encrypted = FS_CheckForCrypt(dllpath.as_mut_ptr());
    if index < 0 as libc::c_int && (*hInst).encrypted as u64 == 0 &&
           !search.is_null() {
        Q_snprintf((*hInst).fullPath.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s%s\x00" as *const u8 as *const libc::c_char,
                   (*search).filename.as_mut_ptr(), dllpath.as_mut_ptr());
        (*hInst).custom_loader = false_0
        // we can loading from disk and use normal debugging
    } else {
        // NOTE: if search is NULL let the OS found library himself
        Q_strncpy((*hInst).fullPath.as_mut_ptr(), dllpath.as_mut_ptr(),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        if !search.is_null() &&
               (!(*search).wad.is_null() || !(*search).pack.is_null() ||
                    !(*search).zip.is_null()) {
            // a1ba: custom loader is non-portable (I just don't want to touch it)
            Con_Printf(b"^3Warning:^7 %s: loading libraries from packs is unsupported on this platform\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*::std::mem::transmute::<&[u8; 15],
                                                 &[libc::c_char; 15]>(b"FS_FindLibrary\x00")).as_ptr()); // always reset direct paths
            (*hInst).custom_loader = false_0
        } else { (*hInst).custom_loader = false_0 }
    }
    fs_ext_path = false_0;
    return hInst;
}
/*
==================
FS_FileSize

return size of file in bytes
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FileSize(mut filename: *const libc::c_char,
                                     mut gamedironly: qboolean)
 -> fs_offset_t {
    let mut length: libc::c_int =
        -(1 as libc::c_int); // in case file was missed
    let mut fp: *mut file_t = 0 as *mut file_t;
    fp =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                gamedironly);
    if !fp.is_null() {
        // it exists
        FS_Seek(fp, 0 as libc::c_int as fs_offset_t, 2 as libc::c_int);
        length = FS_Tell(fp) as libc::c_int;
        FS_Close(fp);
    }
    return length as fs_offset_t;
}
/*
==================
FS_FileLength

return size of file in bytes
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FileLength(mut f: *mut file_t) -> fs_offset_t {
    if f.is_null() { return 0 as libc::c_int as fs_offset_t }
    return (*f).real_length;
}
/*
==================
FS_FileTime

return time of creation file in seconds
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FileTime(mut filename: *const libc::c_char,
                                     mut gamedironly: qboolean)
 -> libc::c_int {
    let mut search: *mut searchpath_t =
        0 as *mut searchpath_t; // doesn't exist
    let mut pack_ind: libc::c_int = 0;
    search = FS_FindFile(filename, &mut pack_ind, gamedironly);
    if search.is_null() { return -(1 as libc::c_int) }
    if !(*search).pack.is_null() {
        // grab pack filetime
        return (*(*search).pack).filetime as libc::c_int
    } else {
        if !(*search).wad.is_null() {
            // grab wad filetime
            return (*(*search).wad).filetime as libc::c_int
        } else {
            if !(*search).zip.is_null() {
                return (*(*search).zip).filetime as libc::c_int
            } else {
                if pack_ind < 0 as libc::c_int {
                    // found in the filesystem?
                    let mut path: [libc::c_char; 1024] = [0; 1024];
                    Q_sprintf(path.as_mut_ptr(),
                              b"%s%s\x00" as *const u8 as *const libc::c_char,
                              (*search).filename.as_mut_ptr(), filename);
                    return FS_SysFileTime(path.as_mut_ptr())
                }
            }
        }
    }
    return -(1 as libc::c_int);
    // doesn't exist
}
/*
==================
FS_Rename

rename specified file from gamefolder
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Rename(mut oldname: *const libc::c_char,
                                   mut newname: *const libc::c_char)
 -> qboolean {
    let mut oldpath: [libc::c_char; 1024] = [0; 1024];
    let mut newpath: [libc::c_char; 1024] = [0; 1024];
    let mut iRet: qboolean = false_0;
    if oldname.is_null() || newname.is_null() || *oldname == 0 ||
           *newname == 0 {
        return false_0
    }
    Q_snprintf(oldpath.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
               b"%s%s\x00" as *const u8 as *const libc::c_char,
               fs_writedir.as_mut_ptr(), oldname);
    Q_snprintf(newpath.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
               b"%s%s\x00" as *const u8 as *const libc::c_char,
               fs_writedir.as_mut_ptr(), newname);
    COM_FixSlashes(oldpath.as_mut_ptr());
    COM_FixSlashes(newpath.as_mut_ptr());
    iRet = rename(oldpath.as_mut_ptr(), newpath.as_mut_ptr()) as qboolean;
    return (iRet as libc::c_uint == 0 as libc::c_int as libc::c_uint) as
               libc::c_int as qboolean;
}
/*
==================
FS_Delete

delete specified file from gamefolder
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Delete(mut path: *const libc::c_char)
 -> qboolean {
    let mut real_path: [libc::c_char; 1024] = [0; 1024];
    let mut iRet: qboolean = false_0;
    if path.is_null() || *path == 0 { return false_0 }
    Q_snprintf(real_path.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
               b"%s%s\x00" as *const u8 as *const libc::c_char,
               fs_writedir.as_mut_ptr(), path);
    COM_FixSlashes(real_path.as_mut_ptr());
    iRet = remove(real_path.as_mut_ptr()) as qboolean;
    return (iRet as libc::c_uint == 0 as libc::c_int as libc::c_uint) as
               libc::c_int as qboolean;
}
/*
==================
FS_FileCopy

==================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FileCopy(mut pOutput: *mut file_t,
                                     mut pInput: *mut file_t,
                                     mut fileSize: libc::c_int) -> qboolean {
    let mut buf: *mut libc::c_char =
        _Mem_Alloc(fs_mempool,
                   (1024 as libc::c_int * 1024 as libc::c_int) as size_t,
                   false_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 3539 as libc::c_int) as
            *mut libc::c_char;
    let mut size: libc::c_int = 0;
    let mut readSize: libc::c_int = 0;
    let mut done: qboolean = true_0;
    while fileSize > 0 as libc::c_int {
        if fileSize > 1024 as libc::c_int * 1024 as libc::c_int {
            size = 1024 as libc::c_int * 1024 as libc::c_int
        } else { size = fileSize }
        readSize =
            FS_Read(pInput, buf as *mut libc::c_void, size as size_t) as
                libc::c_int;
        if readSize < size {
            Con_Reportf(b"^1Error:^7 FS_FileCopy: unexpected end of input file (%d < %d)\n\x00"
                            as *const u8 as *const libc::c_char, readSize,
                        size);
            fileSize = 0 as libc::c_int;
            done = false_0;
            break ;
        } else {
            FS_Write(pOutput, buf as *const libc::c_void, readSize as size_t);
            fileSize -= size
        }
    }
    _Mem_Free(buf as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 3561 as libc::c_int);
    return done;
}
/*
===========
FS_Search

Allocate and fill a search structure with information on matching filenames.
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Search(mut pattern: *const libc::c_char,
                                   mut caseinsensitive: libc::c_int,
                                   mut gamedironly: libc::c_int)
 -> *mut search_t {
    let mut search: *mut search_t = 0 as *mut search_t; // punctuation issues
    let mut searchpath: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut wad: *mut wfile_t = 0 as *mut wfile_t;
    let mut i: libc::c_int = 0;
    let mut basepathlength: libc::c_int = 0;
    let mut numfiles: libc::c_int = 0;
    let mut numchars: libc::c_int = 0;
    let mut resultlistindex: libc::c_int = 0;
    let mut dirlistindex: libc::c_int = 0;
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    let mut backslash: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut netpath: string = [0; 256];
    let mut temp: string = [0; 256];
    let mut resultlist: stringlist_t =
        stringlist_t{maxstrings: 0,
                     numstrings: 0,
                     strings: 0 as *mut *mut libc::c_char,};
    let mut dirlist: stringlist_t =
        stringlist_t{maxstrings: 0,
                     numstrings: 0,
                     strings: 0 as *mut *mut libc::c_char,};
    let mut basepath: *mut libc::c_char = 0 as *mut libc::c_char;
    if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
           ||
           *pattern.offset(0 as libc::c_int as isize) as libc::c_int ==
               ':' as i32 ||
           *pattern.offset(0 as libc::c_int as isize) as libc::c_int ==
               '/' as i32 ||
           *pattern.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\\' as i32 {
        return 0 as *mut search_t
    }
    stringlistinit(&mut resultlist);
    stringlistinit(&mut dirlist);
    slash = Q_strrchr(pattern, '/' as i32 as libc::c_char);
    backslash = Q_strrchr(pattern, '\\' as i32 as libc::c_char);
    colon = Q_strrchr(pattern, ':' as i32 as libc::c_char);
    separator = if slash > backslash { slash } else { backslash };
    separator = if separator > colon { separator } else { colon };
    basepathlength =
        if !separator.is_null() {
            separator.offset(1 as libc::c_int as
                                 isize).wrapping_offset_from(pattern) as
                libc::c_long
        } else { 0 as libc::c_int as libc::c_long } as libc::c_int;
    basepath =
        _Mem_Alloc(fs_mempool, (basepathlength + 1 as libc::c_int) as size_t,
                   true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 3597 as libc::c_int) as
            *mut libc::c_char;
    if basepathlength != 0 {
        memcpy(basepath as *mut libc::c_void, pattern as *const libc::c_void,
               basepathlength as libc::c_ulong);
    }
    *basepath.offset(basepathlength as isize) =
        0 as libc::c_int as libc::c_char;
    // search through the path, one element at a time
    searchpath = fs_searchpaths;
    while !searchpath.is_null() {
        if !(gamedironly != 0 &&
                 (*searchpath).flags as libc::c_uint &
                     ((1 as libc::c_uint) << 2 as libc::c_int |
                          (1 as libc::c_uint) << 3 as libc::c_int |
                          (1 as libc::c_uint) << 4 as libc::c_int) == 0) {
            // is the element a pak file?
            if !(*searchpath).pack.is_null() {
                // look through all the pak file elements
                pak = (*searchpath).pack;
                i = 0 as libc::c_int;
                while i < (*pak).numfiles {
                    Q_strncpy(temp.as_mut_ptr(),
                              (*(*pak).files.offset(i as
                                                        isize)).name.as_mut_ptr(),
                              ::std::mem::size_of::<string>() as
                                  libc::c_ulong);
                    while temp[0 as libc::c_int as usize] != 0 {
                        if matchpattern(temp.as_mut_ptr(),
                                        pattern as *mut libc::c_char, true_0)
                               != 0 {
                            resultlistindex = 0 as libc::c_int;
                            while resultlistindex < resultlist.numstrings {
                                if Q_strncmp(*resultlist.strings.offset(resultlistindex
                                                                            as
                                                                            isize),
                                             temp.as_mut_ptr(),
                                             99999 as libc::c_int) == 0 {
                                    break ;
                                }
                                resultlistindex += 1
                            }
                            if resultlistindex == resultlist.numstrings {
                                stringlistappend(&mut resultlist,
                                                 temp.as_mut_ptr());
                            }
                        }
                        // strip off one path element at a time until empty
					// this way directories are added to the listing if they match the pattern
                        slash =
                            Q_strrchr(temp.as_mut_ptr(),
                                      '/' as i32 as libc::c_char);
                        backslash =
                            Q_strrchr(temp.as_mut_ptr(),
                                      '\\' as i32 as libc::c_char);
                        colon =
                            Q_strrchr(temp.as_mut_ptr(),
                                      ':' as i32 as libc::c_char);
                        separator = temp.as_mut_ptr();
                        if separator < slash { separator = slash }
                        if separator < backslash { separator = backslash }
                        if separator < colon { separator = colon }
                        *(separator as *mut libc::c_char) =
                            0 as libc::c_int as libc::c_char
                    }
                    i += 1
                }
            } else if !(*searchpath).wad.is_null() {
                let mut wadpattern: string = [0; 256];
                let mut wadname: string = [0; 256];
                let mut temp2: string = [0; 256];
                let mut type_0: libc::c_schar = W_TypeFromExt(pattern);
                let mut anywadname: qboolean = true_0;
                let mut wadfolder: string = [0; 256];
                // quick reject by filetype
                if !(type_0 as libc::c_int == 0 as libc::c_int) {
                    COM_ExtractFilePath(pattern, wadname.as_mut_ptr());
                    COM_FileBase(pattern, wadpattern.as_mut_ptr());
                    wadfolder[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    if if *wadname.as_mut_ptr() == 0 {
                           0 as libc::c_int
                       } else { 1 as libc::c_int } != 0 {
                        COM_FileBase(wadname.as_mut_ptr(),
                                     wadname.as_mut_ptr());
                        Q_strncpy(wadfolder.as_mut_ptr(),
                                  wadname.as_mut_ptr(),
                                  ::std::mem::size_of::<string>() as
                                      libc::c_ulong);
                        COM_DefaultExtension(wadname.as_mut_ptr(),
                                             b".wad\x00" as *const u8 as
                                                 *const libc::c_char);
                        anywadname = false_0
                    }
                    // make wadname from wad fullpath
                    COM_FileBase((*(*searchpath).wad).filename.as_mut_ptr(),
                                 temp2.as_mut_ptr());
                    COM_DefaultExtension(temp2.as_mut_ptr(),
                                         b".wad\x00" as *const u8 as
                                             *const libc::c_char);
                    // quick reject by wadname
                    if !(anywadname as u64 == 0 &&
                             Q_strnicmp(wadname.as_mut_ptr(),
                                        temp2.as_mut_ptr(),
                                        99999 as libc::c_int) != 0) {
                        // look through all the wad file elements
                        wad = (*searchpath).wad;
                        i = 0 as libc::c_int;
                        while i < (*wad).numlumps {
                            // if type not matching, we already have no chance ...
                            if !(type_0 as libc::c_int != -(1 as libc::c_int)
                                     &&
                                     (*(*wad).lumps.offset(i as isize)).type_0
                                         as libc::c_int !=
                                         type_0 as libc::c_int) {
                                // build the lumpname with image suffix (if present)
                                Q_strncpy(temp.as_mut_ptr(),
                                          (*(*wad).lumps.offset(i as
                                                                    isize)).name.as_mut_ptr(),
                                          ::std::mem::size_of::<string>() as
                                              libc::c_ulong);
                                while temp[0 as libc::c_int as usize] != 0 {
                                    if matchpattern(temp.as_mut_ptr(),
                                                    wadpattern.as_mut_ptr(),
                                                    true_0) != 0 {
                                        resultlistindex = 0 as libc::c_int;
                                        while resultlistindex <
                                                  resultlist.numstrings {
                                            if Q_strncmp(*resultlist.strings.offset(resultlistindex
                                                                                        as
                                                                                        isize),
                                                         temp.as_mut_ptr(),
                                                         99999 as libc::c_int)
                                                   == 0 {
                                                break ;
                                            }
                                            resultlistindex += 1
                                        }
                                        if resultlistindex ==
                                               resultlist.numstrings {
                                            // build path: wadname/lumpname.ext
                                            Q_snprintf(temp2.as_mut_ptr(),
                                                       ::std::mem::size_of::<string>()
                                                           as libc::c_ulong,
                                                       b"%s/%s\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       wadfolder.as_mut_ptr(),
                                                       temp.as_mut_ptr());
                                            COM_DefaultExtension(temp2.as_mut_ptr(),
                                                                 va(b".%s\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    W_ExtFromType((*(*wad).lumps.offset(i
                                                                                                            as
                                                                                                            isize)).type_0)));
                                            stringlistappend(&mut resultlist,
                                                             temp2.as_mut_ptr());
                                        }
                                    }
                                    // strip off one path element at a time until empty
					// this way directories are added to the listing if they match the pattern
                                    slash =
                                        Q_strrchr(temp.as_mut_ptr(),
                                                  '/' as i32 as libc::c_char);
                                    backslash =
                                        Q_strrchr(temp.as_mut_ptr(),
                                                  '\\' as i32 as
                                                      libc::c_char);
                                    colon =
                                        Q_strrchr(temp.as_mut_ptr(),
                                                  ':' as i32 as libc::c_char);
                                    separator = temp.as_mut_ptr();
                                    if separator < slash { separator = slash }
                                    if separator < backslash {
                                        separator = backslash
                                    }
                                    if separator < colon { separator = colon }
                                    *(separator as *mut libc::c_char) =
                                        0 as libc::c_int as libc::c_char
                                }
                            }
                            i += 1
                        }
                    }
                }
            } else {
                // get a directory listing and look at each name
                Q_sprintf(netpath.as_mut_ptr(),
                          b"%s%s\x00" as *const u8 as *const libc::c_char,
                          (*searchpath).filename.as_mut_ptr(), basepath);
                stringlistinit(&mut dirlist);
                listdirectory(&mut dirlist, netpath.as_mut_ptr(),
                              caseinsensitive as qboolean);
                dirlistindex = 0 as libc::c_int;
                while dirlistindex < dirlist.numstrings {
                    Q_sprintf(temp.as_mut_ptr(),
                              b"%s%s\x00" as *const u8 as *const libc::c_char,
                              basepath,
                              *dirlist.strings.offset(dirlistindex as isize));
                    if matchpattern(temp.as_mut_ptr(),
                                    pattern as *mut libc::c_char, true_0) != 0
                       {
                        resultlistindex = 0 as libc::c_int;
                        while resultlistindex < resultlist.numstrings {
                            if Q_strncmp(*resultlist.strings.offset(resultlistindex
                                                                        as
                                                                        isize),
                                         temp.as_mut_ptr(),
                                         99999 as libc::c_int) == 0 {
                                break ;
                            }
                            resultlistindex += 1
                        }
                        if resultlistindex == resultlist.numstrings {
                            stringlistappend(&mut resultlist,
                                             temp.as_mut_ptr());
                        }
                    }
                    dirlistindex += 1
                }
                stringlistfreecontents(&mut dirlist);
            }
        }
        searchpath = (*searchpath).next
    }
    if resultlist.numstrings != 0 {
        stringlistsort(&mut resultlist);
        numfiles = resultlist.numstrings;
        numchars = 0 as libc::c_int;
        resultlistindex = 0 as libc::c_int;
        while resultlistindex < resultlist.numstrings {
            numchars +=
                Q_strlen(*resultlist.strings.offset(resultlistindex as isize))
                    as libc::c_int + 1 as libc::c_int;
            resultlistindex += 1
        }
        search =
            _Mem_Alloc(fs_mempool,
                       (::std::mem::size_of::<search_t>() as
                            libc::c_ulong).wrapping_add(numchars as
                                                            libc::c_ulong).wrapping_add((numfiles
                                                                                             as
                                                                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                                                                             as
                                                                                                                             libc::c_ulong)),
                       true_0,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 3757 as libc::c_int) as
                *mut search_t;
        (*search).filenames =
            (search as
                 *mut libc::c_char).offset(::std::mem::size_of::<search_t>()
                                               as libc::c_ulong as isize) as
                *mut *mut libc::c_char;
        (*search).filenamesbuffer =
            (search as
                 *mut libc::c_char).offset(::std::mem::size_of::<search_t>()
                                               as libc::c_ulong as
                                               isize).offset((numfiles as
                                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as isize);
        (*search).numfilenames = numfiles;
        numchars = 0 as libc::c_int;
        numfiles = numchars;
        resultlistindex = 0 as libc::c_int;
        while resultlistindex < resultlist.numstrings {
            let mut textlen: size_t = 0;
            let ref mut fresh133 =
                *(*search).filenames.offset(numfiles as isize);
            *fresh133 = (*search).filenamesbuffer.offset(numchars as isize);
            textlen =
                Q_strlen(*resultlist.strings.offset(resultlistindex as
                                                        isize)).wrapping_add(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong);
            memcpy(*(*search).filenames.offset(numfiles as isize) as
                       *mut libc::c_void,
                   *resultlist.strings.offset(resultlistindex as isize) as
                       *const libc::c_void, textlen);
            numfiles += 1;
            numchars += textlen as libc::c_int;
            resultlistindex += 1
        }
    }
    stringlistfreecontents(&mut resultlist);
    _Mem_Free(basepath as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 3777 as libc::c_int);
    return search;
}
unsafe extern "C" fn FS_InitMemory() {
    fs_mempool =
        _Mem_AllocPool(b"FileSystem Pool\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 3784 as libc::c_int);
    fs_searchpaths = 0 as *mut searchpath_t;
}
/*
=============================================================================

WADSYSTEM PRIVATE ROUTINES

=============================================================================
*/
// associate extension with wad type
static mut wad_types: [wadtype_t; 7] =
    [{
         let mut init =
             wadtype_s{ext: b"pal\x00" as *const u8 as *const libc::c_char,
                       type_0: 64 as libc::c_int as libc::c_schar,};
         init
     },
     {
         let mut init =
             wadtype_s{ext: b"dds\x00" as *const u8 as *const libc::c_char,
                       type_0: 65 as libc::c_int as libc::c_schar,};
         init
     },
     {
         let mut init =
             wadtype_s{ext: b"lmp\x00" as *const u8 as *const libc::c_char,
                       type_0: 66 as libc::c_int as libc::c_schar,};
         init
     },
     {
         let mut init =
             wadtype_s{ext: b"fnt\x00" as *const u8 as *const libc::c_char,
                       type_0: 70 as libc::c_int as libc::c_schar,};
         init
     },
     {
         let mut init =
             wadtype_s{ext: b"mip\x00" as *const u8 as *const libc::c_char,
                       type_0: 67 as libc::c_int as libc::c_schar,};
         init
     },
     {
         let mut init =
             wadtype_s{ext: b"txt\x00" as *const u8 as *const libc::c_char,
                       type_0: 68 as libc::c_int as libc::c_schar,};
         init
     },
     {
         let mut init =
             wadtype_s{ext: 0 as *const libc::c_char,
                       type_0: 0 as libc::c_int as libc::c_schar,};
         init
     }];
/*
===========
W_TypeFromExt

Extracts file type from extension
===========
*/
unsafe extern "C" fn W_TypeFromExt(mut lumpname: *const libc::c_char)
 -> libc::c_schar {
    let mut ext: *const libc::c_char = COM_FileExtension(lumpname);
    let mut type_0: *const wadtype_t = 0 as *const wadtype_t;
    // we not known about filetype, so match only by filename
    if Q_strncmp(ext, b"*\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 ||
           Q_strncmp(ext, b"\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
        return -(1 as libc::c_int) as libc::c_schar
    }
    type_0 = wad_types.as_ptr();
    while !(*type_0).ext.is_null() {
        if Q_strnicmp(ext, (*type_0).ext, 99999 as libc::c_int) == 0 {
            return (*type_0).type_0
        }
        type_0 = type_0.offset(1)
    }
    return 0 as libc::c_int as libc::c_schar;
}
/*
===========
W_ExtFromType

Convert type to extension
===========
*/
unsafe extern "C" fn W_ExtFromType(mut lumptype: libc::c_schar)
 -> *const libc::c_char {
    let mut type_0: *const wadtype_t = 0 as *const wadtype_t;
    // we not known aboyt filetype, so match only by filename
    if lumptype as libc::c_int == 0 as libc::c_int ||
           lumptype as libc::c_int == -(1 as libc::c_int) {
        return b"\x00" as *const u8 as *const libc::c_char
    }
    type_0 = wad_types.as_ptr();
    while !(*type_0).ext.is_null() {
        if lumptype as libc::c_int == (*type_0).type_0 as libc::c_int {
            return (*type_0).ext
        }
        type_0 = type_0.offset(1)
    }
    return b"\x00" as *const u8 as *const libc::c_char;
}
/*
===========
W_FindLump

Serach for already existed lump
===========
*/
unsafe extern "C" fn W_FindLump(mut wad: *mut wfile_t,
                                mut name: *const libc::c_char,
                                matchtype: libc::c_schar)
 -> *mut dlumpinfo_t {
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    if wad.is_null() || (*wad).lumps.is_null() ||
           matchtype as libc::c_int == 0 as libc::c_int {
        return 0 as *mut dlumpinfo_t
    }
    // look for the file (binary search)
    left = 0 as libc::c_int; // found
    right = (*wad).numlumps - 1 as libc::c_int;
    while left <= right {
        let mut middle: libc::c_int = (left + right) / 2 as libc::c_int;
        let mut diff: libc::c_int =
            Q_strnicmp((*(*wad).lumps.offset(middle as
                                                 isize)).name.as_mut_ptr(),
                       name, 99999 as libc::c_int);
        if diff == 0 {
            if matchtype as libc::c_int == -(1 as libc::c_int) ||
                   matchtype as libc::c_int ==
                       (*(*wad).lumps.offset(middle as isize)).type_0 as
                           libc::c_int {
                return &mut *(*wad).lumps.offset(middle as isize) as
                           *mut dlumpinfo_t
            } else if ((*(*wad).lumps.offset(middle as isize)).type_0 as
                           libc::c_int) < matchtype as libc::c_int {
                diff = 1 as libc::c_int
            } else {
                if !((*(*wad).lumps.offset(middle as isize)).type_0 as
                         libc::c_int > matchtype as libc::c_int) {
                    break ;
                }
                diff = -(1 as libc::c_int)
            }
            // not found
        }
        // if we're too far in the list
        if diff > 0 as libc::c_int {
            right = middle - 1 as libc::c_int
        } else { left = middle + 1 as libc::c_int }
    }
    return 0 as *mut dlumpinfo_t;
}
/*
====================
W_AddFileToWad

Add a file to the list of files contained into a package
and sort LAT in alpha-bethical order
====================
*/
unsafe extern "C" fn W_AddFileToWad(mut name: *const libc::c_char,
                                    mut wad: *mut wfile_t,
                                    mut newlump: *mut dlumpinfo_t)
 -> *mut dlumpinfo_t {
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut plump: *mut dlumpinfo_t = 0 as *mut dlumpinfo_t;
    // look for the slot we should put that file into (binary search)
    left = 0 as libc::c_int;
    right = (*wad).numlumps - 1 as libc::c_int;
    while left <= right {
        let mut middle: libc::c_int = (left + right) / 2 as libc::c_int;
        let mut diff: libc::c_int =
            Q_strnicmp((*(*wad).lumps.offset(middle as
                                                 isize)).name.as_mut_ptr(),
                       name, 99999 as libc::c_int);
        if diff == 0 {
            if ((*(*wad).lumps.offset(middle as isize)).type_0 as libc::c_int)
                   < (*newlump).type_0 as libc::c_int {
                diff = 1 as libc::c_int
            } else if (*(*wad).lumps.offset(middle as isize)).type_0 as
                          libc::c_int > (*newlump).type_0 as libc::c_int {
                diff = -(1 as libc::c_int)
            } else {
                Con_Reportf(b"^3Warning:^7 Wad %s contains the file %s several times\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*wad).filename.as_mut_ptr(), name);
            }
        }
        // If we're too far in the list
        if diff > 0 as libc::c_int {
            right = middle - 1 as libc::c_int
        } else { left = middle + 1 as libc::c_int }
    }
    // we have to move the right of the list by one slot to free the one we need
    plump = &mut *(*wad).lumps.offset(left as isize) as *mut dlumpinfo_t;
    memmove(plump.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            plump as *const libc::c_void,
            (((*wad).numlumps - left) as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<dlumpinfo_t>()
                                                 as libc::c_ulong));
    (*wad).numlumps += 1;
    *plump = *newlump;
    memcpy((*plump).name.as_mut_ptr() as *mut libc::c_void,
           name as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
    return plump;
}
/*
===========
W_ReadLump

reading lump into temp buffer
===========
*/
#[no_mangle]
pub unsafe extern "C" fn W_ReadLump(mut wad: *mut wfile_t,
                                    mut lump: *mut dlumpinfo_t,
                                    mut lumpsizeptr: *mut fs_offset_t)
 -> *mut byte {
    let mut oldpos: size_t = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut buf: *mut byte = 0 as *mut byte;
    // assume error
    if !lumpsizeptr.is_null() {
        *lumpsizeptr = 0 as libc::c_int as fs_offset_t
    }
    // no wads loaded
    if wad.is_null() || lump.is_null() {
        return 0 as *mut byte
    } // don't forget restore original position
    oldpos = FS_Tell((*wad).handle) as size_t;
    if FS_Seek((*wad).handle, (*lump).filepos as fs_offset_t,
               0 as libc::c_int) == -(1 as libc::c_int) {
        Con_Reportf(b"^1Error:^7 W_ReadLump: %s is corrupted\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*lump).name.as_mut_ptr());
        FS_Seek((*wad).handle, oldpos as fs_offset_t, 0 as libc::c_int);
        return 0 as *mut byte
    }
    buf =
        _Mem_Alloc((*wad).mempool, (*lump).disksize as size_t, false_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 3970 as libc::c_int) as *mut byte;
    size =
        FS_Read((*wad).handle, buf as *mut libc::c_void,
                (*lump).disksize as size_t) as size_t;
    if size < (*lump).disksize as libc::c_ulong {
        Con_Reportf(b"^3Warning:^7 W_ReadLump: %s is probably corrupted\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*lump).name.as_mut_ptr());
        FS_Seek((*wad).handle, oldpos as fs_offset_t, 0 as libc::c_int);
        _Mem_Free(buf as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 3977 as libc::c_int);
        return 0 as *mut byte
    }
    if !lumpsizeptr.is_null() {
        *lumpsizeptr = (*lump).disksize as fs_offset_t
    }
    FS_Seek((*wad).handle, oldpos as fs_offset_t, 0 as libc::c_int);
    return buf;
}
/*
=============================================================================

WADSYSTEM PUBLIC BASE FUNCTIONS

=============================================================================
*/
/*
===========
W_Open

open the wad for reading & writing
===========
*/
unsafe extern "C" fn W_Open(mut filename: *const libc::c_char,
                            mut error: *mut libc::c_int) -> *mut wfile_t {
    let mut wad: *mut wfile_t =
        _Mem_Alloc(fs_mempool,
                   ::std::mem::size_of::<wfile_t>() as libc::c_ulong, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 4003 as libc::c_int) as
            *mut wfile_t;
    let mut basename: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut lumpcount: libc::c_int = 0;
    let mut srclumps: *mut dlumpinfo_t = 0 as *mut dlumpinfo_t;
    let mut lat_size: size_t = 0;
    let mut header: dwadinfo_t =
        dwadinfo_t{ident: 0, numlumps: 0, infotableofs: 0,};
    // NOTE: FS_Open is load wad file from the first pak in the list (while fs_ext_path is false)
    if fs_ext_path as u64 != 0 {
        basename = filename
    } else { basename = COM_FileWithoutPath(filename) }
    (*wad).handle =
        FS_Open(basename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    // HACKHACK: try to open WAD by full path for RoDir, when searchpaths are not ready
    if (if *host.rodir.as_mut_ptr() == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) != 0 && fs_ext_path as libc::c_uint != 0
           && (*wad).handle.is_null() {
        (*wad).handle =
            FS_SysOpen(filename,
                       b"rb\x00" as *const u8 as *const libc::c_char)
    }
    if (*wad).handle.is_null() {
        Con_Reportf(b"^1Error:^7 W_Open: couldn\'t open %s\n\x00" as *const u8
                        as *const libc::c_char, filename);
        if !error.is_null() { *error = 1 as libc::c_int }
        W_Close(wad);
        return 0 as *mut wfile_t
    }
    // copy wad name
    Q_strncpy((*wad).filename.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as
                  libc::c_ulong); // save infotableofs position
    (*wad).filetime = FS_SysFileTime(filename) as time_t;
    (*wad).mempool =
        _Mem_AllocPool(filename,
                       b"../engine/common/filesystem.c\x00" as *const u8 as
                           *const libc::c_char, 4031 as libc::c_int);
    if FS_Read((*wad).handle,
               &mut header as *mut dwadinfo_t as *mut libc::c_void,
               ::std::mem::size_of::<dwadinfo_t>() as libc::c_ulong) as
           libc::c_ulong !=
           ::std::mem::size_of::<dwadinfo_t>() as libc::c_ulong {
        Con_Reportf(b"^1Error:^7 W_Open: %s can\'t read header\n\x00" as
                        *const u8 as *const libc::c_char, filename);
        if !error.is_null() { *error = 2 as libc::c_int }
        W_Close(wad);
        return 0 as *mut wfile_t
    }
    if header.ident !=
           (('2' as i32) << 24 as libc::c_int) +
               (('D' as i32) << 16 as libc::c_int) +
               (('A' as i32) << 8 as libc::c_int) + 'W' as i32 &&
           header.ident !=
               (('3' as i32) << 24 as libc::c_int) +
                   (('D' as i32) << 16 as libc::c_int) +
                   (('A' as i32) << 8 as libc::c_int) + 'W' as i32 {
        Con_Reportf(b"^1Error:^7 W_Open: %s is not a WAD2 or WAD3 file\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        if !error.is_null() { *error = 2 as libc::c_int }
        W_Close(wad);
        return 0 as *mut wfile_t
    }
    lumpcount = header.numlumps;
    if lumpcount >= 65535 as libc::c_int {
        Con_Reportf(b"^3Warning:^7 W_Open: %s is full (%i lumps)\n\x00" as
                        *const u8 as *const libc::c_char, filename,
                    lumpcount);
        if !error.is_null() { *error = 4 as libc::c_int }
    } else if lumpcount <= 0 as libc::c_int {
        Con_Reportf(b"^1Error:^7 W_Open: %s has no lumps\n\x00" as *const u8
                        as *const libc::c_char, filename);
        if !error.is_null() { *error = 5 as libc::c_int }
        W_Close(wad);
        return 0 as *mut wfile_t
    } else { if !error.is_null() { *error = 0 as libc::c_int } }
    (*wad).infotableofs = header.infotableofs;
    if FS_Seek((*wad).handle, (*wad).infotableofs as fs_offset_t,
               0 as libc::c_int) == -(1 as libc::c_int) {
        Con_Reportf(b"^1Error:^7 W_Open: %s can\'t find lump allocation table\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        if !error.is_null() { *error = 3 as libc::c_int }
        W_Close(wad);
        return 0 as *mut wfile_t
    }
    lat_size =
        (lumpcount as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<dlumpinfo_t>()
                                             as libc::c_ulong);
    // NOTE: lumps table can be reallocated for O_APPEND mode
    srclumps =
        _Mem_Alloc((*wad).mempool, lat_size, false_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 4078 as libc::c_int) as
            *mut dlumpinfo_t;
    if FS_Read((*wad).handle, srclumps as *mut libc::c_void, lat_size) as
           libc::c_ulong != lat_size {
        Con_Reportf(b"^1Error:^7 W_ReadLumpTable: %s has corrupted lump allocation table\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*wad).filename.as_mut_ptr());
        if !error.is_null() { *error = 6 as libc::c_int }
        _Mem_Free(srclumps as *mut libc::c_void,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 4084 as libc::c_int);
        W_Close(wad);
        return 0 as *mut wfile_t
    }
    // starting to add lumps
    (*wad).lumps =
        _Mem_Alloc((*wad).mempool, lat_size, true_0,
                   b"../engine/common/filesystem.c\x00" as *const u8 as
                       *const libc::c_char, 4090 as libc::c_int) as
            *mut dlumpinfo_t;
    (*wad).numlumps = 0 as libc::c_int;
    // sort lumps for binary search
    i = 0 as libc::c_int;
    while i < lumpcount {
        let mut name: [libc::c_char; 16] = [0; 16];
        let mut k: libc::c_int = 0;
        // cleanup lumpname
        Q_strnlwr((*srclumps.offset(i as isize)).name.as_mut_ptr(),
                  name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 16]>() as
                      libc::c_ulong);
        // check for '*' symbol issues (quake1)
        k =
            Q_strlen(Q_strrchr(name.as_mut_ptr(), '*' as i32 as libc::c_char))
                as libc::c_int;
        if k != 0 {
            name[Q_strlen(name.as_mut_ptr()).wrapping_sub(k as libc::c_ulong)
                     as usize] = '!' as i32 as libc::c_char
        }
        // check for Quake 'conchars' issues (only lmp loader really allows to read this lame pic)
        if (*srclumps.offset(i as isize)).type_0 as libc::c_int ==
               68 as libc::c_int &&
               Q_strnicmp((*srclumps.offset(i as isize)).name.as_mut_ptr(),
                          b"conchars\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
            (*srclumps.offset(i as isize)).type_0 =
                66 as libc::c_int as libc::c_schar
        }
        W_AddFileToWad(name.as_mut_ptr(), wad,
                       &mut *srclumps.offset(i as isize));
        i += 1
    }
    // release source lumps
    _Mem_Free(srclumps as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 4114 as libc::c_int);
    // and leave the file open
    return wad;
}
/*
===========
W_Close

finalize wad or just close
===========
*/
#[no_mangle]
pub unsafe extern "C" fn W_Close(mut wad: *mut wfile_t) {
    if wad.is_null() { return }
    _Mem_FreePool(&mut (*wad).mempool,
                  b"../engine/common/filesystem.c\x00" as *const u8 as
                      *const libc::c_char, 4131 as libc::c_int);
    if !(*wad).handle.is_null() { FS_Close((*wad).handle); }
    _Mem_Free(wad as *mut libc::c_void,
              b"../engine/common/filesystem.c\x00" as *const u8 as
                  *const libc::c_char, 4134 as libc::c_int);
    // free himself
}
/*
=============================================================================

FILESYSTEM IMPLEMENTATION

=============================================================================
*/
/*
===========
W_LoadFile

loading lump into the tmp buffer
===========
*/
unsafe extern "C" fn W_LoadFile(mut path: *const libc::c_char,
                                mut lumpsizeptr: *mut fs_offset_t,
                                mut gamedironly: qboolean) -> *mut byte {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut index: libc::c_int = 0;
    search = FS_FindFile(path, &mut index, gamedironly);
    if !search.is_null() && !(*search).wad.is_null() {
        return W_ReadLump((*search).wad,
                          &mut *(*(*search).wad).lumps.offset(index as isize),
                          lumpsizeptr)
    }
    return 0 as *mut byte;
}
