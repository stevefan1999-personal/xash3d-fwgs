#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut aliastriangleparms: aliastriangleparms_t;
    #[no_mangle]
    fn R_DrawTriangle();
    #[no_mangle]
    fn R_AliasProjectAndClipTestFinalVert(fv_0: *mut finalvert_t);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type matrix4x4 = [[vec_t; 4]; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
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
pub type model_t = model_s;
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub pnext: *mut vrect_s,
}
pub type vrect_t = vrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_instance_t {
    pub params: libc::c_int,
    pub drawWorld: qboolean,
    pub isSkyVisible: qboolean,
    pub onlyClientDraw: qboolean,
    pub drawOrtho: qboolean,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub currententity: *mut cl_entity_t,
    pub currentmodel: *mut model_t,
    pub currentbeam: *mut cl_entity_t,
    pub viewport: [libc::c_int; 4],
    pub viewleaf: *mut mleaf_t,
    pub oldviewleaf: *mut mleaf_t,
    pub pvsorigin: vec3_t,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub base_vup: vec3_t,
    pub base_vpn: vec3_t,
    pub base_vright: vec3_t,
    pub cullorigin: vec3_t,
    pub cull_vforward: vec3_t,
    pub cull_vright: vec3_t,
    pub cull_vup: vec3_t,
    pub cached_contents: libc::c_int,
    pub cached_waterlevel: libc::c_int,
    pub farClip: libc::c_float,
    pub skyMins: [[libc::c_float; 6]; 2],
    pub skyMaxs: [[libc::c_float; 6]; 2],
    pub objectMatrix: matrix4x4,
    pub worldviewMatrix: matrix4x4,
    pub modelviewMatrix: matrix4x4,
    pub projectionMatrix: matrix4x4,
    pub worldviewProjectionMatrix: matrix4x4,
    pub visbytes: [byte; 4096],
    pub viewplanedist: libc::c_float,
    pub vrect: vrect_t,
    pub aliasvrect: vrect_t,
    pub vrectright: libc::c_int,
    pub vrectbottom: libc::c_int,
    pub aliasvrectright: libc::c_int,
    pub aliasvrectbottom: libc::c_int,
    pub vrectrightedge: libc::c_float,
    pub fvrectx: libc::c_float,
    pub fvrecty: libc::c_float,
    pub fvrectx_adj: libc::c_float,
    pub fvrecty_adj: libc::c_float,
    pub vrect_x_adj_shift20: libc::c_int,
    pub vrectright_adj_shift20: libc::c_int,
    pub fvrectright_adj: libc::c_float,
    pub fvrectbottom_adj: libc::c_float,
    pub fvrectright: libc::c_float,
    pub fvrectbottom: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct finalvert_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub s: libc::c_int,
    pub t: libc::c_int,
    pub l: libc::c_int,
    pub zi: libc::c_int,
    pub flags: libc::c_int,
    pub xyz: [libc::c_float; 3],
}
pub type finalvert_t = finalvert_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliastriangleparms_t {
    pub a: *mut finalvert_t,
    pub b: *mut finalvert_t,
    pub c: *mut finalvert_t,
}
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
// r_aclip.c: clip routines for drawing Alias models directly to the screen
static mut fv: [[finalvert_t; 8]; 2] =
    [[finalvert_t{u: 0,
                  v: 0,
                  s: 0,
                  t: 0,
                  l: 0,
                  zi: 0,
                  flags: 0,
                  xyz: [0.; 3],}; 8]; 2];
/*
================
R_Alias_clip_z

pfv0 is the unclipped vertex, pfv1 is the z-clipped vertex
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_z(mut pfv0: *mut finalvert_t,
                                        mut pfv1: *mut finalvert_t,
                                        mut out: *mut finalvert_t) {
    let mut scale: libc::c_float = 0.;
    scale =
        (4 as libc::c_int as libc::c_float -
             (*pfv0).xyz[2 as libc::c_int as usize]) /
            ((*pfv1).xyz[2 as libc::c_int as usize] -
                 (*pfv0).xyz[2 as libc::c_int as usize]);
    (*out).xyz[0 as libc::c_int as usize] =
        (*pfv0).xyz[0 as libc::c_int as usize] +
            ((*pfv1).xyz[0 as libc::c_int as usize] -
                 (*pfv0).xyz[0 as libc::c_int as usize]) * scale;
    (*out).xyz[1 as libc::c_int as usize] =
        (*pfv0).xyz[1 as libc::c_int as usize] +
            ((*pfv1).xyz[1 as libc::c_int as usize] -
                 (*pfv0).xyz[1 as libc::c_int as usize]) * scale;
    (*out).xyz[2 as libc::c_int as usize] = 4 as libc::c_int as libc::c_float;
    (*out).s =
        ((*pfv0).s as libc::c_float +
             ((*pfv1).s - (*pfv0).s) as libc::c_float * scale) as libc::c_int;
    (*out).t =
        ((*pfv0).t as libc::c_float +
             ((*pfv1).t - (*pfv0).t) as libc::c_float * scale) as libc::c_int;
    (*out).l =
        ((*pfv0).l as libc::c_float +
             ((*pfv1).l - (*pfv0).l) as libc::c_float * scale) as libc::c_int;
    R_AliasProjectAndClipTestFinalVert(out);
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_left(mut pfv0: *mut finalvert_t,
                                           mut pfv1: *mut finalvert_t,
                                           mut out: *mut finalvert_t) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale =
            (RI.aliasvrect.x - (*pfv0).u) as libc::c_float /
                ((*pfv1).u - (*pfv0).u) as libc::c_float;
        (*out).u =
            ((*pfv0).u as libc::c_float +
                 ((*pfv1).u - (*pfv0).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv0).v as libc::c_float +
                 ((*pfv1).v - (*pfv0).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv0).s as libc::c_float +
                 ((*pfv1).s - (*pfv0).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv0).t as libc::c_float +
                 ((*pfv1).t - (*pfv0).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv0).l as libc::c_float +
                 ((*pfv1).l - (*pfv0).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv0).zi as libc::c_float +
                 ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    } else {
        scale =
            (RI.aliasvrect.x - (*pfv1).u) as libc::c_float /
                ((*pfv0).u - (*pfv1).u) as libc::c_float;
        (*out).u =
            ((*pfv1).u as libc::c_float +
                 ((*pfv0).u - (*pfv1).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv1).v as libc::c_float +
                 ((*pfv0).v - (*pfv1).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv1).s as libc::c_float +
                 ((*pfv0).s - (*pfv1).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv1).t as libc::c_float +
                 ((*pfv0).t - (*pfv1).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv1).l as libc::c_float +
                 ((*pfv0).l - (*pfv1).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv1).zi as libc::c_float +
                 ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_right(mut pfv0: *mut finalvert_t,
                                            mut pfv1: *mut finalvert_t,
                                            mut out: *mut finalvert_t) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale =
            (RI.aliasvrectright - (*pfv0).u) as libc::c_float /
                ((*pfv1).u - (*pfv0).u) as libc::c_float;
        (*out).u =
            ((*pfv0).u as libc::c_float +
                 ((*pfv1).u - (*pfv0).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv0).v as libc::c_float +
                 ((*pfv1).v - (*pfv0).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv0).s as libc::c_float +
                 ((*pfv1).s - (*pfv0).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv0).t as libc::c_float +
                 ((*pfv1).t - (*pfv0).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv0).l as libc::c_float +
                 ((*pfv1).l - (*pfv0).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv0).zi as libc::c_float +
                 ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    } else {
        scale =
            (RI.aliasvrectright - (*pfv1).u) as libc::c_float /
                ((*pfv0).u - (*pfv1).u) as libc::c_float;
        (*out).u =
            ((*pfv1).u as libc::c_float +
                 ((*pfv0).u - (*pfv1).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv1).v as libc::c_float +
                 ((*pfv0).v - (*pfv1).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv1).s as libc::c_float +
                 ((*pfv0).s - (*pfv1).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv1).t as libc::c_float +
                 ((*pfv0).t - (*pfv1).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv1).l as libc::c_float +
                 ((*pfv0).l - (*pfv1).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv1).zi as libc::c_float +
                 ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_top(mut pfv0: *mut finalvert_t,
                                          mut pfv1: *mut finalvert_t,
                                          mut out: *mut finalvert_t) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale =
            (RI.aliasvrect.y - (*pfv0).v) as libc::c_float /
                ((*pfv1).v - (*pfv0).v) as libc::c_float;
        (*out).u =
            ((*pfv0).u as libc::c_float +
                 ((*pfv1).u - (*pfv0).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv0).v as libc::c_float +
                 ((*pfv1).v - (*pfv0).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv0).s as libc::c_float +
                 ((*pfv1).s - (*pfv0).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv0).t as libc::c_float +
                 ((*pfv1).t - (*pfv0).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv0).l as libc::c_float +
                 ((*pfv1).l - (*pfv0).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv0).zi as libc::c_float +
                 ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    } else {
        scale =
            (RI.aliasvrect.y - (*pfv1).v) as libc::c_float /
                ((*pfv0).v - (*pfv1).v) as libc::c_float;
        (*out).u =
            ((*pfv1).u as libc::c_float +
                 ((*pfv0).u - (*pfv1).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv1).v as libc::c_float +
                 ((*pfv0).v - (*pfv1).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv1).s as libc::c_float +
                 ((*pfv0).s - (*pfv1).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv1).t as libc::c_float +
                 ((*pfv0).t - (*pfv1).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv1).l as libc::c_float +
                 ((*pfv0).l - (*pfv1).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv1).zi as libc::c_float +
                 ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_bottom(mut pfv0: *mut finalvert_t,
                                             mut pfv1: *mut finalvert_t,
                                             mut out: *mut finalvert_t) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale =
            (RI.aliasvrectbottom - (*pfv0).v) as libc::c_float /
                ((*pfv1).v - (*pfv0).v) as libc::c_float;
        (*out).u =
            ((*pfv0).u as libc::c_float +
                 ((*pfv1).u - (*pfv0).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv0).v as libc::c_float +
                 ((*pfv1).v - (*pfv0).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv0).s as libc::c_float +
                 ((*pfv1).s - (*pfv0).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv0).t as libc::c_float +
                 ((*pfv1).t - (*pfv0).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv0).l as libc::c_float +
                 ((*pfv1).l - (*pfv0).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv0).zi as libc::c_float +
                 ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    } else {
        scale =
            (RI.aliasvrectbottom - (*pfv1).v) as libc::c_float /
                ((*pfv0).v - (*pfv1).v) as libc::c_float;
        (*out).u =
            ((*pfv1).u as libc::c_float +
                 ((*pfv0).u - (*pfv1).u) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).v =
            ((*pfv1).v as libc::c_float +
                 ((*pfv0).v - (*pfv1).v) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).s =
            ((*pfv1).s as libc::c_float +
                 ((*pfv0).s - (*pfv1).s) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).t =
            ((*pfv1).t as libc::c_float +
                 ((*pfv0).t - (*pfv1).t) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).l =
            ((*pfv1).l as libc::c_float +
                 ((*pfv0).l - (*pfv1).l) as libc::c_float * scale + 0.5f32) as
                libc::c_int;
        (*out).zi =
            ((*pfv1).zi as libc::c_float +
                 ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale + 0.5f32)
                as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasClip(mut in_0: *mut finalvert_t,
                                     mut out: *mut finalvert_t,
                                     mut flag: libc::c_int,
                                     mut count: libc::c_int,
                                     mut clip:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut finalvert_t,
                                                                     _:
                                                                         *mut finalvert_t,
                                                                     _:
                                                                         *mut finalvert_t)
                                                    -> ()>) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut oldflags: libc::c_int = 0;
    j = count - 1 as libc::c_int;
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        oldflags = (*in_0.offset(j as isize)).flags & flag;
        flags = (*in_0.offset(i as isize)).flags & flag;
        if !(flags != 0 && oldflags != 0) {
            if oldflags ^ flags != 0 {
                clip.expect("non-null function pointer")(&mut *in_0.offset(j
                                                                               as
                                                                               isize),
                                                         &mut *in_0.offset(i
                                                                               as
                                                                               isize),
                                                         &mut *out.offset(k as
                                                                              isize));
                (*out.offset(k as isize)).flags = 0 as libc::c_int;
                if (*out.offset(k as isize)).u < RI.aliasvrect.x {
                    (*out.offset(k as isize)).flags |= 0x1 as libc::c_int
                }
                if (*out.offset(k as isize)).v < RI.aliasvrect.y {
                    (*out.offset(k as isize)).flags |= 0x2 as libc::c_int
                }
                if (*out.offset(k as isize)).u > RI.aliasvrectright {
                    (*out.offset(k as isize)).flags |= 0x4 as libc::c_int
                }
                if (*out.offset(k as isize)).v > RI.aliasvrectbottom {
                    (*out.offset(k as isize)).flags |= 0x8 as libc::c_int
                }
                k += 1
            }
            if flags == 0 {
                *out.offset(k as isize) = *in_0.offset(i as isize);
                k += 1
            }
        }
        j = i;
        i += 1
    }
    return k;
}
/*
================
R_AliasClipTriangle
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasClipTriangle(mut index0: *mut finalvert_t,
                                             mut index1: *mut finalvert_t,
                                             mut index2: *mut finalvert_t) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pingpong: libc::c_int = 0;
    let mut clipflags: libc::c_uint = 0;
    // copy vertexes and fix seam texture coordinates
    fv[0 as libc::c_int as usize][0 as libc::c_int as usize] = *index0;
    fv[0 as libc::c_int as usize][1 as libc::c_int as usize] = *index1;
    fv[0 as libc::c_int as usize][2 as libc::c_int as usize] = *index2;
    // clip
    clipflags =
        (fv[0 as libc::c_int as usize][0 as libc::c_int as usize].flags |
             fv[0 as libc::c_int as usize][1 as libc::c_int as usize].flags |
             fv[0 as libc::c_int as usize][2 as libc::c_int as usize].flags)
            as libc::c_uint;
    if clipflags & 0x10 as libc::c_int as libc::c_uint != 0 {
        k =
            R_AliasClip(fv[0 as libc::c_int as usize].as_mut_ptr(),
                        fv[1 as libc::c_int as usize].as_mut_ptr(),
                        0x10 as libc::c_int, 3 as libc::c_int,
                        Some(R_Alias_clip_z as
                                 unsafe extern "C" fn(_: *mut finalvert_t,
                                                      _: *mut finalvert_t,
                                                      _: *mut finalvert_t)
                                     -> ()));
        if k == 0 as libc::c_int { return }
        pingpong = 1 as libc::c_int;
        clipflags =
            (fv[1 as libc::c_int as usize][0 as libc::c_int as usize].flags |
                 fv[1 as libc::c_int as
                        usize][1 as libc::c_int as usize].flags |
                 fv[1 as libc::c_int as
                        usize][2 as libc::c_int as usize].flags) as
                libc::c_uint
    } else { pingpong = 0 as libc::c_int; k = 3 as libc::c_int }
    if clipflags & 0x1 as libc::c_int as libc::c_uint != 0 {
        k =
            R_AliasClip(fv[pingpong as usize].as_mut_ptr(),
                        fv[(pingpong ^ 1 as libc::c_int) as
                               usize].as_mut_ptr(), 0x1 as libc::c_int, k,
                        Some(R_Alias_clip_left as
                                 unsafe extern "C" fn(_: *mut finalvert_t,
                                                      _: *mut finalvert_t,
                                                      _: *mut finalvert_t)
                                     -> ()));
        if k == 0 as libc::c_int { return }
        pingpong ^= 1 as libc::c_int
    }
    if clipflags & 0x4 as libc::c_int as libc::c_uint != 0 {
        k =
            R_AliasClip(fv[pingpong as usize].as_mut_ptr(),
                        fv[(pingpong ^ 1 as libc::c_int) as
                               usize].as_mut_ptr(), 0x4 as libc::c_int, k,
                        Some(R_Alias_clip_right as
                                 unsafe extern "C" fn(_: *mut finalvert_t,
                                                      _: *mut finalvert_t,
                                                      _: *mut finalvert_t)
                                     -> ()));
        if k == 0 as libc::c_int { return }
        pingpong ^= 1 as libc::c_int
    }
    if clipflags & 0x8 as libc::c_int as libc::c_uint != 0 {
        k =
            R_AliasClip(fv[pingpong as usize].as_mut_ptr(),
                        fv[(pingpong ^ 1 as libc::c_int) as
                               usize].as_mut_ptr(), 0x8 as libc::c_int, k,
                        Some(R_Alias_clip_bottom as
                                 unsafe extern "C" fn(_: *mut finalvert_t,
                                                      _: *mut finalvert_t,
                                                      _: *mut finalvert_t)
                                     -> ()));
        if k == 0 as libc::c_int { return }
        pingpong ^= 1 as libc::c_int
    }
    if clipflags & 0x2 as libc::c_int as libc::c_uint != 0 {
        k =
            R_AliasClip(fv[pingpong as usize].as_mut_ptr(),
                        fv[(pingpong ^ 1 as libc::c_int) as
                               usize].as_mut_ptr(), 0x2 as libc::c_int, k,
                        Some(R_Alias_clip_top as
                                 unsafe extern "C" fn(_: *mut finalvert_t,
                                                      _: *mut finalvert_t,
                                                      _: *mut finalvert_t)
                                     -> ()));
        if k == 0 as libc::c_int { return }
        pingpong ^= 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < k {
        if fv[pingpong as usize][i as usize].u < RI.aliasvrect.x {
            fv[pingpong as usize][i as usize].u = RI.aliasvrect.x
        } else if fv[pingpong as usize][i as usize].u > RI.aliasvrectright {
            fv[pingpong as usize][i as usize].u = RI.aliasvrectright
        }
        if fv[pingpong as usize][i as usize].v < RI.aliasvrect.y {
            fv[pingpong as usize][i as usize].v = RI.aliasvrect.y
        } else if fv[pingpong as usize][i as usize].v > RI.aliasvrectbottom {
            fv[pingpong as usize][i as usize].v = RI.aliasvrectbottom
        }
        fv[pingpong as usize][i as usize].flags = 0 as libc::c_int;
        i += 1
    }
    // draw triangles
    i = 1 as libc::c_int;
    while i < k - 1 as libc::c_int {
        aliastriangleparms.a =
            &mut *(*fv.as_mut_ptr().offset(pingpong as
                                               isize)).as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                as *mut finalvert_t;
        aliastriangleparms.b =
            &mut *(*fv.as_mut_ptr().offset(pingpong as
                                               isize)).as_mut_ptr().offset(i
                                                                               as
                                                                               isize)
                as *mut finalvert_t;
        aliastriangleparms.c =
            &mut *(*fv.as_mut_ptr().offset(pingpong as
                                               isize)).as_mut_ptr().offset((i
                                                                                +
                                                                                1
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               isize)
                as *mut finalvert_t;
        R_DrawTriangle();
        i += 1
    };
}
