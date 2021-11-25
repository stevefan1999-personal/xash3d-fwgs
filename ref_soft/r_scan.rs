#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    static mut d_sdivzstepu: libc::c_float;
    #[no_mangle]
    static mut d_tdivzstepu: libc::c_float;
    #[no_mangle]
    static mut d_zistepu: libc::c_float;
    #[no_mangle]
    static mut d_sdivzstepv: libc::c_float;
    #[no_mangle]
    static mut d_tdivzstepv: libc::c_float;
    #[no_mangle]
    static mut d_zistepv: libc::c_float;
    #[no_mangle]
    static mut d_sdivzorigin: libc::c_float;
    #[no_mangle]
    static mut d_tdivzorigin: libc::c_float;
    #[no_mangle]
    static mut d_ziorigin: libc::c_float;
    #[no_mangle]
    static mut sadjust: fixed16_t;
    #[no_mangle]
    static mut tadjust: fixed16_t;
    #[no_mangle]
    static mut bbextents: fixed16_t;
    #[no_mangle]
    static mut bbextentt: fixed16_t;
    #[no_mangle]
    static mut cachewidth: libc::c_int;
    #[no_mangle]
    static mut sw_texfilt: *mut cvar_t;
    #[no_mangle]
    static mut r_screenwidth: libc::c_int;
    #[no_mangle]
    static mut d_viewbuffer: *mut pixel_t;
    #[no_mangle]
    static mut cacheblock: *mut pixel_t;
    #[no_mangle]
    static mut d_zwidth: libc::c_uint;
    #[no_mangle]
    static mut d_pzbuffer: *mut libc::c_short;
    #[no_mangle]
    static mut sintable: [libc::c_int; 1280];
    #[no_mangle]
    static mut blanktable: [libc::c_int; 1280];
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
pub type cvar_t = cvar_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color24 {
    pub r: byte,
    pub g: byte,
    pub b: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorVec {
    pub r: libc::c_uint,
    pub g: libc::c_uint,
    pub b: libc::c_uint,
    pub a: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmodel_t {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub headnode: [libc::c_int; 4],
    pub visleafs: libc::c_int,
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
}
pub type modtype_t = libc::c_int;
pub const mod_studio: modtype_t = 3;
pub const mod_alias: modtype_t = 2;
pub const mod_sprite: modtype_t = 1;
pub const mod_brush: modtype_t = 0;
pub const mod_bad: modtype_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mclipnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texture_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub gl_texturenum: libc::c_int,
    pub texturechain: *mut msurface_s,
    pub anim_total: libc::c_int,
    pub anim_min: libc::c_int,
    pub anim_max: libc::c_int,
    pub anim_next: *mut texture_s,
    pub alternate_anims: *mut texture_s,
    pub fb_texturenum: libc::c_ushort,
    pub dt_texturenum: libc::c_ushort,
    pub unused: [libc::c_uint; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msurface_s {
    pub visframe: libc::c_int,
    pub plane: *mut mplane_t,
    pub flags: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub texturemins: [libc::c_short; 2],
    pub extents: [libc::c_short; 2],
    pub light_s: libc::c_int,
    pub light_t: libc::c_int,
    pub polys: *mut glpoly_t,
    pub texturechain: *mut msurface_s,
    pub texinfo: *mut mtexinfo_t,
    pub dlightframe: libc::c_int,
    pub dlightbits: libc::c_int,
    pub lightmaptexturenum: libc::c_int,
    pub styles: [byte; 4],
    pub cached_light: [libc::c_int; 4],
    pub info: *mut mextrasurf_t,
    pub samples: *mut color24,
    pub pdecals: *mut decal_t,
}
pub type decal_t = decal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decal_s {
    pub pnext: *mut decal_t,
    pub psurface: *mut msurface_t,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub scale: libc::c_float,
    pub texture: libc::c_short,
    pub flags: libc::c_short,
    pub entityIndex: libc::c_short,
    pub position: vec3_t,
    pub polys: *mut glpoly_t,
    pub reserved: [libc::c_int; 4],
}
pub type glpoly_t = glpoly_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glpoly_s {
    pub next: *mut glpoly_s,
    pub chain: *mut glpoly_s,
    pub numverts: libc::c_int,
    pub flags: libc::c_int,
    pub verts: [[libc::c_float; 7]; 4],
}
pub type msurface_t = msurface_s;
pub type mextrasurf_t = mextrasurf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mextrasurf_s {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub surf: *mut msurface_s,
    pub dlight_s: libc::c_int,
    pub dlight_t: libc::c_int,
    pub lightmapmins: [libc::c_short; 2],
    pub lightextents: [libc::c_short; 2],
    pub lmvecs: [[libc::c_float; 4]; 2],
    pub deluxemap: *mut color24,
    pub shadowmap: *mut byte,
    pub lightmapchain: *mut msurface_s,
    pub detailchain: *mut mextrasurf_s,
    pub bevel: *mut mfacebevel_t,
    pub lumachain: *mut mextrasurf_s,
    pub parent: *mut cl_entity_s,
    pub mirrortexturenum: libc::c_int,
    pub mirrormatrix: [[libc::c_float; 4]; 4],
    pub grass: *mut grasshdr_s,
    pub grasscount: libc::c_ushort,
    pub numverts: libc::c_ushort,
    pub firstvertex: libc::c_int,
    pub reserved: [libc::c_int; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_entity_s {
    pub index: libc::c_int,
    pub player: qboolean,
    pub baseline: entity_state_t,
    pub prevstate: entity_state_t,
    pub curstate: entity_state_t,
    pub current_position: libc::c_int,
    pub ph: [position_history_t; 64],
    pub mouth: mouth_t,
    pub latched: latchedvars_t,
    pub lastmove: libc::c_float,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub attachment: [vec3_t; 4],
    pub trivial_accept: libc::c_int,
    pub model: *mut model_s,
    pub efrag: *mut efrag_s,
    pub topnode: *mut mnode_s,
    pub syncbase: libc::c_float,
    pub visframe: libc::c_int,
    pub cvFloorColor: colorVec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut mplane_t,
    pub children: [*mut mnode_s; 2],
    pub firstsurface: libc::c_ushort,
    pub numsurfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct efrag_s {
    pub leaf: *mut mleaf_s,
    pub leafnext: *mut efrag_s,
    pub entity: *mut cl_entity_s,
    pub entnext: *mut efrag_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mleaf_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub compressed_vis: *mut byte,
    pub efrags: *mut efrag_s,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub cluster: libc::c_int,
    pub ambient_sound_level: [byte; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub needload: qboolean,
    pub type_0: modtype_t,
    pub numframes: libc::c_int,
    pub mempool: poolhandle_t,
    pub flags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub radius: libc::c_float,
    pub firstmodelsurface: libc::c_int,
    pub nummodelsurfaces: libc::c_int,
    pub numsubmodels: libc::c_int,
    pub submodels: *mut dmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut mplane_t,
    pub numleafs: libc::c_int,
    pub leafs: *mut mleaf_t,
    pub numvertexes: libc::c_int,
    pub vertexes: *mut mvertex_t,
    pub numedges: libc::c_int,
    pub edges: *mut medge_t,
    pub numnodes: libc::c_int,
    pub nodes: *mut mnode_t,
    pub numtexinfo: libc::c_int,
    pub texinfo: *mut mtexinfo_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut msurface_t,
    pub numsurfedges: libc::c_int,
    pub surfedges: *mut libc::c_int,
    pub numclipnodes: libc::c_int,
    pub clipnodes: *mut mclipnode_t,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut msurface_t,
    pub hulls: [hull_t; 4],
    pub numtextures: libc::c_int,
    pub textures: *mut *mut texture_t,
    pub visdata: *mut byte,
    pub lightdata: *mut color24,
    pub entities: *mut libc::c_char,
    pub cache: cache_user_t,
}
pub type cache_user_t = cache_user_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_user_s {
    pub data: *mut libc::c_void,
}
pub type texture_t = texture_s;
pub type hull_t = hull_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hull_s {
    pub clipnodes: *mut mclipnode_t,
    pub planes: *mut mplane_t,
    pub firstclipnode: libc::c_int,
    pub lastclipnode: libc::c_int,
    pub clip_mins: vec3_t,
    pub clip_maxs: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_t {
    pub vecs: [[libc::c_float; 4]; 2],
    pub faceinfo: *mut mfaceinfo_t,
    pub texture: *mut texture_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mfaceinfo_t {
    pub landname: [libc::c_char; 16],
    pub texture_step: libc::c_ushort,
    pub max_extent: libc::c_ushort,
    pub groupid: libc::c_short,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub reserved: [libc::c_int; 32],
}
pub type mnode_t = mnode_s;
pub type mleaf_t = mleaf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latchedvars_t {
    pub prevanimtime: libc::c_float,
    pub sequencetime: libc::c_float,
    pub prevseqblending: [byte; 2],
    pub prevorigin: vec3_t,
    pub prevangles: vec3_t,
    pub prevsequence: libc::c_int,
    pub prevframe: libc::c_float,
    pub prevcontroller: [byte; 4],
    pub prevblending: [byte; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouth_t {
    pub mouthopen: byte,
    pub sndcount: byte,
    pub sndavg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position_history_t {
    pub animtime: libc::c_float,
    pub origin: vec3_t,
    pub angles: vec3_t,
}
pub type entity_state_t = entity_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_state_s {
    pub entityType: libc::c_int,
    pub number: libc::c_int,
    pub msg_time: libc::c_float,
    pub messagenum: libc::c_int,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub modelindex: libc::c_int,
    pub sequence: libc::c_int,
    pub frame: libc::c_float,
    pub colormap: libc::c_int,
    pub skin: libc::c_short,
    pub solid: libc::c_short,
    pub effects: libc::c_int,
    pub scale: libc::c_float,
    pub eflags: byte,
    pub rendermode: libc::c_int,
    pub renderamt: libc::c_int,
    pub rendercolor: color24,
    pub renderfx: libc::c_int,
    pub movetype: libc::c_int,
    pub animtime: libc::c_float,
    pub framerate: libc::c_float,
    pub body: libc::c_int,
    pub controller: [byte; 4],
    pub blending: [byte; 4],
    pub velocity: vec3_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub aiment: libc::c_int,
    pub owner: libc::c_int,
    pub friction: libc::c_float,
    pub gravity: libc::c_float,
    pub team: libc::c_int,
    pub playerclass: libc::c_int,
    pub health: libc::c_int,
    pub spectator: qboolean,
    pub weaponmodel: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub basevelocity: vec3_t,
    pub usehull: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub onground: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub startpos: vec3_t,
    pub endpos: vec3_t,
    pub impacttime: libc::c_float,
    pub starttime: libc::c_float,
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mfacebevel_t {
    pub edges: *mut mplane_t,
    pub numedges: libc::c_int,
    pub origin: vec3_t,
    pub radius: vec_t,
    pub contents: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedface_t {
    pub surf: *mut msurface_t,
    pub cull: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_globals_s {
    pub developer: qboolean,
    pub time: libc::c_float,
    pub oldtime: libc::c_float,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fullScreen: qboolean,
    pub wideScreen: qboolean,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub draw_surfaces: *mut sortedface_t,
    pub max_surfaces: libc::c_int,
    pub visbytes: size_t,
    pub desktopBitsPixel: libc::c_int,
}
pub type ref_globals_t = ref_globals_s;
pub type fixed16_t = libc::c_int;
pub type pixel_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub buffer: *mut pixel_t,
    pub colormap: [pixel_t; 262144],
    pub screen: [pixel_t; 65536],
    pub screen32: [libc::c_uint; 65536],
    pub addmap: [byte; 65536],
    pub modmap: [byte; 65536],
    pub alphamap: [pixel_t; 786432],
    pub color: pixel_t,
    pub is2d: qboolean,
    pub alpha: byte,
    pub rendermode: libc::c_int,
    pub rowbytes: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct espan_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub count: libc::c_int,
    pub pnext: *mut espan_s,
}
pub type espan_t = espan_s;
/*
Copyright (C) 1997-2001 Id Software, Inc.

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA  02111-1307, USA.

*/
// d_scan.c
//
// Portable C scan-level rasterization code, all pixel depths.
#[no_mangle]
pub static mut r_turb_pbase: *mut pixel_t =
    0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut r_turb_pdest: *mut pixel_t =
    0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut r_turb_pz: *mut libc::c_short =
    0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut r_turb_s: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_t: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_sstep: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_tstep: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_izistep: libc::c_int = 0;
#[no_mangle]
pub static mut r_turb_izi: libc::c_int = 0;
#[no_mangle]
pub static mut r_turb_turb: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
static mut r_turb_spancount: libc::c_int = 0;
#[no_mangle]
pub static mut alpha: libc::c_int = 0;
/*
=============
D_WarpScreen

this performs a slight compression of the screen at the same time as
the sine warp, to keep the edges from wrapping
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_WarpScreen() { }
/*
=============
D_DrawTurbulent8Span
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_DrawTurbulent8Span() {
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    loop  {
        sturb =
            r_turb_s +
                *r_turb_turb.offset((r_turb_t >> 16 as libc::c_int &
                                         128 as libc::c_int -
                                             1 as libc::c_int) as isize) >>
                16 as libc::c_int & 63 as libc::c_int;
        tturb =
            r_turb_t +
                *r_turb_turb.offset((r_turb_s >> 16 as libc::c_int &
                                         128 as libc::c_int -
                                             1 as libc::c_int) as isize) >>
                16 as libc::c_int & 63 as libc::c_int;
        let fresh0 = r_turb_pdest;
        r_turb_pdest = r_turb_pdest.offset(1);
        *fresh0 =
            *r_turb_pbase.offset((tturb << 6 as libc::c_int) as
                                     isize).offset(sturb as isize);
        r_turb_s += r_turb_sstep;
        r_turb_t += r_turb_tstep;
        r_turb_spancount -= 1;
        if !(r_turb_spancount > 0 as libc::c_int) { break ; }
    };
}
/*
=============
D_DrawTurbulent8Span
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_DrawTurbulent8ZSpan() {
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    loop  {
        sturb =
            r_turb_s +
                *r_turb_turb.offset((r_turb_t >> 16 as libc::c_int &
                                         128 as libc::c_int -
                                             1 as libc::c_int) as isize) >>
                16 as libc::c_int & 63 as libc::c_int;
        tturb =
            r_turb_t +
                *r_turb_turb.offset((r_turb_s >> 16 as libc::c_int &
                                         128 as libc::c_int -
                                             1 as libc::c_int) as isize) >>
                16 as libc::c_int & 63 as libc::c_int;
        if *r_turb_pz as libc::c_int <= r_turb_izi >> 16 as libc::c_int {
            let mut btemp: pixel_t =
                *r_turb_pbase.offset((tturb << 6 as libc::c_int) as
                                         isize).offset(sturb as isize);
            if alpha == 7 as libc::c_int {
                *r_turb_pdest = btemp
            } else {
                *r_turb_pdest =
                    if alpha > 3 as libc::c_int {
                        (vid.alphamap[(7 as libc::c_int - 1 as libc::c_int -
                                           alpha << 18 as libc::c_int |
                                           (*r_turb_pdest as libc::c_int &
                                                0xff00 as libc::c_int) <<
                                               2 as libc::c_int |
                                           btemp as libc::c_int >>
                                               6 as libc::c_int) as usize] as
                             libc::c_int) |
                            btemp as libc::c_int & 0x3f as libc::c_int
                    } else {
                        (vid.alphamap[((alpha - 1 as libc::c_int) <<
                                           18 as libc::c_int |
                                           (btemp as libc::c_int &
                                                0xff00 as libc::c_int) <<
                                               2 as libc::c_int |
                                           *r_turb_pdest as libc::c_int >>
                                               6 as libc::c_int) as usize] as
                             libc::c_int) |
                            *r_turb_pdest as libc::c_int & 0x3f as libc::c_int
                    } as pixel_t
            }
        }
        r_turb_pdest = r_turb_pdest.offset(1);
        r_turb_pz = r_turb_pz.offset(1);
        r_turb_izi += r_turb_izistep;
        r_turb_s += r_turb_sstep;
        r_turb_t += r_turb_tstep;
        r_turb_spancount -= 1;
        if !(r_turb_spancount > 0 as libc::c_int) { break ; }
    };
}
// !id386
/*
=============
Turbulent8
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Turbulent8(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0; // keep compiler happy
    let mut snext: fixed16_t = 0; // ditto
    let mut tnext: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz16stepu: libc::c_float = 0.;
    let mut tdivz16stepu: libc::c_float = 0.;
    let mut zi16stepu: libc::c_float = 0.;
    r_turb_turb =
        sintable.as_mut_ptr().offset((((*gpGlobals).time *
                                           20 as libc::c_int as libc::c_float)
                                          as libc::c_int &
                                          128 as libc::c_int -
                                              1 as libc::c_int) as isize);
    r_turb_sstep = 0 as libc::c_int;
    r_turb_tstep = 0 as libc::c_int;
    r_turb_pbase = cacheblock;
    sdivz16stepu = d_sdivzstepu * 16 as libc::c_int as libc::c_float;
    tdivz16stepu = d_tdivzstepu * 16 as libc::c_int as libc::c_float;
    zi16stepu = d_zistepu * 16 as libc::c_int as libc::c_float;
    loop  {
        r_turb_pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        r_turb_s = (sdivz * z) as libc::c_int + sadjust;
        if r_turb_s > bbextents {
            r_turb_s = bbextents
        } else if r_turb_s < 0 as libc::c_int { r_turb_s = 0 as libc::c_int }
        r_turb_t = (tdivz * z) as libc::c_int + tadjust;
        if r_turb_t > bbextentt {
            r_turb_t = bbextentt
        } else if r_turb_t < 0 as libc::c_int { r_turb_t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 16 as libc::c_int {
                r_turb_spancount = 16 as libc::c_int
            } else { r_turb_spancount = count }
            count -= r_turb_spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz16stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz16stepu; // prevent round-off error on <0 steps from
                zi += zi16stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int
                }
                r_turb_sstep = snext - r_turb_s >> 4 as libc::c_int;
                r_turb_tstep = tnext - r_turb_t >> 4 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			// can't step off polygon), clamp, calculate s and t steps across
			// span by division, biasing steps low so we don't run off the
			// texture
                spancountminus1 =
                    (r_turb_spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int
                }
                if r_turb_spancount > 1 as libc::c_int {
                    r_turb_sstep =
                        (snext - r_turb_s) /
                            (r_turb_spancount - 1 as libc::c_int);
                    r_turb_tstep =
                        (tnext - r_turb_t) /
                            (r_turb_spancount - 1 as libc::c_int)
                }
            }
            r_turb_s =
                r_turb_s &
                    ((128 as libc::c_int) << 16 as libc::c_int) -
                        1 as libc::c_int;
            r_turb_t =
                r_turb_t &
                    ((128 as libc::c_int) << 16 as libc::c_int) -
                        1 as libc::c_int;
            D_DrawTurbulent8Span();
            r_turb_s = snext;
            r_turb_t = tnext;
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
/*
=============
Turbulent8
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TurbulentZ8(mut pspan: *mut espan_t,
                                     mut alpha1: libc::c_int) {
    let mut count: libc::c_int = 0; // keep compiler happy
    let mut snext: fixed16_t = 0; // ditto
    let mut tnext: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz16stepu: libc::c_float = 0.;
    let mut tdivz16stepu: libc::c_float = 0.;
    let mut zi16stepu: libc::c_float = 0.;
    alpha = alpha1;
    if alpha > 7 as libc::c_int { alpha = 7 as libc::c_int }
    if alpha == 0 as libc::c_int { return }
    r_turb_turb =
        sintable.as_mut_ptr().offset((((*gpGlobals).time *
                                           20 as libc::c_int as libc::c_float)
                                          as libc::c_int &
                                          128 as libc::c_int -
                                              1 as libc::c_int) as isize);
    r_turb_sstep = 0 as libc::c_int;
    r_turb_tstep = 0 as libc::c_int;
    r_turb_pbase = cacheblock;
    sdivz16stepu = d_sdivzstepu * 16 as libc::c_int as libc::c_float;
    tdivz16stepu = d_tdivzstepu * 16 as libc::c_int as libc::c_float;
    zi16stepu = d_zistepu * 16 as libc::c_int as libc::c_float;
    r_turb_izistep =
        (d_zistepu * 0x8000 as libc::c_int as libc::c_float *
             0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    loop  {
        r_turb_pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        r_turb_pz =
            d_pzbuffer.offset(d_zwidth.wrapping_mul((*pspan).v as
                                                        libc::c_uint) as
                                  isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        r_turb_izi =
            (zi * 0x8000 as libc::c_int as libc::c_float *
                 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
        r_turb_s = (sdivz * z) as libc::c_int + sadjust;
        if r_turb_s > bbextents {
            r_turb_s = bbextents
        } else if r_turb_s < 0 as libc::c_int { r_turb_s = 0 as libc::c_int }
        r_turb_t = (tdivz * z) as libc::c_int + tadjust;
        if r_turb_t > bbextentt {
            r_turb_t = bbextentt
        } else if r_turb_t < 0 as libc::c_int { r_turb_t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 16 as libc::c_int {
                r_turb_spancount = 16 as libc::c_int
            } else { r_turb_spancount = count }
            count -= r_turb_spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz16stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz16stepu; // prevent round-off error on <0 steps from
                zi += zi16stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int
                }
                r_turb_sstep = snext - r_turb_s >> 4 as libc::c_int;
                r_turb_tstep = tnext - r_turb_t >> 4 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			// can't step off polygon), clamp, calculate s and t steps across
			// span by division, biasing steps low so we don't run off the
			// texture
                spancountminus1 =
                    (r_turb_spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int
                }
                if r_turb_spancount > 1 as libc::c_int {
                    r_turb_sstep =
                        (snext - r_turb_s) /
                            (r_turb_spancount - 1 as libc::c_int);
                    r_turb_tstep =
                        (tnext - r_turb_t) /
                            (r_turb_spancount - 1 as libc::c_int)
                }
            }
            r_turb_s =
                r_turb_s &
                    ((128 as libc::c_int) << 16 as libc::c_int) -
                        1 as libc::c_int;
            r_turb_t =
                r_turb_t &
                    ((128 as libc::c_int) << 16 as libc::c_int) -
                        1 as libc::c_int;
            D_DrawTurbulent8ZSpan();
            r_turb_s = snext;
            r_turb_t = tnext;
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
//====================
//PGM
/*
=============
NonTurbulent8 - this is for drawing scrolling textures. they're warping water textures
	but the turbulence is automatically 0.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn NonTurbulent8(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz16stepu: libc::c_float = 0.;
    let mut tdivz16stepu: libc::c_float = 0.;
    let mut zi16stepu: libc::c_float = 0.;
    //	r_turb_turb = sintable + ((int)(r_newrefdef.time*SPEED)&(CYCLE-1));
    r_turb_turb = blanktable.as_mut_ptr(); // keep compiler happy
    r_turb_sstep = 0 as libc::c_int; // ditto
    r_turb_tstep = 0 as libc::c_int;
    r_turb_pbase = cacheblock;
    sdivz16stepu = d_sdivzstepu * 16 as libc::c_int as libc::c_float;
    tdivz16stepu = d_tdivzstepu * 16 as libc::c_int as libc::c_float;
    zi16stepu = d_zistepu * 16 as libc::c_int as libc::c_float;
    loop  {
        r_turb_pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        r_turb_s = (sdivz * z) as libc::c_int + sadjust;
        if r_turb_s > bbextents {
            r_turb_s = bbextents
        } else if r_turb_s < 0 as libc::c_int { r_turb_s = 0 as libc::c_int }
        r_turb_t = (tdivz * z) as libc::c_int + tadjust;
        if r_turb_t > bbextentt {
            r_turb_t = bbextentt
        } else if r_turb_t < 0 as libc::c_int { r_turb_t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 16 as libc::c_int {
                r_turb_spancount = 16 as libc::c_int
            } else { r_turb_spancount = count }
            count -= r_turb_spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz16stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz16stepu; // prevent round-off error on <0 steps from
                zi += zi16stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int
                }
                r_turb_sstep = snext - r_turb_s >> 4 as libc::c_int;
                r_turb_tstep = tnext - r_turb_t >> 4 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			// can't step off polygon), clamp, calculate s and t steps across
			// span by division, biasing steps low so we don't run off the
			// texture
                spancountminus1 =
                    (r_turb_spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int
                }
                if r_turb_spancount > 1 as libc::c_int {
                    r_turb_sstep =
                        (snext - r_turb_s) /
                            (r_turb_spancount - 1 as libc::c_int);
                    r_turb_tstep =
                        (tnext - r_turb_t) /
                            (r_turb_spancount - 1 as libc::c_int)
                }
            }
            r_turb_s =
                r_turb_s &
                    ((128 as libc::c_int) << 16 as libc::c_int) -
                        1 as libc::c_int;
            r_turb_t =
                r_turb_t &
                    ((128 as libc::c_int) << 16 as libc::c_int) -
                        1 as libc::c_int;
            D_DrawTurbulent8Span();
            r_turb_s = snext;
            r_turb_t = tnext;
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
//PGM
//====================
#[no_mangle]
pub static mut kernel: [[[libc::c_int; 2]; 2]; 2] =
    [[[16384 as libc::c_int, 0 as libc::c_int],
      [49152 as libc::c_int, 32768 as libc::c_int]],
     [[32768 as libc::c_int, 49152 as libc::c_int],
      [0 as libc::c_int, 16384 as libc::c_int]]];
/*
=============
D_DrawSpans16

  FIXME: actually make this subdivide by 16 instead of 8!!!
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_DrawSpans16(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0; // keep compiler happy
    let mut spancount: libc::c_int = 0; // ditto
    let mut pbase: *mut pixel_t = 0 as *mut pixel_t;
    let mut pdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut s: fixed16_t = 0;
    let mut t: fixed16_t = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sstep: fixed16_t = 0;
    let mut tstep: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz8stepu: libc::c_float = 0.;
    let mut tdivz8stepu: libc::c_float = 0.;
    let mut zi8stepu: libc::c_float = 0.;
    sstep = 0 as libc::c_int;
    tstep = 0 as libc::c_int;
    pbase = cacheblock;
    sdivz8stepu = d_sdivzstepu * 8 as libc::c_int as libc::c_float;
    tdivz8stepu = d_tdivzstepu * 8 as libc::c_int as libc::c_float;
    zi8stepu = d_zistepu * 8 as libc::c_int as libc::c_float;
    loop  {
        pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        s = (sdivz * z) as libc::c_int + sadjust;
        if s > bbextents {
            s = bbextents
        } else if s < 0 as libc::c_int { s = 0 as libc::c_int }
        t = (tdivz * z) as libc::c_int + tadjust;
        if t > bbextentt {
            t = bbextentt
        } else if t < 0 as libc::c_int { t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 8 as libc::c_int {
                spancount = 8 as libc::c_int
            } else { spancount = count }
            count -= spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz8stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz8stepu; // prevent round-off error on <0 steps from
                zi += zi8stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                sstep = snext - s >> 3 as libc::c_int;
                tstep = tnext - t >> 3 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			  // can't step off polygon), clamp, calculate s and t steps across
			  // span by division, biasing steps low so we don't run off the
			  // texture
                spancountminus1 =
                    (spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                if spancount > 1 as libc::c_int {
                    sstep = (snext - s) / (spancount - 1 as libc::c_int);
                    tstep = (tnext - t) / (spancount - 1 as libc::c_int)
                }
            }
            // Drawing phrase
            if !((*sw_texfilt).value == 1.0f32) {
                loop  {
                    let fresh1 = pdest;
                    pdest = pdest.offset(1);
                    *fresh1 =
                        *pbase.offset((s >> 16 as libc::c_int) as
                                          isize).offset(((t >>
                                                              16 as
                                                                  libc::c_int)
                                                             * cachewidth) as
                                                            isize);
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            } else {
                loop  {
                    let mut idiths: libc::c_int = s;
                    let mut iditht: libc::c_int = t;
                    let mut X: libc::c_int =
                        (*pspan).u + spancount & 1 as libc::c_int;
                    let mut Y: libc::c_int = (*pspan).v & 1 as libc::c_int;
                    //Using the kernel
                    idiths +=
                        kernel[X as
                                   usize][Y as
                                              usize][0 as libc::c_int as
                                                         usize];
                    iditht +=
                        kernel[X as
                                   usize][Y as
                                              usize][1 as libc::c_int as
                                                         usize];
                    idiths = idiths >> 16 as libc::c_int;
                    idiths =
                        if idiths != 0 {
                            (idiths) - 1 as libc::c_int
                        } else { idiths };
                    iditht = iditht >> 16 as libc::c_int;
                    iditht =
                        if iditht != 0 {
                            (iditht) - 1 as libc::c_int
                        } else { iditht };
                    let fresh2 = pdest;
                    pdest = pdest.offset(1);
                    *fresh2 =
                        *pbase.offset(idiths as
                                          isize).offset((iditht * cachewidth)
                                                            as isize);
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            }
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
/*
=============
D_DrawSpans16

  FIXME: actually make this subdivide by 16 instead of 8!!!
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_AlphaSpans16(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0; // keep compiler happy
    let mut spancount: libc::c_int = 0; // ditto
    let mut pbase: *mut pixel_t = 0 as *mut pixel_t;
    let mut pdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut s: fixed16_t = 0;
    let mut t: fixed16_t = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sstep: fixed16_t = 0;
    let mut tstep: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz8stepu: libc::c_float = 0.;
    let mut tdivz8stepu: libc::c_float = 0.;
    let mut zi8stepu: libc::c_float = 0.;
    let mut izi: libc::c_int = 0;
    let mut izistep: libc::c_int = 0;
    let mut pz: *mut libc::c_short = 0 as *mut libc::c_short;
    sstep = 0 as libc::c_int;
    tstep = 0 as libc::c_int;
    pbase = cacheblock;
    sdivz8stepu = d_sdivzstepu * 8 as libc::c_int as libc::c_float;
    tdivz8stepu = d_tdivzstepu * 8 as libc::c_int as libc::c_float;
    zi8stepu = d_zistepu * 8 as libc::c_int as libc::c_float;
    izistep =
        (d_zistepu * 0x8000 as libc::c_int as libc::c_float *
             0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    loop  {
        pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        pz =
            d_pzbuffer.offset(d_zwidth.wrapping_mul((*pspan).v as
                                                        libc::c_uint) as
                                  isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        izi =
            (zi * 0x8000 as libc::c_int as libc::c_float *
                 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        s = (sdivz * z) as libc::c_int + sadjust;
        if s > bbextents {
            s = bbextents
        } else if s < 0 as libc::c_int { s = 0 as libc::c_int }
        t = (tdivz * z) as libc::c_int + tadjust;
        if t > bbextentt {
            t = bbextentt
        } else if t < 0 as libc::c_int { t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 8 as libc::c_int {
                spancount = 8 as libc::c_int
            } else { spancount = count }
            count -= spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz8stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz8stepu; // prevent round-off error on <0 steps from
                zi += zi8stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                sstep = snext - s >> 3 as libc::c_int;
                tstep = tnext - t >> 3 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			  // can't step off polygon), clamp, calculate s and t steps across
			  // span by division, biasing steps low so we don't run off the
			  // texture
                spancountminus1 =
                    (spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                if spancount > 1 as libc::c_int {
                    sstep = (snext - s) / (spancount - 1 as libc::c_int);
                    tstep = (tnext - t) / (spancount - 1 as libc::c_int)
                }
            }
            // Drawing phrase
            if !((*sw_texfilt).value == 1.0f32) {
                loop  {
                    let mut btemp: pixel_t = 0;
                    btemp =
                        *pbase.offset((s >> 16 as libc::c_int) as
                                          isize).offset(((t >>
                                                              16 as
                                                                  libc::c_int)
                                                             * cachewidth) as
                                                            isize);
                    if btemp as libc::c_int != 0x349 as libc::c_int {
                        if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                            *pdest = btemp;
                            *pz = (izi >> 16 as libc::c_int) as libc::c_short
                        }
                    }
                    pdest = pdest.offset(1);
                    pz = pz.offset(1);
                    izi += izistep;
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            } else {
                loop  {
                    if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                        let mut idiths: libc::c_int = s;
                        let mut iditht: libc::c_int = t;
                        let mut X: libc::c_int =
                            (*pspan).u + spancount & 1 as libc::c_int;
                        let mut Y: libc::c_int =
                            (*pspan).v & 1 as libc::c_int;
                        let mut btemp_0: pixel_t = 0;
                        //Using the kernel
                        idiths +=
                            kernel[X as
                                       usize][Y as
                                                  usize][0 as libc::c_int as
                                                             usize];
                        iditht +=
                            kernel[X as
                                       usize][Y as
                                                  usize][1 as libc::c_int as
                                                             usize];
                        idiths = idiths >> 16 as libc::c_int;
                        idiths =
                            if idiths != 0 {
                                (idiths) - 1 as libc::c_int
                            } else { idiths };
                        iditht = iditht >> 16 as libc::c_int;
                        iditht =
                            if iditht != 0 {
                                (iditht) - 1 as libc::c_int
                            } else { iditht };
                        btemp_0 =
                            *pbase.offset(idiths as
                                              isize).offset((iditht *
                                                                 cachewidth)
                                                                as isize);
                        if btemp_0 as libc::c_int != 0x349 as libc::c_int {
                            *pdest = btemp_0;
                            *pz = (izi >> 16 as libc::c_int) as libc::c_short
                        }
                    }
                    pdest = pdest.offset(1);
                    pz = pz.offset(1);
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            }
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
/*
=============
D_DrawSpans16

  FIXME: actually make this subdivide by 16 instead of 8!!!
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_BlendSpans16(mut pspan: *mut espan_t,
                                        mut alpha_0: libc::c_int) {
    let mut count: libc::c_int = 0; // keep compiler happy
    let mut spancount: libc::c_int = 0; // ditto
    let mut pbase: *mut pixel_t = 0 as *mut pixel_t;
    let mut pdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut s: fixed16_t = 0;
    let mut t: fixed16_t = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sstep: fixed16_t = 0;
    let mut tstep: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz8stepu: libc::c_float = 0.;
    let mut tdivz8stepu: libc::c_float = 0.;
    let mut zi8stepu: libc::c_float = 0.;
    let mut izi: libc::c_int = 0;
    let mut izistep: libc::c_int = 0;
    let mut pz: *mut libc::c_short = 0 as *mut libc::c_short;
    if alpha_0 > 7 as libc::c_int { alpha_0 = 7 as libc::c_int }
    if alpha_0 == 0 as libc::c_int { return }
    sstep = 0 as libc::c_int;
    tstep = 0 as libc::c_int;
    pbase = cacheblock;
    sdivz8stepu = d_sdivzstepu * 8 as libc::c_int as libc::c_float;
    tdivz8stepu = d_tdivzstepu * 8 as libc::c_int as libc::c_float;
    zi8stepu = d_zistepu * 8 as libc::c_int as libc::c_float;
    izistep =
        (d_zistepu * 0x8000 as libc::c_int as libc::c_float *
             0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    loop  {
        pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        pz =
            d_pzbuffer.offset(d_zwidth.wrapping_mul((*pspan).v as
                                                        libc::c_uint) as
                                  isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        izi =
            (zi * 0x8000 as libc::c_int as libc::c_float *
                 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        s = (sdivz * z) as libc::c_int + sadjust;
        if s > bbextents {
            s = bbextents
        } else if s < 0 as libc::c_int { s = 0 as libc::c_int }
        t = (tdivz * z) as libc::c_int + tadjust;
        if t > bbextentt {
            t = bbextentt
        } else if t < 0 as libc::c_int { t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 8 as libc::c_int {
                spancount = 8 as libc::c_int
            } else { spancount = count }
            count -= spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz8stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz8stepu; // prevent round-off error on <0 steps from
                zi += zi8stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                sstep = snext - s >> 3 as libc::c_int;
                tstep = tnext - t >> 3 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			  // can't step off polygon), clamp, calculate s and t steps across
			  // span by division, biasing steps low so we don't run off the
			  // texture
                spancountminus1 =
                    (spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                if spancount > 1 as libc::c_int {
                    sstep = (snext - s) / (spancount - 1 as libc::c_int);
                    tstep = (tnext - t) / (spancount - 1 as libc::c_int)
                }
            }
            // Drawing phrase
            if !((*sw_texfilt).value == 1.0f32) {
                loop  {
                    if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                        let mut btemp: pixel_t = 0;
                        btemp =
                            *pbase.offset((s >> 16 as libc::c_int) as
                                              isize).offset(((t >>
                                                                  16 as
                                                                      libc::c_int)
                                                                 * cachewidth)
                                                                as isize);
                        if btemp as libc::c_int != 0x349 as libc::c_int {
                            if alpha_0 != 7 as libc::c_int {
                                btemp =
                                    if alpha_0 > 3 as libc::c_int {
                                        (vid.alphamap[(7 as libc::c_int -
                                                           1 as libc::c_int -
                                                           alpha_0 <<
                                                           18 as libc::c_int |
                                                           (*pdest as
                                                                libc::c_int &
                                                                0xff00 as
                                                                    libc::c_int)
                                                               <<
                                                               2 as
                                                                   libc::c_int
                                                           |
                                                           btemp as
                                                               libc::c_int >>
                                                               6 as
                                                                   libc::c_int)
                                                          as usize] as
                                             libc::c_int) |
                                            btemp as libc::c_int &
                                                0x3f as libc::c_int
                                    } else {
                                        (vid.alphamap[((alpha_0 -
                                                            1 as libc::c_int)
                                                           <<
                                                           18 as libc::c_int |
                                                           (btemp as
                                                                libc::c_int &
                                                                0xff00 as
                                                                    libc::c_int)
                                                               <<
                                                               2 as
                                                                   libc::c_int
                                                           |
                                                           *pdest as
                                                               libc::c_int >>
                                                               6 as
                                                                   libc::c_int)
                                                          as usize] as
                                             libc::c_int) |
                                            *pdest as libc::c_int &
                                                0x3f as libc::c_int
                                    } as pixel_t
                            }
                            *pdest = btemp
                        }
                        //*pz    = izi >> 16;
                    }
                    pdest = pdest.offset(1);
                    pz = pz.offset(1);
                    izi += izistep;
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            } else {
                loop  {
                    let mut idiths: libc::c_int = s;
                    let mut iditht: libc::c_int = t;
                    let mut X: libc::c_int =
                        (*pspan).u + spancount & 1 as libc::c_int;
                    let mut Y: libc::c_int = (*pspan).v & 1 as libc::c_int;
                    if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                        let mut btemp_0: pixel_t = 0;
                        //*pz    = izi >> 16;
                        idiths +=
                            kernel[X as
                                       usize][Y as
                                                  usize][0 as libc::c_int as
                                                             usize];
                        iditht +=
                            kernel[X as
                                       usize][Y as
                                                  usize][1 as libc::c_int as
                                                             usize];
                        idiths = idiths >> 16 as libc::c_int;
                        idiths =
                            if idiths != 0 {
                                (idiths) - 1 as libc::c_int
                            } else { idiths };
                        iditht = iditht >> 16 as libc::c_int;
                        iditht =
                            if iditht != 0 {
                                (iditht) - 1 as libc::c_int
                            } else { iditht };
                        btemp_0 =
                            *pbase.offset(idiths as
                                              isize).offset((iditht *
                                                                 cachewidth)
                                                                as isize);
                        if btemp_0 as libc::c_int != 0x349 as libc::c_int {
                            if alpha_0 != 7 as libc::c_int {
                                btemp_0 =
                                    if alpha_0 > 3 as libc::c_int {
                                        (vid.alphamap[(7 as libc::c_int -
                                                           1 as libc::c_int -
                                                           alpha_0 <<
                                                           18 as libc::c_int |
                                                           (*pdest as
                                                                libc::c_int &
                                                                0xff00 as
                                                                    libc::c_int)
                                                               <<
                                                               2 as
                                                                   libc::c_int
                                                           |
                                                           btemp_0 as
                                                               libc::c_int >>
                                                               6 as
                                                                   libc::c_int)
                                                          as usize] as
                                             libc::c_int) |
                                            btemp_0 as libc::c_int &
                                                0x3f as libc::c_int
                                    } else {
                                        (vid.alphamap[((alpha_0 -
                                                            1 as libc::c_int)
                                                           <<
                                                           18 as libc::c_int |
                                                           (btemp_0 as
                                                                libc::c_int &
                                                                0xff00 as
                                                                    libc::c_int)
                                                               <<
                                                               2 as
                                                                   libc::c_int
                                                           |
                                                           *pdest as
                                                               libc::c_int >>
                                                               6 as
                                                                   libc::c_int)
                                                          as usize] as
                                             libc::c_int) |
                                            *pdest as libc::c_int &
                                                0x3f as libc::c_int
                                    } as pixel_t
                            }
                            *pdest = btemp_0
                        }
                    }
                    pdest = pdest.offset(1);
                    pz = pz.offset(1);
                    izi += izistep;
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            }
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
//Using the kernel
/*
=============
D_DrawSpans16

  FIXME: actually make this subdivide by 16 instead of 8!!!
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_AddSpans16(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0; // keep compiler happy
    let mut spancount: libc::c_int = 0; // ditto
    let mut pbase: *mut pixel_t = 0 as *mut pixel_t;
    let mut pdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut s: fixed16_t = 0;
    let mut t: fixed16_t = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sstep: fixed16_t = 0;
    let mut tstep: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz8stepu: libc::c_float = 0.;
    let mut tdivz8stepu: libc::c_float = 0.;
    let mut zi8stepu: libc::c_float = 0.;
    let mut izi: libc::c_int = 0;
    let mut izistep: libc::c_int = 0;
    let mut pz: *mut libc::c_short = 0 as *mut libc::c_short;
    sstep = 0 as libc::c_int;
    tstep = 0 as libc::c_int;
    pbase = cacheblock;
    sdivz8stepu = d_sdivzstepu * 8 as libc::c_int as libc::c_float;
    tdivz8stepu = d_tdivzstepu * 8 as libc::c_int as libc::c_float;
    zi8stepu = d_zistepu * 8 as libc::c_int as libc::c_float;
    izistep =
        (d_zistepu * 0x8000 as libc::c_int as libc::c_float *
             0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    loop  {
        pdest =
            d_viewbuffer.offset((r_screenwidth * (*pspan).v) as
                                    isize).offset((*pspan).u as isize);
        pz =
            d_pzbuffer.offset(d_zwidth.wrapping_mul((*pspan).v as
                                                        libc::c_uint) as
                                  isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial s/z, t/z, 1/z, s, and t and clamp
        du = (*pspan).u as libc::c_float; // prescale to 16.16 fixed-point
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        izi =
            (zi * 0x8000 as libc::c_int as libc::c_float *
                 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        s = (sdivz * z) as libc::c_int + sadjust;
        if s > bbextents {
            s = bbextents
        } else if s < 0 as libc::c_int { s = 0 as libc::c_int }
        t = (tdivz * z) as libc::c_int + tadjust;
        if t > bbextentt {
            t = bbextentt
        } else if t < 0 as libc::c_int { t = 0 as libc::c_int }
        loop  {
            // calculate s and t at the far end of the span
            if count >= 8 as libc::c_int {
                spancount = 8 as libc::c_int
            } else { spancount = count }
            count -= spancount;
            if count != 0 {
                // calculate s/z, t/z, zi->fixed s and t at far end of span,
			// calculate s and t steps across span by shifting
                sdivz += sdivz8stepu; // prescale to 16.16 fixed-point
                tdivz +=
                    tdivz8stepu; // prevent round-off error on <0 steps from
                zi += zi8stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                sstep = snext - s >> 3 as libc::c_int;
                tstep = tnext - t >> 3 as libc::c_int
            } else {
                // calculate s/z, t/z, zi->fixed s and t at last pixel in span (so
			  // can't step off polygon), clamp, calculate s and t steps across
			  // span by division, biasing steps low so we don't run off the
			  // texture
                spancountminus1 =
                    (spancount - 1 as libc::c_int) as
                        libc::c_float; // prescale to 16.16 fixed-point
                sdivz +=
                    d_sdivzstepu *
                        spancountminus1; // prevent round-off error on <0 steps from
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int
                }
                //  from causing overstepping & running off the
								//  edge of the texture
                tnext =
                    (tdivz * z) as libc::c_int +
                        tadjust; // guard against round-off error on <0 steps
                if tnext > bbextentt {
                    tnext = bbextentt
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int
                }
                if spancount > 1 as libc::c_int {
                    sstep = (snext - s) / (spancount - 1 as libc::c_int);
                    tstep = (tnext - t) / (spancount - 1 as libc::c_int)
                }
            }
            // Drawing phrase
            if !((*sw_texfilt).value == 1.0f32) {
                loop  {
                    if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                        let mut btemp: pixel_t = 0;
                        btemp =
                            *pbase.offset((s >> 16 as libc::c_int) as
                                              isize).offset(((t >>
                                                                  16 as
                                                                      libc::c_int)
                                                                 * cachewidth)
                                                                as isize);
                        if btemp as libc::c_int != 0x349 as libc::c_int {
                            btemp =
                                ((vid.addmap[(btemp as libc::c_int &
                                                  0xff00 as libc::c_int |
                                                  *pdest as libc::c_int >>
                                                      8 as libc::c_int) as
                                                 usize] as libc::c_int) <<
                                     8 as libc::c_int |
                                     *pdest as libc::c_int &
                                         0xff as libc::c_int |
                                     (btemp as libc::c_int &
                                          0xff as libc::c_int) >>
                                         0 as libc::c_int) as pixel_t;
                            *pdest = btemp
                        }
                        //*pz    = izi >> 16;
                    }
                    pdest = pdest.offset(1);
                    pz = pz.offset(1);
                    izi += izistep;
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            } else {
                loop  {
                    let mut idiths: libc::c_int = s;
                    let mut iditht: libc::c_int = t;
                    let mut X: libc::c_int =
                        (*pspan).u + spancount & 1 as libc::c_int;
                    let mut Y: libc::c_int = (*pspan).v & 1 as libc::c_int;
                    if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                        let mut btemp_0: pixel_t = 0;
                        //*pz    = izi >> 16;
                        idiths +=
                            kernel[X as
                                       usize][Y as
                                                  usize][0 as libc::c_int as
                                                             usize];
                        iditht +=
                            kernel[X as
                                       usize][Y as
                                                  usize][1 as libc::c_int as
                                                             usize];
                        idiths = idiths >> 16 as libc::c_int;
                        idiths =
                            if idiths != 0 {
                                (idiths) - 1 as libc::c_int
                            } else { idiths };
                        iditht = iditht >> 16 as libc::c_int;
                        iditht =
                            if iditht != 0 {
                                (iditht) - 1 as libc::c_int
                            } else { iditht };
                        btemp_0 =
                            *pbase.offset(idiths as
                                              isize).offset((iditht *
                                                                 cachewidth)
                                                                as isize);
                        if btemp_0 as libc::c_int != 0x349 as libc::c_int {
                            btemp_0 =
                                ((vid.addmap[(btemp_0 as libc::c_int &
                                                  0xff00 as libc::c_int |
                                                  *pdest as libc::c_int >>
                                                      8 as libc::c_int) as
                                                 usize] as libc::c_int) <<
                                     8 as libc::c_int |
                                     *pdest as libc::c_int &
                                         0xff as libc::c_int |
                                     (btemp_0 as libc::c_int &
                                          0xff as libc::c_int) >>
                                         0 as libc::c_int) as pixel_t;
                            *pdest = btemp_0
                        }
                    }
                    pdest = pdest.offset(1);
                    pz = pz.offset(1);
                    izi += izistep;
                    s += sstep;
                    t += tstep;
                    spancount -= 1;
                    if !(spancount > 0 as libc::c_int) { break ; }
                }
            }
            if !(count > 0 as libc::c_int) { break ; }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
//Using the kernel
/*
=============
D_DrawZSpans
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_DrawZSpans(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0;
    let mut doublecount: libc::c_int = 0;
    let mut izistep: libc::c_int = 0;
    let mut izi: libc::c_int = 0;
    let mut pdest: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut ltemp: libc::c_uint = 0;
    let mut zi: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    // FIXME: check for clamping/range problems
// we count on FP exceptions being turned off to avoid range problems
    izistep =
        (d_zistepu * 0x8000 as libc::c_int as libc::c_float *
             0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    loop  {
        pdest =
            d_pzbuffer.offset(d_zwidth.wrapping_mul((*pspan).v as
                                                        libc::c_uint) as
                                  isize).offset((*pspan).u as isize);
        count = (*pspan).count;
        // calculate the initial 1/z
        du = (*pspan).u as libc::c_float;
        dv = (*pspan).v as libc::c_float;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        // we count on FP exceptions being turned off to avoid range problems
        izi =
            (zi * 0x8000 as libc::c_int as libc::c_float *
                 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
        if pdest as libc::c_long & 0x2 as libc::c_int as libc::c_long != 0 {
            let fresh3 = pdest;
            pdest = pdest.offset(1);
            *fresh3 = (izi >> 16 as libc::c_int) as libc::c_short;
            izi += izistep;
            count -= 1
        }
        doublecount = count >> 1 as libc::c_int;
        if doublecount > 0 as libc::c_int {
            loop  {
                ltemp = (izi >> 16 as libc::c_int) as libc::c_uint;
                izi += izistep;
                ltemp |= izi as libc::c_uint & 0xffff0000 as libc::c_uint;
                izi += izistep;
                *(pdest as *mut libc::c_int) = ltemp as libc::c_int;
                pdest = pdest.offset(2 as libc::c_int as isize);
                doublecount -= 1;
                if !(doublecount > 0 as libc::c_int) { break ; }
            }
        }
        if count & 1 as libc::c_int != 0 {
            *pdest = (izi >> 16 as libc::c_int) as libc::c_short
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() { break ; }
    };
}
