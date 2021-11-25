#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn Matrix4x4_Invert_Simple(out: *mut [vec_t; 4], in1: *const [vec_t; 4]);
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
}
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type matrix4x4 = [[vec_t; 4]; 4];
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link_s {
    pub prev: *mut link_s,
    pub next: *mut link_s,
}
pub type link_t = link_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edict_s {
    pub free: qboolean,
    pub serialnumber: libc::c_int,
    pub area: link_t,
    pub headnode: libc::c_int,
    pub num_leafs: libc::c_int,
    pub leafnums: [libc::c_short; 48],
    pub freetime: libc::c_float,
    pub pvPrivateData: *mut libc::c_void,
    pub v: entvars_t,
}
pub type entvars_t = entvars_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entvars_s {
    pub classname: string_t,
    pub globalname: string_t,
    pub origin: vec3_t,
    pub oldorigin: vec3_t,
    pub velocity: vec3_t,
    pub basevelocity: vec3_t,
    pub clbasevelocity: vec3_t,
    pub movedir: vec3_t,
    pub angles: vec3_t,
    pub avelocity: vec3_t,
    pub punchangle: vec3_t,
    pub v_angle: vec3_t,
    pub endpos: vec3_t,
    pub startpos: vec3_t,
    pub impacttime: libc::c_float,
    pub starttime: libc::c_float,
    pub fixangle: libc::c_int,
    pub idealpitch: libc::c_float,
    pub pitch_speed: libc::c_float,
    pub ideal_yaw: libc::c_float,
    pub yaw_speed: libc::c_float,
    pub modelindex: libc::c_int,
    pub model: string_t,
    pub viewmodel: libc::c_int,
    pub weaponmodel: libc::c_int,
    pub absmin: vec3_t,
    pub absmax: vec3_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub size: vec3_t,
    pub ltime: libc::c_float,
    pub nextthink: libc::c_float,
    pub movetype: libc::c_int,
    pub solid: libc::c_int,
    pub skin: libc::c_int,
    pub body: libc::c_int,
    pub effects: libc::c_int,
    pub gravity: libc::c_float,
    pub friction: libc::c_float,
    pub light_level: libc::c_int,
    pub sequence: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub frame: libc::c_float,
    pub animtime: libc::c_float,
    pub framerate: libc::c_float,
    pub controller: [byte; 4],
    pub blending: [byte; 2],
    pub scale: libc::c_float,
    pub rendermode: libc::c_int,
    pub renderamt: libc::c_float,
    pub rendercolor: vec3_t,
    pub renderfx: libc::c_int,
    pub health: libc::c_float,
    pub frags: libc::c_float,
    pub weapons: libc::c_int,
    pub takedamage: libc::c_float,
    pub deadflag: libc::c_int,
    pub view_ofs: vec3_t,
    pub button: libc::c_int,
    pub impulse: libc::c_int,
    pub chain: *mut edict_t,
    pub dmg_inflictor: *mut edict_t,
    pub enemy: *mut edict_t,
    pub aiment: *mut edict_t,
    pub owner: *mut edict_t,
    pub groundentity: *mut edict_t,
    pub spawnflags: libc::c_int,
    pub flags: libc::c_int,
    pub colormap: libc::c_int,
    pub team: libc::c_int,
    pub max_health: libc::c_float,
    pub teleport_time: libc::c_float,
    pub armortype: libc::c_float,
    pub armorvalue: libc::c_float,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub target: string_t,
    pub targetname: string_t,
    pub netname: string_t,
    pub message: string_t,
    pub dmg_take: libc::c_float,
    pub dmg_save: libc::c_float,
    pub dmg: libc::c_float,
    pub dmgtime: libc::c_float,
    pub noise: string_t,
    pub noise1: string_t,
    pub noise2: string_t,
    pub noise3: string_t,
    pub speed: libc::c_float,
    pub air_finished: libc::c_float,
    pub pain_finished: libc::c_float,
    pub radsuit_finished: libc::c_float,
    pub pContainingEntity: *mut edict_t,
    pub playerclass: libc::c_int,
    pub maxspeed: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub pushmsec: libc::c_int,
    pub bInDuck: libc::c_int,
    pub flTimeStepSound: libc::c_int,
    pub flSwimTime: libc::c_int,
    pub flDuckTime: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub gamestate: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub groupinfo: libc::c_int,
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
    pub vuser1: vec3_t,
    pub vuser2: vec3_t,
    pub vuser3: vec3_t,
    pub vuser4: vec3_t,
    pub euser1: *mut edict_t,
    pub euser2: *mut edict_t,
    pub euser3: *mut edict_t,
    pub euser4: *mut edict_t,
}
pub type edict_t = edict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plane_t {
    pub normal: vec3_t,
    pub dist: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_t {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub inopen: qboolean,
    pub inwater: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: plane_t,
    pub ent: *mut edict_t,
    pub hitgroup: libc::c_int,
}
/*
world.c - common worldtrace routines
Copyright (C) 2009 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// just for debug
#[no_mangle]
pub static mut et_name: [*const libc::c_char; 5] =
    [b"normal\x00" as *const u8 as *const libc::c_char,
     b"player\x00" as *const u8 as *const libc::c_char,
     b"tempentity\x00" as *const u8 as *const libc::c_char,
     b"beam\x00" as *const u8 as *const libc::c_char,
     b"fragmented\x00" as *const u8 as *const libc::c_char];
/*
===============================================================================

	ENTITY LINKING

===============================================================================
*/
/*
===============
ClearLink

ClearLink is used for new headnodes
===============
*/
#[no_mangle]
pub unsafe extern "C" fn ClearLink(mut l: *mut link_t) {
    (*l).next = l;
    (*l).prev = (*l).next;
}
/*
===============
RemoveLink

remove link from chain
===============
*/
#[no_mangle]
pub unsafe extern "C" fn RemoveLink(mut l: *mut link_t) {
    (*(*l).next).prev = (*l).prev;
    (*(*l).prev).next = (*l).next;
}
/*
===============
InsertLinkBefore

kept trigger and solid entities seperate
===============
*/
#[no_mangle]
pub unsafe extern "C" fn InsertLinkBefore(mut l: *mut link_t,
                                          mut before: *mut link_t) {
    (*l).next = before;
    (*l).prev = (*before).prev;
    (*(*l).prev).next = l;
    (*(*l).next).prev = l;
}
/*
==================
World_MoveBounds
==================
*/
#[no_mangle]
pub unsafe extern "C" fn World_MoveBounds(mut start: *const vec_t,
                                          mut mins: *mut vec_t,
                                          mut maxs: *mut vec_t,
                                          mut end: *const vec_t,
                                          mut boxmins: *mut vec_t,
                                          mut boxmaxs: *mut vec_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *end.offset(i as isize) > *start.offset(i as isize) {
            *boxmins.offset(i as isize) =
                *start.offset(i as isize) + *mins.offset(i as isize) - 1.0f32;
            *boxmaxs.offset(i as isize) =
                *end.offset(i as isize) + *maxs.offset(i as isize) + 1.0f32
        } else {
            *boxmins.offset(i as isize) =
                *end.offset(i as isize) + *mins.offset(i as isize) - 1.0f32;
            *boxmaxs.offset(i as isize) =
                *start.offset(i as isize) + *maxs.offset(i as isize) + 1.0f32
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn World_CombineTraces(mut cliptrace: *mut trace_t,
                                             mut trace: *mut trace_t,
                                             mut touch: *mut edict_t)
 -> trace_t {
    if (*trace).allsolid as libc::c_uint != 0 ||
           (*trace).startsolid as libc::c_uint != 0 ||
           (*trace).fraction < (*cliptrace).fraction {
        (*trace).ent = touch;
        if (*cliptrace).startsolid as u64 != 0 {
            *cliptrace = *trace;
            (*cliptrace).startsolid = true_0
        } else { *cliptrace = *trace }
    }
    return *cliptrace;
}
/*
==================
World_TransformAABB
==================
*/
#[no_mangle]
pub unsafe extern "C" fn World_TransformAABB(mut transform: *mut [vec_t; 4],
                                             mut mins: *const vec_t,
                                             mut maxs: *const vec_t,
                                             mut outmins: *mut vec_t,
                                             mut outmaxs: *mut vec_t) {
    let mut p1: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    let mut itransform: matrix4x4 = [[0.; 4]; 4];
    let mut i: libc::c_int = 0;
    if outmins.is_null() || outmaxs.is_null() { return }
    Matrix4x4_Invert_Simple(itransform.as_mut_ptr(),
                            transform as *const [vec_t; 4]);
    ClearBounds(outmins, outmaxs);
    // compute a full bounding box
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        p1[0 as libc::c_int as usize] =
            if i & 1 as libc::c_int != 0 {
                *mins.offset(0 as libc::c_int as isize)
            } else { *maxs.offset(0 as libc::c_int as isize) };
        p1[1 as libc::c_int as usize] =
            if i & 2 as libc::c_int != 0 {
                *mins.offset(1 as libc::c_int as isize)
            } else { *maxs.offset(1 as libc::c_int as isize) };
        p1[2 as libc::c_int as usize] =
            if i & 4 as libc::c_int != 0 {
                *mins.offset(2 as libc::c_int as isize)
            } else { *maxs.offset(2 as libc::c_int as isize) };
        p2[0 as libc::c_int as usize] =
            p1[0 as libc::c_int as usize] *
                itransform[0 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
                p1[1 as libc::c_int as usize] *
                    itransform[0 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
                p1[2 as libc::c_int as usize] *
                    itransform[0 as libc::c_int as
                                   usize][2 as libc::c_int as usize];
        p2[1 as libc::c_int as usize] =
            p1[0 as libc::c_int as usize] *
                itransform[1 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
                p1[1 as libc::c_int as usize] *
                    itransform[1 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
                p1[2 as libc::c_int as usize] *
                    itransform[1 as libc::c_int as
                                   usize][2 as libc::c_int as usize];
        p2[2 as libc::c_int as usize] =
            p1[0 as libc::c_int as usize] *
                itransform[2 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
                p1[1 as libc::c_int as usize] *
                    itransform[2 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
                p1[2 as libc::c_int as usize] *
                    itransform[2 as libc::c_int as
                                   usize][2 as libc::c_int as usize];
        if p2[0 as libc::c_int as usize] <
               *outmins.offset(0 as libc::c_int as isize) {
            *outmins.offset(0 as libc::c_int as isize) =
                p2[0 as libc::c_int as usize]
        }
        if p2[0 as libc::c_int as usize] >
               *outmaxs.offset(0 as libc::c_int as isize) {
            *outmaxs.offset(0 as libc::c_int as isize) =
                p2[0 as libc::c_int as usize]
        }
        if p2[1 as libc::c_int as usize] <
               *outmins.offset(1 as libc::c_int as isize) {
            *outmins.offset(1 as libc::c_int as isize) =
                p2[1 as libc::c_int as usize]
        }
        if p2[1 as libc::c_int as usize] >
               *outmaxs.offset(1 as libc::c_int as isize) {
            *outmaxs.offset(1 as libc::c_int as isize) =
                p2[1 as libc::c_int as usize]
        }
        if p2[2 as libc::c_int as usize] <
               *outmins.offset(2 as libc::c_int as isize) {
            *outmins.offset(2 as libc::c_int as isize) =
                p2[2 as libc::c_int as usize]
        }
        if p2[2 as libc::c_int as usize] >
               *outmaxs.offset(2 as libc::c_int as isize) {
            *outmaxs.offset(2 as libc::c_int as isize) =
                p2[2 as libc::c_int as usize]
        }
        i += 1
    }
    // sanity check
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *outmins.offset(i as isize) > *outmaxs.offset(i as isize) {
            let ref mut fresh0 = *outmins.offset(2 as libc::c_int as isize);
            *fresh0 = 0 as libc::c_int as vec_t;
            let ref mut fresh1 = *outmins.offset(1 as libc::c_int as isize);
            *fresh1 = *fresh0;
            *outmins.offset(0 as libc::c_int as isize) = *fresh1;
            let ref mut fresh2 = *outmaxs.offset(2 as libc::c_int as isize);
            *fresh2 = 0 as libc::c_int as vec_t;
            let ref mut fresh3 = *outmaxs.offset(1 as libc::c_int as isize);
            *fresh3 = *fresh2;
            *outmaxs.offset(0 as libc::c_int as isize) = *fresh3;
            return
        }
        i += 1
    };
}
/*
==================
RankForContents

Used for determine contents priority
==================
*/
#[no_mangle]
pub unsafe extern "C" fn RankForContents(mut contents: libc::c_int)
 -> libc::c_int {
    match contents {
        -1 => {
            return 0 as libc::c_int
            // any user contents has more priority than default
        }
        -3 => { return 1 as libc::c_int }
        -15 => { return 2 as libc::c_int }
        -9 => { return 3 as libc::c_int }
        -10 => { return 4 as libc::c_int }
        -11 => { return 5 as libc::c_int }
        -12 => { return 6 as libc::c_int }
        -13 => { return 7 as libc::c_int }
        -14 => { return 8 as libc::c_int }
        -4 => { return 9 as libc::c_int }
        -5 => { return 10 as libc::c_int }
        -6 => { return 11 as libc::c_int }
        -2 => { return 12 as libc::c_int }
        _ => { return 13 as libc::c_int }
    };
}
