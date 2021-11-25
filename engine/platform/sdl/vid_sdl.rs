#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type SDL_BlitMap;
    pub type SDL_Window;
    pub type SDL_Renderer;
    pub type SDL_Texture;
    pub type file_s;
    pub type grasshdr_s;
    pub type mip_s;
    #[no_mangle]
    fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);
    #[no_mangle]
    fn SDL_DestroyTexture(texture: *mut SDL_Texture);
    #[no_mangle]
    fn SDL_RenderPresent(renderer: *mut SDL_Renderer);
    #[no_mangle]
    fn SDL_RenderCopy(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture,
                      srcrect: *const SDL_Rect, dstrect: *const SDL_Rect)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_PixelFormatEnumToMasks(format: Uint32, bpp: *mut libc::c_int,
                                  Rmask: *mut Uint32, Gmask: *mut Uint32,
                                  Bmask: *mut Uint32, Amask: *mut Uint32)
     -> SDL_bool;
    #[no_mangle]
    fn SDL_CreateRGBSurfaceFrom(pixels: *mut libc::c_void, width: libc::c_int,
                                height: libc::c_int, depth: libc::c_int,
                                pitch: libc::c_int, Rmask: Uint32,
                                Gmask: Uint32, Bmask: Uint32, Amask: Uint32)
     -> *mut SDL_Surface;
    #[no_mangle]
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    #[no_mangle]
    fn SDL_LockSurface(surface: *mut SDL_Surface) -> libc::c_int;
    #[no_mangle]
    fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    #[no_mangle]
    fn SDL_UpperBlit(src: *mut SDL_Surface, srcrect: *const SDL_Rect,
                     dst: *mut SDL_Surface, dstrect: *mut SDL_Rect)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_VideoQuit();
    #[no_mangle]
    fn SDL_GetNumDisplayModes(displayIndex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetDisplayMode(displayIndex: libc::c_int, modeIndex: libc::c_int,
                          mode: *mut SDL_DisplayMode) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetDesktopDisplayMode(displayIndex: libc::c_int,
                                 mode: *mut SDL_DisplayMode) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetCurrentDisplayMode(displayIndex: libc::c_int,
                                 mode: *mut SDL_DisplayMode) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetClosestDisplayMode(displayIndex: libc::c_int,
                                 mode: *const SDL_DisplayMode,
                                 closest: *mut SDL_DisplayMode)
     -> *mut SDL_DisplayMode;
    #[no_mangle]
    fn SDL_SetWindowDisplayMode(window: *mut SDL_Window,
                                mode: *const SDL_DisplayMode) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetWindowPixelFormat(window: *mut SDL_Window) -> Uint32;
    #[no_mangle]
    fn SDL_CreateWindow(title: *const libc::c_char, x: libc::c_int,
                        y: libc::c_int, w: libc::c_int, h: libc::c_int,
                        flags: Uint32) -> *mut SDL_Window;
    #[no_mangle]
    fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface);
    #[no_mangle]
    fn SDL_SetWindowSize(window: *mut SDL_Window, w: libc::c_int,
                         h: libc::c_int);
    #[no_mangle]
    fn SDL_SetWindowBordered(window: *mut SDL_Window, bordered: SDL_bool);
    #[no_mangle]
    fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool);
    #[no_mangle]
    fn SDL_ShowWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn SDL_MinimizeWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn SDL_SetWindowFullscreen(window: *mut SDL_Window, flags: Uint32)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_GetWindowSurface(window: *mut SDL_Window) -> *mut SDL_Surface;
    #[no_mangle]
    fn SDL_UpdateWindowSurface(window: *mut SDL_Window) -> libc::c_int;
    #[no_mangle]
    fn SDL_SetWindowGrab(window: *mut SDL_Window, grabbed: SDL_bool);
    #[no_mangle]
    fn SDL_DestroyWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn SDL_GL_LoadLibrary(path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn SDL_GL_GetProcAddress(proc_0: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn SDL_GL_ResetAttributes();
    #[no_mangle]
    fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;
    #[no_mangle]
    fn SDL_GL_MakeCurrent(window: *mut SDL_Window, context: SDL_GLContext)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_GL_GetDrawableSize(window: *mut SDL_Window, w: *mut libc::c_int,
                              h: *mut libc::c_int);
    #[no_mangle]
    fn SDL_GL_SetSwapInterval(interval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_GL_SwapWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn SDL_GL_DeleteContext(context: SDL_GLContext);
    #[no_mangle]
    fn SDL_SetHint(name: *const libc::c_char, value: *const libc::c_char)
     -> SDL_bool;
    #[no_mangle]
    fn SDL_CreateRenderer(window: *mut SDL_Window, index: libc::c_int,
                          flags: Uint32) -> *mut SDL_Renderer;
    #[no_mangle]
    fn SDL_GetRendererInfo(renderer: *mut SDL_Renderer,
                           info: *mut SDL_RendererInfo) -> libc::c_int;
    #[no_mangle]
    fn SDL_CreateTexture(renderer: *mut SDL_Renderer, format: Uint32,
                         access: libc::c_int, w: libc::c_int, h: libc::c_int)
     -> *mut SDL_Texture;
    #[no_mangle]
    fn SDL_SetTextureBlendMode(texture: *mut SDL_Texture,
                               blendMode: SDL_BlendMode) -> libc::c_int;
    #[no_mangle]
    fn SDL_LockTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect,
                       pixels: *mut *mut libc::c_void,
                       pitch: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_UnlockTexture(texture: *mut SDL_Texture);
    #[no_mangle]
    fn SDL_RenderSetLogicalSize(renderer: *mut SDL_Renderer, w: libc::c_int,
                                h: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut gl_vsync: *mut convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_LoadImage(filename: *const libc::c_char, buffer: *const byte,
                    size: size_t) -> *mut rgbdata_t;
    #[no_mangle]
    fn FS_FreeImage(pack: *mut rgbdata_t);
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn Sys_Warn(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn COM_DefaultExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn _Sys_GetParmFromCmdLine(parm: *const libc::c_char,
                               out: *mut libc::c_char, size: size_t)
     -> qboolean;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut glw_state: glwstate_t;
    #[no_mangle]
    static mut vid_fullscreen: *mut convar_t;
    #[no_mangle]
    static mut vid_highdpi: *mut convar_t;
    #[no_mangle]
    static mut vid_rotate: *mut convar_t;
    #[no_mangle]
    static mut vid_scale: *mut convar_t;
    #[no_mangle]
    static mut gl_msaa_samples: *mut convar_t;
    #[no_mangle]
    fn R_SaveVideoMode(w: libc::c_int, h: libc::c_int, render_w: libc::c_int,
                       render_h: libc::c_int);
    #[no_mangle]
    fn VID_StartupGamma();
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Uint32 = uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_PIXELFORMAT_EXTERNAL_OES: C2RustUnnamed = 542328143;
pub const SDL_PIXELFORMAT_NV21: C2RustUnnamed = 825382478;
pub const SDL_PIXELFORMAT_NV12: C2RustUnnamed = 842094158;
pub const SDL_PIXELFORMAT_YVYU: C2RustUnnamed = 1431918169;
pub const SDL_PIXELFORMAT_UYVY: C2RustUnnamed = 1498831189;
pub const SDL_PIXELFORMAT_YUY2: C2RustUnnamed = 844715353;
pub const SDL_PIXELFORMAT_IYUV: C2RustUnnamed = 1448433993;
pub const SDL_PIXELFORMAT_YV12: C2RustUnnamed = 842094169;
pub const SDL_PIXELFORMAT_ABGR32: C2RustUnnamed = 373694468;
pub const SDL_PIXELFORMAT_BGRA32: C2RustUnnamed = 372645892;
pub const SDL_PIXELFORMAT_ARGB32: C2RustUnnamed = 377888772;
pub const SDL_PIXELFORMAT_RGBA32: C2RustUnnamed = 376840196;
pub const SDL_PIXELFORMAT_ARGB2101010: C2RustUnnamed = 372711428;
pub const SDL_PIXELFORMAT_BGRA8888: C2RustUnnamed = 377888772;
pub const SDL_PIXELFORMAT_ABGR8888: C2RustUnnamed = 376840196;
pub const SDL_PIXELFORMAT_RGBA8888: C2RustUnnamed = 373694468;
pub const SDL_PIXELFORMAT_ARGB8888: C2RustUnnamed = 372645892;
pub const SDL_PIXELFORMAT_BGRX8888: C2RustUnnamed = 375789572;
pub const SDL_PIXELFORMAT_BGR888: C2RustUnnamed = 374740996;
pub const SDL_PIXELFORMAT_RGBX8888: C2RustUnnamed = 371595268;
pub const SDL_PIXELFORMAT_RGB888: C2RustUnnamed = 370546692;
pub const SDL_PIXELFORMAT_BGR24: C2RustUnnamed = 390076419;
pub const SDL_PIXELFORMAT_RGB24: C2RustUnnamed = 386930691;
pub const SDL_PIXELFORMAT_BGR565: C2RustUnnamed = 357896194;
pub const SDL_PIXELFORMAT_RGB565: C2RustUnnamed = 353701890;
pub const SDL_PIXELFORMAT_BGRA5551: C2RustUnnamed = 360976386;
pub const SDL_PIXELFORMAT_ABGR1555: C2RustUnnamed = 359862274;
pub const SDL_PIXELFORMAT_RGBA5551: C2RustUnnamed = 356782082;
pub const SDL_PIXELFORMAT_ARGB1555: C2RustUnnamed = 355667970;
pub const SDL_PIXELFORMAT_BGRA4444: C2RustUnnamed = 360845314;
pub const SDL_PIXELFORMAT_ABGR4444: C2RustUnnamed = 359796738;
pub const SDL_PIXELFORMAT_RGBA4444: C2RustUnnamed = 356651010;
pub const SDL_PIXELFORMAT_ARGB4444: C2RustUnnamed = 355602434;
pub const SDL_PIXELFORMAT_BGR555: C2RustUnnamed = 357764866;
pub const SDL_PIXELFORMAT_RGB555: C2RustUnnamed = 353570562;
pub const SDL_PIXELFORMAT_RGB444: C2RustUnnamed = 353504258;
pub const SDL_PIXELFORMAT_RGB332: C2RustUnnamed = 336660481;
pub const SDL_PIXELFORMAT_INDEX8: C2RustUnnamed = 318769153;
pub const SDL_PIXELFORMAT_INDEX4MSB: C2RustUnnamed = 304088064;
pub const SDL_PIXELFORMAT_INDEX4LSB: C2RustUnnamed = 303039488;
pub const SDL_PIXELFORMAT_INDEX1MSB: C2RustUnnamed = 287310080;
pub const SDL_PIXELFORMAT_INDEX1LSB: C2RustUnnamed = 286261504;
pub const SDL_PIXELFORMAT_UNKNOWN: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: libc::c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: libc::c_int,
    pub next: *mut SDL_PixelFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
pub type SDL_BlendMode = libc::c_uint;
pub const SDL_BLENDMODE_INVALID: SDL_BlendMode = 2147483647;
pub const SDL_BLENDMODE_MOD: SDL_BlendMode = 4;
pub const SDL_BLENDMODE_ADD: SDL_BlendMode = 2;
pub const SDL_BLENDMODE_BLEND: SDL_BlendMode = 1;
pub const SDL_BLENDMODE_NONE: SDL_BlendMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub pitch: libc::c_int,
    pub pixels: *mut libc::c_void,
    pub userdata: *mut libc::c_void,
    pub locked: libc::c_int,
    pub lock_data: *mut libc::c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayMode {
    pub format: Uint32,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub refresh_rate: libc::c_int,
    pub driverdata: *mut libc::c_void,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SDL_WINDOW_VULKAN: C2RustUnnamed_0 = 268435456;
pub const SDL_WINDOW_POPUP_MENU: C2RustUnnamed_0 = 524288;
pub const SDL_WINDOW_TOOLTIP: C2RustUnnamed_0 = 262144;
pub const SDL_WINDOW_UTILITY: C2RustUnnamed_0 = 131072;
pub const SDL_WINDOW_SKIP_TASKBAR: C2RustUnnamed_0 = 65536;
pub const SDL_WINDOW_ALWAYS_ON_TOP: C2RustUnnamed_0 = 32768;
pub const SDL_WINDOW_MOUSE_CAPTURE: C2RustUnnamed_0 = 16384;
pub const SDL_WINDOW_ALLOW_HIGHDPI: C2RustUnnamed_0 = 8192;
pub const SDL_WINDOW_FOREIGN: C2RustUnnamed_0 = 2048;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: C2RustUnnamed_0 = 4097;
pub const SDL_WINDOW_MOUSE_FOCUS: C2RustUnnamed_0 = 1024;
pub const SDL_WINDOW_INPUT_FOCUS: C2RustUnnamed_0 = 512;
pub const SDL_WINDOW_INPUT_GRABBED: C2RustUnnamed_0 = 256;
pub const SDL_WINDOW_MAXIMIZED: C2RustUnnamed_0 = 128;
pub const SDL_WINDOW_MINIMIZED: C2RustUnnamed_0 = 64;
pub const SDL_WINDOW_RESIZABLE: C2RustUnnamed_0 = 32;
pub const SDL_WINDOW_BORDERLESS: C2RustUnnamed_0 = 16;
pub const SDL_WINDOW_HIDDEN: C2RustUnnamed_0 = 8;
pub const SDL_WINDOW_SHOWN: C2RustUnnamed_0 = 4;
pub const SDL_WINDOW_OPENGL: C2RustUnnamed_0 = 2;
pub const SDL_WINDOW_FULLSCREEN: C2RustUnnamed_0 = 1;
pub type SDL_GLContext = *mut libc::c_void;
pub type SDL_GLattr = libc::c_uint;
pub const SDL_GL_CONTEXT_NO_ERROR: SDL_GLattr = 26;
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: SDL_GLattr = 25;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: SDL_GLattr = 24;
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: SDL_GLattr = 23;
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: SDL_GLattr = 22;
pub const SDL_GL_CONTEXT_PROFILE_MASK: SDL_GLattr = 21;
pub const SDL_GL_CONTEXT_FLAGS: SDL_GLattr = 20;
pub const SDL_GL_CONTEXT_EGL: SDL_GLattr = 19;
pub const SDL_GL_CONTEXT_MINOR_VERSION: SDL_GLattr = 18;
pub const SDL_GL_CONTEXT_MAJOR_VERSION: SDL_GLattr = 17;
pub const SDL_GL_RETAINED_BACKING: SDL_GLattr = 16;
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = 15;
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = 14;
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = 13;
pub const SDL_GL_STEREO: SDL_GLattr = 12;
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = 11;
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = 10;
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = 9;
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = 8;
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = 7;
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = 6;
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = 5;
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = 4;
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = 3;
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = 2;
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = 1;
pub const SDL_GL_RED_SIZE: SDL_GLattr = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_RendererInfo {
    pub name: *const libc::c_char,
    pub flags: Uint32,
    pub num_texture_formats: Uint32,
    pub texture_formats: [Uint32; 16],
    pub max_texture_width: libc::c_int,
    pub max_texture_height: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SDL_TEXTUREACCESS_TARGET: C2RustUnnamed_1 = 2;
pub const SDL_TEXTUREACCESS_STREAMING: C2RustUnnamed_1 = 1;
pub const SDL_TEXTUREACCESS_STATIC: C2RustUnnamed_1 = 0;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
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
pub type vec2_t = [vec_t; 2];
pub type vec3_t = [vec_t; 3];
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
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
pub type rserr_t = libc::c_uint;
pub const rserr_unknown: rserr_t = 3;
pub const rserr_invalid_mode: rserr_t = 2;
pub const rserr_invalid_fullscreen: rserr_t = 1;
pub const rserr_ok: rserr_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vidmode_s {
    pub desc: *const libc::c_char,
    pub width: libc::c_int,
    pub height: libc::c_int,
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
pub type vidmode_t = vidmode_s;
pub const REF_SOFTWARE: ref_graphic_apis_e = 0;
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
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
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
pub type msurface_t = msurface_s;
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
pub struct mfacebevel_t {
    pub edges: *mut mplane_t,
    pub numedges: libc::c_int,
    pub origin: vec3_t,
    pub radius: vec_t,
    pub contents: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_t {
    pub vecs: [[libc::c_float; 4]; 2],
    pub faceinfo: *mut mfaceinfo_t,
    pub texture: *mut texture_t,
    pub flags: libc::c_int,
}
pub type texture_t = texture_s;
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
pub struct mfaceinfo_t {
    pub landname: [libc::c_char; 16],
    pub texture_step: libc::c_ushort,
    pub max_extent: libc::c_ushort,
    pub groupid: libc::c_short,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub reserved: [libc::c_int; 32],
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
pub struct mclipnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
}
pub type mnode_t = mnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
pub type mleaf_t = mleaf_s;
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
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
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
pub type model_t = model_s;
pub type cl_entity_t = cl_entity_s;
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
pub type rgbdata_t = rgbdata_s;
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
pub const REF_GL: ref_graphic_apis_e = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub prev_width: libc::c_int,
    pub prev_height: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub renderer: *mut SDL_Renderer,
    pub tex: *mut SDL_Texture,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub surf: *mut SDL_Surface,
    pub win: *mut SDL_Surface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glwstate_t {
    pub context: *mut libc::c_void,
    pub safe: libc::c_int,
    pub desktopBitsPixel: libc::c_int,
    pub desktopHeight: libc::c_int,
    pub initialized: qboolean,
    pub extended: qboolean,
    pub software: qboolean,
}
pub type gameinfo_t = gameinfo_s;
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
pub type sysinfo_t = sysinfo_s;
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
pub const SAFE_LAST: C2RustUnnamed_4 = 8;
pub const SAFE_NOMSAA: C2RustUnnamed_4 = 1;
pub const SAFE_NO: C2RustUnnamed_4 = 0;
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
pub const SAFE_DONTCARE: C2RustUnnamed_4 = 7;
pub const ca_active: connstate_e = 4;
pub type connstate_t = connstate_e;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
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
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
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
pub type netsrc_t = libc::c_uint;
pub const NS_COUNT: netsrc_t = 2;
pub const NS_SERVER: netsrc_t = 1;
pub const NS_CLIENT: netsrc_t = 0;
pub type local_state_t = local_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
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
pub type keydest_t = libc::c_uint;
pub const key_message: keydest_t = 3;
pub const key_menu: keydest_t = 2;
pub const key_game: keydest_t = 1;
pub const key_console: keydest_t = 0;
pub const REF_GL_CONTEXT_NO_ERROR: C2RustUnnamed_5 = 19;
pub const REF_GL_CONTEXT_RESET_NOTIFICATION: C2RustUnnamed_5 = 18;
pub const REF_GL_CONTEXT_RELEASE_BEHAVIOR: C2RustUnnamed_5 = 17;
pub const REF_GL_CONTEXT_PROFILE_ES: C2RustUnnamed_6 = 4;
pub const REF_GL_CONTEXT_PROFILE_MASK: C2RustUnnamed_5 = 14;
pub const REF_GL_FRAMEBUFFER_SRGB_CAPABLE: C2RustUnnamed_5 = 16;
pub const REF_GL_SHARE_WITH_CURRENT_CONTEXT: C2RustUnnamed_5 = 15;
pub const REF_GL_CONTEXT_FLAGS: C2RustUnnamed_5 = 13;
pub const REF_GL_CONTEXT_EGL: C2RustUnnamed_5 = 12;
pub const REF_GL_CONTEXT_MINOR_VERSION: C2RustUnnamed_5 = 11;
pub const REF_GL_CONTEXT_MAJOR_VERSION: C2RustUnnamed_5 = 10;
pub const REF_GL_ACCELERATED_VISUAL: C2RustUnnamed_5 = 9;
pub const REF_GL_MULTISAMPLESAMPLES: C2RustUnnamed_5 = 8;
pub const REF_GL_MULTISAMPLEBUFFERS: C2RustUnnamed_5 = 7;
pub const REF_GL_STENCIL_SIZE: C2RustUnnamed_5 = 6;
pub const REF_GL_DEPTH_SIZE: C2RustUnnamed_5 = 5;
pub const REF_GL_DOUBLEBUFFER: C2RustUnnamed_5 = 4;
pub const REF_GL_ALPHA_SIZE: C2RustUnnamed_5 = 3;
pub const REF_GL_BLUE_SIZE: C2RustUnnamed_5 = 2;
pub const REF_GL_GREEN_SIZE: C2RustUnnamed_5 = 1;
pub const REF_GL_RED_SIZE: C2RustUnnamed_5 = 0;
pub type ref_graphic_apis_e = libc::c_uint;
pub const REF_D3D: ref_graphic_apis_e = 2;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SAFE_NOCOLOR: C2RustUnnamed_4 = 6;
pub const SAFE_NODEPTH: C2RustUnnamed_4 = 5;
pub const SAFE_NOALPHA: C2RustUnnamed_4 = 4;
pub const SAFE_NOSTENCIL: C2RustUnnamed_4 = 3;
pub const SAFE_NOACC: C2RustUnnamed_4 = 2;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const REF_GL_ATTRIBUTES_COUNT: C2RustUnnamed_5 = 20;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const REF_GL_CONTEXT_PROFILE_COMPATIBILITY: C2RustUnnamed_6 = 2;
pub const REF_GL_CONTEXT_PROFILE_CORE: C2RustUnnamed_6 = 1;
/*
vid_sdl.c - SDL vid component
Copyright (C) 2018 a1batross

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
static mut vidmodes: *mut vidmode_t = 0 as *const vidmode_t as *mut vidmode_t;
static mut num_vidmodes: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sdlState: C2RustUnnamed_2 =
    {
        let mut init =
            C2RustUnnamed_2{prev_width: 640 as libc::c_int,
                            prev_height: 480 as libc::c_int,};
        init
    };
#[no_mangle]
pub static mut sw: C2RustUnnamed_3 =
    C2RustUnnamed_3{renderer: 0 as *const SDL_Renderer as *mut SDL_Renderer,
                    tex: 0 as *const SDL_Texture as *mut SDL_Texture,
                    width: 0,
                    height: 0,
                    surf: 0 as *const SDL_Surface as *mut SDL_Surface,
                    win: 0 as *const SDL_Surface as *mut SDL_Surface,};
#[no_mangle]
pub unsafe extern "C" fn SW_CreateBuffer(mut width: libc::c_int,
                                         mut height: libc::c_int,
                                         mut stride: *mut uint,
                                         mut bpp: *mut uint, mut r: *mut uint,
                                         mut g: *mut uint, mut b: *mut uint)
 -> qboolean {
    sw.width = width;
    sw.height = height;
    if !sw.renderer.is_null() {
        let mut format: libc::c_uint =
            SDL_GetWindowPixelFormat(host.hWnd as *mut SDL_Window);
        SDL_RenderSetLogicalSize(sw.renderer, refState.width,
                                 refState.height);
        if !sw.tex.is_null() { SDL_DestroyTexture(sw.tex); }
        // guess
        if format == SDL_PIXELFORMAT_UNKNOWN as libc::c_int as libc::c_uint {
            if refState.desktopBitsPixel == 16 as libc::c_int {
                format = SDL_PIXELFORMAT_RGB565 as libc::c_int as libc::c_uint
            } else {
                format =
                    SDL_PIXELFORMAT_RGBA8888 as libc::c_int as libc::c_uint
            }
        }
        // we can only copy fast 16 or 32 bits
		// SDL_Renderer does not allow zero-copy, so 24 bits will be ineffective
        if !((if format != 0 &&
                     format >> 28 as libc::c_int &
                         0xf as libc::c_int as libc::c_uint !=
                         1 as libc::c_int as libc::c_uint {
                  (if format ==
                          SDL_PIXELFORMAT_YUY2 as libc::c_int as libc::c_uint
                          ||
                          format ==
                              SDL_PIXELFORMAT_UYVY as libc::c_int as
                                  libc::c_uint ||
                          format ==
                              SDL_PIXELFORMAT_YVYU as libc::c_int as
                                  libc::c_uint {
                       2 as libc::c_int
                   } else { 1 as libc::c_int }) as libc::c_uint
              } else {
                  (format >> 0 as libc::c_int) &
                      0xff as libc::c_int as libc::c_uint
              }) == 2 as libc::c_int as libc::c_uint ||
                 (if format != 0 &&
                         format >> 28 as libc::c_int &
                             0xf as libc::c_int as libc::c_uint !=
                             1 as libc::c_int as libc::c_uint {
                      (if format ==
                              SDL_PIXELFORMAT_YUY2 as libc::c_int as
                                  libc::c_uint ||
                              format ==
                                  SDL_PIXELFORMAT_UYVY as libc::c_int as
                                      libc::c_uint ||
                              format ==
                                  SDL_PIXELFORMAT_YVYU as libc::c_int as
                                      libc::c_uint {
                           2 as libc::c_int
                       } else { 1 as libc::c_int }) as libc::c_uint
                  } else {
                      (format >> 0 as libc::c_int) &
                          0xff as libc::c_int as libc::c_uint
                  }) == 4 as libc::c_int as libc::c_uint) {
            format = SDL_PIXELFORMAT_RGBA8888 as libc::c_int as libc::c_uint
        }
        sw.tex =
            SDL_CreateTexture(sw.renderer, format,
                              SDL_TEXTUREACCESS_STREAMING as libc::c_int,
                              width, height);
        // fallback
        if sw.tex.is_null() &&
               format !=
                   SDL_PIXELFORMAT_RGBA8888 as libc::c_int as libc::c_uint {
            format = SDL_PIXELFORMAT_RGBA8888 as libc::c_int as libc::c_uint;
            sw.tex =
                SDL_CreateTexture(sw.renderer, format,
                                  SDL_TEXTUREACCESS_STREAMING as libc::c_int,
                                  width, height)
        }
        if sw.tex.is_null() {
            SDL_DestroyRenderer(sw.renderer);
            sw.renderer = 0 as *mut SDL_Renderer
        } else {
            let mut pixels: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut pitch: libc::c_int = 0;
            if SDL_LockTexture(sw.tex, 0 as *const SDL_Rect, &mut pixels,
                               &mut pitch) == 0 {
                let mut bits: libc::c_int = 0;
                let mut amask: uint = 0;
                // lock successfull, release
                SDL_UnlockTexture(sw.tex);
                // enough for building blitter tables
                SDL_PixelFormatEnumToMasks(format, &mut bits, r, g, b,
                                           &mut amask);
                *bpp =
                    if format != 0 &&
                           format >> 28 as libc::c_int &
                               0xf as libc::c_int as libc::c_uint !=
                               1 as libc::c_int as libc::c_uint {
                        (if format ==
                                SDL_PIXELFORMAT_YUY2 as libc::c_int as
                                    libc::c_uint ||
                                format ==
                                    SDL_PIXELFORMAT_UYVY as libc::c_int as
                                        libc::c_uint ||
                                format ==
                                    SDL_PIXELFORMAT_YVYU as libc::c_int as
                                        libc::c_uint {
                             2 as libc::c_int
                         } else { 1 as libc::c_int }) as libc::c_uint
                    } else {
                        (format >> 0 as libc::c_int) &
                            0xff as libc::c_int as libc::c_uint
                    };
                *stride = (pitch as libc::c_uint).wrapping_div(*bpp);
                return true_0
            }
            // fallback to surf
            SDL_DestroyTexture(sw.tex);
            sw.tex = 0 as *mut SDL_Texture;
            SDL_DestroyRenderer(sw.renderer);
            sw.renderer = 0 as *mut SDL_Renderer
        }
    }
    if sw.renderer.is_null() {
        sw.win = SDL_GetWindowSurface(host.hWnd as *mut SDL_Window);
        // / TODO: check somehow if ref_soft can handle native format
        if sw.win.is_null() {
            Sys_Warn(b"failed to initialize software output, try enable sw_glblit\x00"
                         as *const u8 as *const libc::c_char);
            return false_0
        }
        *bpp = (*(*sw.win).format).BytesPerPixel as uint;
        *r = (*(*sw.win).format).Rmask;
        *g = (*(*sw.win).format).Gmask;
        *b = (*(*sw.win).format).Bmask;
        *stride =
            ((*sw.win).pitch /
                 (*(*sw.win).format).BytesPerPixel as libc::c_int) as uint
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // sdl will create renderer if hw framebuffer unavailiable, so cannot fallback here
		// if it is failed, it is not possible to draw with SDL in REF_SOFTWARE mode
    // we can't create ref_soft buffer
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SW_LockBuffer() -> *mut libc::c_void {
    if !sw.renderer.is_null() {
        let mut pixels: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut stride: libc::c_int = 0;
        if SDL_LockTexture(sw.tex, 0 as *const SDL_Rect, &mut pixels,
                           &mut stride) != 0 {
            Sys_Error(b"%s\x00" as *const u8 as *const libc::c_char,
                      SDL_GetError());
        }
        return pixels
    }
    // ensure it not changed (do we really need this?)
    sw.win = SDL_GetWindowSurface(host.hWnd as *mut SDL_Window);
    //if( !sw.win )
		//SDL_GetWindowSurface( host.hWnd );
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // prevent buffer overrun
    if sw.win.is_null() || (*sw.win).w < sw.width || (*sw.win).h < sw.height {
        return 0 as *mut libc::c_void
    }
    if !sw.surf.is_null() {
        SDL_LockSurface(sw.surf);
        return (*sw.surf).pixels
    } else {
        // SDL_VERSION_ATLEAST( 2, 0, 0 )
        // real window pixels (x11 shm region, dma buffer, etc)
		// or SDL_Renderer texture if not supported
        SDL_LockSurface(sw.win);
        return (*sw.win).pixels
    };
}
#[no_mangle]
pub unsafe extern "C" fn SW_UnlockBuffer() {
    if !sw.renderer.is_null() {
        let mut src: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
        let mut dst: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
        src.y = 0 as libc::c_int;
        src.x = src.y;
        src.w = sw.width;
        src.h = sw.height;
        dst = src;
        SDL_UnlockTexture(sw.tex);
        SDL_SetTextureBlendMode(sw.tex, SDL_BLENDMODE_NONE);
        SDL_RenderCopy(sw.renderer, sw.tex, &mut src, &mut dst);
        SDL_RenderPresent(sw.renderer);
        return
        //Con_Printf("%s\n", SDL_GetError());
    }
    // blit if blitting surface availiable
    if !sw.surf.is_null() {
        let mut src_0: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
        let mut dst_0: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
        src_0.y = 0 as libc::c_int;
        src_0.x = src_0.y;
        src_0.w = sw.width;
        src_0.h = sw.height;
        dst_0 = src_0;
        SDL_UnlockSurface(sw.surf);
        SDL_UpperBlit(sw.surf, &mut src_0, sw.win, &mut dst_0);
        return
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // already blitted
    SDL_UnlockSurface(sw.win);
    SDL_UpdateWindowSurface(host.hWnd as *mut SDL_Window);
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
#[no_mangle]
pub unsafe extern "C" fn R_MaxVideoModes() -> libc::c_int {
    return num_vidmodes; // TODO: handle multiple displays somehow
}
#[no_mangle]
pub unsafe extern "C" fn R_GetVideoMode(mut num: libc::c_int)
 -> *mut vidmode_s {
    if vidmodes.is_null() || num < 0 as libc::c_int ||
           num >= R_MaxVideoModes() {
        return 0 as *mut vidmode_s
    }
    return vidmodes.offset(num as isize);
}
unsafe extern "C" fn R_InitVideoModes() {
    let mut displayIndex: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut modes: libc::c_int = 0;
    num_vidmodes = 0 as libc::c_int;
    modes = SDL_GetNumDisplayModes(displayIndex);
    if modes == 0 { return }
    vidmodes =
        _Mem_Alloc(host.mempool,
                   (modes as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<vidmode_t>()
                                                        as libc::c_ulong),
                   false_0,
                   b"../engine/platform/sdl/vid_sdl.c\x00" as *const u8 as
                       *const libc::c_char, 269 as libc::c_int) as
            *mut vidmode_t;
    i = 0 as libc::c_int;
    while i < modes {
        let mut j: libc::c_int = 0;
        let mut mode: SDL_DisplayMode =
            SDL_DisplayMode{format: 0,
                            w: 0,
                            h: 0,
                            refresh_rate: 0,
                            driverdata: 0 as *mut libc::c_void,};
        if SDL_GetDisplayMode(displayIndex, i, &mut mode) != 0 {
            Con_Printf(b"SDL_GetDisplayMode: %s\n\x00" as *const u8 as
                           *const libc::c_char, SDL_GetError());
        } else if !(mode.w < 320 as libc::c_int ||
                        mode.h < 200 as libc::c_int) {
            j = 0 as libc::c_int;
            while j < num_vidmodes {
                if mode.w == (*vidmodes.offset(j as isize)).width &&
                       mode.h == (*vidmodes.offset(j as isize)).height {
                    break ;
                }
                j += 1
            }
            if !(j != num_vidmodes) {
                (*vidmodes.offset(num_vidmodes as isize)).width = mode.w;
                (*vidmodes.offset(num_vidmodes as isize)).height = mode.h;
                let ref mut fresh0 =
                    (*vidmodes.offset(num_vidmodes as isize)).desc;
                *fresh0 =
                    _copystring(host.mempool,
                                va(b"%ix%i\x00" as *const u8 as
                                       *const libc::c_char, mode.w, mode.h),
                                b"../engine/platform/sdl/vid_sdl.c\x00" as
                                    *const u8 as *const libc::c_char,
                                298 as libc::c_int);
                num_vidmodes += 1
            }
        }
        i += 1
    };
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
unsafe extern "C" fn R_FreeVideoModes() {
    let mut i: libc::c_int = 0;
    if vidmodes.is_null() { return }
    i = 0 as libc::c_int;
    while i < num_vidmodes {
        _Mem_Free((*vidmodes.offset(i as isize)).desc as *mut libc::c_char as
                      *mut libc::c_void,
                  b"../engine/platform/sdl/vid_sdl.c\x00" as *const u8 as
                      *const libc::c_char, 349 as libc::c_int);
        i += 1
    }
    _Mem_Free(vidmodes as *mut libc::c_void,
              b"../engine/platform/sdl/vid_sdl.c\x00" as *const u8 as
                  *const libc::c_char, 350 as libc::c_int);
    vidmodes = 0 as *mut vidmode_t;
}
/*
=================
GL_GetProcAddress
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_GetProcAddress(mut name: *const libc::c_char)
 -> *mut libc::c_void {
    let mut func: *mut libc::c_void = SDL_GL_GetProcAddress(name);
    if func.is_null() {
        Con_Reportf(b"^1Error:^7 Error: GL_GetProcAddress failed for %s\n\x00"
                        as *const u8 as *const libc::c_char, name);
    }
    return func;
}
/*
===============
GL_UpdateSwapInterval
===============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_UpdateSwapInterval() {
    // disable VSync while level is loading
    if (cls.state as libc::c_uint) < ca_active as libc::c_int as libc::c_uint
       {
        SDL_GL_SetSwapInterval(0 as libc::c_int);
        (*gl_vsync).flags =
            (*gl_vsync).flags | (1 as libc::c_int) << 13 as libc::c_int
    } else if (*gl_vsync).flags & (1 as libc::c_int) << 13 as libc::c_int != 0
     {
        (*gl_vsync).flags =
            (*gl_vsync).flags & !((1 as libc::c_int) << 13 as libc::c_int);
        if SDL_GL_SetSwapInterval((*gl_vsync).value as libc::c_int) != 0 {
            Con_Reportf(b"^1Error:^7 SDL_GL_SetSwapInterval: %s\n\x00" as
                            *const u8 as *const libc::c_char, SDL_GetError());
        }
    };
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
/*
=================
GL_DeleteContext

always return false
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_DeleteContext() -> qboolean {
    if !glw_state.context.is_null() {
        SDL_GL_DeleteContext(glw_state.context);
        glw_state.context = 0 as *mut libc::c_void
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    return false_0;
}
/*
=================
GL_CreateContext
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CreateContext() -> qboolean {
    glw_state.context = SDL_GL_CreateContext(host.hWnd as *mut SDL_Window);
    if glw_state.context.is_null() {
        Con_Reportf(b"^1Error:^7 GL_CreateContext: %s\n\x00" as *const u8 as
                        *const libc::c_char, SDL_GetError());
        return GL_DeleteContext()
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    return true_0;
}
/*
=================
GL_UpdateContext
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_UpdateContext() -> qboolean {
    if SDL_GL_MakeCurrent(host.hWnd as *mut SDL_Window, glw_state.context) !=
           0 {
        Con_Reportf(b"^1Error:^7 GL_UpdateContext: %s\n\x00" as *const u8 as
                        *const libc::c_char, SDL_GetError());
        return GL_DeleteContext()
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    return true_0; // don't care
}
#[no_mangle]
pub unsafe extern "C" fn VID_SaveWindowSize(mut width: libc::c_int,
                                            mut height: libc::c_int) {
    let mut render_w: libc::c_int = width;
    let mut render_h: libc::c_int = height;
    let mut rotate: uint = (*vid_rotate).value as uint;
    if glw_state.software as u64 == 0 {
        SDL_GL_GetDrawableSize(host.hWnd as *mut SDL_Window, &mut render_w,
                               &mut render_h);
    } else { SDL_RenderSetLogicalSize(sw.renderer, width, height); }
    if ref_0.dllFuncs.R_SetDisplayTransform.expect("non-null function pointer")(rotate
                                                                                    as
                                                                                    ref_screen_rotation_t,
                                                                                0
                                                                                    as
                                                                                    libc::c_int,
                                                                                0
                                                                                    as
                                                                                    libc::c_int,
                                                                                (*vid_scale).value,
                                                                                (*vid_scale).value)
           as u64 != 0 {
        if rotate & 1 as libc::c_int as libc::c_uint != 0 {
            let mut swap: libc::c_int = render_w;
            render_w = render_h;
            render_h = swap
        }
        render_h =
            (render_h as libc::c_float / (*vid_scale).value) as libc::c_int;
        render_w =
            (render_w as libc::c_float / (*vid_scale).value) as libc::c_int
    } else {
        Con_Printf(b"^3Warning:^7 failed to setup screen transform\n\x00" as
                       *const u8 as *const libc::c_char);
    }
    R_SaveVideoMode(width, height, render_w, render_h);
}
unsafe extern "C" fn VID_SetScreenResolution(mut width: libc::c_int,
                                             mut height: libc::c_int)
 -> qboolean {
    let mut want: SDL_DisplayMode =
        SDL_DisplayMode{format: 0,
                        w: 0,
                        h: 0,
                        refresh_rate: 0,
                        driverdata: 0 as *mut libc::c_void,};
    let mut got: SDL_DisplayMode =
        SDL_DisplayMode{format: 0,
                        w: 0,
                        h: 0,
                        refresh_rate: 0,
                        driverdata: 0 as *mut libc::c_void,};
    let mut wndFlags: Uint32 = 0 as libc::c_int as Uint32;
    static mut wndname: string = [0; 256];
    if (*vid_highdpi).value != 0. {
        wndFlags |= SDL_WINDOW_ALLOW_HIGHDPI as libc::c_int as libc::c_uint
    }
    Q_strncpy(wndname.as_mut_ptr(), (*SI.GameInfo).title.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    want.w = width;
    want.h = height;
    want.driverdata = 0 as *mut libc::c_void;
    want.refresh_rate = 0 as libc::c_int;
    want.format = want.refresh_rate as Uint32;
    if SDL_GetClosestDisplayMode(0 as libc::c_int, &mut want,
                                 &mut got).is_null() {
        return false_0
    }
    Con_Reportf(b"Got closest display mode: %ix%i@%i\n\x00" as *const u8 as
                    *const libc::c_char, got.w, got.h, got.refresh_rate);
    if SDL_SetWindowDisplayMode(host.hWnd as *mut SDL_Window, &mut got) ==
           -(1 as libc::c_int) {
        return false_0
    }
    if SDL_SetWindowFullscreen(host.hWnd as *mut SDL_Window,
                               SDL_WINDOW_FULLSCREEN as libc::c_int as Uint32)
           == -(1 as libc::c_int) {
        return false_0
    }
    SDL_SetWindowBordered(host.hWnd as *mut SDL_Window, SDL_FALSE);
    //SDL_SetWindowPosition( host.hWnd, 0, 0 );
    SDL_SetWindowGrab(host.hWnd as *mut SDL_Window, SDL_TRUE);
    SDL_SetWindowSize(host.hWnd as *mut SDL_Window, got.w, got.h);
    VID_SaveWindowSize(got.w, got.h);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn VID_RestoreScreenResolution() {
    if Cvar_VariableInteger(b"fullscreen\x00" as *const u8 as
                                *const libc::c_char) == 0 {
        SDL_SetWindowBordered(host.hWnd as *mut SDL_Window, SDL_TRUE);
        SDL_SetWindowGrab(host.hWnd as *mut SDL_Window, SDL_FALSE);
    } else {
        SDL_MinimizeWindow(host.hWnd as *mut SDL_Window);
        SDL_SetWindowFullscreen(host.hWnd as *mut SDL_Window,
                                0 as libc::c_int as Uint32);
    };
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
// ICO support only for Win32
/*
=================
VID_CreateWindow
=================
*/
#[no_mangle]
pub unsafe extern "C" fn VID_CreateWindow(mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut fullscreen: qboolean)
 -> qboolean {
    static mut wndname: string = [0; 256];
    let mut wndFlags: Uint32 =
        (SDL_WINDOW_OPENGL as libc::c_int | SDL_WINDOW_SHOWN as libc::c_int |
             SDL_WINDOW_MOUSE_FOCUS as libc::c_int) as Uint32;
    let mut icon: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut iconLoaded: qboolean = false_0;
    let mut iconpath: [libc::c_char; 256] = [0; 256];
    let mut xpos: libc::c_int = 0;
    let mut ypos: libc::c_int = 0;
    if (*vid_highdpi).value != 0. {
        wndFlags |= SDL_WINDOW_ALLOW_HIGHDPI as libc::c_int as libc::c_uint
    }
    Q_strncpy(wndname.as_mut_ptr(), (*SI.GameInfo).title.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    if glw_state.software as u64 != 0 {
        wndFlags &= !(SDL_WINDOW_OPENGL as libc::c_int) as libc::c_uint
    }
    if fullscreen as u64 == 0 {
        wndFlags |= SDL_WINDOW_RESIZABLE as libc::c_int as libc::c_uint;
        xpos =
            Cvar_VariableInteger(b"_window_xpos\x00" as *const u8 as
                                     *const libc::c_char);
        ypos =
            Cvar_VariableInteger(b"_window_ypos\x00" as *const u8 as
                                     *const libc::c_char);
        if xpos < 0 as libc::c_int {
            xpos =
                (0x2fff0000 as libc::c_uint |
                     0 as libc::c_int as libc::c_uint) as libc::c_int
        }
        if ypos < 0 as libc::c_int {
            ypos =
                (0x2fff0000 as libc::c_uint |
                     0 as libc::c_int as libc::c_uint) as libc::c_int
        }
    } else {
        wndFlags |=
            (SDL_WINDOW_FULLSCREEN as libc::c_int |
                 SDL_WINDOW_BORDERLESS as libc::c_int |
                 SDL_WINDOW_INPUT_GRABBED as libc::c_int) as libc::c_uint;
        ypos = 0 as libc::c_int;
        xpos = ypos
    }
    while glw_state.safe >= SAFE_NO as libc::c_int &&
              glw_state.safe < SAFE_LAST as libc::c_int {
        host.hWnd =
            SDL_CreateWindow(wndname.as_mut_ptr(), xpos, ypos, width, height,
                             wndFlags) as *mut libc::c_void;
        // re-choose attributes
        // try again create window
        // we have window, exit loop
        if !host.hWnd.is_null() {
            break ; // no need to skip msaa, if we already disabled it
        }
        Con_Reportf(b"^1Error:^7 VID_CreateWindow: couldn\'t create \'%s\' with safegl level %d: %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    wndname.as_mut_ptr(), glw_state.safe, SDL_GetError());
        glw_state.safe += 1;
        if (*gl_msaa_samples).value == 0. &&
               glw_state.safe == SAFE_NOMSAA as libc::c_int {
            glw_state.safe += 1
        }
        GL_SetupAttributes();
    }
    // window creation has failed...
    if glw_state.safe >= SAFE_LAST as libc::c_int { return false_0 }
    if fullscreen as u64 != 0 {
        if VID_SetScreenResolution(width, height) as u64 == 0 {
            return false_0
        }
    } else { VID_RestoreScreenResolution(); }
    // ICO support only for Win32
    // _WIN32 && !XASH_64BIT
    if iconLoaded as u64 == 0 {
        Q_strncpy(iconpath.as_mut_ptr(), (*SI.GameInfo).iconpath.as_mut_ptr(),
                  99999 as libc::c_int as size_t);
        COM_StripExtension(iconpath.as_mut_ptr());
        COM_DefaultExtension(iconpath.as_mut_ptr(),
                             b".tga\x00" as *const u8 as *const libc::c_char);
        icon =
            FS_LoadImage(iconpath.as_mut_ptr(), 0 as *const byte,
                         0 as libc::c_int as size_t);
        if !icon.is_null() {
            let mut surface: *mut SDL_Surface =
                SDL_CreateRGBSurfaceFrom((*icon).buffer as *mut libc::c_void,
                                         (*icon).width as libc::c_int,
                                         (*icon).height as libc::c_int,
                                         32 as libc::c_int,
                                         4 as libc::c_int *
                                             (*icon).width as libc::c_int,
                                         0xff as libc::c_int as Uint32,
                                         0xff00 as libc::c_int as Uint32,
                                         0xff0000 as libc::c_int as Uint32,
                                         0xff000000 as libc::c_uint);
            if !surface.is_null() {
                SDL_SetWindowIcon(host.hWnd as *mut SDL_Window, surface);
                SDL_FreeSurface(surface);
                iconLoaded = true_0
            }
            FS_FreeImage(icon);
        }
    }
    // ICO support only for Win32
    SDL_ShowWindow(host.hWnd as *mut SDL_Window);
    if glw_state.software as u64 != 0 {
        let mut sdl_renderer: libc::c_int = -(2 as libc::c_int);
        let mut cmd: [libc::c_char; 64] = [0; 64];
        if _Sys_GetParmFromCmdLine(b"-sdl_renderer\x00" as *const u8 as
                                       *const libc::c_char, cmd.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong) as u64 != 0 {
            sdl_renderer = Q_atoi(cmd.as_mut_ptr())
        }
        if sdl_renderer >= -(1 as libc::c_int) {
            sw.renderer =
                SDL_CreateRenderer(host.hWnd as *mut SDL_Window, sdl_renderer,
                                   0 as libc::c_int as Uint32);
            if sw.renderer.is_null() {
                Con_Printf(b"^1Error:^7 failed to create SDL renderer: %s\n\x00"
                               as *const u8 as *const libc::c_char,
                           SDL_GetError());
            } else {
                let mut info: SDL_RendererInfo =
                    SDL_RendererInfo{name: 0 as *const libc::c_char,
                                     flags: 0,
                                     num_texture_formats: 0,
                                     texture_formats: [0; 16],
                                     max_texture_width: 0,
                                     max_texture_height: 0,};
                SDL_GetRendererInfo(sw.renderer, &mut info);
                Con_Printf(b"SDL_Renderer %s initialized\n\x00" as *const u8
                               as *const libc::c_char, info.name);
            }
        }
    } else {
        if glw_state.initialized as u64 == 0 {
            if GL_CreateContext() as u64 == 0 { return false_0 }
            VID_StartupGamma();
        }
        if GL_UpdateContext() as u64 == 0 { return false_0 }
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    VID_SaveWindowSize(width, height);
    return true_0;
}
/*
=================
VID_DestroyWindow
=================
*/
#[no_mangle]
pub unsafe extern "C" fn VID_DestroyWindow() {
    GL_DeleteContext();
    VID_RestoreScreenResolution();
    if !host.hWnd.is_null() {
        SDL_DestroyWindow(host.hWnd as *mut SDL_Window);
        // SDL_VERSION_ATLEAST( 2, 0, 0 )
        host.hWnd = 0 as *mut libc::c_void
    }
    if refState.fullScreen as u64 != 0 { refState.fullScreen = false_0 };
}
/*
==================
GL_SetupAttributes
==================
*/
unsafe extern "C" fn GL_SetupAttributes() {
    SDL_GL_ResetAttributes();
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    ref_0.dllFuncs.GL_SetupAttributes.expect("non-null function pointer")(glw_state.safe);
}
#[no_mangle]
pub unsafe extern "C" fn GL_SwapBuffers() {
    SDL_GL_SwapWindow(host.hWnd as *mut SDL_Window);
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
#[no_mangle]
pub unsafe extern "C" fn GL_SetAttribute(mut attr: libc::c_int,
                                         mut val: libc::c_int)
 -> libc::c_int {
    match attr {
        0 => { return SDL_GL_SetAttribute(SDL_GL_RED_SIZE, val) }
        1 => { return SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, val) }
        2 => { return SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, val) }
        3 => { return SDL_GL_SetAttribute(SDL_GL_ALPHA_SIZE, val) }
        4 => { return SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, val) }
        5 => { return SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, val) }
        6 => { return SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, val) }
        7 => { return SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, val) }
        8 => { return SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, val) }
        9 => { return SDL_GL_SetAttribute(SDL_GL_ACCELERATED_VISUAL, val) }
        10 => {
            return SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, val)
        }
        11 => {
            return SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, val)
        }
        12 => { return SDL_GL_SetAttribute(SDL_GL_CONTEXT_EGL, val) }
        13 => { return SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, val) }
        15 => {
            return SDL_GL_SetAttribute(SDL_GL_SHARE_WITH_CURRENT_CONTEXT, val)
        }
        16 => {
            return SDL_GL_SetAttribute(SDL_GL_FRAMEBUFFER_SRGB_CAPABLE, val)
        }
        14 => {
            if val == REF_GL_CONTEXT_PROFILE_ES as libc::c_int {
                SDL_SetHint(b"SDL_OPENGL_ES_DRIVER\x00" as *const u8 as
                                *const libc::c_char,
                            b"1\x00" as *const u8 as *const libc::c_char);
                SDL_SetHint(b"SDL_VIDEO_X11_FORCE_EGL\x00" as *const u8 as
                                *const libc::c_char,
                            b"1\x00" as *const u8 as *const libc::c_char);
            }
            // SDL_HINT_OPENGL_ES_DRIVER
            return SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, val)
        }
        17 => {
            return SDL_GL_SetAttribute(SDL_GL_CONTEXT_RELEASE_BEHAVIOR, val)
        }
        18 => {
            return SDL_GL_SetAttribute(SDL_GL_CONTEXT_RESET_NOTIFICATION, val)
        }
        19 => { return SDL_GL_SetAttribute(SDL_GL_CONTEXT_NO_ERROR, val) }
        _ => { }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GL_GetAttribute(mut attr: libc::c_int,
                                         mut val: *mut libc::c_int)
 -> libc::c_int {
    match attr {
        0 => { return SDL_GL_GetAttribute(SDL_GL_RED_SIZE, val) }
        1 => { return SDL_GL_GetAttribute(SDL_GL_GREEN_SIZE, val) }
        2 => { return SDL_GL_GetAttribute(SDL_GL_BLUE_SIZE, val) }
        3 => { return SDL_GL_GetAttribute(SDL_GL_ALPHA_SIZE, val) }
        4 => { return SDL_GL_GetAttribute(SDL_GL_DOUBLEBUFFER, val) }
        5 => { return SDL_GL_GetAttribute(SDL_GL_DEPTH_SIZE, val) }
        6 => { return SDL_GL_GetAttribute(SDL_GL_STENCIL_SIZE, val) }
        7 => { return SDL_GL_GetAttribute(SDL_GL_MULTISAMPLEBUFFERS, val) }
        8 => { return SDL_GL_GetAttribute(SDL_GL_MULTISAMPLESAMPLES, val) }
        9 => { return SDL_GL_GetAttribute(SDL_GL_ACCELERATED_VISUAL, val) }
        10 => {
            return SDL_GL_GetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, val)
        }
        11 => {
            return SDL_GL_GetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, val)
        }
        12 => { return SDL_GL_GetAttribute(SDL_GL_CONTEXT_EGL, val) }
        13 => { return SDL_GL_GetAttribute(SDL_GL_CONTEXT_FLAGS, val) }
        15 => {
            return SDL_GL_GetAttribute(SDL_GL_SHARE_WITH_CURRENT_CONTEXT, val)
        }
        16 => {
            return SDL_GL_GetAttribute(SDL_GL_FRAMEBUFFER_SRGB_CAPABLE, val)
        }
        14 => { return SDL_GL_GetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, val) }
        17 => {
            return SDL_GL_GetAttribute(SDL_GL_CONTEXT_RELEASE_BEHAVIOR, val)
        }
        18 => {
            return SDL_GL_GetAttribute(SDL_GL_CONTEXT_RESET_NOTIFICATION, val)
        }
        19 => { return SDL_GL_GetAttribute(SDL_GL_CONTEXT_NO_ERROR, val) }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
==================
R_Init_Video
==================
*/
#[no_mangle]
pub unsafe extern "C" fn R_Init_Video(type_0: libc::c_int) -> qboolean {
    let mut safe: string = [0; 256];
    let mut retval: qboolean = false_0;
    let mut displayMode: SDL_DisplayMode =
        SDL_DisplayMode{format: 0,
                        w: 0,
                        h: 0,
                        refresh_rate: 0,
                        driverdata: 0 as *mut libc::c_void,};
    SDL_GetCurrentDisplayMode(0 as libc::c_int, &mut displayMode);
    refState.desktopBitsPixel =
        (displayMode.format >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_int;
    SDL_SetHint(b"SDL_VIDEO_X11_XRANDR\x00" as *const u8 as
                    *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char);
    SDL_SetHint(b"SDL_VIDEO_X11_XVIDMODE\x00" as *const u8 as
                    *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char);
    if Sys_CheckParm(b"-egl\x00" as *const u8 as *const libc::c_char) != 0 {
        SDL_SetHint(b"SDL_VIDEO_X11_FORCE_EGL\x00" as *const u8 as
                        *const libc::c_char,
                    b"1\x00" as *const u8 as *const libc::c_char);
    }
    // must be initialized before creating window
    match type_0 {
        0 => { glw_state.software = true_0 }
        1 => {
            if glw_state.safe == 0 &&
                   _Sys_GetParmFromCmdLine(b"-safegl\x00" as *const u8 as
                                               *const libc::c_char,
                                           safe.as_mut_ptr(),
                                           ::std::mem::size_of::<string>() as
                                               libc::c_ulong) as libc::c_uint
                       != 0 {
                glw_state.safe =
                    if Q_atoi(safe.as_mut_ptr()) >= SAFE_NO as libc::c_int {
                        if Q_atoi(safe.as_mut_ptr()) <
                               SAFE_DONTCARE as libc::c_int {
                            Q_atoi(safe.as_mut_ptr())
                        } else { SAFE_DONTCARE as libc::c_int }
                    } else { SAFE_NO as libc::c_int }
            }
            // refdll can request some attributes
            GL_SetupAttributes();
            if SDL_GL_LoadLibrary(0 as *const libc::c_char) != 0 {
                Con_Reportf(b"^1Error:^7 Couldn\'t initialize OpenGL: %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            SDL_GetError());
                return false_0
            }
        }
        _ => {
            Host_Error(b"Can\'t initialize unknown context type %d!\n\x00" as
                           *const u8 as *const libc::c_char, type_0);
        }
    }
    retval = VID_SetMode();
    if retval as u64 == 0 { return retval }
    match type_0 {
        1 => {
            // refdll also can check extensions
            ref_0.dllFuncs.GL_InitExtensions.expect("non-null function pointer")();
        }
        0 | _ => { }
    }
    R_InitVideoModes();
    host.renderinfo_changed = false_0;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_ChangeDisplaySettings(mut width: libc::c_int,
                                                 mut height: libc::c_int,
                                                 mut fullscreen: qboolean)
 -> rserr_t {
    let mut displayMode: SDL_DisplayMode =
        SDL_DisplayMode{format: 0,
                        w: 0,
                        h: 0,
                        refresh_rate: 0,
                        driverdata: 0 as *mut libc::c_void,};
    SDL_GetCurrentDisplayMode(0 as libc::c_int, &mut displayMode);
    // check our desktop attributes
    refState.desktopBitsPixel =
        (displayMode.format >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_int;
    Con_Reportf(b"R_ChangeDisplaySettings: Setting video mode to %dx%d %s\n\x00"
                    as *const u8 as *const libc::c_char, width, height,
                if fullscreen as libc::c_uint != 0 {
                    b"fullscreen\x00" as *const u8 as *const libc::c_char
                } else {
                    b"windowed\x00" as *const u8 as *const libc::c_char
                });
    refState.fullScreen = fullscreen;
    if host.hWnd.is_null() {
        if VID_CreateWindow(width, height, fullscreen) as u64 == 0 {
            return rserr_invalid_mode
        }
    } else if fullscreen as u64 != 0 {
        if VID_SetScreenResolution(width, height) as u64 == 0 {
            return rserr_invalid_fullscreen
        }
    } else {
        VID_RestoreScreenResolution();
        if SDL_SetWindowFullscreen(host.hWnd as *mut SDL_Window,
                                   0 as libc::c_int as Uint32) != 0 {
            return rserr_invalid_fullscreen
        }
        SDL_SetWindowResizable(host.hWnd as *mut SDL_Window, SDL_TRUE);
        SDL_SetWindowBordered(host.hWnd as *mut SDL_Window, SDL_TRUE);
        SDL_SetWindowSize(host.hWnd as *mut SDL_Window, width, height);
        // SDL_VERSION_ATLEAST( 2, 0, 0 )
        VID_SaveWindowSize(width, height);
    }
    return rserr_ok;
}
/*
==================
VID_SetMode

Set the described video mode
==================
*/
#[no_mangle]
pub unsafe extern "C" fn VID_SetMode() -> qboolean {
    let mut fullscreen: qboolean = false_0;
    let mut iScreenWidth: libc::c_int = 0;
    let mut iScreenHeight: libc::c_int = 0;
    let mut err: rserr_t = rserr_ok;
    iScreenWidth =
        Cvar_VariableInteger(b"width\x00" as *const u8 as
                                 *const libc::c_char);
    iScreenHeight =
        Cvar_VariableInteger(b"height\x00" as *const u8 as
                                 *const libc::c_char);
    if iScreenWidth < 320 as libc::c_int || iScreenHeight < 200 as libc::c_int
       {
        // trying to get resolution automatically by default
        let mut mode: SDL_DisplayMode =
            SDL_DisplayMode{format: 0,
                            w: 0,
                            h: 0,
                            refresh_rate: 0,
                            driverdata: 0 as *mut libc::c_void,};
        SDL_GetDesktopDisplayMode(0 as libc::c_int, &mut mode);
        iScreenWidth = mode.w;
        iScreenHeight = mode.h
        // SDL_VERSION_ATLEAST( 2, 0, 0 )
        // SDL_VERSION_ATLEAST( 2, 0, 0 )
    }
    if (*vid_fullscreen).flags & (1 as libc::c_int) << 13 as libc::c_int == 0
       {
        Cvar_SetValue(b"fullscreen\x00" as *const u8 as *const libc::c_char,
                      1 as libc::c_int as libc::c_float);
    } else {
        (*vid_fullscreen).flags =
            (*vid_fullscreen).flags &
                !((1 as libc::c_int) << 13 as libc::c_int)
    }
    (*gl_vsync).flags =
        (*gl_vsync).flags | (1 as libc::c_int) << 13 as libc::c_int;
    fullscreen =
        (Cvar_VariableInteger(b"fullscreen\x00" as *const u8 as
                                  *const libc::c_char) != 0 as libc::c_int) as
            libc::c_int as qboolean;
    err = R_ChangeDisplaySettings(iScreenWidth, iScreenHeight, fullscreen);
    if err as libc::c_uint == rserr_ok as libc::c_int as libc::c_uint {
        sdlState.prev_width = iScreenWidth;
        sdlState.prev_height = iScreenHeight
    } else {
        if err as libc::c_uint ==
               rserr_invalid_fullscreen as libc::c_int as libc::c_uint {
            Cvar_SetValue(b"fullscreen\x00" as *const u8 as
                              *const libc::c_char,
                          0 as libc::c_int as libc::c_float);
            Con_Reportf(b"^1Error:^7 VID_SetMode: fullscreen unavailable in this mode\n\x00"
                            as *const u8 as *const libc::c_char);
            Sys_Warn(b"fullscreen unavailable in this mode!\x00" as *const u8
                         as *const libc::c_char);
            err =
                R_ChangeDisplaySettings(iScreenWidth, iScreenHeight, false_0);
            if err as libc::c_uint == rserr_ok as libc::c_int as libc::c_uint
               {
                return true_0
            }
        } else if err as libc::c_uint ==
                      rserr_invalid_mode as libc::c_int as libc::c_uint {
            Con_Reportf(b"^1Error:^7 VID_SetMode: invalid mode\n\x00" as
                            *const u8 as *const libc::c_char);
            Sys_Warn(b"invalid mode, engine will run in %dx%d\x00" as
                         *const u8 as *const libc::c_char,
                     sdlState.prev_width, sdlState.prev_height);
        }
        // try setting it back to something safe
        err =
            R_ChangeDisplaySettings(sdlState.prev_width, sdlState.prev_height,
                                    false_0);
        if err as libc::c_uint != rserr_ok as libc::c_int as libc::c_uint {
            Con_Reportf(b"^1Error:^7 VID_SetMode: could not revert to safe mode\n\x00"
                            as *const u8 as *const libc::c_char);
            Sys_Warn(b"could not revert to safe mode!\x00" as *const u8 as
                         *const libc::c_char);
            return false_0
        }
    }
    return true_0;
}
/*
==================
R_Free_Video
==================
*/
#[no_mangle]
pub unsafe extern "C" fn R_Free_Video() {
    GL_DeleteContext();
    VID_DestroyWindow();
    R_FreeVideoModes();
    ref_0.dllFuncs.GL_ClearExtensions.expect("non-null function pointer")();
    SDL_VideoQuit();
}
// XASH_DEDICATED
