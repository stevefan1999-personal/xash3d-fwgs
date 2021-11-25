#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type mpg_off_t = libc::c_long;
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
/*
index.c - compact version of famous library mpg123
Copyright (C) 2017 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// the next expected frame offset, one step ahead.
unsafe extern "C" fn fi_next(mut fi: *mut frame_index_t) -> mpg_off_t {
    return (*fi).fill as mpg_off_t * (*fi).step;
}
// shrink down the used index to the half.
// be careful with size = 1 ... there's no shrinking possible there.
unsafe extern "C" fn fi_shrink(mut fi: *mut frame_index_t) {
    if (*fi).fill < 2 as libc::c_int as libc::c_ulong {
        return
        // won't shrink below 1.
    } else {
        let mut c: size_t = 0;
        // double the step, half the fill. Should work as well for fill%2 = 1
        (*fi).step *= 2 as libc::c_int as libc::c_long;
        (*fi).fill =
            ((*fi).fill as
                 libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        // move the data down.
        c = 0 as libc::c_int as size_t;
        while c < (*fi).fill {
            *(*fi).data.offset(c as isize) =
                *(*fi).data.offset((2 as libc::c_int as
                                        libc::c_ulong).wrapping_mul(c) as
                                       isize);
            c = c.wrapping_add(1)
        }
    }
    (*fi).next = fi_next(fi);
}
#[no_mangle]
pub unsafe extern "C" fn fi_init(mut fi: *mut frame_index_t) {
    (*fi).data = 0 as *mut mpg_off_t;
    (*fi).step = 1 as libc::c_int as mpg_off_t;
    (*fi).fill = 0 as libc::c_int as size_t;
    (*fi).size = 0 as libc::c_int as size_t;
    (*fi).grow_size = 0 as libc::c_int as size_t;
    (*fi).next = fi_next(fi);
}
#[no_mangle]
pub unsafe extern "C" fn fi_exit(mut fi: *mut frame_index_t) {
    if (*fi).size != 0 && !(*fi).data.is_null() {
        free((*fi).data as *mut libc::c_void);
    }
    fi_init(fi);
    // be prepared for further fun, still.
}
#[no_mangle]
pub unsafe extern "C" fn fi_resize(mut fi: *mut frame_index_t,
                                   mut newsize: size_t) -> libc::c_int {
    let mut newdata: *mut mpg_off_t = 0 as *mut mpg_off_t;
    if newsize == (*fi).size { return 0 as libc::c_int }
    if newsize > 0 as libc::c_int as libc::c_ulong && newsize < (*fi).size {
        // when we reduce buffer size a bit, shrink stuff.
        while (*fi).fill > newsize { fi_shrink(fi); }
    }
    newdata =
        realloc((*fi).data as *mut libc::c_void,
                newsize.wrapping_mul(::std::mem::size_of::<mpg_off_t>() as
                                         libc::c_ulong)) as *mut mpg_off_t;
    if newsize == 0 as libc::c_int as libc::c_ulong || !newdata.is_null() {
        (*fi).data = newdata;
        (*fi).size = newsize;
        if (*fi).fill > (*fi).size { (*fi).fill = (*fi).size }
        (*fi).next = fi_next(fi);
        return 0 as libc::c_int
    } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn fi_add(mut fi: *mut frame_index_t,
                                mut pos: mpg_off_t) {
    if (*fi).fill == (*fi).size {
        let mut framenum: mpg_off_t =
            (*fi).fill.wrapping_mul((*fi).step as libc::c_ulong) as mpg_off_t;
        // index is full, we need to shrink... or grow.
		// store the current frame number to check later if we still want it.
        // if we want not / cannot grow, we shrink.
        if !((*fi).grow_size != 0 &&
                 fi_resize(fi, (*fi).size.wrapping_add((*fi).grow_size)) ==
                     0 as libc::c_int) {
            fi_shrink(fi);
        }
        // now check if we still want to add this frame (could be that not, because of changed step).
        if (*fi).next != framenum { return }
    }
    // when we are here, we want that frame.
    if (*fi).fill < (*fi).size {
        // safeguard for size = 1, or just generally
        *(*fi).data.offset((*fi).fill as isize) = pos;
        (*fi).fill = (*fi).fill.wrapping_add(1);
        (*fi).next = fi_next(fi)
    };
}
#[no_mangle]
pub unsafe extern "C" fn fi_set(mut fi: *mut frame_index_t,
                                mut offsets: *mut mpg_off_t,
                                mut step: mpg_off_t, mut fill: size_t)
 -> libc::c_int {
    if fi_resize(fi, fill) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    (*fi).step = step;
    if !offsets.is_null() {
        memcpy((*fi).data as *mut libc::c_void,
               offsets as *const libc::c_void,
               fill.wrapping_mul(::std::mem::size_of::<mpg_off_t>() as
                                     libc::c_ulong));
        (*fi).fill = fill
    } else {
        // allocation only, no entries in index yet
        (*fi).fill = 0 as libc::c_int as size_t
    }
    (*fi).next = fi_next(fi);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fi_reset(mut fi: *mut frame_index_t) {
    (*fi).fill = 0 as libc::c_int as size_t;
    (*fi).step = 1 as libc::c_int as mpg_off_t;
    (*fi).next = fi_next(fi);
}
