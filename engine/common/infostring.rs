#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_tolower(in_0: libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
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
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_Userinfo() -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type file_t = file_s;
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
/*
=======================================================================

			INFOSTRING STUFF

=======================================================================
*/
/*
===============
Info_Print

printing current key-value pair
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Info_Print(mut s: *const libc::c_char) {
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut value: [libc::c_char; 128] = [0; 128];
    let mut l: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
    while *s != 0 {
        count = 0 as libc::c_int;
        o = key.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = o;
            o = o.offset(1);
            *fresh1 = *fresh0;
            count += 1
        }
        l =
            o.wrapping_offset_from(key.as_mut_ptr()) as libc::c_long as
                libc::c_int;
        if l < 20 as libc::c_int {
            memset(o as *mut libc::c_void, ' ' as i32,
                   (20 as libc::c_int - l) as libc::c_ulong);
            key[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
        } else { *o = 0 as libc::c_int as libc::c_char }
        Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   key.as_mut_ptr());
        if *s == 0 {
            Con_Printf(b"(null)\n\x00" as *const u8 as *const libc::c_char);
            return
        }
        count = 0 as libc::c_int;
        o = value.as_mut_ptr();
        s = s.offset(1);
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            let fresh3 = o;
            o = o.offset(1);
            *fresh3 = *fresh2;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if *s != 0 { s = s.offset(1) }
        Con_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   value.as_mut_ptr());
    };
}
/*
==============
Info_IsValid

check infostring for potential problems
==============
*/
#[no_mangle]
pub unsafe extern "C" fn Info_IsValid(mut s: *const libc::c_char)
 -> qboolean {
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut value: [libc::c_char; 128] = [0; 128];
    let mut count: libc::c_int = 0;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
    while *s != 0 {
        count = 0 as libc::c_int;
        o = key.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh4 = s;
            s = s.offset(1);
            let fresh5 = o;
            o = o.offset(1);
            *fresh5 = *fresh4;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if *s == 0 { return false_0 }
        count = 0 as libc::c_int;
        o = value.as_mut_ptr();
        s = s.offset(1);
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh6 = s;
            s = s.offset(1);
            let fresh7 = o;
            o = o.offset(1);
            *fresh7 = *fresh6;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if if *value.as_mut_ptr() == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            return false_0
        }
        if *s != 0 { s = s.offset(1) }
    }
    return true_0;
}
/*
==============
Info_WriteVars

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Info_WriteVars(mut f: *mut file_t) {
    let mut s: *mut libc::c_char =
        CL_Userinfo(); // use two buffers so compares work without stomping on each other
    let mut pkey: [libc::c_char; 512] = [0; 512];
    static mut value: [[libc::c_char; 512]; 4] = [[0; 512]; 4];
    static mut valueindex: libc::c_int = 0;
    let mut pcvar: *mut convar_t = 0 as *mut convar_t;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    valueindex = (valueindex + 1 as libc::c_int) % 4 as libc::c_int;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
    loop  {
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if *s == 0 { return }
            let fresh8 = s;
            s = s.offset(1);
            let fresh9 = o;
            o = o.offset(1);
            *fresh9 = *fresh8
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = value[valueindex as usize].as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            if *s == 0 { return }
            let fresh10 = s;
            s = s.offset(1);
            let fresh11 = o;
            o = o.offset(1);
            *fresh11 = *fresh10
        }
        *o = 0 as libc::c_int as libc::c_char;
        pcvar = Cvar_FindVarExt(pkey.as_mut_ptr(), 0 as libc::c_int);
        if pcvar.is_null() &&
               pkey[0 as libc::c_int as usize] as libc::c_int != '*' as i32 {
            // don't store out star keys
            FS_Printf(f,
                      b"setinfo \"%s\" \"%s\"\n\x00" as *const u8 as
                          *const libc::c_char, pkey.as_mut_ptr(),
                      value[valueindex as usize].as_mut_ptr());
        }
        if *s == 0 { return }
        s = s.offset(1)
    };
}
// XASH_DEDICATED
/*
===============
Info_ValueForKey

Searches the string for the given
key and returns the associated value, or an empty string.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Info_ValueForKey(mut s: *const libc::c_char,
                                          mut key: *const libc::c_char)
 -> *const libc::c_char {
    let mut pkey: [libc::c_char; 128] =
        [0;
            128]; // use two buffers so compares work without stomping on each other
    static mut value: [[libc::c_char; 128]; 4] =
        [[0; 128]; 4]; // remove this part
    static mut valueindex: libc::c_int = 0; // just clear variable
    let mut count: libc::c_int = 0;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    valueindex = (valueindex + 1 as libc::c_int) % 4 as libc::c_int;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
    loop  {
        count = 0 as libc::c_int;
        o = pkey.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != '\\' as i32 {
            if *s == 0 { return b"\x00" as *const u8 as *const libc::c_char }
            let fresh12 = s;
            s = s.offset(1);
            let fresh13 = o;
            o = o.offset(1);
            *fresh13 = *fresh12;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = value[valueindex as usize].as_mut_ptr();
        count = 0 as libc::c_int;
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            if *s == 0 { return b"\x00" as *const u8 as *const libc::c_char }
            let fresh14 = s;
            s = s.offset(1);
            let fresh15 = o;
            o = o.offset(1);
            *fresh15 = *fresh14;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if Q_strncmp(key, pkey.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return value[valueindex as usize].as_mut_ptr()
        }
        if *s == 0 { return b"\x00" as *const u8 as *const libc::c_char }
        s = s.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey(mut s: *mut libc::c_char,
                                        mut key: *const libc::c_char)
 -> qboolean {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pkey: [libc::c_char; 128] = [0; 128];
    let mut value: [libc::c_char; 128] = [0; 128];
    let mut cmpsize: libc::c_int = Q_strlen(key) as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if cmpsize > 128 as libc::c_int - 1 as libc::c_int {
        cmpsize = 128 as libc::c_int - 1 as libc::c_int
    }
    if !Q_strchr(key, '\\' as i32 as libc::c_char).is_null() {
        return false_0
    }
    loop  {
        start = s;
        if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
        count = 0 as libc::c_int;
        o = pkey.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != '\\' as i32 {
            if *s == 0 { return false_0 }
            let fresh16 = s;
            s = s.offset(1);
            let fresh17 = o;
            o = o.offset(1);
            *fresh17 = *fresh16;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        count = 0 as libc::c_int;
        o = value.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            if *s == 0 { return false_0 }
            let fresh18 = s;
            s = s.offset(1);
            let fresh19 = o;
            o = o.offset(1);
            *fresh19 = *fresh18;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if Q_strncmp(key, pkey.as_mut_ptr(), cmpsize) == 0 {
            Q_strncpy(start, s, 99999 as libc::c_int as size_t);
            return true_0
        }
        if *s == 0 { return false_0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemovePrefixedKeys(mut start: *mut libc::c_char,
                                                 mut prefix: libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pkey: [libc::c_char; 128] = [0; 128];
    let mut value: [libc::c_char; 128] = [0; 128];
    let mut count: libc::c_int = 0;
    s = start;
    loop  {
        if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
        count = 0 as libc::c_int;
        o = pkey.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != '\\' as i32 {
            if *s == 0 { return }
            let fresh20 = s;
            s = s.offset(1);
            let fresh21 = o;
            o = o.offset(1);
            *fresh21 = *fresh20;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        count = 0 as libc::c_int;
        o = value.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            if *s == 0 { return }
            let fresh22 = s;
            s = s.offset(1);
            let fresh23 = o;
            o = o.offset(1);
            *fresh23 = *fresh22;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if pkey[0 as libc::c_int as usize] as libc::c_int ==
               prefix as libc::c_int {
            Info_RemoveKey(start, pkey.as_mut_ptr());
            s = start
        }
        if *s == 0 { return }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_IsKeyImportant(mut key: *const libc::c_char)
 -> qboolean {
    if *key.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
        return true_0
    }
    if Q_strncmp(key, b"name\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"model\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"rate\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"topcolor\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"bottomcolor\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key,
                 b"cl_updaterate\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"cl_lw\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"cl_lc\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    if Q_strncmp(key, b"cl_nopred\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        return true_0
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Info_FindLargestKey(mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut value: [libc::c_char; 128] = [0; 128];
    static mut largest_key: [libc::c_char; 128] = [0; 128];
    let mut largest_size: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    *largest_key.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1) }
    while *s != 0 {
        let mut size: libc::c_int = 0 as libc::c_int;
        count = 0 as libc::c_int;
        o = key.as_mut_ptr();
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh24 = s;
            s = s.offset(1);
            let fresh25 = o;
            o = o.offset(1);
            *fresh25 = *fresh24;
            count += 1
        }
        l =
            o.wrapping_offset_from(key.as_mut_ptr()) as libc::c_long as
                libc::c_int;
        *o = 0 as libc::c_int as libc::c_char;
        size = Q_strlen(key.as_mut_ptr()) as libc::c_int;
        if *s == 0 { return largest_key.as_mut_ptr() }
        count = 0 as libc::c_int;
        o = value.as_mut_ptr();
        s = s.offset(1);
        while count < 128 as libc::c_int - 1 as libc::c_int &&
                  *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh26 = s;
            s = s.offset(1);
            let fresh27 = o;
            o = o.offset(1);
            *fresh27 = *fresh26;
            count += 1
        }
        *o = 0 as libc::c_int as libc::c_char;
        if *s != 0 { s = s.offset(1) }
        size =
            (size as libc::c_ulong).wrapping_add(Q_strlen(value.as_mut_ptr()))
                as libc::c_int as libc::c_int;
        if size > largest_size &&
               Info_IsKeyImportant(key.as_mut_ptr()) as u64 == 0 {
            Q_strncpy(largest_key.as_mut_ptr(), key.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 128]>() as
                          libc::c_ulong);
            largest_size = size
        }
    }
    return largest_key.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForStarKey(mut s: *mut libc::c_char,
                                                 mut key: *const libc::c_char,
                                                 mut value:
                                                     *const libc::c_char,
                                                 mut maxsize: libc::c_int)
 -> qboolean {
    let mut new: [libc::c_char; 1024] = [0; 1024];
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut team: libc::c_int = 0;
    if !Q_strchr(key, '\\' as i32 as libc::c_char).is_null() ||
           !Q_strchr(value, '\\' as i32 as libc::c_char).is_null() {
        Con_Printf(b"^1Error:^7 SetValueForKey: can\'t use keys or values with a \\\n\x00"
                       as *const u8 as *const libc::c_char);
        return false_0
    }
    if !Q_strstr(key, b"..\x00" as *const u8 as *const libc::c_char).is_null()
           ||
           !Q_strstr(value,
                     b"..\x00" as *const u8 as *const libc::c_char).is_null()
       {
        return false_0
    }
    if !Q_strchr(key, '\"' as i32 as libc::c_char).is_null() ||
           !Q_strchr(value, '\"' as i32 as libc::c_char).is_null() {
        Con_Printf(b"^1Error:^7 SetValueForKey: can\'t use keys or values with a \"\n\x00"
                       as *const u8 as *const libc::c_char);
        return false_0
    }
    if Q_strlen(key) >
           (128 as libc::c_int - 1 as libc::c_int) as libc::c_ulong ||
           Q_strlen(value) >
               (128 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        return false_0
    }
    Info_RemoveKey(s, key);
    if if value.is_null() || *value == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return true_0
    }
    Q_snprintf(new.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
               b"\\%s\\%s\x00" as *const u8 as *const libc::c_char, key,
               value);
    if Q_strlen(new.as_mut_ptr()).wrapping_add(Q_strlen(s)) >
           maxsize as libc::c_ulong {
        // no more room in buffer to add key/value
        if Info_IsKeyImportant(key) as u64 != 0 {
            // keep removing the largest key/values until we have room
            let mut largekey: *mut libc::c_char = 0 as *mut libc::c_char;
            loop  {
                largekey = Info_FindLargestKey(s);
                Info_RemoveKey(s, largekey);
                if !(Q_strlen(new.as_mut_ptr()).wrapping_add(Q_strlen(s)) >=
                         maxsize as libc::c_ulong &&
                         *largekey as libc::c_int != 0 as libc::c_int) {
                    break ;
                }
            }
            if *largekey.offset(0 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int {
                // no room to add setting
                return true_0
                // info changed, new value can't saved
            }
        } else {
            // no room to add setting
            return true_0
            // info changed, new value can't saved
        }
    }
    // only copy ascii values
    s = s.offset(Q_strlen(s) as isize);
    v = new.as_mut_ptr();
    team =
        if Q_strnicmp(key, b"team\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 as libc::c_int {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int };
    while *v != 0 {
        let fresh28 = v;
        v = v.offset(1);
        c = *fresh28 as byte as libc::c_int;
        if team != 0 { c = Q_tolower(c as libc::c_char) as libc::c_int }
        if c > 13 as libc::c_int {
            let fresh29 = s;
            s = s.offset(1);
            *fresh29 = c as libc::c_char
        }
    }
    *s = 0 as libc::c_int as libc::c_char;
    // all done
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey(mut s: *mut libc::c_char,
                                             mut key: *const libc::c_char,
                                             mut value: *const libc::c_char,
                                             mut maxsize: libc::c_int)
 -> qboolean {
    if *key.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
        Con_Printf(b"^1Error:^7 Can\'t set *keys\n\x00" as *const u8 as
                       *const libc::c_char);
        return false_0
    }
    return Info_SetValueForStarKey(s, key, value, maxsize);
}
