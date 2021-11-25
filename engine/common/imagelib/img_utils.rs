#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    fn _Mem_Check(filename: *const libc::c_char, fileline: libc::c_int);
    #[no_mangle]
    fn Image_SavePNG(name: *const libc::c_char, pix: *mut rgbdata_t)
     -> qboolean;
    #[no_mangle]
    fn Image_SaveBMP(name: *const libc::c_char, pix: *mut rgbdata_t)
     -> qboolean;
    #[no_mangle]
    fn Image_SaveTGA(name: *const libc::c_char, pix: *mut rgbdata_t)
     -> qboolean;
    #[no_mangle]
    static mut image: imglib_t;
    #[no_mangle]
    fn Image_LoadPAL(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadFNT(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadLMP(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadSPR(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadMDL(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadMIP(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadPNG(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadBMP(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadTGA(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Image_LoadDDS(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn FS_LoadImage(filename: *const libc::c_char, buffer: *const byte,
                    size: size_t) -> *mut rgbdata_t;
    #[no_mangle]
    fn FS_FreeImage(pack: *mut rgbdata_t);
    #[no_mangle]
    static PFDesc: [bpc_desc_t; 0];
    #[no_mangle]
    fn Image_Quantize(pic: *mut rgbdata_t) -> *mut rgbdata_t;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Image_Reset();
    #[no_mangle]
    fn LightToTexGamma(b: byte) -> byte;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PF_TOTALCOUNT: C2RustUnnamed_0 = 12;
pub const PF_ATI2: C2RustUnnamed_0 = 11;
pub const PF_DXT5: C2RustUnnamed_0 = 10;
pub const PF_DXT3: C2RustUnnamed_0 = 9;
pub const PF_DXT1: C2RustUnnamed_0 = 8;
pub const PF_LUMINANCE: C2RustUnnamed_0 = 7;
pub const PF_BGR_24: C2RustUnnamed_0 = 6;
pub const PF_RGB_24: C2RustUnnamed_0 = 5;
pub const PF_BGRA_32: C2RustUnnamed_0 = 4;
pub const PF_RGBA_32: C2RustUnnamed_0 = 3;
pub const PF_INDEXED_32: C2RustUnnamed_0 = 2;
pub const PF_INDEXED_24: C2RustUnnamed_0 = 1;
pub const PF_UNKNOWN: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type bpc_desc_t = bpc_desc_s;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_1 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_1 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_1 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_1 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_1 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_1 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_2 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_2 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_2 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_2 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_2 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_2 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_2 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_2 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_2 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_2 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_2 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_2 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_2 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_2 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_2 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_2 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_2 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_2 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_2 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_2 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_2 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_2 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_2 = 1;
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
pub type savepixformat_t = saveformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveformat_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub savefunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut rgbdata_t) -> qboolean>,
}
pub type imglib_t = imglib_s;
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
pub type image_hint_t = libc::c_uint;
pub const IL_HINT_HL: image_hint_t = 2;
pub const IL_HINT_Q1: image_hint_t = 1;
pub const IL_HINT_NO: image_hint_t = 0;
pub type loadpixformat_t = loadformat_s;
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
pub const LUMP_NORMAL: C2RustUnnamed_3 = 0;
pub const LUMP_HALFLIFE: C2RustUnnamed_3 = 4;
pub const LUMP_EXTENDED: C2RustUnnamed_3 = 3;
pub const LUMP_MASKED: C2RustUnnamed_3 = 1;
pub const LUMP_GRADIENT: C2RustUnnamed_3 = 2;
pub const LUMP_QUAKE1: C2RustUnnamed_3 = 5;
pub const PAL_QUAKE1: C2RustUnnamed_4 = 1;
pub const PAL_CUSTOM: C2RustUnnamed_4 = 0;
pub const PAL_HALFLIFE: C2RustUnnamed_4 = 2;
pub const PAL_INVALID: C2RustUnnamed_4 = -1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_int;
#[no_mangle]
pub static mut d_8toQ1table: [uint; 256] = [0; 256];
#[no_mangle]
pub static mut d_8toHLtable: [uint; 256] = [0; 256];
#[no_mangle]
pub static mut d_8to24table: [uint; 256] = [0; 256];
#[no_mangle]
pub static mut q1palette_init: qboolean = false_0;
#[no_mangle]
pub static mut hlpalette_init: qboolean = false_0;
static mut palette_q1: [byte; 768] =
    [0 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 15 as libc::c_int as byte,
     15 as libc::c_int as byte, 15 as libc::c_int as byte,
     31 as libc::c_int as byte, 31 as libc::c_int as byte,
     31 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     63 as libc::c_int as byte, 63 as libc::c_int as byte,
     63 as libc::c_int as byte, 75 as libc::c_int as byte,
     75 as libc::c_int as byte, 75 as libc::c_int as byte,
     91 as libc::c_int as byte, 91 as libc::c_int as byte,
     91 as libc::c_int as byte, 107 as libc::c_int as byte,
     107 as libc::c_int as byte, 107 as libc::c_int as byte,
     123 as libc::c_int as byte, 123 as libc::c_int as byte,
     123 as libc::c_int as byte, 139 as libc::c_int as byte,
     139 as libc::c_int as byte, 139 as libc::c_int as byte,
     155 as libc::c_int as byte, 155 as libc::c_int as byte,
     155 as libc::c_int as byte, 171 as libc::c_int as byte,
     171 as libc::c_int as byte, 171 as libc::c_int as byte,
     187 as libc::c_int as byte, 187 as libc::c_int as byte,
     187 as libc::c_int as byte, 203 as libc::c_int as byte,
     203 as libc::c_int as byte, 203 as libc::c_int as byte,
     219 as libc::c_int as byte, 219 as libc::c_int as byte,
     219 as libc::c_int as byte, 235 as libc::c_int as byte,
     235 as libc::c_int as byte, 235 as libc::c_int as byte,
     15 as libc::c_int as byte, 11 as libc::c_int as byte,
     7 as libc::c_int as byte, 23 as libc::c_int as byte,
     15 as libc::c_int as byte, 11 as libc::c_int as byte,
     31 as libc::c_int as byte, 23 as libc::c_int as byte,
     11 as libc::c_int as byte, 39 as libc::c_int as byte,
     27 as libc::c_int as byte, 15 as libc::c_int as byte,
     47 as libc::c_int as byte, 35 as libc::c_int as byte,
     19 as libc::c_int as byte, 55 as libc::c_int as byte,
     43 as libc::c_int as byte, 23 as libc::c_int as byte,
     63 as libc::c_int as byte, 47 as libc::c_int as byte,
     23 as libc::c_int as byte, 75 as libc::c_int as byte,
     55 as libc::c_int as byte, 27 as libc::c_int as byte,
     83 as libc::c_int as byte, 59 as libc::c_int as byte,
     27 as libc::c_int as byte, 91 as libc::c_int as byte,
     67 as libc::c_int as byte, 31 as libc::c_int as byte,
     99 as libc::c_int as byte, 75 as libc::c_int as byte,
     31 as libc::c_int as byte, 107 as libc::c_int as byte,
     83 as libc::c_int as byte, 31 as libc::c_int as byte,
     115 as libc::c_int as byte, 87 as libc::c_int as byte,
     31 as libc::c_int as byte, 123 as libc::c_int as byte,
     95 as libc::c_int as byte, 35 as libc::c_int as byte,
     131 as libc::c_int as byte, 103 as libc::c_int as byte,
     35 as libc::c_int as byte, 143 as libc::c_int as byte,
     111 as libc::c_int as byte, 35 as libc::c_int as byte,
     11 as libc::c_int as byte, 11 as libc::c_int as byte,
     15 as libc::c_int as byte, 19 as libc::c_int as byte,
     19 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 27 as libc::c_int as byte,
     39 as libc::c_int as byte, 39 as libc::c_int as byte,
     39 as libc::c_int as byte, 51 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     63 as libc::c_int as byte, 55 as libc::c_int as byte,
     55 as libc::c_int as byte, 75 as libc::c_int as byte,
     63 as libc::c_int as byte, 63 as libc::c_int as byte,
     87 as libc::c_int as byte, 71 as libc::c_int as byte,
     71 as libc::c_int as byte, 103 as libc::c_int as byte,
     79 as libc::c_int as byte, 79 as libc::c_int as byte,
     115 as libc::c_int as byte, 91 as libc::c_int as byte,
     91 as libc::c_int as byte, 127 as libc::c_int as byte,
     99 as libc::c_int as byte, 99 as libc::c_int as byte,
     139 as libc::c_int as byte, 107 as libc::c_int as byte,
     107 as libc::c_int as byte, 151 as libc::c_int as byte,
     115 as libc::c_int as byte, 115 as libc::c_int as byte,
     163 as libc::c_int as byte, 123 as libc::c_int as byte,
     123 as libc::c_int as byte, 175 as libc::c_int as byte,
     131 as libc::c_int as byte, 131 as libc::c_int as byte,
     187 as libc::c_int as byte, 139 as libc::c_int as byte,
     139 as libc::c_int as byte, 203 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 7 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     11 as libc::c_int as byte, 11 as libc::c_int as byte,
     0 as libc::c_int as byte, 19 as libc::c_int as byte,
     19 as libc::c_int as byte, 0 as libc::c_int as byte,
     27 as libc::c_int as byte, 27 as libc::c_int as byte,
     0 as libc::c_int as byte, 35 as libc::c_int as byte,
     35 as libc::c_int as byte, 0 as libc::c_int as byte,
     43 as libc::c_int as byte, 43 as libc::c_int as byte,
     7 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 7 as libc::c_int as byte,
     55 as libc::c_int as byte, 55 as libc::c_int as byte,
     7 as libc::c_int as byte, 63 as libc::c_int as byte,
     63 as libc::c_int as byte, 7 as libc::c_int as byte,
     71 as libc::c_int as byte, 71 as libc::c_int as byte,
     7 as libc::c_int as byte, 75 as libc::c_int as byte,
     75 as libc::c_int as byte, 11 as libc::c_int as byte,
     83 as libc::c_int as byte, 83 as libc::c_int as byte,
     11 as libc::c_int as byte, 91 as libc::c_int as byte,
     91 as libc::c_int as byte, 11 as libc::c_int as byte,
     99 as libc::c_int as byte, 99 as libc::c_int as byte,
     11 as libc::c_int as byte, 107 as libc::c_int as byte,
     107 as libc::c_int as byte, 15 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 15 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     23 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 31 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     39 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 47 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     55 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 63 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     71 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 79 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     87 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 95 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     103 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 111 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     119 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 127 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     0 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 0 as libc::c_int as byte,
     35 as libc::c_int as byte, 35 as libc::c_int as byte,
     0 as libc::c_int as byte, 47 as libc::c_int as byte,
     43 as libc::c_int as byte, 0 as libc::c_int as byte,
     55 as libc::c_int as byte, 47 as libc::c_int as byte,
     0 as libc::c_int as byte, 67 as libc::c_int as byte,
     55 as libc::c_int as byte, 0 as libc::c_int as byte,
     75 as libc::c_int as byte, 59 as libc::c_int as byte,
     7 as libc::c_int as byte, 87 as libc::c_int as byte,
     67 as libc::c_int as byte, 7 as libc::c_int as byte,
     95 as libc::c_int as byte, 71 as libc::c_int as byte,
     7 as libc::c_int as byte, 107 as libc::c_int as byte,
     75 as libc::c_int as byte, 11 as libc::c_int as byte,
     119 as libc::c_int as byte, 83 as libc::c_int as byte,
     15 as libc::c_int as byte, 131 as libc::c_int as byte,
     87 as libc::c_int as byte, 19 as libc::c_int as byte,
     139 as libc::c_int as byte, 91 as libc::c_int as byte,
     19 as libc::c_int as byte, 151 as libc::c_int as byte,
     95 as libc::c_int as byte, 27 as libc::c_int as byte,
     163 as libc::c_int as byte, 99 as libc::c_int as byte,
     31 as libc::c_int as byte, 175 as libc::c_int as byte,
     103 as libc::c_int as byte, 35 as libc::c_int as byte,
     35 as libc::c_int as byte, 19 as libc::c_int as byte,
     7 as libc::c_int as byte, 47 as libc::c_int as byte,
     23 as libc::c_int as byte, 11 as libc::c_int as byte,
     59 as libc::c_int as byte, 31 as libc::c_int as byte,
     15 as libc::c_int as byte, 75 as libc::c_int as byte,
     35 as libc::c_int as byte, 19 as libc::c_int as byte,
     87 as libc::c_int as byte, 43 as libc::c_int as byte,
     23 as libc::c_int as byte, 99 as libc::c_int as byte,
     47 as libc::c_int as byte, 31 as libc::c_int as byte,
     115 as libc::c_int as byte, 55 as libc::c_int as byte,
     35 as libc::c_int as byte, 127 as libc::c_int as byte,
     59 as libc::c_int as byte, 43 as libc::c_int as byte,
     143 as libc::c_int as byte, 67 as libc::c_int as byte,
     51 as libc::c_int as byte, 159 as libc::c_int as byte,
     79 as libc::c_int as byte, 51 as libc::c_int as byte,
     175 as libc::c_int as byte, 99 as libc::c_int as byte,
     47 as libc::c_int as byte, 191 as libc::c_int as byte,
     119 as libc::c_int as byte, 47 as libc::c_int as byte,
     207 as libc::c_int as byte, 143 as libc::c_int as byte,
     43 as libc::c_int as byte, 223 as libc::c_int as byte,
     171 as libc::c_int as byte, 39 as libc::c_int as byte,
     239 as libc::c_int as byte, 203 as libc::c_int as byte,
     31 as libc::c_int as byte, 255 as libc::c_int as byte,
     243 as libc::c_int as byte, 27 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     0 as libc::c_int as byte, 27 as libc::c_int as byte,
     19 as libc::c_int as byte, 0 as libc::c_int as byte,
     43 as libc::c_int as byte, 35 as libc::c_int as byte,
     15 as libc::c_int as byte, 55 as libc::c_int as byte,
     43 as libc::c_int as byte, 19 as libc::c_int as byte,
     71 as libc::c_int as byte, 51 as libc::c_int as byte,
     27 as libc::c_int as byte, 83 as libc::c_int as byte,
     55 as libc::c_int as byte, 35 as libc::c_int as byte,
     99 as libc::c_int as byte, 63 as libc::c_int as byte,
     43 as libc::c_int as byte, 111 as libc::c_int as byte,
     71 as libc::c_int as byte, 51 as libc::c_int as byte,
     127 as libc::c_int as byte, 83 as libc::c_int as byte,
     63 as libc::c_int as byte, 139 as libc::c_int as byte,
     95 as libc::c_int as byte, 71 as libc::c_int as byte,
     155 as libc::c_int as byte, 107 as libc::c_int as byte,
     83 as libc::c_int as byte, 167 as libc::c_int as byte,
     123 as libc::c_int as byte, 95 as libc::c_int as byte,
     183 as libc::c_int as byte, 135 as libc::c_int as byte,
     107 as libc::c_int as byte, 195 as libc::c_int as byte,
     147 as libc::c_int as byte, 123 as libc::c_int as byte,
     211 as libc::c_int as byte, 163 as libc::c_int as byte,
     139 as libc::c_int as byte, 227 as libc::c_int as byte,
     179 as libc::c_int as byte, 151 as libc::c_int as byte,
     171 as libc::c_int as byte, 139 as libc::c_int as byte,
     163 as libc::c_int as byte, 159 as libc::c_int as byte,
     127 as libc::c_int as byte, 151 as libc::c_int as byte,
     147 as libc::c_int as byte, 115 as libc::c_int as byte,
     135 as libc::c_int as byte, 139 as libc::c_int as byte,
     103 as libc::c_int as byte, 123 as libc::c_int as byte,
     127 as libc::c_int as byte, 91 as libc::c_int as byte,
     111 as libc::c_int as byte, 119 as libc::c_int as byte,
     83 as libc::c_int as byte, 99 as libc::c_int as byte,
     107 as libc::c_int as byte, 75 as libc::c_int as byte,
     87 as libc::c_int as byte, 95 as libc::c_int as byte,
     63 as libc::c_int as byte, 75 as libc::c_int as byte,
     87 as libc::c_int as byte, 55 as libc::c_int as byte,
     67 as libc::c_int as byte, 75 as libc::c_int as byte,
     47 as libc::c_int as byte, 55 as libc::c_int as byte,
     67 as libc::c_int as byte, 39 as libc::c_int as byte,
     47 as libc::c_int as byte, 55 as libc::c_int as byte,
     31 as libc::c_int as byte, 35 as libc::c_int as byte,
     43 as libc::c_int as byte, 23 as libc::c_int as byte,
     27 as libc::c_int as byte, 35 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     23 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 15 as libc::c_int as byte,
     7 as libc::c_int as byte, 7 as libc::c_int as byte,
     187 as libc::c_int as byte, 115 as libc::c_int as byte,
     159 as libc::c_int as byte, 175 as libc::c_int as byte,
     107 as libc::c_int as byte, 143 as libc::c_int as byte,
     163 as libc::c_int as byte, 95 as libc::c_int as byte,
     131 as libc::c_int as byte, 151 as libc::c_int as byte,
     87 as libc::c_int as byte, 119 as libc::c_int as byte,
     139 as libc::c_int as byte, 79 as libc::c_int as byte,
     107 as libc::c_int as byte, 127 as libc::c_int as byte,
     75 as libc::c_int as byte, 95 as libc::c_int as byte,
     115 as libc::c_int as byte, 67 as libc::c_int as byte,
     83 as libc::c_int as byte, 107 as libc::c_int as byte,
     59 as libc::c_int as byte, 75 as libc::c_int as byte,
     95 as libc::c_int as byte, 51 as libc::c_int as byte,
     63 as libc::c_int as byte, 83 as libc::c_int as byte,
     43 as libc::c_int as byte, 55 as libc::c_int as byte,
     71 as libc::c_int as byte, 35 as libc::c_int as byte,
     43 as libc::c_int as byte, 59 as libc::c_int as byte,
     31 as libc::c_int as byte, 35 as libc::c_int as byte,
     47 as libc::c_int as byte, 23 as libc::c_int as byte,
     27 as libc::c_int as byte, 35 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     23 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 15 as libc::c_int as byte,
     7 as libc::c_int as byte, 7 as libc::c_int as byte,
     219 as libc::c_int as byte, 195 as libc::c_int as byte,
     187 as libc::c_int as byte, 203 as libc::c_int as byte,
     179 as libc::c_int as byte, 167 as libc::c_int as byte,
     191 as libc::c_int as byte, 163 as libc::c_int as byte,
     155 as libc::c_int as byte, 175 as libc::c_int as byte,
     151 as libc::c_int as byte, 139 as libc::c_int as byte,
     163 as libc::c_int as byte, 135 as libc::c_int as byte,
     123 as libc::c_int as byte, 151 as libc::c_int as byte,
     123 as libc::c_int as byte, 111 as libc::c_int as byte,
     135 as libc::c_int as byte, 111 as libc::c_int as byte,
     95 as libc::c_int as byte, 123 as libc::c_int as byte,
     99 as libc::c_int as byte, 83 as libc::c_int as byte,
     107 as libc::c_int as byte, 87 as libc::c_int as byte,
     71 as libc::c_int as byte, 95 as libc::c_int as byte,
     75 as libc::c_int as byte, 59 as libc::c_int as byte,
     83 as libc::c_int as byte, 63 as libc::c_int as byte,
     51 as libc::c_int as byte, 67 as libc::c_int as byte,
     51 as libc::c_int as byte, 39 as libc::c_int as byte,
     55 as libc::c_int as byte, 43 as libc::c_int as byte,
     31 as libc::c_int as byte, 39 as libc::c_int as byte,
     31 as libc::c_int as byte, 23 as libc::c_int as byte,
     27 as libc::c_int as byte, 19 as libc::c_int as byte,
     15 as libc::c_int as byte, 15 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     111 as libc::c_int as byte, 131 as libc::c_int as byte,
     123 as libc::c_int as byte, 103 as libc::c_int as byte,
     123 as libc::c_int as byte, 111 as libc::c_int as byte,
     95 as libc::c_int as byte, 115 as libc::c_int as byte,
     103 as libc::c_int as byte, 87 as libc::c_int as byte,
     107 as libc::c_int as byte, 95 as libc::c_int as byte,
     79 as libc::c_int as byte, 99 as libc::c_int as byte,
     87 as libc::c_int as byte, 71 as libc::c_int as byte,
     91 as libc::c_int as byte, 79 as libc::c_int as byte,
     63 as libc::c_int as byte, 83 as libc::c_int as byte,
     71 as libc::c_int as byte, 55 as libc::c_int as byte,
     75 as libc::c_int as byte, 63 as libc::c_int as byte,
     47 as libc::c_int as byte, 67 as libc::c_int as byte,
     55 as libc::c_int as byte, 43 as libc::c_int as byte,
     59 as libc::c_int as byte, 47 as libc::c_int as byte,
     35 as libc::c_int as byte, 51 as libc::c_int as byte,
     39 as libc::c_int as byte, 31 as libc::c_int as byte,
     43 as libc::c_int as byte, 31 as libc::c_int as byte,
     23 as libc::c_int as byte, 35 as libc::c_int as byte,
     23 as libc::c_int as byte, 15 as libc::c_int as byte,
     27 as libc::c_int as byte, 19 as libc::c_int as byte,
     11 as libc::c_int as byte, 19 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     255 as libc::c_int as byte, 243 as libc::c_int as byte,
     27 as libc::c_int as byte, 239 as libc::c_int as byte,
     223 as libc::c_int as byte, 23 as libc::c_int as byte,
     219 as libc::c_int as byte, 203 as libc::c_int as byte,
     19 as libc::c_int as byte, 203 as libc::c_int as byte,
     183 as libc::c_int as byte, 15 as libc::c_int as byte,
     187 as libc::c_int as byte, 167 as libc::c_int as byte,
     15 as libc::c_int as byte, 171 as libc::c_int as byte,
     151 as libc::c_int as byte, 11 as libc::c_int as byte,
     155 as libc::c_int as byte, 131 as libc::c_int as byte,
     7 as libc::c_int as byte, 139 as libc::c_int as byte,
     115 as libc::c_int as byte, 7 as libc::c_int as byte,
     123 as libc::c_int as byte, 99 as libc::c_int as byte,
     7 as libc::c_int as byte, 107 as libc::c_int as byte,
     83 as libc::c_int as byte, 0 as libc::c_int as byte,
     91 as libc::c_int as byte, 71 as libc::c_int as byte,
     0 as libc::c_int as byte, 75 as libc::c_int as byte,
     55 as libc::c_int as byte, 0 as libc::c_int as byte,
     59 as libc::c_int as byte, 43 as libc::c_int as byte,
     0 as libc::c_int as byte, 43 as libc::c_int as byte,
     31 as libc::c_int as byte, 0 as libc::c_int as byte,
     27 as libc::c_int as byte, 15 as libc::c_int as byte,
     0 as libc::c_int as byte, 11 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     255 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 239 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     223 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 207 as libc::c_int as byte,
     35 as libc::c_int as byte, 35 as libc::c_int as byte,
     191 as libc::c_int as byte, 43 as libc::c_int as byte,
     43 as libc::c_int as byte, 175 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     159 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 143 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     127 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 111 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     95 as libc::c_int as byte, 43 as libc::c_int as byte,
     43 as libc::c_int as byte, 79 as libc::c_int as byte,
     35 as libc::c_int as byte, 35 as libc::c_int as byte,
     63 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 47 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     31 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 15 as libc::c_int as byte,
     43 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 59 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     75 as libc::c_int as byte, 7 as libc::c_int as byte,
     0 as libc::c_int as byte, 95 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     111 as libc::c_int as byte, 15 as libc::c_int as byte,
     0 as libc::c_int as byte, 127 as libc::c_int as byte,
     23 as libc::c_int as byte, 7 as libc::c_int as byte,
     147 as libc::c_int as byte, 31 as libc::c_int as byte,
     7 as libc::c_int as byte, 163 as libc::c_int as byte,
     39 as libc::c_int as byte, 11 as libc::c_int as byte,
     183 as libc::c_int as byte, 51 as libc::c_int as byte,
     15 as libc::c_int as byte, 195 as libc::c_int as byte,
     75 as libc::c_int as byte, 27 as libc::c_int as byte,
     207 as libc::c_int as byte, 99 as libc::c_int as byte,
     43 as libc::c_int as byte, 219 as libc::c_int as byte,
     127 as libc::c_int as byte, 59 as libc::c_int as byte,
     227 as libc::c_int as byte, 151 as libc::c_int as byte,
     79 as libc::c_int as byte, 231 as libc::c_int as byte,
     171 as libc::c_int as byte, 95 as libc::c_int as byte,
     239 as libc::c_int as byte, 191 as libc::c_int as byte,
     119 as libc::c_int as byte, 247 as libc::c_int as byte,
     211 as libc::c_int as byte, 139 as libc::c_int as byte,
     167 as libc::c_int as byte, 123 as libc::c_int as byte,
     59 as libc::c_int as byte, 183 as libc::c_int as byte,
     155 as libc::c_int as byte, 55 as libc::c_int as byte,
     199 as libc::c_int as byte, 195 as libc::c_int as byte,
     55 as libc::c_int as byte, 231 as libc::c_int as byte,
     227 as libc::c_int as byte, 87 as libc::c_int as byte,
     127 as libc::c_int as byte, 191 as libc::c_int as byte,
     255 as libc::c_int as byte, 171 as libc::c_int as byte,
     231 as libc::c_int as byte, 255 as libc::c_int as byte,
     215 as libc::c_int as byte, 255 as libc::c_int as byte,
     255 as libc::c_int as byte, 103 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     139 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 179 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     215 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 255 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     255 as libc::c_int as byte, 243 as libc::c_int as byte,
     147 as libc::c_int as byte, 255 as libc::c_int as byte,
     247 as libc::c_int as byte, 199 as libc::c_int as byte,
     255 as libc::c_int as byte, 255 as libc::c_int as byte,
     255 as libc::c_int as byte, 159 as libc::c_int as byte,
     91 as libc::c_int as byte, 83 as libc::c_int as byte];
// this is used only for particle colors
static mut palette_hl: [byte; 768] =
    [0 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 15 as libc::c_int as byte,
     15 as libc::c_int as byte, 15 as libc::c_int as byte,
     31 as libc::c_int as byte, 31 as libc::c_int as byte,
     31 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     63 as libc::c_int as byte, 63 as libc::c_int as byte,
     63 as libc::c_int as byte, 75 as libc::c_int as byte,
     75 as libc::c_int as byte, 75 as libc::c_int as byte,
     91 as libc::c_int as byte, 91 as libc::c_int as byte,
     91 as libc::c_int as byte, 107 as libc::c_int as byte,
     107 as libc::c_int as byte, 107 as libc::c_int as byte,
     123 as libc::c_int as byte, 123 as libc::c_int as byte,
     123 as libc::c_int as byte, 139 as libc::c_int as byte,
     139 as libc::c_int as byte, 139 as libc::c_int as byte,
     155 as libc::c_int as byte, 155 as libc::c_int as byte,
     155 as libc::c_int as byte, 171 as libc::c_int as byte,
     171 as libc::c_int as byte, 171 as libc::c_int as byte,
     187 as libc::c_int as byte, 187 as libc::c_int as byte,
     187 as libc::c_int as byte, 203 as libc::c_int as byte,
     203 as libc::c_int as byte, 203 as libc::c_int as byte,
     219 as libc::c_int as byte, 219 as libc::c_int as byte,
     219 as libc::c_int as byte, 235 as libc::c_int as byte,
     235 as libc::c_int as byte, 235 as libc::c_int as byte,
     15 as libc::c_int as byte, 11 as libc::c_int as byte,
     7 as libc::c_int as byte, 23 as libc::c_int as byte,
     15 as libc::c_int as byte, 11 as libc::c_int as byte,
     31 as libc::c_int as byte, 23 as libc::c_int as byte,
     11 as libc::c_int as byte, 39 as libc::c_int as byte,
     27 as libc::c_int as byte, 15 as libc::c_int as byte,
     47 as libc::c_int as byte, 35 as libc::c_int as byte,
     19 as libc::c_int as byte, 55 as libc::c_int as byte,
     43 as libc::c_int as byte, 23 as libc::c_int as byte,
     63 as libc::c_int as byte, 47 as libc::c_int as byte,
     23 as libc::c_int as byte, 75 as libc::c_int as byte,
     55 as libc::c_int as byte, 27 as libc::c_int as byte,
     83 as libc::c_int as byte, 59 as libc::c_int as byte,
     27 as libc::c_int as byte, 91 as libc::c_int as byte,
     67 as libc::c_int as byte, 31 as libc::c_int as byte,
     99 as libc::c_int as byte, 75 as libc::c_int as byte,
     31 as libc::c_int as byte, 107 as libc::c_int as byte,
     83 as libc::c_int as byte, 31 as libc::c_int as byte,
     115 as libc::c_int as byte, 87 as libc::c_int as byte,
     31 as libc::c_int as byte, 123 as libc::c_int as byte,
     95 as libc::c_int as byte, 35 as libc::c_int as byte,
     131 as libc::c_int as byte, 103 as libc::c_int as byte,
     35 as libc::c_int as byte, 143 as libc::c_int as byte,
     111 as libc::c_int as byte, 35 as libc::c_int as byte,
     11 as libc::c_int as byte, 11 as libc::c_int as byte,
     15 as libc::c_int as byte, 19 as libc::c_int as byte,
     19 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 27 as libc::c_int as byte,
     39 as libc::c_int as byte, 39 as libc::c_int as byte,
     39 as libc::c_int as byte, 51 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     63 as libc::c_int as byte, 55 as libc::c_int as byte,
     55 as libc::c_int as byte, 75 as libc::c_int as byte,
     63 as libc::c_int as byte, 63 as libc::c_int as byte,
     87 as libc::c_int as byte, 71 as libc::c_int as byte,
     71 as libc::c_int as byte, 103 as libc::c_int as byte,
     79 as libc::c_int as byte, 79 as libc::c_int as byte,
     115 as libc::c_int as byte, 91 as libc::c_int as byte,
     91 as libc::c_int as byte, 127 as libc::c_int as byte,
     99 as libc::c_int as byte, 99 as libc::c_int as byte,
     139 as libc::c_int as byte, 107 as libc::c_int as byte,
     107 as libc::c_int as byte, 151 as libc::c_int as byte,
     115 as libc::c_int as byte, 115 as libc::c_int as byte,
     163 as libc::c_int as byte, 123 as libc::c_int as byte,
     123 as libc::c_int as byte, 175 as libc::c_int as byte,
     131 as libc::c_int as byte, 131 as libc::c_int as byte,
     187 as libc::c_int as byte, 139 as libc::c_int as byte,
     139 as libc::c_int as byte, 203 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 7 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     11 as libc::c_int as byte, 11 as libc::c_int as byte,
     0 as libc::c_int as byte, 19 as libc::c_int as byte,
     19 as libc::c_int as byte, 0 as libc::c_int as byte,
     27 as libc::c_int as byte, 27 as libc::c_int as byte,
     0 as libc::c_int as byte, 35 as libc::c_int as byte,
     35 as libc::c_int as byte, 0 as libc::c_int as byte,
     43 as libc::c_int as byte, 43 as libc::c_int as byte,
     7 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 7 as libc::c_int as byte,
     55 as libc::c_int as byte, 55 as libc::c_int as byte,
     7 as libc::c_int as byte, 63 as libc::c_int as byte,
     63 as libc::c_int as byte, 7 as libc::c_int as byte,
     71 as libc::c_int as byte, 71 as libc::c_int as byte,
     7 as libc::c_int as byte, 75 as libc::c_int as byte,
     75 as libc::c_int as byte, 11 as libc::c_int as byte,
     83 as libc::c_int as byte, 83 as libc::c_int as byte,
     11 as libc::c_int as byte, 91 as libc::c_int as byte,
     91 as libc::c_int as byte, 11 as libc::c_int as byte,
     99 as libc::c_int as byte, 99 as libc::c_int as byte,
     11 as libc::c_int as byte, 107 as libc::c_int as byte,
     107 as libc::c_int as byte, 15 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 15 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     23 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 31 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     39 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 47 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     55 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 63 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     71 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 79 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     87 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 95 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     103 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 111 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     119 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 127 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     0 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 0 as libc::c_int as byte,
     35 as libc::c_int as byte, 35 as libc::c_int as byte,
     0 as libc::c_int as byte, 47 as libc::c_int as byte,
     43 as libc::c_int as byte, 0 as libc::c_int as byte,
     55 as libc::c_int as byte, 47 as libc::c_int as byte,
     0 as libc::c_int as byte, 67 as libc::c_int as byte,
     55 as libc::c_int as byte, 0 as libc::c_int as byte,
     75 as libc::c_int as byte, 59 as libc::c_int as byte,
     7 as libc::c_int as byte, 87 as libc::c_int as byte,
     67 as libc::c_int as byte, 7 as libc::c_int as byte,
     95 as libc::c_int as byte, 71 as libc::c_int as byte,
     7 as libc::c_int as byte, 107 as libc::c_int as byte,
     75 as libc::c_int as byte, 11 as libc::c_int as byte,
     119 as libc::c_int as byte, 83 as libc::c_int as byte,
     15 as libc::c_int as byte, 131 as libc::c_int as byte,
     87 as libc::c_int as byte, 19 as libc::c_int as byte,
     139 as libc::c_int as byte, 91 as libc::c_int as byte,
     19 as libc::c_int as byte, 151 as libc::c_int as byte,
     95 as libc::c_int as byte, 27 as libc::c_int as byte,
     163 as libc::c_int as byte, 99 as libc::c_int as byte,
     31 as libc::c_int as byte, 175 as libc::c_int as byte,
     103 as libc::c_int as byte, 35 as libc::c_int as byte,
     35 as libc::c_int as byte, 19 as libc::c_int as byte,
     7 as libc::c_int as byte, 47 as libc::c_int as byte,
     23 as libc::c_int as byte, 11 as libc::c_int as byte,
     59 as libc::c_int as byte, 31 as libc::c_int as byte,
     15 as libc::c_int as byte, 75 as libc::c_int as byte,
     35 as libc::c_int as byte, 19 as libc::c_int as byte,
     87 as libc::c_int as byte, 43 as libc::c_int as byte,
     23 as libc::c_int as byte, 99 as libc::c_int as byte,
     47 as libc::c_int as byte, 31 as libc::c_int as byte,
     115 as libc::c_int as byte, 55 as libc::c_int as byte,
     35 as libc::c_int as byte, 127 as libc::c_int as byte,
     59 as libc::c_int as byte, 43 as libc::c_int as byte,
     143 as libc::c_int as byte, 67 as libc::c_int as byte,
     51 as libc::c_int as byte, 159 as libc::c_int as byte,
     79 as libc::c_int as byte, 51 as libc::c_int as byte,
     175 as libc::c_int as byte, 99 as libc::c_int as byte,
     47 as libc::c_int as byte, 191 as libc::c_int as byte,
     119 as libc::c_int as byte, 47 as libc::c_int as byte,
     207 as libc::c_int as byte, 143 as libc::c_int as byte,
     43 as libc::c_int as byte, 223 as libc::c_int as byte,
     171 as libc::c_int as byte, 39 as libc::c_int as byte,
     239 as libc::c_int as byte, 203 as libc::c_int as byte,
     31 as libc::c_int as byte, 255 as libc::c_int as byte,
     243 as libc::c_int as byte, 27 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     0 as libc::c_int as byte, 27 as libc::c_int as byte,
     19 as libc::c_int as byte, 0 as libc::c_int as byte,
     43 as libc::c_int as byte, 35 as libc::c_int as byte,
     15 as libc::c_int as byte, 55 as libc::c_int as byte,
     43 as libc::c_int as byte, 19 as libc::c_int as byte,
     71 as libc::c_int as byte, 51 as libc::c_int as byte,
     27 as libc::c_int as byte, 83 as libc::c_int as byte,
     55 as libc::c_int as byte, 35 as libc::c_int as byte,
     99 as libc::c_int as byte, 63 as libc::c_int as byte,
     43 as libc::c_int as byte, 111 as libc::c_int as byte,
     71 as libc::c_int as byte, 51 as libc::c_int as byte,
     127 as libc::c_int as byte, 83 as libc::c_int as byte,
     63 as libc::c_int as byte, 139 as libc::c_int as byte,
     95 as libc::c_int as byte, 71 as libc::c_int as byte,
     155 as libc::c_int as byte, 107 as libc::c_int as byte,
     83 as libc::c_int as byte, 167 as libc::c_int as byte,
     123 as libc::c_int as byte, 95 as libc::c_int as byte,
     183 as libc::c_int as byte, 135 as libc::c_int as byte,
     107 as libc::c_int as byte, 195 as libc::c_int as byte,
     147 as libc::c_int as byte, 123 as libc::c_int as byte,
     211 as libc::c_int as byte, 163 as libc::c_int as byte,
     139 as libc::c_int as byte, 227 as libc::c_int as byte,
     179 as libc::c_int as byte, 151 as libc::c_int as byte,
     171 as libc::c_int as byte, 139 as libc::c_int as byte,
     163 as libc::c_int as byte, 159 as libc::c_int as byte,
     127 as libc::c_int as byte, 151 as libc::c_int as byte,
     147 as libc::c_int as byte, 115 as libc::c_int as byte,
     135 as libc::c_int as byte, 139 as libc::c_int as byte,
     103 as libc::c_int as byte, 123 as libc::c_int as byte,
     127 as libc::c_int as byte, 91 as libc::c_int as byte,
     111 as libc::c_int as byte, 119 as libc::c_int as byte,
     83 as libc::c_int as byte, 99 as libc::c_int as byte,
     107 as libc::c_int as byte, 75 as libc::c_int as byte,
     87 as libc::c_int as byte, 95 as libc::c_int as byte,
     63 as libc::c_int as byte, 75 as libc::c_int as byte,
     87 as libc::c_int as byte, 55 as libc::c_int as byte,
     67 as libc::c_int as byte, 75 as libc::c_int as byte,
     47 as libc::c_int as byte, 55 as libc::c_int as byte,
     67 as libc::c_int as byte, 39 as libc::c_int as byte,
     47 as libc::c_int as byte, 55 as libc::c_int as byte,
     31 as libc::c_int as byte, 35 as libc::c_int as byte,
     43 as libc::c_int as byte, 23 as libc::c_int as byte,
     27 as libc::c_int as byte, 35 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     23 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 15 as libc::c_int as byte,
     7 as libc::c_int as byte, 7 as libc::c_int as byte,
     187 as libc::c_int as byte, 115 as libc::c_int as byte,
     159 as libc::c_int as byte, 175 as libc::c_int as byte,
     107 as libc::c_int as byte, 143 as libc::c_int as byte,
     163 as libc::c_int as byte, 95 as libc::c_int as byte,
     131 as libc::c_int as byte, 151 as libc::c_int as byte,
     87 as libc::c_int as byte, 119 as libc::c_int as byte,
     139 as libc::c_int as byte, 79 as libc::c_int as byte,
     107 as libc::c_int as byte, 127 as libc::c_int as byte,
     75 as libc::c_int as byte, 95 as libc::c_int as byte,
     115 as libc::c_int as byte, 67 as libc::c_int as byte,
     83 as libc::c_int as byte, 107 as libc::c_int as byte,
     59 as libc::c_int as byte, 75 as libc::c_int as byte,
     95 as libc::c_int as byte, 51 as libc::c_int as byte,
     63 as libc::c_int as byte, 83 as libc::c_int as byte,
     43 as libc::c_int as byte, 55 as libc::c_int as byte,
     71 as libc::c_int as byte, 35 as libc::c_int as byte,
     43 as libc::c_int as byte, 59 as libc::c_int as byte,
     31 as libc::c_int as byte, 35 as libc::c_int as byte,
     47 as libc::c_int as byte, 23 as libc::c_int as byte,
     27 as libc::c_int as byte, 35 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     23 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 15 as libc::c_int as byte,
     7 as libc::c_int as byte, 7 as libc::c_int as byte,
     219 as libc::c_int as byte, 195 as libc::c_int as byte,
     187 as libc::c_int as byte, 203 as libc::c_int as byte,
     179 as libc::c_int as byte, 167 as libc::c_int as byte,
     191 as libc::c_int as byte, 163 as libc::c_int as byte,
     155 as libc::c_int as byte, 175 as libc::c_int as byte,
     151 as libc::c_int as byte, 139 as libc::c_int as byte,
     163 as libc::c_int as byte, 135 as libc::c_int as byte,
     123 as libc::c_int as byte, 151 as libc::c_int as byte,
     123 as libc::c_int as byte, 111 as libc::c_int as byte,
     135 as libc::c_int as byte, 111 as libc::c_int as byte,
     95 as libc::c_int as byte, 123 as libc::c_int as byte,
     99 as libc::c_int as byte, 83 as libc::c_int as byte,
     107 as libc::c_int as byte, 87 as libc::c_int as byte,
     71 as libc::c_int as byte, 95 as libc::c_int as byte,
     75 as libc::c_int as byte, 59 as libc::c_int as byte,
     83 as libc::c_int as byte, 63 as libc::c_int as byte,
     51 as libc::c_int as byte, 67 as libc::c_int as byte,
     51 as libc::c_int as byte, 39 as libc::c_int as byte,
     55 as libc::c_int as byte, 43 as libc::c_int as byte,
     31 as libc::c_int as byte, 39 as libc::c_int as byte,
     31 as libc::c_int as byte, 23 as libc::c_int as byte,
     27 as libc::c_int as byte, 19 as libc::c_int as byte,
     15 as libc::c_int as byte, 15 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     111 as libc::c_int as byte, 131 as libc::c_int as byte,
     123 as libc::c_int as byte, 103 as libc::c_int as byte,
     123 as libc::c_int as byte, 111 as libc::c_int as byte,
     95 as libc::c_int as byte, 115 as libc::c_int as byte,
     103 as libc::c_int as byte, 87 as libc::c_int as byte,
     107 as libc::c_int as byte, 95 as libc::c_int as byte,
     79 as libc::c_int as byte, 99 as libc::c_int as byte,
     87 as libc::c_int as byte, 71 as libc::c_int as byte,
     91 as libc::c_int as byte, 79 as libc::c_int as byte,
     63 as libc::c_int as byte, 83 as libc::c_int as byte,
     71 as libc::c_int as byte, 55 as libc::c_int as byte,
     75 as libc::c_int as byte, 63 as libc::c_int as byte,
     47 as libc::c_int as byte, 67 as libc::c_int as byte,
     55 as libc::c_int as byte, 43 as libc::c_int as byte,
     59 as libc::c_int as byte, 47 as libc::c_int as byte,
     35 as libc::c_int as byte, 51 as libc::c_int as byte,
     39 as libc::c_int as byte, 31 as libc::c_int as byte,
     43 as libc::c_int as byte, 31 as libc::c_int as byte,
     23 as libc::c_int as byte, 35 as libc::c_int as byte,
     23 as libc::c_int as byte, 15 as libc::c_int as byte,
     27 as libc::c_int as byte, 19 as libc::c_int as byte,
     11 as libc::c_int as byte, 19 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     11 as libc::c_int as byte, 7 as libc::c_int as byte,
     255 as libc::c_int as byte, 243 as libc::c_int as byte,
     27 as libc::c_int as byte, 239 as libc::c_int as byte,
     223 as libc::c_int as byte, 23 as libc::c_int as byte,
     219 as libc::c_int as byte, 203 as libc::c_int as byte,
     19 as libc::c_int as byte, 203 as libc::c_int as byte,
     183 as libc::c_int as byte, 15 as libc::c_int as byte,
     187 as libc::c_int as byte, 167 as libc::c_int as byte,
     15 as libc::c_int as byte, 171 as libc::c_int as byte,
     151 as libc::c_int as byte, 11 as libc::c_int as byte,
     155 as libc::c_int as byte, 131 as libc::c_int as byte,
     7 as libc::c_int as byte, 139 as libc::c_int as byte,
     115 as libc::c_int as byte, 7 as libc::c_int as byte,
     123 as libc::c_int as byte, 99 as libc::c_int as byte,
     7 as libc::c_int as byte, 107 as libc::c_int as byte,
     83 as libc::c_int as byte, 0 as libc::c_int as byte,
     91 as libc::c_int as byte, 71 as libc::c_int as byte,
     0 as libc::c_int as byte, 75 as libc::c_int as byte,
     55 as libc::c_int as byte, 0 as libc::c_int as byte,
     59 as libc::c_int as byte, 43 as libc::c_int as byte,
     0 as libc::c_int as byte, 43 as libc::c_int as byte,
     31 as libc::c_int as byte, 0 as libc::c_int as byte,
     27 as libc::c_int as byte, 15 as libc::c_int as byte,
     0 as libc::c_int as byte, 11 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     255 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 239 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     223 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 207 as libc::c_int as byte,
     35 as libc::c_int as byte, 35 as libc::c_int as byte,
     191 as libc::c_int as byte, 43 as libc::c_int as byte,
     43 as libc::c_int as byte, 175 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     159 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 143 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     127 as libc::c_int as byte, 47 as libc::c_int as byte,
     47 as libc::c_int as byte, 111 as libc::c_int as byte,
     47 as libc::c_int as byte, 47 as libc::c_int as byte,
     95 as libc::c_int as byte, 43 as libc::c_int as byte,
     43 as libc::c_int as byte, 79 as libc::c_int as byte,
     35 as libc::c_int as byte, 35 as libc::c_int as byte,
     63 as libc::c_int as byte, 27 as libc::c_int as byte,
     27 as libc::c_int as byte, 47 as libc::c_int as byte,
     19 as libc::c_int as byte, 19 as libc::c_int as byte,
     31 as libc::c_int as byte, 11 as libc::c_int as byte,
     11 as libc::c_int as byte, 15 as libc::c_int as byte,
     43 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 59 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     75 as libc::c_int as byte, 7 as libc::c_int as byte,
     0 as libc::c_int as byte, 95 as libc::c_int as byte,
     7 as libc::c_int as byte, 0 as libc::c_int as byte,
     111 as libc::c_int as byte, 15 as libc::c_int as byte,
     0 as libc::c_int as byte, 127 as libc::c_int as byte,
     23 as libc::c_int as byte, 7 as libc::c_int as byte,
     147 as libc::c_int as byte, 31 as libc::c_int as byte,
     7 as libc::c_int as byte, 163 as libc::c_int as byte,
     39 as libc::c_int as byte, 11 as libc::c_int as byte,
     183 as libc::c_int as byte, 51 as libc::c_int as byte,
     15 as libc::c_int as byte, 195 as libc::c_int as byte,
     75 as libc::c_int as byte, 27 as libc::c_int as byte,
     207 as libc::c_int as byte, 99 as libc::c_int as byte,
     43 as libc::c_int as byte, 219 as libc::c_int as byte,
     127 as libc::c_int as byte, 59 as libc::c_int as byte,
     227 as libc::c_int as byte, 151 as libc::c_int as byte,
     79 as libc::c_int as byte, 231 as libc::c_int as byte,
     171 as libc::c_int as byte, 95 as libc::c_int as byte,
     239 as libc::c_int as byte, 191 as libc::c_int as byte,
     119 as libc::c_int as byte, 247 as libc::c_int as byte,
     211 as libc::c_int as byte, 139 as libc::c_int as byte,
     167 as libc::c_int as byte, 123 as libc::c_int as byte,
     59 as libc::c_int as byte, 183 as libc::c_int as byte,
     155 as libc::c_int as byte, 55 as libc::c_int as byte,
     199 as libc::c_int as byte, 195 as libc::c_int as byte,
     55 as libc::c_int as byte, 231 as libc::c_int as byte,
     227 as libc::c_int as byte, 87 as libc::c_int as byte,
     0 as libc::c_int as byte, 255 as libc::c_int as byte,
     0 as libc::c_int as byte, 171 as libc::c_int as byte,
     231 as libc::c_int as byte, 255 as libc::c_int as byte,
     215 as libc::c_int as byte, 255 as libc::c_int as byte,
     255 as libc::c_int as byte, 103 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     139 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 179 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     215 as libc::c_int as byte, 0 as libc::c_int as byte,
     0 as libc::c_int as byte, 255 as libc::c_int as byte,
     0 as libc::c_int as byte, 0 as libc::c_int as byte,
     255 as libc::c_int as byte, 243 as libc::c_int as byte,
     147 as libc::c_int as byte, 255 as libc::c_int as byte,
     247 as libc::c_int as byte, 199 as libc::c_int as byte,
     255 as libc::c_int as byte, 255 as libc::c_int as byte,
     255 as libc::c_int as byte, 159 as libc::c_int as byte,
     91 as libc::c_int as byte, 83 as libc::c_int as byte];
static mut img_emboss: [[libc::c_float; 5]; 5] =
    [[-0.7f32, -0.7f32, -0.7f32, -0.7f32, 0.0f32],
     [-0.7f32, -0.7f32, -0.7f32, 0.0f32, 0.7f32],
     [-0.7f32, -0.7f32, 0.0f32, 0.7f32, 0.7f32],
     [-0.7f32, 0.0f32, 0.7f32, 0.7f32, 0.7f32],
     [0.0f32, 0.7f32, 0.7f32, 0.7f32, 0.7f32]];
/*
=============================================================================

	XASH3D LOAD IMAGE FORMATS

=============================================================================
*/
// stub
static mut load_null: [loadpixformat_t; 1] =
    [{
         let mut init =
             loadformat_s{formatstring: 0 as *const libc::c_char,
                          ext: 0 as *const libc::c_char,
                          loadfunc: None,
                          hint: IL_HINT_NO,};
         init
     }];
static mut load_game: [loadpixformat_t; 11] =
    unsafe {
        [{
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"dds\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadDDS as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"tga\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadTGA as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"bmp\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadBMP as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"png\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadPNG as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"mip\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadMIP as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"mdl\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadMDL as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_HL,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"spr\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadSPR as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_HL,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"lmp\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadLMP as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"fnt\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadFNT as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_HL,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"pal\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Image_LoadPAL as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),
                              hint: IL_HINT_NO,};
             init
         },
         {
             let mut init =
                 loadformat_s{formatstring: 0 as *const libc::c_char,
                              ext: 0 as *const libc::c_char,
                              loadfunc: None,
                              hint: IL_HINT_NO,};
             init
         }]
    };
/*
=============================================================================

	XASH3D SAVE IMAGE FORMATS

=============================================================================
*/
// stub
static mut save_null: [savepixformat_t; 1] =
    [{
         let mut init =
             saveformat_s{formatstring: 0 as *const libc::c_char,
                          ext: 0 as *const libc::c_char,
                          savefunc: None,};
         init
     }];
// Xash3D normal instance
static mut save_game: [savepixformat_t; 4] =
    unsafe {
        [{
             let mut init =
                 saveformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"tga\x00" as *const u8 as
                                      *const libc::c_char,
                              savefunc:
                                  Some(Image_SaveTGA as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *mut rgbdata_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 saveformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"bmp\x00" as *const u8 as
                                      *const libc::c_char,
                              savefunc:
                                  Some(Image_SaveBMP as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *mut rgbdata_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 saveformat_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"png\x00" as *const u8 as
                                      *const libc::c_char,
                              savefunc:
                                  Some(Image_SavePNG as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *mut rgbdata_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 saveformat_s{formatstring: 0 as *const libc::c_char,
                              ext: 0 as *const libc::c_char,
                              savefunc: None,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn Image_Setup() {
    image.cmd_flags =
        IL_USE_LERPING as libc::c_int | IL_ALLOW_OVERWRITE as libc::c_int;
    image.loadformats = load_game.as_ptr();
    image.saveformats = save_game.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Image_Init() {
    // init pools
    host.imagepool =
        _Mem_AllocPool(b"ImageLib Pool\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/common/imagelib/img_utils.c\x00" as
                           *const u8 as *const libc::c_char,
                       152 as libc::c_int);
    // install image formats (can be re-install later by Image_Setup)
    match host.type_0 {
        0 => { Image_Setup(); }
        1 => {
            image.cmd_flags = 0 as libc::c_int;
            image.loadformats = load_game.as_ptr();
            image.saveformats = save_null.as_ptr()
        }
        _ => {
            // all other instances not using imagelib
            image.cmd_flags = 0 as libc::c_int; // check for leaks
            image.loadformats = load_null.as_ptr();
            image.saveformats = save_null.as_ptr()
        }
    }
    image.tempbuffer = 0 as *mut byte;
}
#[no_mangle]
pub unsafe extern "C" fn Image_Shutdown() {
    _Mem_Check(b"../engine/common/imagelib/img_utils.c\x00" as *const u8 as
                   *const libc::c_char, 177 as libc::c_int);
    _Mem_FreePool(&mut host.imagepool,
                  b"../engine/common/imagelib/img_utils.c\x00" as *const u8 as
                      *const libc::c_char, 178 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Image_Copy(mut size: size_t) -> *mut byte {
    let mut out: *mut byte = 0 as *mut byte;
    out =
        _Mem_Alloc(host.imagepool, size, false_0,
                   b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                       as *const libc::c_char, 185 as libc::c_int) as
            *mut byte;
    memcpy(out as *mut libc::c_void, image.tempbuffer as *const libc::c_void,
           size);
    return out;
}
/*
=================
Image_CustomPalette
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_CustomPalette() -> qboolean {
    return image.custom_palette;
}
/*
=================
Image_CheckFlag
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_CheckFlag(mut bit: libc::c_int) -> qboolean {
    if image.force_flags & bit != 0 { return true_0 }
    if image.cmd_flags & bit != 0 { return true_0 }
    return false_0;
}
/*
=================
Image_SetForceFlags
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_SetForceFlags(mut flags: uint) {
    image.force_flags =
        (image.force_flags as libc::c_uint | flags) as libc::c_int;
}
/*
=================
Image_ClearForceFlags
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_ClearForceFlags() {
    image.force_flags = 0 as libc::c_int;
}
/*
=================
Image_AddCmdFlags
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_AddCmdFlags(mut flags: uint) {
    image.cmd_flags =
        (image.cmd_flags as libc::c_uint | flags) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Image_ValidSize(mut name: *const libc::c_char)
 -> qboolean {
    if image.width as libc::c_int > 8192 as libc::c_int ||
           image.height as libc::c_int > 8192 as libc::c_int ||
           image.width as libc::c_int <= 0 as libc::c_int ||
           image.height as libc::c_int <= 0 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image: (%s) dims out of range [%dx%d]\n\x00"
                        as *const u8 as *const libc::c_char, name,
                    image.width as libc::c_int, image.height as libc::c_int);
        return false_0
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Image_LumpValidSize(mut name: *const libc::c_char)
 -> qboolean {
    if image.width as libc::c_int > 1024 as libc::c_int ||
           image.height as libc::c_int > 1024 as libc::c_int ||
           image.width as libc::c_int <= 0 as libc::c_int ||
           image.height as libc::c_int <= 0 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image: (%s) dims out of range [%dx%d]\n\x00"
                        as *const u8 as *const libc::c_char, name,
                    image.width as libc::c_int, image.height as libc::c_int);
        return false_0
    }
    return true_0;
}
/*
=============
Image_ComparePalette
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_ComparePalette(mut pal: *const byte)
 -> libc::c_int {
    if pal.is_null() {
        return PAL_INVALID as libc::c_int
    } else {
        if memcmp(palette_q1.as_mut_ptr() as *const libc::c_void,
                  pal as *const libc::c_void,
                  765 as libc::c_int as libc::c_ulong) == 0 {
            // last color was changed
            return PAL_QUAKE1 as libc::c_int
        } else {
            if memcmp(palette_hl.as_mut_ptr() as *const libc::c_void,
                      pal as *const libc::c_void,
                      765 as libc::c_int as libc::c_ulong) == 0 {
                return PAL_HALFLIFE as libc::c_int
            }
        }
    }
    return PAL_CUSTOM as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Image_SetPalette(mut pal: *const byte,
                                          mut d_table: *mut uint) {
    let mut rgba: [byte; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    // setup palette
    match image.d_rendermode {
        0 => {
            i = 0 as libc::c_int; // does nothing
            while i < 256 as libc::c_int {
                rgba[0 as libc::c_int as usize] =
                    *pal.offset((i * 3 as libc::c_int + 0 as libc::c_int) as
                                    isize); // already created ?
                rgba[1 as libc::c_int as usize] =
                    *pal.offset((i * 3 as libc::c_int + 1 as libc::c_int) as
                                    isize); // 255 is transparent
                rgba[2 as libc::c_int as usize] =
                    *pal.offset((i * 3 as libc::c_int + 2 as libc::c_int) as
                                    isize); // 255 is transparent
                rgba[3 as libc::c_int as usize] = 0xff as libc::c_int as byte;
                *d_table.offset(i as isize) =
                    *(rgba.as_mut_ptr() as *mut uint);
                i += 1
            }
        }
        2 => {
            i = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                rgba[0 as libc::c_int as usize] =
                    *pal.offset(765 as libc::c_int as isize);
                rgba[1 as libc::c_int as usize] =
                    *pal.offset(766 as libc::c_int as isize);
                rgba[2 as libc::c_int as usize] =
                    *pal.offset(767 as libc::c_int as isize);
                rgba[3 as libc::c_int as usize] = i as byte;
                *d_table.offset(i as isize) =
                    *(rgba.as_mut_ptr() as *mut uint);
                i += 1
            }
        }
        1 => {
            i = 0 as libc::c_int;
            while i < 255 as libc::c_int {
                rgba[0 as libc::c_int as usize] =
                    *pal.offset((i * 3 as libc::c_int + 0 as libc::c_int) as
                                    isize);
                rgba[1 as libc::c_int as usize] =
                    *pal.offset((i * 3 as libc::c_int + 1 as libc::c_int) as
                                    isize);
                rgba[2 as libc::c_int as usize] =
                    *pal.offset((i * 3 as libc::c_int + 2 as libc::c_int) as
                                    isize);
                rgba[3 as libc::c_int as usize] = 0xff as libc::c_int as byte;
                *d_table.offset(i as isize) =
                    *(rgba.as_mut_ptr() as *mut uint);
                i += 1
            }
            *d_table.offset(255 as libc::c_int as isize) =
                0 as libc::c_int as uint
        }
        3 => {
            i = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                rgba[0 as libc::c_int as usize] =
                    *pal.offset((i * 4 as libc::c_int + 0 as libc::c_int) as
                                    isize);
                rgba[1 as libc::c_int as usize] =
                    *pal.offset((i * 4 as libc::c_int + 1 as libc::c_int) as
                                    isize);
                rgba[2 as libc::c_int as usize] =
                    *pal.offset((i * 4 as libc::c_int + 2 as libc::c_int) as
                                    isize);
                rgba[3 as libc::c_int as usize] =
                    *pal.offset((i * 4 as libc::c_int + 3 as libc::c_int) as
                                    isize);
                *d_table.offset(i as isize) =
                    *(rgba.as_mut_ptr() as *mut uint);
                i += 1
            }
        }
        _ => { }
    };
}
unsafe extern "C" fn Image_ConvertPalTo24bit(mut pic: *mut rgbdata_t) {
    let mut pal32: *mut byte = 0 as *mut byte;
    let mut pal24: *mut byte = 0 as *mut byte;
    let mut converted: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    if (*pic).type_0 == PF_INDEXED_24 as libc::c_int as libc::c_uint {
        return
    }
    converted =
        _Mem_Alloc(host.imagepool, 768 as libc::c_int as size_t, false_0,
                   b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                       as *const libc::c_char, 344 as libc::c_int) as
            *mut byte;
    pal24 = converted;
    pal32 = (*pic).palette;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        *pal24.offset(0 as libc::c_int as isize) =
            *pal32.offset(0 as libc::c_int as isize);
        *pal24.offset(1 as libc::c_int as isize) =
            *pal32.offset(1 as libc::c_int as isize);
        *pal24.offset(2 as libc::c_int as isize) =
            *pal32.offset(2 as libc::c_int as isize);
        i += 1;
        pal24 = pal24.offset(3 as libc::c_int as isize);
        pal32 = pal32.offset(4 as libc::c_int as isize)
    }
    _Mem_Free((*pic).palette as *mut libc::c_void,
              b"../engine/common/imagelib/img_utils.c\x00" as *const u8 as
                  *const libc::c_char, 354 as libc::c_int);
    (*pic).palette = converted;
    (*pic).type_0 = PF_INDEXED_24 as libc::c_int as uint;
}
#[no_mangle]
pub unsafe extern "C" fn Image_CopyPalette32bit() {
    if !image.palette.is_null() { return }
    image.palette =
        _Mem_Alloc(host.imagepool, 1024 as libc::c_int as size_t, false_0,
                   b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                       as *const libc::c_char, 362 as libc::c_int) as
            *mut byte;
    memcpy(image.palette as *mut libc::c_void,
           image.d_currentpal as *const libc::c_void,
           1024 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn Image_CheckPaletteQ1() {
    let mut pic: *mut rgbdata_t =
        FS_LoadImage(b"gfx/palette.lmp\x00" as *const u8 as
                         *const libc::c_char, 0 as *const byte,
                     0 as libc::c_int as size_t);
    if !pic.is_null() && (*pic).size == 1024 as libc::c_int as libc::c_ulong {
        Image_ConvertPalTo24bit(pic);
        if Image_ComparePalette((*pic).palette) == PAL_CUSTOM as libc::c_int {
            image.d_rendermode = LUMP_NORMAL as libc::c_int;
            Con_DPrintf(b"custom quake palette detected\n\x00" as *const u8 as
                            *const libc::c_char);
            Image_SetPalette((*pic).palette, d_8toQ1table.as_mut_ptr());
            d_8toQ1table[255 as libc::c_int as usize] =
                0 as libc::c_int as uint;
            image.custom_palette = true_0;
            q1palette_init = true_0
        }
    }
    if !pic.is_null() { FS_FreeImage(pic); };
}
#[no_mangle]
pub unsafe extern "C" fn Image_GetPaletteQ1() {
    if q1palette_init as u64 == 0 {
        image.d_rendermode = LUMP_NORMAL as libc::c_int;
        Image_SetPalette(palette_q1.as_mut_ptr(), d_8toQ1table.as_mut_ptr());
        d_8toQ1table[255 as libc::c_int as usize] = 0 as libc::c_int as uint;
        q1palette_init = true_0
    }
    image.d_rendermode = LUMP_QUAKE1 as libc::c_int;
    image.d_currentpal = d_8toQ1table.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Image_GetPaletteHL() {
    if hlpalette_init as u64 == 0 {
        image.d_rendermode = LUMP_NORMAL as libc::c_int;
        Image_SetPalette(palette_hl.as_mut_ptr(), d_8toHLtable.as_mut_ptr());
        hlpalette_init = true_0
    }
    image.d_rendermode = LUMP_HALFLIFE as libc::c_int;
    image.d_currentpal = d_8toHLtable.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Image_GetPaletteBMP(mut pal: *const byte) {
    image.d_rendermode = LUMP_EXTENDED as libc::c_int;
    if !pal.is_null() {
        Image_SetPalette(pal, d_8to24table.as_mut_ptr());
        image.d_currentpal = d_8to24table.as_mut_ptr()
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_GetPaletteLMP(mut pal: *const byte,
                                             mut rendermode: libc::c_int) {
    image.d_rendermode = rendermode;
    if !pal.is_null() {
        Image_SetPalette(pal, d_8to24table.as_mut_ptr());
        image.d_currentpal = d_8to24table.as_mut_ptr()
    } else {
        match rendermode {
            5 => { Image_GetPaletteQ1(); }
            4 => { Image_GetPaletteHL(); }
            _ => {
                // defaulting to half-life palette
                Image_GetPaletteHL();
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_PaletteHueReplace(mut palSrc: *mut byte,
                                                 mut newHue: libc::c_int,
                                                 mut start: libc::c_int,
                                                 mut end: libc::c_int,
                                                 mut pal_size: libc::c_int) {
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut maxcol: libc::c_float = 0.;
    let mut mincol: libc::c_float = 0.;
    let mut hue: libc::c_float = 0.;
    let mut val: libc::c_float = 0.;
    let mut sat: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    hue =
        newHue as libc::c_float *
            (360.0f32 / 255 as libc::c_int as libc::c_float);
    pal_size =
        if pal_size >= 3 as libc::c_int {
            if pal_size < 4 as libc::c_int {
                pal_size
            } else { 4 as libc::c_int }
        } else { 3 as libc::c_int };
    i = start;
    while i <= end {
        r =
            *palSrc.offset((i * pal_size + 0 as libc::c_int) as isize) as
                libc::c_float;
        g =
            *palSrc.offset((i * pal_size + 1 as libc::c_int) as isize) as
                libc::c_float;
        b =
            *palSrc.offset((i * pal_size + 2 as libc::c_int) as isize) as
                libc::c_float;
        maxcol =
            (if (if r > g { r } else { g }) > b {
                 (if r > g { r } else { g })
             } else { b }) / 255.0f32;
        mincol =
            (if (if r < g { r } else { g }) < b {
                 (if r < g { r } else { g })
             } else { b }) / 255.0f32;
        if !(maxcol == 0 as libc::c_int as libc::c_float) {
            val = maxcol;
            sat = (maxcol - mincol) / maxcol;
            mincol = val * (1.0f32 - sat);
            if hue <= 120.0f32 {
                b = mincol;
                if hue < 60 as libc::c_int as libc::c_float {
                    r = val;
                    g = mincol + hue * (val - mincol) / (120.0f32 - hue)
                } else {
                    g = val;
                    r = mincol + (120.0f32 - hue) * (val - mincol) / hue
                }
            } else if hue <= 240.0f32 {
                r = mincol;
                if hue < 180.0f32 {
                    g = val;
                    b =
                        mincol +
                            (hue - 120.0f32) * (val - mincol) /
                                (240.0f32 - hue)
                } else {
                    b = val;
                    g =
                        mincol +
                            (240.0f32 - hue) * (val - mincol) /
                                (hue - 120.0f32)
                }
            } else {
                g = mincol;
                if hue < 300.0f32 {
                    b = val;
                    r =
                        mincol +
                            (hue - 240.0f32) * (val - mincol) /
                                (360.0f32 - hue)
                } else {
                    r = val;
                    b =
                        mincol +
                            (360.0f32 - hue) * (val - mincol) /
                                (hue - 240.0f32)
                }
            }
            *palSrc.offset((i * pal_size + 0 as libc::c_int) as isize) =
                (r * 255 as libc::c_int as libc::c_float) as byte;
            *palSrc.offset((i * pal_size + 1 as libc::c_int) as isize) =
                (g * 255 as libc::c_int as libc::c_float) as byte;
            *palSrc.offset((i * pal_size + 2 as libc::c_int) as isize) =
                (b * 255 as libc::c_int as libc::c_float) as byte
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_PaletteTranslate(mut palSrc: *mut byte,
                                                mut top: libc::c_int,
                                                mut bottom: libc::c_int,
                                                mut pal_size: libc::c_int) {
    let mut dst: [byte; 256] = [0; 256];
    let mut src: [byte; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    pal_size =
        if pal_size >= 3 as libc::c_int {
            if pal_size < 4 as libc::c_int {
                pal_size
            } else { 4 as libc::c_int }
        } else { 3 as libc::c_int };
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int { src[i as usize] = i as byte; i += 1 }
    memcpy(dst.as_mut_ptr() as *mut libc::c_void,
           src.as_mut_ptr() as *const libc::c_void,
           256 as libc::c_int as libc::c_ulong);
    if top < 128 as libc::c_int {
        // the artists made some backwards ranges. sigh.
        memcpy(dst.as_mut_ptr().offset(16 as libc::c_int as isize) as
                   *mut libc::c_void,
               src.as_mut_ptr().offset(top as isize) as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    } else {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            dst[(16 as libc::c_int + i) as usize] =
                src[(top + 15 as libc::c_int - i) as usize];
            i += 1
        }
    }
    if bottom < 128 as libc::c_int {
        memcpy(dst.as_mut_ptr().offset(96 as libc::c_int as isize) as
                   *mut libc::c_void,
               src.as_mut_ptr().offset(bottom as isize) as
                   *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    } else {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            dst[(96 as libc::c_int + i) as usize] =
                src[(bottom + 15 as libc::c_int - i) as usize];
            i += 1
        }
    }
    // last color isn't changed
    i = 0 as libc::c_int; // may be NULL
    while i < 255 as libc::c_int {
        *palSrc.offset((i * pal_size + 0 as libc::c_int) as isize) =
            palette_q1[(dst[i as usize] as libc::c_int * 3 as libc::c_int +
                            0 as libc::c_int) as usize];
        *palSrc.offset((i * pal_size + 1 as libc::c_int) as isize) =
            palette_q1[(dst[i as usize] as libc::c_int * 3 as libc::c_int +
                            1 as libc::c_int) as usize];
        *palSrc.offset((i * pal_size + 2 as libc::c_int) as isize) =
            palette_q1[(dst[i as usize] as libc::c_int * 3 as libc::c_int +
                            2 as libc::c_int) as usize];
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_CopyParms(mut src: *mut rgbdata_t) {
    Image_Reset();
    image.width = (*src).width;
    image.height = (*src).height;
    image.type_0 = (*src).type_0;
    image.flags = (*src).flags;
    image.size = (*src).size;
    image.palette = (*src).palette;
    memcpy(image.fogParams.as_mut_ptr() as *mut libc::c_void,
           (*src).fogParams.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<rgba_t>() as libc::c_ulong);
}
/*
============
Image_Copy8bitRGBA

NOTE: must call Image_GetPaletteXXX before used
============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_Copy8bitRGBA(mut in_0: *const byte,
                                            mut out: *mut byte,
                                            mut pixels: libc::c_int)
 -> qboolean {
    let mut iout: *mut libc::c_int = out as *mut libc::c_int;
    let mut fin: *mut byte = in_0 as *mut byte;
    let mut col: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    if in_0.is_null() || image.d_currentpal.is_null() { return false_0 }
    // this is a base image with luma - clear luma pixels
    if image.flags & IMAGE_HAS_LUMA as libc::c_int as libc::c_uint != 0 {
        i = 0 as libc::c_int;
        while i < image.width as libc::c_int * image.height as libc::c_int {
            *fin.offset(i as isize) =
                if (*fin.offset(i as isize) as libc::c_int) <
                       224 as libc::c_int {
                    *fin.offset(i as isize) as libc::c_int
                } else { 0 as libc::c_int } as byte;
            i += 1
        }
    }
    // check for color
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        col =
            &mut *image.d_currentpal.offset(i as isize) as *mut uint as
                *mut byte;
        if *col.offset(0 as libc::c_int as isize) as libc::c_int !=
               *col.offset(1 as libc::c_int as isize) as libc::c_int ||
               *col.offset(1 as libc::c_int as isize) as libc::c_int !=
                   *col.offset(2 as libc::c_int as isize) as libc::c_int {
            image.flags |= IMAGE_HAS_COLOR as libc::c_int as libc::c_uint;
            break ;
        } else { i += 1 }
    }
    while pixels >= 8 as libc::c_int {
        *iout.offset(0 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(0 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(1 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(1 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(2 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(2 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(3 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(3 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(4 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(4 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(5 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(5 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(6 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(6 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(7 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(7 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        in_0 = in_0.offset(8 as libc::c_int as isize);
        iout = iout.offset(8 as libc::c_int as isize);
        pixels -= 8 as libc::c_int
    }
    if pixels & 4 as libc::c_int != 0 {
        *iout.offset(0 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(0 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(1 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(1 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(2 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(2 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(3 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(3 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        in_0 = in_0.offset(4 as libc::c_int as isize);
        iout = iout.offset(4 as libc::c_int as isize)
    }
    if pixels & 2 as libc::c_int != 0 {
        *iout.offset(0 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(0 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        *iout.offset(1 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(1 as libc::c_int as isize)
                                           as isize) as libc::c_int;
        in_0 = in_0.offset(2 as libc::c_int as isize);
        iout = iout.offset(2 as libc::c_int as isize)
    }
    if pixels & 1 as libc::c_int != 0 {
        // last byte
        *iout.offset(0 as libc::c_int as isize) =
            *image.d_currentpal.offset(*in_0.offset(0 as libc::c_int as isize)
                                           as isize) as libc::c_int
    } // update image type;
    image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    return true_0;
}
unsafe extern "C" fn Image_Resample32LerpLine(mut in_0: *const byte,
                                              mut out: *mut byte,
                                              mut inwidth: libc::c_int,
                                              mut outwidth: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut xi: libc::c_int = 0;
    let mut oldx: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut fstep: libc::c_int = 0;
    let mut endx: libc::c_int = 0;
    let mut lerp: libc::c_int = 0;
    fstep =
        (inwidth as libc::c_float * 65536.0f32 / outwidth as libc::c_float) as
            libc::c_int;
    endx = inwidth - 1 as libc::c_int;
    j = 0 as libc::c_int;
    f = 0 as libc::c_int;
    while j < outwidth {
        xi = f >> 16 as libc::c_int;
        if xi != oldx {
            in_0 = in_0.offset(((xi - oldx) * 4 as libc::c_int) as isize);
            oldx = xi
        }
        if xi < endx {
            lerp = f & 0xffff as libc::c_int;
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 =
                (((*in_0.offset(4 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                    as byte;
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 =
                (((*in_0.offset(5 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                    as byte;
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 =
                (((*in_0.offset(6 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(2 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(2 as libc::c_int as isize) as libc::c_int)
                    as byte;
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 =
                (((*in_0.offset(7 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(3 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(3 as libc::c_int as isize) as libc::c_int)
                    as byte
        } else {
            // last pixel of the line has no pixel to lerp to
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = *in_0.offset(0 as libc::c_int as isize);
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = *in_0.offset(1 as libc::c_int as isize);
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = *in_0.offset(2 as libc::c_int as isize);
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = *in_0.offset(3 as libc::c_int as isize)
        }
        j += 1;
        f += fstep
    };
}
unsafe extern "C" fn Image_Resample24LerpLine(mut in_0: *const byte,
                                              mut out: *mut byte,
                                              mut inwidth: libc::c_int,
                                              mut outwidth: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut xi: libc::c_int = 0;
    let mut oldx: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut fstep: libc::c_int = 0;
    let mut endx: libc::c_int = 0;
    let mut lerp: libc::c_int = 0;
    fstep =
        (inwidth as libc::c_float * 65536.0f32 / outwidth as libc::c_float) as
            libc::c_int;
    endx = inwidth - 1 as libc::c_int;
    j = 0 as libc::c_int;
    f = 0 as libc::c_int;
    while j < outwidth {
        xi = f >> 16 as libc::c_int;
        if xi != oldx {
            in_0 = in_0.offset(((xi - oldx) * 3 as libc::c_int) as isize);
            oldx = xi
        }
        if xi < endx {
            lerp = f & 0xffff as libc::c_int;
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 =
                (((*in_0.offset(3 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                    as byte;
            let fresh9 = out;
            out = out.offset(1);
            *fresh9 =
                (((*in_0.offset(4 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                    as byte;
            let fresh10 = out;
            out = out.offset(1);
            *fresh10 =
                (((*in_0.offset(5 as libc::c_int as isize) as libc::c_int -
                       *in_0.offset(2 as libc::c_int as isize) as libc::c_int)
                      * lerp >> 16 as libc::c_int) +
                     *in_0.offset(2 as libc::c_int as isize) as libc::c_int)
                    as byte
        } else {
            // last pixel of the line has no pixel to lerp to
            let fresh11 = out; // relies on int being 4 bytes
            out = out.offset(1);
            *fresh11 = *in_0.offset(0 as libc::c_int as isize);
            let fresh12 = out;
            out = out.offset(1);
            *fresh12 = *in_0.offset(1 as libc::c_int as isize);
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = *in_0.offset(2 as libc::c_int as isize)
        }
        j += 1;
        f += fstep
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_Resample32Lerp(mut indata: *const libc::c_void,
                                              mut inwidth: libc::c_int,
                                              mut inheight: libc::c_int,
                                              mut outdata: *mut libc::c_void,
                                              mut outwidth: libc::c_int,
                                              mut outheight: libc::c_int) {
    let mut inrow: *const byte = 0 as *const byte;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut yi: libc::c_int = 0;
    let mut oldy: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut fstep: libc::c_int = 0;
    let mut lerp: libc::c_int = 0;
    let mut endy: libc::c_int = inheight - 1 as libc::c_int;
    let mut inwidth4: libc::c_int = inwidth * 4 as libc::c_int;
    let mut outwidth4: libc::c_int = outwidth * 4 as libc::c_int;
    let mut out: *mut byte = outdata as *mut byte;
    let mut resamplerow1: *mut byte = 0 as *mut byte;
    let mut resamplerow2: *mut byte = 0 as *mut byte;
    fstep =
        (inheight as libc::c_float * 65536.0f32 / outheight as libc::c_float)
            as libc::c_int;
    resamplerow1 =
        _Mem_Alloc(host.imagepool,
                   (outwidth * 4 as libc::c_int * 2 as libc::c_int) as size_t,
                   false_0,
                   b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                       as *const libc::c_char, 735 as libc::c_int) as
            *mut byte;
    resamplerow2 =
        resamplerow1.offset((outwidth * 4 as libc::c_int) as isize);
    inrow = indata as *const byte;
    Image_Resample32LerpLine(inrow, resamplerow1, inwidth, outwidth);
    Image_Resample32LerpLine(inrow.offset(inwidth4 as isize), resamplerow2,
                             inwidth, outwidth);
    i = 0 as libc::c_int;
    f = 0 as libc::c_int;
    while i < outheight {
        yi = f >> 16 as libc::c_int;
        if yi < endy {
            lerp = f & 0xffff as libc::c_int;
            if yi != oldy {
                inrow =
                    (indata as *mut byte).offset((inwidth4 * yi) as isize);
                if yi == oldy + 1 as libc::c_int {
                    memcpy(resamplerow1 as *mut libc::c_void,
                           resamplerow2 as *const libc::c_void,
                           outwidth4 as libc::c_ulong);
                } else {
                    Image_Resample32LerpLine(inrow, resamplerow1, inwidth,
                                             outwidth);
                }
                Image_Resample32LerpLine(inrow.offset(inwidth4 as isize),
                                         resamplerow2, inwidth, outwidth);
                oldy = yi
            }
            j = outwidth - 4 as libc::c_int;
            while j >= 0 as libc::c_int {
                r =
                    *resamplerow1.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(0 as libc::c_int as isize) =
                    (((*resamplerow2.offset(0 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(1 as libc::c_int as isize) =
                    (((*resamplerow2.offset(1 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(2 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(2 as libc::c_int as isize) =
                    (((*resamplerow2.offset(2 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(3 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(3 as libc::c_int as isize) =
                    (((*resamplerow2.offset(3 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(4 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(4 as libc::c_int as isize) =
                    (((*resamplerow2.offset(4 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(5 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(5 as libc::c_int as isize) =
                    (((*resamplerow2.offset(5 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(6 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(6 as libc::c_int as isize) =
                    (((*resamplerow2.offset(6 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(7 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(7 as libc::c_int as isize) =
                    (((*resamplerow2.offset(7 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(8 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(8 as libc::c_int as isize) =
                    (((*resamplerow2.offset(8 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(9 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(9 as libc::c_int as isize) =
                    (((*resamplerow2.offset(9 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(10 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(10 as libc::c_int as isize) =
                    (((*resamplerow2.offset(10 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(11 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(11 as libc::c_int as isize) =
                    (((*resamplerow2.offset(11 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(12 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(12 as libc::c_int as isize) =
                    (((*resamplerow2.offset(12 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(13 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(13 as libc::c_int as isize) =
                    (((*resamplerow2.offset(13 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(14 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(14 as libc::c_int as isize) =
                    (((*resamplerow2.offset(14 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(15 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(15 as libc::c_int as isize) =
                    (((*resamplerow2.offset(15 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                out = out.offset(16 as libc::c_int as isize);
                resamplerow1 =
                    resamplerow1.offset(16 as libc::c_int as isize);
                resamplerow2 =
                    resamplerow2.offset(16 as libc::c_int as isize);
                j -= 4 as libc::c_int
            }
            if j & 2 as libc::c_int != 0 {
                r =
                    *resamplerow1.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(0 as libc::c_int as isize) =
                    (((*resamplerow2.offset(0 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(1 as libc::c_int as isize) =
                    (((*resamplerow2.offset(1 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(2 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(2 as libc::c_int as isize) =
                    (((*resamplerow2.offset(2 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(3 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(3 as libc::c_int as isize) =
                    (((*resamplerow2.offset(3 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(4 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(4 as libc::c_int as isize) =
                    (((*resamplerow2.offset(4 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(5 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(5 as libc::c_int as isize) =
                    (((*resamplerow2.offset(5 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(6 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(6 as libc::c_int as isize) =
                    (((*resamplerow2.offset(6 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(7 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(7 as libc::c_int as isize) =
                    (((*resamplerow2.offset(7 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                out = out.offset(8 as libc::c_int as isize);
                resamplerow1 = resamplerow1.offset(8 as libc::c_int as isize);
                resamplerow2 = resamplerow2.offset(8 as libc::c_int as isize)
            }
            if j & 1 as libc::c_int != 0 {
                r =
                    *resamplerow1.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(0 as libc::c_int as isize) =
                    (((*resamplerow2.offset(0 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(1 as libc::c_int as isize) =
                    (((*resamplerow2.offset(1 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(2 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(2 as libc::c_int as isize) =
                    (((*resamplerow2.offset(2 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(3 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(3 as libc::c_int as isize) =
                    (((*resamplerow2.offset(3 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                out = out.offset(4 as libc::c_int as isize);
                resamplerow1 = resamplerow1.offset(4 as libc::c_int as isize);
                resamplerow2 = resamplerow2.offset(4 as libc::c_int as isize)
            }
            resamplerow1 = resamplerow1.offset(-(outwidth4 as isize));
            resamplerow2 = resamplerow2.offset(-(outwidth4 as isize))
        } else {
            if yi != oldy {
                inrow =
                    (indata as *mut byte).offset((inwidth4 * yi) as isize);
                if yi == oldy + 1 as libc::c_int {
                    memcpy(resamplerow1 as *mut libc::c_void,
                           resamplerow2 as *const libc::c_void,
                           outwidth4 as libc::c_ulong);
                } else {
                    Image_Resample32LerpLine(inrow, resamplerow1, inwidth,
                                             outwidth);
                }
                oldy = yi
            }
            memcpy(out as *mut libc::c_void,
                   resamplerow1 as *const libc::c_void,
                   outwidth4 as libc::c_ulong);
        }
        i += 1;
        f += fstep
    }
    _Mem_Free(resamplerow1 as *mut libc::c_void,
              b"../engine/common/imagelib/img_utils.c\x00" as *const u8 as
                  *const libc::c_char, 828 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Image_Resample32Nolerp(mut indata:
                                                    *const libc::c_void,
                                                mut inwidth: libc::c_int,
                                                mut inheight: libc::c_int,
                                                mut outdata:
                                                    *mut libc::c_void,
                                                mut outwidth: libc::c_int,
                                                mut outheight: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut frac: uint = 0;
    let mut fracstep: uint = 0;
    let mut inrow: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out: *mut libc::c_int = outdata as *mut libc::c_int;
    fracstep = (inwidth * 0x10000 as libc::c_int / outwidth) as uint;
    i = 0 as libc::c_int;
    while i < outheight {
        inrow =
            (indata as
                 *mut libc::c_int).offset((inwidth *
                                               (i * inheight / outheight)) as
                                              isize);
        frac = fracstep >> 1 as libc::c_int;
        j = outwidth - 4 as libc::c_int;
        while j >= 0 as libc::c_int {
            *out.offset(0 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            *out.offset(1 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            *out.offset(2 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            *out.offset(3 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            out = out.offset(4 as libc::c_int as isize);
            j -= 4 as libc::c_int
        }
        if j & 2 as libc::c_int != 0 {
            *out.offset(0 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            *out.offset(1 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            out = out.offset(2 as libc::c_int as isize)
        }
        if j & 1 as libc::c_int != 0 {
            *out.offset(0 as libc::c_int as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            out = out.offset(1 as libc::c_int as isize)
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_Resample24Lerp(mut indata: *const libc::c_void,
                                              mut inwidth: libc::c_int,
                                              mut inheight: libc::c_int,
                                              mut outdata: *mut libc::c_void,
                                              mut outwidth: libc::c_int,
                                              mut outheight: libc::c_int) {
    let mut inrow: *const byte = 0 as *const byte;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut yi: libc::c_int = 0;
    let mut oldy: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut fstep: libc::c_int = 0;
    let mut lerp: libc::c_int = 0;
    let mut endy: libc::c_int = inheight - 1 as libc::c_int;
    let mut inwidth3: libc::c_int = inwidth * 3 as libc::c_int;
    let mut outwidth3: libc::c_int = outwidth * 3 as libc::c_int;
    let mut out: *mut byte = outdata as *mut byte;
    let mut resamplerow1: *mut byte = 0 as *mut byte;
    let mut resamplerow2: *mut byte = 0 as *mut byte;
    fstep =
        (inheight as libc::c_float * 65536.0f32 / outheight as libc::c_float)
            as libc::c_int;
    resamplerow1 =
        _Mem_Alloc(host.imagepool,
                   (outwidth * 3 as libc::c_int * 2 as libc::c_int) as size_t,
                   false_0,
                   b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                       as *const libc::c_char, 882 as libc::c_int) as
            *mut byte;
    resamplerow2 =
        resamplerow1.offset((outwidth * 3 as libc::c_int) as isize);
    inrow = indata as *const byte;
    oldy = 0 as libc::c_int;
    Image_Resample24LerpLine(inrow, resamplerow1, inwidth, outwidth);
    Image_Resample24LerpLine(inrow.offset(inwidth3 as isize), resamplerow2,
                             inwidth, outwidth);
    i = 0 as libc::c_int;
    f = 0 as libc::c_int;
    while i < outheight {
        yi = f >> 16 as libc::c_int;
        if yi < endy {
            lerp = f & 0xffff as libc::c_int;
            if yi != oldy {
                inrow =
                    (indata as *mut byte).offset((inwidth3 * yi) as isize);
                if yi == oldy + 1 as libc::c_int {
                    memcpy(resamplerow1 as *mut libc::c_void,
                           resamplerow2 as *const libc::c_void,
                           outwidth3 as libc::c_ulong);
                } else {
                    Image_Resample24LerpLine(inrow, resamplerow1, inwidth,
                                             outwidth);
                }
                Image_Resample24LerpLine(inrow.offset(inwidth3 as isize),
                                         resamplerow2, inwidth, outwidth);
                oldy = yi
            }
            j = outwidth - 4 as libc::c_int;
            while j >= 0 as libc::c_int {
                r =
                    *resamplerow1.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(0 as libc::c_int as isize) =
                    (((*resamplerow2.offset(0 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(1 as libc::c_int as isize) =
                    (((*resamplerow2.offset(1 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(2 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(2 as libc::c_int as isize) =
                    (((*resamplerow2.offset(2 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(3 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(3 as libc::c_int as isize) =
                    (((*resamplerow2.offset(3 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(4 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(4 as libc::c_int as isize) =
                    (((*resamplerow2.offset(4 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(5 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(5 as libc::c_int as isize) =
                    (((*resamplerow2.offset(5 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(6 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(6 as libc::c_int as isize) =
                    (((*resamplerow2.offset(6 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(7 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(7 as libc::c_int as isize) =
                    (((*resamplerow2.offset(7 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(8 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(8 as libc::c_int as isize) =
                    (((*resamplerow2.offset(8 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(9 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(9 as libc::c_int as isize) =
                    (((*resamplerow2.offset(9 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(10 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(10 as libc::c_int as isize) =
                    (((*resamplerow2.offset(10 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(11 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(11 as libc::c_int as isize) =
                    (((*resamplerow2.offset(11 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                out = out.offset(12 as libc::c_int as isize);
                resamplerow1 =
                    resamplerow1.offset(12 as libc::c_int as isize);
                resamplerow2 =
                    resamplerow2.offset(12 as libc::c_int as isize);
                j -= 4 as libc::c_int
            }
            if j & 2 as libc::c_int != 0 {
                r =
                    *resamplerow1.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(0 as libc::c_int as isize) =
                    (((*resamplerow2.offset(0 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(1 as libc::c_int as isize) =
                    (((*resamplerow2.offset(1 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(2 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(2 as libc::c_int as isize) =
                    (((*resamplerow2.offset(2 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(3 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(3 as libc::c_int as isize) =
                    (((*resamplerow2.offset(3 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(4 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(4 as libc::c_int as isize) =
                    (((*resamplerow2.offset(4 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(5 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(5 as libc::c_int as isize) =
                    (((*resamplerow2.offset(5 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                out = out.offset(6 as libc::c_int as isize);
                resamplerow1 = resamplerow1.offset(6 as libc::c_int as isize);
                resamplerow2 = resamplerow2.offset(6 as libc::c_int as isize)
            }
            if j & 1 as libc::c_int != 0 {
                r =
                    *resamplerow1.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(0 as libc::c_int as isize) =
                    (((*resamplerow2.offset(0 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(1 as libc::c_int as isize) =
                    (((*resamplerow2.offset(1 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                r =
                    *resamplerow1.offset(2 as libc::c_int as isize) as
                        libc::c_int;
                *out.offset(2 as libc::c_int as isize) =
                    (((*resamplerow2.offset(2 as libc::c_int as isize) as
                           libc::c_int - r) * lerp >> 16 as libc::c_int) + r)
                        as byte;
                out = out.offset(3 as libc::c_int as isize);
                resamplerow1 = resamplerow1.offset(3 as libc::c_int as isize);
                resamplerow2 = resamplerow2.offset(3 as libc::c_int as isize)
            }
            resamplerow1 = resamplerow1.offset(-(outwidth3 as isize));
            resamplerow2 = resamplerow2.offset(-(outwidth3 as isize))
        } else {
            if yi != oldy {
                inrow =
                    (indata as *mut byte).offset((inwidth3 * yi) as isize);
                if yi == oldy + 1 as libc::c_int {
                    memcpy(resamplerow1 as *mut libc::c_void,
                           resamplerow2 as *const libc::c_void,
                           outwidth3 as libc::c_ulong);
                } else {
                    Image_Resample24LerpLine(inrow, resamplerow1, inwidth,
                                             outwidth);
                }
                oldy = yi
            }
            memcpy(out as *mut libc::c_void,
                   resamplerow1 as *const libc::c_void,
                   outwidth3 as libc::c_ulong);
        }
        i += 1;
        f += fstep
    }
    _Mem_Free(resamplerow1 as *mut libc::c_void,
              b"../engine/common/imagelib/img_utils.c\x00" as *const u8 as
                  *const libc::c_char, 968 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Image_Resample24Nolerp(mut indata:
                                                    *const libc::c_void,
                                                mut inwidth: libc::c_int,
                                                mut inheight: libc::c_int,
                                                mut outdata:
                                                    *mut libc::c_void,
                                                mut outwidth: libc::c_int,
                                                mut outheight: libc::c_int) {
    let mut frac: uint = 0;
    let mut fracstep: uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut inwidth3: libc::c_int = inwidth * 3 as libc::c_int;
    let mut inrow: *mut byte = 0 as *mut byte;
    let mut out: *mut byte = outdata as *mut byte;
    fracstep = (inwidth * 0x10000 as libc::c_int / outwidth) as uint;
    i = 0 as libc::c_int;
    while i < outheight {
        inrow =
            (indata as
                 *mut byte).offset((inwidth3 * (i * inheight / outheight)) as
                                       isize);
        frac = fracstep >> 1 as libc::c_int;
        j = outwidth - 4 as libc::c_int;
        while j >= 0 as libc::c_int {
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh18 = out;
            out = out.offset(1);
            *fresh18 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh20 = out;
            out = out.offset(1);
            *fresh20 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh21 = out;
            out = out.offset(1);
            *fresh21 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh23 = out;
            out = out.offset(1);
            *fresh23 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh25 = out;
            out = out.offset(1);
            *fresh25 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            j -= 4 as libc::c_int
        }
        if j & 2 as libc::c_int != 0 {
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh26 = out;
            out = out.offset(1);
            *fresh26 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh28 = out;
            out = out.offset(1);
            *fresh28 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh29 = out;
            out = out.offset(1);
            *fresh29 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh30 = out;
            out = out.offset(1);
            *fresh30 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh31 = out;
            out = out.offset(1);
            *fresh31 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            out = out.offset(2 as libc::c_int as isize)
        }
        if j & 1 as libc::c_int != 0 {
            f =
                (frac >>
                     16 as
                         libc::c_int).wrapping_mul(3 as libc::c_int as
                                                       libc::c_uint) as
                    libc::c_int;
            let fresh32 = out;
            out = out.offset(1);
            *fresh32 = *inrow.offset((f + 0 as libc::c_int) as isize);
            let fresh33 = out;
            out = out.offset(1);
            *fresh33 = *inrow.offset((f + 1 as libc::c_int) as isize);
            let fresh34 = out;
            out = out.offset(1);
            *fresh34 = *inrow.offset((f + 2 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            out = out.offset(1 as libc::c_int as isize)
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Image_Resample8Nolerp(mut indata:
                                                   *const libc::c_void,
                                               mut inwidth: libc::c_int,
                                               mut inheight: libc::c_int,
                                               mut outdata: *mut libc::c_void,
                                               mut outwidth: libc::c_int,
                                               mut outheight: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut in_0: *mut byte = 0 as *mut byte;
    let mut inrow: *mut byte = 0 as *mut byte;
    let mut frac: uint = 0;
    let mut fracstep: uint = 0;
    let mut out: *mut byte = outdata as *mut byte;
    in_0 = indata as *mut byte;
    fracstep = (inwidth * 0x10000 as libc::c_int / outwidth) as uint;
    i = 0 as libc::c_int;
    while i < outheight {
        inrow = in_0.offset((inwidth * (i * inheight / outheight)) as isize);
        frac = fracstep >> 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < outwidth {
            *out.offset(j as isize) =
                *inrow.offset((frac >> 16 as libc::c_int) as isize);
            frac =
                (frac as libc::c_uint).wrapping_add(fracstep) as uint as uint;
            j += 1
        }
        i += 1;
        out = out.offset(outwidth as isize)
    };
}
/*
================
Image_Resample
================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_ResampleInternal(mut indata:
                                                    *const libc::c_void,
                                                mut inwidth: libc::c_int,
                                                mut inheight: libc::c_int,
                                                mut outwidth: libc::c_int,
                                                mut outheight: libc::c_int,
                                                mut type_0: libc::c_int,
                                                mut resampled: *mut qboolean)
 -> *mut byte {
    let mut quality: qboolean =
        Image_CheckFlag(IL_USE_LERPING as libc::c_int);
    // nothing to resample ?
    if inwidth == outwidth && inheight == outheight {
        *resampled = false_0;
        return indata as *mut byte
    }
    // alloc new buffer
    match type_0 {
        1 | 2 => {
            image.tempbuffer =
                _Mem_Realloc(host.imagepool,
                             image.tempbuffer as *mut libc::c_void,
                             (outwidth * outheight) as size_t, true_0,
                             b"../engine/common/imagelib/img_utils.c\x00" as
                                 *const u8 as *const libc::c_char,
                             1081 as libc::c_int) as *mut byte;
            Image_Resample8Nolerp(indata, inwidth, inheight,
                                  image.tempbuffer as *mut libc::c_void,
                                  outwidth, outheight);
        }
        5 | 6 => {
            image.tempbuffer =
                _Mem_Realloc(host.imagepool,
                             image.tempbuffer as *mut libc::c_void,
                             (outwidth * outheight * 3 as libc::c_int) as
                                 size_t, true_0,
                             b"../engine/common/imagelib/img_utils.c\x00" as
                                 *const u8 as *const libc::c_char,
                             1086 as libc::c_int) as *mut byte;
            if quality as u64 != 0 {
                Image_Resample24Lerp(indata, inwidth, inheight,
                                     image.tempbuffer as *mut libc::c_void,
                                     outwidth, outheight);
            } else {
                Image_Resample24Nolerp(indata, inwidth, inheight,
                                       image.tempbuffer as *mut libc::c_void,
                                       outwidth, outheight);
            }
        }
        3 | 4 => {
            image.tempbuffer =
                _Mem_Realloc(host.imagepool,
                             image.tempbuffer as *mut libc::c_void,
                             (outwidth * outheight * 4 as libc::c_int) as
                                 size_t, true_0,
                             b"../engine/common/imagelib/img_utils.c\x00" as
                                 *const u8 as *const libc::c_char,
                             1092 as libc::c_int) as *mut byte;
            if quality as u64 != 0 {
                Image_Resample32Lerp(indata, inwidth, inheight,
                                     image.tempbuffer as *mut libc::c_void,
                                     outwidth, outheight);
            } else {
                Image_Resample32Nolerp(indata, inwidth, inheight,
                                       image.tempbuffer as *mut libc::c_void,
                                       outwidth, outheight);
            }
        }
        _ => { *resampled = false_0; return indata as *mut byte }
    }
    *resampled = true_0;
    return image.tempbuffer;
}
/*
================
Image_Flip
================
*/
#[no_mangle]
pub unsafe extern "C" fn Image_FlipInternal(mut in_0: *const byte,
                                            mut srcwidth: *mut word,
                                            mut srcheight: *mut word,
                                            mut type_0: libc::c_int,
                                            mut flags: libc::c_int)
 -> *mut byte {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: word = *srcwidth;
    let mut height: word = *srcheight;
    let mut samples: libc::c_int =
        (*PFDesc.as_ptr().offset(type_0 as isize)).bpp;
    let mut flip_x: qboolean =
        if flags & IMAGE_FLIP_X as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut flip_y: qboolean =
        if flags & IMAGE_FLIP_Y as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut flip_i: qboolean =
        if flags & IMAGE_ROT_90 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut row_inc: libc::c_int =
        (if flip_y as libc::c_uint != 0 { -samples } else { samples }) *
            width as libc::c_int;
    let mut col_inc: libc::c_int =
        if flip_x as libc::c_uint != 0 { -samples } else { samples };
    let mut row_ofs: libc::c_int =
        if flip_y as libc::c_uint != 0 {
            ((height as libc::c_int - 1 as libc::c_int) *
                 width as libc::c_int) * samples
        } else { 0 as libc::c_int };
    let mut col_ofs: libc::c_int =
        if flip_x as libc::c_uint != 0 {
            (width as libc::c_int - 1 as libc::c_int) * samples
        } else { 0 as libc::c_int };
    let mut p: *const byte = 0 as *const byte;
    let mut line: *const byte = 0 as *const byte;
    let mut out: *mut byte = 0 as *mut byte;
    // nothing to process
    if flags &
           (IMAGE_FLIP_X as libc::c_int | IMAGE_FLIP_Y as libc::c_int |
                IMAGE_ROT_90 as libc::c_int) == 0 {
        return in_0 as *mut byte
    }
    match type_0 {
        1 | 2 | 5 | 6 | 3 | 4 => {
            image.tempbuffer =
                _Mem_Realloc(host.imagepool,
                             image.tempbuffer as *mut libc::c_void,
                             (width as libc::c_int * height as libc::c_int *
                                  samples) as size_t, true_0,
                             b"../engine/common/imagelib/img_utils.c\x00" as
                                 *const u8 as *const libc::c_char,
                             1138 as libc::c_int) as *mut byte
        }
        _ => { return in_0 as *mut byte }
    }
    out = image.tempbuffer;
    if flip_i as u64 != 0 {
        x = 0 as libc::c_int;
        line = in_0.offset(col_ofs as isize);
        while x < width as libc::c_int {
            y = 0 as libc::c_int;
            p = line.offset(row_ofs as isize);
            while y < height as libc::c_int {
                i = 0 as libc::c_int;
                while i < samples {
                    *out.offset(i as isize) = *p.offset(i as isize);
                    i += 1
                }
                y += 1;
                p = p.offset(row_inc as isize);
                out = out.offset(samples as isize)
            }
            x += 1;
            line = line.offset(col_inc as isize)
        }
    } else {
        y = 0 as libc::c_int;
        line = in_0.offset(row_ofs as isize);
        while y < height as libc::c_int {
            x = 0 as libc::c_int;
            p = line.offset(col_ofs as isize);
            while x < width as libc::c_int {
                i = 0 as libc::c_int;
                while i < samples {
                    *out.offset(i as isize) = *p.offset(i as isize);
                    i += 1
                }
                x += 1;
                p = p.offset(col_inc as isize);
                out = out.offset(samples as isize)
            }
            y += 1;
            line = line.offset(row_inc as isize)
        }
    }
    // update dims
    if flags & IMAGE_ROT_90 as libc::c_int != 0 {
        *srcwidth = height;
        *srcheight = width
    } else { *srcwidth = width; *srcheight = height }
    return image.tempbuffer;
}
#[no_mangle]
pub unsafe extern "C" fn Image_CreateLumaInternal(mut fin: *mut byte,
                                                  mut width: libc::c_int,
                                                  mut height: libc::c_int,
                                                  mut type_0: libc::c_int,
                                                  mut flags: libc::c_int)
 -> *mut byte {
    let mut out: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    if flags & IMAGE_HAS_LUMA as libc::c_int == 0 { return fin }
    match type_0 {
        1 | 2 => {
            image.tempbuffer =
                _Mem_Realloc(host.imagepool,
                             image.tempbuffer as *mut libc::c_void,
                             (width * height) as size_t, true_0,
                             b"../engine/common/imagelib/img_utils.c\x00" as
                                 *const u8 as *const libc::c_char,
                             1188 as libc::c_int) as *mut byte;
            out = image.tempbuffer;
            i = 0 as libc::c_int;
            while i < width * height {
                let fresh35 = out;
                out = out.offset(1);
                *fresh35 =
                    if *fin.offset(i as isize) as libc::c_int >=
                           224 as libc::c_int {
                        *fin.offset(i as isize) as libc::c_int
                    } else { 0 as libc::c_int } as byte;
                i += 1
            }
        }
        _ => {
            // another formats does ugly result :(
            Con_Printf(b"^1Error:^7 Image_MakeLuma: unsupported format %s\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*PFDesc.as_ptr().offset(type_0 as
                                                    isize)).name.as_ptr());
            return fin
        }
    }
    return image.tempbuffer;
}
#[no_mangle]
pub unsafe extern "C" fn Image_AddIndexedImageToPack(mut in_0: *const byte,
                                                     mut width: libc::c_int,
                                                     mut height: libc::c_int)
 -> qboolean {
    let mut mipsize: libc::c_int = width * height;
    let mut expand_to_rgba: qboolean = true_0;
    if Image_CheckFlag(IL_KEEP_8BIT as libc::c_int) as u64 != 0 {
        expand_to_rgba = false_0
    } else if image.flags &
                  (IMAGE_HAS_LUMA as libc::c_int |
                       IMAGE_QUAKESKY as libc::c_int) as libc::c_uint != 0 {
        expand_to_rgba = false_0
    }
    image.size = mipsize as size_t;
    if expand_to_rgba as u64 != 0 {
        image.size =
            (image.size as
                 libc::c_ulong).wrapping_mul(4 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    } else { Image_CopyPalette32bit(); }
    // reallocate image buffer
    image.rgba =
        _Mem_Alloc(host.imagepool, image.size, false_0,
                   b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                       as *const libc::c_char, 1217 as libc::c_int) as
            *mut byte; // probably pallette not installed
    if expand_to_rgba as u64 == 0 {
        memcpy(image.rgba as *mut libc::c_void, in_0 as *const libc::c_void,
               image.size);
    } else if Image_Copy8bitRGBA(in_0, image.rgba, mipsize) as u64 == 0 {
        return false_0
    }
    return true_0;
}
/*
=============
Image_Decompress

force to unpack any image to 32-bit buffer
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_Decompress(mut data: *const byte) -> qboolean {
    let mut fin: *mut byte = 0 as *mut byte;
    let mut fout: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if data.is_null() { return false_0 }
    fin = data as *mut byte;
    size =
        image.width as libc::c_int * image.height as libc::c_int *
            4 as libc::c_int;
    image.tempbuffer =
        _Mem_Realloc(host.imagepool, image.tempbuffer as *mut libc::c_void,
                     size as size_t, true_0,
                     b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                         as *const libc::c_char, 1241 as libc::c_int) as
            *mut byte;
    fout = image.tempbuffer;
    let mut current_block_38: u64;
    match (*PFDesc.as_ptr().offset(image.type_0 as isize)).format {
        1 => {
            if image.flags & IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint !=
                   0 {
                if image.flags &
                       IMAGE_COLORINDEX as libc::c_int as libc::c_uint != 0 {
                    Image_GetPaletteLMP(image.palette,
                                        LUMP_GRADIENT as libc::c_int);
                } else {
                    Image_GetPaletteLMP(image.palette,
                                        LUMP_MASKED as libc::c_int);
                }
            } else {
                Image_GetPaletteLMP(image.palette,
                                    LUMP_NORMAL as libc::c_int);
            }
            current_block_38 = 14779602513444186494;
        }
        2 => { current_block_38 = 14779602513444186494; }
        6 => {
            i = 0 as libc::c_int;
            while i < image.width as libc::c_int * image.height as libc::c_int
                  {
                *fout.offset(((i << 2 as libc::c_int) + 0 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 3 as libc::c_int + 2 as libc::c_int) as
                                    isize);
                *fout.offset(((i << 2 as libc::c_int) + 1 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 3 as libc::c_int + 1 as libc::c_int) as
                                    isize);
                *fout.offset(((i << 2 as libc::c_int) + 2 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 3 as libc::c_int + 0 as libc::c_int) as
                                    isize);
                *fout.offset(((i << 2 as libc::c_int) + 3 as libc::c_int) as
                                 isize) = 255 as libc::c_int as byte;
                i += 1
            }
            current_block_38 = 10891380440665537214;
        }
        5 => {
            i = 0 as libc::c_int;
            while i < image.width as libc::c_int * image.height as libc::c_int
                  {
                *fout.offset(((i << 2 as libc::c_int) + 0 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 3 as libc::c_int + 0 as libc::c_int) as
                                    isize);
                *fout.offset(((i << 2 as libc::c_int) + 1 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 3 as libc::c_int + 1 as libc::c_int) as
                                    isize);
                *fout.offset(((i << 2 as libc::c_int) + 2 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 3 as libc::c_int + 2 as libc::c_int) as
                                    isize);
                *fout.offset(((i << 2 as libc::c_int) + 3 as libc::c_int) as
                                 isize) = 255 as libc::c_int as byte;
                i += 1
            }
            current_block_38 = 10891380440665537214;
        }
        4 => {
            i = 0 as libc::c_int;
            while i < image.width as libc::c_int * image.height as libc::c_int
                  {
                *fout.offset((i * 4 as libc::c_int + 0 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 4 as libc::c_int + 2 as libc::c_int) as
                                    isize);
                *fout.offset((i * 4 as libc::c_int + 1 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 4 as libc::c_int + 1 as libc::c_int) as
                                    isize);
                *fout.offset((i * 4 as libc::c_int + 2 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 4 as libc::c_int + 0 as libc::c_int) as
                                    isize);
                *fout.offset((i * 4 as libc::c_int + 3 as libc::c_int) as
                                 isize) =
                    *fin.offset((i * 4 as libc::c_int + 3 as libc::c_int) as
                                    isize);
                i += 1
            }
            current_block_38 = 10891380440665537214;
        }
        3 => {
            // fast default case
            memcpy(fout as *mut libc::c_void, fin as *const libc::c_void,
                   size as libc::c_ulong);
            current_block_38 = 10891380440665537214;
        }
        _ => { return false_0 }
    }
    match current_block_38 {
        14779602513444186494 =>
        // intentionally fallthrough
        {
            if image.d_currentpal.is_null() {
                image.d_currentpal = image.palette as *mut uint
            }
            if Image_Copy8bitRGBA(fin, fout,
                                  image.width as libc::c_int *
                                      image.height as libc::c_int) as u64 == 0
               {
                return false_0
            }
        }
        _ => { }
    }
    // set new size
    image.size = size as size_t;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Image_DecompressInternal(mut pic: *mut rgbdata_t)
 -> *mut rgbdata_t {
    // quick case to reject unneeded conversions
    if (*pic).type_0 == PF_RGBA_32 as libc::c_int as libc::c_uint {
        return pic
    }
    Image_CopyParms(pic);
    image.ptr = 0 as libc::c_int as uint;
    image.size = image.ptr as size_t;
    Image_Decompress((*pic).buffer);
    // now we can change type to RGBA
    (*pic).type_0 = PF_RGBA_32 as libc::c_int as uint;
    (*pic).buffer =
        _Mem_Realloc(host.imagepool, (*pic).buffer as *mut libc::c_void,
                     image.size, true_0,
                     b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                         as *const libc::c_char, 1314 as libc::c_int) as
            *mut byte;
    memcpy((*pic).buffer as *mut libc::c_void,
           image.tempbuffer as *const libc::c_void, image.size);
    if !(*pic).palette.is_null() {
        _Mem_Free((*pic).palette as *mut libc::c_void,
                  b"../engine/common/imagelib/img_utils.c\x00" as *const u8 as
                      *const libc::c_char, 1316 as libc::c_int);
    }
    (*pic).flags = image.flags;
    (*pic).palette = 0 as *mut byte;
    return pic;
}
#[no_mangle]
pub unsafe extern "C" fn Image_LightGamma(mut pic: *mut rgbdata_t)
 -> *mut rgbdata_t {
    let mut in_0: *mut byte = (*pic).buffer;
    let mut i: libc::c_int = 0;
    if (*pic).type_0 != PF_RGBA_32 as libc::c_int as libc::c_uint {
        return pic
    }
    i = 0 as libc::c_int;
    while i < (*pic).width as libc::c_int * (*pic).height as libc::c_int {
        *in_0.offset(0 as libc::c_int as isize) =
            LightToTexGamma(*in_0.offset(0 as libc::c_int as isize));
        *in_0.offset(1 as libc::c_int as isize) =
            LightToTexGamma(*in_0.offset(1 as libc::c_int as isize));
        *in_0.offset(2 as libc::c_int as isize) =
            LightToTexGamma(*in_0.offset(2 as libc::c_int as isize));
        i += 1;
        in_0 = in_0.offset(4 as libc::c_int as isize)
    }
    return pic;
}
#[no_mangle]
pub unsafe extern "C" fn Image_RemapInternal(mut pic: *mut rgbdata_t,
                                             mut topColor: libc::c_int,
                                             mut bottomColor: libc::c_int)
 -> qboolean {
    if (*pic).palette.is_null() { return false_0 }
    match (*pic).type_0 {
        1 => { }
        2 => { Image_ConvertPalTo24bit(pic); }
        _ => { return false_0 }
    }
    if Image_ComparePalette((*pic).palette) == PAL_QUAKE1 as libc::c_int {
        Image_PaletteTranslate((*pic).palette, topColor * 16 as libc::c_int,
                               bottomColor * 16 as libc::c_int,
                               3 as libc::c_int);
    } else {
        // g-cont. preview images has a swapped top and bottom colors. I don't know why.
        Image_PaletteHueReplace((*pic).palette, topColor, 192 as libc::c_int,
                                223 as libc::c_int, 3 as libc::c_int);
        Image_PaletteHueReplace((*pic).palette, bottomColor,
                                160 as libc::c_int, 191 as libc::c_int,
                                3 as libc::c_int);
    }
    return true_0;
}
/*
==================
Image_ApplyFilter

Applies a 5 x 5 filtering matrix to the texture, then runs it through a simulated OpenGL texture environment
blend with the original data to derive a new texture.  Freaky, funky, and *f--king* *fantastic*.  You can do
reasonable enough "fake bumpmapping" with this baby...

Filtering algorithm from http://www.student.kuleuven.ac.be/~m0216922/CG/filtering.html
All credit due
==================
*/
unsafe extern "C" fn Image_ApplyFilter(mut pic: *mut rgbdata_t,
                                       mut factor: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut fin: *mut uint = 0 as *mut uint;
    let mut fout: *mut uint = 0 as *mut uint;
    let mut size: size_t = 0;
    // don't waste time
    if factor <= 0.0f32 { return }
    // first expand the image into 32-bit buffer
    pic = Image_DecompressInternal(pic);
    factor =
        if factor >= 0.0f32 {
            if factor < 1.0f32 { factor } else { 1.0f32 }
        } else { 0.0f32 };
    size =
        (image.width as libc::c_int * image.height as libc::c_int *
             4 as libc::c_int) as size_t;
    image.tempbuffer =
        _Mem_Realloc(host.imagepool, image.tempbuffer as *mut libc::c_void,
                     size, true_0,
                     b"../engine/common/imagelib/img_utils.c\x00" as *const u8
                         as *const libc::c_char, 1396 as libc::c_int) as
            *mut byte;
    fout = image.tempbuffer as *mut uint;
    fin = (*pic).buffer as *mut uint;
    x = 0 as libc::c_int;
    while x < image.width as libc::c_int {
        y = 0 as libc::c_int;
        while y < image.height as libc::c_int {
            let mut vout: vec3_t = [0.0f32, 0.0f32, 0.0f32];
            let mut pos_x: libc::c_int = 0;
            let mut pos_y: libc::c_int = 0;
            let mut avg: libc::c_float = 0.;
            pos_x = 0 as libc::c_int;
            while pos_x < 5 as libc::c_int {
                pos_y = 0 as libc::c_int;
                while pos_y < 5 as libc::c_int {
                    let mut img_x: libc::c_int =
                        (x - 5 as libc::c_int / 2 as libc::c_int + pos_x +
                             image.width as libc::c_int) %
                            image.width as libc::c_int;
                    let mut img_y: libc::c_int =
                        (y - 5 as libc::c_int / 2 as libc::c_int + pos_y +
                             image.height as libc::c_int) %
                            image.height as libc::c_int;
                    // casting's a unary operation anyway, so the othermost set of brackets in the left part
					// of the rvalue should not be necessary... but i'm paranoid when it comes to C...
                    vout[0 as libc::c_int as usize] +=
                        *(&mut *fin.offset((img_y * image.width as libc::c_int
                                                + img_x) as isize) as
                              *mut uint as
                              *mut byte).offset(0 as libc::c_int as isize) as
                            libc::c_float *
                            img_emboss[pos_x as usize][pos_y as usize];
                    vout[1 as libc::c_int as usize] +=
                        *(&mut *fin.offset((img_y * image.width as libc::c_int
                                                + img_x) as isize) as
                              *mut uint as
                              *mut byte).offset(1 as libc::c_int as isize) as
                            libc::c_float *
                            img_emboss[pos_x as usize][pos_y as usize];
                    vout[2 as libc::c_int as usize] +=
                        *(&mut *fin.offset((img_y * image.width as libc::c_int
                                                + img_x) as isize) as
                              *mut uint as
                              *mut byte).offset(2 as libc::c_int as isize) as
                            libc::c_float *
                            img_emboss[pos_x as usize][pos_y as usize];
                    pos_y += 1
                }
                pos_x += 1
            }
            // multiply by factor, add bias, and clamp
            i = 0 as libc::c_int; // base
            while i < 3 as libc::c_int {
                vout[i as usize] *= factor;
                vout[i as usize] += 128.0f32;
                vout[i as usize] =
                    if vout[i as usize] >= 0.0f32 {
                        if vout[i as usize] < 255.0f32 {
                            vout[i as usize]
                        } else { 255.0f32 }
                    } else { 0.0f32 };
                i += 1
            }
            // NTSC greyscale conversion standard
            avg =
                (vout[0 as libc::c_int as usize] * 30.0f32 +
                     vout[1 as libc::c_int as usize] * 59.0f32 +
                     vout[2 as libc::c_int as usize] * 11.0f32) / 100.0f32;
            // divide by 255 so GL operations work as expected
            vout[0 as libc::c_int as usize] = avg / 255.0f32;
            vout[1 as libc::c_int as usize] = avg / 255.0f32;
            vout[2 as libc::c_int as usize] = avg / 255.0f32;
            // write to temp - first, write data in (to get the alpha channel quickly and
			// easily, which will be left well alone by this particular operation...!)
            *fout.offset((y * image.width as libc::c_int + x) as isize) =
                *fin.offset((y * image.width as libc::c_int + x) as isize);
            // now write in each element, applying the blend operator.  blend
			// operators are based on standard OpenGL TexEnv modes, and the
			// formulas are derived from the OpenGL specs (http://www.opengl.org).
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                // divide by 255 so GL operations work as expected
                let mut src: libc::c_float =
                    *(&mut *fin.offset((y * image.width as libc::c_int + x) as
                                           isize) as *mut uint as
                          *mut byte).offset(i as isize) as libc::c_float /
                        255.0f32;
                let mut tmp: libc::c_float = 0.;
                // default is GL_BLEND here
				// CsS + CdD works out as Src * Dst * 2
                tmp = vout[i as usize] * src * 2.0f32;
                // multiply back by 255 to get the proper byte scale
                tmp *= 255.0f32;
                // bound the temp target again now, cos the operation may have thrown it out
                tmp =
                    if tmp >= 0.0f32 {
                        if tmp < 255.0f32 { tmp } else { 255.0f32 }
                    } else { 0.0f32 };
                // and copy it in
                *(&mut *fout.offset((y * image.width as libc::c_int + x) as
                                        isize) as *mut uint as
                      *mut byte).offset(i as isize) = tmp as byte;
                i += 1
            }
            y += 1
        }
        x += 1
    }
    // copy result back
    memcpy(fin as *mut libc::c_void, fout as *const libc::c_void, size);
}
#[no_mangle]
pub unsafe extern "C" fn Image_Process(mut pix: *mut *mut rgbdata_t,
                                       mut width: libc::c_int,
                                       mut height: libc::c_int,
                                       mut flags: uint,
                                       mut bumpscale: libc::c_float)
 -> qboolean {
    let mut pic: *mut rgbdata_t = *pix;
    let mut result: qboolean = true_0;
    let mut out: *mut byte = 0 as *mut byte;
    // check for buffers
    if pic.is_null() || (*pic).buffer.is_null() {
        image.force_flags = 0 as libc::c_int;
        return false_0
    }
    if flags == 0 {
        // clear any force flags
        image.force_flags = 0 as libc::c_int;
        return false_0
        // no operation specfied
    }
    if flags & IMAGE_MAKE_LUMA as libc::c_int as libc::c_uint != 0 {
        out =
            Image_CreateLumaInternal((*pic).buffer,
                                     (*pic).width as libc::c_int,
                                     (*pic).height as libc::c_int,
                                     (*pic).type_0 as libc::c_int,
                                     (*pic).flags as libc::c_int);
        if (*pic).buffer != out {
            memcpy((*pic).buffer as *mut libc::c_void,
                   image.tempbuffer as *const libc::c_void, (*pic).size);
        }
        (*pic).flags =
            (*pic).flags & !(IMAGE_HAS_LUMA as libc::c_int) as libc::c_uint
    }
    if flags & IMAGE_REMAP as libc::c_int as libc::c_uint != 0 {
        // NOTE: user should keep copy of indexed image manually for new changes
        if Image_RemapInternal(pic, width, height) as u64 != 0 {
            pic = Image_DecompressInternal(pic)
        }
    }
    // update format to RGBA if any
    if flags & IMAGE_FORCE_RGBA as libc::c_int as libc::c_uint != 0 {
        pic = Image_DecompressInternal(pic)
    } // 1 - 4096
    if flags & IMAGE_LIGHTGAMMA as libc::c_int as libc::c_uint != 0 {
        pic = Image_LightGamma(pic)
    } // 1 - 4096
    if flags & IMAGE_EMBOSS as libc::c_int as libc::c_uint != 0 {
        Image_ApplyFilter(pic, bumpscale);
    }
    out =
        Image_FlipInternal((*pic).buffer, &mut (*pic).width,
                           &mut (*pic).height, (*pic).type_0 as libc::c_int,
                           flags as libc::c_int);
    if (*pic).buffer != out {
        memcpy((*pic).buffer as *mut libc::c_void,
               image.tempbuffer as *const libc::c_void, (*pic).size);
    }
    if flags & IMAGE_RESAMPLE as libc::c_int as libc::c_uint != 0 &&
           width > 0 as libc::c_int && height > 0 as libc::c_int {
        let mut w: libc::c_int =
            if width >= 1 as libc::c_int {
                if width < 8192 as libc::c_int {
                    width
                } else { 8192 as libc::c_int }
            } else { 1 as libc::c_int };
        let mut h: libc::c_int =
            if height >= 1 as libc::c_int {
                if height < 8192 as libc::c_int {
                    height
                } else { 8192 as libc::c_int }
            } else { 1 as libc::c_int };
        let mut resampled: qboolean = false_0;
        out =
            Image_ResampleInternal((*pic).buffer as *mut uint as
                                       *const libc::c_void,
                                   (*pic).width as libc::c_int,
                                   (*pic).height as libc::c_int, w, h,
                                   (*pic).type_0 as libc::c_int,
                                   &mut resampled);
        if resampled as u64 != 0 {
            // resampled or filled
            Con_Reportf(b"Image_Resample: from[%d x %d] to [%d x %d]\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*pic).width as libc::c_int,
                        (*pic).height as libc::c_int, w, h);
            (*pic).width = w as word;
            (*pic).height = h as word;
            (*pic).size =
                (w * h *
                     (*PFDesc.as_ptr().offset((*pic).type_0 as isize)).bpp) as
                    size_t;
            // unzone buffer (don't touch image.tempbuffer)
            _Mem_Free((*pic).buffer as *mut libc::c_void,
                      b"../engine/common/imagelib/img_utils.c\x00" as
                          *const u8 as *const libc::c_char,
                      1531 as libc::c_int); // free original image buffer
            (*pic).buffer = Image_Copy((*pic).size)
        } else {
            // not a resampled or filled
            result = false_0
        }
    }
    // quantize image
    if flags & IMAGE_QUANTIZE as libc::c_int as libc::c_uint != 0 {
        pic = Image_Quantize(pic)
    }
    *pix = pic;
    // clear any force flags
    image.force_flags = 0 as libc::c_int;
    return result;
}
