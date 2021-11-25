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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn SDL_setenv(name: *const libc::c_char, value: *const libc::c_char,
                  overwrite: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_GetCurrentAudioDriver() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_OpenAudioDevice(device: *const libc::c_char,
                           iscapture: libc::c_int,
                           desired: *const SDL_AudioSpec,
                           obtained: *mut SDL_AudioSpec,
                           allowed_changes: libc::c_int) -> SDL_AudioDeviceID;
    #[no_mangle]
    fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: libc::c_int);
    #[no_mangle]
    fn SDL_LockAudio();
    #[no_mangle]
    fn SDL_UnlockAudio();
    #[no_mangle]
    fn SDL_CloseAudioDevice(dev: SDL_AudioDeviceID);
    #[no_mangle]
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    #[no_mangle]
    fn SDL_QuitSubSystem(flags: Uint32);
    #[no_mangle]
    fn SDL_WasInit(flags: Uint32) -> Uint32;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut dma: dma_t;
    #[no_mangle]
    static mut s_samplecount: convar_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Uint8 = uint8_t;
pub type Uint16 = uint16_t;
pub type Uint32 = uint32_t;
pub type SDL_AudioFormat = Uint16;
pub type SDL_AudioCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut Uint8,
                                _: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioSpec {
    pub freq: libc::c_int,
    pub format: SDL_AudioFormat,
    pub channels: Uint8,
    pub silence: Uint8,
    pub samples: Uint16,
    pub padding: Uint16,
    pub size: Uint32,
    pub callback: SDL_AudioCallback,
    pub userdata: *mut libc::c_void,
}
pub type SDL_AudioDeviceID = Uint32;
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
pub struct dma_t {
    pub format: snd_format_t,
    pub samples: libc::c_int,
    pub samplepos: libc::c_int,
    pub buffer: *mut byte,
    pub initialized: qboolean,
}
pub type snd_format_t = snd_format_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_format_s {
    pub speed: libc::c_uint,
    pub width: libc::c_uint,
    pub channels: libc::c_uint,
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
/*
=======================================================================
Global variables. Must be visible to window-procedure function
so it can unlock and free the data block after it has been played.
=======================================================================
*/
static mut sdl_dev: libc::c_int = 0;
//static qboolean	snd_firsttime = true;
//static qboolean	primary_format_set;
#[no_mangle]
pub unsafe extern "C" fn SDL_SoundCallback(mut userdata: *mut libc::c_void,
                                           mut stream: *mut Uint8,
                                           mut len: libc::c_int) {
    let mut size: libc::c_int = dma.samples << 1 as libc::c_int;
    let mut pos: libc::c_int = dma.samplepos << 1 as libc::c_int;
    let mut wrapped: libc::c_int = pos + len - size;
    if wrapped < 0 as libc::c_int {
        memcpy(stream as *mut libc::c_void,
               dma.buffer.offset(pos as isize) as *const libc::c_void,
               len as libc::c_ulong);
        dma.samplepos += len >> 1 as libc::c_int
    } else {
        let mut remaining: libc::c_int = size - pos;
        memcpy(stream as *mut libc::c_void,
               dma.buffer.offset(pos as isize) as *const libc::c_void,
               remaining as libc::c_ulong);
        memcpy(stream.offset(remaining as isize) as *mut libc::c_void,
               dma.buffer as *const libc::c_void, wrapped as libc::c_ulong);
        dma.samplepos = wrapped >> 1 as libc::c_int
    };
}
/*
==================
SNDDMA_Init

Try to find a sound device to mix for.
Returns false if nothing is found.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Init() -> qboolean {
    let mut desired: SDL_AudioSpec =
        SDL_AudioSpec{freq: 0,
                      format: 0,
                      channels: 0,
                      silence: 0,
                      samples: 0,
                      padding: 0,
                      size: 0,
                      callback: None,
                      userdata: 0 as *mut libc::c_void,};
    let mut obtained: SDL_AudioSpec =
        SDL_AudioSpec{freq: 0,
                      format: 0,
                      channels: 0,
                      silence: 0,
                      samples: 0,
                      padding: 0,
                      size: 0,
                      callback: None,
                      userdata: 0 as *mut libc::c_void,};
    let mut samplecount: libc::c_int = 0;
    if SDL_Init(0x10 as libc::c_uint) != 0 {
        Con_Reportf(b"^1Error:^7 Audio: SDL: %s \n\x00" as *const u8 as
                        *const libc::c_char, SDL_GetError());
        return false_0
    }
    // even if we don't have PA
	// we still can safely set env variables
    SDL_setenv(b"PULSE_PROP_application.name\x00" as *const u8 as
                   *const libc::c_char, (*SI.GameInfo).title.as_mut_ptr(),
               1 as libc::c_int);
    SDL_setenv(b"PULSE_PROP_media.role\x00" as *const u8 as
                   *const libc::c_char,
               b"game\x00" as *const u8 as *const libc::c_char,
               1 as libc::c_int);
    memset(&mut desired as *mut SDL_AudioSpec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<SDL_AudioSpec>() as libc::c_ulong);
    desired.freq = 44100 as libc::c_int;
    desired.format = 0x8010 as libc::c_int as SDL_AudioFormat;
    desired.samples = 1024 as libc::c_int as Uint16;
    desired.channels = 2 as libc::c_int as Uint8;
    desired.callback =
        Some(SDL_SoundCallback as
                 unsafe extern "C" fn(_: *mut libc::c_void, _: *mut Uint8,
                                      _: libc::c_int) -> ());
    sdl_dev =
        SDL_OpenAudioDevice(0 as *const libc::c_char, 0 as libc::c_int,
                            &mut desired, &mut obtained, 0 as libc::c_int) as
            libc::c_int;
    if sdl_dev == 0 as libc::c_int {
        Con_Printf(b"Couldn\'t open SDL audio: %s\n\x00" as *const u8 as
                       *const libc::c_char, SDL_GetError());
        return false_0
    }
    if obtained.format as libc::c_int != 0x8010 as libc::c_int {
        Con_Printf(b"SDL audio format %d unsupported.\n\x00" as *const u8 as
                       *const libc::c_char, obtained.format as libc::c_int);
    } else if obtained.channels as libc::c_int != 1 as libc::c_int &&
                  obtained.channels as libc::c_int != 2 as libc::c_int {
        Con_Printf(b"SDL audio channels %d unsupported.\n\x00" as *const u8 as
                       *const libc::c_char, obtained.channels as libc::c_int);
    } else {
        dma.format.speed = obtained.freq as libc::c_uint;
        dma.format.channels = obtained.channels as libc::c_uint;
        dma.format.width = 2 as libc::c_int as libc::c_uint;
        samplecount = s_samplecount.value as libc::c_int;
        if samplecount == 0 { samplecount = 0x8000 as libc::c_int }
        dma.samples = samplecount * obtained.channels as libc::c_int;
        dma.buffer =
            _Mem_Alloc(host.mempool,
                       (dma.samples * 2 as libc::c_int) as size_t, true_0,
                       b"../engine/platform/sdl/s_sdl.c\x00" as *const u8 as
                           *const libc::c_char, 133 as libc::c_int) as
                *mut byte;
        dma.samplepos = 0 as libc::c_int;
        Con_Printf(b"Using SDL audio driver: %s @ %d Hz\n\x00" as *const u8 as
                       *const libc::c_char, SDL_GetCurrentAudioDriver(),
                   obtained.freq);
        dma.initialized = true_0;
        SNDDMA_Activate(true_0);
        return true_0
    }
    SNDDMA_Shutdown();
    return false_0;
}
/*
==============
SNDDMA_BeginPainting

Makes sure dma.buffer is valid
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_BeginPainting() { SDL_LockAudio(); }
/*
==============
SNDDMA_Submit

Send sound to device if buffer isn't really the dma buffer
Also unlocks the dsound buffer
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Submit() { SDL_UnlockAudio(); }
/*
==============
SNDDMA_Shutdown

Reset the sound device for exiting
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Shutdown() {
    Con_Printf(b"Shutting down audio.\n\x00" as *const u8 as
                   *const libc::c_char);
    dma.initialized = false_0;
    if sdl_dev != 0 {
        SNDDMA_Activate(false_0);
        SDL_CloseAudioDevice(sdl_dev as SDL_AudioDeviceID);
    }
    if SDL_WasInit(0x10 as libc::c_uint) != 0 {
        SDL_QuitSubSystem(0x10 as libc::c_uint);
    }
    if !dma.buffer.is_null() {
        _Mem_Free(dma.buffer as *mut libc::c_void,
                  b"../engine/platform/sdl/s_sdl.c\x00" as *const u8 as
                      *const libc::c_char, 203 as libc::c_int);
        dma.buffer = 0 as *mut byte
    };
}
/*
===========
SNDDMA_Activate
Called when the main window gains or loses focus.
The window have been destroyed and recreated
between a deactivate and an activate.
===========
*/
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Activate(mut active: qboolean) {
    if dma.initialized as u64 == 0 { return }
    SDL_PauseAudioDevice(sdl_dev as SDL_AudioDeviceID,
                         (active as u64 == 0) as libc::c_int);
}
// XASH_SOUND == SOUND_SDL
