#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_Bind(tmu: libc::c_int, texnum: libc::c_uint);
    #[no_mangle]
    fn R_GetTexture(texnum: libc::c_uint) -> *mut image_t;
    #[no_mangle]
    fn _TriColor4f(r: libc::c_float, g: libc::c_float, b: libc::c_float,
                   a: libc::c_float);
    #[no_mangle]
    fn GL_SetRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn R_GetSpriteFrame(pModel: *const model_t, frame: libc::c_int,
                        yaw: libc::c_float) -> *mut mspriteframe_t;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type rgba_t = [byte; 4];
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
pub struct mspriteframe_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub up: libc::c_float,
    pub down: libc::c_float,
    pub left: libc::c_float,
    pub right: libc::c_float,
    pub gl_texturenum: libc::c_int,
}
pub type mspriteframe_t = mspriteframe_s;
pub type cl_entity_t = cl_entity_s;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_0 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_0 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_0 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_0 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_0 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_0 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_0 = -1;
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_s {
    pub name: [libc::c_char; 256],
    pub srcWidth: word,
    pub srcHeight: word,
    pub width: word,
    pub height: word,
    pub depth: word,
    pub numMips: byte,
    pub flags: texFlags_t,
    pub fogParams: rgba_t,
    pub original: *mut rgbdata_t,
    pub size: size_t,
    pub xscale: libc::c_float,
    pub yscale: libc::c_float,
    pub type_0: imagetype_t,
    pub pixels: [*mut pixel_t; 4],
    pub alpha_pixels: *mut pixel_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub nextHash: *mut image_s,
}
pub type image_t = image_s;
/*
gl_draw.c - orthogonal drawing stuff
Copyright (C) 2010 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
/*
=============
R_GetImageParms
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetTextureParms(mut w: *mut libc::c_int,
                                           mut h: *mut libc::c_int,
                                           mut texnum: libc::c_int) {
    let mut glt: *mut image_t = 0 as *mut image_t;
    glt = R_GetTexture(texnum as libc::c_uint);
    if !w.is_null() { *w = (*glt).srcWidth as libc::c_int }
    if !h.is_null() { *h = (*glt).srcHeight as libc::c_int };
}
/*
=============
R_GetSpriteParms

same as GetImageParms but used
for sprite models
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetSpriteParms(mut frameWidth: *mut libc::c_int,
                                          mut frameHeight: *mut libc::c_int,
                                          mut numFrames: *mut libc::c_int,
                                          mut currentFrame: libc::c_int,
                                          mut pSprite: *const model_t) {
    let mut pFrame: *mut mspriteframe_t =
        0 as *mut mspriteframe_t; // bad model ?
    if pSprite.is_null() ||
           (*pSprite).type_0 as libc::c_int != mod_sprite as libc::c_int {
        return
    }
    pFrame = R_GetSpriteFrame(pSprite, currentFrame, 0.0f32);
    if !frameWidth.is_null() { *frameWidth = (*pFrame).width }
    if !frameHeight.is_null() { *frameHeight = (*pFrame).height }
    if !numFrames.is_null() { *numFrames = (*pSprite).numframes };
}
#[no_mangle]
pub unsafe extern "C" fn R_GetSpriteTexture(mut m_pSpriteModel:
                                                *const model_t,
                                            mut frame: libc::c_int)
 -> libc::c_int {
    if m_pSpriteModel.is_null() ||
           (*m_pSpriteModel).type_0 as libc::c_int !=
               mod_sprite as libc::c_int ||
           (*m_pSpriteModel).cache.data.is_null() {
        return 0 as libc::c_int
    }
    return (*R_GetSpriteFrame(m_pSpriteModel, frame, 0.0f32)).gl_texturenum;
}
/*
=============
Draw_StretchPicImplementation
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawStretchPicImplementation(mut x: libc::c_int,
                                                        mut y: libc::c_int,
                                                        mut w: libc::c_int,
                                                        mut h: libc::c_int,
                                                        mut s1: libc::c_int,
                                                        mut t1: libc::c_int,
                                                        mut s2: libc::c_int,
                                                        mut t2: libc::c_int,
                                                        mut pic:
                                                            *mut image_t) {
    let mut source: *mut pixel_t = 0 as *mut pixel_t;
    let mut dest: *mut pixel_t = 0 as *mut pixel_t;
    let mut v: libc::c_uint = 0;
    let mut u: libc::c_uint = 0;
    let mut sv: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut f: libc::c_uint = 0;
    let mut fstep: libc::c_uint = 0;
    let mut skip: libc::c_int = 0;
    let mut transparent: qboolean = false_0;
    let mut buffer: *mut pixel_t = 0 as *mut pixel_t;
    if x < 0 as libc::c_int { s1 += -x * (s2 - s1) / w; x = 0 as libc::c_int }
    if x + w > vid.width {
        s2 -= (x + w - vid.width) * (s2 - s1) / w;
        w = vid.width - x
    }
    if y + h > vid.height {
        t2 -= (y + h - vid.height) * (t2 - t1) / h;
        h = vid.height - y
    }
    if (*pic).pixels[0 as libc::c_int as usize].is_null() || s1 >= s2 ||
           t1 >= t2 {
        return
    }
    //gEngfuncs.Con_Printf ("pixels is %p\n", pic->pixels[0] );
    height = h as libc::c_uint;
    if y < 0 as libc::c_int {
        skip = -y;
        height = height.wrapping_add(y as libc::c_uint);
        y = 0 as libc::c_int
    } else { skip = 0 as libc::c_int }
    dest = vid.buffer.offset((y * vid.rowbytes) as isize).offset(x as isize);
    if !(*pic).alpha_pixels.is_null() {
        buffer = (*pic).alpha_pixels;
        transparent = true_0
    } else { buffer = (*pic).pixels[0 as libc::c_int as usize] }
    v = 0 as libc::c_int as libc::c_uint;
    while v < height {
        let mut alpha1: libc::c_int = vid.alpha as libc::c_int;
        sv =
            (skip as
                 libc::c_uint).wrapping_add(v).wrapping_mul((t2 - t1) as
                                                                libc::c_uint).wrapping_div(h
                                                                                               as
                                                                                               libc::c_uint).wrapping_add(t1
                                                                                                                              as
                                                                                                                              libc::c_uint);
        source =
            buffer.offset(sv.wrapping_mul((*pic).width as libc::c_uint) as
                              isize).offset(s1 as isize);
        f = 0 as libc::c_int as libc::c_uint;
        fstep = ((s2 - s1 << 16 as libc::c_int) / w) as libc::c_uint;
        u = 0 as libc::c_int as libc::c_uint;
        while u < w as libc::c_uint {
            let mut src: pixel_t =
                *source.offset((f >> 16 as libc::c_int) as isize);
            let mut alpha: libc::c_int = alpha1;
            f = f.wrapping_add(fstep);
            if transparent as u64 != 0 {
                alpha &=
                    src as libc::c_int >>
                        16 as libc::c_int - 3 as libc::c_int;
                src = ((src as libc::c_int) << 3 as libc::c_int) as pixel_t
            }
            if !(alpha == 0 as libc::c_int) {
                if vid.color as libc::c_int != 0xffff as libc::c_int {
                    src =
                        ((vid.modmap[(src as libc::c_int &
                                          0xff00 as libc::c_int |
                                          vid.color as libc::c_int >>
                                              8 as libc::c_int) as usize] as
                              libc::c_int) << 8 as libc::c_int |
                             src as libc::c_int & vid.color as libc::c_int &
                                 0xff as libc::c_int |
                             (src as libc::c_int & 0xff as libc::c_int) >>
                                 3 as libc::c_int) as pixel_t
                }
                if vid.rendermode == kRenderTransAdd as libc::c_int {
                    let mut screen: pixel_t = *dest.offset(u as isize);
                    *dest.offset(u as isize) =
                        ((vid.addmap[(src as libc::c_int &
                                          0xff00 as libc::c_int |
                                          screen as libc::c_int >>
                                              8 as libc::c_int) as usize] as
                              libc::c_int) << 8 as libc::c_int |
                             screen as libc::c_int & 0xff as libc::c_int |
                             (src as libc::c_int & 0xff as libc::c_int) >>
                                 0 as libc::c_int) as pixel_t
                } else if alpha < 7 as libc::c_int {
                    // && (vid.rendermode == kRenderTransAlpha || vid.rendermode == kRenderTransTexture ) )
                    let mut screen_0: pixel_t =
                        *dest.offset(u as isize); //  | 0xff & screen & src ;
                    *dest.offset(u as isize) =
                        if alpha > 3 as libc::c_int {
                            (vid.alphamap[(7 as libc::c_int - 1 as libc::c_int
                                               - alpha << 18 as libc::c_int |
                                               (screen_0 as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               src as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                src as libc::c_int & 0x3f as libc::c_int
                        } else {
                            (vid.alphamap[((alpha - 1 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (src as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               screen_0 as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                screen_0 as libc::c_int & 0x3f as libc::c_int
                        } as pixel_t
                } else { *dest.offset(u as isize) = src }
            }
            u = u.wrapping_add(1)
        }
        dest = dest.offset(vid.rowbytes as isize);
        v = v.wrapping_add(1)
    };
}
/*
=============
R_DrawStretchPic
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawStretchPic(mut x: libc::c_float,
                                          mut y: libc::c_float,
                                          mut w: libc::c_float,
                                          mut h: libc::c_float,
                                          mut s1: libc::c_float,
                                          mut t1: libc::c_float,
                                          mut s2: libc::c_float,
                                          mut t2: libc::c_float,
                                          mut texnum: libc::c_int) {
    let mut pic: *mut image_t = R_GetTexture(texnum as libc::c_uint);
    let mut width: libc::c_int = (*pic).width as libc::c_int;
    let mut height: libc::c_int = (*pic).height as libc::c_int;
    //	GL_Bind( XASH_TEXTURE0, texnum );
    if s2 > 1.0f32 || t2 > 1.0f32 { return }
    if s1 < 0.0f32 || t1 < 0.0f32 { return }
    if w < 1.0f32 || h < 1.0f32 { return }
    R_DrawStretchPicImplementation(x as libc::c_int, y as libc::c_int,
                                   w as libc::c_int, h as libc::c_int,
                                   (width as libc::c_float * s1) as
                                       libc::c_int,
                                   (height as libc::c_float * t1) as
                                       libc::c_int,
                                   (width as libc::c_float * s2) as
                                       libc::c_int,
                                   (height as libc::c_float * t2) as
                                       libc::c_int, pic);
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Fill(mut x: libc::c_int, mut y: libc::c_int,
                                   mut w: libc::c_int, mut h: libc::c_int) {
    let mut dest: *mut pixel_t = 0 as *mut pixel_t;
    let mut v: libc::c_uint = 0;
    let mut u: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut skip: libc::c_int = 0;
    let mut src: pixel_t = vid.color;
    let mut alpha: libc::c_int = vid.alpha as libc::c_int;
    if x < 0 as libc::c_int { x = 0 as libc::c_int }
    if x + w > vid.width { w = vid.width - x }
    if w <= 0 as libc::c_int { return }
    if y + h > vid.height { h = vid.height - y }
    if h <= 0 as libc::c_int { return }
    height = h as libc::c_uint;
    if y < 0 as libc::c_int {
        if h <= -y { return }
        skip = -y;
        height = height.wrapping_add(y as libc::c_uint);
        y = 0 as libc::c_int
    } else { skip = 0 as libc::c_int }
    dest = vid.buffer.offset((y * vid.rowbytes) as isize).offset(x as isize);
    v = 0 as libc::c_int as libc::c_uint;
    while v < height {
        u = 0 as libc::c_int as libc::c_uint;
        while u < w as libc::c_uint {
            if !(alpha == 0 as libc::c_int) {
                if vid.rendermode == kRenderTransAdd as libc::c_int {
                    let mut screen: pixel_t = *dest.offset(u as isize);
                    *dest.offset(u as isize) =
                        ((vid.addmap[(src as libc::c_int &
                                          0xff00 as libc::c_int |
                                          screen as libc::c_int >>
                                              8 as libc::c_int) as usize] as
                              libc::c_int) << 8 as libc::c_int |
                             screen as libc::c_int & 0xff as libc::c_int |
                             (src as libc::c_int & 0xff as libc::c_int) >>
                                 0 as libc::c_int) as pixel_t
                } else if alpha < 7 as libc::c_int {
                    // && (vid.rendermode == kRenderTransAlpha || vid.rendermode == kRenderTransTexture ) )
                    let mut screen_0: pixel_t =
                        *dest.offset(u as isize); //  | 0xff & screen & src ;
                    *dest.offset(u as isize) =
                        if alpha > 3 as libc::c_int {
                            (vid.alphamap[(7 as libc::c_int - 1 as libc::c_int
                                               - alpha << 18 as libc::c_int |
                                               (screen_0 as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               src as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                src as libc::c_int & 0x3f as libc::c_int
                        } else {
                            (vid.alphamap[((alpha - 1 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (src as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               screen_0 as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                screen_0 as libc::c_int & 0x3f as libc::c_int
                        } as pixel_t
                } else { *dest.offset(u as isize) = src }
            }
            u = u.wrapping_add(1)
        }
        dest = dest.offset(vid.rowbytes as isize);
        v = v.wrapping_add(1)
    };
}
/*
=============
Draw_TileClear

This repeats a 64*64 tile graphic to fill the screen around a sized down
refresh window.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawTileClear(mut texnum: libc::c_int,
                                         mut x: libc::c_int,
                                         mut y: libc::c_int,
                                         mut w: libc::c_int,
                                         mut h: libc::c_int) {
    let mut tw: libc::c_int = 0;
    let mut th: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pic: *mut image_t = 0 as *mut image_t;
    let mut psrc: *mut pixel_t = 0 as *mut pixel_t;
    let mut pdest: *mut pixel_t = 0 as *mut pixel_t;
    GL_SetRenderMode(kRenderNormal as libc::c_int);
    _TriColor4f(1.0f32, 1.0f32, 1.0f32, 1.0f32);
    GL_Bind(XASH_TEXTURE0 as libc::c_int, texnum as libc::c_uint);
    pic = R_GetTexture(texnum as libc::c_uint);
    tw = (*pic).width as libc::c_int;
    th = (*pic).height as libc::c_int;
    if x < 0 as libc::c_int { w += x; x = 0 as libc::c_int }
    if y < 0 as libc::c_int { h += y; y = 0 as libc::c_int }
    if x + w > vid.width { w = vid.width - x }
    if y + h > vid.height { h = vid.height - y }
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int { return }
    x2 = x + w;
    pdest = vid.buffer.offset((y * vid.rowbytes) as isize);
    i = 0 as libc::c_int;
    while i < h {
        psrc =
            (*pic).pixels[0 as libc::c_int as
                              usize].offset((tw * (i + y & 63 as libc::c_int))
                                                as isize);
        j = x;
        while j < x2 {
            *pdest.offset(j as isize) =
                *psrc.offset((j & 63 as libc::c_int) as isize);
            j += 1
        }
        i += 1;
        pdest = pdest.offset(vid.rowbytes as isize)
    };
}
/*
=============
R_DrawStretchRaw
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawStretchRaw(mut x: libc::c_float,
                                          mut y: libc::c_float,
                                          mut w: libc::c_float,
                                          mut h: libc::c_float,
                                          mut cols: libc::c_int,
                                          mut rows: libc::c_int,
                                          mut data: *const byte,
                                          mut dirty: qboolean) {
    let mut raw: *mut byte = 0 as *mut byte;
    let mut tex: *mut image_t = 0 as *mut image_t;
    raw = data as *mut byte;
    //pglDisable( GL_BLEND );
	//pglDisable( GL_ALPHA_TEST );
	//pglTexEnvi( GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_REPLACE );
    tex = R_GetTexture(tr.cinTexture as libc::c_uint);
    GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.cinTexture as libc::c_uint);
}
/*
=============
R_UploadStretchRaw
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_UploadStretchRaw(mut texture: libc::c_int,
                                            mut cols: libc::c_int,
                                            mut rows: libc::c_int,
                                            mut width: libc::c_int,
                                            mut height: libc::c_int,
                                            mut data: *const byte) {
    let mut raw: *mut byte = 0 as *mut byte;
    let mut tex: *mut image_t = 0 as *mut image_t;
    raw = data as *mut byte;
    tex = R_GetTexture(texture as libc::c_uint);
    GL_Bind(GL_KEEP_UNIT as libc::c_int, texture as libc::c_uint);
    (*tex).width = cols as word;
    (*tex).height = rows as word;
}
/*
===============
R_Set2DMode
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Set2DMode(mut enable: qboolean) {
    vid.color = 0xffff as libc::c_int as pixel_t;
    vid.is2d = enable;
    vid.alpha = 7 as libc::c_int as byte;
    if enable as u64 != 0 {
        //		if( glState.in2DMode )
	//		return;
        //	glState.in2DMode = true;
        RI.currententity = 0 as *mut cl_entity_t;
        RI.currentmodel = 0 as *mut model_t
    };
}
