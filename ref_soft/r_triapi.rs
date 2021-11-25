#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    fn fmodf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_Bind(tmu: libc::c_int, texnum: libc::c_uint);
    #[no_mangle]
    fn R_WorldToScreen(point: *const vec_t, screen: *mut vec_t)
     -> libc::c_int;
    #[no_mangle]
    fn R_GetSpriteTexture(m_pSpriteModel: *const model_s, frame: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static mut r_affinetridesc: affinetridesc_t;
    #[no_mangle]
    fn R_RenderTriangle(fv1: *mut finalvert_t, fv2: *mut finalvert_t,
                        fv3: *mut finalvert_t);
    #[no_mangle]
    fn R_SetupFinalVert(fv: *mut finalvert_t, x: libc::c_float,
                        y: libc::c_float, z: libc::c_float,
                        light_0: libc::c_int, s_0: libc::c_int,
                        t_0: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type matrix4x4 = [[vec_t; 4]; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed = 5;
pub const kRenderTransAlpha: C2RustUnnamed = 4;
pub const kRenderGlow: C2RustUnnamed = 3;
pub const kRenderTransTexture: C2RustUnnamed = 2;
pub const kRenderTransColor: C2RustUnnamed = 1;
pub const kRenderNormal: C2RustUnnamed = 0;
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
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_0 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_0 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_0 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_0 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_0 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_0 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_0 = -1;
pub type pixel_t = libc::c_ushort;
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
pub struct draw_list_t {
    pub edge_entities: [*mut cl_entity_t; 2048],
    pub solid_entities: [*mut cl_entity_t; 2048],
    pub trans_entities: [*mut cl_entity_t; 2048],
    pub beam_entities: [*mut cl_entity_t; 2048],
    pub num_edge_entities: uint,
    pub num_solid_entities: uint,
    pub num_trans_entities: uint,
    pub num_beam_entities: uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_globals_t {
    pub defaultTexture: libc::c_int,
    pub particleTexture: libc::c_int,
    pub whiteTexture: libc::c_int,
    pub grayTexture: libc::c_int,
    pub blackTexture: libc::c_int,
    pub solidskyTexture: libc::c_int,
    pub alphaskyTexture: libc::c_int,
    pub lightmapTextures: [libc::c_int; 256],
    pub dlightTexture: libc::c_int,
    pub skyboxTextures: [libc::c_int; 6],
    pub cinTexture: libc::c_int,
    pub draw_stack: [draw_list_t; 2],
    pub draw_stack_pos: libc::c_int,
    pub draw_list: *mut draw_list_t,
    pub draw_decals: [*mut msurface_t; 4096],
    pub num_draw_decals: libc::c_int,
    pub modelviewIdentity: qboolean,
    pub visframecount: libc::c_int,
    pub dlightframecount: libc::c_int,
    pub realframecount: libc::c_int,
    pub framecount: libc::c_int,
    pub ignore_lightgamma: qboolean,
    pub fCustomRendering: qboolean,
    pub fResetVis: qboolean,
    pub fFlipViewModel: qboolean,
    pub recursion_level: libc::c_int,
    pub max_recursion: libc::c_int,
    pub visbytes: [byte; 4096],
    pub lightstylevalue: [libc::c_int; 64],
    pub block_size: libc::c_int,
    pub frametime: libc::c_double,
    pub blend: libc::c_float,
    pub modelorg: vec3_t,
    pub fCustomSkybox: qboolean,
    pub sample_size: libc::c_int,
    pub sample_bits: uint,
    pub map_unload: qboolean,
}
/*
gl_triapi.c - TriAPI draw methods
Copyright (C) 2011 Uncle Mike
Copyright (C) 2019 a1batross

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub renderMode: libc::c_int,
    pub triRGBA: vec4_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct affinetridesc_t {
    pub pskin: *mut libc::c_void,
    pub pskindesc: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub ptriangles: *mut dtriangle_t,
    pub pfinalverts: *mut finalvert_t,
    pub numtriangles: libc::c_int,
    pub drawtype: libc::c_int,
    pub seamfixupX16: libc::c_int,
    pub do_vis_thresh: qboolean,
    pub vis_thresh: libc::c_int,
}
pub type finalvert_t = finalvert_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtriangle_t {
    pub index_xyz: [libc::c_short; 3],
    pub index_st: [libc::c_short; 3],
}
static mut ds: C2RustUnnamed_1 =
    C2RustUnnamed_1{renderMode: 0, triRGBA: [0.; 4],};
#[no_mangle]
pub static mut triv: [finalvert_t; 3] =
    [finalvert_t{u: 0,
                 v: 0,
                 s: 0,
                 t: 0,
                 l: 0,
                 zi: 0,
                 flags: 0,
                 xyz: [0.; 3],}; 3];
#[no_mangle]
pub static mut vertcount: libc::c_int = 0;
#[no_mangle]
pub static mut n: libc::c_int = 0;
#[no_mangle]
pub static mut mode: libc::c_int = 0;
#[no_mangle]
pub static mut s: libc::c_short = 0;
#[no_mangle]
pub static mut t: libc::c_short = 0;
#[no_mangle]
pub static mut light: uint = 0;
/*
===============================================================

  TRIAPI IMPLEMENTATION

===============================================================
*/
/*
=============
TriRenderMode

set rendermode
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriRenderMode(mut mode_0: libc::c_int) {
    vid.rendermode = mode_0;
    ds.renderMode = vid.rendermode;
}
/*
=============
TriBegin

begin triangle sequence
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriBegin(mut mode1: libc::c_int) {
    if mode1 == 2 as libc::c_int { mode1 = 1 as libc::c_int }
    mode = mode1;
    vertcount = 0 as libc::c_int;
    n = vertcount;
    vertcount = n;
}
/*
=============
TriEnd

draw triangle sequence
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriEnd() {
    //if( vertcount == 3 )
	//pglEnd( );
}
/*
=============
_TriColor4f

=============
*/
#[no_mangle]
pub unsafe extern "C" fn _TriColor4f(mut rr: libc::c_float,
                                     mut gg: libc::c_float,
                                     mut bb: libc::c_float,
                                     mut aa: libc::c_float) {
    //pglColor4f( r, g, b, a );
    let mut r: libc::c_ushort = 0;
    let mut g: libc::c_ushort = 0;
    let mut b: libc::c_ushort = 0;
    let mut major: libc::c_uint = 0;
    let mut minor: libc::c_uint = 0;
    if vid.rendermode == kRenderTransAdd as libc::c_int ||
           vid.rendermode == kRenderGlow as libc::c_int {
        rr *= aa;
        gg *= aa;
        bb *= aa
    }
    //gEngfuncs.Con_Printf("%d\n", vid.alpha);
    light =
        ((rr + gg + bb) * 31 as libc::c_int as libc::c_float /
             3 as libc::c_int as libc::c_float) as uint;
    if light > 31 as libc::c_int as libc::c_uint {
        light = 31 as libc::c_int as uint
    }
    if vid.is2d as u64 == 0 && vid.rendermode == kRenderNormal as libc::c_int
       {
        return
    }
    vid.alpha = (aa * 7 as libc::c_int as libc::c_float) as byte;
    if vid.alpha as libc::c_int > 7 as libc::c_int {
        vid.alpha = 7 as libc::c_int as byte
    }
    if rr == 1 as libc::c_int as libc::c_float &&
           gg == 1 as libc::c_int as libc::c_float &&
           bb == 1 as libc::c_int as libc::c_float {
        vid.color = 0xffff as libc::c_int as pixel_t;
        return
    }
    r = (rr * 31 as libc::c_int as libc::c_float) as libc::c_ushort;
    g = (gg * 63 as libc::c_int as libc::c_float) as libc::c_ushort;
    b = (bb * 31 as libc::c_int as libc::c_float) as libc::c_ushort;
    if r as libc::c_int > 31 as libc::c_int {
        r = 31 as libc::c_int as libc::c_ushort
    }
    if g as libc::c_int > 63 as libc::c_int {
        g = 63 as libc::c_int as libc::c_ushort
    }
    if b as libc::c_int > 31 as libc::c_int {
        b = 31 as libc::c_int as libc::c_ushort
    }
    major =
        ((r as libc::c_int >> 2 as libc::c_int) as libc::c_uint &
             ((1 as libc::c_uint) <<
                  3 as
                      libc::c_int).wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint)) <<
            5 as libc::c_int |
            ((g as libc::c_int >> 3 as libc::c_int) as libc::c_uint &
                 ((1 as libc::c_uint) <<
                      3 as
                          libc::c_int).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint)) <<
                2 as libc::c_int |
            (b as libc::c_int >> 3 as libc::c_int) as libc::c_uint &
                ((1 as libc::c_uint) <<
                     2 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
    // save minor GBRGBRGB
    minor =
        (((r as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int) >>
              1 as libc::c_int) << 5 as libc::c_int |
             ((r as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int) >>
                  0 as libc::c_int) << 2 as libc::c_int |
             ((g as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int) >>
                  2 as libc::c_int) << 7 as libc::c_int |
             ((g as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int) >>
                  1 as libc::c_int) << 4 as libc::c_int |
             ((g as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int) >>
                  0 as libc::c_int) << 1 as libc::c_int |
             ((b as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int) >>
                  2 as libc::c_int) << 6 as libc::c_int |
             ((b as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int) >>
                  1 as libc::c_int) << 3 as libc::c_int |
             ((b as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int) >>
                  0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    vid.color =
        (major << 8 as libc::c_int |
             minor & 0xff as libc::c_int as libc::c_uint) as pixel_t;
}
/*
=============
TriColor4ub

=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriColor4ub(mut r: byte, mut g: byte, mut b: byte,
                                     mut a: byte) {
    ds.triRGBA[0 as libc::c_int as usize] =
        r as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    ds.triRGBA[1 as libc::c_int as usize] =
        g as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    ds.triRGBA[2 as libc::c_int as usize] =
        b as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    ds.triRGBA[3 as libc::c_int as usize] =
        a as libc::c_int as libc::c_float * (1.0f32 / 255.0f32);
    _TriColor4f(ds.triRGBA[0 as libc::c_int as usize],
                ds.triRGBA[1 as libc::c_int as usize],
                ds.triRGBA[2 as libc::c_int as usize], 1.0f32);
}
/*
=============
TriColor4ub

=============
*/
#[no_mangle]
pub unsafe extern "C" fn _TriColor4ub(mut r: byte, mut g: byte, mut b: byte,
                                      mut a: byte) {
    _TriColor4f(r as libc::c_int as libc::c_float * (1.0f32 / 255.0f32),
                g as libc::c_int as libc::c_float * (1.0f32 / 255.0f32),
                b as libc::c_int as libc::c_float * (1.0f32 / 255.0f32),
                a as libc::c_int as libc::c_float * (1.0f32 / 255.0f32));
}
/*
=================
TriColor4f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn TriColor4f(mut r: libc::c_float,
                                    mut g: libc::c_float,
                                    mut b: libc::c_float,
                                    mut a: libc::c_float) {
    //if( a < 0.5 )
	//	a = 1;
    if ds.renderMode == kRenderTransAlpha as libc::c_int {
        TriColor4ub((r * 255.0f32) as byte, (g * 255.0f32) as byte,
                    (b * 255.0f32) as byte, (a * 255.0f32) as byte);
    } else { _TriColor4f(r * a, g * a, b * a, 1.0f64 as libc::c_float); }
    ds.triRGBA[0 as libc::c_int as usize] = r;
    ds.triRGBA[1 as libc::c_int as usize] = g;
    ds.triRGBA[2 as libc::c_int as usize] = b;
    ds.triRGBA[3 as libc::c_int as usize] = a;
}
/*
=============
TriTexCoord2f

=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriTexCoord2f(mut u: libc::c_float,
                                       mut v: libc::c_float) {
    let mut u1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut v1: libc::c_double = 0 as libc::c_int as libc::c_double;
    u = fmodf(u, 10 as libc::c_int as libc::c_float);
    v = fmodf(v, 10 as libc::c_int as libc::c_float);
    if u < 1000 as libc::c_int as libc::c_float &&
           u > -(1000 as libc::c_int) as libc::c_float {
        u1 = u as libc::c_double
    }
    if v < 1000 as libc::c_int as libc::c_float &&
           v > -(1000 as libc::c_int) as libc::c_float {
        v1 = v as libc::c_double
    }
    while u1 < 0 as libc::c_int as libc::c_double {
        u1 = u1 + 1 as libc::c_int as libc::c_double
    }
    while v1 < 0 as libc::c_int as libc::c_double {
        v1 = v1 + 1 as libc::c_int as libc::c_double
    }
    while u1 > 1 as libc::c_int as libc::c_double {
        u1 = u1 - 1 as libc::c_int as libc::c_double
    }
    while v1 > 1 as libc::c_int as libc::c_double {
        v1 = v1 - 1 as libc::c_int as libc::c_double
    }
    s =
        (r_affinetridesc.skinwidth as libc::c_double *
             (if u1 >= 0.01f64 {
                  (if u1 < 0.99f64 { u1 } else { 0.99f64 })
              } else { 0.01f64 })) as libc::c_short;
    t =
        (r_affinetridesc.skinheight as libc::c_double *
             (if v1 >= 0.01f64 {
                  (if v1 < 0.99f64 { v1 } else { 0.99f64 })
              } else { 0.01f64 })) as libc::c_short;
}
/*
=============
TriVertex3fv

=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriVertex3fv(mut v: *const libc::c_float) {
    //pglVertex3fv( v );
    TriVertex3f(*v.offset(0 as libc::c_int as isize),
                *v.offset(1 as libc::c_int as isize),
                *v.offset(2 as libc::c_int as isize));
}
/*
=============
TriVertex3f

=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriVertex3f(mut x: libc::c_float,
                                     mut y: libc::c_float,
                                     mut z: libc::c_float) {
    if mode == 0 as libc::c_int {
        R_SetupFinalVert(&mut *triv.as_mut_ptr().offset(vertcount as isize),
                         x, y, z, (light << 8 as libc::c_int) as libc::c_int,
                         s as libc::c_int, t as libc::c_int);
        vertcount += 1;
        if vertcount == 3 as libc::c_int {
            R_RenderTriangle(&mut *triv.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize),
                             &mut *triv.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize),
                             &mut *triv.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize));
            //R_RenderTriangle( &triv[2], &triv[1], &triv[0] );
            vertcount = 0 as libc::c_int
        }
    }
    if mode == 1 as libc::c_int {
        R_SetupFinalVert(&mut *triv.as_mut_ptr().offset(vertcount as isize),
                         x, y, z, (light << 8 as libc::c_int) as libc::c_int,
                         s as libc::c_int, t as libc::c_int);
        vertcount += 1;
        if vertcount >= 3 as libc::c_int {
            R_RenderTriangle(&mut *triv.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize),
                             &mut *triv.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize),
                             &mut *triv.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize));
            //R_RenderTriangle( &triv[2], &triv[1], &triv[0] );
            triv[1 as libc::c_int as usize] = triv[2 as libc::c_int as usize];
            vertcount = 2 as libc::c_int
        }
    }
    if mode == 5 as libc::c_int {
        R_SetupFinalVert(&mut *triv.as_mut_ptr().offset(n as isize), x, y, z,
                         (light << 8 as libc::c_int) as libc::c_int,
                         s as libc::c_int, t as libc::c_int);
        n += 1;
        vertcount += 1;
        if n == 3 as libc::c_int { n = 0 as libc::c_int }
        if vertcount >= 3 as libc::c_int {
            if vertcount & 1 as libc::c_int != 0 {
                R_RenderTriangle(&mut *triv.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                                 &mut *triv.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                                 &mut *triv.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
            } else {
                R_RenderTriangle(&mut *triv.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                                 &mut *triv.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                                 &mut *triv.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
            }
        }
    };
}
/*
=============
TriWorldToScreen

convert world coordinates (x,y,z) into screen (x, y)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriWorldToScreen(mut world: *const libc::c_float,
                                          mut screen: *mut libc::c_float)
 -> libc::c_int {
    let mut retval: libc::c_int = 0;
    retval = R_WorldToScreen(world, screen);
    *screen.offset(0 as libc::c_int as isize) =
        0.5f32 * *screen.offset(0 as libc::c_int as isize) *
            RI.viewport[2 as libc::c_int as usize] as libc::c_float;
    *screen.offset(1 as libc::c_int as isize) =
        -0.5f32 * *screen.offset(1 as libc::c_int as isize) *
            RI.viewport[3 as libc::c_int as usize] as libc::c_float;
    *screen.offset(0 as libc::c_int as isize) +=
        0.5f32 * RI.viewport[2 as libc::c_int as usize] as libc::c_float;
    *screen.offset(1 as libc::c_int as isize) +=
        0.5f32 * RI.viewport[3 as libc::c_int as usize] as libc::c_float;
    return retval;
}
/*
=============
TriSpriteTexture

bind current texture
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriSpriteTexture(mut pSpriteModel: *mut model_t,
                                          mut frame: libc::c_int)
 -> libc::c_int {
    let mut gl_texturenum: libc::c_int = 0;
    gl_texturenum = R_GetSpriteTexture(pSpriteModel, frame);
    if gl_texturenum == 0 as libc::c_int { return 0 as libc::c_int }
    if gl_texturenum <= 0 as libc::c_int ||
           gl_texturenum > 4096 as libc::c_int {
        gl_texturenum = tr.defaultTexture
    }
    GL_Bind(XASH_TEXTURE0 as libc::c_int, gl_texturenum as libc::c_uint);
    return 1 as libc::c_int;
}
/*
=============
TriFog

enables global fog on the level
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriFog(mut flFogColor: *mut libc::c_float,
                                mut flStart: libc::c_float,
                                mut flEnd: libc::c_float,
                                mut bOn: libc::c_int) {
}
/*
=============
TriGetMatrix

very strange export
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriGetMatrix(pname: libc::c_int,
                                      mut matrix: *mut libc::c_float) {
    //pglGetFloatv( pname, matrix );
}
/*
=============
TriForParams

=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriFogParams(mut flDensity: libc::c_float,
                                      mut iFogSkybox: libc::c_int) {
    //RI.fogDensity = flDensity;
	//RI.fogSkybox = iFogSkybox;
}
/*
=============
TriCullFace

=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriCullFace(mut mode_0: TRICULLSTYLE) { }
/*
=============
TriBrightness
=============
*/
#[no_mangle]
pub unsafe extern "C" fn TriBrightness(mut brightness: libc::c_float) {
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    //if( brightness < 0.5 )
//		brightness = 1; //0.5;
//ds.triRGBA[3] = 1;
    r =
        ds.triRGBA[0 as libc::c_int as usize] *
            ds.triRGBA[3 as libc::c_int as usize] * brightness;
    g =
        ds.triRGBA[1 as libc::c_int as usize] *
            ds.triRGBA[3 as libc::c_int as usize] * brightness;
    b =
        ds.triRGBA[2 as libc::c_int as usize] *
            ds.triRGBA[3 as libc::c_int as usize] * brightness;
    _TriColor4f(r, g, b, 1.0f32);
}
