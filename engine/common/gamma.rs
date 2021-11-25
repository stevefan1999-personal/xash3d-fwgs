#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
}
pub type byte = libc::c_uchar;
#[inline(always)]
unsafe extern "C" fn __tg_pow(mut __x: libc::c_float, mut __y: libc::c_float)
 -> libc::c_float {
    return powf(__x, __y);
}
/*
gamma.c - gamma routines
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
//-----------------------------------------------------------------------------
// Gamma conversion support
//-----------------------------------------------------------------------------
static mut texgammatable: [byte; 256] = [0; 256];
// palette is sent through this to convert to screen gamma
static mut lightgammatable: [byte; 256] = [0; 256];
static mut lineargammatable: [libc::c_int; 1024] = [0; 1024];
static mut screengammatable: [libc::c_int; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn BuildGammaTable(mut lightgamma: libc::c_float,
                                         mut brightness: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut inf: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut g1: libc::c_float = 0.;
    let mut g3: libc::c_float = 0.;
    lightgamma =
        if lightgamma >= 1.8f32 {
            if lightgamma < 3.0f32 { lightgamma } else { 3.0f32 }
        } else { 1.8f32 };
    brightness =
        if brightness >= 0.0f32 {
            if brightness < 10.0f32 { brightness } else { 10.0f32 }
        } else { 0.0f32 };
    if brightness <= 0.0f32 {
        g3 = 0.125f32
    } else if brightness > 1.0f32 {
        g3 = 0.05f32
    } else { g3 = 0.125f32 - brightness * brightness * 0.075f32 }
    g = 1.0f32 / lightgamma;
    g1 = 2.2f32 * g;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        f = __tg_pow(i as libc::c_float / 255.0f32, 2.2f32);
        // scale up
        if brightness > 1.0f32 { f = f * brightness }
        // shift up
        if f <= g3 {
            f = f / g3 * 0.125f32
        } else { f = 0.125f32 + (f - g3) / (1.0f32 - g3) * 0.875f32 }
        // convert linear space to desired gamma space
        inf = (255.0f32 * __tg_pow(f, g)) as libc::c_int;
        lightgammatable[i as usize] =
            if inf >= 0 as libc::c_int {
                if inf < 255 as libc::c_int {
                    inf
                } else { 255 as libc::c_int }
            } else { 0 as libc::c_int } as byte;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        f = 255.0f32 * __tg_pow(i as libc::c_float / 255.0f32, 0.9f32);
        inf = (f + 0.5f32) as libc::c_int;
        texgammatable[i as usize] =
            if inf >= 0 as libc::c_int {
                if inf < 255 as libc::c_int {
                    inf
                } else { 255 as libc::c_int }
            } else { 0 as libc::c_int } as byte;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        // convert from screen gamma space to linear space
        lineargammatable[i as usize] =
            (1023 as libc::c_int as libc::c_float *
                 __tg_pow(i as libc::c_float / 1023.0f32, g1)) as libc::c_int;
        // convert from linear gamma space to screen space
        screengammatable[i as usize] =
            (1023 as libc::c_int as libc::c_float *
                 __tg_pow(i as libc::c_float / 1023.0f32, 1.0f32 / g1)) as
                libc::c_int;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn LightToTexGamma(mut b: byte) -> byte {
    return lightgammatable[b as usize];
}
#[no_mangle]
pub unsafe extern "C" fn TextureToGamma(mut b: byte) -> byte {
    return texgammatable[b as usize];
}
