#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type file_t = file_s;
pub type fs_offset_t = off_t;
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
pub type scrvardef_t = scrvardef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrvardef_s {
    pub name: [libc::c_char; 256],
    pub value: [libc::c_char; 256],
    pub desc: [libc::c_char; 256],
    pub fMin: libc::c_float,
    pub fMax: libc::c_float,
    pub type_0: cvartype_t,
    pub flags: libc::c_int,
    pub fHandled: qboolean,
}
pub type cvartype_t = libc::c_uint;
pub const T_COUNT: cvartype_t = 5;
pub const T_STRING: cvartype_t = 4;
pub const T_LIST: cvartype_t = 3;
pub const T_NUMBER: cvartype_t = 2;
pub const T_BOOL: cvartype_t = 1;
pub const T_NONE: cvartype_t = 0;
pub type parserstate_t = parserstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parserstate_s {
    pub buf: *mut libc::c_char,
    pub token: [libc::c_char; 256],
    pub filename: *const libc::c_char,
}
#[no_mangle]
pub static mut cvartypes: [*const libc::c_char; 5] =
    [0 as *const libc::c_char,
     b"BOOL\x00" as *const u8 as *const libc::c_char,
     b"NUMBER\x00" as *const u8 as *const libc::c_char,
     b"LIST\x00" as *const u8 as *const libc::c_char,
     b"STRING\x00" as *const u8 as *const libc::c_char];
/*
===================
CSCR_ExpectString

Return true if next token is pExpext and skip it
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CSCR_ExpectString(mut ps: *mut parserstate_t,
                                           mut pExpect: *const libc::c_char,
                                           mut skip: qboolean,
                                           mut error: qboolean) -> qboolean {
    let mut tmp: *mut libc::c_char =
        _COM_ParseFileSafe((*ps).buf, (*ps).token.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 256]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_strnicmp((*ps).token.as_mut_ptr(), pExpect, 99999 as libc::c_int) ==
           0 {
        (*ps).buf = tmp;
        return true_0
    }
    if skip as u64 != 0 { (*ps).buf = tmp }
    if error as u64 != 0 {
        Con_DPrintf(b"^1Error:^7 Syntax error in %s: got \"%s\" instead of \"%s\"\n\x00"
                        as *const u8 as *const libc::c_char, (*ps).filename,
                    (*ps).token.as_mut_ptr(), pExpect);
    }
    return false_0;
}
/*
===================
CSCR_ParseType

Determine script variable type
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CSCR_ParseType(mut ps: *mut parserstate_t)
 -> cvartype_t {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < T_COUNT as libc::c_int {
        if CSCR_ExpectString(ps, cvartypes[i as usize], false_0, false_0) as
               u64 != 0 {
            return i as cvartype_t
        }
        i += 1
    }
    Con_DPrintf(b"^1Error:^7 Cannot parse %s: Bad type %s\n\x00" as *const u8
                    as *const libc::c_char, (*ps).filename,
                (*ps).token.as_mut_ptr());
    return T_NONE;
}
/*
=========================
CSCR_ParseSingleCvar
=========================
*/
#[no_mangle]
pub unsafe extern "C" fn CSCR_ParseSingleCvar(mut ps: *mut parserstate_t,
                                              mut result: *mut scrvardef_t)
 -> qboolean {
    // read the name
    (*ps).buf =
        _COM_ParseFileSafe((*ps).buf, (*result).name.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 256]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if CSCR_ExpectString(ps, b"{\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    // read description
    (*ps).buf =
        _COM_ParseFileSafe((*ps).buf, (*result).desc.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 256]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if CSCR_ExpectString(ps, b"{\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    (*result).type_0 = CSCR_ParseType(ps);
    match (*result).type_0 as libc::c_uint {
        1 => {
            // bool only has description
            if CSCR_ExpectString(ps,
                                 b"}\x00" as *const u8 as *const libc::c_char,
                                 false_0, true_0) as u64 == 0 {
                return false_0
            }
        }
        2 => {
            // min
            (*ps).buf =
                _COM_ParseFileSafe((*ps).buf, (*ps).token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 256]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*result).fMin = Q_atof((*ps).token.as_mut_ptr());
            // max
            (*ps).buf =
                _COM_ParseFileSafe((*ps).buf, (*ps).token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 256]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            (*result).fMax = Q_atof((*ps).token.as_mut_ptr());
            if CSCR_ExpectString(ps,
                                 b"}\x00" as *const u8 as *const libc::c_char,
                                 false_0, true_0) as u64 == 0 {
                return false_0
            }
        }
        4 => {
            if CSCR_ExpectString(ps,
                                 b"}\x00" as *const u8 as *const libc::c_char,
                                 false_0, true_0) as u64 == 0 {
                return false_0
            }
        }
        3 => {
            // read token for each item here
            while CSCR_ExpectString(ps,
                                    b"}\x00" as *const u8 as
                                        *const libc::c_char, true_0, false_0)
                      as u64 == 0 {
            }
        }
        _ => { return false_0 }
    }
    if CSCR_ExpectString(ps, b"{\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    // default value
    (*ps).buf =
        _COM_ParseFileSafe((*ps).buf, (*result).value.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 256]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if CSCR_ExpectString(ps, b"}\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    if CSCR_ExpectString(ps,
                         b"SetInfo\x00" as *const u8 as *const libc::c_char,
                         false_0, false_0) as u64 != 0 {
        (*result).flags |= (1 as libc::c_int) << 1 as libc::c_int
    }
    if CSCR_ExpectString(ps, b"}\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    return true_0;
}
/*
======================
CSCR_ParseHeader

Check version and seek to first cvar name
======================
*/
#[no_mangle]
pub unsafe extern "C" fn CSCR_ParseHeader(mut ps: *mut parserstate_t)
 -> qboolean {
    if CSCR_ExpectString(ps,
                         b"VERSION\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    // Parse in the version #
	// Get the first token.
    (*ps).buf =
        _COM_ParseFileSafe((*ps).buf, (*ps).token.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 256]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_atof((*ps).token.as_mut_ptr()) != 1 as libc::c_int as libc::c_float {
        Con_DPrintf(b"^1Error:^7 File %s has wrong version %s!\n\x00" as
                        *const u8 as *const libc::c_char, (*ps).filename,
                    (*ps).token.as_mut_ptr());
        return false_0
    }
    if CSCR_ExpectString(ps,
                         b"DESCRIPTION\x00" as *const u8 as
                             *const libc::c_char, false_0, true_0) as u64 == 0
       {
        return false_0
    }
    (*ps).buf =
        _COM_ParseFileSafe((*ps).buf, (*ps).token.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 256]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_strnicmp((*ps).token.as_mut_ptr(),
                  b"INFO_OPTIONS\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 &&
           Q_strnicmp((*ps).token.as_mut_ptr(),
                      b"SERVER_OPTIONS\x00" as *const u8 as
                          *const libc::c_char, 99999 as libc::c_int) != 0 {
        Con_DPrintf(b"^1Error:^7 DESCRIPTION must be INFO_OPTIONS or SERVER_OPTIONS\n\x00"
                        as *const u8 as *const libc::c_char);
        return false_0
    }
    if CSCR_ExpectString(ps, b"{\x00" as *const u8 as *const libc::c_char,
                         false_0, true_0) as u64 == 0 {
        return false_0
    }
    return true_0;
}
/*
==============
CSCR_ParseFile

generic scr parser
will callback on each scrvardef_t
==============
*/
unsafe extern "C" fn CSCR_ParseFile(mut scriptfilename: *const libc::c_char,
                                    mut callback:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut scrvardef_t,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()>,
                                    mut userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut state: parserstate_t =
        {
            let mut init =
                parserstate_s{buf: 0 as *mut libc::c_char,
                              token: [0; 256],
                              filename: 0 as *const libc::c_char,};
            init
        };
    let mut success: qboolean = false_0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut length: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    state.filename = scriptfilename;
    start =
        FS_LoadFile(scriptfilename, &mut length, true_0) as *mut libc::c_char;
    state.buf = start;
    if state.buf.is_null() || length == 0 { return 0 as libc::c_int }
    Con_DPrintf(b"Reading config script file %s\n\x00" as *const u8 as
                    *const libc::c_char, scriptfilename);
    if !(CSCR_ParseHeader(&mut state) as u64 == 0) {
        while CSCR_ExpectString(&mut state,
                                b"}\x00" as *const u8 as *const libc::c_char,
                                false_0, false_0) as u64 == 0 {
            let mut var: scrvardef_t =
                {
                    let mut init =
                        scrvardef_s{name:
                                        [0 as libc::c_int as libc::c_char, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0],
                                    value: [0; 256],
                                    desc: [0; 256],
                                    fMin: 0.,
                                    fMax: 0.,
                                    type_0: T_NONE,
                                    flags: 0,
                                    fHandled: false_0,};
                    init
                };
            // Create a new object
            if !(CSCR_ParseSingleCvar(&mut state, &mut var) as u64 != 0) {
                break ;
            }
            callback.expect("non-null function pointer")(&mut var, userdata);
            count += 1;
            if count > 1024 as libc::c_int { break ; }
        }
        if !_COM_ParseFileSafe(state.buf, state.token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 256]>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int).is_null() {
            Con_DPrintf(b"^1Error:^7 Got extra tokens!\n\x00" as *const u8 as
                            *const libc::c_char);
        } else { success = true_0 }
    }
    if success as u64 == 0 {
        state.token[(::std::mem::size_of::<[libc::c_char; 256]>() as
                         libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) as
                        usize] = 0 as libc::c_int as libc::c_char;
        if !start.is_null() && !state.buf.is_null() {
            Con_DPrintf(b"^1Error:^7 Parse error in %s, byte %d, token %s\n\x00"
                            as *const u8 as *const libc::c_char,
                        scriptfilename,
                        state.buf.wrapping_offset_from(start) as libc::c_long
                            as libc::c_int, state.token.as_mut_ptr());
        } else {
            Con_DPrintf(b"^1Error:^7 Parse error in %s, token %s\n\x00" as
                            *const u8 as *const libc::c_char, scriptfilename,
                        state.token.as_mut_ptr());
        }
    }
    if !start.is_null() {
        _Mem_Free(start as *mut libc::c_void,
                  b"../engine/common/cfgscript.c\x00" as *const u8 as
                      *const libc::c_char, 262 as libc::c_int);
    }
    return count;
}
unsafe extern "C" fn CSCR_WriteVariableToFile(mut var: *mut scrvardef_t,
                                              mut file: *mut libc::c_void) {
    let mut cfg: *mut file_t = file as *mut file_t;
    let mut cvar: *mut convar_t =
        Cvar_FindVarExt((*var).name.as_mut_ptr(), 0 as libc::c_int);
    if !cvar.is_null() &&
           (*cvar).flags &
               ((1 as libc::c_int) << 2 as libc::c_int |
                    (1 as libc::c_int) << 0 as libc::c_int) == 0 {
        // cvars will be placed in game.cfg and restored on map start
        if (*var).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            FS_Printf(cfg,
                      b"setinfo %s \"%s\"\n\x00" as *const u8 as
                          *const libc::c_char, (*var).name.as_mut_ptr(),
                      (*cvar).string);
        } else {
            FS_Printf(cfg,
                      b"%s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                      (*var).name.as_mut_ptr(), (*cvar).string);
        }
    };
}
/*
======================
CSCR_WriteGameCVars

Print all cvars declared in script to game.cfg file
======================
*/
#[no_mangle]
pub unsafe extern "C" fn CSCR_WriteGameCVars(mut cfg: *mut file_t,
                                             mut scriptfilename:
                                                 *const libc::c_char)
 -> libc::c_int {
    return CSCR_ParseFile(scriptfilename,
                          Some(CSCR_WriteVariableToFile as
                                   unsafe extern "C" fn(_: *mut scrvardef_t,
                                                        _: *mut libc::c_void)
                                       -> ()), cfg as *mut libc::c_void);
}
unsafe extern "C" fn CSCR_RegisterVariable(mut var: *mut scrvardef_t,
                                           mut unused: *mut libc::c_void) {
    if Cvar_FindVarExt((*var).name.as_mut_ptr(), 0 as libc::c_int).is_null() {
        Cvar_Get((*var).name.as_mut_ptr(), (*var).value.as_mut_ptr(),
                 (*var).flags | (1 as libc::c_int) << 21 as libc::c_int,
                 (*var).desc.as_mut_ptr());
    };
}
/*
======================
CSCR_LoadDefaultCVars

Register all cvars declared in config file and set default values
======================
*/
#[no_mangle]
pub unsafe extern "C" fn CSCR_LoadDefaultCVars(mut scriptfilename:
                                                   *const libc::c_char)
 -> libc::c_int {
    return CSCR_ParseFile(scriptfilename,
                          Some(CSCR_RegisterVariable as
                                   unsafe extern "C" fn(_: *mut scrvardef_t,
                                                        _: *mut libc::c_void)
                                       -> ()), 0 as *mut libc::c_void);
}
