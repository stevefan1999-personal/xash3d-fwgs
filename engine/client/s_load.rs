#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn COM_HashKey(string: *const libc::c_char, hashSize: uint) -> uint;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn FS_LoadSound(filename: *const libc::c_char, buffer: *const byte,
                    size: size_t) -> *mut wavdata_t;
    #[no_mangle]
    fn FS_FreeSound(pack: *mut wavdata_t);
    #[no_mangle]
    fn Sound_Process(wav: *mut *mut wavdata_t, rate: libc::c_int,
                     width: libc::c_int, flags: uint) -> qboolean;
    #[no_mangle]
    fn CL_Active() -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn S_StopAllSounds(ambient: qboolean);
    #[no_mangle]
    static mut ambient_sfx: [sound_t; 4];
    #[no_mangle]
    static mut snd_ambient: qboolean;
    #[no_mangle]
    static mut dma: dma_t;
    #[no_mangle]
    static mut s_warn_late_precache: convar_t;
    #[no_mangle]
    static mut sndpool: poolhandle_t;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type sound_t = libc::c_int;
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string = [libc::c_char; 256];
pub type word = libc::c_ushort;
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
pub type C2RustUnnamed = libc::c_uint;
pub const SOUND_CONVERT16BIT: C2RustUnnamed = 8192;
pub const SOUND_RESAMPLE: C2RustUnnamed = 4096;
pub const SOUND_STREAM: C2RustUnnamed = 2;
pub const SOUND_LOOPED: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavdata_t {
    pub rate: word,
    pub width: byte,
    pub channels: byte,
    pub loopStart: libc::c_int,
    pub samples: libc::c_int,
    pub type_0: uint,
    pub flags: uint,
    pub buffer: *mut byte,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub cache: *mut wavdata_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub hashNext: *mut sfx_s,
}
pub type sfx_t = sfx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_format_s {
    pub speed: libc::c_uint,
    pub width: libc::c_uint,
    pub channels: libc::c_uint,
}
pub type snd_format_t = snd_format_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub format: snd_format_t,
    pub samples: libc::c_int,
    pub samplepos: libc::c_int,
    pub buffer: *mut byte,
    pub initialized: qboolean,
}
static mut s_numSfx: libc::c_int = 0 as libc::c_int;
static mut s_knownSfx: [sfx_t; 8192] =
    [sfx_t{name: [0; 64],
           cache: 0 as *const wavdata_t as *mut wavdata_t,
           servercount: 0,
           hashValue: 0,
           hashNext: 0 as *const sfx_s as *mut sfx_s,}; 8192];
static mut s_sfxHashList: [*mut sfx_t; 2048] =
    [0 as *const sfx_t as *mut sfx_t; 2048];
static mut s_sentenceImmediateName: string = [0; 256];
// keep dummy sentence name
#[no_mangle]
pub static mut s_registering: qboolean = false_0;
#[no_mangle]
pub static mut s_registration_sequence: libc::c_int = 0 as libc::c_int;
/*
=================
S_SoundList_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SoundList_f() {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut sc: *mut wavdata_t = 0 as *mut wavdata_t;
    let mut i: libc::c_int = 0;
    let mut totalSfx: libc::c_int = 0 as libc::c_int;
    let mut totalSize: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    sfx = s_knownSfx.as_mut_ptr();
    while i < s_numSfx {
        if !((*sfx).name[0 as libc::c_int as usize] == 0) {
            sc = (*sfx).cache;
            if !sc.is_null() {
                totalSize =
                    (totalSize as libc::c_ulong).wrapping_add((*sc).size) as
                        libc::c_int as libc::c_int;
                if (*sc).loopStart >= 0 as libc::c_int {
                    Con_Printf(b"L\x00" as *const u8 as *const libc::c_char);
                } else {
                    Con_Printf(b" \x00" as *const u8 as *const libc::c_char);
                }
                if (*sfx).name[0 as libc::c_int as usize] as libc::c_int ==
                       '*' as i32 {
                    Con_Printf(b" (%2db) %s : %s\n\x00" as *const u8 as
                                   *const libc::c_char,
                               (*sc).width as libc::c_int * 8 as libc::c_int,
                               Q_pretifymem((*sc).size as libc::c_float,
                                            2 as libc::c_int),
                               (*sfx).name.as_mut_ptr());
                } else {
                    Con_Printf(b" (%2db) %s : sound/%s\n\x00" as *const u8 as
                                   *const libc::c_char,
                               (*sc).width as libc::c_int * 8 as libc::c_int,
                               Q_pretifymem((*sc).size as libc::c_float,
                                            2 as libc::c_int),
                               (*sfx).name.as_mut_ptr());
                }
                totalSfx += 1
            }
        }
        i += 1;
        sfx = sfx.offset(1)
    }
    Con_Printf(b"-------------------------------------------\n\x00" as
                   *const u8 as *const libc::c_char);
    Con_Printf(b"%i total sounds\n\x00" as *const u8 as *const libc::c_char,
               totalSfx);
    Con_Printf(b"%s total memory\n\x00" as *const u8 as *const libc::c_char,
               Q_pretifymem(totalSize as libc::c_float, 2 as libc::c_int));
    Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
// return true if char 'c' is one of 1st 2 characters in pch
#[no_mangle]
pub unsafe extern "C" fn S_TestSoundChar(mut pch: *const libc::c_char,
                                         mut c: libc::c_char) -> qboolean {
    let mut pcht: *mut libc::c_char = pch as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if pch.is_null() || *pch == 0 { return false_0 }
    // check first 2 characters
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if *pcht as libc::c_int == c as libc::c_int { return true_0 }
        pcht = pcht.offset(1);
        i += 1
    }
    return false_0;
}
// return pointer to first valid character in file name
#[no_mangle]
pub unsafe extern "C" fn S_SkipSoundChar(mut pch: *const libc::c_char)
 -> *mut libc::c_char {
    let mut pcht: *mut libc::c_char = pch as *mut libc::c_char;
    // check first character
    if *pcht as libc::c_int == '!' as i32 { pcht = pcht.offset(1) }
    return pcht;
}
/*
=================
S_CreateDefaultSound
=================
*/
unsafe extern "C" fn S_CreateDefaultSound() -> *mut wavdata_t {
    let mut sc: *mut wavdata_t = 0 as *mut wavdata_t;
    sc =
        _Mem_Alloc(sndpool,
                   ::std::mem::size_of::<wavdata_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/client/s_load.c\x00" as *const u8 as
                       *const libc::c_char, 109 as libc::c_int) as
            *mut wavdata_t;
    (*sc).width = 2 as libc::c_int as byte;
    (*sc).channels = 1 as libc::c_int as byte;
    (*sc).loopStart = -(1 as libc::c_int);
    (*sc).rate = 44100 as libc::c_int as word;
    (*sc).samples = 44100 as libc::c_int;
    (*sc).size =
        ((*sc).samples * (*sc).width as libc::c_int *
             (*sc).channels as libc::c_int) as size_t;
    (*sc).buffer =
        _Mem_Alloc(sndpool, (*sc).size, true_0,
                   b"../engine/client/s_load.c\x00" as *const u8 as
                       *const libc::c_char, 117 as libc::c_int) as *mut byte;
    return sc;
}
/*
=================
S_LoadSound
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_LoadSound(mut sfx: *mut sfx_t) -> *mut wavdata_t {
    let mut sc: *mut wavdata_t = 0 as *mut wavdata_t;
    if sfx.is_null() { return 0 as *mut wavdata_t }
    // see if still in memory
    if !(*sfx).cache.is_null() { return (*sfx).cache }
    if if (*sfx).name.as_mut_ptr().is_null() || *(*sfx).name.as_mut_ptr() == 0
          {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as *mut wavdata_t
    }
    // load it from disk
    if Q_strnicmp((*sfx).name.as_mut_ptr(),
                  b"*default\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        // load it from disk
        if s_warn_late_precache.value > 0 as libc::c_int as libc::c_float &&
               CL_Active() != 0 {
            Con_Printf(b"^3Warning:^7 S_LoadSound: late precache of %s\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*sfx).name.as_mut_ptr());
        }
        if (*sfx).name[0 as libc::c_int as usize] as libc::c_int == '*' as i32
           {
            sc =
                FS_LoadSound((*sfx).name.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize),
                             0 as *const byte, 0 as libc::c_int as size_t)
        } else {
            sc =
                FS_LoadSound((*sfx).name.as_mut_ptr(), 0 as *const byte,
                             0 as libc::c_int as size_t)
        }
    }
    if sc.is_null() { sc = S_CreateDefaultSound() }
    if ((*sc).rate as libc::c_int) < 11025 as libc::c_int {
        // some bad sounds
        Sound_Process(&mut sc, 11025 as libc::c_int,
                      (*sc).width as libc::c_int,
                      SOUND_RESAMPLE as libc::c_int as uint);
    } else if (*sc).rate as libc::c_int > 11025 as libc::c_int &&
                  ((*sc).rate as libc::c_int) < 22050 as libc::c_int {
        // some bad sounds
        Sound_Process(&mut sc, 22050 as libc::c_int,
                      (*sc).width as libc::c_int,
                      SOUND_RESAMPLE as libc::c_int as uint);
    } else if (*sc).rate as libc::c_int > 22050 as libc::c_int &&
                  (*sc).rate as libc::c_int <= 32000 as libc::c_int {
        // some bad sounds
        Sound_Process(&mut sc, 44100 as libc::c_int,
                      (*sc).width as libc::c_int,
                      SOUND_RESAMPLE as libc::c_int as uint);
    }
    (*sfx).cache = sc;
    return (*sfx).cache;
}
// =======================================================================
// Load a sound
// =======================================================================
/*
==================
S_FindName

==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FindName(mut pname: *const libc::c_char,
                                    mut pfInCache: *mut libc::c_int)
 -> *mut sfx_t {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut i: uint = 0;
    let mut hash: uint = 0;
    let mut name: string = [0; 256];
    if (if pname.is_null() || *pname == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 || dma.initialized as u64 == 0 {
        return 0 as *mut sfx_t
    }
    if Q_strlen(pname) >=
           ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong {
        return 0 as *mut sfx_t
    }
    Q_strncpy(name.as_mut_ptr(), pname,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    // see if already loaded
    hash =
        COM_HashKey(name.as_mut_ptr(),
                    (8192 as libc::c_int / 4 as libc::c_int) as uint);
    sfx = s_sfxHashList[hash as usize];
    while !sfx.is_null() {
        if Q_strncmp((*sfx).name.as_mut_ptr(), name.as_mut_ptr(),
                     99999 as libc::c_int) == 0 {
            if !pfInCache.is_null() {
                // indicate whether or not sound is currently in the cache.
                *pfInCache =
                    if !(*sfx).cache.is_null() {
                        true_0 as libc::c_int
                    } else { false_0 as libc::c_int }
            }
            // prolonge registration
            (*sfx).servercount = s_registration_sequence;
            return sfx
        }
        sfx = (*sfx).hashNext
    }
    // find a free sfx slot spot
    i = 0 as libc::c_int as uint; // free spot
    sfx = s_knownSfx.as_mut_ptr();
    while i < s_numSfx as libc::c_uint {
        if (*sfx).name[0 as libc::c_int as usize] == 0 { break ; }
        i = i.wrapping_add(1);
        sfx = sfx.offset(1)
    }
    if i == s_numSfx as libc::c_uint {
        if s_numSfx == 8192 as libc::c_int { return 0 as *mut sfx_t }
        s_numSfx += 1
    }
    sfx = &mut *s_knownSfx.as_mut_ptr().offset(i as isize) as *mut sfx_t;
    memset(sfx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sfx_t>() as libc::c_ulong);
    if !pfInCache.is_null() { *pfInCache = false_0 as libc::c_int }
    Q_strncpy((*sfx).name.as_mut_ptr(), name.as_mut_ptr(),
              256 as libc::c_int as size_t);
    (*sfx).servercount = s_registration_sequence;
    (*sfx).hashValue =
        COM_HashKey((*sfx).name.as_mut_ptr(),
                    (8192 as libc::c_int / 4 as libc::c_int) as uint);
    // link it in
    (*sfx).hashNext = s_sfxHashList[(*sfx).hashValue as usize];
    s_sfxHashList[(*sfx).hashValue as usize] = sfx;
    return sfx;
}
/*
==================
S_FreeSound
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FreeSound(mut sfx: *mut sfx_t) {
    let mut hashSfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut prev: *mut *mut sfx_t = 0 as *mut *mut sfx_t;
    if sfx.is_null() || (*sfx).name[0 as libc::c_int as usize] == 0 { return }
    // de-link it from the hash tree
    prev =
        &mut *s_sfxHashList.as_mut_ptr().offset((*sfx).hashValue as isize) as
            *mut *mut sfx_t;
    loop  {
        hashSfx = *prev;
        if hashSfx.is_null() { break ; }
        if hashSfx == sfx {
            *prev = (*hashSfx).hashNext;
            break ;
        } else { prev = &mut (*hashSfx).hashNext }
    }
    if !(*sfx).cache.is_null() { FS_FreeSound((*sfx).cache); }
    memset(sfx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sfx_t>() as libc::c_ulong);
}
/*
=====================
S_BeginRegistration

=====================
*/
#[no_mangle]
pub unsafe extern "C" fn S_BeginRegistration() {
    let mut i: libc::c_int = 0;
    s_registration_sequence += 1;
    snd_ambient = false_0;
    // check for automatic ambient sounds
    i = 0 as libc::c_int; // empty slot
    while i < 4 as libc::c_int {
        if !((*SI.GameInfo).ambientsound[i as
                                             usize][0 as libc::c_int as usize]
                 == 0) {
            ambient_sfx[i as usize] =
                S_RegisterSound((*SI.GameInfo).ambientsound[i as
                                                                usize].as_mut_ptr());
            if ambient_sfx[i as usize] != 0 { snd_ambient = true_0 }
        }
        i += 1
        // allow auto-ambients
    }
    s_registering = true_0;
}
/*
=====================
S_EndRegistration

=====================
*/
#[no_mangle]
pub unsafe extern "C" fn S_EndRegistration() {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut i: libc::c_int = 0;
    if s_registering as u64 == 0 || dma.initialized as u64 == 0 { return }
    // free any sounds not from this registration sequence
    i = 0 as libc::c_int; // don't release default sound
    sfx = s_knownSfx.as_mut_ptr();
    while i < s_numSfx {
        if !((*sfx).name[0 as libc::c_int as usize] == 0 ||
                 Q_strnicmp((*sfx).name.as_mut_ptr(),
                            b"*default\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                     0) {
            if (*sfx).servercount != s_registration_sequence {
                S_FreeSound(sfx);
            }
        }
        i += 1;
        sfx = sfx.offset(1)
        // don't need this sound
    }
    // load everything in
    i = 0 as libc::c_int;
    sfx = s_knownSfx.as_mut_ptr();
    while i < s_numSfx {
        if !((*sfx).name[0 as libc::c_int as usize] == 0) {
            S_LoadSound(sfx);
        }
        i += 1;
        sfx = sfx.offset(1)
    }
    s_registering = false_0;
}
/*
==================
S_RegisterSound

==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_RegisterSound(mut name: *const libc::c_char)
 -> sound_t {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if (if name.is_null() || *name == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 || dma.initialized as u64 == 0 {
        return -(1 as libc::c_int)
    }
    if S_TestSoundChar(name, '!' as i32 as libc::c_char) as u64 != 0 {
        Q_strncpy(s_sentenceImmediateName.as_mut_ptr(), name,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
        return -(99999 as libc::c_int)
    }
    // some stupid mappers used leading '/' or '\' in path to models or sounds
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 ||
           *name.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\\' as i32 {
        name = name.offset(1)
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 ||
           *name.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\\' as i32 {
        name = name.offset(1)
    }
    sfx = S_FindName(name, 0 as *mut libc::c_int);
    if sfx.is_null() { return -(1 as libc::c_int) }
    (*sfx).servercount = s_registration_sequence;
    if s_registering as u64 == 0 { S_LoadSound(sfx); }
    return sfx.wrapping_offset_from(s_knownSfx.as_mut_ptr()) as libc::c_long
               as sound_t;
}
#[no_mangle]
pub unsafe extern "C" fn S_GetSfxByHandle(mut handle: sound_t) -> *mut sfx_t {
    if dma.initialized as u64 == 0 { return 0 as *mut sfx_t }
    // create new sfx
    if handle == -(99999 as libc::c_int) {
        return S_FindName(s_sentenceImmediateName.as_mut_ptr(),
                          0 as *mut libc::c_int)
    }
    if handle < 0 as libc::c_int || handle >= s_numSfx {
        return 0 as *mut sfx_t
    }
    return &mut *s_knownSfx.as_mut_ptr().offset(handle as isize) as
               *mut sfx_t;
}
/*
=================
S_InitSounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_InitSounds() {
    // create unused 0-entry
    Q_strncpy((*s_knownSfx.as_mut_ptr()).name.as_mut_ptr(),
              b"*default\x00" as *const u8 as *const libc::c_char,
              64 as libc::c_int as size_t);
    (*s_knownSfx.as_mut_ptr()).hashValue =
        COM_HashKey((*s_knownSfx.as_mut_ptr()).name.as_mut_ptr(),
                    (8192 as libc::c_int / 4 as libc::c_int) as uint);
    let ref mut fresh0 = (*s_knownSfx.as_mut_ptr()).hashNext;
    *fresh0 = s_sfxHashList[(*s_knownSfx.as_mut_ptr()).hashValue as usize];
    s_sfxHashList[(*s_knownSfx.as_mut_ptr()).hashValue as usize] =
        s_knownSfx.as_mut_ptr();
    let ref mut fresh1 = (*s_knownSfx.as_mut_ptr()).cache;
    *fresh1 = S_CreateDefaultSound();
    s_numSfx = 1 as libc::c_int;
}
/*
=================
S_FreeSounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FreeSounds() {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut i: libc::c_int = 0;
    if dma.initialized as u64 == 0 { return }
    // stop all sounds
    S_StopAllSounds(true_0);
    // free all sounds
    i = 0 as libc::c_int;
    sfx = s_knownSfx.as_mut_ptr();
    while i < s_numSfx { S_FreeSound(sfx); i += 1; sfx = sfx.offset(1) }
    memset(s_knownSfx.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[sfx_t; 8192]>() as libc::c_ulong);
    memset(s_sfxHashList.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut sfx_t; 2048]>() as libc::c_ulong);
    s_numSfx = 0 as libc::c_int;
}
