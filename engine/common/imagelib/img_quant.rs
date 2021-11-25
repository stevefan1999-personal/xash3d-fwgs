#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type decallist_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Image_CopyParms(src: *mut rgbdata_t);
    #[no_mangle]
    static mut host: host_parm_t;
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
    static PFDesc: [bpc_desc_t; 0];
    #[no_mangle]
    static mut image: imglib_t;
}
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
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
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type fs_offset_t = off_t;
pub type word = libc::c_ushort;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PF_TOTALCOUNT: C2RustUnnamed = 12;
pub const PF_ATI2: C2RustUnnamed = 11;
pub const PF_DXT5: C2RustUnnamed = 10;
pub const PF_DXT3: C2RustUnnamed = 9;
pub const PF_DXT1: C2RustUnnamed = 8;
pub const PF_LUMINANCE: C2RustUnnamed = 7;
pub const PF_BGR_24: C2RustUnnamed = 6;
pub const PF_RGB_24: C2RustUnnamed = 5;
pub const PF_BGRA_32: C2RustUnnamed = 4;
pub const PF_RGBA_32: C2RustUnnamed = 3;
pub const PF_INDEXED_32: C2RustUnnamed = 2;
pub const PF_INDEXED_24: C2RustUnnamed = 1;
pub const PF_UNKNOWN: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type bpc_desc_t = bpc_desc_s;
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
pub type image_hint_t = libc::c_uint;
pub const IL_HINT_HL: image_hint_t = 2;
pub const IL_HINT_Q1: image_hint_t = 1;
pub const IL_HINT_NO: image_hint_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loadformat_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub loadfunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const byte, _: fs_offset_t)
                             -> qboolean>,
    pub hint: image_hint_t,
}
pub type loadpixformat_t = loadformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveformat_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub savefunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut rgbdata_t) -> qboolean>,
}
pub type savepixformat_t = saveformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imglib_s {
    pub loadformats: *const loadpixformat_t,
    pub saveformats: *const savepixformat_t,
    pub width: word,
    pub height: word,
    pub depth: word,
    pub num_mips: byte,
    pub encode: word,
    pub type_0: uint,
    pub flags: uint,
    pub size: size_t,
    pub ptr: uint,
    pub bpp: libc::c_int,
    pub rgba: *mut byte,
    pub source_width: libc::c_int,
    pub source_height: libc::c_int,
    pub source_type: uint,
    pub num_sides: libc::c_int,
    pub cubemap: *mut byte,
    pub d_currentpal: *mut uint,
    pub d_rendermode: libc::c_int,
    pub palette: *mut byte,
    pub fogParams: rgba_t,
    pub hint: image_hint_t,
    pub tempbuffer: *mut byte,
    pub cmd_flags: libc::c_int,
    pub force_flags: libc::c_int,
    pub custom_palette: qboolean,
}
pub type imglib_t = imglib_s;
#[no_mangle]
pub static mut alphadec: libc::c_int = 0;
// types and global variables
static mut thepicture: *mut byte = 0 as *const byte as *mut byte;
// the input image itself
static mut lengthcount: libc::c_int = 0;
// lengthcount = H*W*3
static mut samplefac: libc::c_int = 0;
// sampling factor 1..30
static mut network: [[libc::c_int; 4]; 256] = [[0; 4]; 256];
// the network itself
static mut netindex: [libc::c_int; 256] = [0; 256];
// for network lookup - really 256
static mut bias: [libc::c_int; 256] = [0; 256];
// bias and freq arrays for learning
static mut freq: [libc::c_int; 256] = [0; 256];
static mut radpower: [libc::c_int; 32] = [0; 32];
// radpower for precomputation
#[no_mangle]
pub unsafe extern "C" fn initnet(mut thepic: *mut byte, mut len: libc::c_int,
                                 mut sample: libc::c_int) {
    let mut i: libc::c_int = 0; // 1 / netsize
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    thepicture = thepic;
    lengthcount = len;
    samplefac = sample;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        p = network[i as usize].as_mut_ptr();
        let ref mut fresh0 = *p.offset(2 as libc::c_int as isize);
        *fresh0 =
            (i << 4 as libc::c_int + 8 as libc::c_int) / 256 as libc::c_int;
        let ref mut fresh1 = *p.offset(1 as libc::c_int as isize);
        *fresh1 = *fresh0;
        *p.offset(0 as libc::c_int as isize) = *fresh1;
        freq[i as usize] =
            ((1 as libc::c_int) << 16 as libc::c_int) / 256 as libc::c_int;
        bias[i as usize] = 0 as libc::c_int;
        i += 1
    };
}
// Unbias network to give byte values 0..255 and record position i to prepare for sort
#[no_mangle]
pub unsafe extern "C" fn unbiasnet() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            // OLD CODE: network[i][j] >>= netbiasshift;
			// Fix based on bug report by Juergen Weigert jw@suse.de
            temp =
                network[i as usize][j as usize] +
                    ((1 as libc::c_int) <<
                         4 as libc::c_int - 1 as libc::c_int) >>
                    4 as libc::c_int;
            if temp > 255 as libc::c_int { temp = 255 as libc::c_int }
            network[i as usize][j as usize] = temp;
            j += 1
        }
        network[i as usize][3 as libc::c_int as usize] = i;
        i += 1
        // record colour num
    };
}
// Insertion sort of network and building of netindex[0..255] (to do after unbias)
#[no_mangle]
pub unsafe extern "C" fn inxbuild() {
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int; // index on g
    let mut q: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut smallpos: libc::c_int = 0;
    let mut smallval: libc::c_int = 0;
    let mut previouscol: libc::c_int = 0;
    let mut startpos: libc::c_int = 0;
    previouscol = 0 as libc::c_int;
    startpos = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        p = network[i as usize].as_mut_ptr();
        smallpos = i;
        smallval = *p.offset(1 as libc::c_int as isize);
        // find smallest in i..netsize-1
        j = i + 1 as libc::c_int;
        while j < 256 as libc::c_int {
            q = network[j as usize].as_mut_ptr();
            if *q.offset(1 as libc::c_int as isize) < smallval {
                // index on g
                smallpos = j;
                smallval = *q.offset(1 as libc::c_int as isize)
                // index on g
            }
            j += 1
        }
        q = network[smallpos as usize].as_mut_ptr();
        // swap p (i) and q (smallpos) entries
        if i != smallpos {
            j = *q.offset(0 as libc::c_int as isize);
            *q.offset(0 as libc::c_int as isize) =
                *p.offset(0 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) = j;
            j = *q.offset(1 as libc::c_int as isize);
            *q.offset(1 as libc::c_int as isize) =
                *p.offset(1 as libc::c_int as isize);
            *p.offset(1 as libc::c_int as isize) = j;
            j = *q.offset(2 as libc::c_int as isize);
            *q.offset(2 as libc::c_int as isize) =
                *p.offset(2 as libc::c_int as isize);
            *p.offset(2 as libc::c_int as isize) = j;
            j = *q.offset(3 as libc::c_int as isize);
            *q.offset(3 as libc::c_int as isize) =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(3 as libc::c_int as isize) = j
        }
        // smallval entry is now in position i
        if smallval != previouscol {
            netindex[previouscol as usize] = startpos + i >> 1 as libc::c_int;
            j = previouscol + 1 as libc::c_int;
            while j < smallval { netindex[j as usize] = i; j += 1 }
            previouscol = smallval;
            startpos = i
        }
        i += 1
    }
    netindex[previouscol as usize] =
        startpos + (256 as libc::c_int - 1 as libc::c_int) >>
            1 as libc::c_int;
    j = previouscol + 1 as libc::c_int;
    while j < 256 as libc::c_int {
        netindex[j as usize] = 256 as libc::c_int - 1 as libc::c_int;
        j += 1
    };
    // really 256
}
// Search for BGR values 0..255 (after net is unbiased) and return colour index
#[no_mangle]
pub unsafe extern "C" fn inxsearch(mut r: libc::c_int, mut g: libc::c_int,
                                   mut b: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0; // biggest possible dist is 256 * 3
    let mut j: libc::c_int = 0; // index on g
    let mut dist: libc::c_int = 0; // start at netindex[g] and work outwards
    let mut a: libc::c_int = 0; // inx key
    let mut bestd: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut best: libc::c_int = 0;
    bestd = 1000 as libc::c_int;
    best = -(1 as libc::c_int);
    i = netindex[g as usize];
    j = i - 1 as libc::c_int;
    while i < 256 as libc::c_int || j >= 0 as libc::c_int {
        if i < 256 as libc::c_int {
            p = network[i as usize].as_mut_ptr();
            dist = *p.offset(1 as libc::c_int as isize) - g;
            if dist >= bestd {
                i = 256 as libc::c_int
                // stop iter
            } else {
                i += 1; // inx key - reverse dif
                if dist < 0 as libc::c_int { dist = -dist }
                a = *p.offset(2 as libc::c_int as isize) - b;
                if a < 0 as libc::c_int { a = -a }
                dist += a;
                if dist < bestd {
                    a = *p.offset(0 as libc::c_int as isize) - r;
                    if a < 0 as libc::c_int { a = -a }
                    dist += a;
                    if dist < bestd {
                        bestd = dist;
                        best = *p.offset(3 as libc::c_int as isize)
                    }
                }
            }
        }
        if j >= 0 as libc::c_int {
            p = network[j as usize].as_mut_ptr();
            dist = g - *p.offset(1 as libc::c_int as isize);
            if dist >= bestd {
                j = -(1 as libc::c_int)
                // stop iter
            } else {
                j -= 1;
                if dist < 0 as libc::c_int { dist = -dist }
                a = *p.offset(2 as libc::c_int as isize) - b;
                if a < 0 as libc::c_int { a = -a }
                dist += a;
                if dist < bestd {
                    a = *p.offset(0 as libc::c_int as isize) - r;
                    if a < 0 as libc::c_int { a = -a }
                    dist += a;
                    if dist < bestd {
                        bestd = dist;
                        best = *p.offset(3 as libc::c_int as isize)
                    }
                }
            }
        }
    }
    return best;
}
// Search for biased BGR values
#[no_mangle]
pub unsafe extern "C" fn contest(mut r: libc::c_int, mut g: libc::c_int,
                                 mut b: libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut f: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut biasdist: libc::c_int = 0;
    let mut betafreq: libc::c_int = 0;
    let mut bestpos: libc::c_int = 0;
    let mut bestbiaspos: libc::c_int = 0;
    let mut bestd: libc::c_int = 0;
    let mut bestbiasd: libc::c_int = 0;
    // finds closest neuron (min dist) and updates freq
	// finds best neuron (min dist-bias) and returns position
	// for frequently chosen neurons, freq[i] is high and bias[i] is negative
	// bias[i] = gamma * ((1 / netsize) - freq[i])
    bestd = 2147483647 as libc::c_int;
    bestbiasd = bestd;
    bestpos = -(1 as libc::c_int);
    bestbiaspos = bestpos;
    p = bias.as_mut_ptr();
    f = freq.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        n = network[i as usize].as_mut_ptr();
        dist = *n.offset(2 as libc::c_int as isize) - b;
        if dist < 0 as libc::c_int { dist = -dist }
        a = *n.offset(1 as libc::c_int as isize) - g;
        if a < 0 as libc::c_int { a = -a }
        dist += a;
        a = *n.offset(0 as libc::c_int as isize) - r;
        if a < 0 as libc::c_int { a = -a }
        dist += a;
        if dist < bestd { bestd = dist; bestpos = i }
        biasdist = dist - (*p >> 16 as libc::c_int - 4 as libc::c_int);
        if biasdist < bestbiasd { bestbiasd = biasdist; bestbiaspos = i }
        betafreq = *f >> 10 as libc::c_int;
        let fresh2 = f;
        f = f.offset(1);
        *fresh2 -= betafreq;
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 += betafreq << 10 as libc::c_int;
        i += 1
    }
    freq[bestpos as usize] +=
        (1 as libc::c_int) << 16 as libc::c_int >> 10 as libc::c_int;
    bias[bestpos as usize] -=
        ((1 as libc::c_int) << 16 as libc::c_int) <<
            10 as libc::c_int - 10 as libc::c_int;
    return bestbiaspos;
}
// Move neuron i towards biased (b,g,r) by factor alpha
#[no_mangle]
pub unsafe extern "C" fn altersingle(mut alpha: libc::c_int,
                                     mut i: libc::c_int, mut r: libc::c_int,
                                     mut g: libc::c_int, mut b: libc::c_int) {
    let mut n: *mut libc::c_int = 0 as *mut libc::c_int; // alter hit neuron
    n = network[i as usize].as_mut_ptr();
    *n -= alpha * (*n - r) / ((1 as libc::c_int) << 10 as libc::c_int);
    n = n.offset(1);
    *n -= alpha * (*n - g) / ((1 as libc::c_int) << 10 as libc::c_int);
    n = n.offset(1);
    *n -= alpha * (*n - b) / ((1 as libc::c_int) << 10 as libc::c_int);
}
// Move adjacent neurons by precomputed alpha*(1-((i-j)^2/[r]^2)) in radpower[|i-j|]
#[no_mangle]
pub unsafe extern "C" fn alterneigh(mut rad: libc::c_int, mut i: libc::c_int,
                                    mut r: libc::c_int, mut g: libc::c_int,
                                    mut b: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut q: *mut libc::c_int = 0 as *mut libc::c_int;
    lo = i - rad;
    if lo < -(1 as libc::c_int) { lo = -(1 as libc::c_int) }
    hi = i + rad;
    if hi > 256 as libc::c_int { hi = 256 as libc::c_int }
    j = i + 1 as libc::c_int;
    k = i - 1 as libc::c_int;
    q = radpower.as_mut_ptr();
    while j < hi || k > lo {
        q = q.offset(1);
        a = *q;
        if j < hi {
            p = network[j as usize].as_mut_ptr();
            *p -=
                a * (*p - r) /
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int);
            p = p.offset(1);
            *p -=
                a * (*p - g) /
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int);
            p = p.offset(1);
            *p -=
                a * (*p - b) /
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int);
            j += 1
        }
        if k > lo {
            p = network[k as usize].as_mut_ptr();
            *p -=
                a * (*p - r) /
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int);
            p = p.offset(1);
            *p -=
                a * (*p - g) /
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int);
            p = p.offset(1);
            *p -=
                a * (*p - b) /
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int);
            k -= 1
        }
    };
}
// Main Learning Loop
#[no_mangle]
pub unsafe extern "C" fn learn() {
    let mut p: *mut byte = 0 as *mut byte; // alter neighbours
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut radius: libc::c_int = 0;
    let mut rad: libc::c_int = 0;
    let mut alpha: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut samplepixels: libc::c_int = 0;
    let mut lim: *mut byte = 0 as *mut byte;
    alphadec =
        30 as libc::c_int + (samplefac - 1 as libc::c_int) / 3 as libc::c_int;
    p = thepicture;
    lim = thepicture.offset(lengthcount as isize);
    samplepixels = lengthcount / (image.bpp * samplefac);
    delta = samplepixels / 100 as libc::c_int;
    alpha = (1 as libc::c_int) << 10 as libc::c_int;
    radius =
        (256 as libc::c_int >> 3 as libc::c_int) *
            ((1 as libc::c_int) << 6 as libc::c_int);
    rad = radius >> 6 as libc::c_int;
    if rad <= 1 as libc::c_int { rad = 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < rad {
        radpower[i as usize] =
            alpha *
                ((rad * rad - i * i) *
                     ((1 as libc::c_int) << 8 as libc::c_int) / (rad * rad));
        i += 1
    }
    if delta <= 0 as libc::c_int { return }
    if lengthcount % 499 as libc::c_int != 0 as libc::c_int {
        step = 499 as libc::c_int * image.bpp
    } else if lengthcount % 491 as libc::c_int != 0 as libc::c_int {
        step = 491 as libc::c_int * image.bpp
    } else if lengthcount % 487 as libc::c_int != 0 as libc::c_int {
        step = 487 as libc::c_int * image.bpp
    } else { step = 503 as libc::c_int * image.bpp }
    i = 0 as libc::c_int;
    while i < samplepixels {
        r =
            (*p.offset(0 as libc::c_int as isize) as libc::c_int) <<
                4 as libc::c_int;
        g =
            (*p.offset(1 as libc::c_int as isize) as libc::c_int) <<
                4 as libc::c_int;
        b =
            (*p.offset(2 as libc::c_int as isize) as libc::c_int) <<
                4 as libc::c_int;
        j = contest(r, g, b);
        altersingle(alpha, j, r, g, b);
        if rad != 0 { alterneigh(rad, j, r, g, b); }
        p = p.offset(step as isize);
        if p >= lim { p = p.offset(-(lengthcount as isize)) }
        i += 1;
        if i % delta == 0 as libc::c_int {
            alpha -= alpha / alphadec;
            radius -= radius / 30 as libc::c_int;
            rad = radius >> 6 as libc::c_int;
            if rad <= 1 as libc::c_int { rad = 0 as libc::c_int }
            j = 0 as libc::c_int;
            while j < rad {
                radpower[j as usize] =
                    alpha *
                        ((rad * rad - j * j) *
                             ((1 as libc::c_int) << 8 as libc::c_int) /
                             (rad * rad));
                j += 1
            }
        }
    };
}
// returns the actual number of palette entries.
#[no_mangle]
pub unsafe extern "C" fn Image_Quantize(mut pic: *mut rgbdata_t)
 -> *mut rgbdata_t {
    let mut i: libc::c_int = 0;
    // quick case to reject unneeded conversions
    if (*pic).type_0 == PF_INDEXED_24 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_INDEXED_32 as libc::c_int as libc::c_uint {
        return pic
    }
    Image_CopyParms(pic);
    image.size =
        (image.width as libc::c_int * image.height as libc::c_int) as size_t;
    image.bpp = (*PFDesc.as_ptr().offset((*pic).type_0 as isize)).bpp;
    image.ptr = 0 as libc::c_int as uint;
    // allocate 8-bit buffer
    image.tempbuffer =
        _Mem_Realloc(host.imagepool, image.tempbuffer as *mut libc::c_void,
                     image.size, true_0,
                     b"../engine/common/imagelib/img_quant.c\x00" as *const u8
                         as *const libc::c_char, 443 as libc::c_int) as
            *mut byte; // red
    initnet((*pic).buffer, (*pic).size as libc::c_int, 10 as libc::c_int);
    learn();
    unbiasnet();
    (*pic).palette =
        _Mem_Alloc(host.imagepool,
                   (256 as libc::c_int * 3 as libc::c_int) as size_t, false_0,
                   b"../engine/common/imagelib/img_quant.c\x00" as *const u8
                       as *const libc::c_char, 449 as libc::c_int) as
            *mut byte;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        *(*pic).palette.offset((i * 3 as libc::c_int + 0 as libc::c_int) as
                                   isize) =
            network[i as usize][0 as libc::c_int as usize] as byte;
        // blue
        *(*pic).palette.offset((i * 3 as libc::c_int + 1 as libc::c_int) as
                                   isize) =
            network[i as usize][1 as libc::c_int as usize] as byte; // green
        *(*pic).palette.offset((i * 3 as libc::c_int + 2 as libc::c_int) as
                                   isize) =
            network[i as usize][2 as libc::c_int as usize] as byte;
        i += 1
    }
    inxbuild();
    i = 0 as libc::c_int;
    while i < image.width as libc::c_int * image.height as libc::c_int {
        *image.tempbuffer.offset(i as isize) =
            inxsearch(*(*pic).buffer.offset((i * image.bpp + 0 as libc::c_int)
                                                as isize) as libc::c_int,
                      *(*pic).buffer.offset((i * image.bpp + 1 as libc::c_int)
                                                as isize) as libc::c_int,
                      *(*pic).buffer.offset((i * image.bpp + 2 as libc::c_int)
                                                as isize) as libc::c_int) as
                byte;
        i += 1
    }
    (*pic).buffer =
        _Mem_Realloc(host.imagepool, (*pic).buffer as *mut libc::c_void,
                     image.size, true_0,
                     b"../engine/common/imagelib/img_quant.c\x00" as *const u8
                         as *const libc::c_char, 465 as libc::c_int) as
            *mut byte;
    memcpy((*pic).buffer as *mut libc::c_void,
           image.tempbuffer as *const libc::c_void, image.size);
    (*pic).type_0 = PF_INDEXED_24 as libc::c_int as uint;
    (*pic).size = image.size;
    return pic;
}
