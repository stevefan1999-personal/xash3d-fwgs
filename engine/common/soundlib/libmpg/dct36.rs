#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut tfcos36: [libc::c_float; 9];
    #[no_mangle]
    static mut COS6_1: libc::c_float;
    #[no_mangle]
    static mut COS6_2: libc::c_float;
    #[no_mangle]
    static mut cos9: [libc::c_float; 3];
    #[no_mangle]
    static mut cos18: [libc::c_float; 3];
    #[no_mangle]
    static mut tfcos12: [libc::c_float; 3];
}
/*
	This is an optimized DCT from Jeff Tsay's maplay 1.2+ package.
	Saved one multiplication by doing the 'twiddle factor' stuff
	together with the window mul. (MH)

	This uses Byeong Gi Lee's Fast Cosine Transform algorithm, but the
	9 point IDCT needs to be reduced further. Unfortunately, I don't
	know how to do that, because 9 is not an even number. - Jeff.

	Original Message:

	9 Point Inverse Discrete Cosine Transform

	This piece of code is Copyright 1997 Mikko Tommila and is freely usable
	by anybody. The algorithm itself is of course in the public domain.

	Again derived heuristically from the 9-point WFTA.

	The algorithm is optimized (?) for speed, not for small rounding errors or
	good readability.

	36 additions, 11 multiplications

	Again this is very likely sub-optimal.

	The code is optimized to use a minimum number of temporary variables,
	so it should compile quite well even on 8-register Intel x86 processors.
	This makes the code quite obfuscated and very difficult to understand.

	References:
	[1] S. Winograd: "On Computing the Discrete Fourier Transform",
	    Mathematics of Computation, Volume 32, Number 141, January 1978,
	    Pages 175-199
*/
// calculation of the inverse MDCT
// used to be static without 3dnow - does that floatly matter?
#[no_mangle]
pub unsafe extern "C" fn dct36(mut inbuf: *mut libc::c_float,
                               mut o1: *mut libc::c_float,
                               mut o2: *mut libc::c_float,
                               mut wintab: *mut libc::c_float,
                               mut tsbuf: *mut libc::c_float) {
    let mut tmp: [libc::c_float; 18] = [0.; 18];
    let mut in_0: *mut libc::c_float = inbuf;
    *in_0.offset(17 as libc::c_int as isize) +=
        *in_0.offset(16 as libc::c_int as isize);
    *in_0.offset(16 as libc::c_int as isize) +=
        *in_0.offset(15 as libc::c_int as isize);
    *in_0.offset(15 as libc::c_int as isize) +=
        *in_0.offset(14 as libc::c_int as isize);
    *in_0.offset(14 as libc::c_int as isize) +=
        *in_0.offset(13 as libc::c_int as isize);
    *in_0.offset(13 as libc::c_int as isize) +=
        *in_0.offset(12 as libc::c_int as isize);
    *in_0.offset(12 as libc::c_int as isize) +=
        *in_0.offset(11 as libc::c_int as isize);
    *in_0.offset(11 as libc::c_int as isize) +=
        *in_0.offset(10 as libc::c_int as isize);
    *in_0.offset(10 as libc::c_int as isize) +=
        *in_0.offset(9 as libc::c_int as isize);
    *in_0.offset(9 as libc::c_int as isize) +=
        *in_0.offset(8 as libc::c_int as isize);
    *in_0.offset(8 as libc::c_int as isize) +=
        *in_0.offset(7 as libc::c_int as isize);
    *in_0.offset(7 as libc::c_int as isize) +=
        *in_0.offset(6 as libc::c_int as isize);
    *in_0.offset(6 as libc::c_int as isize) +=
        *in_0.offset(5 as libc::c_int as isize);
    *in_0.offset(5 as libc::c_int as isize) +=
        *in_0.offset(4 as libc::c_int as isize);
    *in_0.offset(4 as libc::c_int as isize) +=
        *in_0.offset(3 as libc::c_int as isize);
    *in_0.offset(3 as libc::c_int as isize) +=
        *in_0.offset(2 as libc::c_int as isize);
    *in_0.offset(2 as libc::c_int as isize) +=
        *in_0.offset(1 as libc::c_int as isize);
    *in_0.offset(1 as libc::c_int as isize) +=
        *in_0.offset(0 as libc::c_int as isize);
    *in_0.offset(17 as libc::c_int as isize) +=
        *in_0.offset(15 as libc::c_int as isize);
    *in_0.offset(15 as libc::c_int as isize) +=
        *in_0.offset(13 as libc::c_int as isize);
    *in_0.offset(13 as libc::c_int as isize) +=
        *in_0.offset(11 as libc::c_int as isize);
    *in_0.offset(11 as libc::c_int as isize) +=
        *in_0.offset(9 as libc::c_int as isize);
    *in_0.offset(9 as libc::c_int as isize) +=
        *in_0.offset(7 as libc::c_int as isize);
    *in_0.offset(7 as libc::c_int as isize) +=
        *in_0.offset(5 as libc::c_int as isize);
    *in_0.offset(5 as libc::c_int as isize) +=
        *in_0.offset(3 as libc::c_int as isize);
    *in_0.offset(3 as libc::c_int as isize) +=
        *in_0.offset(1 as libc::c_int as isize);
    let mut t3: libc::c_float = 0.;
    let mut t0: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    t0 =
        COS6_2 *
            (*in_0.offset(8 as libc::c_int as isize) +
                 *in_0.offset(16 as libc::c_int as isize) -
                 *in_0.offset(4 as libc::c_int as isize));
    t1 = COS6_2 * *in_0.offset(12 as libc::c_int as isize);
    t3 = *in_0.offset(0 as libc::c_int as isize);
    t2 = t3 - t1 - t1;
    tmp[7 as libc::c_int as usize] = t2 - t0;
    tmp[1 as libc::c_int as usize] = tmp[7 as libc::c_int as usize];
    tmp[4 as libc::c_int as usize] = t2 + t0 + t0;
    t3 += t1;
    t2 =
        COS6_1 *
            (*in_0.offset(10 as libc::c_int as isize) +
                 *in_0.offset(14 as libc::c_int as isize) -
                 *in_0.offset(2 as libc::c_int as isize));
    tmp[1 as libc::c_int as usize] -= t2;
    tmp[7 as libc::c_int as usize] += t2;
    let mut t0_0: libc::c_float = 0.;
    let mut t1_0: libc::c_float = 0.;
    let mut t2_0: libc::c_float = 0.;
    t0_0 =
        cos9[0 as libc::c_int as usize] *
            (*in_0.offset(4 as libc::c_int as isize) +
                 *in_0.offset(8 as libc::c_int as isize));
    t1_0 =
        cos9[1 as libc::c_int as usize] *
            (*in_0.offset(8 as libc::c_int as isize) -
                 *in_0.offset(16 as libc::c_int as isize));
    t2_0 =
        cos9[2 as libc::c_int as usize] *
            (*in_0.offset(4 as libc::c_int as isize) +
                 *in_0.offset(16 as libc::c_int as isize));
    tmp[6 as libc::c_int as usize] = t3 - t0_0 - t2_0;
    tmp[2 as libc::c_int as usize] = tmp[6 as libc::c_int as usize];
    tmp[8 as libc::c_int as usize] = t3 + t0_0 + t1_0;
    tmp[0 as libc::c_int as usize] = tmp[8 as libc::c_int as usize];
    tmp[5 as libc::c_int as usize] = t3 - t1_0 + t2_0;
    tmp[3 as libc::c_int as usize] = tmp[5 as libc::c_int as usize];
    let mut t1_1: libc::c_float = 0.;
    let mut t2_1: libc::c_float = 0.;
    let mut t3_0: libc::c_float = 0.;
    t1_1 =
        cos18[0 as libc::c_int as usize] *
            (*in_0.offset(2 as libc::c_int as isize) +
                 *in_0.offset(10 as libc::c_int as isize));
    t2_1 =
        cos18[1 as libc::c_int as usize] *
            (*in_0.offset(10 as libc::c_int as isize) -
                 *in_0.offset(14 as libc::c_int as isize));
    t3_0 = COS6_1 * *in_0.offset(6 as libc::c_int as isize);
    let mut t0_1: libc::c_float = t1_1 + t2_1 + t3_0;
    tmp[0 as libc::c_int as usize] += t0_1;
    tmp[8 as libc::c_int as usize] -= t0_1;
    t2_1 -= t3_0;
    t1_1 -= t3_0;
    t3_0 =
        cos18[2 as libc::c_int as usize] *
            (*in_0.offset(2 as libc::c_int as isize) +
                 *in_0.offset(14 as libc::c_int as isize));
    t1_1 += t3_0;
    tmp[3 as libc::c_int as usize] += t1_1;
    tmp[5 as libc::c_int as usize] -= t1_1;
    t2_1 -= t3_0;
    tmp[2 as libc::c_int as usize] += t2_1;
    tmp[6 as libc::c_int as usize] -= t2_1;
    let mut t0_2: libc::c_float = 0.;
    let mut t1_2: libc::c_float = 0.;
    let mut t2_2: libc::c_float = 0.;
    let mut t3_1: libc::c_float = 0.;
    let mut t4: libc::c_float = 0.;
    let mut t5: libc::c_float = 0.;
    let mut t6: libc::c_float = 0.;
    let mut t7: libc::c_float = 0.;
    t1_2 = COS6_2 * *in_0.offset(13 as libc::c_int as isize);
    t2_2 =
        COS6_2 *
            (*in_0.offset(9 as libc::c_int as isize) +
                 *in_0.offset(17 as libc::c_int as isize) -
                 *in_0.offset(5 as libc::c_int as isize));
    t3_1 = *in_0.offset(1 as libc::c_int as isize) + t1_2;
    t4 = *in_0.offset(1 as libc::c_int as isize) - t1_2 - t1_2;
    t5 = t4 - t2_2;
    t0_2 =
        cos9[0 as libc::c_int as usize] *
            (*in_0.offset(5 as libc::c_int as isize) +
                 *in_0.offset(9 as libc::c_int as isize));
    t1_2 =
        cos9[1 as libc::c_int as usize] *
            (*in_0.offset(9 as libc::c_int as isize) -
                 *in_0.offset(17 as libc::c_int as isize));
    tmp[13 as libc::c_int as usize] =
        (t4 + t2_2 + t2_2) *
            tfcos36[(17 as libc::c_int - 13 as libc::c_int) as usize];
    t2_2 =
        cos9[2 as libc::c_int as usize] *
            (*in_0.offset(5 as libc::c_int as isize) +
                 *in_0.offset(17 as libc::c_int as isize));
    t6 = t3_1 - t0_2 - t2_2;
    t0_2 += t3_1 + t1_2;
    t3_1 += t2_2 - t1_2;
    t2_2 =
        cos18[0 as libc::c_int as usize] *
            (*in_0.offset(3 as libc::c_int as isize) +
                 *in_0.offset(11 as libc::c_int as isize));
    t4 =
        cos18[1 as libc::c_int as usize] *
            (*in_0.offset(11 as libc::c_int as isize) -
                 *in_0.offset(15 as libc::c_int as isize));
    t7 = COS6_1 * *in_0.offset(7 as libc::c_int as isize);
    t1_2 = t2_2 + t4 + t7;
    tmp[17 as libc::c_int as usize] =
        (t0_2 + t1_2) *
            tfcos36[(17 as libc::c_int - 17 as libc::c_int) as usize];
    tmp[9 as libc::c_int as usize] =
        (t0_2 - t1_2) *
            tfcos36[(17 as libc::c_int - 9 as libc::c_int) as usize];
    t1_2 =
        cos18[2 as libc::c_int as usize] *
            (*in_0.offset(3 as libc::c_int as isize) +
                 *in_0.offset(15 as libc::c_int as isize));
    t2_2 += t1_2 - t7;
    tmp[14 as libc::c_int as usize] =
        (t3_1 + t2_2) *
            tfcos36[(17 as libc::c_int - 14 as libc::c_int) as usize];
    t0_2 =
        COS6_1 *
            (*in_0.offset(11 as libc::c_int as isize) +
                 *in_0.offset(15 as libc::c_int as isize) -
                 *in_0.offset(3 as libc::c_int as isize));
    tmp[12 as libc::c_int as usize] =
        (t3_1 - t2_2) *
            tfcos36[(17 as libc::c_int - 12 as libc::c_int) as usize];
    t4 -= t1_2 + t7;
    tmp[16 as libc::c_int as usize] =
        (t5 - t0_2) *
            tfcos36[(17 as libc::c_int - 16 as libc::c_int) as usize];
    tmp[10 as libc::c_int as usize] =
        (t5 + t0_2) *
            tfcos36[(17 as libc::c_int - 10 as libc::c_int) as usize];
    tmp[15 as libc::c_int as usize] =
        (t6 + t4) * tfcos36[(17 as libc::c_int - 15 as libc::c_int) as usize];
    tmp[11 as libc::c_int as usize] =
        (t6 - t4) * tfcos36[(17 as libc::c_int - 11 as libc::c_int) as usize];
    let mut out2: *mut libc::c_float = o2;
    let mut w: *mut libc::c_float = wintab;
    let mut out1: *mut libc::c_float = o1;
    let mut ts: *mut libc::c_float = tsbuf;
    let mut tmpval: libc::c_float = 0.;
    tmpval =
        tmp[0 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 0 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 0 as libc::c_int) as isize) =
        tmpval * *w.offset((27 as libc::c_int + 0 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 0 as libc::c_int) as isize) =
        tmpval * *w.offset((26 as libc::c_int - 0 as libc::c_int) as isize);
    tmpval =
        tmp[0 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 0 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 0 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 0 as libc::c_int) as isize) +
            tmpval *
                *w.offset((8 as libc::c_int - 0 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 0 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 0 as libc::c_int) as isize) +
            tmpval *
                *w.offset((9 as libc::c_int + 0 as libc::c_int) as isize);
    let mut tmpval_0: libc::c_float = 0.;
    tmpval_0 =
        tmp[1 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 1 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 1 as libc::c_int) as isize) =
        tmpval_0 * *w.offset((27 as libc::c_int + 1 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 1 as libc::c_int) as isize) =
        tmpval_0 * *w.offset((26 as libc::c_int - 1 as libc::c_int) as isize);
    tmpval_0 =
        tmp[1 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 1 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 1 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 1 as libc::c_int) as isize) +
            tmpval_0 *
                *w.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 1 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 1 as libc::c_int) as isize) +
            tmpval_0 *
                *w.offset((9 as libc::c_int + 1 as libc::c_int) as isize);
    let mut tmpval_1: libc::c_float = 0.;
    tmpval_1 =
        tmp[2 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 2 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 2 as libc::c_int) as isize) =
        tmpval_1 * *w.offset((27 as libc::c_int + 2 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 2 as libc::c_int) as isize) =
        tmpval_1 * *w.offset((26 as libc::c_int - 2 as libc::c_int) as isize);
    tmpval_1 =
        tmp[2 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 2 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 2 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 2 as libc::c_int) as isize) +
            tmpval_1 *
                *w.offset((8 as libc::c_int - 2 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 2 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 2 as libc::c_int) as isize) +
            tmpval_1 *
                *w.offset((9 as libc::c_int + 2 as libc::c_int) as isize);
    let mut tmpval_2: libc::c_float = 0.;
    tmpval_2 =
        tmp[3 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 3 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 3 as libc::c_int) as isize) =
        tmpval_2 * *w.offset((27 as libc::c_int + 3 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 3 as libc::c_int) as isize) =
        tmpval_2 * *w.offset((26 as libc::c_int - 3 as libc::c_int) as isize);
    tmpval_2 =
        tmp[3 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 3 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 3 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 3 as libc::c_int) as isize) +
            tmpval_2 *
                *w.offset((8 as libc::c_int - 3 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 3 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 3 as libc::c_int) as isize) +
            tmpval_2 *
                *w.offset((9 as libc::c_int + 3 as libc::c_int) as isize);
    let mut tmpval_3: libc::c_float = 0.;
    tmpval_3 =
        tmp[4 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 4 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 4 as libc::c_int) as isize) =
        tmpval_3 * *w.offset((27 as libc::c_int + 4 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 4 as libc::c_int) as isize) =
        tmpval_3 * *w.offset((26 as libc::c_int - 4 as libc::c_int) as isize);
    tmpval_3 =
        tmp[4 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 4 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 4 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 4 as libc::c_int) as isize) +
            tmpval_3 *
                *w.offset((8 as libc::c_int - 4 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 4 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 4 as libc::c_int) as isize) +
            tmpval_3 *
                *w.offset((9 as libc::c_int + 4 as libc::c_int) as isize);
    let mut tmpval_4: libc::c_float = 0.;
    tmpval_4 =
        tmp[5 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 5 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 5 as libc::c_int) as isize) =
        tmpval_4 * *w.offset((27 as libc::c_int + 5 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 5 as libc::c_int) as isize) =
        tmpval_4 * *w.offset((26 as libc::c_int - 5 as libc::c_int) as isize);
    tmpval_4 =
        tmp[5 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 5 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 5 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 5 as libc::c_int) as isize) +
            tmpval_4 *
                *w.offset((8 as libc::c_int - 5 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 5 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 5 as libc::c_int) as isize) +
            tmpval_4 *
                *w.offset((9 as libc::c_int + 5 as libc::c_int) as isize);
    let mut tmpval_5: libc::c_float = 0.;
    tmpval_5 =
        tmp[6 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 6 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 6 as libc::c_int) as isize) =
        tmpval_5 * *w.offset((27 as libc::c_int + 6 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 6 as libc::c_int) as isize) =
        tmpval_5 * *w.offset((26 as libc::c_int - 6 as libc::c_int) as isize);
    tmpval_5 =
        tmp[6 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 6 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 6 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 6 as libc::c_int) as isize) +
            tmpval_5 *
                *w.offset((8 as libc::c_int - 6 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 6 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 6 as libc::c_int) as isize) +
            tmpval_5 *
                *w.offset((9 as libc::c_int + 6 as libc::c_int) as isize);
    let mut tmpval_6: libc::c_float = 0.;
    tmpval_6 =
        tmp[7 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 7 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 7 as libc::c_int) as isize) =
        tmpval_6 * *w.offset((27 as libc::c_int + 7 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 7 as libc::c_int) as isize) =
        tmpval_6 * *w.offset((26 as libc::c_int - 7 as libc::c_int) as isize);
    tmpval_6 =
        tmp[7 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 7 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 7 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 7 as libc::c_int) as isize) +
            tmpval_6 *
                *w.offset((8 as libc::c_int - 7 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 7 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 7 as libc::c_int) as isize) +
            tmpval_6 *
                *w.offset((9 as libc::c_int + 7 as libc::c_int) as isize);
    let mut tmpval_7: libc::c_float = 0.;
    tmpval_7 =
        tmp[8 as libc::c_int as usize] +
            tmp[(17 as libc::c_int - 8 as libc::c_int) as usize];
    *out2.offset((9 as libc::c_int + 8 as libc::c_int) as isize) =
        tmpval_7 * *w.offset((27 as libc::c_int + 8 as libc::c_int) as isize);
    *out2.offset((8 as libc::c_int - 8 as libc::c_int) as isize) =
        tmpval_7 * *w.offset((26 as libc::c_int - 8 as libc::c_int) as isize);
    tmpval_7 =
        tmp[8 as libc::c_int as usize] -
            tmp[(17 as libc::c_int - 8 as libc::c_int) as usize];
    *ts.offset((32 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)) as
                   isize) =
        *out1.offset((8 as libc::c_int - 8 as libc::c_int) as isize) +
            tmpval_7 *
                *w.offset((8 as libc::c_int - 8 as libc::c_int) as isize);
    *ts.offset((32 as libc::c_int * (9 as libc::c_int + 8 as libc::c_int)) as
                   isize) =
        *out1.offset((9 as libc::c_int + 8 as libc::c_int) as isize) +
            tmpval_7 *
                *w.offset((9 as libc::c_int + 8 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn dct12(mut in_0: *mut libc::c_float,
                               mut rawout1: *mut libc::c_float,
                               mut rawout2: *mut libc::c_float,
                               mut wi: *mut libc::c_float,
                               mut ts: *mut libc::c_float) {
    let mut in0: libc::c_float = 0.;
    let mut in1: libc::c_float = 0.;
    let mut in2: libc::c_float = 0.;
    let mut in3: libc::c_float = 0.;
    let mut in4: libc::c_float = 0.;
    let mut in5: libc::c_float = 0.;
    let mut out1: *mut libc::c_float = rawout1;
    *ts.offset((32 as libc::c_int * 0 as libc::c_int) as isize) =
        *out1.offset(0 as libc::c_int as isize);
    *ts.offset((32 as libc::c_int * 1 as libc::c_int) as isize) =
        *out1.offset(1 as libc::c_int as isize);
    *ts.offset((32 as libc::c_int * 2 as libc::c_int) as isize) =
        *out1.offset(2 as libc::c_int as isize);
    *ts.offset((32 as libc::c_int * 3 as libc::c_int) as isize) =
        *out1.offset(3 as libc::c_int as isize);
    *ts.offset((32 as libc::c_int * 4 as libc::c_int) as isize) =
        *out1.offset(4 as libc::c_int as isize);
    *ts.offset((32 as libc::c_int * 5 as libc::c_int) as isize) =
        *out1.offset(5 as libc::c_int as isize);
    in5 = *in_0.offset((5 as libc::c_int * 3 as libc::c_int) as isize);
    in4 = *in_0.offset((4 as libc::c_int * 3 as libc::c_int) as isize);
    in5 += in4;
    in3 = *in_0.offset((3 as libc::c_int * 3 as libc::c_int) as isize);
    in4 += in3;
    in2 = *in_0.offset((2 as libc::c_int * 3 as libc::c_int) as isize);
    in3 += in2;
    in1 = *in_0.offset((1 as libc::c_int * 3 as libc::c_int) as isize);
    in2 += in1;
    in0 = *in_0.offset((0 as libc::c_int * 3 as libc::c_int) as isize);
    in1 += in0;
    in5 += in3;
    in3 += in1;
    in2 = in2 * COS6_1;
    in3 = in3 * COS6_1;
    let mut tmp0: libc::c_float = 0.;
    let mut tmp1: libc::c_float = in0 - in4;
    let mut tmp2: libc::c_float =
        (in1 - in5) * tfcos12[1 as libc::c_int as usize];
    tmp0 = tmp1 + tmp2;
    tmp1 -= tmp2;
    *ts.offset(((17 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((17 as libc::c_int - 1 as libc::c_int) as isize) +
            tmp0 *
                *wi.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
    *ts.offset(((12 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((12 as libc::c_int + 1 as libc::c_int) as isize) +
            tmp0 * *wi.offset((6 as libc::c_int + 1 as libc::c_int) as isize);
    *ts.offset(((6 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((6 as libc::c_int + 1 as libc::c_int) as isize) +
            tmp1 * *wi.offset(1 as libc::c_int as isize);
    *ts.offset(((11 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((11 as libc::c_int - 1 as libc::c_int) as isize) +
            tmp1 * *wi.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
    in0 += in4 * COS6_2;
    in4 = in0 + in2;
    in0 -= in2;
    in1 += in5 * COS6_2;
    in5 = (in1 + in3) * tfcos12[0 as libc::c_int as usize];
    in1 = (in1 - in3) * tfcos12[2 as libc::c_int as usize];
    in3 = in4 + in5;
    in4 -= in5;
    in2 = in0 + in1;
    in0 -= in1;
    *ts.offset(((17 as libc::c_int - 0 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((17 as libc::c_int - 0 as libc::c_int) as isize) +
            in2 * *wi.offset((11 as libc::c_int - 0 as libc::c_int) as isize);
    *ts.offset(((12 as libc::c_int + 0 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((12 as libc::c_int + 0 as libc::c_int) as isize) +
            in2 * *wi.offset((6 as libc::c_int + 0 as libc::c_int) as isize);
    *ts.offset(((12 as libc::c_int + 2 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((12 as libc::c_int + 2 as libc::c_int) as isize) +
            in3 * *wi.offset((6 as libc::c_int + 2 as libc::c_int) as isize);
    *ts.offset(((17 as libc::c_int - 2 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((17 as libc::c_int - 2 as libc::c_int) as isize) +
            in3 * *wi.offset((11 as libc::c_int - 2 as libc::c_int) as isize);
    *ts.offset(((6 as libc::c_int + 0 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((6 as libc::c_int + 0 as libc::c_int) as isize) +
            in0 * *wi.offset(0 as libc::c_int as isize);
    *ts.offset(((11 as libc::c_int - 0 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((11 as libc::c_int - 0 as libc::c_int) as isize) +
            in0 * *wi.offset((5 as libc::c_int - 0 as libc::c_int) as isize);
    *ts.offset(((6 as libc::c_int + 2 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((6 as libc::c_int + 2 as libc::c_int) as isize) +
            in4 * *wi.offset(2 as libc::c_int as isize);
    *ts.offset(((11 as libc::c_int - 2 as libc::c_int) * 32 as libc::c_int) as
                   isize) =
        *out1.offset((11 as libc::c_int - 2 as libc::c_int) as isize) +
            in4 * *wi.offset((5 as libc::c_int - 2 as libc::c_int) as isize);
    in_0 = in_0.offset(1);
    let mut in0_0: libc::c_float = 0.;
    let mut in1_0: libc::c_float = 0.;
    let mut in2_0: libc::c_float = 0.;
    let mut in3_0: libc::c_float = 0.;
    let mut in4_0: libc::c_float = 0.;
    let mut in5_0: libc::c_float = 0.;
    let mut out2: *mut libc::c_float = rawout2;
    in5_0 = *in_0.offset((5 as libc::c_int * 3 as libc::c_int) as isize);
    in4_0 = *in_0.offset((4 as libc::c_int * 3 as libc::c_int) as isize);
    in5_0 += in4_0;
    in3_0 = *in_0.offset((3 as libc::c_int * 3 as libc::c_int) as isize);
    in4_0 += in3_0;
    in2_0 = *in_0.offset((2 as libc::c_int * 3 as libc::c_int) as isize);
    in3_0 += in2_0;
    in1_0 = *in_0.offset((1 as libc::c_int * 3 as libc::c_int) as isize);
    in2_0 += in1_0;
    in0_0 = *in_0.offset((0 as libc::c_int * 3 as libc::c_int) as isize);
    in1_0 += in0_0;
    in5_0 += in3_0;
    in3_0 += in1_0;
    in2_0 = in2_0 * COS6_1;
    in3_0 = in3_0 * COS6_1;
    let mut tmp0_0: libc::c_float = 0.;
    let mut tmp1_0: libc::c_float = in0_0 - in4_0;
    let mut tmp2_0: libc::c_float =
        (in1_0 - in5_0) * tfcos12[1 as libc::c_int as usize];
    tmp0_0 = tmp1_0 + tmp2_0;
    tmp1_0 -= tmp2_0;
    *out2.offset((5 as libc::c_int - 1 as libc::c_int) as isize) =
        tmp0_0 * *wi.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
    *out2.offset((0 as libc::c_int + 1 as libc::c_int) as isize) =
        tmp0_0 * *wi.offset((6 as libc::c_int + 1 as libc::c_int) as isize);
    *ts.offset(((12 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int) as
                   isize) += tmp1_0 * *wi.offset(1 as libc::c_int as isize);
    *ts.offset(((17 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as
                   isize) +=
        tmp1_0 * *wi.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
    in0_0 += in4_0 * COS6_2;
    in4_0 = in0_0 + in2_0;
    in0_0 -= in2_0;
    in1_0 += in5_0 * COS6_2;
    in5_0 = (in1_0 + in3_0) * tfcos12[0 as libc::c_int as usize];
    in1_0 = (in1_0 - in3_0) * tfcos12[2 as libc::c_int as usize];
    in3_0 = in4_0 + in5_0;
    in4_0 -= in5_0;
    in2_0 = in0_0 + in1_0;
    in0_0 -= in1_0;
    *out2.offset((5 as libc::c_int - 0 as libc::c_int) as isize) =
        in2_0 * *wi.offset((11 as libc::c_int - 0 as libc::c_int) as isize);
    *out2.offset((0 as libc::c_int + 0 as libc::c_int) as isize) =
        in2_0 * *wi.offset((6 as libc::c_int + 0 as libc::c_int) as isize);
    *out2.offset((0 as libc::c_int + 2 as libc::c_int) as isize) =
        in3_0 * *wi.offset((6 as libc::c_int + 2 as libc::c_int) as isize);
    *out2.offset((5 as libc::c_int - 2 as libc::c_int) as isize) =
        in3_0 * *wi.offset((11 as libc::c_int - 2 as libc::c_int) as isize);
    *ts.offset(((12 as libc::c_int + 0 as libc::c_int) * 32 as libc::c_int) as
                   isize) += in0_0 * *wi.offset(0 as libc::c_int as isize);
    *ts.offset(((17 as libc::c_int - 0 as libc::c_int) * 32 as libc::c_int) as
                   isize) +=
        in0_0 * *wi.offset((5 as libc::c_int - 0 as libc::c_int) as isize);
    *ts.offset(((12 as libc::c_int + 2 as libc::c_int) * 32 as libc::c_int) as
                   isize) += in4_0 * *wi.offset(2 as libc::c_int as isize);
    *ts.offset(((17 as libc::c_int - 2 as libc::c_int) * 32 as libc::c_int) as
                   isize) +=
        in4_0 * *wi.offset((5 as libc::c_int - 2 as libc::c_int) as isize);
    in_0 = in_0.offset(1);
    let mut in0_1: libc::c_float = 0.;
    let mut in1_1: libc::c_float = 0.;
    let mut in2_1: libc::c_float = 0.;
    let mut in3_1: libc::c_float = 0.;
    let mut in4_1: libc::c_float = 0.;
    let mut in5_1: libc::c_float = 0.;
    let mut out2_0: *mut libc::c_float = rawout2;
    let ref mut fresh0 = *out2_0.offset(17 as libc::c_int as isize);
    *fresh0 = 0.0f64 as libc::c_float;
    let ref mut fresh1 = *out2_0.offset(16 as libc::c_int as isize);
    *fresh1 = *fresh0;
    let ref mut fresh2 = *out2_0.offset(15 as libc::c_int as isize);
    *fresh2 = *fresh1;
    let ref mut fresh3 = *out2_0.offset(14 as libc::c_int as isize);
    *fresh3 = *fresh2;
    let ref mut fresh4 = *out2_0.offset(13 as libc::c_int as isize);
    *fresh4 = *fresh3;
    *out2_0.offset(12 as libc::c_int as isize) = *fresh4;
    in5_1 = *in_0.offset((5 as libc::c_int * 3 as libc::c_int) as isize);
    in4_1 = *in_0.offset((4 as libc::c_int * 3 as libc::c_int) as isize);
    in5_1 += in4_1;
    in3_1 = *in_0.offset((3 as libc::c_int * 3 as libc::c_int) as isize);
    in4_1 += in3_1;
    in2_1 = *in_0.offset((2 as libc::c_int * 3 as libc::c_int) as isize);
    in3_1 += in2_1;
    in1_1 = *in_0.offset((1 as libc::c_int * 3 as libc::c_int) as isize);
    in2_1 += in1_1;
    in0_1 = *in_0.offset((0 as libc::c_int * 3 as libc::c_int) as isize);
    in1_1 += in0_1;
    in5_1 += in3_1;
    in3_1 += in1_1;
    in2_1 = in2_1 * COS6_1;
    in3_1 = in3_1 * COS6_1;
    let mut tmp0_1: libc::c_float = 0.;
    let mut tmp1_1: libc::c_float = in0_1 - in4_1;
    let mut tmp2_1: libc::c_float =
        (in1_1 - in5_1) * tfcos12[1 as libc::c_int as usize];
    tmp0_1 = tmp1_1 + tmp2_1;
    tmp1_1 -= tmp2_1;
    *out2_0.offset((11 as libc::c_int - 1 as libc::c_int) as isize) =
        tmp0_1 * *wi.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
    *out2_0.offset((6 as libc::c_int + 1 as libc::c_int) as isize) =
        tmp0_1 * *wi.offset((6 as libc::c_int + 1 as libc::c_int) as isize);
    *out2_0.offset((0 as libc::c_int + 1 as libc::c_int) as isize) +=
        tmp1_1 * *wi.offset(1 as libc::c_int as isize);
    *out2_0.offset((5 as libc::c_int - 1 as libc::c_int) as isize) +=
        tmp1_1 * *wi.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
    in0_1 += in4_1 * COS6_2;
    in4_1 = in0_1 + in2_1;
    in0_1 -= in2_1;
    in1_1 += in5_1 * COS6_2;
    in5_1 = (in1_1 + in3_1) * tfcos12[0 as libc::c_int as usize];
    in1_1 = (in1_1 - in3_1) * tfcos12[2 as libc::c_int as usize];
    in3_1 = in4_1 + in5_1;
    in4_1 -= in5_1;
    in2_1 = in0_1 + in1_1;
    in0_1 -= in1_1;
    *out2_0.offset((11 as libc::c_int - 0 as libc::c_int) as isize) =
        in2_1 * *wi.offset((11 as libc::c_int - 0 as libc::c_int) as isize);
    *out2_0.offset((6 as libc::c_int + 0 as libc::c_int) as isize) =
        in2_1 * *wi.offset((6 as libc::c_int + 0 as libc::c_int) as isize);
    *out2_0.offset((6 as libc::c_int + 2 as libc::c_int) as isize) =
        in3_1 * *wi.offset((6 as libc::c_int + 2 as libc::c_int) as isize);
    *out2_0.offset((11 as libc::c_int - 2 as libc::c_int) as isize) =
        in3_1 * *wi.offset((11 as libc::c_int - 2 as libc::c_int) as isize);
    *out2_0.offset((0 as libc::c_int + 0 as libc::c_int) as isize) +=
        in0_1 * *wi.offset(0 as libc::c_int as isize);
    *out2_0.offset((5 as libc::c_int - 0 as libc::c_int) as isize) +=
        in0_1 * *wi.offset((5 as libc::c_int - 0 as libc::c_int) as isize);
    *out2_0.offset((0 as libc::c_int + 2 as libc::c_int) as isize) +=
        in4_1 * *wi.offset(2 as libc::c_int as isize);
    *out2_0.offset((5 as libc::c_int - 2 as libc::c_int) as isize) +=
        in4_1 * *wi.offset((5 as libc::c_int - 2 as libc::c_int) as isize);
}
