#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn frame_index_find(fr: *mut mpg123_handle_t, want_frame: mpg_off_t,
                        get_frame: *mut mpg_off_t) -> mpg_off_t;
    #[no_mangle]
    fn read_frame(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int)
     -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpg123_handle_s {
    pub fresh: libc::c_int,
    pub new_format: libc::c_int,
    pub hybrid_block: [[[libc::c_float; 576]; 2]; 2],
    pub hybrid_blc: [libc::c_int; 2],
    pub short_buffs: [[*mut libc::c_short; 2]; 2],
    pub float_buffs: [[*mut libc::c_float; 2]; 2],
    pub rawbuffs: *mut byte,
    pub rawbuffss: libc::c_int,
    pub bo: libc::c_int,
    pub rawdecwin: *mut byte,
    pub rawdecwins: libc::c_int,
    pub decwin: *mut libc::c_float,
    pub ssave: [byte; 34],
    pub halfphase: libc::c_int,
    pub longLimit: [[libc::c_int; 23]; 9],
    pub shortLimit: [[libc::c_int; 14]; 9],
    pub gainpow2: [libc::c_float; 378],
    pub synths: synth_t,
    pub verbose: libc::c_int,
    pub alloc: *const al_table_t,
    pub synth: func_synth,
    pub synth_stereo: func_synth_stereo,
    pub synth_mono: func_synth_mono,
    pub make_decode_tables: Option<unsafe extern "C" fn(_:
                                                            *mut mpg123_handle_t)
                                       -> ()>,
    pub stereo: libc::c_int,
    pub jsbound: libc::c_int,
    pub single: libc::c_int,
    pub II_sblimit: libc::c_int,
    pub down_sample_sblimit: libc::c_int,
    pub lsf: libc::c_int,
    pub mpeg25: libc::c_int,
    pub down_sample: libc::c_int,
    pub header_change: libc::c_int,
    pub lay: libc::c_int,
    pub spf: libc::c_long,
    pub do_layer: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t)
                             -> libc::c_int>,
    pub error_protection: libc::c_int,
    pub bitrate_index: libc::c_int,
    pub sampling_frequency: libc::c_int,
    pub padding: libc::c_int,
    pub extension: libc::c_int,
    pub mode: libc::c_int,
    pub mode_ext: libc::c_int,
    pub copyright: libc::c_int,
    pub original: libc::c_int,
    pub emphasis: libc::c_int,
    pub framesize: libc::c_int,
    pub freesize: libc::c_int,
    pub vbr: libc::c_int,
    pub num: mpg_off_t,
    pub input_offset: mpg_off_t,
    pub playnum: mpg_off_t,
    pub audio_start: mpg_off_t,
    pub state_flags: libc::c_int,
    pub silent_resync: libc::c_char,
    pub xing_toc: *mut byte,
    pub freeformat: libc::c_int,
    pub freeformat_framesize: libc::c_long,
    pub bitindex: libc::c_int,
    pub wordpointer: *mut byte,
    pub ultmp: ulong,
    pub uctmp: byte,
    pub maxoutburst: libc::c_double,
    pub lastscale: libc::c_double,
    pub rva: C2RustUnnamed_0,
    pub track_frames: mpg_off_t,
    pub track_samples: mpg_off_t,
    pub mean_framesize: libc::c_double,
    pub mean_frames: mpg_off_t,
    pub fsizeold: libc::c_int,
    pub ssize: libc::c_int,
    pub bitreservoir: uint,
    pub bsspace: [[byte; 3968]; 2],
    pub bsbuf: *mut byte,
    pub bsbufold: *mut byte,
    pub bsnum: libc::c_int,
    pub oldhead: ulong,
    pub firsthead: ulong,
    pub abr_rate: libc::c_int,
    pub index: frame_index_t,
    pub buffer: outbuffer_t,
    pub af: audioformat_t,
    pub own_buffer: libc::c_int,
    pub outblock: size_t,
    pub to_decode: libc::c_int,
    pub to_ignore: libc::c_int,
    pub firstframe: mpg_off_t,
    pub lastframe: mpg_off_t,
    pub ignoreframe: mpg_off_t,
    pub gapless_frames: mpg_off_t,
    pub firstoff: mpg_off_t,
    pub lastoff: mpg_off_t,
    pub begin_s: mpg_off_t,
    pub begin_os: mpg_off_t,
    pub end_s: mpg_off_t,
    pub end_os: mpg_off_t,
    pub fullend_os: mpg_off_t,
    pub crc: uint,
    pub rd: *mut reader_t,
    pub rdat: reader_data_t,
    pub p: mpg123_parm_t,
    pub err: libc::c_int,
    pub decoder_change: libc::c_int,
    pub delayed_change: libc::c_int,
    pub clip: libc::c_long,
    pub metaflags: libc::c_int,
    pub id3buf: [byte; 128],
    pub layerscratch: *mut libc::c_float,
    pub layer3: C2RustUnnamed,
    pub wrapperdata: *mut libc::c_void,
    pub wrapperclean: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub hybrid_in: *mut [[libc::c_float; 18]; 32],
    pub hybrid_out: *mut [[libc::c_float; 32]; 18],
}
pub type byte = libc::c_uchar;
pub type mpg123_parm_t = mpg123_parm_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpg123_parm_s {
    pub verbose: libc::c_int,
    pub flags: libc::c_long,
    pub down_sample: libc::c_int,
    pub rva: libc::c_int,
    pub halfspeed: libc::c_long,
    pub doublespeed: libc::c_long,
    pub timeout: libc::c_long,
    pub audio_caps: [[[libc::c_char; 2]; 10]; 2],
    pub outscale: libc::c_double,
    pub resync_limit: libc::c_long,
    pub index_size: libc::c_long,
    pub preframes: libc::c_long,
    pub feedpool: libc::c_long,
    pub feedbuffer: libc::c_long,
}
pub type reader_data_t = reader_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reader_data_s {
    pub filelen: mpg_off_t,
    pub filepos: mpg_off_t,
    pub filept: libc::c_int,
    pub iohandle: *mut libc::c_void,
    pub flags: libc::c_int,
    pub timeout_sec: libc::c_long,
    pub fdread: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                            _: *mut libc::c_void, _: size_t)
                           -> mpg_ssize_t>,
    pub r_read: Option<unsafe extern "C" fn(_: libc::c_int,
                                            _: *mut libc::c_void, _: size_t)
                           -> mpg_ssize_t>,
    pub r_lseek: Option<unsafe extern "C" fn(_: libc::c_int, _: mpg_off_t,
                                             _: libc::c_int) -> mpg_off_t>,
    pub r_read_handle: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: *mut libc::c_void,
                                                   _: size_t) -> mpg_ssize_t>,
    pub r_lseek_handle: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                    _: mpg_off_t,
                                                    _: libc::c_int)
                                   -> mpg_off_t>,
    pub cleanup_handle: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
    pub read: Option<unsafe extern "C" fn(_: libc::c_int,
                                          _: *mut libc::c_void, _: size_t)
                         -> mpg_ssize_t>,
    pub lseek: Option<unsafe extern "C" fn(_: libc::c_int, _: mpg_off_t,
                                           _: libc::c_int) -> mpg_off_t>,
    pub fullread: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                              _: *mut byte, _: mpg_ssize_t)
                             -> mpg_ssize_t>,
    pub buffer: bufferchain_t,
}
pub type bufferchain_t = bufferchain_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferchain_s {
    pub first: *mut buffy_s,
    pub last: *mut buffy_s,
    pub size: mpg_ssize_t,
    pub pos: mpg_ssize_t,
    pub firstpos: mpg_ssize_t,
    pub fileoff: mpg_off_t,
    pub bufblock: size_t,
    pub pool_size: size_t,
    pub pool_fill: size_t,
    pub pool: *mut buffy_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffy_s {
    pub data: *mut byte,
    pub size: mpg_ssize_t,
    pub realsize: mpg_ssize_t,
    pub next: *mut buffy_s,
}
pub type mpg_ssize_t = ssize_t;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type mpg_off_t = libc::c_long;
pub type mpg123_handle_t = mpg123_handle_s;
pub type reader_t = reader_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reader_s {
    pub init: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t)
                         -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ()>,
    pub fullread: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                              _: *mut byte, _: mpg_ssize_t)
                             -> mpg_ssize_t>,
    pub head_read: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                               _: *mut ulong) -> libc::c_int>,
    pub head_shift: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: *mut ulong)
                               -> libc::c_int>,
    pub skip_bytes: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: mpg_off_t) -> mpg_off_t>,
    pub read_frame_body: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                     _: *mut byte,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub back_bytes: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: mpg_off_t) -> libc::c_int>,
    pub seek_frame: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: mpg_off_t) -> libc::c_int>,
    pub tell: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t)
                         -> mpg_off_t>,
    pub rewind: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ()>,
    pub forget: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ()>,
}
pub type ulong = libc::c_ulong;
pub type uint = libc::c_uint;
pub type audioformat_t = audioformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audioformat_s {
    pub encoding: libc::c_int,
    pub encsize: libc::c_int,
    pub dec_enc: libc::c_int,
    pub dec_encsize: libc::c_int,
    pub channels: libc::c_int,
    pub rate: libc::c_int,
}
pub type outbuffer_t = outbuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outbuffer_s {
    pub data: *mut byte,
    pub p: *mut byte,
    pub fill: size_t,
    pub size: size_t,
    pub rdata: *mut byte,
}
pub type frame_index_t = frame_index_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_index_s {
    pub data: *mut mpg_off_t,
    pub step: mpg_off_t,
    pub next: mpg_off_t,
    pub size: size_t,
    pub fill: size_t,
    pub grow_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub level: [libc::c_int; 2],
    pub gain: [libc::c_float; 2],
    pub peak: [libc::c_float; 2],
}
pub type func_synth_mono
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                _: *mut mpg123_handle_t) -> libc::c_int>;
pub type func_synth_stereo
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_float, _: *mut libc::c_float,
                                _: *mut mpg123_handle_t) -> libc::c_int>;
pub type func_synth
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_float, _: libc::c_int,
                                _: *mut mpg123_handle_t, _: libc::c_int)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct al_table_t {
    pub bits: libc::c_short,
    pub d: libc::c_short,
}
pub type synth_t = synth_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct synth_s {
    pub plain: [[func_synth; 1]; 1],
    pub stereo: [[func_synth_stereo; 1]; 1],
    pub mono2stereo: [[func_synth_mono; 1]; 1],
    pub mono: [[func_synth_mono; 1]; 1],
}
pub type __off_t = libc::c_long;
pub type buffy_t = buffy_s;
pub const MPG123_OK: mpg123_errors = 0;
pub const MPG123_ERR: mpg123_errors = -1;
pub const MPG123_LSEEK_FAILED: mpg123_errors = 40;
pub const MPG123_NO_SEEK: mpg123_errors = 23;
pub const MPG123_NEED_MORE: mpg123_errors = -10;
pub const MPG123_SEEKBUFFER: mpg123_param_flags = 256;
pub const MPG123_NO_READER: mpg123_errors = 24;
pub type mpg123_errors = libc::c_int;
pub const MPG123_INT_OVERFLOW: mpg123_errors = 43;
pub const MPG123_LFS_OVERFLOW: mpg123_errors = 42;
pub const MPG123_BAD_CUSTOM_IO: mpg123_errors = 41;
pub const MPG123_BAD_VALUE: mpg123_errors = 39;
pub const MPG123_MISSING_FEATURE: mpg123_errors = 38;
pub const MPG123_BAD_DECODER_SETUP: mpg123_errors = 37;
pub const MPG123_INDEX_FAIL: mpg123_errors = 36;
pub const MPG123_NO_INDEX: mpg123_errors = 35;
pub const MPG123_BAD_KEY: mpg123_errors = 34;
pub const MPG123_NULL_POINTER: mpg123_errors = 33;
pub const MPG123_NO_RELSEEK: mpg123_errors = 32;
pub const MPG123_NULL_BUFFER: mpg123_errors = 31;
pub const MPG123_BAD_ALIGN: mpg123_errors = 30;
pub const MPG123_NO_8BIT: mpg123_errors = 29;
pub const MPG123_RESYNC_FAIL: mpg123_errors = 28;
pub const MPG123_OUT_OF_SYNC: mpg123_errors = 27;
pub const MPG123_BAD_INDEX_PAR: mpg123_errors = 26;
pub const MPG123_BAD_PARS: mpg123_errors = 25;
pub const MPG123_BAD_FILE: mpg123_errors = 22;
pub const MPG123_NO_TIMEOUT: mpg123_errors = 21;
pub const MPG123_BAD_WHENCE: mpg123_errors = 20;
pub const MPG123_NO_SEEK_FROM_END: mpg123_errors = 19;
pub const MPG123_ERR_READER: mpg123_errors = 18;
pub const MPG123_ERR_NULL: mpg123_errors = 17;
pub const MPG123_BAD_BAND: mpg123_errors = 16;
pub const MPG123_BAD_TYPES: mpg123_errors = 15;
pub const MPG123_NO_SPACE: mpg123_errors = 14;
pub const MPG123_NO_GAPLESS: mpg123_errors = 13;
pub const MPG123_BAD_RVA: mpg123_errors = 12;
pub const MPG123_NO_BUFFERS: mpg123_errors = 11;
pub const MPG123_BAD_HANDLE: mpg123_errors = 10;
pub const MPG123_BAD_DECODER: mpg123_errors = 9;
pub const MPG123_NOT_INITIALIZED: mpg123_errors = 8;
pub const MPG123_OUT_OF_MEM: mpg123_errors = 7;
pub const MPG123_BAD_BUFFER: mpg123_errors = 6;
pub const MPG123_BAD_PARAM: mpg123_errors = 5;
pub const MPG123_ERR_16TO8TABLE: mpg123_errors = 4;
pub const MPG123_BAD_RATE: mpg123_errors = 3;
pub const MPG123_BAD_CHANNEL: mpg123_errors = 2;
pub const MPG123_BAD_OUTFORMAT: mpg123_errors = 1;
pub const MPG123_NEW_FORMAT: mpg123_errors = -11;
pub const MPG123_DONE: mpg123_errors = -12;
pub type mpg123_param_flags = libc::c_uint;
pub const MPG123_AUTO_RESAMPLE: mpg123_param_flags = 32768;
pub const MPG123_IGNORE_INFOFRAME: mpg123_param_flags = 16384;
pub const MPG123_IGNORE_STREAMLENGTH: mpg123_param_flags = 4096;
pub const MPG123_FUZZY: mpg123_param_flags = 512;
pub const MPG123_NO_RESYNC: mpg123_param_flags = 128;
pub const MPG123_GAPLESS: mpg123_param_flags = 64;
pub const MPG123_QUIET: mpg123_param_flags = 32;
pub const MPG123_FORCE_STEREO: mpg123_param_flags = 8;
pub const MPG123_MONO_MIX: mpg123_param_flags = 4;
pub const MPG123_MONO_RIGHT: mpg123_param_flags = 2;
pub const MPG123_MONO_LEFT: mpg123_param_flags = 1;
pub const MPG123_FORCE_MONO: mpg123_param_flags = 7;
// methods for the buffer chain, mainly used for feed reader, but not just that.
unsafe extern "C" fn buffy_new(mut size: size_t, mut minsize: size_t)
 -> *mut buffy_t {
    let mut newbuf: *mut buffy_t =
        malloc(::std::mem::size_of::<buffy_t>() as libc::c_ulong) as
            *mut buffy_t;
    if newbuf.is_null() { return 0 as *mut buffy_t }
    (*newbuf).realsize =
        if size > minsize { size } else { minsize } as mpg_ssize_t;
    (*newbuf).data = malloc((*newbuf).realsize as libc::c_ulong) as *mut byte;
    if (*newbuf).data.is_null() {
        free(newbuf as *mut libc::c_void);
        return 0 as *mut buffy_t
    }
    (*newbuf).size = 0 as libc::c_int as mpg_ssize_t;
    (*newbuf).next = 0 as *mut buffy_s;
    return newbuf;
}
unsafe extern "C" fn buffy_del(mut buf: *mut buffy_t) {
    if !buf.is_null() {
        free((*buf).data as *mut libc::c_void);
        free(buf as *mut libc::c_void);
    };
}
// delete this buffy and all following buffies.
unsafe extern "C" fn buffy_del_chain(mut buf: *mut buffy_t) {
    while !buf.is_null() {
        let mut next: *mut buffy_t = (*buf).next;
        buffy_del(buf);
        buf = next
    };
}
// fetch a buffer from the pool (if possible) or create one.
unsafe extern "C" fn bc_alloc(mut bc: *mut bufferchain_t, mut size: size_t)
 -> *mut buffy_t {
    // Easy route: Just try the first available buffer.
	// size does not matter, it's only a hint for creation of new buffers.
    if !(*bc).pool.is_null() {
        let mut buf: *mut buffy_t =
            (*bc).pool; // that shall be set to a sensible value later.
        (*bc).pool = (*buf).next;
        (*buf).next = 0 as *mut buffy_s;
        (*buf).size = 0 as libc::c_int as mpg_ssize_t;
        (*bc).pool_fill = (*bc).pool_fill.wrapping_sub(1);
        return buf
    }
    return buffy_new(size, (*bc).bufblock);
}
// either stuff the buffer back into the pool or free it for good.
unsafe extern "C" fn bc_free(mut bc: *mut bufferchain_t,
                             mut buf: *mut buffy_t) {
    if buf.is_null() { return }
    if (*bc).pool_fill < (*bc).pool_size {
        (*buf).next = (*bc).pool;
        (*bc).pool = buf;
        (*bc).pool_fill = (*bc).pool_fill.wrapping_add(1)
    } else { buffy_del(buf); };
}
// make the buffer count in the pool match the pool size.
unsafe extern "C" fn bc_fill_pool(mut bc: *mut bufferchain_t) -> libc::c_int {
    // remove superfluous ones.
    while (*bc).pool_fill > (*bc).pool_size {
        // lazyness: Just work on the front.
        let mut buf: *mut buffy_t = (*bc).pool;
        (*bc).pool = (*buf).next;
        buffy_del(buf);
        (*bc).pool_fill = (*bc).pool_fill.wrapping_sub(1)
    }
    // add missing ones.
    while (*bc).pool_fill < (*bc).pool_size {
        // again, just work on the front.
        let mut buf_0: *mut buffy_t =
            buffy_new(0 as libc::c_int as size_t,
                      (*bc).bufblock); // use default block size.
        if buf_0.is_null() { return -(1 as libc::c_int) }
        (*buf_0).next = (*bc).pool;
        (*bc).pool = buf_0;
        (*bc).pool_fill = (*bc).pool_fill.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn bc_init(mut bc: *mut bufferchain_t) {
    (*bc).first = 0 as *mut buffy_s;
    (*bc).last = (*bc).first;
    (*bc).size = 0 as libc::c_int as mpg_ssize_t;
    (*bc).pos = 0 as libc::c_int as mpg_ssize_t;
    (*bc).firstpos = 0 as libc::c_int as mpg_ssize_t;
    (*bc).fileoff = 0 as libc::c_int as mpg_off_t;
}
unsafe extern "C" fn bc_reset(mut bc: *mut bufferchain_t) {
    // free current chain, possibly stuffing back into the pool.
    while !(*bc).first.is_null() {
        let mut buf: *mut buffy_t = (*bc).first; // ignoring an error here...
        (*bc).first = (*buf).next;
        bc_free(bc, buf);
    }
    bc_fill_pool(bc);
    bc_init(bc);
}
// create a new buffy at the end to be filled.
unsafe extern "C" fn bc_append(mut bc: *mut bufferchain_t,
                               mut size: mpg_ssize_t) -> libc::c_int {
    let mut newbuf: *mut buffy_t = 0 as *mut buffy_t;
    if size < 1 as libc::c_int as libc::c_long { return -(1 as libc::c_int) }
    newbuf = bc_alloc(bc, size as size_t);
    if newbuf.is_null() { return -(2 as libc::c_int) }
    if !(*bc).last.is_null() {
        (*(*bc).last).next = newbuf
    } else if (*bc).first.is_null() { (*bc).first = newbuf }
    (*bc).last = newbuf;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bc_prepare(mut bc: *mut bufferchain_t,
                                    mut pool_size: size_t,
                                    mut bufblock: size_t) {
    bc_poolsize(bc, pool_size, bufblock);
    (*bc).pool = 0 as *mut buffy_s;
    (*bc).pool_fill = 0 as libc::c_int as size_t;
    bc_init(bc);
    // ensure that members are zeroed for read-only use.
}
#[no_mangle]
pub unsafe extern "C" fn bc_fill(mut bc: *mut bufferchain_t) -> size_t {
    return ((*bc).size - (*bc).pos) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn bc_poolsize(mut bc: *mut bufferchain_t,
                                     mut pool_size: size_t,
                                     mut bufblock: size_t) {
    (*bc).pool_size = pool_size;
    (*bc).bufblock = bufblock;
}
#[no_mangle]
pub unsafe extern "C" fn bc_cleanup(mut bc: *mut bufferchain_t) {
    buffy_del_chain((*bc).pool);
    (*bc).pool_fill = 0 as libc::c_int as size_t;
    (*bc).pool = 0 as *mut buffy_s;
}
// append a new buffer and copy content to it.
unsafe extern "C" fn bc_add(mut bc: *mut bufferchain_t, mut data: *const byte,
                            mut size: mpg_ssize_t) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut part: mpg_ssize_t = 0 as libc::c_int as mpg_ssize_t;
    while size > 0 as libc::c_int as libc::c_long {
        // try to fill up the last buffer block.
        if !(*bc).last.is_null() &&
               (*(*bc).last).size < (*(*bc).last).realsize {
            part = (*(*bc).last).realsize - (*(*bc).last).size;
            if part > size { part = size }
            memcpy((*(*bc).last).data.offset((*(*bc).last).size as isize) as
                       *mut libc::c_void, data as *const libc::c_void,
                   part as libc::c_ulong);
            (*(*bc).last).size += part;
            size -= part;
            (*bc).size += part;
            data = data.offset(part as isize)
        }
        // if there is still data left, put it into a new buffer block.
        if size > 0 as libc::c_int as libc::c_long &&
               { ret = bc_append(bc, size); (ret) != 0 as libc::c_int } {
            break ;
        }
    }
    return ret;
}
// common handler for "You want more than I can give." situation.
unsafe extern "C" fn bc_need_more(mut bc: *mut bufferchain_t) -> mpg_ssize_t {
    // go back to firstpos, undo the previous reads
    (*bc).pos = (*bc).firstpos;
    return MPG123_NEED_MORE as libc::c_int as mpg_ssize_t;
}
// give some data, advancing position but not forgetting yet.
unsafe extern "C" fn bc_give(mut bc: *mut bufferchain_t, mut out: *mut byte,
                             mut size: mpg_ssize_t) -> mpg_ssize_t {
    let mut b: *mut buffy_t = (*bc).first;
    let mut gotcount: mpg_ssize_t = 0 as libc::c_int as mpg_ssize_t;
    let mut offset: mpg_ssize_t = 0 as libc::c_int as mpg_ssize_t;
    if (*bc).size - (*bc).pos < size { return bc_need_more(bc) }
    // find the current buffer
    while !b.is_null() && offset + (*b).size <= (*bc).pos {
        offset += (*b).size;
        b = (*b).next
    }
    // now start copying from there
    while gotcount < size && !b.is_null() {
        let mut loff: mpg_ssize_t =
            (*bc).pos - offset; // amount of bytes to get from here...
        let mut chunk: mpg_ssize_t = size - gotcount;
        if chunk > (*b).size - loff { chunk = (*b).size - loff }
        memcpy(out.offset(gotcount as isize) as *mut libc::c_void,
               (*b).data.offset(loff as isize) as *const libc::c_void,
               chunk as libc::c_ulong);
        gotcount += chunk;
        (*bc).pos += chunk;
        offset += (*b).size;
        b = (*b).next
    }
    return gotcount;
}
// skip some bytes and return the new position.
// the buffers are still there, just the read pointer is moved!
unsafe extern "C" fn bc_skip(mut bc: *mut bufferchain_t,
                             mut count: mpg_ssize_t) -> mpg_ssize_t {
    if count >= 0 as libc::c_int as libc::c_long {
        if (*bc).size - (*bc).pos < count { return bc_need_more(bc) }
        (*bc).pos += count;
        return (*bc).pos
    }
    return MPG123_ERR as libc::c_int as mpg_ssize_t;
}
unsafe extern "C" fn bc_seekback(mut bc: *mut bufferchain_t,
                                 mut count: mpg_ssize_t) -> mpg_ssize_t {
    if count >= 0 as libc::c_int as libc::c_long && count <= (*bc).pos {
        (*bc).pos -= count;
        return (*bc).pos
    }
    return MPG123_ERR as libc::c_int as mpg_ssize_t;
}
// throw away buffies that we passed.
unsafe extern "C" fn bc_forget(mut bc: *mut bufferchain_t) {
    let mut b: *mut buffy_t = (*bc).first;
    // free all buffers that are def'n'tly outdated
	// we have buffers until filepos... delete all buffers fully below it
    while !b.is_null() && (*bc).pos >= (*b).size {
        let mut n: *mut buffy_t =
            (*b).next; // != NULL or this is indeed the end and the last cycle anyway
        if n.is_null() {
            (*bc).last = 0 as *mut buffy_s
        } // Going to delete the last buffy...
        (*bc).fileoff += (*b).size;
        (*bc).pos -= (*b).size;
        (*bc).size -= (*b).size;
        bc_free(bc, b);
        b = n
    }
    (*bc).first = b;
    (*bc).firstpos = (*bc).pos;
}
// reader for input via manually provided buffers
unsafe extern "C" fn feed_init(mut fr: *mut mpg123_handle_t) -> libc::c_int {
    bc_init(&mut (*fr).rdat.buffer);
    bc_fill_pool(&mut (*fr).rdat.buffer);
    (*fr).rdat.filelen = 0 as libc::c_int as mpg_off_t;
    (*fr).rdat.filepos = 0 as libc::c_int as mpg_off_t;
    (*fr).rdat.flags |= 0x8 as libc::c_int;
    return 0 as libc::c_int;
}
// externally called function, returns 0 on success, -1 on error
#[no_mangle]
pub unsafe extern "C" fn feed_more(mut fr: *mut mpg123_handle_t,
                                   mut in_0: *const byte,
                                   mut count: libc::c_long) -> libc::c_int {
    if bc_add(&mut (*fr).rdat.buffer, in_0, count) != 0 as libc::c_int {
        return MPG123_ERR as libc::c_int
    }
    return MPG123_OK as libc::c_int;
}
unsafe extern "C" fn feed_read(mut fr: *mut mpg123_handle_t,
                               mut out: *mut byte, mut count: mpg_ssize_t)
 -> mpg_ssize_t {
    let mut gotcount: mpg_ssize_t =
        bc_give(&mut (*fr).rdat.buffer, out, count);
    if gotcount >= 0 as libc::c_int as libc::c_long && gotcount != count {
        return MPG123_ERR as libc::c_int as mpg_ssize_t
    }
    return gotcount;
}
// returns reached position... negative ones are bad...
unsafe extern "C" fn feed_skip_bytes(mut fr: *mut mpg123_handle_t,
                                     mut len: mpg_off_t) -> mpg_off_t {
    // this is either the new buffer offset or some negative error value.
    let mut res: mpg_off_t = bc_skip(&mut (*fr).rdat.buffer, len);
    if res < 0 as libc::c_int as libc::c_long { return res }
    return (*fr).rdat.buffer.fileoff + res;
}
unsafe extern "C" fn feed_back_bytes(mut fr: *mut mpg123_handle_t,
                                     mut bytes: mpg_off_t) -> libc::c_int {
    if bytes >= 0 as libc::c_int as libc::c_long {
        return if bc_seekback(&mut (*fr).rdat.buffer, bytes) >=
                      0 as libc::c_int as libc::c_long {
                   0 as libc::c_int
               } else { MPG123_ERR as libc::c_int }
    }
    return if feed_skip_bytes(fr, -bytes) >= 0 as libc::c_int as libc::c_long
              {
               0 as libc::c_int
           } else { MPG123_ERR as libc::c_int };
}
unsafe extern "C" fn feed_seek_frame(mut fr: *mut mpg123_handle_t,
                                     mut num: mpg_off_t) -> libc::c_int {
    return MPG123_ERR as libc::c_int;
}
// not just for feed reader, also for self-feeding buffered reader.
unsafe extern "C" fn buffered_forget(mut fr: *mut mpg123_handle_t) {
    bc_forget(&mut (*fr).rdat.buffer);
    (*fr).rdat.filepos = (*fr).rdat.buffer.fileoff + (*fr).rdat.buffer.pos;
}
#[no_mangle]
pub unsafe extern "C" fn feed_set_pos(mut fr: *mut mpg123_handle_t,
                                      mut pos: mpg_off_t) -> mpg_off_t {
    let mut bc: *mut bufferchain_t = &mut (*fr).rdat.buffer;
    if pos >= (*bc).fileoff && pos - (*bc).fileoff < (*bc).size {
        // we have the position!
        (*bc).pos = pos - (*bc).fileoff;
        // next input after end of buffer...
        return (*bc).fileoff + (*bc).size
    } else {
        // i expect to get the specific position on next feed. Forget what I have now.
        bc_reset(bc);
        (*bc).fileoff = pos;
        // next input from exactly that position.
        return pos
    };
}
// the specific stuff for buffered stream reader.
unsafe extern "C" fn buffered_fullread(mut fr: *mut mpg123_handle_t,
                                       mut out: *mut byte,
                                       mut count: mpg_ssize_t)
 -> mpg_ssize_t {
    let mut bc: *mut bufferchain_t = &mut (*fr).rdat.buffer;
    let mut gotcount: mpg_ssize_t = 0;
    if (*bc).size - (*bc).pos < count {
        // add more stuff to buffer. If hitting end of file, adjust count.
        let mut readbuf: [byte; 4096] = [0; 4096];
        let mut need: mpg_ssize_t = count - ((*bc).size - (*bc).pos);
        while need > 0 as libc::c_int as libc::c_long {
            let mut got: mpg_ssize_t =
                (*fr).rdat.fullread.expect("non-null function pointer")(fr,
                                                                        readbuf.as_mut_ptr(),
                                                                        ::std::mem::size_of::<[byte; 4096]>()
                                                                            as
                                                                            libc::c_ulong
                                                                            as
                                                                            mpg_ssize_t);
            let mut ret: libc::c_int = 0;
            if got < 0 as libc::c_int as libc::c_long {
                return MPG123_ERR as libc::c_int as mpg_ssize_t
            }
            if got > 0 as libc::c_int as libc::c_long &&
                   {
                       ret = bc_add(bc, readbuf.as_mut_ptr(), got);
                       (ret) != 0 as libc::c_int
                   } {
                return MPG123_ERR as libc::c_int as mpg_ssize_t
            }
            // end.
            need -= got; // may underflow here...
            if (got as libc::c_ulong) <
                   ::std::mem::size_of::<[byte; 4096]>() as libc::c_ulong {
                break ;
            }
        }
        if (*bc).size - (*bc).pos < count { count = (*bc).size - (*bc).pos }
    }
    gotcount = bc_give(bc, out, count);
    if gotcount != count { return MPG123_ERR as libc::c_int as mpg_ssize_t }
    return gotcount;
}
// stream based operation
unsafe extern "C" fn plain_fullread(mut fr: *mut mpg123_handle_t,
                                    mut buf: *mut byte,
                                    mut count: mpg_ssize_t) -> mpg_ssize_t {
    let mut ret: mpg_ssize_t = 0;
    let mut cnt: mpg_ssize_t = 0 as libc::c_int as mpg_ssize_t;
    // there used to be a check for expected file end here (length value or ID3 flag).
	// this is not needed:
	// 1. EOF is indicated by fdread returning zero bytes anyway.
	// 2. We get false positives of EOF for either files that grew or
	// 3. ... files that have ID3v1 tags in between (stream with intro).
    while cnt < count {
        ret =
            (*fr).rdat.fdread.expect("non-null function pointer")(fr,
                                                                  buf.offset(cnt
                                                                                 as
                                                                                 isize)
                                                                      as
                                                                      *mut libc::c_void,
                                                                  (count -
                                                                       cnt) as
                                                                      size_t);
        if ret < 0 as libc::c_int as libc::c_long {
            return MPG123_ERR as libc::c_int as mpg_ssize_t
        }
        if ret == 0 as libc::c_int as libc::c_long { break ; }
        if (*fr).rdat.flags & 0x8 as libc::c_int == 0 {
            (*fr).rdat.filepos += ret
        }
        cnt += ret
    }
    return cnt;
}
// wrappers for actual reading/seeking... I'm full of wrappers here.
unsafe extern "C" fn io_seek(mut rdat: *mut reader_data_t,
                             mut offset: mpg_off_t, mut whence: libc::c_int)
 -> mpg_off_t {
    if (*rdat).flags & 0x40 as libc::c_int != 0 {
        if (*rdat).r_lseek_handle.is_some() {
            return (*rdat).r_lseek_handle.expect("non-null function pointer")((*rdat).iohandle,
                                                                              offset,
                                                                              whence)
        }
        return -(1 as libc::c_int) as mpg_off_t
    }
    return (*rdat).lseek.expect("non-null function pointer")((*rdat).filept,
                                                             offset, whence);
}
unsafe extern "C" fn io_read(mut rdat: *mut reader_data_t,
                             mut buf: *mut libc::c_void, mut count: size_t)
 -> mpg_ssize_t {
    if (*rdat).flags & 0x40 as libc::c_int != 0 {
        if (*rdat).r_read_handle.is_some() {
            return (*rdat).r_read_handle.expect("non-null function pointer")((*rdat).iohandle,
                                                                             buf,
                                                                             count)
        }
        return -(1 as libc::c_int) as mpg_ssize_t
    }
    return (*rdat).read.expect("non-null function pointer")((*rdat).filept,
                                                            buf, count);
}
// A normal read and a read with timeout.
unsafe extern "C" fn plain_read(mut fr: *mut mpg123_handle_t,
                                mut buf: *mut libc::c_void, mut count: size_t)
 -> mpg_ssize_t {
    return io_read(&mut (*fr).rdat, buf, count);
}
unsafe extern "C" fn stream_lseek(mut fr: *mut mpg123_handle_t,
                                  mut pos: mpg_off_t, mut whence: libc::c_int)
 -> mpg_off_t {
    let mut ret: mpg_off_t = 0;
    ret = io_seek(&mut (*fr).rdat, pos, whence);
    if ret >= 0 as libc::c_int as libc::c_long {
        (*fr).rdat.filepos = ret
    } else {
        (*fr).err = MPG123_LSEEK_FAILED as libc::c_int;
        ret = MPG123_ERR as libc::c_int as mpg_off_t
    }
    return ret;
}
unsafe extern "C" fn stream_close(mut fr: *mut mpg123_handle_t) {
    if (*fr).rdat.flags & 0x1 as libc::c_int != 0 {
        close((*fr).rdat.filept);
    }
    (*fr).rdat.filept = 0 as libc::c_int;
    if (*fr).rdat.flags & 0x8 as libc::c_int != 0 {
        bc_reset(&mut (*fr).rdat.buffer);
    }
    if (*fr).rdat.flags & 0x40 as libc::c_int != 0 {
        if (*fr).rdat.cleanup_handle.is_some() {
            (*fr).rdat.cleanup_handle.expect("non-null function pointer")((*fr).rdat.iohandle);
        }
        (*fr).rdat.iohandle = 0 as *mut libc::c_void
    };
}
unsafe extern "C" fn stream_seek_frame(mut fr: *mut mpg123_handle_t,
                                       mut newframe: mpg_off_t)
 -> libc::c_int {
    // seekable streams can go backwards and jump forwards.
	// non-seekable streams still can go forward, just not jump.
    if (*fr).rdat.flags & 0x4 as libc::c_int != 0 || newframe >= (*fr).num {
        let mut preframe: mpg_off_t = 0; // a leading frame we jump to
        let mut seek_to: mpg_off_t = 0; // the byte offset we want to reach
        let mut to_skip: mpg_off_t =
            0; // bytes to skip to get there (can be negative)
        // now seek to nearest leading index position and read from there until newframe is reached.
		// we use skip_bytes, which handles seekable and non-seekable streams
		// (the latter only for positive offset, which we ensured before entering here).
        seek_to = frame_index_find(fr, newframe, &mut preframe);
        // no need to seek to index position if we are closer already.
		// but I am picky about fr->num == newframe, play safe by reading the frame again.
		// if you think that's stupid, don't call a seek to the current frame.
        if (*fr).num >= newframe || (*fr).num < preframe {
            to_skip =
                seek_to -
                    (*(*fr).rd).tell.expect("non-null function pointer")(fr);
            if (*(*fr).rd).skip_bytes.expect("non-null function pointer")(fr,
                                                                          to_skip)
                   != seek_to {
                return MPG123_ERR as libc::c_int
            }
            (*fr).num = preframe - 1 as libc::c_int as libc::c_long
            // watch out! I am going to read preframe... fr->num should indicate the frame before!
        }
        while (*fr).num < newframe {
            // try to be non-fatal now... frameNum only gets advanced on success anyway
            if read_frame(fr) == 0 { break ; }
        }
        // now the wanted frame should be ready for decoding.
        return MPG123_OK as libc::c_int
    } else {
        (*fr).err = MPG123_NO_SEEK as libc::c_int;
        return MPG123_ERR as libc::c_int
        // invalid, no seek happened
    };
}
// return FALSE on error, TRUE on success, READER_MORE on occasion
unsafe extern "C" fn generic_head_read(mut fr: *mut mpg123_handle_t,
                                       mut newhead: *mut ulong)
 -> libc::c_int {
    let mut hbuf: [byte; 4] = [0; 4];
    let mut ret: libc::c_int =
        (*(*fr).rd).fullread.expect("non-null function pointer")(fr,
                                                                 hbuf.as_mut_ptr(),
                                                                 4 as
                                                                     libc::c_int
                                                                     as
                                                                     mpg_ssize_t)
            as libc::c_int;
    if ret == MPG123_NEED_MORE as libc::c_int { return ret }
    if ret != 4 as libc::c_int { return 0 as libc::c_int }
    *newhead =
        (hbuf[0 as libc::c_int as usize] as ulong) << 24 as libc::c_int |
            (hbuf[1 as libc::c_int as usize] as ulong) << 16 as libc::c_int |
            (hbuf[2 as libc::c_int as usize] as ulong) << 8 as libc::c_int |
            hbuf[3 as libc::c_int as usize] as ulong;
    return (0 as libc::c_int == 0) as libc::c_int;
}
// return FALSE on error, TRUE on success, READER_MORE on occasion
unsafe extern "C" fn generic_head_shift(mut fr: *mut mpg123_handle_t,
                                        mut head: *mut ulong) -> libc::c_int {
    let mut hbuf: byte = 0;
    let mut ret: libc::c_int =
        (*(*fr).rd).fullread.expect("non-null function pointer")(fr,
                                                                 &mut hbuf,
                                                                 1 as
                                                                     libc::c_int
                                                                     as
                                                                     mpg_ssize_t)
            as libc::c_int;
    if ret == MPG123_NEED_MORE as libc::c_int { return ret }
    if ret != 1 as libc::c_int { return 0 as libc::c_int }
    *head <<= 8 as libc::c_int;
    *head |= hbuf as libc::c_ulong;
    *head &= 0xffffffff as libc::c_uint as libc::c_ulong;
    return (0 as libc::c_int == 0) as libc::c_int;
}
// returns reached position... negative ones are bad...
unsafe extern "C" fn stream_skip_bytes(mut fr: *mut mpg123_handle_t,
                                       mut len: mpg_off_t) -> mpg_off_t {
    if (*fr).rdat.flags & 0x4 as libc::c_int != 0 {
        let mut ret: mpg_off_t =
            stream_lseek(fr, len,
                         1 as
                             libc::c_int); // ThOr: Compaq cxx complained and it makes sense to me... or should one do a cast? What for?
        return if ret < 0 as libc::c_int as libc::c_long {
                   MPG123_ERR as libc::c_int as libc::c_long
               } else { ret }
    } else if len >= 0 as libc::c_int as libc::c_long {
        let mut buf: [byte; 1024] =
            [0;
                1024]; // EOF... an error? interface defined to tell the actual position...
        let mut ret_0: mpg_ssize_t = 0;
        while len > 0 as libc::c_int as libc::c_long {
            let mut num: mpg_ssize_t =
                if len <
                       ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong
                           as mpg_off_t {
                    len
                } else {
                    ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong as
                        mpg_ssize_t
                };
            ret_0 =
                (*(*fr).rd).fullread.expect("non-null function pointer")(fr,
                                                                         buf.as_mut_ptr(),
                                                                         num);
            if ret_0 < 0 as libc::c_int as libc::c_long {
                return ret_0
            } else {
                if ret_0 == 0 as libc::c_int as libc::c_long { break ; }
                len -= ret_0
            }
        }
        return (*(*fr).rd).tell.expect("non-null function pointer")(fr)
    } else if (*fr).rdat.flags & 0x8 as libc::c_int != 0 {
        // perhaps we _can_ go a bit back.
        if (*fr).rdat.buffer.pos >= -len {
            (*fr).rdat.buffer.pos += len;
            return (*(*fr).rd).tell.expect("non-null function pointer")(fr)
        } else {
            (*fr).err = MPG123_NO_SEEK as libc::c_int;
            return MPG123_ERR as libc::c_int as mpg_off_t
        }
    } else {
        (*fr).err = MPG123_NO_SEEK as libc::c_int;
        return MPG123_ERR as libc::c_int as mpg_off_t
    };
}
// return 0 on success...
unsafe extern "C" fn stream_back_bytes(mut fr: *mut mpg123_handle_t,
                                       mut bytes: mpg_off_t) -> libc::c_int {
    let mut want: mpg_off_t =
        (*(*fr).rd).tell.expect("non-null function pointer")(fr) - bytes;
    if want < 0 as libc::c_int as libc::c_long {
        return MPG123_ERR as libc::c_int
    }
    if stream_skip_bytes(fr, -bytes) != want {
        return MPG123_ERR as libc::c_int
    }
    return 0 as libc::c_int;
}
// returns size on success...
unsafe extern "C" fn generic_read_frame_body(mut fr: *mut mpg123_handle_t,
                                             mut buf: *mut byte,
                                             mut size: libc::c_int)
 -> libc::c_int {
    let mut l: libc::c_long = 0;
    l =
        (*(*fr).rd).fullread.expect("non-null function pointer")(fr, buf,
                                                                 size as
                                                                     mpg_ssize_t);
    if l != size as libc::c_long { return MPG123_ERR as libc::c_int }
    return l as libc::c_int;
}
unsafe extern "C" fn generic_tell(mut fr: *mut mpg123_handle_t) -> mpg_off_t {
    if (*fr).rdat.flags & 0x8 as libc::c_int != 0 {
        (*fr).rdat.filepos = (*fr).rdat.buffer.fileoff + (*fr).rdat.buffer.pos
    }
    return (*fr).rdat.filepos;
}
// this does not (fully) work for non-seekable streams... You have to check for that flag, pal!
unsafe extern "C" fn stream_rewind(mut fr: *mut mpg123_handle_t) {
    if (*fr).rdat.flags & 0x4 as libc::c_int != 0 {
        (*fr).rdat.filepos =
            stream_lseek(fr, 0 as libc::c_int as mpg_off_t, 0 as libc::c_int);
        (*fr).rdat.buffer.fileoff = (*fr).rdat.filepos
    }
    if (*fr).rdat.flags & 0x8 as libc::c_int != 0 {
        (*fr).rdat.buffer.pos = 0 as libc::c_int as mpg_ssize_t;
        (*fr).rdat.buffer.firstpos = 0 as libc::c_int as mpg_ssize_t;
        (*fr).rdat.filepos = (*fr).rdat.buffer.fileoff
    };
}
// returns length of a file (if filept points to a file)
// reads the last 128 bytes information into buffer
// ... that is not totally safe...
unsafe extern "C" fn get_fileinfo(mut fr: *mut mpg123_handle_t) -> mpg_off_t {
    let mut len: mpg_off_t = 0;
    len =
        io_seek(&mut (*fr).rdat, 0 as libc::c_int as mpg_off_t,
                2 as libc::c_int);
    if len < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as mpg_off_t
    }
    if io_seek(&mut (*fr).rdat, -(128 as libc::c_int) as mpg_off_t,
               2 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as mpg_off_t
    }
    if (*(*fr).rd).fullread.expect("non-null function pointer")(fr,
                                                                (*fr).id3buf.as_mut_ptr(),
                                                                128 as
                                                                    libc::c_int
                                                                    as
                                                                    mpg_ssize_t)
           != 128 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as mpg_off_t
    }
    if strncmp((*fr).id3buf.as_mut_ptr() as *mut libc::c_char,
               b"TAG\x00" as *const u8 as *const libc::c_char,
               3 as libc::c_int as libc::c_ulong) == 0 {
        len -= 128 as libc::c_int as libc::c_long
    }
    if io_seek(&mut (*fr).rdat, 0 as libc::c_int as mpg_off_t,
               0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as mpg_off_t
    }
    if len <= 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as mpg_off_t
    }
    return len;
}
unsafe extern "C" fn bad_init(mut mh: *mut mpg123_handle_t) -> libc::c_int {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int;
}
unsafe extern "C" fn bad_fullread(mut mh: *mut mpg123_handle_t,
                                  mut data: *mut byte, mut count: mpg_ssize_t)
 -> mpg_ssize_t {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int as mpg_ssize_t;
}
unsafe extern "C" fn bad_head_read(mut mh: *mut mpg123_handle_t,
                                   mut newhead: *mut ulong) -> libc::c_int {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int;
}
unsafe extern "C" fn bad_head_shift(mut mh: *mut mpg123_handle_t,
                                    mut head: *mut ulong) -> libc::c_int {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int;
}
unsafe extern "C" fn bad_skip_bytes(mut mh: *mut mpg123_handle_t,
                                    mut len: mpg_off_t) -> mpg_off_t {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int as mpg_off_t;
}
unsafe extern "C" fn bad_read_frame_body(mut mh: *mut mpg123_handle_t,
                                         mut data: *mut byte,
                                         mut size: libc::c_int)
 -> libc::c_int {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int;
}
unsafe extern "C" fn bad_back_bytes(mut mh: *mut mpg123_handle_t,
                                    mut bytes: mpg_off_t) -> libc::c_int {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int;
}
unsafe extern "C" fn bad_seek_frame(mut mh: *mut mpg123_handle_t,
                                    mut num: mpg_off_t) -> libc::c_int {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int;
}
unsafe extern "C" fn bad_tell(mut mh: *mut mpg123_handle_t) -> mpg_off_t {
    (*mh).err = MPG123_NO_READER as libc::c_int;
    return MPG123_ERR as libc::c_int as mpg_off_t;
}
unsafe extern "C" fn bad_rewind(mut mh: *mut mpg123_handle_t) { }
unsafe extern "C" fn bad_close(mut mh: *mut mpg123_handle_t) { }
static mut bad_reader: reader_t =
    unsafe {
        {
            let mut init =
                reader_s{init:
                             Some(bad_init as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t)
                                          -> libc::c_int),
                         close:
                             Some(bad_close as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t)
                                          -> ()),
                         fullread:
                             Some(bad_fullread as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: *mut byte,
                                                           _: mpg_ssize_t)
                                          -> mpg_ssize_t),
                         head_read:
                             Some(bad_head_read as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: *mut ulong)
                                          -> libc::c_int),
                         head_shift:
                             Some(bad_head_shift as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: *mut ulong)
                                          -> libc::c_int),
                         skip_bytes:
                             Some(bad_skip_bytes as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: mpg_off_t)
                                          -> mpg_off_t),
                         read_frame_body:
                             Some(bad_read_frame_body as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: *mut byte,
                                                           _: libc::c_int)
                                          -> libc::c_int),
                         back_bytes:
                             Some(bad_back_bytes as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: mpg_off_t)
                                          -> libc::c_int),
                         seek_frame:
                             Some(bad_seek_frame as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t,
                                                           _: mpg_off_t)
                                          -> libc::c_int),
                         tell:
                             Some(bad_tell as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t)
                                          -> mpg_off_t),
                         rewind:
                             Some(bad_rewind as
                                      unsafe extern "C" fn(_:
                                                               *mut mpg123_handle_t)
                                          -> ()),
                         forget: None,};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn open_bad(mut mh: *mut mpg123_handle_t) {
    (*mh).rd = &mut bad_reader;
    (*mh).rdat.flags = 0 as libc::c_int;
    bc_init(&mut (*mh).rdat.buffer);
    (*mh).rdat.filelen = -(1 as libc::c_int) as mpg_off_t;
}
static mut readers: [reader_t; 3] =
    unsafe {
        [{
             let mut init =
                 reader_s{init:
                              Some(default_init as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> libc::c_int),
                          close:
                              Some(stream_close as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),
                          fullread:
                              Some(plain_fullread as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut byte,
                                                            _: mpg_ssize_t)
                                           -> mpg_ssize_t),
                          head_read:
                              Some(generic_head_read as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut ulong)
                                           -> libc::c_int),
                          head_shift:
                              Some(generic_head_shift as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut ulong)
                                           -> libc::c_int),
                          skip_bytes:
                              Some(stream_skip_bytes as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> mpg_off_t),
                          read_frame_body:
                              Some(generic_read_frame_body as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut byte,
                                                            _: libc::c_int)
                                           -> libc::c_int),
                          back_bytes:
                              Some(stream_back_bytes as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> libc::c_int),
                          seek_frame:
                              Some(stream_seek_frame as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> libc::c_int),
                          tell:
                              Some(generic_tell as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> mpg_off_t),
                          rewind:
                              Some(stream_rewind as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),
                          forget: None,};
             init
         },
         {
             let mut init =
                 reader_s{init:
                              Some(feed_init as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> libc::c_int),
                          close:
                              Some(stream_close as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),
                          fullread:
                              Some(feed_read as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut byte,
                                                            _: mpg_ssize_t)
                                           -> mpg_ssize_t),
                          head_read:
                              Some(generic_head_read as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut ulong)
                                           -> libc::c_int),
                          head_shift:
                              Some(generic_head_shift as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut ulong)
                                           -> libc::c_int),
                          skip_bytes:
                              Some(feed_skip_bytes as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> mpg_off_t),
                          read_frame_body:
                              Some(generic_read_frame_body as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut byte,
                                                            _: libc::c_int)
                                           -> libc::c_int),
                          back_bytes:
                              Some(feed_back_bytes as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> libc::c_int),
                          seek_frame:
                              Some(feed_seek_frame as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> libc::c_int),
                          tell:
                              Some(generic_tell as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> mpg_off_t),
                          rewind:
                              Some(stream_rewind as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),
                          forget:
                              Some(buffered_forget as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),};
             init
         },
         {
             let mut init =
                 reader_s{init:
                              Some(default_init as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> libc::c_int),
                          close:
                              Some(stream_close as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),
                          fullread:
                              Some(buffered_fullread as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut byte,
                                                            _: mpg_ssize_t)
                                           -> mpg_ssize_t),
                          head_read:
                              Some(generic_head_read as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut ulong)
                                           -> libc::c_int),
                          head_shift:
                              Some(generic_head_shift as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut ulong)
                                           -> libc::c_int),
                          skip_bytes:
                              Some(stream_skip_bytes as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> mpg_off_t),
                          read_frame_body:
                              Some(generic_read_frame_body as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: *mut byte,
                                                            _: libc::c_int)
                                           -> libc::c_int),
                          back_bytes:
                              Some(stream_back_bytes as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> libc::c_int),
                          seek_frame:
                              Some(stream_seek_frame as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t,
                                                            _: mpg_off_t)
                                           -> libc::c_int),
                          tell:
                              Some(generic_tell as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> mpg_off_t),
                          rewind:
                              Some(stream_rewind as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),
                          forget:
                              Some(buffered_forget as
                                       unsafe extern "C" fn(_:
                                                                *mut mpg123_handle_t)
                                           -> ()),};
             init
         }]
    };
// final code common to open_stream and open_stream_handle.
unsafe extern "C" fn open_finish(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    (*fr).rd =
        &mut *readers.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut reader_t;
    if (*(*fr).rd).init.expect("non-null function pointer")(fr) <
           0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return MPG123_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_stream_handle(mut fr: *mut mpg123_handle_t,
                                            mut iohandle: *mut libc::c_void)
 -> libc::c_int {
    (*fr).rdat.filelen = -(1 as libc::c_int) as mpg_off_t;
    (*fr).rdat.filept = -(1 as libc::c_int);
    (*fr).rdat.iohandle = iohandle;
    (*fr).rdat.flags = 0 as libc::c_int;
    (*fr).rdat.flags |= 0x40 as libc::c_int;
    return open_finish(fr);
}
#[no_mangle]
pub unsafe extern "C" fn open_feed(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    (*fr).rd =
        &mut *readers.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut reader_t;
    (*fr).rdat.flags = 0 as libc::c_int;
    if (*(*fr).rd).init.expect("non-null function pointer")(fr) <
           0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn default_init(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    (*fr).rdat.fdread =
        Some(plain_read as
                 unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                      _: *mut libc::c_void, _: size_t)
                     -> mpg_ssize_t);
    (*fr).rdat.read =
        if (*fr).rdat.r_read.is_some() {
            (*fr).rdat.r_read
        } else {
            Some(read as
                     unsafe extern "C" fn(_: libc::c_int,
                                          _: *mut libc::c_void, _: size_t)
                         -> ssize_t)
        };
    (*fr).rdat.lseek =
        if (*fr).rdat.r_lseek.is_some() {
            (*fr).rdat.r_lseek
        } else {
            Some(lseek as
                     unsafe extern "C" fn(_: libc::c_int, _: __off_t,
                                          _: libc::c_int) -> __off_t)
        };
    (*fr).rdat.filelen = get_fileinfo(fr);
    (*fr).rdat.filepos = 0 as libc::c_int as mpg_off_t;
    // don't enable seeking on ICY streams, just plain normal files.
	// this check is necessary since the client can enforce ICY parsing on files that would otherwise be seekable.
	// it is a task for the future to make the ICY parsing safe with seeks ... or not.
    if (*fr).rdat.filelen >= 0 as libc::c_int as libc::c_long {
        (*fr).rdat.flags |= 0x4 as libc::c_int;
        if strncmp((*fr).id3buf.as_mut_ptr() as *mut libc::c_char,
                   b"TAG\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            (*fr).rdat.flags |= 0x2 as libc::c_int;
            (*fr).metaflags |= 0x1 as libc::c_int
        }
    } else if (*fr).p.flags & MPG123_SEEKBUFFER as libc::c_int as libc::c_long
                  != 0 {
        // switch reader to a buffered one, if allowed.
        if (*fr).rd ==
               &mut *readers.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut reader_t {
            (*fr).rd =
                &mut *readers.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as
                    *mut reader_t; // we carry the offset, but never know how big the stream is.
            (*fr).rdat.fullread =
                Some(plain_fullread as
                         unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                              _: *mut byte, _: mpg_ssize_t)
                             -> mpg_ssize_t)
        } else { return -(1 as libc::c_int) }
        bc_init(&mut (*fr).rdat.buffer);
        (*fr).rdat.filelen = 0 as libc::c_int as mpg_off_t;
        (*fr).rdat.flags |= 0x8 as libc::c_int
    }
    return 0 as libc::c_int;
}
