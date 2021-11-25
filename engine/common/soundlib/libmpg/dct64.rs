#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut pnts: [*mut libc::c_float; 0];
}
/*
	dct64.c: DCT64, the plain C version

	copyright ?-2006 by the mpg123 project - free software under the terms of the LGPL 2.1
	see COPYING and AUTHORS files in distribution or http://mpg123.org
	initially written by Michael Hipp
*/
/*
 * Discrete Cosine Transform (DCT) for subband synthesis
 *
 * -funroll-loops (for gcc) will remove the loops for better performance
 * using loops in the source-code enhances readabillity
 *
 */
#[no_mangle]
pub unsafe extern "C" fn dct64(mut out0: *mut libc::c_float,
                               mut out1: *mut libc::c_float,
                               mut samples: *mut libc::c_float) {
    let mut bufs: [libc::c_float; 64] = [0.; 64];
    let mut b1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut b2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut bs: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut costab: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    b1 = samples;
    bs = bufs.as_mut_ptr();
    costab =
        (*pnts.as_mut_ptr().offset(0 as libc::c_int as
                                       isize)).offset(16 as libc::c_int as
                                                          isize);
    b2 = b1.offset(32 as libc::c_int as isize);
    i = 15 as libc::c_int;
    while i >= 0 as libc::c_int {
        let fresh0 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh1 = bs;
        bs = bs.offset(1);
        *fresh1 = *fresh0 + *b2;
        i -= 1
    }
    i = 15 as libc::c_int;
    while i >= 0 as libc::c_int {
        b2 = b2.offset(-1);
        let fresh2 = b1;
        b1 = b1.offset(1);
        costab = costab.offset(-1);
        let fresh3 = bs;
        bs = bs.offset(1);
        *fresh3 = (*b2 - *fresh2) * *costab;
        i -= 1
    }
    b1 = bufs.as_mut_ptr();
    costab =
        (*pnts.as_mut_ptr().offset(1 as libc::c_int as
                                       isize)).offset(8 as libc::c_int as
                                                          isize);
    b2 = b1.offset(16 as libc::c_int as isize);
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        let fresh4 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh5 = bs;
        bs = bs.offset(1);
        *fresh5 = *fresh4 + *b2;
        i -= 1
    }
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        b2 = b2.offset(-1);
        let fresh6 = b1;
        b1 = b1.offset(1);
        costab = costab.offset(-1);
        let fresh7 = bs;
        bs = bs.offset(1);
        *fresh7 = (*b2 - *fresh6) * *costab;
        i -= 1
    }
    b2 = b2.offset(32 as libc::c_int as isize);
    costab = costab.offset(8 as libc::c_int as isize);
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        let fresh8 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh9 = bs;
        bs = bs.offset(1);
        *fresh9 = *fresh8 + *b2;
        i -= 1
    }
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        let fresh10 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        costab = costab.offset(-1);
        let fresh11 = bs;
        bs = bs.offset(1);
        *fresh11 = (*fresh10 - *b2) * *costab;
        i -= 1
    }
    b2 = b2.offset(32 as libc::c_int as isize);
    bs = bufs.as_mut_ptr();
    costab = *pnts.as_mut_ptr().offset(2 as libc::c_int as isize);
    b2 = b1.offset(8 as libc::c_int as isize);
    j = 2 as libc::c_int;
    while j != 0 {
        i = 3 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh12 = b1;
            b1 = b1.offset(1);
            b2 = b2.offset(-1);
            let fresh13 = bs;
            bs = bs.offset(1);
            *fresh13 = *fresh12 + *b2;
            i -= 1
        }
        i = 3 as libc::c_int;
        while i >= 0 as libc::c_int {
            b2 = b2.offset(-1);
            let fresh14 = b1;
            b1 = b1.offset(1);
            let fresh15 = bs;
            bs = bs.offset(1);
            *fresh15 = (*b2 - *fresh14) * *costab.offset(i as isize);
            i -= 1
        }
        b2 = b2.offset(16 as libc::c_int as isize);
        i = 3 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh16 = b1;
            b1 = b1.offset(1);
            b2 = b2.offset(-1);
            let fresh17 = bs;
            bs = bs.offset(1);
            *fresh17 = *fresh16 + *b2;
            i -= 1
        }
        i = 3 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh18 = b1;
            b1 = b1.offset(1);
            b2 = b2.offset(-1);
            let fresh19 = bs;
            bs = bs.offset(1);
            *fresh19 = (*fresh18 - *b2) * *costab.offset(i as isize);
            i -= 1
        }
        b2 = b2.offset(16 as libc::c_int as isize);
        j -= 1
    }
    b1 = bufs.as_mut_ptr();
    costab = *pnts.as_mut_ptr().offset(3 as libc::c_int as isize);
    b2 = b1.offset(4 as libc::c_int as isize);
    j = 4 as libc::c_int;
    while j != 0 {
        let fresh20 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh21 = bs;
        bs = bs.offset(1);
        *fresh21 = *fresh20 + *b2;
        let fresh22 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh23 = bs;
        bs = bs.offset(1);
        *fresh23 = *fresh22 + *b2;
        b2 = b2.offset(-1);
        let fresh24 = b1;
        b1 = b1.offset(1);
        let fresh25 = bs;
        bs = bs.offset(1);
        *fresh25 =
            (*b2 - *fresh24) * *costab.offset(1 as libc::c_int as isize);
        b2 = b2.offset(-1);
        let fresh26 = b1;
        b1 = b1.offset(1);
        let fresh27 = bs;
        bs = bs.offset(1);
        *fresh27 =
            (*b2 - *fresh26) * *costab.offset(0 as libc::c_int as isize);
        b2 = b2.offset(8 as libc::c_int as isize);
        let fresh28 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh29 = bs;
        bs = bs.offset(1);
        *fresh29 = *fresh28 + *b2;
        let fresh30 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh31 = bs;
        bs = bs.offset(1);
        *fresh31 = *fresh30 + *b2;
        let fresh32 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh33 = bs;
        bs = bs.offset(1);
        *fresh33 =
            (*fresh32 - *b2) * *costab.offset(1 as libc::c_int as isize);
        let fresh34 = b1;
        b1 = b1.offset(1);
        b2 = b2.offset(-1);
        let fresh35 = bs;
        bs = bs.offset(1);
        *fresh35 =
            (*fresh34 - *b2) * *costab.offset(0 as libc::c_int as isize);
        b2 = b2.offset(8 as libc::c_int as isize);
        j -= 1
    }
    bs = bufs.as_mut_ptr();
    costab = *pnts.as_mut_ptr().offset(4 as libc::c_int as isize);
    j = 8 as libc::c_int;
    while j != 0 {
        let mut v0: libc::c_float = 0.;
        let mut v1: libc::c_float = 0.;
        let fresh36 = b1;
        b1 = b1.offset(1);
        v0 = *fresh36;
        let fresh37 = b1;
        b1 = b1.offset(1);
        v1 = *fresh37;
        let fresh38 = bs;
        bs = bs.offset(1);
        *fresh38 = v0 + v1;
        let fresh39 = bs;
        bs = bs.offset(1);
        *fresh39 = (v0 - v1) * *costab;
        let fresh40 = b1;
        b1 = b1.offset(1);
        v0 = *fresh40;
        let fresh41 = b1;
        b1 = b1.offset(1);
        v1 = *fresh41;
        let fresh42 = bs;
        bs = bs.offset(1);
        *fresh42 = v0 + v1;
        let fresh43 = bs;
        bs = bs.offset(1);
        *fresh43 = (v1 - v0) * *costab;
        j -= 1
    }
    let mut b1_0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i_0: libc::c_int = 0;
    b1_0 = bufs.as_mut_ptr();
    i_0 = 8 as libc::c_int;
    while i_0 != 0 {
        *b1_0.offset(2 as libc::c_int as isize) +=
            *b1_0.offset(3 as libc::c_int as isize);
        i_0 -= 1;
        b1_0 = b1_0.offset(4 as libc::c_int as isize)
    }
    b1_0 = bufs.as_mut_ptr();
    i_0 = 4 as libc::c_int;
    while i_0 != 0 {
        *b1_0.offset(4 as libc::c_int as isize) +=
            *b1_0.offset(6 as libc::c_int as isize);
        *b1_0.offset(6 as libc::c_int as isize) +=
            *b1_0.offset(5 as libc::c_int as isize);
        *b1_0.offset(5 as libc::c_int as isize) +=
            *b1_0.offset(7 as libc::c_int as isize);
        i_0 -= 1;
        b1_0 = b1_0.offset(8 as libc::c_int as isize)
    }
    b1_0 = bufs.as_mut_ptr();
    i_0 = 2 as libc::c_int;
    while i_0 != 0 {
        *b1_0.offset(8 as libc::c_int as isize) +=
            *b1_0.offset(12 as libc::c_int as isize);
        *b1_0.offset(12 as libc::c_int as isize) +=
            *b1_0.offset(10 as libc::c_int as isize);
        *b1_0.offset(10 as libc::c_int as isize) +=
            *b1_0.offset(14 as libc::c_int as isize);
        *b1_0.offset(14 as libc::c_int as isize) +=
            *b1_0.offset(9 as libc::c_int as isize);
        *b1_0.offset(9 as libc::c_int as isize) +=
            *b1_0.offset(13 as libc::c_int as isize);
        *b1_0.offset(13 as libc::c_int as isize) +=
            *b1_0.offset(11 as libc::c_int as isize);
        *b1_0.offset(11 as libc::c_int as isize) +=
            *b1_0.offset(15 as libc::c_int as isize);
        i_0 -= 1;
        b1_0 = b1_0.offset(16 as libc::c_int as isize)
    }
    *out0.offset((0x10 as libc::c_int * 16 as libc::c_int) as isize) =
        bufs[0 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 15 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 0 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 8 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 14 as libc::c_int) as isize) =
        bufs[8 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 13 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 8 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 4 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 12 as libc::c_int) as isize) =
        bufs[4 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 11 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 4 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 12 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 10 as libc::c_int) as isize) =
        bufs[12 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 9 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 12 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 2 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 8 as libc::c_int) as isize) =
        bufs[2 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 7 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 2 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 10 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 6 as libc::c_int) as isize) =
        bufs[10 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 5 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 10 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 6 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 4 as libc::c_int) as isize) =
        bufs[6 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 3 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 6 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 14 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 2 as libc::c_int) as isize) =
        bufs[14 as libc::c_int as usize];
    *out0.offset((0x10 as libc::c_int * 1 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 14 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 1 as libc::c_int) as usize];
    *out0.offset((0x10 as libc::c_int * 0 as libc::c_int) as isize) =
        bufs[1 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 0 as libc::c_int) as isize) =
        bufs[1 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 1 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 1 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 9 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 2 as libc::c_int) as isize) =
        bufs[9 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 3 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 9 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 5 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 4 as libc::c_int) as isize) =
        bufs[5 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 5 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 5 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 13 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 6 as libc::c_int) as isize) =
        bufs[13 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 7 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 13 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 3 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 8 as libc::c_int) as isize) =
        bufs[3 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 9 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 3 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 11 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 10 as libc::c_int) as isize) =
        bufs[11 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 11 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 11 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 7 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 12 as libc::c_int) as isize) =
        bufs[7 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 13 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 7 as libc::c_int) as usize] +
            bufs[(16 as libc::c_int + 15 as libc::c_int) as usize];
    *out1.offset((0x10 as libc::c_int * 14 as libc::c_int) as isize) =
        bufs[15 as libc::c_int as usize];
    *out1.offset((0x10 as libc::c_int * 15 as libc::c_int) as isize) =
        bufs[(16 as libc::c_int + 15 as libc::c_int) as usize];
}
