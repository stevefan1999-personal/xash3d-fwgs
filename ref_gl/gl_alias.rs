#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type con_nprint_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    pub type world_static_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn anglemod(a: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn VectorCompareEpsilon(vec1: *const vec_t, vec2: *const vec_t,
                            epsilon: vec_t) -> qboolean;
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn AddPointToBounds(v: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn AngleQuaternion(angles: *const vec_t, q: *mut vec_t, studio: qboolean);
    #[no_mangle]
    fn QuaternionAngle(q: *const vec_t, angles: *mut vec_t);
    #[no_mangle]
    fn QuaternionSlerp(p: *const vec_t, q: *const vec_t, t: libc::c_float,
                       qt: *mut vec_t);
    #[no_mangle]
    fn Matrix3x4_VectorTransform(in_0: *const [vec_t; 4],
                                 v: *const libc::c_float,
                                 out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_VectorIRotate(in_0: *const [vec_t; 4],
                               v: *const libc::c_float,
                               out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    static mut boxpnt: [[libc::c_int; 4]; 6];
    #[no_mangle]
    static m_bytenormals: [[libc::c_float; 3]; 162];
    #[no_mangle]
    static mut pglAlphaFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLclampf) -> ()>;
    #[no_mangle]
    static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglBlendFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglColor3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglColor4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglDepthFunc: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnd: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglPointSize: Option<unsafe extern "C" fn(_: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglScalef:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglShadeModel: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglTexCoord2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexEnvf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTranslatef:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglVertex3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_CleanUpTextureUnits(last: libc::c_int);
    #[no_mangle]
    fn GL_Bind(tmu: GLint, texnum: GLenum);
    #[no_mangle]
    fn GL_MultiTexCoord2f(texture: GLenum, s: GLfloat, t: GLfloat);
    #[no_mangle]
    fn GL_SetRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn R_CullModel(e: *mut cl_entity_t, absmin: *const vec_t,
                   absmax: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn R_GetTexture(texnum: GLenum) -> *mut gl_texture_t;
    #[no_mangle]
    fn GL_LoadTextureFromBuffer(name: *const libc::c_char,
                                pic: *mut rgbdata_t, flags: texFlags_t,
                                update: qboolean) -> libc::c_int;
    #[no_mangle]
    fn GL_FreeTexture(texnum: GLenum);
    #[no_mangle]
    fn R_LightVec(start: *const vec_t, end: *const vec_t,
                  lightspot: *mut vec_t, lightvec: *mut vec_t) -> colorVec;
    #[no_mangle]
    fn R_LoadIdentity();
    #[no_mangle]
    fn R_RotateForEntity(e: *mut cl_entity_t);
    #[no_mangle]
    fn pfnPlayerInfo(index: libc::c_int) -> *mut player_info_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut r_dynamic: *mut cvar_t;
    #[no_mangle]
    static mut r_lightmap: *mut cvar_t;
    #[no_mangle]
    static mut r_fullbright: *mut cvar_t;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut glState: glstate_t;
    #[no_mangle]
    fn GL_Support(r_ext: libc::c_int) -> qboolean;
    #[no_mangle]
    fn TriColor4f(r: libc::c_float, g: libc::c_float, b: libc::c_float,
                  a: libc::c_float);
    #[no_mangle]
    fn TriBegin(mode: libc::c_int);
    #[no_mangle]
    static mut r_drawentities: *mut cvar_t;
    #[no_mangle]
    fn TriRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn TriEnd();
    #[no_mangle]
    fn TriVertex3fv(v: *const libc::c_float);
    #[no_mangle]
    fn TriBrightness(brightness: libc::c_float);
    /*
gl_alias.c - alias model renderer
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
    #[no_mangle]
    static mut r_shadows: cvar_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type rgba_t = [byte; 4];
pub type matrix4x4 = [[vec_t; 4]; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string = [libc::c_char; 256];
pub type fs_offset_t = off_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed = 5;
pub const kRenderTransAlpha: C2RustUnnamed = 4;
pub const kRenderGlow: C2RustUnnamed = 3;
pub const kRenderTransTexture: C2RustUnnamed = 2;
pub const kRenderTransColor: C2RustUnnamed = 1;
pub const kRenderNormal: C2RustUnnamed = 0;
pub type word = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alight_s {
    pub ambientlight: libc::c_int,
    pub shadelight: libc::c_int,
    pub color: vec3_t,
    pub plightvec: *mut libc::c_float,
}
pub type alight_t = alight_s;
pub type resourcetype_t = libc::c_uint;
pub const t_world: resourcetype_t = 6;
pub const t_eventscript: resourcetype_t = 5;
pub const t_generic: resourcetype_t = 4;
pub const t_decal: resourcetype_t = 3;
pub const t_model: resourcetype_t = 2;
pub const t_skin: resourcetype_t = 1;
pub const t_sound: resourcetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_s {
    pub szFileName: [libc::c_char; 64],
    pub type_0: resourcetype_t,
    pub nIndex: libc::c_int,
    pub nDownloadSize: libc::c_int,
    pub ucFlags: libc::c_uchar,
    pub rgucMD5_hash: [libc::c_uchar; 16],
    pub playernum: libc::c_uchar,
    pub rguc_reserved: [libc::c_uchar; 32],
    pub pNext: *mut resource_s,
    pub pPrev: *mut resource_s,
}
pub type resource_t = resource_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct customization_s {
    pub bInUse: qboolean,
    pub resource: resource_t,
    pub bTranslated: qboolean,
    pub nUserData1: libc::c_int,
    pub nUserData2: libc::c_int,
    pub pInfo: *mut libc::c_void,
    pub pBuffer: *mut libc::c_void,
    pub pNext: *mut customization_s,
}
pub type customization_t = customization_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_info_s {
    pub userid: libc::c_int,
    pub userinfo: [libc::c_char; 256],
    pub name: [libc::c_char; 32],
    pub spectator: libc::c_int,
    pub ping: libc::c_int,
    pub packet_loss: libc::c_int,
    pub model: [libc::c_char; 64],
    pub topcolor: libc::c_int,
    pub bottomcolor: libc::c_int,
    pub renderframe: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub gaitframe: libc::c_float,
    pub gaityaw: libc::c_float,
    pub prevgaitorigin: vec3_t,
    pub customdata: customization_t,
    pub hashedcdkey: [libc::c_char; 16],
}
pub type player_info_t = player_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trivertex_t {
    pub v: [byte; 3],
    pub lightnormalindex: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maliasframedesc_t {
    pub firstpose: libc::c_int,
    pub numposes: libc::c_int,
    pub bboxmin: trivertex_t,
    pub bboxmax: trivertex_t,
    pub interval: libc::c_float,
    pub name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliashdr_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub scale: vec3_t,
    pub scale_origin: vec3_t,
    pub boundingradius: libc::c_float,
    pub eyeposition: vec3_t,
    pub numskins: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub numverts: libc::c_int,
    pub numtris: libc::c_int,
    pub numframes: libc::c_int,
    pub synctype: libc::c_int,
    pub flags: libc::c_int,
    pub size: libc::c_float,
    pub reserved: [libc::c_int; 8],
    pub numposes: libc::c_int,
    pub poseverts: libc::c_int,
    pub posedata: *mut trivertex_t,
    pub commands: *mut libc::c_int,
    pub gl_texturenum: [[libc::c_ushort; 4]; 32],
    pub fb_texturenum: [[libc::c_ushort; 4]; 32],
    pub gl_reserved0: [[libc::c_ushort; 4]; 32],
    pub gl_reserved1: [[libc::c_ushort; 4]; 32],
    pub gl_reserved2: [[libc::c_ushort; 4]; 32],
    pub frames: [maliasframedesc_t; 1],
}
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub pattern: [libc::c_char; 256],
    pub map: [libc::c_float; 256],
    pub length: libc::c_int,
    pub value: libc::c_float,
    pub interp: qboolean,
    pub time: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_s {
    pub origin: vec3_t,
    pub radius: libc::c_float,
    pub color: color24,
    pub die: libc::c_float,
    pub decay: libc::c_float,
    pub minlight: libc::c_float,
    pub key: libc::c_int,
    pub dark: qboolean,
}
pub type dlight_t = dlight_s;
pub type texFlags_t = libc::c_uint;
pub const TF_MULTISAMPLE: texFlags_t = 536870912;
pub const TF_ARB_16BIT: texFlags_t = 268435456;
pub const TF_NOCOMPARE: texFlags_t = 134217728;
pub const TF_ARB_FLOAT: texFlags_t = 67108864;
pub const TF_IMG_UPLOADED: texFlags_t = 33554432;
pub const TF_ALPHACONTRAST: texFlags_t = 4194304;
pub const TF_ATLAS_PAGE: texFlags_t = 2097152;
pub const TF_TEXTURE_3D: texFlags_t = 1048576;
pub const TF_BORDER: texFlags_t = 524288;
pub const TF_UPDATE: texFlags_t = 262144;
pub const TF_FORCE_COLOR: texFlags_t = 131072;
pub const TF_HAS_ALPHA: texFlags_t = 65536;
pub const TF_NORMALMAP: texFlags_t = 32768;
pub const TF_MAKELUMA: texFlags_t = 16384;
pub const TF_HAS_LUMA: texFlags_t = 8192;
pub const TF_NOMIPMAP: texFlags_t = 4096;
pub const TF_CLAMP: texFlags_t = 2048;
pub const TF_SKYSIDE: texFlags_t = 1024;
pub const TF_LUMINANCE: texFlags_t = 512;
pub const TF_QUAKEPAL: texFlags_t = 256;
pub const TF_DEPTHMAP: texFlags_t = 128;
pub const TF_CUBEMAP: texFlags_t = 64;
pub const TF_RECTANGLE: texFlags_t = 32;
pub const TF_ALLOW_EMBOSS: texFlags_t = 16;
pub const TF_EXPAND_SOURCE: texFlags_t = 8;
pub const TF_NOFLIP_TGA: texFlags_t = 4;
pub const TF_KEEP_SOURCE: texFlags_t = 2;
pub const TF_NEAREST: texFlags_t = 1;
pub const TF_COLORMAP: texFlags_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modelstate_s {
    pub sequence: libc::c_short,
    pub frame: libc::c_short,
    pub blending: [byte; 2],
    pub controller: [byte; 4],
    pub poseparam: [byte; 16],
    pub body: byte,
    pub skin: byte,
    pub scale: libc::c_short,
}
pub type modelstate_t = modelstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decallist_s {
    pub position: vec3_t,
    pub name: [libc::c_char; 64],
    pub entityIndex: libc::c_short,
    pub depth: byte,
    pub flags: byte,
    pub scale: libc::c_float,
    pub impactPlaneNormal: vec3_t,
    pub studio_state: modelstate_t,
}
pub type decallist_t = decallist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_viewpass_s {
    pub viewport: [libc::c_int; 4],
    pub vieworigin: vec3_t,
    pub viewangles: vec3_t,
    pub viewentity: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_overview_s {
    pub origin: vec3_t,
    pub rotated: qboolean,
    pub xLeft: libc::c_float,
    pub xRight: libc::c_float,
    pub yTop: libc::c_float,
    pub yBottom: libc::c_float,
    pub zFar: libc::c_float,
    pub zNear: libc::c_float,
    pub flZoom: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_interface_s {
    pub version: libc::c_int,
    pub GL_RenderFrame: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> libc::c_int>,
    pub GL_BuildLightmaps: Option<unsafe extern "C" fn() -> ()>,
    pub GL_OrthoBounds: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_CreateStudioDecalList: Option<unsafe extern "C" fn(_:
                                                                 *mut decallist_t,
                                                             _: libc::c_int)
                                            -> libc::c_int>,
    pub R_ClearStudioDecals: Option<unsafe extern "C" fn() -> ()>,
    pub R_SpeedsMessage: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: size_t) -> qboolean>,
    pub Mod_ProcessUserData: Option<unsafe extern "C" fn(_: *mut model_s,
                                                         _: qboolean,
                                                         _: *const byte)
                                        -> ()>,
    pub R_ProcessEntData: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub Mod_GetCurrentVis: Option<unsafe extern "C" fn() -> *mut byte>,
    pub R_NewMap: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub CL_UpdateLatchedVars: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                          _: qboolean) -> ()>,
}
pub type render_interface_t = render_interface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_frustum_s {
    pub planes: [mplane_t; 6],
    pub clipFlags: libc::c_uint,
}
pub type gl_frustum_t = gl_frustum_s;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PF_TOTALCOUNT: C2RustUnnamed_0 = 12;
pub const PF_ATI2: C2RustUnnamed_0 = 11;
pub const PF_DXT5: C2RustUnnamed_0 = 10;
pub const PF_DXT3: C2RustUnnamed_0 = 9;
pub const PF_DXT1: C2RustUnnamed_0 = 8;
pub const PF_LUMINANCE: C2RustUnnamed_0 = 7;
pub const PF_BGR_24: C2RustUnnamed_0 = 6;
pub const PF_RGB_24: C2RustUnnamed_0 = 5;
pub const PF_BGRA_32: C2RustUnnamed_0 = 4;
pub const PF_RGBA_32: C2RustUnnamed_0 = 3;
pub const PF_INDEXED_32: C2RustUnnamed_0 = 2;
pub const PF_INDEXED_24: C2RustUnnamed_0 = 1;
pub const PF_UNKNOWN: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_1 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_1 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_1 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_1 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_1 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_1 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_1 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_1 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_1 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_1 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_1 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_1 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_1 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_1 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_1 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_1 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_1 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_1 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_1 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_1 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_1 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_1 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgbdata_s {
    pub width: word,
    pub height: word,
    pub depth: word,
    pub type_0: uint,
    pub flags: uint,
    pub encode: word,
    pub numMips: byte,
    pub palette: *mut byte,
    pub buffer: *mut byte,
    pub fogParams: rgba_t,
    pub size: size_t,
}
pub type rgbdata_t = rgbdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct studiohdr_s {
    pub ident: int32_t,
    pub version: int32_t,
    pub name: [libc::c_char; 64],
    pub length: int32_t,
    pub eyeposition: vec3_t,
    pub min: vec3_t,
    pub max: vec3_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
    pub flags: int32_t,
    pub numbones: int32_t,
    pub boneindex: int32_t,
    pub numbonecontrollers: int32_t,
    pub bonecontrollerindex: int32_t,
    pub numhitboxes: int32_t,
    pub hitboxindex: int32_t,
    pub numseq: int32_t,
    pub seqindex: int32_t,
    pub numseqgroups: int32_t,
    pub seqgroupindex: int32_t,
    pub numtextures: int32_t,
    pub textureindex: int32_t,
    pub texturedataindex: int32_t,
    pub numskinref: int32_t,
    pub numskinfamilies: int32_t,
    pub skinindex: int32_t,
    pub numbodyparts: int32_t,
    pub bodypartindex: int32_t,
    pub numattachments: int32_t,
    pub attachmentindex: int32_t,
    pub studiohdr2index: int32_t,
    pub unused: int32_t,
    pub unused2: int32_t,
    pub unused3: int32_t,
    pub numtransitions: int32_t,
    pub transitionindex: int32_t,
}
pub type studiohdr_t = studiohdr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiobone_s {
    pub name: [libc::c_char; 32],
    pub parent: int32_t,
    pub unused: int32_t,
    pub bonecontroller: [int32_t; 6],
    pub value: [vec_t; 6],
    pub scale: [vec_t; 6],
}
pub type mstudiobone_t = mstudiobone_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioseqdesc_s {
    pub label: [libc::c_char; 32],
    pub fps: vec_t,
    pub flags: int32_t,
    pub activity: int32_t,
    pub actweight: int32_t,
    pub numevents: int32_t,
    pub eventindex: int32_t,
    pub numframes: int32_t,
    pub weightlistindex: int32_t,
    pub iklockindex: int32_t,
    pub motiontype: int32_t,
    pub motionbone: int32_t,
    pub linearmovement: vec3_t,
    pub autolayerindex: int32_t,
    pub keyvalueindex: int32_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
    pub numblends: int32_t,
    pub animindex: int32_t,
    pub blendtype: [int32_t; 2],
    pub blendstart: [vec_t; 2],
    pub blendend: [vec_t; 2],
    pub groupsize: [uint8_t; 2],
    pub numautolayers: uint8_t,
    pub numiklocks: uint8_t,
    pub seqgroup: int32_t,
    pub entrynode: int32_t,
    pub exitnode: int32_t,
    pub nodeflags: uint8_t,
    pub cycleposeindex: uint8_t,
    pub fadeintime: uint8_t,
    pub fadeouttime: uint8_t,
    pub animdescindex: int32_t,
}
pub type mstudioseqdesc_t = mstudioseqdesc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioanim_s {
    pub offset: [uint16_t; 6],
}
pub type mstudioanim_t = mstudioanim_s;
pub type ptype_t = libc::c_uint;
pub const pt_clientcustom: ptype_t = 10;
pub const pt_vox_grav: ptype_t = 9;
pub const pt_vox_slowgrav: ptype_t = 8;
pub const pt_blob2: ptype_t = 7;
pub const pt_blob: ptype_t = 6;
pub const pt_explode2: ptype_t = 5;
pub const pt_explode: ptype_t = 4;
pub const pt_fire: ptype_t = 3;
pub const pt_slowgrav: ptype_t = 2;
pub const pt_grav: ptype_t = 1;
pub const pt_static: ptype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_s {
    pub org: vec3_t,
    pub color: libc::c_short,
    pub packedColor: libc::c_short,
    pub next: *mut particle_s,
    pub vel: vec3_t,
    pub ramp: libc::c_float,
    pub die: libc::c_float,
    pub type_0: ptype_t,
    pub deathfunc: Option<unsafe extern "C" fn(_: *mut particle_s) -> ()>,
    pub callback: Option<unsafe extern "C" fn(_: *mut particle_s,
                                              _: libc::c_float) -> ()>,
    pub context: libc::c_uchar,
}
pub type particle_t = particle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmtrace_s {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub inopen: qboolean,
    pub inwater: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: pmplane_t,
    pub ent: libc::c_int,
    pub deltavelocity: vec3_t,
    pub hitgroup: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmplane_t {
    pub normal: vec3_t,
    pub dist: libc::c_float,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DEMO_QUAKE1: C2RustUnnamed_2 = 2;
pub const DEMO_XASH3D: C2RustUnnamed_2 = 1;
pub const DEMO_INACTIVE: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_3 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_3 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_3 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_3 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_3 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_3 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_3 = -1;
pub type ref_defaultsprite_e = libc::c_uint;
pub const REF_CHROME_SPRITE: ref_defaultsprite_e = 1;
pub const REF_DOT_SPRITE: ref_defaultsprite_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remap_info_s {
    pub textures: [libc::c_ushort; 32],
    pub ptexture: *mut mstudiotex_s,
    pub numtextures: libc::c_short,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub model: *mut model_t,
}
pub type remap_info_t = remap_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct engine_studio_api_s {
    pub Mem_Calloc: Option<unsafe extern "C" fn(_: libc::c_int, _: size_t)
                               -> *mut libc::c_void>,
    pub Cache_Check: Option<unsafe extern "C" fn(_: *mut cache_user_s)
                                -> *mut libc::c_void>,
    pub LoadCacheFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: *mut cache_user_s)
                                  -> ()>,
    pub Mod_ForName: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_int)
                                -> *mut model_s>,
    pub Mod_Extradata: Option<unsafe extern "C" fn(_: *mut model_s)
                                  -> *mut libc::c_void>,
    pub GetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut model_s>,
    pub GetCurrentEntity: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub PlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int)
                               -> *mut player_info_s>,
    pub GetPlayerState: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut entity_state_s>,
    pub GetViewEntity: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetTimes: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                              _: *mut libc::c_double,
                                              _: *mut libc::c_double) -> ()>,
    pub GetCvar: Option<unsafe extern "C" fn(_: *const libc::c_char)
                            -> *mut cvar_s>,
    pub GetViewInfo: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float)
                                -> ()>,
    pub GetChromeSprite: Option<unsafe extern "C" fn() -> *mut model_s>,
    pub GetModelCounters: Option<unsafe extern "C" fn(_:
                                                          *mut *mut libc::c_int,
                                                      _:
                                                          *mut *mut libc::c_int)
                                     -> ()>,
    pub GetAliasScale: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub StudioGetBoneTransform: Option<unsafe extern "C" fn()
                                           ->
                                               *mut *mut *mut *mut libc::c_float>,
    pub StudioGetLightTransform: Option<unsafe extern "C" fn()
                                            ->
                                                *mut *mut *mut *mut libc::c_float>,
    pub StudioGetAliasTransform: Option<unsafe extern "C" fn()
                                            -> *mut *mut *mut libc::c_float>,
    pub StudioGetRotationMatrix: Option<unsafe extern "C" fn()
                                            -> *mut *mut *mut libc::c_float>,
    pub StudioSetupModel: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _:
                                                          *mut *mut libc::c_void,
                                                      _:
                                                          *mut *mut libc::c_void)
                                     -> ()>,
    pub StudioCheckBBox: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub StudioDynamicLight: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                        _: *mut alight_s)
                                       -> ()>,
    pub StudioEntityLight: Option<unsafe extern "C" fn(_: *mut alight_s)
                                      -> ()>,
    pub StudioSetupLighting: Option<unsafe extern "C" fn(_: *mut alight_s)
                                        -> ()>,
    pub StudioDrawPoints: Option<unsafe extern "C" fn() -> ()>,
    pub StudioDrawHulls: Option<unsafe extern "C" fn() -> ()>,
    pub StudioDrawAbsBBox: Option<unsafe extern "C" fn() -> ()>,
    pub StudioDrawBones: Option<unsafe extern "C" fn() -> ()>,
    pub StudioSetupSkin: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: libc::c_int) -> ()>,
    pub StudioSetRemapColors: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub SetupPlayerModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut model_s>,
    pub StudioClientEvents: Option<unsafe extern "C" fn() -> ()>,
    pub GetForceFaceFlags: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub SetForceFaceFlags: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub StudioSetHeader: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
    pub SetRenderModel: Option<unsafe extern "C" fn(_: *mut model_s) -> ()>,
    pub SetupRenderer: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub RestoreRenderer: Option<unsafe extern "C" fn() -> ()>,
    pub SetChromeOrigin: Option<unsafe extern "C" fn() -> ()>,
    pub IsHardware: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GL_StudioDrawShadow: Option<unsafe extern "C" fn() -> ()>,
    pub GL_SetRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub StudioSetRenderamt: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub StudioSetCullState: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub StudioRenderShadow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float)
                                       -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct r_studio_interface_s {
    pub version: libc::c_int,
    pub StudioDrawModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> libc::c_int>,
    pub StudioDrawPlayer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut entity_state_s)
                                     -> libc::c_int>,
}
pub type C2RustUnnamed_4 = libc::c_int;
pub const PARM_NUMMODELS: C2RustUnnamed_4 = -13;
pub const PARM_NUMENTITIES: C2RustUnnamed_4 = -12;
pub const PARM_LOCAL_GAME: C2RustUnnamed_4 = -11;
pub const PARM_LOCAL_HEALTH: C2RustUnnamed_4 = -10;
pub const PARM_MAX_CLIENTS: C2RustUnnamed_4 = -9;
pub const PARM_WATER_LEVEL: C2RustUnnamed_4 = -8;
pub const PARM_PLAYING_DEMO: C2RustUnnamed_4 = -7;
pub const PARM_CONNSTATE: C2RustUnnamed_4 = -6;
pub const PARM_VIEWENT_INDEX: C2RustUnnamed_4 = -5;
pub const PARM_PLAYER_INDEX: C2RustUnnamed_4 = -4;
pub const PARM_QUAKE_COMPATIBLE: C2RustUnnamed_4 = -3;
pub const PARM_THIRDPERSON: C2RustUnnamed_4 = -2;
pub const PARM_DEV_OVERVIEW: C2RustUnnamed_4 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_api_s {
    pub EngineGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub Cvar_Get: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char,
                                              _: libc::c_int,
                                              _: *const libc::c_char)
                             -> *mut cvar_t>,
    pub pfnGetCvarPointer: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: libc::c_int)
                                      -> *mut cvar_t>,
    pub pfnGetCvarFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnGetCvarString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_float) -> ()>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub Cvar_RegisterVariable: Option<unsafe extern "C" fn(_: *mut cvar_t)
                                          -> ()>,
    pub Cvar_FullSet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int) -> ()>,
    pub Cmd_AddCommand: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _:
                                                        Option<unsafe extern "C" fn()
                                                                   -> ()>,
                                                    _: *const libc::c_char)
                                   -> libc::c_int>,
    pub Cmd_RemoveCommand: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> ()>,
    pub Cmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                             -> *const libc::c_char>,
    pub Cmd_Args: Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub Cbuf_AddText: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> ()>,
    pub Cbuf_InsertText: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> ()>,
    pub Cbuf_Execute: Option<unsafe extern "C" fn() -> ()>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_Reportf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NXPrintf: Option<unsafe extern "C" fn(_: *mut con_nprint_s,
                                                  _: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub CL_CenterPrint: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_float) -> ()>,
    pub Con_DrawStringLen: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> ()>,
    pub Con_DrawString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *const libc::c_char,
                                                    _: *mut byte)
                                   -> libc::c_int>,
    pub CL_DrawCenterPrint: Option<unsafe extern "C" fn() -> ()>,
    pub GetLocalPlayer: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetViewModel: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetEntityByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_s>,
    pub R_BeamGetEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut cl_entity_s>,
    pub CL_GetWaterEntity: Option<unsafe extern "C" fn(_: *const vec_t)
                                      -> *mut cl_entity_s>,
    pub CL_AddVisibleEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                         _: libc::c_int)
                                        -> qboolean>,
    pub Mod_SampleSizeForFace: Option<unsafe extern "C" fn(_: *mut msurface_s)
                                          -> libc::c_int>,
    pub Mod_BoxVisible: Option<unsafe extern "C" fn(_: *const vec_t,
                                                    _: *const vec_t,
                                                    _: *const byte)
                                   -> qboolean>,
    pub GetWorld: Option<unsafe extern "C" fn() -> *mut world_static_s>,
    pub Mod_PointInLeaf: Option<unsafe extern "C" fn(_: *const vec_t,
                                                     _: *mut mnode_t)
                                    -> *mut mleaf_t>,
    pub Mod_CreatePolygonsForHull: Option<unsafe extern "C" fn(_: libc::c_int)
                                              -> ()>,
    pub R_StudioSlerpBones: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut vec4_t,
                                                        _:
                                                            *mut [libc::c_float; 3],
                                                        _: *mut vec4_t,
                                                        _:
                                                            *mut [libc::c_float; 3],
                                                        _: libc::c_float)
                                       -> ()>,
    pub R_StudioCalcBoneQuaternion: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    *mut mstudiobone_t,
                                                                _:
                                                                    *mut mstudioanim_t,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _: *mut vec_t)
                                               -> ()>,
    pub R_StudioCalcBonePosition: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  libc::c_float,
                                                              _:
                                                                  *mut mstudiobone_t,
                                                              _:
                                                                  *mut mstudioanim_t,
                                                              _: *mut vec_t,
                                                              _: *mut vec_t)
                                             -> ()>,
    pub R_StudioGetAnim: Option<unsafe extern "C" fn(_: *mut studiohdr_t,
                                                     _: *mut model_t,
                                                     _: *mut mstudioseqdesc_t)
                                    -> *mut libc::c_void>,
    pub pfnStudioEvent: Option<unsafe extern "C" fn(_: *const mstudioevent_s,
                                                    _: *const cl_entity_t)
                                   -> ()>,
    pub CL_DrawEFX: Option<unsafe extern "C" fn(_: libc::c_float, _: qboolean)
                               -> ()>,
    pub CL_ThinkParticle: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: *mut particle_t)
                                     -> ()>,
    pub R_FreeDeadParticles: Option<unsafe extern "C" fn(_:
                                                             *mut *mut particle_t)
                                        -> ()>,
    pub CL_AllocParticleFast: Option<unsafe extern "C" fn()
                                         -> *mut particle_t>,
    pub CL_AllocElight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_s>,
    pub GetDefaultSprite: Option<unsafe extern "C" fn(_: ref_defaultsprite_e)
                                     -> *mut model_s>,
    pub R_StoreEfrags: Option<unsafe extern "C" fn(_: *mut *mut efrag_s,
                                                   _: libc::c_int) -> ()>,
    pub Mod_ForName: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: qboolean, _: qboolean)
                                -> *mut model_t>,
    pub Mod_Extradata: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut model_t)
                                  -> *mut libc::c_void>,
    pub pfnGetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut model_s>,
    pub Mod_GetCurrentLoadingModel: Option<unsafe extern "C" fn()
                                               -> *mut model_s>,
    pub Mod_SetCurrentLoadingModel: Option<unsafe extern "C" fn(_:
                                                                    *mut model_s)
                                               -> ()>,
    pub CL_GetRemapInfoForEntity: Option<unsafe extern "C" fn(_:
                                                                  *mut cl_entity_t)
                                             -> *mut remap_info_s>,
    pub CL_AllocRemapInfo: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                       _: *mut model_t,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub CL_FreeRemapInfo: Option<unsafe extern "C" fn(_: *mut remap_info_s)
                                     -> ()>,
    pub CL_UpdateRemapInfo: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub CL_ExtraUpdate: Option<unsafe extern "C" fn() -> ()>,
    pub Host_Error: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub COM_SetRandomSeed: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub COM_RandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                     _: libc::c_float)
                                    -> libc::c_float>,
    pub COM_RandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GetScreenFade: Option<unsafe extern "C" fn() -> *mut screenfade_s>,
    pub pfnTextMessageGet: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut client_textmessage_s>,
    pub GetPredictedOrigin: Option<unsafe extern "C" fn(_: *mut vec_t) -> ()>,
    pub CL_GetPaletteColor: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut color24>,
    pub CL_GetScreenInfo: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int)
                                     -> ()>,
    pub SetLocalLightLevel: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub Sys_CheckParm: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> libc::c_int>,
    pub pfnPlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut player_info_t>,
    pub pfnGetPlayerState: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> *mut entity_state_t>,
    pub Mod_CacheCheck: Option<unsafe extern "C" fn(_: *mut cache_user_s)
                                   -> *mut libc::c_void>,
    pub Mod_LoadCacheFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut cache_user_s)
                                      -> ()>,
    pub Mod_Calloc: Option<unsafe extern "C" fn(_: libc::c_int, _: size_t)
                               -> *mut libc::c_void>,
    pub pfnGetStudioModelInterface: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut *mut r_studio_interface_s,
                                                                _:
                                                                    *mut engine_studio_api_s)
                                               -> libc::c_int>,
    pub _Mem_AllocPool: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> poolhandle_t>,
    pub _Mem_FreePool: Option<unsafe extern "C" fn(_: *mut poolhandle_t,
                                                   _: *const libc::c_char,
                                                   _: libc::c_int) -> ()>,
    pub _Mem_Alloc: Option<unsafe extern "C" fn(_: poolhandle_t, _: size_t,
                                                _: qboolean,
                                                _: *const libc::c_char,
                                                _: libc::c_int)
                               -> *mut libc::c_void>,
    pub _Mem_Realloc: Option<unsafe extern "C" fn(_: poolhandle_t,
                                                  _: *mut libc::c_void,
                                                  _: size_t, _: qboolean,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int)
                                 -> *mut libc::c_void>,
    pub _Mem_Free: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: *const libc::c_char,
                                               _: libc::c_int) -> ()>,
    pub COM_LoadLibrary: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: libc::c_int,
                                                     _: qboolean)
                                    -> *mut libc::c_void>,
    pub COM_FreeLibrary: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
    pub COM_GetProcAddress: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                        _:
                                                            *const libc::c_char)
                                       -> *mut libc::c_void>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut fs_offset_t,
                                                  _: qboolean) -> *mut byte>,
    pub FS_FileExists: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub FS_AllowDirectPaths: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_Init_Video: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> qboolean>,
    pub R_Free_Video: Option<unsafe extern "C" fn() -> ()>,
    pub GL_SetAttribute: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub GL_GetAttribute: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_int)
                                    -> libc::c_int>,
    pub GL_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut libc::c_void>,
    pub GL_SwapBuffers: Option<unsafe extern "C" fn() -> ()>,
    pub SW_CreateBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint)
                                    -> qboolean>,
    pub SW_LockBuffer: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub SW_UnlockBuffer: Option<unsafe extern "C" fn() -> ()>,
    pub BuildGammaTable: Option<unsafe extern "C" fn(_: libc::c_float,
                                                     _: libc::c_float) -> ()>,
    pub LightToTexGamma: Option<unsafe extern "C" fn(_: byte) -> byte>,
    pub R_DoResetGamma: Option<unsafe extern "C" fn() -> qboolean>,
    pub GetLightStyle: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut lightstyle_t>,
    pub GetDynamicLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut dlight_t>,
    pub GetEntityLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub R_FatPVS: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: libc::c_float, _: *mut byte,
                                              _: qboolean, _: qboolean)
                             -> libc::c_int>,
    pub GetOverviewParms: Option<unsafe extern "C" fn()
                                     -> *const ref_overview_s>,
    pub pfnTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub EV_GetPhysent: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut physent_s>,
    pub EV_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub EV_VisTraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int)
                                    -> *mut pmtrace_s>,
    pub CL_TraceLine: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                  _: *mut vec_t,
                                                  _: libc::c_int)
                                 -> pmtrace_s>,
    pub pfnGetMoveVars: Option<unsafe extern "C" fn() -> *mut movevars_s>,
    pub Image_AddCmdFlags: Option<unsafe extern "C" fn(_: uint) -> ()>,
    pub Image_SetForceFlags: Option<unsafe extern "C" fn(_: uint) -> ()>,
    pub Image_ClearForceFlags: Option<unsafe extern "C" fn() -> ()>,
    pub Image_CustomPalette: Option<unsafe extern "C" fn() -> qboolean>,
    pub Image_Process: Option<unsafe extern "C" fn(_: *mut *mut rgbdata_t,
                                                   _: libc::c_int,
                                                   _: libc::c_int, _: uint,
                                                   _: libc::c_float)
                                  -> qboolean>,
    pub FS_LoadImage: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const byte, _: size_t)
                                 -> *mut rgbdata_t>,
    pub FS_SaveImage: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut rgbdata_t)
                                 -> qboolean>,
    pub FS_CopyImage: Option<unsafe extern "C" fn(_: *mut rgbdata_t)
                                 -> *mut rgbdata_t>,
    pub FS_FreeImage: Option<unsafe extern "C" fn(_: *mut rgbdata_t) -> ()>,
    pub Image_SetMDLPointer: Option<unsafe extern "C" fn(_: *mut byte) -> ()>,
    pub Image_GetPool: Option<unsafe extern "C" fn() -> poolhandle_t>,
    pub Image_GetPFDesc: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *const bpc_desc_s>,
    pub pfnDrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawTransparentTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub drawFuncs: *mut render_interface_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movevars_s {
    pub gravity: libc::c_float,
    pub stopspeed: libc::c_float,
    pub maxspeed: libc::c_float,
    pub spectatormaxspeed: libc::c_float,
    pub accelerate: libc::c_float,
    pub airaccelerate: libc::c_float,
    pub wateraccelerate: libc::c_float,
    pub friction: libc::c_float,
    pub edgefriction: libc::c_float,
    pub waterfriction: libc::c_float,
    pub entgravity: libc::c_float,
    pub bounce: libc::c_float,
    pub stepsize: libc::c_float,
    pub maxvelocity: libc::c_float,
    pub zmax: libc::c_float,
    pub waveHeight: libc::c_float,
    pub footsteps: qboolean,
    pub skyName: [libc::c_char; 32],
    pub rollangle: libc::c_float,
    pub rollspeed: libc::c_float,
    pub skycolor_r: libc::c_float,
    pub skycolor_g: libc::c_float,
    pub skycolor_b: libc::c_float,
    pub skyvec_x: libc::c_float,
    pub skyvec_y: libc::c_float,
    pub skyvec_z: libc::c_float,
    pub features: libc::c_int,
    pub fog_settings: libc::c_int,
    pub wateralpha: libc::c_float,
    pub skydir_x: libc::c_float,
    pub skydir_y: libc::c_float,
    pub skydir_z: libc::c_float,
    pub skyangle: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct physent_s {
    pub name: [libc::c_char; 32],
    pub player: libc::c_int,
    pub origin: vec3_t,
    pub model: *mut model_s,
    pub studiomodel: *mut model_s,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub info: libc::c_int,
    pub angles: vec3_t,
    pub solid: libc::c_int,
    pub skin: libc::c_int,
    pub rendermode: libc::c_int,
    pub frame: libc::c_float,
    pub sequence: libc::c_int,
    pub controller: [byte; 4],
    pub blending: [byte; 2],
    pub movetype: libc::c_int,
    pub takedamage: libc::c_int,
    pub blooddecal: libc::c_int,
    pub team: libc::c_int,
    pub classnumber: libc::c_int,
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
pub type ref_api_t = ref_api_s;
pub type movevars_t = movevars_s;
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLint = libc::c_int;
pub type GLuint = uint;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gltexture_s {
    pub name: [libc::c_char; 256],
    pub srcWidth: word,
    pub srcHeight: word,
    pub width: word,
    pub height: word,
    pub depth: word,
    pub numMips: byte,
    pub target: GLuint,
    pub texnum: GLuint,
    pub format: GLint,
    pub encode: GLint,
    pub flags: texFlags_t,
    pub fogParams: rgba_t,
    pub original: *mut rgbdata_t,
    pub size: size_t,
    pub xscale: libc::c_float,
    pub yscale: libc::c_float,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub nextHash: *mut gltexture_s,
}
pub type gl_texture_t = gltexture_s;
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
    pub frustum: gl_frustum_t,
    pub viewleaf: *mut mleaf_t,
    pub oldviewleaf: *mut mleaf_t,
    pub pvsorigin: vec3_t,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub cullorigin: vec3_t,
    pub cull_vforward: vec3_t,
    pub cull_vright: vec3_t,
    pub cull_vup: vec3_t,
    pub farClip: libc::c_float,
    pub fogCustom: qboolean,
    pub fogEnabled: qboolean,
    pub fogSkybox: qboolean,
    pub fogColor: vec4_t,
    pub fogDensity: libc::c_float,
    pub fogStart: libc::c_float,
    pub fogEnd: libc::c_float,
    pub cached_contents: libc::c_int,
    pub cached_waterlevel: libc::c_int,
    pub skyMins: [[libc::c_float; 6]; 2],
    pub skyMaxs: [[libc::c_float; 6]; 2],
    pub objectMatrix: matrix4x4,
    pub worldviewMatrix: matrix4x4,
    pub modelviewMatrix: matrix4x4,
    pub projectionMatrix: matrix4x4,
    pub worldviewProjectionMatrix: matrix4x4,
    pub visbytes: [byte; 4096],
    pub viewplanedist: libc::c_float,
    pub clipPlane: mplane_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct draw_list_t {
    pub solid_entities: [*mut cl_entity_t; 2048],
    pub trans_entities: [*mut cl_entity_t; 2048],
    pub beam_entities: [*mut cl_entity_t; 2048],
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
    pub skytexturenum: libc::c_int,
    pub skyboxbasenum: libc::c_int,
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
    pub visbytes: [byte; 4096],
    pub lightstylevalue: [libc::c_int; 64],
    pub block_size: libc::c_int,
    pub frametime: libc::c_double,
    pub blend: libc::c_float,
    pub modelorg: vec3_t,
    pub fCustomSkybox: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_speeds_t {
    pub c_world_polys: uint,
    pub c_studio_polys: uint,
    pub c_sprite_polys: uint,
    pub c_alias_polys: uint,
    pub c_world_leafs: uint,
    pub c_view_beams_count: uint,
    pub c_active_tents_count: uint,
    pub c_alias_models_drawn: uint,
    pub c_studio_models_drawn: uint,
    pub c_sprite_models_drawn: uint,
    pub c_particle_count: uint,
    pub c_client_ents: uint,
    pub t_world_node: libc::c_double,
    pub t_world_draw: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stvert_t {
    pub onseam: libc::c_int,
    pub s: libc::c_int,
    pub t: libc::c_int,
}
pub type dtriangle_t = dtriangle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtriangle_s {
    pub facesfront: libc::c_int,
    pub vertindex: [libc::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasframetype_t {
    pub type_0: aliasframetype_t,
}
pub type aliasframetype_t = libc::c_uint;
pub const ALIAS_GROUP: aliasframetype_t = 1;
pub const ALIAS_SINGLE: aliasframetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasframe_t {
    pub bboxmin: trivertex_t,
    pub bboxmax: trivertex_t,
    pub name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasinterval_t {
    pub interval: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasgroup_t {
    pub numframes: libc::c_int,
    pub bboxmin: trivertex_t,
    pub bboxmax: trivertex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasskintype_t {
    pub type_0: aliasskintype_t,
}
pub type aliasskintype_t = libc::c_uint;
pub const ALIAS_SKIN_GROUP: aliasskintype_t = 1;
pub const ALIAS_SKIN_SINGLE: aliasskintype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasskingroup_t {
    pub numskins: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasskininterval_t {
    pub interval: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliashdr_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub scale: vec3_t,
    pub scale_origin: vec3_t,
    pub boundingradius: libc::c_float,
    pub eyeposition: vec3_t,
    pub numskins: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub numverts: libc::c_int,
    pub numtris: libc::c_int,
    pub numframes: libc::c_int,
    pub synctype: synctype_t,
    pub flags: libc::c_int,
    pub size: libc::c_float,
}
pub type synctype_t = libc::c_uint;
pub const ST_RAND: synctype_t = 1;
pub const ST_SYNC: synctype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glstate_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub activeTMU: libc::c_int,
    pub currentTextures: [GLint; 32],
    pub currentTextureTargets: [GLuint; 32],
    pub texIdentityMatrix: [GLboolean; 32],
    pub genSTEnabled: [GLint; 32],
    pub texCoordArrayMode: [GLint; 32],
    pub isFogEnabled: GLint,
    pub faceCull: libc::c_int,
    pub stencilEnabled: qboolean,
    pub in2DMode: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias_draw_state_t {
    pub time: libc::c_double,
    pub frametime: libc::c_double,
    pub framecount: libc::c_int,
    pub interpolate: qboolean,
    pub ambientlight: libc::c_float,
    pub shadelight: libc::c_float,
    pub lightvec: vec3_t,
    pub lightvec_local: vec3_t,
    pub lightspot: vec3_t,
    pub lightcolor: vec3_t,
    pub oldpose: libc::c_int,
    pub newpose: libc::c_int,
    pub lerpfrac: libc::c_float,
}
pub const GL_ARB_MULTITEXTURE: C2RustUnnamed_5 = 1;
pub type pmtrace_t = pmtrace_s;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const GL_EXTCOUNT: C2RustUnnamed_5 = 22;
pub const GL_TEXTURE_MULTISAMPLE: C2RustUnnamed_5 = 21;
pub const GL_DRAW_RANGEELEMENTS_EXT: C2RustUnnamed_5 = 20;
pub const GL_ARB_VERTEX_BUFFER_OBJECT_EXT: C2RustUnnamed_5 = 19;
pub const GL_DEBUG_OUTPUT: C2RustUnnamed_5 = 18;
pub const GL_DEPTH_TEXTURE: C2RustUnnamed_5 = 17;
pub const GL_EXT_GPU_SHADER4: C2RustUnnamed_5 = 16;
pub const GL_ARB_SEAMLESS_CUBEMAP: C2RustUnnamed_5 = 15;
pub const GL_ARB_DEPTH_FLOAT_EXT: C2RustUnnamed_5 = 14;
pub const GL_ARB_TEXTURE_FLOAT_EXT: C2RustUnnamed_5 = 13;
pub const GL_CLAMP_TEXBORDER_EXT: C2RustUnnamed_5 = 12;
pub const GL_ARB_TEXTURE_NPOT_EXT: C2RustUnnamed_5 = 11;
pub const GL_CLAMPTOEDGE_EXT: C2RustUnnamed_5 = 10;
pub const GL_TEXTURE_3D_EXT: C2RustUnnamed_5 = 9;
pub const GL_TEXTURE_ARRAY_EXT: C2RustUnnamed_5 = 8;
pub const GL_TEXTURE_2D_RECT_EXT: C2RustUnnamed_5 = 7;
pub const GL_SHADER_GLSL100_EXT: C2RustUnnamed_5 = 6;
pub const GL_TEXTURE_COMPRESSION_EXT: C2RustUnnamed_5 = 5;
pub const GL_TEXTURE_LOD_BIAS: C2RustUnnamed_5 = 4;
pub const GL_ANISOTROPY_EXT: C2RustUnnamed_5 = 3;
pub const GL_TEXTURE_CUBEMAP_EXT: C2RustUnnamed_5 = 2;
pub const GL_OPENGL_110: C2RustUnnamed_5 = 0;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
static mut g_alias: alias_draw_state_t =
    alias_draw_state_t{time: 0.,
                       frametime: 0.,
                       framecount: 0,
                       interpolate: false_0,
                       ambientlight: 0.,
                       shadelight: 0.,
                       lightvec: [0.; 3],
                       lightvec_local: [0.; 3],
                       lightspot: [0.; 3],
                       lightcolor: [0.; 3],
                       oldpose: 0,
                       newpose: 0,
                       lerpfrac: 0.,};
// global alias state
/*
=================================================================

ALIAS MODEL DISPLAY LIST GENERATION

=================================================================
*/
static mut m_fDoRemap: qboolean = false_0;
static mut m_pAliasHeader: *mut aliashdr_t =
    0 as *const aliashdr_t as *mut aliashdr_t;
static mut g_poseverts: [*mut trivertex_t; 256] =
    [0 as *const trivertex_t as *mut trivertex_t; 256];
static mut g_triangles: [dtriangle_t; 4096] =
    [dtriangle_t{facesfront: 0, vertindex: [0; 3],}; 4096];
static mut g_stverts: [stvert_t; 2048] =
    [stvert_t{onseam: 0, s: 0, t: 0,}; 2048];
static mut g_used: [libc::c_int; 8192] = [0; 8192];
// a pose is a single set of vertexes. a frame may be
// an animating sequence of poses
#[no_mangle]
pub static mut g_posenum: libc::c_int = 0;
// the command list holds counts and s/t values that are valid for
// every frame
static mut g_commands: [libc::c_int; 8192] = [0; 8192];
static mut g_numcommands: libc::c_int = 0;
// all frames will have their vertexes rearranged and expanded
// so they are in the order expected by the command list
static mut g_vertexorder: [libc::c_int; 8192] = [0; 8192];
static mut g_numorder: libc::c_int = 0;
static mut g_stripverts: [libc::c_int; 128] = [0; 128];
static mut g_striptris: [libc::c_int; 128] = [0; 128];
static mut g_stripcount: libc::c_int = 0;
/*
====================
R_StudioInit

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasInit() {
    g_alias.interpolate = true_0;
    m_fDoRemap = false_0;
}
/*
================
StripLength
================
*/
unsafe extern "C" fn StripLength(mut starttri: libc::c_int,
                                 mut startv: libc::c_int) -> libc::c_int {
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut last: *mut dtriangle_t = 0 as *mut dtriangle_t;
    let mut check: *mut dtriangle_t = 0 as *mut dtriangle_t;
    g_used[starttri as usize] = 2 as libc::c_int;
    last =
        &mut *g_triangles.as_mut_ptr().offset(starttri as isize) as
            *mut dtriangle_t;
    g_stripverts[0 as libc::c_int as usize] =
        (*last).vertindex[((startv + 0 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    g_stripverts[1 as libc::c_int as usize] =
        (*last).vertindex[((startv + 1 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    g_stripverts[2 as libc::c_int as usize] =
        (*last).vertindex[((startv + 2 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    g_striptris[0 as libc::c_int as usize] = starttri;
    g_stripcount = 1 as libc::c_int;
    m1 =
        (*last).vertindex[((startv + 2 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    m2 =
        (*last).vertindex[((startv + 1 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    's_44:
        loop 
             // look for a matching triangle
             {
            j = starttri + 1 as libc::c_int;
            check =
                &mut *g_triangles.as_mut_ptr().offset((starttri +
                                                           1 as libc::c_int)
                                                          as isize) as
                    *mut dtriangle_t;
            's_45:
                loop  {
                    if !(j < (*m_pAliasHeader).numtris) { break 's_44 ; }
                    if !((*check).facesfront != (*last).facesfront) {
                        k = 0 as libc::c_int;
                        while k < 3 as libc::c_int {
                            if !((*check).vertindex[k as usize] != m1) {
                                if !((*check).vertindex[((k +
                                                              1 as
                                                                  libc::c_int)
                                                             %
                                                             3 as libc::c_int)
                                                            as usize] != m2) {
                                    // this is the next part of the fan
                                    // if we can't use this triangle, this tristrip is done
                                    if g_used[j as usize] != 0 {
                                        break 's_44 ;
                                    }
                                    // the new edge
                                    if g_stripcount & 1 as libc::c_int != 0 {
                                        m2 =
                                            (*check).vertindex[((k +
                                                                     2 as
                                                                         libc::c_int)
                                                                    %
                                                                    3 as
                                                                        libc::c_int)
                                                                   as usize]
                                    } else {
                                        m1 =
                                            (*check).vertindex[((k +
                                                                     2 as
                                                                         libc::c_int)
                                                                    %
                                                                    3 as
                                                                        libc::c_int)
                                                                   as usize]
                                    }
                                    g_stripverts[(g_stripcount +
                                                      2 as libc::c_int) as
                                                     usize] =
                                        (*check).vertindex[((k +
                                                                 2 as
                                                                     libc::c_int)
                                                                %
                                                                3 as
                                                                    libc::c_int)
                                                               as usize];
                                    g_striptris[g_stripcount as usize] = j;
                                    g_stripcount += 1;
                                    g_used[j as usize] = 2 as libc::c_int;
                                    break 's_45 ;
                                }
                            }
                            k += 1
                        }
                    }
                    j += 1;
                    check = check.offset(1)
                }
        }
    // clear the temp used flags
    j = starttri + 1 as libc::c_int;
    while j < (*m_pAliasHeader).numtris {
        if g_used[j as usize] == 2 as libc::c_int {
            g_used[j as usize] = 0 as libc::c_int
        }
        j += 1
    }
    return g_stripcount;
}
/*
===========
FanLength
===========
*/
unsafe extern "C" fn FanLength(mut starttri: libc::c_int,
                               mut startv: libc::c_int) -> libc::c_int {
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut last: *mut dtriangle_t = 0 as *mut dtriangle_t;
    let mut check: *mut dtriangle_t = 0 as *mut dtriangle_t;
    g_used[starttri as usize] = 2 as libc::c_int;
    last =
        &mut *g_triangles.as_mut_ptr().offset(starttri as isize) as
            *mut dtriangle_t;
    g_stripverts[0 as libc::c_int as usize] =
        (*last).vertindex[((startv + 0 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    g_stripverts[1 as libc::c_int as usize] =
        (*last).vertindex[((startv + 1 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    g_stripverts[2 as libc::c_int as usize] =
        (*last).vertindex[((startv + 2 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    g_striptris[0 as libc::c_int as usize] = starttri;
    g_stripcount = 1 as libc::c_int;
    m1 =
        (*last).vertindex[((startv + 0 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    m2 =
        (*last).vertindex[((startv + 2 as libc::c_int) % 3 as libc::c_int) as
                              usize];
    's_44:
        loop 
             // look for a matching triangle
             {
            j = starttri + 1 as libc::c_int;
            check =
                &mut *g_triangles.as_mut_ptr().offset((starttri +
                                                           1 as libc::c_int)
                                                          as isize) as
                    *mut dtriangle_t;
            's_45:
                loop  {
                    if !(j < (*m_pAliasHeader).numtris) { break 's_44 ; }
                    if !((*check).facesfront != (*last).facesfront) {
                        k = 0 as libc::c_int;
                        while k < 3 as libc::c_int {
                            if !((*check).vertindex[k as usize] != m1) {
                                if !((*check).vertindex[((k +
                                                              1 as
                                                                  libc::c_int)
                                                             %
                                                             3 as libc::c_int)
                                                            as usize] != m2) {
                                    // this is the next part of the fan
			// if we can't use this triangle, this tristrip is done
                                    if g_used[j as usize] != 0 {
                                        break 's_44 ;
                                    }
                                    // the new edge
                                    m2 =
                                        (*check).vertindex[((k +
                                                                 2 as
                                                                     libc::c_int)
                                                                %
                                                                3 as
                                                                    libc::c_int)
                                                               as usize];
                                    g_stripverts[(g_stripcount +
                                                      2 as libc::c_int) as
                                                     usize] = m2;
                                    g_striptris[g_stripcount as usize] = j;
                                    g_stripcount += 1;
                                    g_used[j as usize] = 2 as libc::c_int;
                                    break 's_45 ;
                                }
                            }
                            k += 1
                        }
                    }
                    j += 1;
                    check = check.offset(1)
                }
        }
    // clear the temp used flags
    j = starttri + 1 as libc::c_int;
    while j < (*m_pAliasHeader).numtris {
        if g_used[j as usize] == 2 as libc::c_int {
            g_used[j as usize] = 0 as libc::c_int
        }
        j += 1
    }
    return g_stripcount;
}
/*
================
BuildTris

Generate a list of trifans or strips
for the model, which holds for all frames
================
*/
#[no_mangle]
pub unsafe extern "C" fn BuildTris() {
    let mut len: libc::c_int = 0;
    let mut bestlen: libc::c_int = 0;
    let mut besttype: libc::c_int = 0 as libc::c_int;
    let mut bestverts: [libc::c_int; 1024] = [0; 1024];
    let mut besttris: [libc::c_int; 1024] = [0; 1024];
    let mut type_0: libc::c_int = 0;
    let mut startv: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    //
	// build tristrips
	//
    memset(g_used.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 8192]>() as libc::c_ulong);
    g_numcommands = 0 as libc::c_int;
    g_numorder = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*m_pAliasHeader).numtris {
        // pick an unused triangle and start the trifan
        if !(g_used[i as usize] != 0) {
            bestlen = 0 as libc::c_int;
            type_0 = 0 as libc::c_int;
            while type_0 < 2 as libc::c_int {
                startv = 0 as libc::c_int;
                while startv < 3 as libc::c_int {
                    if type_0 == 1 as libc::c_int {
                        len = StripLength(i, startv)
                    } else { len = FanLength(i, startv) }
                    if len > bestlen {
                        besttype = type_0;
                        bestlen = len;
                        j = 0 as libc::c_int;
                        while j < bestlen + 2 as libc::c_int {
                            bestverts[j as usize] = g_stripverts[j as usize];
                            j += 1
                        }
                        j = 0 as libc::c_int;
                        while j < bestlen {
                            besttris[j as usize] = g_striptris[j as usize];
                            j += 1
                        }
                    }
                    startv += 1
                }
                type_0 += 1
            }
            // mark the tris on the best strip as used
            j = 0 as libc::c_int;
            while j < bestlen {
                g_used[besttris[j as usize] as usize] = 1 as libc::c_int;
                j += 1
            }
            if besttype == 1 as libc::c_int {
                let fresh0 = g_numcommands;
                g_numcommands = g_numcommands + 1;
                g_commands[fresh0 as usize] = bestlen + 2 as libc::c_int
            } else {
                let fresh1 = g_numcommands;
                g_numcommands = g_numcommands + 1;
                g_commands[fresh1 as usize] = -(bestlen + 2 as libc::c_int)
            }
            j = 0 as libc::c_int;
            while j < bestlen + 2 as libc::c_int {
                // emit a vertex into the reorder buffer
                k = bestverts[j as usize];
                let fresh2 = g_numorder;
                g_numorder = g_numorder + 1;
                g_vertexorder[fresh2 as usize] = k;
                // emit s/t coords into the commands stream
                s = g_stverts[k as usize].s as libc::c_float; // on back side
                t = g_stverts[k as usize].t as libc::c_float;
                if g_triangles[besttris[0 as libc::c_int as usize] as
                                   usize].facesfront == 0 &&
                       g_stverts[k as usize].onseam != 0 {
                    s +=
                        ((*m_pAliasHeader).skinwidth / 2 as libc::c_int) as
                            libc::c_float
                }
                s =
                    (s + 0.5f32) /
                        (*m_pAliasHeader).skinwidth as libc::c_float;
                t =
                    (t + 0.5f32) /
                        (*m_pAliasHeader).skinheight as libc::c_float;
                // Carmack use floats and Valve use shorts here...
                let fresh3 = g_numcommands;
                g_numcommands = g_numcommands + 1;
                *(&mut *g_commands.as_mut_ptr().offset(fresh3 as isize) as
                      *mut libc::c_int as *mut libc::c_float) = s;
                let fresh4 = g_numcommands;
                g_numcommands = g_numcommands + 1;
                *(&mut *g_commands.as_mut_ptr().offset(fresh4 as isize) as
                      *mut libc::c_int as *mut libc::c_float) = t;
                j += 1
            }
        }
        i += 1
    }
    let fresh5 = g_numcommands;
    g_numcommands = g_numcommands + 1;
    g_commands[fresh5 as usize] = 0 as libc::c_int;
    // end of list marker
}
/*
================
GL_MakeAliasModelDisplayLists
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_MakeAliasModelDisplayLists(mut m: *mut model_t) {
    let mut verts: *mut trivertex_t = 0 as *mut trivertex_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    BuildTris();
    // save the data out
    (*m_pAliasHeader).poseverts = g_numorder;
    (*m_pAliasHeader).commands =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*m).mempool,
                                                                 (g_numcommands
                                                                      *
                                                                      4 as
                                                                          libc::c_int)
                                                                     as
                                                                     size_t,
                                                                 false_0,
                                                                 b"../ref_gl/gl_alias.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 318 as
                                                                     libc::c_int)
            as *mut libc::c_int;
    memcpy((*m_pAliasHeader).commands as *mut libc::c_void,
           g_commands.as_mut_ptr() as *const libc::c_void,
           (g_numcommands * 4 as libc::c_int) as libc::c_ulong);
    (*m_pAliasHeader).posedata =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*m).mempool,
                                                                 (((*m_pAliasHeader).numposes
                                                                       *
                                                                       (*m_pAliasHeader).poseverts)
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<trivertex_t>()
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 false_0,
                                                                 b"../ref_gl/gl_alias.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 321 as
                                                                     libc::c_int)
            as *mut trivertex_t;
    verts = (*m_pAliasHeader).posedata;
    i = 0 as libc::c_int;
    while i < (*m_pAliasHeader).numposes {
        j = 0 as libc::c_int;
        while j < g_numorder {
            let fresh6 = verts;
            verts = verts.offset(1);
            *fresh6 =
                *g_poseverts[i as
                                 usize].offset(g_vertexorder[j as usize] as
                                                   isize);
            j += 1
        }
        i += 1
    };
}
/*
==============================================================================

ALIAS MODELS

==============================================================================
*/
/*
=================
Mod_LoadAliasFrame
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadAliasFrame(mut pin: *mut libc::c_void,
                                            mut frame: *mut maliasframedesc_t)
 -> *mut libc::c_void {
    let mut pdaliasframe: *mut daliasframe_t = 0 as *mut daliasframe_t;
    let mut pinframe: *mut trivertex_t = 0 as *mut trivertex_t;
    let mut i: libc::c_int = 0;
    pdaliasframe = pin as *mut daliasframe_t;
    Q_strncpy((*frame).name.as_mut_ptr(), (*pdaliasframe).name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
    (*frame).firstpose = g_posenum;
    (*frame).numposes = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*frame).bboxmin.v[i as usize] =
            (*pdaliasframe).bboxmin.v[i as usize];
        (*frame).bboxmax.v[i as usize] =
            (*pdaliasframe).bboxmax.v[i as usize];
        i += 1
    }
    pinframe =
        pdaliasframe.offset(1 as libc::c_int as isize) as *mut trivertex_t;
    g_poseverts[g_posenum as usize] = pinframe;
    pinframe = pinframe.offset((*m_pAliasHeader).numverts as isize);
    g_posenum += 1;
    return pinframe as *mut libc::c_void;
}
/*
=================
Mod_LoadAliasGroup
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadAliasGroup(mut pin: *mut libc::c_void,
                                            mut frame: *mut maliasframedesc_t)
 -> *mut libc::c_void {
    let mut pingroup: *mut daliasgroup_t = 0 as *mut daliasgroup_t;
    let mut i: libc::c_int = 0;
    let mut numframes: libc::c_int = 0;
    let mut pin_intervals: *mut daliasinterval_t = 0 as *mut daliasinterval_t;
    let mut ptemp: *mut libc::c_void = 0 as *mut libc::c_void;
    pingroup = pin as *mut daliasgroup_t;
    numframes = (*pingroup).numframes;
    (*frame).firstpose = g_posenum;
    (*frame).numposes = numframes;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*frame).bboxmin.v[i as usize] = (*pingroup).bboxmin.v[i as usize];
        (*frame).bboxmax.v[i as usize] = (*pingroup).bboxmax.v[i as usize];
        i += 1
    }
    pin_intervals =
        pingroup.offset(1 as libc::c_int as isize) as *mut daliasinterval_t;
    // all the intervals are always equal 0.1 so we don't care about them
    (*frame).interval = (*pin_intervals).interval;
    pin_intervals = pin_intervals.offset(numframes as isize);
    ptemp = pin_intervals as *mut libc::c_void;
    i = 0 as libc::c_int;
    while i < numframes {
        g_poseverts[g_posenum as usize] =
            (ptemp as *mut daliasframe_t).offset(1 as libc::c_int as isize) as
                *mut trivertex_t;
        ptemp =
            ((ptemp as *mut daliasframe_t).offset(1 as libc::c_int as isize)
                 as
                 *mut trivertex_t).offset((*m_pAliasHeader).numverts as isize)
                as *mut libc::c_void;
        g_posenum += 1;
        i += 1
    }
    return ptemp;
}
/*
===============
Mod_CreateSkinData
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_CreateSkinData(mut mod_0: *mut model_t,
                                            mut data: *mut byte,
                                            mut width: libc::c_int,
                                            mut height: libc::c_int)
 -> *mut rgbdata_t {
    static mut skin: rgbdata_t =
        rgbdata_t{width: 0,
                  height: 0,
                  depth: 0,
                  type_0: 0,
                  flags: 0,
                  encode: 0,
                  numMips: 0,
                  palette: 0 as *const byte as *mut byte,
                  buffer: 0 as *const byte as *mut byte,
                  fogParams: [0; 4],
                  size: 0,};
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut loadmodel: *mut model_t =
        gEngfuncs.Mod_GetCurrentLoadingModel.expect("non-null function pointer")();
    skin.width = width as word;
    skin.height = height as word;
    skin.depth = 1 as libc::c_int as word;
    skin.type_0 = PF_INDEXED_24 as libc::c_int as uint;
    skin.flags =
        (IMAGE_HAS_COLOR as libc::c_int | IMAGE_QUAKEPAL as libc::c_int) as
            uint;
    skin.encode = 0 as libc::c_int as word;
    skin.numMips = 1 as libc::c_int as byte;
    skin.buffer = data;
    skin.palette =
        gEngfuncs.CL_GetPaletteColor.expect("non-null function pointer")(0 as
                                                                             libc::c_int)
            as *mut byte;
    skin.size = (width * height) as size_t;
    if gEngfuncs.Image_CustomPalette.expect("non-null function pointer")() as
           u64 == 0 {
        i = 0 as libc::c_int;
        while i < skin.width as libc::c_int * skin.height as libc::c_int {
            if *data.offset(i as isize) as libc::c_int > 224 as libc::c_int &&
                   *data.offset(i as isize) as libc::c_int !=
                       255 as libc::c_int {
                skin.flags =
                    skin.flags |
                        IMAGE_HAS_LUMA as libc::c_int as libc::c_uint;
                break ;
            } else { i += 1 }
        }
    }
    COM_FileBase((*loadmodel).name.as_mut_ptr(), name.as_mut_ptr());
    // for alias models only player can have remap textures
    if !mod_0.is_null() &&
           Q_strnicmp(name.as_mut_ptr(),
                      b"player\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        let mut tx: *mut texture_t = 0 as *mut texture_t;
        let mut i_0: libc::c_int = 0;
        let mut size: libc::c_int = 0;
        i_0 = (*mod_0).numtextures;
        (*mod_0).textures =
            gEngfuncs._Mem_Realloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                       (*mod_0).textures
                                                                           as
                                                                           *mut libc::c_void,
                                                                       ((i_0 +
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut texture_t>()
                                                                                                            as
                                                                                                            libc::c_ulong),
                                                                       true_0,
                                                                       b"../ref_gl/gl_alias.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       455 as
                                                                           libc::c_int)
                as *mut *mut texture_t;
        size = width * height + 768 as libc::c_int;
        tx =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                     (::std::mem::size_of::<texture_t>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(size
                                                                                                          as
                                                                                                          libc::c_ulong),
                                                                     true_0,
                                                                     b"../ref_gl/gl_alias.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     457 as
                                                                         libc::c_int)
                as *mut texture_t;
        let ref mut fresh7 = *(*mod_0).textures.offset(i_0 as isize);
        *fresh7 = tx;
        Q_strncpy((*tx).name.as_mut_ptr(),
                  b"DM_Skin\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 16]>() as
                      libc::c_ulong);
        // done
        (*tx).anim_min = 16 as libc::c_int; // topcolor start
        (*tx).anim_max = 32 as libc::c_int; // topcolor end
        (*tx).anim_total = 112 as libc::c_int;
        (*tx).width = width as libc::c_uint;
        (*tx).height = height as libc::c_uint;
        memcpy(tx.offset(1 as libc::c_int as isize) as *mut libc::c_void,
               data as *const libc::c_void,
               (width * height) as libc::c_ulong);
        memcpy((tx.offset(1 as libc::c_int as isize) as
                    *mut byte).offset((width * height) as isize) as
                   *mut libc::c_void, skin.palette as *const libc::c_void,
               768 as libc::c_int as libc::c_ulong);
        (*mod_0).numtextures += 1
    }
    // bottomcolor start always equal is (topcolor end + 1)
    // bottomcolor end
    // the pixels immediately follow the structures
    // make an copy
    return gEngfuncs.FS_CopyImage.expect("non-null function pointer")(&mut skin);
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadSingleSkin(mut pskintype:
                                                *mut daliasskintype_t,
                                            mut skinnum: libc::c_int,
                                            mut size: libc::c_int)
 -> *mut libc::c_void {
    let mut name: string = [0; 256];
    let mut lumaname: string = [0; 256];
    let mut checkname: string = [0; 256];
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut loadmodel: *mut model_t =
        gEngfuncs.Mod_GetCurrentLoadingModel.expect("non-null function pointer")();
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s:frame%i\x00" as *const u8 as *const libc::c_char,
               (*loadmodel).name.as_mut_ptr(), skinnum);
    Q_snprintf(lumaname.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s:luma%i\x00" as *const u8 as *const libc::c_char,
               (*loadmodel).name.as_mut_ptr(), skinnum);
    Q_snprintf(checkname.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s_%i.tga\x00" as *const u8 as *const libc::c_char,
               (*loadmodel).name.as_mut_ptr(), skinnum);
    if gEngfuncs.FS_FileExists.expect("non-null function pointer")(checkname.as_mut_ptr(),
                                                                   false_0 as
                                                                       libc::c_int)
           == 0 ||
           {
               pic =
                   gEngfuncs.FS_LoadImage.expect("non-null function pointer")(checkname.as_mut_ptr(),
                                                                              0
                                                                                  as
                                                                                  *const byte,
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  size_t);
               pic.is_null()
           } {
        pic =
            Mod_CreateSkinData(loadmodel,
                               pskintype.offset(1 as libc::c_int as isize) as
                                   *mut byte, (*m_pAliasHeader).skinwidth,
                               (*m_pAliasHeader).skinheight)
    }
    (*m_pAliasHeader).gl_texturenum[skinnum as
                                        usize][3 as libc::c_int as usize] =
        GL_LoadTextureFromBuffer(name.as_mut_ptr(), pic, TF_COLORMAP, false_0)
            as libc::c_ushort;
    (*m_pAliasHeader).gl_texturenum[skinnum as
                                        usize][2 as libc::c_int as usize] =
        (*m_pAliasHeader).gl_texturenum[skinnum as
                                            usize][3 as libc::c_int as usize];
    (*m_pAliasHeader).gl_texturenum[skinnum as
                                        usize][1 as libc::c_int as usize] =
        (*m_pAliasHeader).gl_texturenum[skinnum as
                                            usize][2 as libc::c_int as usize];
    (*m_pAliasHeader).gl_texturenum[skinnum as
                                        usize][0 as libc::c_int as usize] =
        (*m_pAliasHeader).gl_texturenum[skinnum as
                                            usize][1 as libc::c_int as usize];
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
    if (*R_GetTexture((*m_pAliasHeader).gl_texturenum[skinnum as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as usize]
                          as GLenum)).flags as libc::c_uint &
           TF_HAS_LUMA as libc::c_int as libc::c_uint != 0 {
        pic =
            Mod_CreateSkinData(0 as *mut model_t,
                               pskintype.offset(1 as libc::c_int as isize) as
                                   *mut byte, (*m_pAliasHeader).skinwidth,
                               (*m_pAliasHeader).skinheight);
        (*m_pAliasHeader).fb_texturenum[skinnum as
                                            usize][3 as libc::c_int as usize]
            =
            GL_LoadTextureFromBuffer(lumaname.as_mut_ptr(), pic, TF_MAKELUMA,
                                     false_0) as libc::c_ushort;
        (*m_pAliasHeader).fb_texturenum[skinnum as
                                            usize][2 as libc::c_int as usize]
            =
            (*m_pAliasHeader).fb_texturenum[skinnum as
                                                usize][3 as libc::c_int as
                                                           usize];
        (*m_pAliasHeader).fb_texturenum[skinnum as
                                            usize][1 as libc::c_int as usize]
            =
            (*m_pAliasHeader).fb_texturenum[skinnum as
                                                usize][2 as libc::c_int as
                                                           usize];
        (*m_pAliasHeader).fb_texturenum[skinnum as
                                            usize][0 as libc::c_int as usize]
            =
            (*m_pAliasHeader).fb_texturenum[skinnum as
                                                usize][1 as libc::c_int as
                                                           usize];
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
    }
    return (pskintype.offset(1 as libc::c_int as isize) as
                *mut byte).offset(size as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadGroupSkin(mut pskintype:
                                               *mut daliasskintype_t,
                                           mut skinnum: libc::c_int,
                                           mut size: libc::c_int)
 -> *mut libc::c_void {
    let mut pinskinintervals: *mut daliasskininterval_t =
        0 as *mut daliasskininterval_t;
    let mut pinskingroup: *mut daliasskingroup_t =
        0 as *mut daliasskingroup_t;
    let mut name: string = [0; 256];
    let mut lumaname: string = [0; 256];
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut loadmodel: *mut model_t =
        gEngfuncs.Mod_GetCurrentLoadingModel.expect("non-null function pointer")();
    // animating skin group.  yuck.
    pskintype = pskintype.offset(1);
    pinskingroup = pskintype as *mut daliasskingroup_t;
    pinskinintervals =
        pinskingroup.offset(1 as libc::c_int as isize) as
            *mut daliasskininterval_t;
    pskintype =
        pinskinintervals.offset((*pinskingroup).numskins as isize) as
            *mut libc::c_void as *mut daliasskintype_t;
    i = 0 as libc::c_int;
    while i < (*pinskingroup).numskins {
        Q_snprintf(name.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s_%i_%i\x00" as *const u8 as *const libc::c_char,
                   (*loadmodel).name.as_mut_ptr(), skinnum, i);
        pic =
            Mod_CreateSkinData(loadmodel, pskintype as *mut byte,
                               (*m_pAliasHeader).skinwidth,
                               (*m_pAliasHeader).skinheight);
        (*m_pAliasHeader).gl_texturenum[skinnum as
                                            usize][(i & 3 as libc::c_int) as
                                                       usize] =
            GL_LoadTextureFromBuffer(name.as_mut_ptr(), pic, TF_COLORMAP,
                                     false_0) as libc::c_ushort;
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
        if (*R_GetTexture((*m_pAliasHeader).gl_texturenum[skinnum as
                                                              usize][(i &
                                                                          3 as
                                                                              libc::c_int)
                                                                         as
                                                                         usize]
                              as GLenum)).flags as libc::c_uint &
               TF_HAS_LUMA as libc::c_int as libc::c_uint != 0 {
            Q_snprintf(lumaname.as_mut_ptr(),
                       ::std::mem::size_of::<string>() as libc::c_ulong,
                       b"%s_%i_%i_luma\x00" as *const u8 as
                           *const libc::c_char,
                       (*loadmodel).name.as_mut_ptr(), skinnum, i);
            pic =
                Mod_CreateSkinData(0 as *mut model_t, pskintype as *mut byte,
                                   (*m_pAliasHeader).skinwidth,
                                   (*m_pAliasHeader).skinheight);
            (*m_pAliasHeader).fb_texturenum[skinnum as
                                                usize][(i & 3 as libc::c_int)
                                                           as usize] =
                GL_LoadTextureFromBuffer(lumaname.as_mut_ptr(), pic,
                                         TF_MAKELUMA, false_0) as
                    libc::c_ushort;
            gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
        }
        pskintype =
            (pskintype as *mut byte).offset(size as isize) as
                *mut daliasskintype_t;
        i += 1
    }
    j = i;
    while i < 4 as libc::c_int {
        (*m_pAliasHeader).gl_texturenum[skinnum as
                                            usize][(i & 3 as libc::c_int) as
                                                       usize] =
            (*m_pAliasHeader).gl_texturenum[skinnum as
                                                usize][(i - j) as usize];
        (*m_pAliasHeader).fb_texturenum[skinnum as
                                            usize][(i & 3 as libc::c_int) as
                                                       usize] =
            (*m_pAliasHeader).fb_texturenum[skinnum as
                                                usize][(i - j) as usize];
        i += 1
    }
    return pskintype as *mut libc::c_void;
}
/*
===============
Mod_LoadAllSkins
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadAllSkins(mut numskins: libc::c_int,
                                          mut pskintype:
                                              *mut daliasskintype_t)
 -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if numskins < 1 as libc::c_int || numskins > 32 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"Mod_LoadAliasModel: Invalid # of skins: %d\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 numskins);
    }
    size = (*m_pAliasHeader).skinwidth * (*m_pAliasHeader).skinheight;
    i = 0 as libc::c_int;
    while i < numskins {
        if (*pskintype).type_0 as libc::c_uint ==
               ALIAS_SKIN_SINGLE as libc::c_int as libc::c_uint {
            pskintype =
                Mod_LoadSingleSkin(pskintype, i, size) as
                    *mut daliasskintype_t
        } else {
            pskintype =
                Mod_LoadGroupSkin(pskintype, i, size) as *mut daliasskintype_t
        }
        i += 1
    }
    return pskintype as *mut libc::c_void;
}
//=========================================================================
/*
=================
Mod_CalcAliasBounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_CalcAliasBounds(mut mod_0: *mut model_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut radius: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    ClearBounds((*mod_0).mins.as_mut_ptr(), (*mod_0).maxs.as_mut_ptr());
    radius = 0.0f32;
    // process verts
    i = 0 as libc::c_int;
    while i < (*m_pAliasHeader).numposes {
        j = 0 as libc::c_int;
        while j < (*m_pAliasHeader).numverts {
            k = 0 as libc::c_int;
            while k < 3 as libc::c_int {
                v[k as usize] =
                    (*g_poseverts[i as
                                      usize].offset(j as isize)).v[k as usize]
                        as libc::c_int as libc::c_float *
                        (*m_pAliasHeader).scale[k as usize] +
                        (*m_pAliasHeader).scale_origin[k as usize];
                k += 1
            }
            AddPointToBounds(v.as_mut_ptr() as *const vec_t,
                             (*mod_0).mins.as_mut_ptr(),
                             (*mod_0).maxs.as_mut_ptr());
            dist =
                v[0 as libc::c_int as usize] * v[0 as libc::c_int as usize] +
                    v[1 as libc::c_int as usize] *
                        v[1 as libc::c_int as usize] +
                    v[2 as libc::c_int as usize] *
                        v[2 as libc::c_int as usize];
            if radius < dist { radius = dist }
            j += 1
        }
        i += 1
    }
    (*mod_0).radius = __tg_sqrt(radius);
}
/*
=================
Mod_LoadAliasModel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadAliasModel(mut mod_0: *mut model_t,
                                            mut buffer: *const libc::c_void,
                                            mut loaded: *mut qboolean) {
    let mut pinmodel: *mut daliashdr_t =
        0 as *mut daliashdr_t; // how to possible is make that?
    let mut pinstverts: *mut stvert_t = 0 as *mut stvert_t;
    let mut pintriangles: *mut dtriangle_t = 0 as *mut dtriangle_t;
    let mut pframetype: *mut daliasframetype_t = 0 as *mut daliasframetype_t;
    let mut pskintype: *mut daliasskintype_t = 0 as *mut daliasskintype_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if !loaded.is_null() { *loaded = false_0 }
    pinmodel = buffer as *mut daliashdr_t;
    i = (*pinmodel).version;
    if i != 6 as libc::c_int {
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^1Error:^7 %s has wrong version number (%i should be %i)\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*mod_0).name.as_mut_ptr(),
                                                                  i,
                                                                  6 as
                                                                      libc::c_int);
        return
    }
    if (*pinmodel).numverts <= 0 as libc::c_int ||
           (*pinmodel).numtris <= 0 as libc::c_int ||
           (*pinmodel).numframes <= 0 as libc::c_int {
        return
    }
    (*mod_0).mempool =
        gEngfuncs._Mem_AllocPool.expect("non-null function pointer")(va(b"^2%s^7\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        (*mod_0).name.as_mut_ptr()),
                                                                     b"../ref_gl/gl_alias.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     644 as
                                                                         libc::c_int);
    // allocate space for a working header, plus all the data except the frames,
	// skin and group info
    size =
        (::std::mem::size_of::<aliashdr_t>() as
             libc::c_ulong).wrapping_add((((*pinmodel).numframes -
                                               1 as libc::c_int) as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<maliasframedesc_t>()
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int; // share effects flags
    m_pAliasHeader =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                 size as
                                                                     size_t,
                                                                 true_0,
                                                                 b"../ref_gl/gl_alias.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 650 as
                                                                     libc::c_int)
            as *mut aliashdr_t;
    (*mod_0).flags = (*pinmodel).flags;
    // endian-adjust and copy the data, starting with the alias model header
    (*m_pAliasHeader).boundingradius = (*pinmodel).boundingradius;
    (*m_pAliasHeader).numskins = (*pinmodel).numskins;
    (*m_pAliasHeader).skinwidth = (*pinmodel).skinwidth;
    (*m_pAliasHeader).skinheight = (*pinmodel).skinheight;
    (*m_pAliasHeader).numverts = (*pinmodel).numverts;
    (*m_pAliasHeader).numtris = (*pinmodel).numtris;
    (*m_pAliasHeader).numframes = (*pinmodel).numframes;
    if (*m_pAliasHeader).numverts > 2048 as libc::c_int {
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^1Error:^7 model %s has too many vertices\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*mod_0).name.as_mut_ptr());
        return
    }
    (*m_pAliasHeader).size = (*pinmodel).size;
    //	mod->synctype = pinmodel->synctype;
    (*mod_0).numframes = (*m_pAliasHeader).numframes;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*m_pAliasHeader).scale[i as usize] = (*pinmodel).scale[i as usize];
        (*m_pAliasHeader).scale_origin[i as usize] =
            (*pinmodel).scale_origin[i as usize];
        (*m_pAliasHeader).eyeposition[i as usize] =
            (*pinmodel).eyeposition[i as usize];
        i += 1
    }
    // load the skins
    pskintype =
        &mut *pinmodel.offset(1 as libc::c_int as isize) as *mut daliashdr_t
            as *mut daliasskintype_t;
    pskintype =
        Mod_LoadAllSkins((*m_pAliasHeader).numskins, pskintype) as
            *mut daliasskintype_t;
    // load base s and t vertices
    pinstverts = pskintype as *mut stvert_t;
    i = 0 as libc::c_int;
    while i < (*m_pAliasHeader).numverts {
        g_stverts[i as usize].onseam =
            (*pinstverts.offset(i as isize)).onseam;
        g_stverts[i as usize].s = (*pinstverts.offset(i as isize)).s;
        g_stverts[i as usize].t = (*pinstverts.offset(i as isize)).t;
        i += 1
    }
    // load triangle lists
    pintriangles =
        &mut *pinstverts.offset((*m_pAliasHeader).numverts as isize) as
            *mut stvert_t as *mut dtriangle_t;
    i = 0 as libc::c_int;
    while i < (*m_pAliasHeader).numtris {
        g_triangles[i as usize].facesfront =
            (*pintriangles.offset(i as isize)).facesfront;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            g_triangles[i as usize].vertindex[j as usize] =
                (*pintriangles.offset(i as isize)).vertindex[j as usize];
            j += 1
        }
        i += 1
    }
    // load the frames
    pframetype =
        &mut *pintriangles.offset((*m_pAliasHeader).numtris as isize) as
            *mut dtriangle_t as *mut daliasframetype_t;
    g_posenum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*m_pAliasHeader).numframes {
        let mut frametype: aliasframetype_t = (*pframetype).type_0;
        if frametype as libc::c_uint ==
               ALIAS_SINGLE as libc::c_int as libc::c_uint {
            pframetype =
                Mod_LoadAliasFrame(pframetype.offset(1 as libc::c_int as
                                                         isize) as
                                       *mut libc::c_void,
                                   &mut *(*m_pAliasHeader).frames.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize))
                    as *mut daliasframetype_t
        } else {
            pframetype =
                Mod_LoadAliasGroup(pframetype.offset(1 as libc::c_int as
                                                         isize) as
                                       *mut libc::c_void,
                                   &mut *(*m_pAliasHeader).frames.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize))
                    as *mut daliasframetype_t
        }
        i += 1
    }
    (*m_pAliasHeader).numposes = g_posenum;
    Mod_CalcAliasBounds(mod_0);
    (*mod_0).type_0 = mod_alias;
    // build the draw lists
    GL_MakeAliasModelDisplayLists(mod_0);
    // move the complete, relocatable alias model to the cache
    let ref mut fresh8 =
        (*gEngfuncs.Mod_GetCurrentLoadingModel.expect("non-null function pointer")()).cache.data;
    *fresh8 = m_pAliasHeader as *mut libc::c_void;
    if !loaded.is_null() { *loaded = true_0 };
    // done
}
#[no_mangle]
pub unsafe extern "C" fn Mod_AliasUnloadTextures(mut data:
                                                     *mut libc::c_void) {
    let mut palias: *mut aliashdr_t = 0 as *mut aliashdr_t; // already freed
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    palias = data as *mut aliashdr_t;
    if palias.is_null() { return }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*palias).gl_texturenum[i as usize][0 as libc::c_int as usize] == 0
           {
            break ;
        }
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            GL_FreeTexture((*palias).gl_texturenum[i as usize][j as usize] as
                               GLenum);
            GL_FreeTexture((*palias).fb_texturenum[i as usize][j as usize] as
                               GLenum);
            j += 1
        }
        i += 1
    };
}
/*
=============================================================

  ALIAS MODELS

=============================================================
*/
/*
===============
R_AliasDynamicLight

similar to R_StudioDynamicLight
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasDynamicLight(mut ent: *mut cl_entity_t,
                                             mut plight: *mut alight_t) {
    let mut mv: *mut movevars_t =
        gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")();
    let mut lightDir: vec3_t = [0.; 3];
    let mut vecSrc: vec3_t = [0.; 3];
    let mut vecEnd: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut dist: vec3_t = [0.; 3];
    let mut finalLight: vec3_t = [0.; 3];
    let mut add: libc::c_float = 0.;
    let mut radius: libc::c_float = 0.;
    let mut total: libc::c_float = 0.;
    let mut light: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
    let mut lnum: uint = 0;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if plight.is_null() || ent.is_null() { return }
    if RI.drawWorld as u64 == 0 || (*r_fullbright).value != 0. ||
           (*ent).curstate.effects as libc::c_uint &
               (1 as libc::c_uint) << 27 as libc::c_int != 0 {
        (*plight).shadelight = 0 as libc::c_int;
        (*plight).ambientlight = 192 as libc::c_int;
        *(*plight).plightvec.offset(0 as libc::c_int as isize) = 0.0f32;
        *(*plight).plightvec.offset(1 as libc::c_int as isize) = 0.0f32;
        *(*plight).plightvec.offset(2 as libc::c_int as isize) = -1.0f32;
        (*plight).color[0 as libc::c_int as usize] = 1.0f32;
        (*plight).color[1 as libc::c_int as usize] = 1.0f32;
        (*plight).color[2 as libc::c_int as usize] = 1.0f32;
        return
    }
    // determine plane to get lightvalues from: ceil or floor
    if (*ent).curstate.effects & 16 as libc::c_int != 0 {
        lightDir[0 as libc::c_int as usize] = 0.0f32;
        lightDir[1 as libc::c_int as usize] = 0.0f32;
        lightDir[2 as libc::c_int as usize] = 1.0f32
    } else {
        lightDir[0 as libc::c_int as usize] = 0.0f32;
        lightDir[1 as libc::c_int as usize] = 0.0f32;
        lightDir[2 as libc::c_int as usize] = -1.0f32
    }
    origin[0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
    vecSrc[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
    vecSrc[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
    vecSrc[2 as libc::c_int as usize] =
        origin[2 as libc::c_int as usize] -
            lightDir[2 as libc::c_int as usize] * 8.0f32;
    light.a = 0 as libc::c_int as libc::c_uint;
    light.b = light.a;
    light.g = light.b;
    light.r = light.g;
    if (*mv).skycolor_r + (*mv).skycolor_g + (*mv).skycolor_b !=
           0 as libc::c_int as libc::c_float {
        let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
        let mut trace: pmtrace_t =
            pmtrace_t{allsolid: false_0,
                      startsolid: false_0,
                      inopen: false_0,
                      inwater: false_0,
                      fraction: 0.,
                      endpos: [0.; 3],
                      plane: pmplane_t{normal: [0.; 3], dist: 0.,},
                      ent: 0,
                      deltavelocity: [0.; 3],
                      hitgroup: 0,};
        if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            vecEnd[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    (*mv).skyvec_x * 65536.0f32;
            vecEnd[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    (*mv).skyvec_y * 65536.0f32;
            vecEnd[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] -
                    (*mv).skyvec_z * 65536.0f32
        } else {
            vecEnd[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    (*mv).skyvec_x * 8192.0f32;
            vecEnd[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    (*mv).skyvec_y * 8192.0f32;
            vecEnd[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] - (*mv).skyvec_z * 8192.0f32
        }
        trace =
            gEngfuncs.CL_TraceLine.expect("non-null function pointer")(vecSrc.as_mut_ptr(),
                                                                       vecEnd.as_mut_ptr(),
                                                                       0x1 as
                                                                           libc::c_int);
        if trace.ent > 0 as libc::c_int {
            psurf =
                gEngfuncs.EV_TraceSurface.expect("non-null function pointer")(trace.ent,
                                                                              vecSrc.as_mut_ptr(),
                                                                              vecEnd.as_mut_ptr())
        } else {
            psurf =
                gEngfuncs.EV_TraceSurface.expect("non-null function pointer")(0
                                                                                  as
                                                                                  libc::c_int,
                                                                              vecSrc.as_mut_ptr(),
                                                                              vecEnd.as_mut_ptr())
        }
        if !psurf.is_null() &&
               (*psurf).flags as libc::c_uint &
                   (1 as libc::c_uint) << 2 as libc::c_int != 0 {
            lightDir[0 as libc::c_int as usize] = (*mv).skyvec_x;
            lightDir[1 as libc::c_int as usize] = (*mv).skyvec_y;
            lightDir[2 as libc::c_int as usize] = (*mv).skyvec_z;
            light.r =
                gEngfuncs.LightToTexGamma.expect("non-null function pointer")(if (*mv).skycolor_r
                                                                                     >=
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                 {
                                                                                  if (*mv).skycolor_r
                                                                                         <
                                                                                         255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                     {
                                                                                      (*mv).skycolor_r
                                                                                  } else {
                                                                                      255
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_float
                                                                                  }
                                                                              } else {
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_float
                                                                              }
                                                                                  as
                                                                                  byte)
                    as libc::c_uint;
            light.g =
                gEngfuncs.LightToTexGamma.expect("non-null function pointer")(if (*mv).skycolor_g
                                                                                     >=
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                 {
                                                                                  if (*mv).skycolor_g
                                                                                         <
                                                                                         255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                     {
                                                                                      (*mv).skycolor_g
                                                                                  } else {
                                                                                      255
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_float
                                                                                  }
                                                                              } else {
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_float
                                                                              }
                                                                                  as
                                                                                  byte)
                    as libc::c_uint;
            light.b =
                gEngfuncs.LightToTexGamma.expect("non-null function pointer")(if (*mv).skycolor_b
                                                                                     >=
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                 {
                                                                                  if (*mv).skycolor_b
                                                                                         <
                                                                                         255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                     {
                                                                                      (*mv).skycolor_b
                                                                                  } else {
                                                                                      255
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_float
                                                                                  }
                                                                              } else {
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_float
                                                                              }
                                                                                  as
                                                                                  byte)
                    as libc::c_uint
        }
    }
    if light.r.wrapping_add(light.g).wrapping_add(light.b) ==
           0 as libc::c_int as libc::c_uint {
        let mut gcolor: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
        let mut grad: [libc::c_float; 4] = [0.; 4];
        vecEnd[0 as libc::c_int as usize] =
            lightDir[0 as libc::c_int as usize] * 2048.0f32;
        vecEnd[1 as libc::c_int as usize] =
            lightDir[1 as libc::c_int as usize] * 2048.0f32;
        vecEnd[2 as libc::c_int as usize] =
            lightDir[2 as libc::c_int as usize] * 2048.0f32;
        vecEnd[0 as libc::c_int as usize] =
            vecEnd[0 as libc::c_int as usize] +
                vecSrc[0 as libc::c_int as usize];
        vecEnd[1 as libc::c_int as usize] =
            vecEnd[1 as libc::c_int as usize] +
                vecSrc[1 as libc::c_int as usize];
        vecEnd[2 as libc::c_int as usize] =
            vecEnd[2 as libc::c_int as usize] +
                vecSrc[2 as libc::c_int as usize];
        light =
            R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                       vecEnd.as_mut_ptr() as *const vec_t,
                       g_alias.lightspot.as_mut_ptr(),
                       g_alias.lightvec.as_mut_ptr());
        if g_alias.lightvec[0 as libc::c_int as usize] == 0.0f32 &&
               g_alias.lightvec[1 as libc::c_int as usize] == 0.0f32 &&
               g_alias.lightvec[2 as libc::c_int as usize] == 0.0f32 {
            vecSrc[0 as libc::c_int as usize] -= 16.0f32;
            vecSrc[1 as libc::c_int as usize] -= 16.0f32;
            vecEnd[0 as libc::c_int as usize] -= 16.0f32;
            vecEnd[1 as libc::c_int as usize] -= 16.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[0 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            vecSrc[0 as libc::c_int as usize] += 32.0f32;
            vecEnd[0 as libc::c_int as usize] += 32.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[1 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            vecSrc[1 as libc::c_int as usize] += 32.0f32;
            vecEnd[1 as libc::c_int as usize] += 32.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[2 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            vecSrc[0 as libc::c_int as usize] -= 32.0f32;
            vecEnd[0 as libc::c_int as usize] -= 32.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[3 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            lightDir[0 as libc::c_int as usize] =
                grad[0 as libc::c_int as usize] -
                    grad[1 as libc::c_int as usize] -
                    grad[2 as libc::c_int as usize] +
                    grad[3 as libc::c_int as usize];
            lightDir[1 as libc::c_int as usize] =
                grad[1 as libc::c_int as usize] +
                    grad[0 as libc::c_int as usize] -
                    grad[2 as libc::c_int as usize] -
                    grad[3 as libc::c_int as usize];
            let mut ilength: libc::c_float =
                __tg_sqrt(lightDir[0 as libc::c_int as usize] *
                              lightDir[0 as libc::c_int as usize] +
                              lightDir[1 as libc::c_int as usize] *
                                  lightDir[1 as libc::c_int as usize] +
                              lightDir[2 as libc::c_int as usize] *
                                  lightDir[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            lightDir[0 as libc::c_int as usize] *= ilength;
            lightDir[1 as libc::c_int as usize] *= ilength;
            lightDir[2 as libc::c_int as usize] *= ilength
        } else {
            lightDir[0 as libc::c_int as usize] =
                g_alias.lightvec[0 as libc::c_int as usize];
            lightDir[1 as libc::c_int as usize] =
                g_alias.lightvec[1 as libc::c_int as usize];
            lightDir[2 as libc::c_int as usize] =
                g_alias.lightvec[2 as libc::c_int as usize]
        }
    }
    finalLight[0 as libc::c_int as usize] = light.r as vec_t;
    finalLight[1 as libc::c_int as usize] = light.g as vec_t;
    finalLight[2 as libc::c_int as usize] = light.b as vec_t;
    (*ent).cvFloorColor = light;
    total =
        if (if light.r > light.g { light.r } else { light.g }) > light.b {
            if light.r > light.g { light.r } else { light.g }
        } else { light.b } as libc::c_float;
    if total == 0.0f32 { total = 1.0f32 }
    // scale lightdir by light intentsity
    lightDir[0 as libc::c_int as usize] =
        lightDir[0 as libc::c_int as usize] * total;
    lightDir[1 as libc::c_int as usize] =
        lightDir[1 as libc::c_int as usize] * total;
    lightDir[2 as libc::c_int as usize] =
        lightDir[2 as libc::c_int as usize] * total;
    lnum = 0 as libc::c_int as uint;
    while lnum < 32 as libc::c_int as libc::c_uint {
        dl =
            gEngfuncs.GetDynamicLight.expect("non-null function pointer")(lnum
                                                                              as
                                                                              libc::c_int);
        if !(((*dl).die as libc::c_double) < g_alias.time ||
                 (*r_dynamic).value == 0.) {
            dist[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    (*dl).origin[0 as libc::c_int as usize];
            dist[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    (*dl).origin[1 as libc::c_int as usize];
            dist[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] -
                    (*dl).origin[2 as libc::c_int as usize];
            radius =
                __tg_sqrt(dist[0 as libc::c_int as usize] *
                              dist[0 as libc::c_int as usize] +
                              dist[1 as libc::c_int as usize] *
                                  dist[1 as libc::c_int as usize] +
                              dist[2 as libc::c_int as usize] *
                                  dist[2 as libc::c_int as usize]);
            add = (*dl).radius - radius;
            if add > 0.0f32 {
                total += add;
                if radius > 1.0f32 {
                    dist[0 as libc::c_int as usize] =
                        dist[0 as libc::c_int as usize] * (add / radius);
                    dist[1 as libc::c_int as usize] =
                        dist[1 as libc::c_int as usize] * (add / radius);
                    dist[2 as libc::c_int as usize] =
                        dist[2 as libc::c_int as usize] * (add / radius)
                } else {
                    dist[0 as libc::c_int as usize] =
                        dist[0 as libc::c_int as usize] * add;
                    dist[1 as libc::c_int as usize] =
                        dist[1 as libc::c_int as usize] * add;
                    dist[2 as libc::c_int as usize] =
                        dist[2 as libc::c_int as usize] * add
                }
                lightDir[0 as libc::c_int as usize] =
                    lightDir[0 as libc::c_int as usize] +
                        dist[0 as libc::c_int as usize];
                lightDir[1 as libc::c_int as usize] =
                    lightDir[1 as libc::c_int as usize] +
                        dist[1 as libc::c_int as usize];
                lightDir[2 as libc::c_int as usize] =
                    lightDir[2 as libc::c_int as usize] +
                        dist[2 as libc::c_int as usize];
                finalLight[0 as libc::c_int as usize] +=
                    gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.r)
                        as libc::c_int as libc::c_float * (add / 256.0f32) *
                        2.0f32;
                finalLight[1 as libc::c_int as usize] +=
                    gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.g)
                        as libc::c_int as libc::c_float * (add / 256.0f32) *
                        2.0f32;
                finalLight[2 as libc::c_int as usize] +=
                    gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.b)
                        as libc::c_int as libc::c_float * (add / 256.0f32) *
                        2.0f32
            }
        }
        lnum = lnum.wrapping_add(1)
    }
    lightDir[0 as libc::c_int as usize] =
        lightDir[0 as libc::c_int as usize] * 0.9f32;
    lightDir[1 as libc::c_int as usize] =
        lightDir[1 as libc::c_int as usize] * 0.9f32;
    lightDir[2 as libc::c_int as usize] =
        lightDir[2 as libc::c_int as usize] * 0.9f32;
    (*plight).shadelight =
        __tg_sqrt(lightDir[0 as libc::c_int as usize] *
                      lightDir[0 as libc::c_int as usize] +
                      lightDir[1 as libc::c_int as usize] *
                          lightDir[1 as libc::c_int as usize] +
                      lightDir[2 as libc::c_int as usize] *
                          lightDir[2 as libc::c_int as usize]) as libc::c_int;
    (*plight).ambientlight =
        (total - (*plight).shadelight as libc::c_float) as libc::c_int;
    total =
        if (if finalLight[0 as libc::c_int as usize] >
                   finalLight[1 as libc::c_int as usize] {
                finalLight[0 as libc::c_int as usize]
            } else { finalLight[1 as libc::c_int as usize] }) >
               finalLight[2 as libc::c_int as usize] {
            if finalLight[0 as libc::c_int as usize] >
                   finalLight[1 as libc::c_int as usize] {
                finalLight[0 as libc::c_int as usize]
            } else { finalLight[1 as libc::c_int as usize] }
        } else { finalLight[2 as libc::c_int as usize] };
    if total > 0.0f32 {
        (*plight).color[0 as libc::c_int as usize] =
            finalLight[0 as libc::c_int as usize] * (1.0f32 / total);
        (*plight).color[1 as libc::c_int as usize] =
            finalLight[1 as libc::c_int as usize] * (1.0f32 / total);
        (*plight).color[2 as libc::c_int as usize] =
            finalLight[2 as libc::c_int as usize] * (1.0f32 / total)
    } else {
        (*plight).color[0 as libc::c_int as usize] = 1.0f32;
        (*plight).color[1 as libc::c_int as usize] = 1.0f32;
        (*plight).color[2 as libc::c_int as usize] = 1.0f32
    }
    if (*plight).ambientlight > 128 as libc::c_int {
        (*plight).ambientlight = 128 as libc::c_int
    }
    if (*plight).ambientlight + (*plight).shadelight > 255 as libc::c_int {
        (*plight).shadelight = 255 as libc::c_int - (*plight).ambientlight
    }
    let mut ilength_0: libc::c_float =
        __tg_sqrt(lightDir[0 as libc::c_int as usize] *
                      lightDir[0 as libc::c_int as usize] +
                      lightDir[1 as libc::c_int as usize] *
                          lightDir[1 as libc::c_int as usize] +
                      lightDir[2 as libc::c_int as usize] *
                          lightDir[2 as libc::c_int as usize]);
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    *(*plight).plightvec.offset(0 as libc::c_int as isize) =
        lightDir[0 as libc::c_int as usize] * ilength_0;
    *(*plight).plightvec.offset(1 as libc::c_int as isize) =
        lightDir[1 as libc::c_int as usize] * ilength_0;
    *(*plight).plightvec.offset(2 as libc::c_int as isize) =
        lightDir[2 as libc::c_int as usize] * ilength_0;
}
/*
===============
R_AliasSetupLighting

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetupLighting(mut plight: *mut alight_t) {
    if m_pAliasHeader.is_null() || plight.is_null() { return }
    g_alias.ambientlight = (*plight).ambientlight as libc::c_float;
    g_alias.shadelight = (*plight).shadelight as libc::c_float;
    g_alias.lightvec[0 as libc::c_int as usize] =
        *(*plight).plightvec.offset(0 as libc::c_int as isize);
    g_alias.lightvec[1 as libc::c_int as usize] =
        *(*plight).plightvec.offset(1 as libc::c_int as isize);
    g_alias.lightvec[2 as libc::c_int as usize] =
        *(*plight).plightvec.offset(2 as libc::c_int as isize);
    g_alias.lightcolor[0 as libc::c_int as usize] =
        (*plight).color[0 as libc::c_int as usize];
    g_alias.lightcolor[1 as libc::c_int as usize] =
        (*plight).color[1 as libc::c_int as usize];
    g_alias.lightcolor[2 as libc::c_int as usize] =
        (*plight).color[2 as libc::c_int as usize];
    // transform back to local space
    Matrix4x4_VectorIRotate(RI.objectMatrix.as_mut_ptr() as *const [vec_t; 4],
                            g_alias.lightvec.as_mut_ptr() as
                                *const libc::c_float,
                            g_alias.lightvec_local.as_mut_ptr());
    let mut ilength: libc::c_float =
        __tg_sqrt(g_alias.lightvec_local[0 as libc::c_int as usize] *
                      g_alias.lightvec_local[0 as libc::c_int as usize] +
                      g_alias.lightvec_local[1 as libc::c_int as usize] *
                          g_alias.lightvec_local[1 as libc::c_int as usize] +
                      g_alias.lightvec_local[2 as libc::c_int as usize] *
                          g_alias.lightvec_local[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    g_alias.lightvec_local[0 as libc::c_int as usize] *= ilength;
    g_alias.lightvec_local[1 as libc::c_int as usize] *= ilength;
    g_alias.lightvec_local[2 as libc::c_int as usize] *= ilength;
}
/*
===============
R_AliasLighting

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasLighting(mut lv: *mut libc::c_float,
                                         mut normal: *const vec_t) {
    let mut illum: libc::c_float =
        g_alias.ambientlight; // -1 colinear, 1 opposite
    let mut r: libc::c_float = 0.;
    let mut lightcos: libc::c_float = 0.;
    lightcos =
        *normal.offset(0 as libc::c_int as isize) *
            g_alias.lightvec_local[0 as libc::c_int as usize] +
            *normal.offset(1 as libc::c_int as isize) *
                g_alias.lightvec_local[1 as libc::c_int as usize] +
            *normal.offset(2 as libc::c_int as isize) *
                g_alias.lightvec_local[2 as libc::c_int as usize];
    if lightcos > 1.0f32 { lightcos = 1.0f32 }
    illum += g_alias.shadelight;
    r = 1.495f32;
    // do modified hemispherical lighting
    if r <= 1.0f32 {
        r += 1.0f32;
        lightcos = (r - 1.0f32 - lightcos) / r;
        if lightcos > 0.0f32 { illum += g_alias.shadelight * lightcos }
    } else {
        lightcos = (lightcos + (r - 1.0f32)) / r;
        if lightcos > 0.0f32 { illum -= g_alias.shadelight * lightcos }
    }
    illum = if illum > 0.0f32 { illum } else { 0.0f32 };
    illum = if illum < 255.0f32 { illum } else { 255.0f32 };
    *lv = illum * (1.0f32 / 255.0f32);
}
/*
===============
R_AliasSetRemapColors

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetRemapColors(mut newTop: libc::c_int,
                                               mut newBottom: libc::c_int) {
    gEngfuncs.CL_AllocRemapInfo.expect("non-null function pointer")(RI.currententity,
                                                                    RI.currentmodel,
                                                                    newTop,
                                                                    newBottom);
    if !gEngfuncs.CL_GetRemapInfoForEntity.expect("non-null function pointer")(RI.currententity).is_null()
       {
        gEngfuncs.CL_UpdateRemapInfo.expect("non-null function pointer")(RI.currententity,
                                                                         newTop,
                                                                         newBottom);
        m_fDoRemap = true_0
    };
}
/*
=============
GL_DrawAliasFrame
=============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_DrawAliasFrame(mut paliashdr: *mut aliashdr_t) {
    let mut lv_tmp: libc::c_float = 0.;
    let mut verts0: *mut trivertex_t = 0 as *mut trivertex_t;
    let mut verts1: *mut trivertex_t = 0 as *mut trivertex_t;
    let mut vert: vec3_t = [0.; 3];
    let mut norm: vec3_t = [0.; 3];
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut count: libc::c_int = 0;
    verts1 = (*paliashdr).posedata;
    verts0 = verts1;
    verts0 =
        verts0.offset((g_alias.oldpose * (*paliashdr).poseverts) as isize);
    verts1 =
        verts1.offset((g_alias.newpose * (*paliashdr).poseverts) as isize);
    order = (*paliashdr).commands;
    loop  {
        // get the vertex count and primitive type
        let fresh9 = order; // done
        order = order.offset(1);
        count = *fresh9;
        if count == 0 { break ; }
        if count < 0 as libc::c_int {
            pglBegin.expect("non-null function pointer")(0x6 as libc::c_int as
                                                             GLenum);
            count = -count
        } else {
            pglBegin.expect("non-null function pointer")(0x5 as libc::c_int as
                                                             GLenum);
        }
        loop  {
            // texture coordinates come from the draw list
            if GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as libc::c_uint
                   != 0 && glState.activeTMU > 0 as libc::c_int {
                GL_MultiTexCoord2f(XASH_TEXTURE0 as libc::c_int as GLenum,
                                   *(order as
                                         *mut libc::c_float).offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                   *(order as
                                         *mut libc::c_float).offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
                GL_MultiTexCoord2f(XASH_TEXTURE1 as libc::c_int as GLenum,
                                   *(order as
                                         *mut libc::c_float).offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                   *(order as
                                         *mut libc::c_float).offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
            } else {
                pglTexCoord2f.expect("non-null function pointer")(*(order as
                                                                        *mut libc::c_float).offset(0
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       isize),
                                                                  *(order as
                                                                        *mut libc::c_float).offset(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       isize));
            }
            order = order.offset(2 as libc::c_int as isize);
            norm[0 as libc::c_int as usize] =
                m_bytenormals[(*verts0).lightnormalindex as
                                  usize][0 as libc::c_int as usize] +
                    g_alias.lerpfrac *
                        (m_bytenormals[(*verts1).lightnormalindex as
                                           usize][0 as libc::c_int as usize] -
                             m_bytenormals[(*verts0).lightnormalindex as
                                               usize][0 as libc::c_int as
                                                          usize]);
            norm[1 as libc::c_int as usize] =
                m_bytenormals[(*verts0).lightnormalindex as
                                  usize][1 as libc::c_int as usize] +
                    g_alias.lerpfrac *
                        (m_bytenormals[(*verts1).lightnormalindex as
                                           usize][1 as libc::c_int as usize] -
                             m_bytenormals[(*verts0).lightnormalindex as
                                               usize][1 as libc::c_int as
                                                          usize]);
            norm[2 as libc::c_int as usize] =
                m_bytenormals[(*verts0).lightnormalindex as
                                  usize][2 as libc::c_int as usize] +
                    g_alias.lerpfrac *
                        (m_bytenormals[(*verts1).lightnormalindex as
                                           usize][2 as libc::c_int as usize] -
                             m_bytenormals[(*verts0).lightnormalindex as
                                               usize][2 as libc::c_int as
                                                          usize]);
            let mut ilength: libc::c_float =
                __tg_sqrt(norm[0 as libc::c_int as usize] *
                              norm[0 as libc::c_int as usize] +
                              norm[1 as libc::c_int as usize] *
                                  norm[1 as libc::c_int as usize] +
                              norm[2 as libc::c_int as usize] *
                                  norm[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            norm[0 as libc::c_int as usize] *= ilength;
            norm[1 as libc::c_int as usize] *= ilength;
            norm[2 as libc::c_int as usize] *= ilength;
            R_AliasLighting(&mut lv_tmp, norm.as_mut_ptr() as *const vec_t);
            pglColor4f.expect("non-null function pointer")(g_alias.lightcolor[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                               * lv_tmp,
                                                           g_alias.lightcolor[1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                               * lv_tmp,
                                                           g_alias.lightcolor[2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                               * lv_tmp,
                                                           tr.blend);
            vert[0 as libc::c_int as usize] =
                (*verts0).v[0 as libc::c_int as usize] as libc::c_int as
                    libc::c_float +
                    g_alias.lerpfrac *
                        ((*verts1).v[0 as libc::c_int as usize] as libc::c_int
                             -
                             (*verts0).v[0 as libc::c_int as usize] as
                                 libc::c_int) as libc::c_float;
            vert[1 as libc::c_int as usize] =
                (*verts0).v[1 as libc::c_int as usize] as libc::c_int as
                    libc::c_float +
                    g_alias.lerpfrac *
                        ((*verts1).v[1 as libc::c_int as usize] as libc::c_int
                             -
                             (*verts0).v[1 as libc::c_int as usize] as
                                 libc::c_int) as libc::c_float;
            vert[2 as libc::c_int as usize] =
                (*verts0).v[2 as libc::c_int as usize] as libc::c_int as
                    libc::c_float +
                    g_alias.lerpfrac *
                        ((*verts1).v[2 as libc::c_int as usize] as libc::c_int
                             -
                             (*verts0).v[2 as libc::c_int as usize] as
                                 libc::c_int) as libc::c_float;
            pglVertex3fv.expect("non-null function pointer")(vert.as_mut_ptr());
            verts0 = verts0.offset(1);
            verts1 = verts1.offset(1);
            count -= 1;
            if !(count != 0) { break ; }
        }
        pglEnd.expect("non-null function pointer")();
    };
}
/*
=============
GL_DrawAliasShadow
=============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_DrawAliasShadow(mut paliashdr: *mut aliashdr_t) {
    let mut verts0: *mut trivertex_t = 0 as *mut trivertex_t;
    let mut verts1: *mut trivertex_t = 0 as *mut trivertex_t;
    let mut vec_x: libc::c_float = 0.;
    let mut vec_y: libc::c_float = 0.;
    let mut av: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut height: libc::c_float = 0.;
    let mut count: libc::c_int = 0;
    if (*RI.currententity).curstate.effects as libc::c_uint &
           (1 as libc::c_uint) << 28 as libc::c_int != 0 {
        return
    }
    if glState.stencilEnabled as u64 != 0 {
        pglEnable.expect("non-null function pointer")(0xb90 as libc::c_int as
                                                          GLenum);
    }
    height = g_alias.lightspot[2 as libc::c_int as usize] + 1.0f32;
    vec_x = -g_alias.lightvec[0 as libc::c_int as usize] * 8.0f32;
    vec_y = -g_alias.lightvec[1 as libc::c_int as usize] * 8.0f32;
    r_stats.c_alias_polys =
        (r_stats.c_alias_polys as
             libc::c_uint).wrapping_add((*paliashdr).numtris as libc::c_uint)
            as uint as uint;
    verts1 = (*paliashdr).posedata;
    verts0 = verts1;
    verts0 =
        verts0.offset((g_alias.oldpose * (*paliashdr).poseverts) as isize);
    verts1 =
        verts1.offset((g_alias.newpose * (*paliashdr).poseverts) as isize);
    order = (*paliashdr).commands;
    loop  {
        // get the vertex count and primitive type
        let fresh10 = order; // done
        order = order.offset(1);
        count = *fresh10;
        if count == 0 { break ; }
        if count < 0 as libc::c_int {
            pglBegin.expect("non-null function pointer")(0x6 as libc::c_int as
                                                             GLenum);
            count = -count
        } else {
            pglBegin.expect("non-null function pointer")(0x5 as libc::c_int as
                                                             GLenum);
        }
        loop  {
            // texture coordinates come from the draw list
			// (skipped for shadows) pglTexCoord2fv ((float *)order);
            order = order.offset(2 as libc::c_int as isize);
            // normals and vertexes come from the frame list
            av[0 as libc::c_int as usize] =
                (*verts0).v[0 as libc::c_int as usize] as libc::c_int as
                    libc::c_float +
                    g_alias.lerpfrac *
                        ((*verts1).v[0 as libc::c_int as usize] as libc::c_int
                             -
                             (*verts0).v[0 as libc::c_int as usize] as
                                 libc::c_int) as libc::c_float;
            av[1 as libc::c_int as usize] =
                (*verts0).v[1 as libc::c_int as usize] as libc::c_int as
                    libc::c_float +
                    g_alias.lerpfrac *
                        ((*verts1).v[1 as libc::c_int as usize] as libc::c_int
                             -
                             (*verts0).v[1 as libc::c_int as usize] as
                                 libc::c_int) as libc::c_float;
            av[2 as libc::c_int as usize] =
                (*verts0).v[2 as libc::c_int as usize] as libc::c_int as
                    libc::c_float +
                    g_alias.lerpfrac *
                        ((*verts1).v[2 as libc::c_int as usize] as libc::c_int
                             -
                             (*verts0).v[2 as libc::c_int as usize] as
                                 libc::c_int) as libc::c_float;
            point[0 as libc::c_int as usize] =
                av[0 as libc::c_int as usize] *
                    (*paliashdr).scale[0 as libc::c_int as usize] +
                    (*paliashdr).scale_origin[0 as libc::c_int as usize];
            point[1 as libc::c_int as usize] =
                av[1 as libc::c_int as usize] *
                    (*paliashdr).scale[1 as libc::c_int as usize] +
                    (*paliashdr).scale_origin[1 as libc::c_int as usize];
            point[2 as libc::c_int as usize] =
                av[2 as libc::c_int as usize] *
                    (*paliashdr).scale[2 as libc::c_int as usize] +
                    (*paliashdr).scale_origin[2 as libc::c_int as usize];
            Matrix3x4_VectorTransform(RI.objectMatrix.as_mut_ptr() as
                                          *const [vec_t; 4],
                                      point.as_mut_ptr() as
                                          *const libc::c_float,
                                      av.as_mut_ptr());
            point[0 as libc::c_int as usize] =
                av[0 as libc::c_int as usize] -
                    vec_x *
                        (av[2 as libc::c_int as usize] -
                             g_alias.lightspot[2 as libc::c_int as usize]);
            point[1 as libc::c_int as usize] =
                av[1 as libc::c_int as usize] -
                    vec_y *
                        (av[2 as libc::c_int as usize] -
                             g_alias.lightspot[2 as libc::c_int as usize]);
            point[2 as libc::c_int as usize] =
                g_alias.lightspot[2 as libc::c_int as usize] + 1.0f32;
            pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
            verts0 = verts0.offset(1);
            verts1 = verts1.offset(1);
            count -= 1;
            if !(count != 0) { break ; }
        }
        pglEnd.expect("non-null function pointer")();
    }
    if glState.stencilEnabled as u64 != 0 {
        pglDisable.expect("non-null function pointer")(0xb90 as libc::c_int as
                                                           GLenum);
    };
}
/*
====================
R_AliasLerpMovement

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasLerpMovement(mut e: *mut cl_entity_t) {
    let mut f: libc::c_float = 1.0f32;
    // don't do it if the goalstarttime hasn't updated in a while.
	// NOTE: Because we need to interpolate multiplayer characters, the interpolation time limit
	// was increased to 1.0 s., which is 2x the max lag we are accounting for.
    if g_alias.interpolate as libc::c_uint != 0 &&
           g_alias.time < ((*e).curstate.animtime + 1.0f32) as libc::c_double
           && (*e).curstate.animtime != (*e).latched.prevanimtime {
        f =
            ((g_alias.time - (*e).curstate.animtime as libc::c_double) /
                 ((*e).curstate.animtime - (*e).latched.prevanimtime) as
                     libc::c_double) as libc::c_float
    } // monsters only
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_PLAYING_DEMO
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           == DEMO_QUAKE1 as libc::c_int {
        f = f + 1.0f32
    }
    g_alias.lerpfrac =
        if f >= 0.0f32 {
            if f < 1.0f32 { f } else { 1.0f32 }
        } else { 0.0f32 };
    if (*e).player as libc::c_uint != 0 ||
           (*e).curstate.movetype != 4 as libc::c_int {
        return
    }
    // Con_Printf( "%4.2f %.2f %.2f\n", f, e->curstate.animtime, g_alias.time );
    (*e).origin[0 as libc::c_int as usize] =
        (*e).latched.prevorigin[0 as libc::c_int as usize] +
            f *
                ((*e).curstate.origin[0 as libc::c_int as usize] -
                     (*e).latched.prevorigin[0 as libc::c_int as usize]);
    (*e).origin[1 as libc::c_int as usize] =
        (*e).latched.prevorigin[1 as libc::c_int as usize] +
            f *
                ((*e).curstate.origin[1 as libc::c_int as usize] -
                     (*e).latched.prevorigin[1 as libc::c_int as usize]);
    (*e).origin[2 as libc::c_int as usize] =
        (*e).latched.prevorigin[2 as libc::c_int as usize] +
            f *
                ((*e).curstate.origin[2 as libc::c_int as usize] -
                     (*e).latched.prevorigin[2 as libc::c_int as usize]);
    if VectorCompareEpsilon((*e).curstate.angles.as_mut_ptr() as *const vec_t,
                            (*e).latched.prevangles.as_mut_ptr() as
                                *const vec_t, 0.1f32) as u64 == 0 {
        let mut q: vec4_t = [0.; 4];
        let mut q1: vec4_t = [0.; 4];
        let mut q2: vec4_t = [0.; 4];
        AngleQuaternion((*e).curstate.angles.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr(), false_0);
        AngleQuaternion((*e).latched.prevangles.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr(), false_0);
        QuaternionSlerp(q2.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr() as *const vec_t, f, q.as_mut_ptr());
        QuaternionAngle(q.as_mut_ptr() as *const vec_t,
                        (*e).angles.as_mut_ptr());
    } else {
        (*e).angles[0 as libc::c_int as usize] =
            (*e).curstate.angles[0 as libc::c_int as usize];
        (*e).angles[1 as libc::c_int as usize] =
            (*e).curstate.angles[1 as libc::c_int as usize];
        (*e).angles[2 as libc::c_int as usize] =
            (*e).curstate.angles[2 as libc::c_int as usize]
    }
    // NOTE: this completely over control about angles and don't broke interpolation
    if (*(*e).model).flags & 0x8 as libc::c_int != 0 {
        (*e).angles[1 as libc::c_int as usize] =
            anglemod((100.0f32 as libc::c_double * g_alias.time) as
                         libc::c_float)
    };
}
/*
=================
R_SetupAliasFrame

=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupAliasFrame(mut e: *mut cl_entity_t,
                                           mut paliashdr: *mut aliashdr_t) {
    let mut newpose: libc::c_int = 0; // lerpframe from
    let mut oldpose: libc::c_int = 0; // lerpframe to
    let mut newframe: libc::c_int = 0;
    let mut oldframe: libc::c_int = 0;
    let mut numposes: libc::c_int = 0;
    let mut cycle: libc::c_int = 0;
    let mut interval: libc::c_float = 0.;
    oldframe = (*e).latched.prevframe as libc::c_int;
    newframe = (*e).curstate.frame as libc::c_int;
    if newframe < 0 as libc::c_int {
        newframe = 0 as libc::c_int
    } else if newframe >= (*paliashdr).numframes {
        if newframe > (*paliashdr).numframes {
            gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^3Warning:^7 R_GetAliasFrame: no such frame %d (%s)\n\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      newframe,
                                                                      (*(*e).model).name.as_mut_ptr());
        }
        newframe = (*paliashdr).numframes - 1 as libc::c_int
    }
    if oldframe >= (*paliashdr).numframes || oldframe < 0 as libc::c_int {
        oldframe = newframe
    }
    numposes =
        (*(*paliashdr).frames.as_mut_ptr().offset(newframe as
                                                      isize)).numposes;
    if numposes > 1 as libc::c_int {
        newpose =
            (*(*paliashdr).frames.as_mut_ptr().offset(newframe as
                                                          isize)).firstpose;
        oldpose = newpose;
        interval =
            1.0f32 /
                (*(*paliashdr).frames.as_mut_ptr().offset(newframe as
                                                              isize)).interval;
        cycle = (g_alias.time * interval as libc::c_double) as libc::c_int;
        oldpose += (cycle + 0 as libc::c_int) % numposes;
        newpose += (cycle + 1 as libc::c_int) % numposes;
        g_alias.lerpfrac =
            (g_alias.time * interval as libc::c_double) as libc::c_float;
        g_alias.lerpfrac -= g_alias.lerpfrac as libc::c_int as libc::c_float
    } else {
        oldpose =
            (*(*paliashdr).frames.as_mut_ptr().offset(oldframe as
                                                          isize)).firstpose;
        newpose =
            (*(*paliashdr).frames.as_mut_ptr().offset(newframe as
                                                          isize)).firstpose
    }
    g_alias.oldpose = oldpose;
    g_alias.newpose = newpose;
    GL_DrawAliasFrame(paliashdr);
}
/*
===============
R_StudioDrawAbsBBox

===============
*/
unsafe extern "C" fn R_AliasDrawAbsBBox(mut e: *mut cl_entity_t,
                                        mut absmin: *const vec_t,
                                        mut absmax: *const vec_t) {
    let mut p: [vec3_t; 8] = [[0.; 3]; 8];
    let mut i: libc::c_int = 0;
    // looks ugly, skip
    if (*r_drawentities).value != 5 as libc::c_int as libc::c_float ||
           e == gEngfuncs.GetViewModel.expect("non-null function pointer")() {
        return
    }
    // compute a full bounding box
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        p[i as usize][0 as libc::c_int as usize] =
            if i & 1 as libc::c_int != 0 {
                *absmin.offset(0 as libc::c_int as isize)
            } else { *absmax.offset(0 as libc::c_int as isize) };
        p[i as usize][1 as libc::c_int as usize] =
            if i & 2 as libc::c_int != 0 {
                *absmin.offset(1 as libc::c_int as isize)
            } else { *absmax.offset(1 as libc::c_int as isize) };
        p[i as usize][2 as libc::c_int as usize] =
            if i & 4 as libc::c_int != 0 {
                *absmin.offset(2 as libc::c_int as isize)
            } else { *absmax.offset(2 as libc::c_int as isize) };
        i += 1
    }
    GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.whiteTexture as GLenum);
    TriColor4f(0.5f32, 0.5f32, 1.0f32, 0.5f32);
    TriRenderMode(kRenderTransAdd as libc::c_int);
    pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int as
                                                       GLfloat);
    TriBegin(2 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        TriBrightness(g_alias.shadelight / 255.0f32);
        TriVertex3fv(p[boxpnt[i as usize][0 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        TriVertex3fv(p[boxpnt[i as usize][1 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        TriVertex3fv(p[boxpnt[i as usize][2 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        TriVertex3fv(p[boxpnt[i as usize][3 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        i += 1
    }
    TriEnd();
    TriRenderMode(kRenderNormal as libc::c_int);
}
unsafe extern "C" fn R_AliasDrawLightTrace(mut e: *mut cl_entity_t) {
    if (*r_drawentities).value == 7 as libc::c_int as libc::c_float {
        let mut origin: vec3_t = [0.; 3];
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglDisable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                           GLenum);
        pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLenum);
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0.5f64 as GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")((*e).origin.as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(g_alias.lightspot.as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLenum);
        pglColor3f.expect("non-null function pointer")(0 as libc::c_int as
                                                           GLfloat,
                                                       0.5f64 as GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat);
        origin[0 as libc::c_int as usize] =
            g_alias.lightspot[0 as libc::c_int as usize] +
                -64.0f32 * g_alias.lightvec[0 as libc::c_int as usize];
        origin[1 as libc::c_int as usize] =
            g_alias.lightspot[1 as libc::c_int as usize] +
                -64.0f32 * g_alias.lightvec[1 as libc::c_int as usize];
        origin[2 as libc::c_int as usize] =
            g_alias.lightspot[2 as libc::c_int as usize] +
                -64.0f32 * g_alias.lightvec[2 as libc::c_int as usize];
        pglVertex3fv.expect("non-null function pointer")(g_alias.lightspot.as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(origin.as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglPointSize.expect("non-null function pointer")(5.0f32);
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglBegin.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLenum);
        pglVertex3fv.expect("non-null function pointer")(g_alias.lightspot.as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglPointSize.expect("non-null function pointer")(1.0f32);
        pglEnable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                          GLenum);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
    };
}
/*
================
R_AliasSetupTimings

init current time for a given model
================
*/
unsafe extern "C" fn R_AliasSetupTimings() {
    if RI.drawWorld as u64 != 0 {
        // synchronize with server time
        g_alias.time = (*gpGlobals).time as libc::c_double
    } else {
        // menu stuff
        g_alias.time = (*gpGlobals).realtime
    }
    m_fDoRemap = false_0;
}
/*
=================
R_DrawAliasModel

=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawAliasModel(mut e: *mut cl_entity_t) {
    let mut clmodel: *mut model_t = 0 as *mut model_t;
    let mut absmin: vec3_t = [0.; 3];
    let mut absmax: vec3_t = [0.; 3];
    let mut pinfo: *mut remap_info_t = 0 as *mut remap_info_t;
    let mut anim: libc::c_int = 0;
    let mut skin: libc::c_int = 0;
    let mut lighting: alight_t =
        alight_t{ambientlight: 0,
                 shadelight: 0,
                 color: [0.; 3],
                 plightvec: 0 as *mut libc::c_float,};
    let mut playerinfo: *mut player_info_t = 0 as *mut player_info_t;
    let mut dir: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    clmodel = (*RI.currententity).model;
    absmin[0 as libc::c_int as usize] =
        (*e).origin[0 as libc::c_int as usize] +
            (*clmodel).mins[0 as libc::c_int as usize];
    absmin[1 as libc::c_int as usize] =
        (*e).origin[1 as libc::c_int as usize] +
            (*clmodel).mins[1 as libc::c_int as usize];
    absmin[2 as libc::c_int as usize] =
        (*e).origin[2 as libc::c_int as usize] +
            (*clmodel).mins[2 as libc::c_int as usize];
    absmax[0 as libc::c_int as usize] =
        (*e).origin[0 as libc::c_int as usize] +
            (*clmodel).maxs[0 as libc::c_int as usize];
    absmax[1 as libc::c_int as usize] =
        (*e).origin[1 as libc::c_int as usize] +
            (*clmodel).maxs[1 as libc::c_int as usize];
    absmax[2 as libc::c_int as usize] =
        (*e).origin[2 as libc::c_int as usize] +
            (*clmodel).maxs[2 as libc::c_int as usize];
    if R_CullModel(e, absmin.as_mut_ptr() as *const vec_t,
                   absmax.as_mut_ptr() as *const vec_t) != 0 {
        return
    }
    //
	// locate the proper data
	//
    m_pAliasHeader =
        gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_alias
                                                                        as
                                                                        libc::c_int,
                                                                    (*RI.currententity).model)
            as *mut aliashdr_t;
    if m_pAliasHeader.is_null() { return }
    // init time
    R_AliasSetupTimings();
    // angles will be modify below keep original
    angles[0 as libc::c_int as usize] =
        (*e).angles[0 as libc::c_int as usize]; // stupid quake bug
    angles[1 as libc::c_int as usize] =
        (*e).angles[1 as libc::c_int as usize];
    angles[2 as libc::c_int as usize] =
        (*e).angles[2 as libc::c_int as usize];
    R_AliasLerpMovement(e);
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        (*e).angles[0 as libc::c_int as usize] =
            -(*e).angles[0 as libc::c_int as usize]
    }
    // don't rotate clients, only aim
    if (*e).player as u64 != 0 {
        (*e).angles[0 as libc::c_int as usize] = 0.0f32
    }
    //
	// get lighting information
	//
    lighting.plightvec = dir.as_mut_ptr();
    R_AliasDynamicLight(e, &mut lighting);
    r_stats.c_alias_polys =
        (r_stats.c_alias_polys as
             libc::c_uint).wrapping_add((*m_pAliasHeader).numtris as
                                            libc::c_uint) as uint as uint;
    r_stats.c_alias_models_drawn =
        r_stats.c_alias_models_drawn.wrapping_add(1);
    //
	// draw all the triangles
	//
    R_RotateForEntity(e);
    // model and frame independant
    R_AliasSetupLighting(&mut lighting);
    GL_SetRenderMode((*e).curstate.rendermode);
    // setup remapping only for players
    if (*e).player as libc::c_uint != 0 &&
           {
               playerinfo =
                   pfnPlayerInfo((*e).curstate.number - 1 as libc::c_int);
               !playerinfo.is_null()
           } {
        // get remap colors
        let mut topcolor: libc::c_int =
            if (*playerinfo).topcolor >= 0 as libc::c_int {
                if (*playerinfo).topcolor < 13 as libc::c_int {
                    (*playerinfo).topcolor
                } else { 13 as libc::c_int }
            } else {
                0 as libc::c_int
            }; // FIXME: allow remapping for skingroups someday
        let mut bottomcolor: libc::c_int =
            if (*playerinfo).bottomcolor >= 0 as libc::c_int {
                if (*playerinfo).bottomcolor < 13 as libc::c_int {
                    (*playerinfo).bottomcolor
                } else { 13 as libc::c_int }
            } else { 0 as libc::c_int };
        R_AliasSetRemapColors(topcolor, bottomcolor);
    }
    if tr.fFlipViewModel as u64 != 0 {
        pglTranslatef.expect("non-null function pointer")((*m_pAliasHeader).scale_origin[0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize],
                                                          -(*m_pAliasHeader).scale_origin[1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize],
                                                          (*m_pAliasHeader).scale_origin[2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]);
        pglScalef.expect("non-null function pointer")((*m_pAliasHeader).scale[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize],
                                                      -(*m_pAliasHeader).scale[1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize],
                                                      (*m_pAliasHeader).scale[2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]);
    } else {
        pglTranslatef.expect("non-null function pointer")((*m_pAliasHeader).scale_origin[0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize],
                                                          (*m_pAliasHeader).scale_origin[1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize],
                                                          (*m_pAliasHeader).scale_origin[2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]);
        pglScalef.expect("non-null function pointer")((*m_pAliasHeader).scale[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize],
                                                      (*m_pAliasHeader).scale[1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize],
                                                      (*m_pAliasHeader).scale[2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]);
    }
    anim =
        (g_alias.time * 10 as libc::c_int as libc::c_double) as libc::c_int &
            3 as libc::c_int;
    skin =
        if (*RI.currententity).curstate.skin as libc::c_int >=
               0 as libc::c_int {
            if ((*RI.currententity).curstate.skin as libc::c_int) <
                   (*m_pAliasHeader).numskins - 1 as libc::c_int {
                (*RI.currententity).curstate.skin as libc::c_int
            } else { ((*m_pAliasHeader).numskins) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    if m_fDoRemap as u64 != 0 {
        pinfo =
            gEngfuncs.CL_GetRemapInfoForEntity.expect("non-null function pointer")(e)
    }
    if (*r_lightmap).value != 0. && (*r_fullbright).value == 0. {
        GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.whiteTexture as GLenum);
    } else if !pinfo.is_null() &&
                  (*pinfo).textures[skin as usize] as libc::c_int !=
                      0 as libc::c_int {
        GL_Bind(XASH_TEXTURE0 as libc::c_int,
                (*pinfo).textures[skin as usize] as GLenum);
    } else {
        GL_Bind(XASH_TEXTURE0 as libc::c_int,
                (*m_pAliasHeader).gl_texturenum[skin as usize][anim as usize]
                    as GLenum);
        if (*R_GetTexture((*m_pAliasHeader).gl_texturenum[skin as
                                                              usize][anim as
                                                                         usize]
                              as GLenum)).flags as libc::c_uint &
               TF_HAS_ALPHA as libc::c_int as libc::c_uint != 0 {
            pglEnable.expect("non-null function pointer")(0xbc0 as libc::c_int
                                                              as GLenum);
            pglAlphaFunc.expect("non-null function pointer")(0x204 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0.0f32);
            tr.blend = 1.0f32
        }
    }
    pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int as
                                                       GLfloat);
    if GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as libc::c_uint != 0 &&
           (*m_pAliasHeader).fb_texturenum[skin as usize][anim as usize] as
               libc::c_int != 0 {
        GL_Bind(XASH_TEXTURE1 as libc::c_int,
                (*m_pAliasHeader).fb_texturenum[skin as usize][anim as usize]
                    as GLenum);
        pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x2200 as libc::c_int
                                                           as GLenum,
                                                       0x104 as libc::c_int as
                                                           GLfloat);
    }
    pglShadeModel.expect("non-null function pointer")(0x1d01 as libc::c_int as
                                                          GLenum);
    R_SetupAliasFrame(e, m_pAliasHeader);
    if GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as libc::c_uint != 0 &&
           (*m_pAliasHeader).fb_texturenum[skin as usize][anim as usize] as
               libc::c_int != 0 {
        GL_CleanUpTextureUnits(1 as libc::c_int);
    }
    pglShadeModel.expect("non-null function pointer")(0x1d00 as libc::c_int as
                                                          GLenum);
    R_LoadIdentity();
    // get lerped origin
    absmin[0 as libc::c_int as usize] =
        (*e).origin[0 as libc::c_int as usize] +
            (*clmodel).mins[0 as libc::c_int as usize];
    absmin[1 as libc::c_int as usize] =
        (*e).origin[1 as libc::c_int as usize] +
            (*clmodel).mins[1 as libc::c_int as usize];
    absmin[2 as libc::c_int as usize] =
        (*e).origin[2 as libc::c_int as usize] +
            (*clmodel).mins[2 as libc::c_int as usize];
    absmax[0 as libc::c_int as usize] =
        (*e).origin[0 as libc::c_int as usize] +
            (*clmodel).maxs[0 as libc::c_int as usize];
    absmax[1 as libc::c_int as usize] =
        (*e).origin[1 as libc::c_int as usize] +
            (*clmodel).maxs[1 as libc::c_int as usize];
    absmax[2 as libc::c_int as usize] =
        (*e).origin[2 as libc::c_int as usize] +
            (*clmodel).maxs[2 as libc::c_int as usize];
    R_AliasDrawAbsBBox(e, absmin.as_mut_ptr() as *const vec_t,
                       absmax.as_mut_ptr() as *const vec_t);
    R_AliasDrawLightTrace(e);
    pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int as
                                                       GLfloat);
    pglAlphaFunc.expect("non-null function pointer")(0x204 as libc::c_int as
                                                         GLenum, 0.0f32);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    if r_shadows.value != 0. {
        // need to compute transformation matrix
        Matrix4x4_CreateFromEntity(RI.objectMatrix.as_mut_ptr(),
                                   (*e).angles.as_mut_ptr() as *const vec_t,
                                   (*e).origin.as_mut_ptr() as *const vec_t,
                                   1.0f32);
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglBlendFunc.expect("non-null function pointer")(0x302 as libc::c_int
                                                             as GLenum,
                                                         0x303 as libc::c_int
                                                             as GLenum);
        pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                          GLenum);
        pglColor4f.expect("non-null function pointer")(0.0f32, 0.0f32, 0.0f32,
                                                       0.5f32);
        pglDepthFunc.expect("non-null function pointer")(0x201 as libc::c_int
                                                             as GLenum);
        GL_DrawAliasShadow(m_pAliasHeader);
        pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int
                                                             as GLenum);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                           GLenum);
        pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                       1.0f32);
        R_LoadIdentity();
    }
    // restore original angles
    (*e).angles[0 as libc::c_int as usize] =
        angles[0 as libc::c_int as usize];
    (*e).angles[1 as libc::c_int as usize] =
        angles[1 as libc::c_int as usize];
    (*e).angles[2 as libc::c_int as usize] =
        angles[2 as libc::c_int as usize];
}
//==================================================================================
