#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
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
pub type __uint8_t = libc::c_uchar;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type uint = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string = [libc::c_char; 256];
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TIME_FILENAME: C2RustUnnamed_0 = 5;
pub const TIME_YEAR_ONLY: C2RustUnnamed_0 = 4;
pub const TIME_NO_SECONDS: C2RustUnnamed_0 = 3;
pub const TIME_TIME_ONLY: C2RustUnnamed_0 = 2;
pub const TIME_DATE_ONLY: C2RustUnnamed_0 = 1;
pub const TIME_FULL: C2RustUnnamed_0 = 0;
/*
crtlib.c - internal stdlib
Copyright (C) 2011 Uncle Mike

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
pub unsafe extern "C" fn Q_strnupr(mut in_0: *const libc::c_char,
                                   mut out: *mut libc::c_char,
                                   mut size_out: size_t) {
    if size_out == 0 as libc::c_int as libc::c_ulong { return }
    while *in_0 as libc::c_int != 0 &&
              size_out > 1 as libc::c_int as libc::c_ulong {
        if *in_0 as libc::c_int >= 'a' as i32 &&
               *in_0 as libc::c_int <= 'z' as i32 {
            let fresh0 = in_0;
            in_0 = in_0.offset(1);
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 =
                (*fresh0 as libc::c_int + 'A' as i32 - 'a' as i32) as
                    libc::c_char
        } else {
            let fresh2 = in_0;
            in_0 = in_0.offset(1);
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = *fresh2
        }
        size_out = size_out.wrapping_sub(1)
    }
    *out = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strnlwr(mut in_0: *const libc::c_char,
                                   mut out: *mut libc::c_char,
                                   mut size_out: size_t) {
    if size_out == 0 as libc::c_int as libc::c_ulong { return }
    while *in_0 as libc::c_int != 0 &&
              size_out > 1 as libc::c_int as libc::c_ulong {
        if *in_0 as libc::c_int >= 'A' as i32 &&
               *in_0 as libc::c_int <= 'Z' as i32 {
            let fresh4 = in_0;
            in_0 = in_0.offset(1);
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 =
                (*fresh4 as libc::c_int + 'a' as i32 - 'A' as i32) as
                    libc::c_char
        } else {
            let fresh6 = in_0;
            in_0 = in_0.offset(1);
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = *fresh6
        }
        size_out = size_out.wrapping_sub(1)
    }
    *out = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isdigit(mut str: *const libc::c_char) -> qboolean {
    if !str.is_null() && *str as libc::c_int != 0 {
        while *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as
                  libc::c_int &
                  _ISdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            str = str.offset(1)
        }
        if *str == 0 { return true_0 }
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strlen(mut string: *const libc::c_char) -> size_t {
    let mut len: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if string.is_null() { return 0 as libc::c_int as size_t }
    len = 0 as libc::c_int as size_t;
    p = string;
    while *p != 0 { p = p.offset(1); len = len.wrapping_add(1) }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn Q_colorstr(mut string: *const libc::c_char)
 -> size_t {
    let mut len: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if string.is_null() { return 0 as libc::c_int as size_t }
    len = 0 as libc::c_int as size_t;
    p = string;
    while *p != 0 {
        if !p.is_null() && *p as libc::c_int == '^' as i32 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            len =
                (len as
                     libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            p = p.offset(2 as libc::c_int as isize)
        } else { p = p.offset(1) }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn Q_toupper(in_0: libc::c_char) -> libc::c_char {
    let mut out: libc::c_char = 0;
    if in_0 as libc::c_int >= 'a' as i32 && in_0 as libc::c_int <= 'z' as i32
       {
        out = (in_0 as libc::c_int + 'A' as i32 - 'a' as i32) as libc::c_char
    } else { out = in_0 }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn Q_tolower(in_0: libc::c_char) -> libc::c_char {
    let mut out: libc::c_char = 0;
    if in_0 as libc::c_int >= 'A' as i32 && in_0 as libc::c_int <= 'Z' as i32
       {
        out = (in_0 as libc::c_int + 'a' as i32 - 'A' as i32) as libc::c_char
    } else { out = in_0 }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncat(mut dst: *mut libc::c_char,
                                   mut src: *const libc::c_char,
                                   mut size: size_t) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = size;
    let mut dlen: size_t = 0;
    if dst.is_null() || src.is_null() || size == 0 {
        return 0 as libc::c_int as size_t
    }
    loop 
         // find the end of dst and adjust bytes left but don't go past end
         {
        let fresh8 = n;
        n = n.wrapping_sub(1);
        if !(fresh8 != 0 as libc::c_int as libc::c_ulong &&
                 *d as libc::c_int != '\u{0}' as i32) {
            break ;
        }
        d = d.offset(1)
    }
    dlen = d.wrapping_offset_from(dst) as libc::c_long as size_t;
    n = size.wrapping_sub(dlen);
    if n == 0 as libc::c_int as libc::c_ulong {
        return dlen.wrapping_add(Q_strlen(s))
    }
    while *s as libc::c_int != '\u{0}' as i32 {
        if n != 1 as libc::c_int as libc::c_ulong {
            let fresh9 = d;
            d = d.offset(1);
            *fresh9 = *s;
            n = n.wrapping_sub(1)
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return dlen.wrapping_add(s.wrapping_offset_from(src) as libc::c_long as
                                 libc::c_ulong);
    // count does not include NULL
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncpy(mut dst: *mut libc::c_char,
                                   mut src: *const libc::c_char,
                                   mut size: size_t) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = size;
    if dst.is_null() || src.is_null() || size == 0 {
        return 0 as libc::c_int as size_t
    }
    // copy as many bytes as will fit
    if n != 0 as libc::c_int as libc::c_ulong &&
           { n = n.wrapping_sub(1); (n) != 0 as libc::c_int as libc::c_ulong }
       {
        loop  {
            let fresh10 = s;
            s = s.offset(1);
            let fresh11 = d;
            d = d.offset(1);
            *fresh11 = *fresh10;
            if *fresh11 as libc::c_int == 0 as libc::c_int { break ; }
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_int as libc::c_ulong) { break ; }
        }
    }
    // not enough room in dst, add NULL and traverse rest of src
    if n == 0 as libc::c_int as libc::c_ulong {
        if size != 0 as libc::c_int as libc::c_ulong {
            *d = '\u{0}' as i32 as libc::c_char
        } // NULL-terminate dst
        loop  {
            let fresh12 = s;
            s = s.offset(1);
            if !(*fresh12 != 0) { break ; }
        }
    }
    return (s.wrapping_offset_from(src) as libc::c_long -
                1 as libc::c_int as libc::c_long) as size_t;
    // count does not include NULL
}
#[no_mangle]
pub unsafe extern "C" fn Q_atoi(mut str: *const libc::c_char) -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    if str.is_null() { return 0 as libc::c_int }
    // check for empty charachters in string
    while !str.is_null() && *str as libc::c_int == ' ' as i32 {
        str = str.offset(1)
    }
    if str.is_null() { return 0 as libc::c_int }
    if *str as libc::c_int == '-' as i32 {
        sign = -(1 as libc::c_int);
        str = str.offset(1)
    } else { sign = 1 as libc::c_int }
    // check for hex
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 &&
           (*str.offset(1 as libc::c_int as isize) as libc::c_int ==
                'x' as i32 ||
                *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                    'X' as i32) {
        str = str.offset(2 as libc::c_int as isize);
        loop  {
            let fresh13 = str;
            str = str.offset(1);
            c = *fresh13 as libc::c_int;
            if c >= '0' as i32 && c <= '9' as i32 {
                val = (val << 4 as libc::c_int) + c - '0' as i32
            } else if c >= 'a' as i32 && c <= 'f' as i32 {
                val =
                    (val << 4 as libc::c_int) + c - 'a' as i32 +
                        10 as libc::c_int
            } else if c >= 'A' as i32 && c <= 'F' as i32 {
                val =
                    (val << 4 as libc::c_int) + c - 'A' as i32 +
                        10 as libc::c_int
            } else { return val * sign }
        }
    }
    // check for character
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32 {
        return sign * *str.offset(1 as libc::c_int as isize) as libc::c_int
    }
    loop 
         // assume decimal
         {
        let fresh14 = str;
        str = str.offset(1);
        c = *fresh14 as libc::c_int;
        if c < '0' as i32 || c > '9' as i32 { return val * sign }
        val = val * 10 as libc::c_int + c - '0' as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Q_atof(mut str: *const libc::c_char)
 -> libc::c_float {
    let mut val: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut c: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut decimal: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    if str.is_null() { return 0.0f32 }
    // check for empty charachters in string
    while !str.is_null() && *str as libc::c_int == ' ' as i32 {
        str = str.offset(1)
    }
    if str.is_null() { return 0.0f32 }
    if *str as libc::c_int == '-' as i32 {
        sign = -(1 as libc::c_int);
        str = str.offset(1)
    } else { sign = 1 as libc::c_int }
    // check for hex
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 &&
           (*str.offset(1 as libc::c_int as isize) as libc::c_int ==
                'x' as i32 ||
                *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                    'X' as i32) {
        str = str.offset(2 as libc::c_int as isize);
        loop  {
            let fresh15 = str;
            str = str.offset(1);
            c = *fresh15 as libc::c_int;
            if c >= '0' as i32 && c <= '9' as i32 {
                val =
                    val * 16 as libc::c_int as libc::c_double +
                        c as libc::c_double - '0' as i32 as libc::c_double
            } else if c >= 'a' as i32 && c <= 'f' as i32 {
                val =
                    val * 16 as libc::c_int as libc::c_double +
                        c as libc::c_double - 'a' as i32 as libc::c_double +
                        10 as libc::c_int as libc::c_double
            } else if c >= 'A' as i32 && c <= 'F' as i32 {
                val =
                    val * 16 as libc::c_int as libc::c_double +
                        c as libc::c_double - 'A' as i32 as libc::c_double +
                        10 as libc::c_int as libc::c_double
            } else { return (val * sign as libc::c_double) as libc::c_float }
        }
    }
    // check for character
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32 {
        return (sign * *str.offset(1 as libc::c_int as isize) as libc::c_int)
                   as libc::c_float
    }
    // assume decimal
    decimal = -(1 as libc::c_int);
    total = 0 as libc::c_int;
    loop  {
        let fresh16 = str;
        str = str.offset(1);
        c = *fresh16 as libc::c_int;
        if c == '.' as i32 {
            decimal = total
        } else {
            if c < '0' as i32 || c > '9' as i32 { break ; }
            val =
                val * 10 as libc::c_int as libc::c_double +
                    c as libc::c_double - '0' as i32 as libc::c_double;
            total += 1
        }
    }
    if decimal == -(1 as libc::c_int) {
        return (val * sign as libc::c_double) as libc::c_float
    }
    while total > decimal {
        val /= 10 as libc::c_int as libc::c_double;
        total -= 1
    }
    return (val * sign as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Q_atov(mut vec: *mut libc::c_float,
                                mut str: *const libc::c_char,
                                mut siz: size_t) {
    let mut buffer: string = [0; 256];
    let mut pstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pfront: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    Q_strncpy(buffer.as_mut_ptr(), str,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    memset(vec as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<vec_t>() as
                libc::c_ulong).wrapping_mul(siz));
    pfront = buffer.as_mut_ptr();
    pstr = pfront;
    j = 0 as libc::c_int;
    while (j as libc::c_ulong) < siz {
        *vec.offset(j as isize) = Q_atof(pfront);
        // valid separator is space
        while *pstr as libc::c_int != 0 && *pstr as libc::c_int != ' ' as i32
              {
            pstr = pstr.offset(1)
        } // strings are equal until end point
        if *pstr == 0 { break ; }
        pstr = pstr.offset(1);
        pfront = pstr;
        j += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Q_strchr(mut s: *const libc::c_char,
                                  mut c: libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = Q_strlen(s);
    loop  {
        let fresh17 = len;
        len = len.wrapping_sub(1);
        if !(fresh17 != 0) { break ; }
        s = s.offset(1);
        if *s as libc::c_int == c as libc::c_int {
            return s as *mut libc::c_char
        }
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strrchr(mut s: *const libc::c_char,
                                   mut c: libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = Q_strlen(s);
    s = s.offset(len as isize);
    loop  {
        let fresh18 = len;
        len = len.wrapping_sub(1);
        if !(fresh18 != 0) { break ; }
        s = s.offset(-1);
        if *s as libc::c_int == c as libc::c_int {
            return s as *mut libc::c_char
        }
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strnicmp(mut s1: *const libc::c_char,
                                    mut s2: *const libc::c_char,
                                    mut n: libc::c_int) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if s1.is_null() {
        if s2.is_null() {
            return 0 as libc::c_int
        } else { return -(1 as libc::c_int) }
    } else { if s2.is_null() { return 1 as libc::c_int } }
    loop  {
        let fresh19 = s1;
        s1 = s1.offset(1);
        c1 = *fresh19 as libc::c_int;
        let fresh20 = s2;
        s2 = s2.offset(1);
        c2 = *fresh20 as libc::c_int;
        let fresh21 = n;
        n = n - 1;
        if fresh21 == 0 { return 0 as libc::c_int }
        if c1 != c2 {
            if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
                c1 -= 'a' as i32 - 'A' as i32
            }
            if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
                c2 -= 'a' as i32 - 'A' as i32
            }
            if c1 != c2 {
                return if c1 < c2 {
                           -(1 as libc::c_int)
                       } else { 1 as libc::c_int }
            }
        }
        if !(c1 != 0) { break ; }
    }
    // strings are equal
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncmp(mut s1: *const libc::c_char,
                                   mut s2: *const libc::c_char,
                                   mut n: libc::c_int) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if s1.is_null() {
        if s2.is_null() {
            return 0 as libc::c_int
        } else { return -(1 as libc::c_int) }
    } else { if s2.is_null() { return 1 as libc::c_int } }
    loop  {
        let fresh22 = s1;
        s1 = s1.offset(1);
        c1 = *fresh22 as libc::c_int;
        let fresh23 = s2;
        s2 = s2.offset(1);
        c2 = *fresh23 as libc::c_int;
        // strings are equal until end point
        let fresh24 = n;
        n = n - 1;
        if fresh24 == 0 { return 0 as libc::c_int }
        if c1 != c2 {
            return if c1 < c2 {
                       -(1 as libc::c_int)
                   } else { 1 as libc::c_int }
        }
        if !(c1 != 0) { break ; }
    }
    // strings are equal
    return 0 as libc::c_int;
}
unsafe extern "C" fn Q_starcmp(mut pattern: *const libc::c_char,
                               mut text: *const libc::c_char) -> qboolean {
    let mut c: libc::c_char = 0;
    let mut c1: libc::c_char = 0;
    let mut p: *const libc::c_char = pattern;
    let mut t: *const libc::c_char = text;
    loop  {
        let fresh25 = p;
        p = p.offset(1);
        c = *fresh25;
        if !(c as libc::c_int == '?' as i32 || c as libc::c_int == '*' as i32)
           {
            break ;
        }
        if c as libc::c_int == '?' as i32 &&
               {
                   let fresh26 = t;
                   t = t.offset(1);
                   (*fresh26 as libc::c_int) == '\u{0}' as i32
               } {
            return false_0
        }
    }
    if c as libc::c_int == '\u{0}' as i32 { return true_0 }
    c1 =
        if c as libc::c_int == '\\' as i32 {
            *p as libc::c_int
        } else { c as libc::c_int } as libc::c_char;
    loop  {
        if Q_tolower(*t) as libc::c_int == c1 as libc::c_int &&
               Q_stricmpext(p.offset(-(1 as libc::c_int as isize)), t) as
                   libc::c_uint != 0 {
            return true_0
        }
        let fresh27 = t;
        t = t.offset(1);
        if *fresh27 as libc::c_int == '\u{0}' as i32 { return false_0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmpext(mut pattern: *const libc::c_char,
                                      mut text: *const libc::c_char)
 -> qboolean {
    let mut c: libc::c_char = 0;
    loop  {
        let fresh28 = pattern;
        pattern = pattern.offset(1);
        c = *fresh28;
        if !(c as libc::c_int != '\u{0}' as i32) { break ; }
        match c as libc::c_int {
            63 => {
                let fresh29 = text;
                text = text.offset(1);
                if *fresh29 as libc::c_int == '\u{0}' as i32 {
                    return false_0
                }
            }
            92 => {
                let fresh30 = pattern;
                pattern = pattern.offset(1);
                let fresh31 = text;
                text = text.offset(1);
                if Q_tolower(*fresh30) as libc::c_int !=
                       Q_tolower(*fresh31) as libc::c_int {
                    return false_0
                }
            }
            42 => { return Q_starcmp(pattern, text) }
            _ => {
                let fresh32 = text;
                text = text.offset(1);
                if Q_tolower(c) as libc::c_int !=
                       Q_tolower(*fresh32) as libc::c_int {
                    return false_0
                }
            }
        }
    }
    return (*text as libc::c_int == '\u{0}' as i32) as libc::c_int as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_timestamp(mut format: libc::c_int)
 -> *const libc::c_char {
    static mut timestamp: string = [0; 256];
    let mut crt_time: time_t = 0;
    let mut crt_tm: *const tm = 0 as *const tm;
    let mut timestring: string = [0; 256];
    time(&mut crt_time);
    crt_tm = localtime(&mut crt_time);
    match format {
        0 => {
            // Build the full timestamp (ex: "Apr03 2007 [23:31.55]");
            strftime(timestring.as_mut_ptr(),
                     ::std::mem::size_of::<string>() as libc::c_ulong,
                     b"%b%d %Y [%H:%M.%S]\x00" as *const u8 as
                         *const libc::c_char, crt_tm);
        }
        1 => {
            // Build the date stamp only (ex: "Apr03 2007");
            strftime(timestring.as_mut_ptr(),
                     ::std::mem::size_of::<string>() as libc::c_ulong,
                     b"%b%d %Y\x00" as *const u8 as *const libc::c_char,
                     crt_tm);
        }
        2 => {
            // Build the time stamp only (ex: "23:31.55");
            strftime(timestring.as_mut_ptr(),
                     ::std::mem::size_of::<string>() as libc::c_ulong,
                     b"%H:%M.%S\x00" as *const u8 as *const libc::c_char,
                     crt_tm);
        }
        3 => {
            // Build the time stamp exclude seconds (ex: "13:46");
            strftime(timestring.as_mut_ptr(),
                     ::std::mem::size_of::<string>() as libc::c_ulong,
                     b"%H:%M\x00" as *const u8 as *const libc::c_char,
                     crt_tm);
        }
        4 => {
            // Build the date stamp year only (ex: "2006");
            strftime(timestring.as_mut_ptr(),
                     ::std::mem::size_of::<string>() as libc::c_ulong,
                     b"%Y\x00" as *const u8 as *const libc::c_char, crt_tm);
        }
        5 => {
            // Build a timestamp that can use for filename (ex: "Nov2006-26 (19.14.28)");
            strftime(timestring.as_mut_ptr(),
                     ::std::mem::size_of::<string>() as libc::c_ulong,
                     b"%b%Y-%d_%H.%M.%S\x00" as *const u8 as
                         *const libc::c_char, crt_tm);
        }
        _ => { return 0 as *const libc::c_char }
    }
    Q_strncpy(timestamp.as_mut_ptr(), timestring.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    return timestamp.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Q_strstr(mut string: *const libc::c_char,
                                  mut string2: *const libc::c_char)
 -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut len: size_t = 0;
    if string.is_null() || string2.is_null() { return 0 as *mut libc::c_char }
    c = *string2 as libc::c_int;
    len = Q_strlen(string2);
    while !string.is_null() {
        while *string as libc::c_int != 0 && *string as libc::c_int != c {
            string = string.offset(1)
        }
        if *string != 0 {
            if Q_strncmp(string, string2, len as libc::c_int) == 0 { break ; }
            string = string.offset(1)
        } else { return 0 as *mut libc::c_char }
    }
    return string as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_stristr(mut string: *const libc::c_char,
                                   mut string2: *const libc::c_char)
 -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut len: size_t = 0;
    if string.is_null() || string2.is_null() { return 0 as *mut libc::c_char }
    c = Q_tolower(*string2) as libc::c_int;
    len = Q_strlen(string2);
    while !string.is_null() {
        while *string as libc::c_int != 0 &&
                  Q_tolower(*string) as libc::c_int != c {
            string = string.offset(1)
        }
        if *string != 0 {
            if Q_strnicmp(string, string2, len as libc::c_int) == 0 {
                break ;
            }
            string = string.offset(1)
        } else { return 0 as *mut libc::c_char }
    }
    return string as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_vsnprintf(mut buffer: *mut libc::c_char,
                                     mut buffersize: size_t,
                                     mut format: *const libc::c_char,
                                     mut args: ::std::ffi::VaList)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = vsnprintf(buffer, buffersize, format, args.as_va_list());
    if result as libc::c_ulong >= buffersize {
        *buffer.offset(buffersize.wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) as isize) =
            '\u{0}' as i32 as libc::c_char;
        return -(1 as libc::c_int)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Q_snprintf(mut buffer: *mut libc::c_char,
                                    mut buffersize: size_t,
                                    mut format: *const libc::c_char,
                                    mut args: ...) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut result: libc::c_int = 0;
    args_0 = args.clone();
    result = Q_vsnprintf(buffer, buffersize, format, args_0.as_va_list());
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Q_sprintf(mut buffer: *mut libc::c_char,
                                   mut format: *const libc::c_char,
                                   mut args: ...) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut result: libc::c_int = 0;
    args_0 = args.clone();
    result =
        Q_vsnprintf(buffer, 99999 as libc::c_int as size_t, format,
                    args_0.as_va_list());
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strpbrk(mut s: *const libc::c_char,
                                   mut accept: *const libc::c_char)
 -> *mut libc::c_char {
    while *s != 0 {
        let mut k: *const libc::c_char = 0 as *const libc::c_char;
        k = accept;
        while *k != 0 {
            if *s as libc::c_int == *k as libc::c_int {
                return s as *mut libc::c_char
            }
            k = k.offset(1)
        }
        s = s.offset(1)
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_hashkey(mut string: *const libc::c_char,
                                   mut hashSize: uint,
                                   mut caseinsensitive: qboolean) -> uint {
    let mut i: uint = 0;
    let mut hashKey: uint = 0 as libc::c_int as uint;
    if caseinsensitive as u64 != 0 {
        i = 0 as libc::c_int as uint;
        while *string.offset(i as isize) != 0 {
            hashKey =
                (hashKey as
                     libc::c_uint).wrapping_add(i.wrapping_mul(119 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_mul(Q_tolower(*string.offset(i
                                                                                                                           as
                                                                                                                           isize))
                                                                                                  as
                                                                                                  libc::c_uint))
                    as uint as uint;
            i = i.wrapping_add(1)
        }
    } else {
        i = 0 as libc::c_int as uint;
        while *string.offset(i as isize) != 0 {
            hashKey =
                (hashKey as
                     libc::c_uint).wrapping_add(i.wrapping_add(119 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_mul(*string.offset(i
                                                                                                                 as
                                                                                                                 isize)
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint))
                    as uint as uint;
            i = i.wrapping_add(1)
        }
    }
    hashKey =
        (hashKey ^ hashKey >> 10 as libc::c_int ^
             hashKey >> 20 as libc::c_int) &
            hashSize.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return hashKey;
}
#[no_mangle]
pub unsafe extern "C" fn Q_pretifymem(mut value: libc::c_float,
                                      mut digitsafterdecimal: libc::c_int)
 -> *mut libc::c_char {
    static mut output: [[libc::c_char; 32]; 8] = [[0; 32]; 8];
    static mut current: libc::c_int = 0;
    let mut onekb: libc::c_float = 1024.0f32;
    let mut onemb: libc::c_float = onekb * onekb;
    let mut suffix: [libc::c_char; 8] = [0; 8];
    let mut out: *mut libc::c_char = output[current as usize].as_mut_ptr();
    let mut val: [libc::c_char; 32] = [0; 32];
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: libc::c_int = 0;
    current =
        current + 1 as libc::c_int & 8 as libc::c_int - 1 as libc::c_int;
    // first figure out which bin to use
    if value > onemb {
        value /= onemb;
        Q_sprintf(suffix.as_mut_ptr(),
                  b" Mb\x00" as *const u8 as *const libc::c_char);
    } else if value > onekb {
        value /= onekb;
        Q_sprintf(suffix.as_mut_ptr(),
                  b" Kb\x00" as *const u8 as *const libc::c_char);
    } else {
        Q_sprintf(suffix.as_mut_ptr(),
                  b" bytes\x00" as *const u8 as *const libc::c_char);
    }
    // clamp to >= 0
    digitsafterdecimal =
        if digitsafterdecimal > 0 as libc::c_int {
            digitsafterdecimal
        } else { 0 as libc::c_int };
    // if it's basically integral, don't do any decimals
    if fabs((value - value as libc::c_int as libc::c_float) as libc::c_double)
           < 0.00001f32 as libc::c_double {
        Q_sprintf(val.as_mut_ptr(),
                  b"%i%s\x00" as *const u8 as *const libc::c_char,
                  value as libc::c_int, suffix.as_mut_ptr());
    } else {
        let mut fmt: [libc::c_char; 32] = [0; 32];
        // otherwise, create a format string for the decimals
        Q_sprintf(fmt.as_mut_ptr(),
                  b"%%.%if%s\x00" as *const u8 as *const libc::c_char,
                  digitsafterdecimal, suffix.as_mut_ptr());
        Q_sprintf(val.as_mut_ptr(), fmt.as_mut_ptr(),
                  value as libc::c_double);
    }
    // copy from in to out
    i = val.as_mut_ptr();
    o = out;
    // search for decimal or if it was integral, find the space after the raw number
    dot =
        Q_strstr(i,
                 b".\x00" as *const u8 as
                     *const libc::c_char); // compute position of dot
    if dot.is_null() {
        dot = Q_strstr(i, b" \x00" as *const u8 as *const libc::c_char)
    } // don't put a comma if it's <= 3 long
    pos = dot.wrapping_offset_from(i) as libc::c_long as libc::c_int;
    pos -= 3 as libc::c_int;
    while *i != 0 {
        // if pos is still valid then insert a comma every third digit, except if we would be
		// putting one in the first spot
        if pos >= 0 as libc::c_int && pos % 3 as libc::c_int == 0 {
            // never in first spot
            if o != out {
                let fresh33 = o;
                o = o.offset(1);
                *fresh33 = ',' as i32 as libc::c_char
            }
        }
        // copy rest of data as normal
        pos -= 1; // count down comma position
        let fresh34 = i; // terminate
        i = i.offset(1);
        let fresh35 = o;
        o = o.offset(1);
        *fresh35 = *fresh34
    }
    *o = 0 as libc::c_int as libc::c_char;
    return out;
}
/*
============
va

does a varargs printf into a temp buffer,
so I don't need to have varargs versions
of all text functions.
============
*/
#[no_mangle]
pub unsafe extern "C" fn va(mut format: *const libc::c_char, mut args: ...)
 -> *mut libc::c_char {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [[libc::c_char; 1024]; 16] = [[0; 1024]; 16];
    static mut s: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    static mut stringindex: libc::c_int = 0 as libc::c_int;
    s = string[stringindex as usize].as_mut_ptr();
    stringindex = stringindex + 1 as libc::c_int & 15 as libc::c_int;
    argptr = args.clone();
    Q_vsnprintf(s,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as
                    libc::c_ulong, format, argptr.as_va_list());
    return s;
}
/*
============
COM_FileBase

Extracts the base name of a file (no path, no extension, assumes '/' as path separator)
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_FileBase(mut in_0: *const libc::c_char,
                                      mut out: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    len = Q_strlen(in_0) as libc::c_int;
    if len == 0 { return }
    // scan backward for '.'
    end = len - 1 as libc::c_int; // found ',', copy to left of '.'
    while end != 0 && *in_0.offset(end as isize) as libc::c_int != '.' as i32
              && *in_0.offset(end as isize) as libc::c_int != '/' as i32 &&
              *in_0.offset(end as isize) as libc::c_int != '\\' as i32 {
        end -= 1
    } // no '.', copy to end
    if *in_0.offset(end as isize) as libc::c_int != '.' as i32 {
        end = len - 1 as libc::c_int
    } else { end -= 1 }
    // scan backward for '/'
    start = len - 1 as libc::c_int;
    while start >= 0 as libc::c_int &&
              *in_0.offset(start as isize) as libc::c_int != '/' as i32 &&
              *in_0.offset(start as isize) as libc::c_int != '\\' as i32 {
        start -= 1
    }
    if start < 0 as libc::c_int ||
           *in_0.offset(start as isize) as libc::c_int != '/' as i32 &&
               *in_0.offset(start as isize) as libc::c_int != '\\' as i32 {
        start = 0 as libc::c_int
    } else { start += 1 }
    // length of new sting
    len = end - start + 1 as libc::c_int;
    // Copy partial string
    Q_strncpy(out, &*in_0.offset(start as isize),
              (len + 1 as libc::c_int) as size_t);
    *out.offset(len as isize) = 0 as libc::c_int as libc::c_char;
}
/*
============
COM_FileExtension
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_FileExtension(mut in_0: *const libc::c_char)
 -> *const libc::c_char {
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut backslash: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let mut dot: *const libc::c_char = 0 as *const libc::c_char;
    separator = Q_strrchr(in_0, '/' as i32 as libc::c_char);
    backslash = Q_strrchr(in_0, '\\' as i32 as libc::c_char);
    if separator.is_null() || separator < backslash { separator = backslash }
    colon = Q_strrchr(in_0, ':' as i32 as libc::c_char);
    if separator.is_null() || separator < colon { separator = colon }
    dot = Q_strrchr(in_0, '.' as i32 as libc::c_char);
    if dot.is_null() || !separator.is_null() && dot < separator {
        return b"\x00" as *const u8 as *const libc::c_char
    }
    return dot.offset(1 as libc::c_int as isize);
}
/*
============
COM_FileWithoutPath
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_FileWithoutPath(mut in_0: *const libc::c_char)
 -> *const libc::c_char {
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut backslash: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    separator = Q_strrchr(in_0, '/' as i32 as libc::c_char);
    backslash = Q_strrchr(in_0, '\\' as i32 as libc::c_char);
    if separator.is_null() || separator < backslash { separator = backslash }
    colon = Q_strrchr(in_0, ':' as i32 as libc::c_char);
    if separator.is_null() || separator < colon { separator = colon }
    return if !separator.is_null() {
               separator.offset(1 as libc::c_int as isize)
           } else { in_0 };
}
/*
============
COM_ExtractFilePath
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_ExtractFilePath(mut path: *const libc::c_char,
                                             mut dest: *mut libc::c_char) {
    let mut src: *const libc::c_char =
        path.offset(Q_strlen(path) as
                        isize).offset(-(1 as libc::c_int as isize));
    // back up until a \ or the start
    while src != path &&
              !(*src.offset(-(1 as libc::c_int as isize)) as libc::c_int ==
                    '\\' as i32 ||
                    *src.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == '/' as i32) {
        src = src.offset(-1)
    }
    if src != path {
        memcpy(dest as *mut libc::c_void, path as *const libc::c_void,
               src.wrapping_offset_from(path) as libc::c_long as
                   libc::c_ulong);
        *dest.offset((src.wrapping_offset_from(path) as libc::c_long -
                          1 as libc::c_int as libc::c_long) as isize) =
            0 as libc::c_int as libc::c_char
        // cutoff backslash
    } else {
        Q_strncpy(dest, b"\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
    };
    // file without path
}
/*
============
COM_StripExtension
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_StripExtension(mut path: *mut libc::c_char) {
    let mut length: size_t = 0;
    length = Q_strlen(path);
    if length > 0 as libc::c_int as libc::c_ulong {
        length = length.wrapping_sub(1)
    }
    while length > 0 as libc::c_int as libc::c_ulong &&
              *path.offset(length as isize) as libc::c_int != '.' as i32 {
        length = length.wrapping_sub(1);
        if *path.offset(length as isize) as libc::c_int == '/' as i32 ||
               *path.offset(length as isize) as libc::c_int == '\\' as i32 ||
               *path.offset(length as isize) as libc::c_int == ':' as i32 {
            return
        }
        // no extension
    }
    if length != 0 {
        *path.offset(length as isize) = 0 as libc::c_int as libc::c_char
    };
}
/*
==================
COM_DefaultExtension
==================
*/
#[no_mangle]
pub unsafe extern "C" fn COM_DefaultExtension(mut path: *mut libc::c_char,
                                              mut extension:
                                                  *const libc::c_char) {
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    // if path doesn't have a .EXT, append extension
	// (extension should include the .)
    src =
        path.offset(Q_strlen(path) as
                        isize).offset(-(1 as libc::c_int as isize));
    while *src as libc::c_int != '/' as i32 &&
              src != path as *const libc::c_char {
        // it has an extension
        if *src as libc::c_int == '.' as i32 { return }
        src = src.offset(-1)
    }
    Q_strncat(path, extension, 99999 as libc::c_int as size_t);
}
/*
==================
COM_ReplaceExtension
==================
*/
#[no_mangle]
pub unsafe extern "C" fn COM_ReplaceExtension(mut path: *mut libc::c_char,
                                              mut extension:
                                                  *const libc::c_char) {
    COM_StripExtension(path);
    COM_DefaultExtension(path, extension);
}
/*
============
COM_RemoveLineFeed
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_RemoveLineFeed(mut str: *mut libc::c_char) {
    while *str as libc::c_int != '\u{0}' as i32 {
        if *str as libc::c_int == '\r' as i32 ||
               *str as libc::c_int == '\n' as i32 {
            *str = '\u{0}' as i32 as libc::c_char
        }
        str = str.offset(1)
    };
}
/*
============
COM_PathSlashFix
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_PathSlashFix(mut path: *mut libc::c_char) {
    let mut len: size_t = 0;
    len = Q_strlen(path);
    if *path.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                        isize) as libc::c_int != '\\' as i32 &&
           *path.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                            isize) as libc::c_int != '/' as i32 {
        Q_strncpy(&mut *path.offset(len as isize),
                  b"/\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
    };
}
/*
============
COM_Hex2Char
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_Hex2Char(mut hex: uint8_t) -> libc::c_char {
    if hex as libc::c_int >= 0 as libc::c_int &&
           hex as libc::c_int <= 0x9 as libc::c_int {
        hex = (hex as libc::c_int + '0' as i32) as uint8_t
    } else if hex as libc::c_int >= 0xa as libc::c_int &&
                  hex as libc::c_int <= 0xf as libc::c_int {
        hex = (hex as libc::c_int + '7' as i32) as uint8_t
    }
    return hex as libc::c_char;
}
/*
============
COM_Hex2String
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_Hex2String(mut hex: uint8_t,
                                        mut str: *mut libc::c_char) {
    let fresh36 = str;
    str = str.offset(1);
    *fresh36 =
        COM_Hex2Char((hex as libc::c_int >> 4 as libc::c_int) as uint8_t);
    let fresh37 = str;
    str = str.offset(1);
    *fresh37 =
        COM_Hex2Char((hex as libc::c_int & 0xf as libc::c_int) as uint8_t);
    *str = '\u{0}' as i32 as libc::c_char;
}
/*
==============
COM_IsSingleChar

interpert this character as single
==============
*/
unsafe extern "C" fn COM_IsSingleChar(mut flags: libc::c_uint,
                                      mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int == '{' as i32 || c as libc::c_int == '}' as i32 ||
           c as libc::c_int == '\'' as i32 || c as libc::c_int == ',' as i32 {
        return true_0 as libc::c_int
    }
    if flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint == 0
           &&
           (c as libc::c_int == ')' as i32 || c as libc::c_int == '(' as i32)
       {
        return true_0 as libc::c_int
    }
    if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
           && c as libc::c_int == ':' as i32 {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
/*
==============
COM_ParseFile

text parser
==============
*/
#[no_mangle]
pub unsafe extern "C" fn _COM_ParseFileSafe(mut data: *mut libc::c_char,
                                            mut token: *mut libc::c_char,
                                            size: libc::c_int,
                                            mut flags: libc::c_uint,
                                            mut plen: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut overflow: qboolean = false_0;
    if token.is_null() || size == 0 {
        if !plen.is_null() { *plen = 0 as libc::c_int }
        return 0 as *mut libc::c_char
    }
    *token.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    if data.is_null() { return 0 as *mut libc::c_char }
    loop 
         // skip whitespace
         {
        loop  {
            c = *data as byte as libc::c_int;
            if !(c <= ' ' as i32) { break ; }
            if c == 0 as libc::c_int {
                if !plen.is_null() {
                    *plen =
                        if overflow as libc::c_uint != 0 {
                            -(1 as libc::c_int)
                        } else { len }
                }
                return 0 as *mut libc::c_char
                // end of file;
            }
            data = data.offset(1)
        }
        // skip // comments
        if !(c == '/' as i32 &&
                 *data.offset(1 as libc::c_int as isize) as libc::c_int ==
                     '/' as i32) {
            break ;
        }
        while *data as libc::c_int != 0 && *data as libc::c_int != '\n' as i32
              {
            data = data.offset(1)
        }
    }
    // handle quoted strings specially
    if c == '\"' as i32 {
        data = data.offset(1);
        loop  {
            c = *data as byte as libc::c_int;
            // unexpected line end
            if c == 0 {
                *token.offset(len as isize) =
                    0 as libc::c_int as libc::c_char;
                if !plen.is_null() {
                    *plen =
                        if overflow as libc::c_uint != 0 {
                            -(1 as libc::c_int)
                        } else { len }
                }
                return data
            }
            data = data.offset(1);
            if c == '\\' as i32 && *data as libc::c_int == '\"' as i32 {
                if (len + 1 as libc::c_int) < size {
                    *token.offset(len as isize) =
                        *data as byte as libc::c_char;
                    len += 1
                } else { overflow = true_0 }
                data = data.offset(1)
            } else {
                if c == '\"' as i32 {
                    *token.offset(len as isize) =
                        0 as libc::c_int as libc::c_char;
                    if !plen.is_null() {
                        *plen =
                            if overflow as libc::c_uint != 0 {
                                -(1 as libc::c_int)
                            } else { len }
                    }
                    return data
                }
                if (len + 1 as libc::c_int) < size {
                    *token.offset(len as isize) = c as libc::c_char;
                    len += 1
                } else { overflow = true_0 }
            }
        }
    }
    // parse single characters
    if COM_IsSingleChar(flags, c as libc::c_char) != 0 {
        if size >= 2 as libc::c_int {
            // char and \0
            *token.offset(len as isize) = c as libc::c_char;
            len += 1;
            *token.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            if !plen.is_null() {
                *plen =
                    if overflow as libc::c_uint != 0 {
                        -(1 as libc::c_int)
                    } else { len }
            }
            return data.offset(1 as libc::c_int as isize)
        } else {
            // couldn't pass anything
            *token.offset(0 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char;
            if !plen.is_null() {
                *plen =
                    if overflow as libc::c_uint != 0 {
                        -(1 as libc::c_int)
                    } else { len }
            }
            return data
        }
    }
    loop 
         // parse a regular word
         {
        if (len + 1 as libc::c_int) < size {
            *token.offset(len as isize) = c as libc::c_char;
            len += 1
        } else { overflow = true_0 }
        data = data.offset(1);
        c = *data as byte as libc::c_int;
        if COM_IsSingleChar(flags, c as libc::c_char) != 0 { break ; }
        if !(c > 32 as libc::c_int) { break ; }
    }
    *token.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    if !plen.is_null() {
        *plen =
            if overflow as libc::c_uint != 0 {
                -(1 as libc::c_int)
            } else { len }
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn matchpattern(mut in_0: *const libc::c_char,
                                      mut pattern: *const libc::c_char,
                                      mut caseinsensitive: qboolean)
 -> libc::c_int {
    return matchpattern_with_separator(in_0, pattern, caseinsensitive,
                                       b"/\\:\x00" as *const u8 as
                                           *const libc::c_char, false_0);
}
// wildcard_least_one: if true * matches 1 or more characters
//                     if false * matches 0 or more characters
#[no_mangle]
pub unsafe extern "C" fn matchpattern_with_separator(mut in_0:
                                                         *const libc::c_char,
                                                     mut pattern:
                                                         *const libc::c_char,
                                                     mut caseinsensitive:
                                                         qboolean,
                                                     mut separators:
                                                         *const libc::c_char,
                                                     mut wildcard_least_one:
                                                         qboolean)
 -> libc::c_int {
    let mut c1: libc::c_int = 0; // end of pattern
    let mut c2: libc::c_int = 0; // no match
    while *pattern != 0 {
        match *pattern as libc::c_int {
            0 => { return 1 as libc::c_int }
            63 => {
                // match any single character
                if *in_0 as libc::c_int == 0 as libc::c_int ||
                       !Q_strchr(separators, *in_0).is_null() {
                    return 0 as libc::c_int
                }
                in_0 = in_0.offset(1);
                pattern = pattern.offset(1)
            }
            42 => {
                // match anything until following string
                if wildcard_least_one as u64 != 0 {
                    if *in_0 as libc::c_int == 0 as libc::c_int ||
                           !Q_strchr(separators, *in_0).is_null() {
                        return 0 as libc::c_int
                    } // no match
                    in_0 = in_0.offset(1)
                }
                pattern = pattern.offset(1);
                while *in_0 != 0 {
                    if !Q_strchr(separators, *in_0).is_null() { break ; }
                    // see if pattern matches at this offset
                    if matchpattern_with_separator(in_0, pattern,
                                                   caseinsensitive,
                                                   separators,
                                                   wildcard_least_one) != 0 {
                        return 1 as libc::c_int
                    }
                    // nope, advance to next offset
                    in_0 = in_0.offset(1)
                }
            }
            _ => {
                if *in_0 as libc::c_int != *pattern as libc::c_int {
                    if caseinsensitive as u64 == 0 { return 0 as libc::c_int }
                    c1 = *in_0 as libc::c_int;
                    if c1 >= 'A' as i32 && c1 <= 'Z' as i32 {
                        c1 += 'a' as i32 - 'A' as i32
                    }
                    c2 = *pattern as libc::c_int;
                    if c2 >= 'A' as i32 && c2 <= 'Z' as i32 {
                        c2 += 'a' as i32 - 'A' as i32
                    }
                    if c1 != c2 { return 0 as libc::c_int } // no match
                    // no match
                } // reached end of pattern but not end of input
                in_0 = in_0.offset(1);
                pattern = pattern.offset(1)
            }
        }
    }
    if *in_0 != 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
    // success
}
