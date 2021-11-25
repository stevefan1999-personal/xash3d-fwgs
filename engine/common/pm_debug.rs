#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut boxpnt: [[libc::c_int; 4]; 6];
    #[no_mangle]
    fn VectorNormalizeLength2(v: *const vec_t, out: *mut vec_t)
     -> libc::c_float;
    #[no_mangle]
    fn CL_Particle(org: *const vec_t, color: libc::c_int, life: libc::c_float,
                   zpos: libc::c_int, zvel: libc::c_int);
}
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
/*
===============
PM_ParticleLine

draw line from particles
================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_ParticleLine(mut start: *const vec_t,
                                         mut end: *const vec_t,
                                         mut pcolor: libc::c_int,
                                         mut life: libc::c_float,
                                         mut zvel: libc::c_float) {
    let mut len: libc::c_float = 0.;
    let mut curdist: libc::c_float = 0.;
    let mut diff: vec3_t = [0.; 3];
    let mut pos: vec3_t = [0.; 3];
    // determine distance
    diff[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    diff[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    diff[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    len =
        VectorNormalizeLength2(diff.as_mut_ptr() as *const vec_t,
                               diff.as_mut_ptr());
    curdist = 0 as libc::c_int as libc::c_float;
    while curdist <= len {
        pos[0 as libc::c_int as usize] =
            *start.offset(0 as libc::c_int as isize) +
                curdist * diff[0 as libc::c_int as usize];
        pos[1 as libc::c_int as usize] =
            *start.offset(1 as libc::c_int as isize) +
                curdist * diff[1 as libc::c_int as usize];
        pos[2 as libc::c_int as usize] =
            *start.offset(2 as libc::c_int as isize) +
                curdist * diff[2 as libc::c_int as usize];
        CL_Particle(pos.as_mut_ptr() as *const vec_t, pcolor, life,
                    0 as libc::c_int, zvel as libc::c_int);
        curdist += 2.0f32
    };
    // XASH_DEDICATED
}
/*
================
PM_DrawRectangle

================
*/
unsafe extern "C" fn PM_DrawRectangle(mut tl: *const vec_t,
                                      mut bl: *const vec_t,
                                      mut tr: *const vec_t,
                                      mut br: *const vec_t,
                                      mut pcolor: libc::c_int,
                                      mut life: libc::c_float) {
    PM_ParticleLine(tl, bl, pcolor, life, 0 as libc::c_int as libc::c_float);
    PM_ParticleLine(bl, br, pcolor, life, 0 as libc::c_int as libc::c_float);
    PM_ParticleLine(br, tr, pcolor, life, 0 as libc::c_int as libc::c_float);
    PM_ParticleLine(tr, tl, pcolor, life, 0 as libc::c_int as libc::c_float);
}
/*
================
PM_DrawBBox

================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_DrawBBox(mut mins: *const vec_t,
                                     mut maxs: *const vec_t,
                                     mut origin: *const vec_t,
                                     mut pcolor: libc::c_int,
                                     mut life: libc::c_float) {
    let mut p: [vec3_t; 8] = [[0.; 3]; 8];
    let mut tmp: vec3_t = [0.; 3];
    let mut gap: libc::c_float = 0.0f32;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        tmp[0 as libc::c_int as usize] =
            if i & 1 as libc::c_int != 0 {
                (*mins.offset(0 as libc::c_int as isize)) - gap
            } else { (*maxs.offset(0 as libc::c_int as isize)) + gap };
        tmp[1 as libc::c_int as usize] =
            if i & 2 as libc::c_int != 0 {
                (*mins.offset(1 as libc::c_int as isize)) - gap
            } else { (*maxs.offset(1 as libc::c_int as isize)) + gap };
        tmp[2 as libc::c_int as usize] =
            if i & 4 as libc::c_int != 0 {
                (*mins.offset(2 as libc::c_int as isize)) - gap
            } else { (*maxs.offset(2 as libc::c_int as isize)) + gap };
        tmp[0 as libc::c_int as usize] =
            tmp[0 as libc::c_int as usize] +
                *origin.offset(0 as libc::c_int as isize);
        tmp[1 as libc::c_int as usize] =
            tmp[1 as libc::c_int as usize] +
                *origin.offset(1 as libc::c_int as isize);
        tmp[2 as libc::c_int as usize] =
            tmp[2 as libc::c_int as usize] +
                *origin.offset(2 as libc::c_int as isize);
        p[i as usize][0 as libc::c_int as usize] =
            tmp[0 as libc::c_int as usize];
        p[i as usize][1 as libc::c_int as usize] =
            tmp[1 as libc::c_int as usize];
        p[i as usize][2 as libc::c_int as usize] =
            tmp[2 as libc::c_int as usize];
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        PM_DrawRectangle(p[boxpnt[i as usize][1 as libc::c_int as usize] as
                               usize].as_mut_ptr() as *const vec_t,
                         p[boxpnt[i as usize][0 as libc::c_int as usize] as
                               usize].as_mut_ptr() as *const vec_t,
                         p[boxpnt[i as usize][2 as libc::c_int as usize] as
                               usize].as_mut_ptr() as *const vec_t,
                         p[boxpnt[i as usize][3 as libc::c_int as usize] as
                               usize].as_mut_ptr() as *const vec_t, pcolor,
                         life);
        i += 1
    };
    // XASH_DEDICATED
}
