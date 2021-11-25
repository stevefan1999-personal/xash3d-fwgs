#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FS_FreeSound(pack: *mut wavdata_t);
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
    fn Q_isdigit(str: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn S_FindName(name: *const libc::c_char, pfInCache: *mut libc::c_int)
     -> *mut sfx_t;
    #[no_mangle]
    fn S_MixDataToDevice(pChannel: *mut channel_t, sampleCount: libc::c_int,
                         outputRate: libc::c_int, outputOffset: libc::c_int,
                         timeCompress: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn S_SetSampleStart(pChan: *mut channel_t, pSource: *mut wavdata_t,
                        newPosition: libc::c_int);
    #[no_mangle]
    fn S_SetSampleEnd(pChan: *mut channel_t, pSource: *mut wavdata_t,
                      newEndPosition: libc::c_int);
    #[no_mangle]
    fn S_LoadSound(sfx: *mut sfx_t) -> *mut wavdata_t;
}
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type fs_offset_t = off_t;
pub type word = libc::c_ushort;
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
pub struct voxword_s {
    pub volume: libc::c_int,
    pub pitch: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub cbtrim: libc::c_int,
    pub fKeepCached: libc::c_int,
    pub samplefrac: libc::c_int,
    pub timecompress: libc::c_int,
    pub sfx: *mut sfx_t,
}
pub type voxword_t = voxword_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentence_t {
    pub pName: *mut libc::c_char,
    pub length: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_s {
    pub name: [libc::c_char; 16],
    pub sfx: *mut sfx_t,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub origin: vec3_t,
    pub dist_mult: libc::c_float,
    pub master_vol: libc::c_int,
    pub isSentence: qboolean,
    pub basePitch: libc::c_int,
    pub pitch: libc::c_float,
    pub use_loop: qboolean,
    pub staticsound: qboolean,
    pub localsound: qboolean,
    pub pMixer: mixer_t,
    pub wordIndex: libc::c_int,
    pub currentWord: *mut mixer_t,
    pub words: [voxword_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixer_t {
    pub sample: libc::c_double,
    pub pData: *mut wavdata_t,
    pub forcedEndSample: libc::c_double,
    pub finished: qboolean,
}
pub type channel_t = channel_s;
/*
s_vox.c - npc sentences
Copyright (C) 2010 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
#[no_mangle]
pub static mut g_Sentences: [sentence_t; 4096] =
    [sentence_t{pName: 0 as *const libc::c_char as *mut libc::c_char,
                length: 0.,}; 4096];
static mut g_numSentences: uint = 0;
static mut rgpparseword: [*mut libc::c_char; 64] =
    [0 as *const libc::c_char as *mut libc::c_char; 64];
// array of pointers to parsed words
static mut voxperiod: [libc::c_char; 8] =
    unsafe {
        *::std::mem::transmute::<&[u8; 8],
                                 &mut [libc::c_char; 8]>(b"_period\x00")
    };
// vocal pause
static mut voxcomma: [libc::c_char; 7] =
    unsafe {
        *::std::mem::transmute::<&[u8; 7],
                                 &mut [libc::c_char; 7]>(b"_comma\x00")
    };
// vocal pause
unsafe extern "C" fn IsNextWord(c: libc::c_char) -> libc::c_int {
    if c as libc::c_int == '.' as i32 || c as libc::c_int == ',' as i32 ||
           c as libc::c_int == ' ' as i32 || c as libc::c_int == '(' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn IsSkipSpace(c: libc::c_char) -> libc::c_int {
    if c as libc::c_int == ',' as i32 || c as libc::c_int == '.' as i32 ||
           c as libc::c_int == ' ' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn IsWhiteSpace(space: libc::c_char) -> libc::c_int {
    if space as libc::c_int == ' ' as i32 ||
           space as libc::c_int == '\t' as i32 ||
           space as libc::c_int == '\r' as i32 ||
           space as libc::c_int == '\n' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn IsCommandChar(c: libc::c_char) -> libc::c_int {
    if c as libc::c_int == 'v' as i32 || c as libc::c_int == 'p' as i32 ||
           c as libc::c_int == 's' as i32 || c as libc::c_int == 'e' as i32 ||
           c as libc::c_int == 't' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn IsDelimitChar(c: libc::c_char) -> libc::c_int {
    if c as libc::c_int == '(' as i32 || c as libc::c_int == ')' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ScanForwardUntil(mut string: *mut libc::c_char,
                                      scan: libc::c_char)
 -> *mut libc::c_char {
    while *string.offset(0 as libc::c_int as isize) != 0 {
        if *string.offset(0 as libc::c_int as isize) as libc::c_int ==
               scan as libc::c_int {
            return string
        }
        string = string.offset(1)
    }
    return string;
}
// backwards scan psz for last '/'
// return substring in szpath null terminated
// if '/' not found, return 'vox/'
unsafe extern "C" fn VOX_GetDirectory(mut szpath: *mut libc::c_char,
                                      mut psz: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut cb: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    len = Q_strlen(psz) as libc::c_int;
    p = psz.offset(len as isize).offset(-(1 as libc::c_int as isize));
    // scan backwards until first '/' or start of string
    c = *p;
    while p > psz && c as libc::c_int != '/' as i32 {
        p = p.offset(-1);
        c = *p;
        cb += 1
    }
    if c as libc::c_int != '/' as i32 {
        // didn't find '/', return default directory
        Q_strncpy(szpath, b"vox/\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
        return psz
    }
    cb = len - cb;
    memcpy(szpath as *mut libc::c_void, psz as *const libc::c_void,
           cb as libc::c_ulong);
    *szpath.offset(cb as isize) = 0 as libc::c_int as libc::c_char;
    return p.offset(1 as libc::c_int as isize);
}
// scan g_Sentences, looking for pszin sentence name
// return pointer to sentence data if found, null if not
// CONSIDER: if we have a large number of sentences, should
// CONSIDER: sort strings in g_Sentences and do binary search.
#[no_mangle]
pub unsafe extern "C" fn VOX_LookupString(mut pSentenceName:
                                              *const libc::c_char,
                                          mut psentencenum: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if Q_isdigit(pSentenceName) as libc::c_uint != 0 &&
           {
               i = Q_atoi(pSentenceName);
               ((i as libc::c_uint)) < g_numSentences
           } {
        if !psentencenum.is_null() { *psentencenum = i }
        return g_Sentences[i as
                               usize].pName.offset(Q_strlen(g_Sentences[i as
                                                                            usize].pName)
                                                       as
                                                       isize).offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < g_numSentences {
        if Q_strnicmp(pSentenceName, g_Sentences[i as usize].pName,
                      99999 as libc::c_int) == 0 {
            if !psentencenum.is_null() { *psentencenum = i }
            return g_Sentences[i as
                                   usize].pName.offset(Q_strlen(g_Sentences[i
                                                                                as
                                                                                usize].pName)
                                                           as
                                                           isize).offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
        }
        i += 1
    }
    return 0 as *mut libc::c_char;
}
// parse a null terminated string of text into component words, with
// pointers to each word stored in rgpparseword
// note: this code actually alters the passed in string!
#[no_mangle]
pub unsafe extern "C" fn VOX_ParseString(mut psz: *mut libc::c_char)
 -> *mut *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut fdone: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut p: *mut libc::c_char = psz;
    memset(rgpparseword.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut libc::c_char>() as
                libc::c_ulong).wrapping_mul(64 as libc::c_int as
                                                libc::c_ulong));
    if psz.is_null() { return 0 as *mut *mut libc::c_char }
    i = 0 as libc::c_int;
    let fresh0 = i;
    i = i + 1;
    rgpparseword[fresh0 as usize] = psz;
    while fdone == 0 && i < 64 as libc::c_int {
        // scan up to next word
        c = *p;
        while c as libc::c_int != 0 && IsNextWord(c) == 0 {
            p = p.offset(1);
            c = *p
        }
        // if '(' then scan for matching ')'
        if c as libc::c_int == '(' as i32 {
            p = ScanForwardUntil(p, ')' as i32 as libc::c_char);
            p = p.offset(1);
            c = *p;
            if c == 0 { fdone = 1 as libc::c_int }
        }
        if fdone != 0 || c == 0 {
            fdone = 1 as libc::c_int
        } else {
            // if . or , insert pause into rgpparseword,
			// unless this is the last character
            if (c as libc::c_int == '.' as i32 ||
                    c as libc::c_int == ',' as i32) &&
                   *p.offset(1 as libc::c_int as isize) as libc::c_int !=
                       '\n' as i32 &&
                   *p.offset(1 as libc::c_int as isize) as libc::c_int !=
                       '\r' as i32 &&
                   *p.offset(1 as libc::c_int as isize) as libc::c_int !=
                       0 as libc::c_int {
                if c as libc::c_int == '.' as i32 {
                    let fresh1 = i;
                    i = i + 1;
                    rgpparseword[fresh1 as usize] = voxperiod.as_mut_ptr()
                } else {
                    let fresh2 = i;
                    i = i + 1;
                    rgpparseword[fresh2 as usize] = voxcomma.as_mut_ptr()
                }
                if i >= 64 as libc::c_int { break ; }
            }
            // null terminate substring
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = 0 as libc::c_int as libc::c_char;
            // skip whitespace
            c = *p;
            while c as libc::c_int != 0 && IsSkipSpace(c) != 0 {
                p = p.offset(1);
                c = *p
            }
            if c == 0 {
                fdone = 1 as libc::c_int
            } else {
                let fresh4 = i;
                i = i + 1;
                rgpparseword[fresh4 as usize] = p
            }
        }
    }
    return rgpparseword.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn VOX_GetVolumeScale(mut pchan: *mut channel_t)
 -> libc::c_float {
    if !(*pchan).currentWord.is_null() {
        if (*pchan).words[(*pchan).wordIndex as usize].volume != 0 {
            let mut volume: libc::c_float =
                (*pchan).words[(*pchan).wordIndex as usize].volume as
                    libc::c_float * 0.01f32;
            if volume < 1.0f32 { return volume }
        }
    }
    return 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn VOX_SetChanVol(mut ch: *mut channel_t) {
    let mut scale: libc::c_float = 0.;
    if (*ch).currentWord.is_null() { return }
    scale = VOX_GetVolumeScale(ch);
    if scale == 1.0f32 { return }
    (*ch).rightvol = ((*ch).rightvol as libc::c_float * scale) as libc::c_int;
    (*ch).leftvol = ((*ch).leftvol as libc::c_float * scale) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VOX_ModifyPitch(mut ch: *mut channel_t,
                                         mut pitch: libc::c_float)
 -> libc::c_float {
    if !(*ch).currentWord.is_null() {
        if (*ch).words[(*ch).wordIndex as usize].pitch > 0 as libc::c_int {
            pitch +=
                ((*ch).words[(*ch).wordIndex as usize].pitch -
                     100 as libc::c_int) as libc::c_float * 0.01f32
        }
    }
    return pitch;
}
//===============================================================================
//  Get any pitch, volume, start, end params into voxword
//  and null out trailing format characters
//  Format:
//		someword(v100 p110 s10 e20)
//
//		v is volume, 0% to n%
//		p is pitch shift up 0% to n%
//		s is start wave offset %
//		e is end wave offset %
//		t is timecompression %
//
//  pass fFirst == 1 if this is the first string in sentence
//  returns 1 if valid string, 0 if parameter block only.
//
//  If a ( xxx ) parameter block does not directly follow a word,
//  then that 'default' parameter block will be used as the default value
//  for all following words.  Default parameter values are reset
//  by another 'default' parameter block.  Default parameter values
//  for a single word are overridden for that word if it has a parameter block.
//
//===============================================================================
#[no_mangle]
pub unsafe extern "C" fn VOX_ParseWordParams(mut psz: *mut libc::c_char,
                                             mut pvoxword: *mut voxword_t,
                                             mut fFirst: libc::c_int)
 -> libc::c_int {
    let mut pszsave: *mut libc::c_char = psz;
    let mut c: libc::c_char = 0;
    let mut ct: libc::c_char = 0;
    let mut sznum: [libc::c_char; 8] = [0; 8];
    static mut voxwordDefault: voxword_t =
        voxword_t{volume: 0,
                  pitch: 0,
                  start: 0,
                  end: 0,
                  cbtrim: 0,
                  fKeepCached: 0,
                  samplefrac: 0,
                  timecompress: 0,
                  sfx: 0 as *const sfx_t as *mut sfx_t,};
    let mut i: libc::c_int = 0;
    // init to defaults if this is the first word in string.
    if fFirst != 0 {
        voxwordDefault.pitch = -(1 as libc::c_int);
        voxwordDefault.volume = 100 as libc::c_int;
        voxwordDefault.start = 0 as libc::c_int;
        voxwordDefault.end = 100 as libc::c_int;
        voxwordDefault.fKeepCached = 0 as libc::c_int;
        voxwordDefault.timecompress = 0 as libc::c_int
    }
    *pvoxword = voxwordDefault;
    // look at next to last char to see if we have a
	// valid format:
    c =
        *psz.offset(Q_strlen(psz) as
                        isize).offset(-(1 as libc::c_int as isize));
    // no formatting, return
    if c as libc::c_int != ')' as i32 { return 1 as libc::c_int }
    // scan forward to first '('
    c = *psz;
    while IsDelimitChar(c) == 0 { psz = psz.offset(1); c = *psz }
    // bogus formatting
    if c as libc::c_int == ')' as i32 { return 0 as libc::c_int }
    // null terminate
    *psz = 0 as libc::c_int as libc::c_char;
    psz = psz.offset(1);
    ct = *psz;
    loop  {
        // scan until we hit a character in the commandSet
        while ct as libc::c_int != 0 && IsCommandChar(ct) == 0 {
            psz = psz.offset(1);
            ct = *psz
        }
        if ct as libc::c_int == ')' as i32 { break ; }
        memset(sznum.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong);
        i = 0 as libc::c_int;
        psz = psz.offset(1);
        c = *psz;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
               libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            break ;
        }
        // read number
        while *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                  libc::c_int &
                  _ISdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 &&
                  (i as libc::c_ulong) <
                      (::std::mem::size_of::<[libc::c_char; 8]>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong) {
            let fresh5 = i;
            i = i + 1;
            sznum[fresh5 as usize] = c;
            psz = psz.offset(1);
            c = *psz
        }
        // get value of number
        i = Q_atoi(sznum.as_mut_ptr());
        match ct as libc::c_int {
            118 => { (*pvoxword).volume = i }
            112 => { (*pvoxword).pitch = i }
            115 => { (*pvoxword).start = i }
            101 => { (*pvoxword).end = i }
            116 => { (*pvoxword).timecompress = i }
            _ => { }
        }
        ct = c
    }
    // if the string has zero length, this was an isolated
	// parameter block.  Set default voxword to these
	// values
    if Q_strlen(pszsave) == 0 as libc::c_int as libc::c_ulong {
        voxwordDefault = *pvoxword;
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VOX_LoadWord(mut pchan: *mut channel_t) {
    if !(*pchan).words[(*pchan).wordIndex as usize].sfx.is_null() {
        let mut pSource: *mut wavdata_t =
            S_LoadSound((*pchan).words[(*pchan).wordIndex as usize].sfx);
        if !pSource.is_null() {
            let mut start: libc::c_int =
                (*pchan).words[(*pchan).wordIndex as usize].start;
            let mut end: libc::c_int =
                (*pchan).words[(*pchan).wordIndex as usize].end;
            // apply mixer
            (*pchan).currentWord = &mut (*pchan).pMixer;
            (*(*pchan).currentWord).pData = pSource;
            // don't allow overlapped ranges
            if end <= start { end = 0 as libc::c_int } // sentence is finished
            if start != 0 || end != 0 {
                let mut sampleCount: libc::c_int = (*pSource).samples;
                if start != 0 {
                    S_SetSampleStart(pchan, pSource,
                                     (sampleCount as libc::c_float * 0.01f32 *
                                          start as libc::c_float) as
                                         libc::c_int);
                }
                if end != 0 {
                    S_SetSampleEnd(pchan, pSource,
                                   (sampleCount as libc::c_float * 0.01f32 *
                                        end as libc::c_float) as libc::c_int);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VOX_FreeWord(mut pchan: *mut channel_t) {
    (*pchan).currentWord = 0 as *mut mixer_t;
    memset(&mut (*pchan).pMixer as *mut mixer_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mixer_t>() as libc::c_ulong);
    // release unused sounds
    if !(*pchan).words[(*pchan).wordIndex as usize].sfx.is_null() {
        // If this wave wasn't precached by the game code
        if (*pchan).words[(*pchan).wordIndex as usize].fKeepCached == 0 {
            FS_FreeSound((*(*pchan).words[(*pchan).wordIndex as
                                              usize].sfx).cache);
            (*(*pchan).words[(*pchan).wordIndex as usize].sfx).cache =
                0 as *mut wavdata_t;
            (*pchan).words[(*pchan).wordIndex as usize].sfx = 0 as *mut sfx_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VOX_LoadFirstWord(mut pchan: *mut channel_t,
                                           mut pwords: *mut voxword_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    // copy each pointer in the sfx temp array into the
	// sentence array, and set the channel to point to the
	// sentence array
    while !(*pwords.offset(i as isize)).sfx.is_null() {
        (*pchan).words[i as usize] = *pwords.offset(i as isize);
        i += 1
    }
    (*pchan).words[i as usize].sfx = 0 as *mut sfx_t;
    (*pchan).wordIndex = 0 as libc::c_int;
    VOX_LoadWord(pchan);
}
// return number of samples mixed
#[no_mangle]
pub unsafe extern "C" fn VOX_MixDataToDevice(mut pchan: *mut channel_t,
                                             mut sampleCount: libc::c_int,
                                             mut outputRate: libc::c_int,
                                             mut outputOffset: libc::c_int)
 -> libc::c_int {
    // save this to compute total output
    let mut startingOffset: libc::c_int = outputOffset;
    if (*pchan).currentWord.is_null() { return 0 as libc::c_int }
    while sampleCount > 0 as libc::c_int && !(*pchan).currentWord.is_null() {
        let mut timeCompress: libc::c_int =
            (*pchan).words[(*pchan).wordIndex as usize].timecompress;
        let mut outputCount: libc::c_int =
            S_MixDataToDevice(pchan, sampleCount, outputRate, outputOffset,
                              timeCompress);
        outputOffset += outputCount;
        sampleCount -= outputCount;
        // if we finished load a next word
        if (*(*pchan).currentWord).finished as u64 != 0 {
            VOX_FreeWord(pchan);
            (*pchan).wordIndex += 1;
            VOX_LoadWord(pchan);
            if !(*pchan).currentWord.is_null() {
                (*pchan).sfx = (*pchan).words[(*pchan).wordIndex as usize].sfx
            }
        }
    }
    return outputOffset - startingOffset;
}
// link all sounds in sentence, start playing first word.
#[no_mangle]
pub unsafe extern "C" fn VOX_LoadSound(mut pchan: *mut channel_t,
                                       mut pszin: *const libc::c_char) {
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut i: libc::c_int = 0;
    let mut cword: libc::c_int = 0;
    let mut pathbuffer: [libc::c_char; 64] = [0; 64];
    let mut szpath: [libc::c_char; 32] = [0; 32];
    let mut rgvoxword: [voxword_t; 64] =
        [voxword_t{volume: 0,
                   pitch: 0,
                   start: 0,
                   end: 0,
                   cbtrim: 0,
                   fKeepCached: 0,
                   samplefrac: 0,
                   timecompress: 0,
                   sfx: 0 as *const sfx_t as *mut sfx_t,}; 64];
    let mut psz: *mut libc::c_char = 0 as *mut libc::c_char;
    if pszin.is_null() || *pszin == 0 { return }
    memset(rgvoxword.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<voxword_t>() as
                libc::c_ulong).wrapping_mul(64 as libc::c_int as
                                                libc::c_ulong));
    memset(buffer.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong);
    // lookup actual string in g_Sentences,
	// set pointer to string data
    if *pszin.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        psz = (pszin as *mut libc::c_char).offset(1 as libc::c_int as isize)
    } else { psz = VOX_LookupString(pszin, 0 as *mut libc::c_int) }
    if psz.is_null() {
        Con_DPrintf(b"^1Error:^7 VOX_LoadSound: no such sentence %s\n\x00" as
                        *const u8 as *const libc::c_char, pszin);
        return
    }
    // get directory from string, advance psz
    psz = VOX_GetDirectory(szpath.as_mut_ptr(), psz);
    if Q_strlen(psz) >
           (::std::mem::size_of::<[libc::c_char; 512]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        Con_Printf(b"^1Error:^7 VOX_LoadSound: sentence is too long %s\n\x00"
                       as *const u8 as *const libc::c_char, psz);
        return
    }
    // copy into buffer
    Q_strncpy(buffer.as_mut_ptr(), psz, 99999 as libc::c_int as size_t);
    psz = buffer.as_mut_ptr();
    // parse sentence (also inserts null terminators between words)
    VOX_ParseString(psz);
    // for each word in the sentence, construct the filename,
	// lookup the sfx and save each pointer in a temp array
    i = 0 as libc::c_int;
    cword = 0 as libc::c_int;
    while !rgpparseword[i as usize].is_null() {
        // Get any pitch, volume, start, end params into voxword
        if VOX_ParseWordParams(rgpparseword[i as usize],
                               &mut *rgvoxword.as_mut_ptr().offset(cword as
                                                                       isize),
                               (i == 0 as libc::c_int) as libc::c_int) != 0 {
            // this is a valid word (as opposed to a parameter block)
            Q_strncpy(pathbuffer.as_mut_ptr(), szpath.as_mut_ptr(),
                      99999 as libc::c_int as size_t);
            Q_strncat(pathbuffer.as_mut_ptr(), rgpparseword[i as usize],
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong);
            Q_strncat(pathbuffer.as_mut_ptr(),
                      b".wav\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong);
            // find name, if already in cache, mark voxword
			// so we don't discard when word is done playing
            rgvoxword[cword as usize].sfx =
                S_FindName(pathbuffer.as_mut_ptr(),
                           &mut (*rgvoxword.as_mut_ptr().offset(cword as
                                                                    isize)).fKeepCached);
            cword += 1
        }
        i += 1
    }
    VOX_LoadFirstWord(pchan, rgvoxword.as_mut_ptr());
    (*pchan).isSentence = true_0;
    (*pchan).sfx = rgvoxword[0 as libc::c_int as usize].sfx;
}
//-----------------------------------------------------------------------------
// Purpose: Take a NULL terminated sentence, and parse any commands contained in
//			{}.  The string is rewritten in place with those commands removed.
//
// Input  : *pSentenceData - sentence data to be modified in place
//			sentenceIndex - global sentence table index for any data that is
//							parsed out
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn VOX_ParseLineCommands(mut pSentenceData:
                                                   *mut libc::c_char,
                                               mut sentenceIndex:
                                                   libc::c_int) {
    let mut tempBuffer: [libc::c_char; 512] = [0; 512];
    let mut pNext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pStart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: libc::c_int = 0;
    let mut tempBufferPos: libc::c_int = 0 as libc::c_int;
    if pSentenceData.is_null() { return }
    pStart = pSentenceData;
    while *pSentenceData != 0 {
        pNext = ScanForwardUntil(pSentenceData, '{' as i32 as libc::c_char);
        // find length of "good" portion of the string (not a {} command)
        length =
            pNext.wrapping_offset_from(pSentenceData) as libc::c_long as
                libc::c_int;
        if (tempBufferPos + length) as libc::c_ulong >
               ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong {
            Con_Printf(b"^1Error:^7 Sentence too long (max length %lu characters)\n\x00"
                           as *const u8 as *const libc::c_char,
                       (::std::mem::size_of::<[libc::c_char; 512]>() as
                            libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong));
            return
        }
        // Copy good string to temp buffer
        memcpy(tempBuffer.as_mut_ptr().offset(tempBufferPos as isize) as
                   *mut libc::c_void, pSentenceData as *const libc::c_void,
               length as libc::c_ulong);
        // move the copy position
        tempBufferPos += length;
        pSentenceData = pNext;
        // skip ahead of the opening brace
        if *pSentenceData != 0 { pSentenceData = pSentenceData.offset(1) }
        // skip whitespace
        while *pSentenceData as libc::c_int != 0 &&
                  *pSentenceData as libc::c_int <= 32 as libc::c_int {
            pSentenceData = pSentenceData.offset(1)
        }
        // simple comparison of string commands:
        match Q_tolower(*pSentenceData) as libc::c_int {
            108 => {
                // all commands starting with the letter 'l' here
                if Q_strnicmp(pSentenceData,
                              b"len\x00" as *const u8 as *const libc::c_char,
                              3 as libc::c_int) == 0 {
                    g_Sentences[sentenceIndex as usize].length =
                        Q_atof(pSentenceData.offset(3 as libc::c_int as
                                                        isize))
                }
            }
            0 | _ => { }
        }
        pSentenceData =
            ScanForwardUntil(pSentenceData, '}' as i32 as libc::c_char);
        // skip the closing brace
        if *pSentenceData != 0 { pSentenceData = pSentenceData.offset(1) }
        // skip trailing whitespace
        while *pSentenceData as libc::c_int != 0 &&
                  *pSentenceData as libc::c_int <= 32 as libc::c_int {
            pSentenceData = pSentenceData.offset(1)
        }
    }
    if (tempBufferPos as libc::c_ulong) <
           ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong {
        // terminate cleaned up copy
        tempBuffer[tempBufferPos as usize] = 0 as libc::c_int as libc::c_char;
        // copy it over the original data
        Q_strncpy(pStart, tempBuffer.as_mut_ptr(),
                  99999 as libc::c_int as size_t);
    };
}
// Load sentence file into memory, insert null terminators to
// delimit sentence name/sentence pairs.  Keep pointer to each
// sentence name so we can search later.
#[no_mangle]
pub unsafe extern "C" fn VOX_ReadSentenceFile(mut psentenceFileName:
                                                  *const libc::c_char) {
    let mut c: libc::c_char = 0;
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pFileData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pchlast: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pSentenceData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileSize: fs_offset_t = 0;
    // load file
    pFileData =
        FS_LoadFile(psentenceFileName, &mut fileSize, false_0) as
            *mut libc::c_char; // this game just doesn't used vox sound system
    if pFileData.is_null() { return }
    pch = pFileData;
    pchlast = pch.offset(fileSize as isize);
    while pch < pchlast {
        if g_numSentences >= 4096 as libc::c_int as libc::c_uint {
            Con_Printf(b"^1Error:^7 VOX_Init: too many sentences specified, max is %d\n\x00"
                           as *const u8 as *const libc::c_char,
                       4096 as libc::c_int);
            break ;
        } else {
            // only process this pass on sentences
            pSentenceData = 0 as *mut libc::c_char;
            // skip newline, cr, tab, space
            c = *pch;
            while pch < pchlast && IsWhiteSpace(c) != 0 {
                pch = pch.offset(1);
                c = *pch
            }
            // skip entire line if first char is /
            if *pch as libc::c_int != '/' as i32 {
                let fresh6 = g_numSentences;
                g_numSentences = g_numSentences.wrapping_add(1);
                let mut pSentence: *mut sentence_t =
                    &mut *g_Sentences.as_mut_ptr().offset(fresh6 as isize) as
                        *mut sentence_t;
                (*pSentence).pName = pch;
                (*pSentence).length = 0 as libc::c_int as libc::c_float;
                // scan forward to first space, insert null terminator
			// after sentence name
                c = *pch;
                while pch < pchlast && c as libc::c_int != ' ' as i32 {
                    pch = pch.offset(1);
                    c = *pch
                }
                if pch < pchlast {
                    let fresh7 = pch;
                    pch = pch.offset(1);
                    *fresh7 = 0 as libc::c_int as libc::c_char
                }
                // a sentence may have some line commands, make an extra pass
                pSentenceData = pch
            }
            // scan forward to end of sentence or eof
            while pch < pchlast &&
                      *pch.offset(0 as libc::c_int as isize) as libc::c_int !=
                          '\n' as i32 &&
                      *pch.offset(0 as libc::c_int as isize) as libc::c_int !=
                          '\r' as i32 {
                pch = pch.offset(1)
            }
            // insert null terminator
            if pch < pchlast {
                let fresh8 = pch;
                pch = pch.offset(1);
                *fresh8 = 0 as libc::c_int as libc::c_char
            }
            // If we have some sentence data, parse out any line commands
            if !pSentenceData.is_null() && pSentenceData < pchlast {
                let mut index: libc::c_int =
                    g_numSentences.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) as
                        libc::c_int;
                // the current sentence has an index of count-1
                VOX_ParseLineCommands(pSentenceData, index);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VOX_Init() {
    memset(g_Sentences.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[sentence_t; 4096]>() as libc::c_ulong);
    g_numSentences = 0 as libc::c_int as uint;
    VOX_ReadSentenceFile(b"sound/sentences.txt\x00" as *const u8 as
                             *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn VOX_Shutdown() {
    g_numSentences = 0 as libc::c_int as uint;
}
