#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut aliastriangleparms: aliastriangleparms_t;
    #[no_mangle]
    static mut aliasxscale: libc::c_float;
    #[no_mangle]
    static mut aliasyscale: libc::c_float;
    #[no_mangle]
    static mut aliasxcenter: libc::c_float;
    #[no_mangle]
    static mut aliasycenter: libc::c_float;
    #[no_mangle]
    fn R_DrawTriangle();
    #[no_mangle]
    fn R_AliasClipTriangle(index0: *mut finalvert_t, index1: *mut finalvert_t,
                           index2: *mut finalvert_t);
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn Matrix3x4_ConcatTransforms(out: *mut [vec_t; 4],
                                  in1: *const [vec_t; 4],
                                  in2: *const [vec_t; 4]);
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
pub struct dtriangle_t {
    pub index_xyz: [libc::c_short; 3],
    pub index_st: [libc::c_short; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliastriangleparms_t {
    pub a: *mut finalvert_t,
    pub b: *mut finalvert_t,
    pub c: *mut finalvert_t,
}
// not really draw alias models here, but use this to draw triangles
#[no_mangle]
pub static mut r_affinetridesc: affinetridesc_t =
    affinetridesc_t{pskin: 0 as *const libc::c_void as *mut libc::c_void,
                    pskindesc: 0,
                    skinwidth: 0,
                    skinheight: 0,
                    ptriangles: 0 as *const dtriangle_t as *mut dtriangle_t,
                    pfinalverts: 0 as *const finalvert_t as *mut finalvert_t,
                    numtriangles: 0,
                    drawtype: 0,
                    seamfixupX16: 0,
                    do_vis_thresh: false_0,
                    vis_thresh: 0,};
#[no_mangle]
pub static mut r_aliasblendcolor: libc::c_int = 0;
#[no_mangle]
pub static mut aliastransform: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
#[no_mangle]
pub static mut aliasworldtransform: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
#[no_mangle]
pub static mut aliasoldworldtransform: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
#[no_mangle]
pub static mut s_ziscale: libc::c_float = 0.;
static mut s_alias_forward: vec3_t = [0.; 3];
static mut s_alias_right: vec3_t = [0.; 3];
static mut s_alias_up: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_avertexnormals: [[libc::c_float; 3]; 162] =
    [[-0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [-0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.955423f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      1.000000f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.850651f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.525731f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.955423f64 as libc::c_float],
     [0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, 0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [-0.525731f64 as libc::c_float, 0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.850651f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.955423f64 as libc::c_float,
      -0.295242f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 1.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.955423f64 as libc::c_float,
      0.295242f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [0.850651f64 as libc::c_float, 0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [0.525731f64 as libc::c_float, 0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [0.955423f64 as libc::c_float, 0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [1.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [0.850651f64 as libc::c_float, -0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.955423f64 as libc::c_float, -0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.525731f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [-0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [-0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.955423f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -1.000000f64 as libc::c_float],
     [0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.955423f64 as libc::c_float],
     [0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.850651f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.525731f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float],
     [0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.955423f64 as libc::c_float,
      -0.295242f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -1.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.850651f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.955423f64 as libc::c_float,
      0.295242f64 as libc::c_float],
     [0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [0.525731f64 as libc::c_float, -0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, -0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [-0.525731f64 as libc::c_float, -0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.525731f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [-0.955423f64 as libc::c_float, 0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [-1.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [-0.955423f64 as libc::c_float, -0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float]];
/*
================
R_AliasCheckBBox
================
*/
/*
================
R_AliasTransformVector
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasTransformVector(mut in_0: *mut vec_t,
                                                mut out: *mut vec_t,
                                                mut xf:
                                                    *mut [libc::c_float; 4]) {
    *out.offset(0 as libc::c_int as isize) =
        *in_0.offset(0 as libc::c_int as isize) *
            (*xf.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            +
            *in_0.offset(1 as libc::c_int as isize) *
                (*xf.offset(0 as libc::c_int as
                                isize))[1 as libc::c_int as usize] +
            *in_0.offset(2 as libc::c_int as isize) *
                (*xf.offset(0 as libc::c_int as
                                isize))[2 as libc::c_int as usize] +
            (*xf.offset(0 as libc::c_int as
                            isize))[3 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *in_0.offset(0 as libc::c_int as isize) *
            (*xf.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            +
            *in_0.offset(1 as libc::c_int as isize) *
                (*xf.offset(1 as libc::c_int as
                                isize))[1 as libc::c_int as usize] +
            *in_0.offset(2 as libc::c_int as isize) *
                (*xf.offset(1 as libc::c_int as
                                isize))[2 as libc::c_int as usize] +
            (*xf.offset(1 as libc::c_int as
                            isize))[3 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *in_0.offset(0 as libc::c_int as isize) *
            (*xf.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            +
            *in_0.offset(1 as libc::c_int as isize) *
                (*xf.offset(2 as libc::c_int as
                                isize))[1 as libc::c_int as usize] +
            *in_0.offset(2 as libc::c_int as isize) *
                (*xf.offset(2 as libc::c_int as
                                isize))[2 as libc::c_int as usize] +
            (*xf.offset(2 as libc::c_int as
                            isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn VectorInverse(mut v: *mut vec_t) {
    *v.offset(0 as libc::c_int as isize) =
        -*v.offset(0 as libc::c_int as isize);
    *v.offset(1 as libc::c_int as isize) =
        -*v.offset(1 as libc::c_int as isize);
    *v.offset(2 as libc::c_int as isize) =
        -*v.offset(2 as libc::c_int as isize);
}
/*
================
R_SetUpWorldTransform
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetUpWorldTransform() {
    let mut i: libc::c_int = 0;
    static mut viewmatrix: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
    let mut angles: vec3_t = [0.; 3];
    // TODO: should really be stored with the entity instead of being reconstructed
// TODO: should use a look-up table
// TODO: could cache lazily, stored in the entity
//
    s_ziscale =
        0x8000 as libc::c_int as libc::c_float *
            0x10000 as libc::c_int as libc::c_float;
    angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    angles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    angles[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    AngleVectors(angles.as_mut_ptr() as *const vec_t,
                 s_alias_forward.as_mut_ptr(), s_alias_right.as_mut_ptr(),
                 s_alias_up.as_mut_ptr());
    // TODO: can do this with simple matrix rearrangement
    memset(aliasworldtransform.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[libc::c_float; 4]; 3]>() as libc::c_ulong);
    memset(aliasoldworldtransform.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[libc::c_float; 4]; 3]>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        aliasworldtransform[i as usize][0 as libc::c_int as usize] =
            s_alias_forward[i as usize];
        aliasoldworldtransform[i as usize][0 as libc::c_int as usize] =
            aliasworldtransform[i as usize][0 as libc::c_int as usize];
        aliasworldtransform[i as usize][1 as libc::c_int as usize] =
            -s_alias_right[i as usize];
        aliasoldworldtransform[i as usize][0 as libc::c_int as usize] =
            aliasworldtransform[i as usize][1 as libc::c_int as usize];
        aliasworldtransform[i as usize][2 as libc::c_int as usize] =
            s_alias_up[i as usize];
        aliasoldworldtransform[i as usize][0 as libc::c_int as usize] =
            aliasworldtransform[i as usize][2 as libc::c_int as usize];
        i += 1
    }
    aliasworldtransform[0 as libc::c_int as usize][3 as libc::c_int as usize]
        = -RI.vieworg[0 as libc::c_int as usize];
    aliasworldtransform[1 as libc::c_int as usize][3 as libc::c_int as usize]
        = -RI.vieworg[1 as libc::c_int as usize];
    aliasworldtransform[2 as libc::c_int as usize][3 as libc::c_int as usize]
        = -RI.vieworg[2 as libc::c_int as usize];
    //aliasoldworldtransform[0][3] = RI.currententity->oldorigin[0]-r_origin[0];
	//aliasoldworldtransform[1][3] = RI.currententity->oldorigin[1]-r_origin[1];
	//aliasoldworldtransform[2][3] = RI.currententity->oldorigin[2]-r_origin[2];
    // FIXME: can do more efficiently than full concatenation
//	memcpy( rotationmatrix, t2matrix, sizeof( rotationmatrix ) );
    //	R_ConcatTransforms (t2matrix, tmatrix, rotationmatrix);
    // TODO: should be global, set when vright, etc., set
    viewmatrix[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        RI.vright[0 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        RI.vright[1 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        RI.vright[2 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        RI.vup[0 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        RI.vup[1 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        RI.vup[2 as libc::c_int as usize];
    VectorInverse(viewmatrix[1 as libc::c_int as usize].as_mut_ptr());
    //VectorScale(viewmatrix[1], -1, viewmatrix[1]);
    viewmatrix[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        RI.vforward[0 as libc::c_int as usize];
    viewmatrix[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        RI.vforward[1 as libc::c_int as usize];
    viewmatrix[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        RI.vforward[2 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    viewmatrix[1 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    viewmatrix[2 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    //	memcpy( aliasworldtransform, rotationmatrix, sizeof( aliastransform ) );
    //R_ConcatTransforms (viewmatrix, aliasworldtransform, aliastransform);
    Matrix3x4_ConcatTransforms(aliastransform.as_mut_ptr(),
                               viewmatrix.as_mut_ptr() as *const [vec_t; 4],
                               aliasworldtransform.as_mut_ptr() as
                                   *const [vec_t; 4]);
    aliasworldtransform[0 as libc::c_int as usize][3 as libc::c_int as usize]
        = 0 as libc::c_int as libc::c_float;
    aliasworldtransform[1 as libc::c_int as usize][3 as libc::c_int as usize]
        = 0 as libc::c_int as libc::c_float;
    aliasworldtransform[2 as libc::c_int as usize][3 as libc::c_int as usize]
        = 0 as libc::c_int as libc::c_float;
    //aliasoldworldtransform[0][3] = RI.currententity->oldorigin[0];
	//aliasoldworldtransform[1][3] = RI.currententity->oldorigin[1];
	//aliasoldworldtransform[2][3] = RI.currententity->oldorigin[2];
}
/*
================
R_AliasSetUpTransform
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetUpTransform() {
    let mut i: libc::c_int = 0;
    static mut viewmatrix: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
    let mut angles: vec3_t = [0.; 3];
    // TODO: should really be stored with the entity instead of being reconstructed
// TODO: should use a look-up table
// TODO: could cache lazily, stored in the entity
//
    s_ziscale =
        0x8000 as libc::c_int as libc::c_float *
            0x10000 as libc::c_int as libc::c_float;
    angles[2 as libc::c_int as usize] =
        (*RI.currententity).angles[2 as libc::c_int as usize];
    angles[0 as libc::c_int as usize] =
        (*RI.currententity).angles[0 as libc::c_int as usize];
    angles[1 as libc::c_int as usize] =
        (*RI.currententity).angles[1 as libc::c_int as usize];
    AngleVectors(angles.as_mut_ptr() as *const vec_t,
                 s_alias_forward.as_mut_ptr(), s_alias_right.as_mut_ptr(),
                 s_alias_up.as_mut_ptr());
    // TODO: can do this with simple matrix rearrangement
    memset(aliasworldtransform.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[libc::c_float; 4]; 3]>() as libc::c_ulong);
    memset(aliasoldworldtransform.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[libc::c_float; 4]; 3]>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        aliasworldtransform[i as usize][0 as libc::c_int as usize] =
            s_alias_forward[i as usize];
        aliasoldworldtransform[i as usize][0 as libc::c_int as usize] =
            aliasworldtransform[i as usize][0 as libc::c_int as usize];
        aliasworldtransform[i as usize][1 as libc::c_int as usize] =
            -s_alias_right[i as usize];
        aliasoldworldtransform[i as usize][0 as libc::c_int as usize] =
            aliasworldtransform[i as usize][1 as libc::c_int as usize];
        aliasworldtransform[i as usize][2 as libc::c_int as usize] =
            s_alias_up[i as usize];
        aliasoldworldtransform[i as usize][0 as libc::c_int as usize] =
            aliasworldtransform[i as usize][2 as libc::c_int as usize];
        i += 1
    }
    aliasworldtransform[0 as libc::c_int as usize][3 as libc::c_int as usize]
        =
        (*RI.currententity).origin[0 as libc::c_int as usize] -
            RI.vieworg[0 as libc::c_int as usize];
    aliasworldtransform[1 as libc::c_int as usize][3 as libc::c_int as usize]
        =
        (*RI.currententity).origin[1 as libc::c_int as usize] -
            RI.vieworg[1 as libc::c_int as usize];
    aliasworldtransform[2 as libc::c_int as usize][3 as libc::c_int as usize]
        =
        (*RI.currententity).origin[2 as libc::c_int as usize] -
            RI.vieworg[2 as libc::c_int as usize];
    //aliasoldworldtransform[0][3] = RI.currententity->oldorigin[0]-r_origin[0];
	//aliasoldworldtransform[1][3] = RI.currententity->oldorigin[1]-r_origin[1];
	//aliasoldworldtransform[2][3] = RI.currententity->oldorigin[2]-r_origin[2];
    // FIXME: can do more efficiently than full concatenation
//	memcpy( rotationmatrix, t2matrix, sizeof( rotationmatrix ) );
    //	R_ConcatTransforms (t2matrix, tmatrix, rotationmatrix);
    // TODO: should be global, set when vright, etc., set
    viewmatrix[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        RI.vright[0 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        RI.vright[1 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        RI.vright[2 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        RI.vup[0 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        RI.vup[1 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        RI.vup[2 as libc::c_int as usize];
    VectorInverse(viewmatrix[1 as libc::c_int as usize].as_mut_ptr());
    //VectorScale(viewmatrix[1], -1, viewmatrix[1]);
    viewmatrix[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        RI.vforward[0 as libc::c_int as usize];
    viewmatrix[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        RI.vforward[1 as libc::c_int as usize];
    viewmatrix[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        RI.vforward[2 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    viewmatrix[1 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    viewmatrix[2 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    //	memcpy( aliasworldtransform, rotationmatrix, sizeof( aliastransform ) );
    //R_ConcatTransforms (viewmatrix, aliasworldtransform, aliastransform);
    Matrix3x4_ConcatTransforms(aliastransform.as_mut_ptr(),
                               viewmatrix.as_mut_ptr() as *const [vec_t; 4],
                               aliasworldtransform.as_mut_ptr() as
                                   *const [vec_t; 4]);
    aliasworldtransform[0 as libc::c_int as usize][3 as libc::c_int as usize]
        = (*RI.currententity).origin[0 as libc::c_int as usize];
    aliasworldtransform[1 as libc::c_int as usize][3 as libc::c_int as usize]
        = (*RI.currententity).origin[1 as libc::c_int as usize];
    aliasworldtransform[2 as libc::c_int as usize][3 as libc::c_int as usize]
        = (*RI.currententity).origin[2 as libc::c_int as usize];
    //aliasoldworldtransform[0][3] = RI.currententity->oldorigin[0];
	//aliasoldworldtransform[1][3] = RI.currententity->oldorigin[1];
	//aliasoldworldtransform[2][3] = RI.currententity->oldorigin[2];
}
/*
================
R_AliasProjectAndClipTestFinalVert
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AliasProjectAndClipTestFinalVert(mut fv:
                                                                *mut finalvert_t) {
    let mut zi: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    // project points
    x = (*fv).xyz[0 as libc::c_int as usize]; // completely clipped
    y = (*fv).xyz[1 as libc::c_int as usize];
    z = (*fv).xyz[2 as libc::c_int as usize];
    zi = 1.0f32 / z;
    (*fv).zi = (zi * s_ziscale) as libc::c_int;
    (*fv).u = (x * aliasxscale * zi + aliasxcenter) as libc::c_int;
    (*fv).v = (y * aliasyscale * zi + aliasycenter) as libc::c_int;
    if (*fv).u < RI.aliasvrect.x { (*fv).flags |= 0x1 as libc::c_int }
    if (*fv).v < RI.aliasvrect.y { (*fv).flags |= 0x2 as libc::c_int }
    if (*fv).u > RI.aliasvrectright { (*fv).flags |= 0x4 as libc::c_int }
    if (*fv).v > RI.aliasvrectbottom { (*fv).flags |= 0x8 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasWorldToScreen(mut v: *const libc::c_float,
                                              mut out: *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            aliastransform[0 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                aliastransform[0 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                aliastransform[0 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
            aliastransform[0 as libc::c_int as
                               usize][3 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            aliastransform[1 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                aliastransform[1 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                aliastransform[1 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
            aliastransform[1 as libc::c_int as
                               usize][3 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            aliastransform[2 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                aliastransform[2 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                aliastransform[2 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
            aliastransform[2 as libc::c_int as
                               usize][3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn R_SetupFinalVert(mut fv: *mut finalvert_t,
                                          mut x: libc::c_float,
                                          mut y: libc::c_float,
                                          mut z: libc::c_float,
                                          mut light: libc::c_int,
                                          mut s: libc::c_int,
                                          mut t: libc::c_int) {
    let mut v: vec3_t = [x, y, z];
    (*fv).xyz[0 as libc::c_int as usize] =
        v[0 as libc::c_int as usize] *
            aliastransform[0 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
            v[1 as libc::c_int as usize] *
                aliastransform[0 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
            v[2 as libc::c_int as usize] *
                aliastransform[0 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
            aliastransform[0 as libc::c_int as
                               usize][3 as libc::c_int as usize];
    (*fv).xyz[1 as libc::c_int as usize] =
        v[0 as libc::c_int as usize] *
            aliastransform[1 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
            v[1 as libc::c_int as usize] *
                aliastransform[1 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
            v[2 as libc::c_int as usize] *
                aliastransform[1 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
            aliastransform[1 as libc::c_int as
                               usize][3 as libc::c_int as usize];
    (*fv).xyz[2 as libc::c_int as usize] =
        v[0 as libc::c_int as usize] *
            aliastransform[2 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
            v[1 as libc::c_int as usize] *
                aliastransform[2 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
            v[2 as libc::c_int as usize] *
                aliastransform[2 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
            aliastransform[2 as libc::c_int as
                               usize][3 as libc::c_int as usize];
    (*fv).flags = 0 as libc::c_int;
    (*fv).l = light;
    if (*fv).xyz[2 as libc::c_int as usize] <
           4 as libc::c_int as libc::c_float {
        (*fv).flags |= 0x10 as libc::c_int
    } else { R_AliasProjectAndClipTestFinalVert(fv); }
    (*fv).s = s << 16 as libc::c_int;
    (*fv).t = t << 16 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderTriangle(mut fv1: *mut finalvert_t,
                                          mut fv2: *mut finalvert_t,
                                          mut fv3: *mut finalvert_t) {
    if (*fv1).flags & (*fv2).flags & (*fv3).flags != 0 { return }
    if (*fv1).flags | (*fv2).flags | (*fv3).flags == 0 {
        // totally unclipped
        aliastriangleparms.a = fv1;
        aliastriangleparms.b = fv2;
        aliastriangleparms.c = fv3;
        R_DrawTriangle();
    } else {
        // partially clipped
        R_AliasClipTriangle(fv1, fv2, fv3);
    };
}
