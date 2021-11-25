#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type pmtrace_s;
    pub type con_nprint_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type physent_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    #[no_mangle]
    fn sinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tanf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_Invert_Full(out: *mut [vec_t; 4], in1: *const [vec_t; 4])
     -> qboolean;
    #[no_mangle]
    fn BoxOnPlaneSide(emins: *const vec_t, emaxs: *const vec_t,
                      p: *const mplane_t) -> libc::c_int;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn R_ClearDecals();
    #[no_mangle]
    fn R_Set2DMode(enable: qboolean);
    #[no_mangle]
    fn R_InitImages();
    #[no_mangle]
    fn R_ShutdownImages();
    #[no_mangle]
    fn R_PushDlights();
    #[no_mangle]
    fn R_MarkLights(light: *mut dlight_t, bit: libc::c_int,
                    node: *mut mnode_t);
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn R_DrawViewModel();
    #[no_mangle]
    fn R_SetUpWorldTransform();
    #[no_mangle]
    fn GL_SetRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn R_DrawSpriteModel(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_DrawStudioModel(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_ScanEdges();
    #[no_mangle]
    fn R_TransformFrustum();
    #[no_mangle]
    fn R_DrawSubmodelPolygons(pmodel: *mut model_t, clipflags: libc::c_int,
                              topnode: *mut mnode_t);
    #[no_mangle]
    fn R_DrawSolidClippedSubmodelPolygons(pmodel: *mut model_t,
                                          topnode: *mut mnode_t);
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn R_RotateBmodel();
    #[no_mangle]
    static mut r_entorigin: vec3_t;
    #[no_mangle]
    static mut qfrustum: qfrustum_s;
    #[no_mangle]
    fn R_BeginEdgeFrame();
    #[no_mangle]
    fn R_SurfacePatch();
    #[no_mangle]
    static mut surfaces: *mut surf_t;
    #[no_mangle]
    static mut surf_max: *mut surf_t;
    #[no_mangle]
    static mut r_edges: *mut edge_t;
    #[no_mangle]
    static mut auxedges: *mut edge_t;
    #[no_mangle]
    fn R_RenderWorld();
    #[no_mangle]
    fn Matrix4x4_Concat(out: *mut [vec_t; 4], in1: *const [vec_t; 4],
                        in2: *const [vec_t; 4]);
    #[no_mangle]
    fn Matrix4x4_CreateProjection(out: *mut [vec_t; 4], xMax: libc::c_float,
                                  xMin: libc::c_float, yMax: libc::c_float,
                                  yMin: libc::c_float, zNear: libc::c_float,
                                  zFar: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateOrtho(m: *mut [vec_t; 4], xLeft: libc::c_float,
                             xRight: libc::c_float, yBottom: libc::c_float,
                             yTop: libc::c_float, zNear: libc::c_float,
                             zFar: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_ConcatTranslate(out: *mut [vec_t; 4], x: libc::c_float,
                                 y: libc::c_float, z: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_ConcatRotate(out: *mut [vec_t; 4], angle: libc::c_float,
                              x: libc::c_float, y: libc::c_float,
                              z: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateModelview(out: *mut [vec_t; 4]);
    #[no_mangle]
    fn R_SetupFrameQ();
    #[no_mangle]
    fn R_GetEntityRenderMode(ent: *mut cl_entity_t) -> libc::c_int;
    #[no_mangle]
    fn D_FlushCaches();
    #[no_mangle]
    fn R_RunViewmodelEvents();
    #[no_mangle]
    static mut r_norefresh: *mut cvar_t;
    #[no_mangle]
    fn R_SpriteInit();
    #[no_mangle]
    fn R_StudioInit();
    #[no_mangle]
    fn R_StudioResetPlayerModels();
    #[no_mangle]
    static mut blanktable: [libc::c_int; 1280];
    #[no_mangle]
    static mut intsintable: [libc::c_int; 1280];
    #[no_mangle]
    static mut sintable: [libc::c_int; 1280];
    #[no_mangle]
    fn R_InitBlit(gl: qboolean);
    #[no_mangle]
    static mut sw_mipcap: *mut cvar_t;
    #[no_mangle]
    static mut sw_mipscale: *mut cvar_t;
    #[no_mangle]
    static mut vid_brightness: *mut cvar_t;
    #[no_mangle]
    static mut gl_emboss_scale: *mut cvar_t;
    #[no_mangle]
    fn R_BlitScreen();
    #[no_mangle]
    static mut surface_p: *mut surf_t;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn R_PolysetFillSpans8(_: *mut libc::c_void);
    #[no_mangle]
    fn R_PolysetDrawSpans8_33(pspanpackage: *mut libc::c_void);
    #[no_mangle]
    static mut alphaspans: qboolean;
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
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kRenderFxClampMinScale: C2RustUnnamed_0 = 20;
pub const kRenderFxGlowShell: C2RustUnnamed_0 = 19;
pub const kRenderFxExplode: C2RustUnnamed_0 = 18;
pub const kRenderFxDeadPlayer: C2RustUnnamed_0 = 17;
pub const kRenderFxHologram: C2RustUnnamed_0 = 16;
pub const kRenderFxDistort: C2RustUnnamed_0 = 15;
pub const kRenderFxNoDissipation: C2RustUnnamed_0 = 14;
pub const kRenderFxFlickerFast: C2RustUnnamed_0 = 13;
pub const kRenderFxFlickerSlow: C2RustUnnamed_0 = 12;
pub const kRenderFxStrobeFaster: C2RustUnnamed_0 = 11;
pub const kRenderFxStrobeFast: C2RustUnnamed_0 = 10;
pub const kRenderFxStrobeSlow: C2RustUnnamed_0 = 9;
pub const kRenderFxSolidFast: C2RustUnnamed_0 = 8;
pub const kRenderFxSolidSlow: C2RustUnnamed_0 = 7;
pub const kRenderFxFadeFast: C2RustUnnamed_0 = 6;
pub const kRenderFxFadeSlow: C2RustUnnamed_0 = 5;
pub const kRenderFxPulseFastWide: C2RustUnnamed_0 = 4;
pub const kRenderFxPulseSlowWide: C2RustUnnamed_0 = 3;
pub const kRenderFxPulseFast: C2RustUnnamed_0 = 2;
pub const kRenderFxPulseSlow: C2RustUnnamed_0 = 1;
pub const kRenderFxNone: C2RustUnnamed_0 = 0;
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
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
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
pub type ref_defaultsprite_e = libc::c_uint;
pub const REF_CHROME_SPRITE: ref_defaultsprite_e = 1;
pub const REF_DOT_SPRITE: ref_defaultsprite_e = 0;
pub type ref_graphic_apis_e = libc::c_uint;
pub const REF_D3D: ref_graphic_apis_e = 2;
pub const REF_GL: ref_graphic_apis_e = 1;
pub const REF_SOFTWARE: ref_graphic_apis_e = 0;
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
pub struct world_static_s {
    pub loading: qboolean,
    pub flags: libc::c_int,
    pub message: [libc::c_char; 2048],
    pub compiler: [libc::c_char; 256],
    pub generator: [libc::c_char; 256],
    pub hull_models: *mut hull_model_t,
    pub num_hull_models: libc::c_int,
    pub deluxedata: *mut color24,
    pub shadowdata: *mut byte,
    pub visbytes: size_t,
    pub fatbytes: size_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub size: vec3_t,
    pub recursion_level: libc::c_int,
    pub max_recursion: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hull_model_t {
    pub polys: hullnode_t,
    pub num_polys: uint,
}
pub type hullnode_t = hullnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hullnode_s {
    pub next: *mut hullnode_s,
    pub prev: *mut hullnode_s,
}
pub type ref_api_t = ref_api_s;
pub type ref_overview_t = ref_overview_s;
pub type ref_viewpass_t = ref_viewpass_s;
pub type fixed16_t = libc::c_int;
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
pub type clipplane_t = clipplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clipplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub next: *mut clipplane_s,
    pub leftedge: byte,
    pub rightedge: byte,
    pub reserved: [byte; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qfrustum_s {
    pub screenedge: [mplane_t; 4],
    pub view_clipplanes: [clipplane_t; 4],
    pub frustum_indexes: [libc::c_int; 24],
    pub pfrustum_indexes: [*mut libc::c_int; 4],
}
pub type surf_t = surf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surf_s {
    pub next: *mut surf_s,
    pub prev: *mut surf_s,
    pub spans: *mut espan_s,
    pub key: libc::c_int,
    pub last_u: libc::c_int,
    pub spanstate: libc::c_int,
    pub flags: libc::c_int,
    pub msurf: *mut msurface_t,
    pub entity: *mut cl_entity_t,
    pub nearzi: libc::c_float,
    pub insubmodel: qboolean,
    pub d_ziorigin: libc::c_float,
    pub d_zistepu: libc::c_float,
    pub d_zistepv: libc::c_float,
    pub pad: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct espan_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub count: libc::c_int,
    pub pnext: *mut espan_s,
}
pub type edge_t = edge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge_s {
    pub u: fixed16_t,
    pub u_step: fixed16_t,
    pub prev: *mut edge_s,
    pub next: *mut edge_s,
    pub surfs: [libc::c_ushort; 2],
    pub nextremove: *mut edge_s,
    pub nearzi: libc::c_float,
    pub owner: *mut medge_t,
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_float) -> libc::c_float {
    return sinf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin_0(mut __x: libc::c_double) -> libc::c_double {
    return sin(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_tan(mut __x: libc::c_float) -> libc::c_float {
    return tanf(__x);
}
/*
gl_rmain.c - renderer main loop
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
//#include "beamdef.h"
//#include "particledef.h"
#[no_mangle]
pub static mut r_cnumsurfs: libc::c_int = 0;
#[no_mangle]
pub static mut RI: ref_instance_t =
    ref_instance_t{params: 0,
                   drawWorld: false_0,
                   isSkyVisible: false_0,
                   onlyClientDraw: false_0,
                   drawOrtho: false_0,
                   fov_x: 0.,
                   fov_y: 0.,
                   currententity: 0 as *const cl_entity_t as *mut cl_entity_t,
                   currentmodel: 0 as *const model_t as *mut model_t,
                   currentbeam: 0 as *const cl_entity_t as *mut cl_entity_t,
                   viewport: [0; 4],
                   viewleaf: 0 as *const mleaf_t as *mut mleaf_t,
                   oldviewleaf: 0 as *const mleaf_t as *mut mleaf_t,
                   pvsorigin: [0.; 3],
                   vieworg: [0.; 3],
                   viewangles: [0.; 3],
                   vforward: [0.; 3],
                   vright: [0.; 3],
                   vup: [0.; 3],
                   base_vup: [0.; 3],
                   base_vpn: [0.; 3],
                   base_vright: [0.; 3],
                   cullorigin: [0.; 3],
                   cull_vforward: [0.; 3],
                   cull_vright: [0.; 3],
                   cull_vup: [0.; 3],
                   cached_contents: 0,
                   cached_waterlevel: 0,
                   farClip: 0.,
                   skyMins: [[0.; 6]; 2],
                   skyMaxs: [[0.; 6]; 2],
                   objectMatrix: [[0.; 4]; 4],
                   worldviewMatrix: [[0.; 4]; 4],
                   modelviewMatrix: [[0.; 4]; 4],
                   projectionMatrix: [[0.; 4]; 4],
                   worldviewProjectionMatrix: [[0.; 4]; 4],
                   visbytes: [0; 4096],
                   viewplanedist: 0.,
                   vrect:
                       vrect_t{x: 0,
                               y: 0,
                               width: 0,
                               height: 0,
                               pnext: 0 as *const vrect_s as *mut vrect_s,},
                   aliasvrect:
                       vrect_t{x: 0,
                               y: 0,
                               width: 0,
                               height: 0,
                               pnext: 0 as *const vrect_s as *mut vrect_s,},
                   vrectright: 0,
                   vrectbottom: 0,
                   aliasvrectright: 0,
                   aliasvrectbottom: 0,
                   vrectrightedge: 0.,
                   fvrectx: 0.,
                   fvrecty: 0.,
                   fvrectx_adj: 0.,
                   fvrecty_adj: 0.,
                   vrect_x_adj_shift20: 0,
                   vrectright_adj_shift20: 0,
                   fvrectright_adj: 0.,
                   fvrectbottom_adj: 0.,
                   fvrectright: 0.,
                   fvrectbottom: 0.,};
// quake defines. will be refactored
// view origin
//
//
// screen size info
//
#[no_mangle]
pub static mut xcenter: libc::c_float = 0.;
#[no_mangle]
pub static mut ycenter: libc::c_float = 0.;
#[no_mangle]
pub static mut xscale: libc::c_float = 0.;
#[no_mangle]
pub static mut yscale: libc::c_float = 0.;
#[no_mangle]
pub static mut xscaleinv: libc::c_float = 0.;
#[no_mangle]
pub static mut yscaleinv: libc::c_float = 0.;
//float		xscaleshrink, yscaleshrink;
#[no_mangle]
pub static mut aliasxscale: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasyscale: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasxcenter: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasycenter: libc::c_float = 0.;
#[no_mangle]
pub static mut r_screenwidth: libc::c_int = 0;
//
// refresh flags
//
//int             d_spanpixcount;
//int             r_polycount;
//int             r_drawnpolycount;
//int             r_wholepolycount;
#[no_mangle]
pub static mut r_viewcluster: libc::c_int = 0;
#[no_mangle]
pub static mut r_oldviewcluster: libc::c_int = 0;
#[no_mangle]
pub static mut r_lefthand: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_aliasstats: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_allow_modex: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_clearcolor: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_drawflat: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_draworder: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_maxedges: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_maxsurfs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_reportedgeout: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_reportsurfout: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_stipplealpha: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_surfcacheoverride: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_waterwarp: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_texfilt: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_notransbrushes: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_noalphabrushes: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawworld: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawentities: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_dspeeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_fullbright: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lerpmodels: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_novis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lightmap: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_dynamic: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_traceglow: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut tracerred: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut tracergreen: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut tracerblue: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut traceralpha: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_speeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lightlevel: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
//FIXME HACK
#[no_mangle]
pub static mut vid_fullscreen: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_gamma: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
//PGM
#[no_mangle]
pub static mut sw_lockpvs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
//PGM
#[no_mangle]
pub static mut r_decals: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut d_sdivzstepu: libc::c_float = 0.;
#[no_mangle]
pub static mut d_tdivzstepu: libc::c_float = 0.;
#[no_mangle]
pub static mut d_zistepu: libc::c_float = 0.;
#[no_mangle]
pub static mut d_sdivzstepv: libc::c_float = 0.;
#[no_mangle]
pub static mut d_tdivzstepv: libc::c_float = 0.;
#[no_mangle]
pub static mut d_zistepv: libc::c_float = 0.;
#[no_mangle]
pub static mut d_sdivzorigin: libc::c_float = 0.;
#[no_mangle]
pub static mut d_tdivzorigin: libc::c_float = 0.;
#[no_mangle]
pub static mut d_ziorigin: libc::c_float = 0.;
#[no_mangle]
pub static mut sadjust: fixed16_t = 0;
#[no_mangle]
pub static mut tadjust: fixed16_t = 0;
#[no_mangle]
pub static mut bbextents: fixed16_t = 0;
#[no_mangle]
pub static mut bbextentt: fixed16_t = 0;
#[no_mangle]
pub static mut cacheblock: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut cachewidth: libc::c_int = 0;
#[no_mangle]
pub static mut d_viewbuffer: *mut pixel_t =
    0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut d_pzbuffer: *mut libc::c_short =
    0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut d_zrowbytes: libc::c_uint = 0;
#[no_mangle]
pub static mut d_zwidth: libc::c_uint = 0;
#[no_mangle]
pub static mut r_pcurrentvertbase: *mut mvertex_t =
    0 as *const mvertex_t as *mut mvertex_t;
//int                     c_surf;
#[no_mangle]
pub static mut r_surfsonstack: qboolean = false_0;
#[no_mangle]
pub static mut r_clipflags: libc::c_int = 0;
#[no_mangle]
pub static mut r_warpbuffer: [byte; 76800] = [0; 76800];
#[no_mangle]
pub static mut r_numallocatededges: libc::c_int = 0;
#[no_mangle]
pub static mut r_aliasuvscale: libc::c_float = 1.0f64 as libc::c_float;
unsafe extern "C" fn R_RankForRenderMode(mut rendermode: libc::c_int)
 -> libc::c_int {
    match rendermode {
        2 => {
            return 1 as libc::c_int
            // must be last!
        }
        5 => { return 2 as libc::c_int }
        3 => { return 3 as libc::c_int }
        _ => { }
    } // draw third
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_AllowFog(mut allowed: qboolean) { }
/*
===============
R_OpaqueEntity

Opaque entity can be brush or studio model but sprite
===============
*/
unsafe extern "C" fn R_OpaqueEntity(mut ent: *mut cl_entity_t) -> qboolean {
    let mut rendermode: libc::c_int = R_GetEntityRenderMode(ent);
    if rendermode == kRenderNormal as libc::c_int { return true_0 }
    if (*sw_notransbrushes).value != 0. && !(*ent).model.is_null() &&
           (*(*ent).model).type_0 as libc::c_int == mod_brush as libc::c_int
           && rendermode == kRenderTransTexture as libc::c_int {
        return true_0
    }
    if (*sw_noalphabrushes).value != 0. && !(*ent).model.is_null() &&
           (*(*ent).model).type_0 as libc::c_int == mod_brush as libc::c_int
           && rendermode == kRenderTransAlpha as libc::c_int {
        return true_0
    }
    return false_0;
}
/*
===============
R_TransEntityCompare

Sorting translucent entities by rendermode then by distance
===============
*/
unsafe extern "C" fn R_TransEntityCompare(mut a: *mut *const cl_entity_t,
                                          mut b: *mut *const cl_entity_t)
 -> libc::c_int {
    let mut ent1: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut ent2: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut vecLen: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut rendermode1: libc::c_int = 0;
    let mut rendermode2: libc::c_int = 0;
    ent1 = *a as *mut cl_entity_t;
    ent2 = *b as *mut cl_entity_t;
    rendermode1 = R_GetEntityRenderMode(ent1);
    rendermode2 = R_GetEntityRenderMode(ent2);
    // sort by distance
    if !(*ent1).model.is_null() &&
           (*(*ent1).model).type_0 as libc::c_int != mod_brush as libc::c_int
           || rendermode1 != kRenderTransAlpha as libc::c_int {
        org[0 as libc::c_int as usize] =
            ((*(*ent1).model).mins[0 as libc::c_int as usize] +
                 (*(*ent1).model).maxs[0 as libc::c_int as usize]) * 0.5f32;
        org[1 as libc::c_int as usize] =
            ((*(*ent1).model).mins[1 as libc::c_int as usize] +
                 (*(*ent1).model).maxs[1 as libc::c_int as usize]) * 0.5f32;
        org[2 as libc::c_int as usize] =
            ((*(*ent1).model).mins[2 as libc::c_int as usize] +
                 (*(*ent1).model).maxs[2 as libc::c_int as usize]) * 0.5f32;
        org[0 as libc::c_int as usize] =
            (*ent1).origin[0 as libc::c_int as usize] +
                org[0 as libc::c_int as usize];
        org[1 as libc::c_int as usize] =
            (*ent1).origin[1 as libc::c_int as usize] +
                org[1 as libc::c_int as usize];
        org[2 as libc::c_int as usize] =
            (*ent1).origin[2 as libc::c_int as usize] +
                org[2 as libc::c_int as usize];
        vecLen[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize] -
                org[0 as libc::c_int as usize];
        vecLen[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize] -
                org[1 as libc::c_int as usize];
        vecLen[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize] -
                org[2 as libc::c_int as usize];
        dist1 =
            vecLen[0 as libc::c_int as usize] *
                vecLen[0 as libc::c_int as usize] +
                vecLen[1 as libc::c_int as usize] *
                    vecLen[1 as libc::c_int as usize] +
                vecLen[2 as libc::c_int as usize] *
                    vecLen[2 as libc::c_int as usize]
    } else { dist1 = 1000000000 as libc::c_int as libc::c_float }
    if !(*ent1).model.is_null() &&
           (*(*ent2).model).type_0 as libc::c_int != mod_brush as libc::c_int
           || rendermode2 != kRenderTransAlpha as libc::c_int {
        org[0 as libc::c_int as usize] =
            ((*(*ent2).model).mins[0 as libc::c_int as usize] +
                 (*(*ent2).model).maxs[0 as libc::c_int as usize]) * 0.5f32;
        org[1 as libc::c_int as usize] =
            ((*(*ent2).model).mins[1 as libc::c_int as usize] +
                 (*(*ent2).model).maxs[1 as libc::c_int as usize]) * 0.5f32;
        org[2 as libc::c_int as usize] =
            ((*(*ent2).model).mins[2 as libc::c_int as usize] +
                 (*(*ent2).model).maxs[2 as libc::c_int as usize]) * 0.5f32;
        org[0 as libc::c_int as usize] =
            (*ent2).origin[0 as libc::c_int as usize] +
                org[0 as libc::c_int as usize];
        org[1 as libc::c_int as usize] =
            (*ent2).origin[1 as libc::c_int as usize] +
                org[1 as libc::c_int as usize];
        org[2 as libc::c_int as usize] =
            (*ent2).origin[2 as libc::c_int as usize] +
                org[2 as libc::c_int as usize];
        vecLen[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize] -
                org[0 as libc::c_int as usize];
        vecLen[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize] -
                org[1 as libc::c_int as usize];
        vecLen[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize] -
                org[2 as libc::c_int as usize];
        dist2 =
            vecLen[0 as libc::c_int as usize] *
                vecLen[0 as libc::c_int as usize] +
                vecLen[1 as libc::c_int as usize] *
                    vecLen[1 as libc::c_int as usize] +
                vecLen[2 as libc::c_int as usize] *
                    vecLen[2 as libc::c_int as usize]
    } else { dist2 = 1000000000 as libc::c_int as libc::c_float }
    if dist1 > dist2 { return -(1 as libc::c_int) }
    if dist1 < dist2 { return 1 as libc::c_int }
    // then sort by rendermode
    if R_RankForRenderMode(rendermode1) > R_RankForRenderMode(rendermode2) {
        return 1 as libc::c_int
    }
    if R_RankForRenderMode(rendermode1) < R_RankForRenderMode(rendermode2) {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
===============
R_WorldToScreen

Convert a given point from world into screen space
Returns true if we behind to screen
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_WorldToScreen(mut point: *const vec_t,
                                         mut screen: *mut vec_t)
 -> libc::c_int {
    let mut worldToScreen: matrix4x4 =
        [[0.; 4]; 4]; // just so we have something valid here
    let mut behind: qboolean = false_0;
    let mut w: libc::c_float = 0.;
    if point.is_null() || screen.is_null() { return true_0 as libc::c_int }
    memcpy(worldToScreen.as_mut_ptr() as *mut libc::c_void,
           RI.worldviewProjectionMatrix.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    *screen.offset(0 as libc::c_int as isize) =
        worldToScreen[0 as libc::c_int as usize][0 as libc::c_int as usize] *
            *point.offset(0 as libc::c_int as isize) +
            worldToScreen[0 as libc::c_int as
                              usize][1 as libc::c_int as usize] *
                *point.offset(1 as libc::c_int as isize) +
            worldToScreen[0 as libc::c_int as
                              usize][2 as libc::c_int as usize] *
                *point.offset(2 as libc::c_int as isize) +
            worldToScreen[0 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *screen.offset(1 as libc::c_int as isize) =
        worldToScreen[1 as libc::c_int as usize][0 as libc::c_int as usize] *
            *point.offset(0 as libc::c_int as isize) +
            worldToScreen[1 as libc::c_int as
                              usize][1 as libc::c_int as usize] *
                *point.offset(1 as libc::c_int as isize) +
            worldToScreen[1 as libc::c_int as
                              usize][2 as libc::c_int as usize] *
                *point.offset(2 as libc::c_int as isize) +
            worldToScreen[1 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    w =
        worldToScreen[3 as libc::c_int as usize][0 as libc::c_int as usize] *
            *point.offset(0 as libc::c_int as isize) +
            worldToScreen[3 as libc::c_int as
                              usize][1 as libc::c_int as usize] *
                *point.offset(1 as libc::c_int as isize) +
            worldToScreen[3 as libc::c_int as
                              usize][2 as libc::c_int as usize] *
                *point.offset(2 as libc::c_int as isize) +
            worldToScreen[3 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *screen.offset(2 as libc::c_int as isize) = 0.0f32;
    if w < 0.001f32 {
        let ref mut fresh0 = *screen.offset(0 as libc::c_int as isize);
        *fresh0 *= 100000 as libc::c_int as libc::c_float;
        let ref mut fresh1 = *screen.offset(1 as libc::c_int as isize);
        *fresh1 *= 100000 as libc::c_int as libc::c_float;
        behind = true_0
    } else {
        let mut invw: libc::c_float = 1.0f32 / w;
        let ref mut fresh2 = *screen.offset(0 as libc::c_int as isize);
        *fresh2 *= invw;
        let ref mut fresh3 = *screen.offset(1 as libc::c_int as isize);
        *fresh3 *= invw;
        behind = false_0
    }
    return behind as libc::c_int;
}
/*
===============
R_ScreenToWorld

Convert a given point from screen into world space
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ScreenToWorld(mut screen: *const vec_t,
                                         mut point: *mut vec_t) {
    let mut screenToWorld: matrix4x4 = [[0.; 4]; 4];
    let mut w: libc::c_float = 0.;
    if point.is_null() || screen.is_null() { return }
    Matrix4x4_Invert_Full(screenToWorld.as_mut_ptr(),
                          RI.worldviewProjectionMatrix.as_mut_ptr() as
                              *const [vec_t; 4]);
    *point.offset(0 as libc::c_int as isize) =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[0 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[0 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[0 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[0 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *point.offset(1 as libc::c_int as isize) =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[1 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[1 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[1 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[1 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *point.offset(2 as libc::c_int as isize) =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[2 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[2 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[2 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[2 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    w =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[3 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[3 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[3 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[3 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    if w != 0.0f32 {
        *point.offset(0 as libc::c_int as isize) =
            *point.offset(0 as libc::c_int as isize) * (1.0f32 / w);
        *point.offset(1 as libc::c_int as isize) =
            *point.offset(1 as libc::c_int as isize) * (1.0f32 / w);
        *point.offset(2 as libc::c_int as isize) =
            *point.offset(2 as libc::c_int as isize) * (1.0f32 / w)
    };
}
/*
===============
R_PushScene
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_PushScene() {
    tr.draw_stack_pos += 1;
    if tr.draw_stack_pos >= 2 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"draw stack overflow\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    tr.draw_list =
        &mut *tr.draw_stack.as_mut_ptr().offset(tr.draw_stack_pos as isize) as
            *mut draw_list_t;
}
/*
===============
R_PopScene
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_PopScene() {
    tr.draw_stack_pos -= 1;
    if tr.draw_stack_pos < 0 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"draw stack underflow\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    tr.draw_list =
        &mut *tr.draw_stack.as_mut_ptr().offset(tr.draw_stack_pos as isize) as
            *mut draw_list_t;
}
/*
===============
R_ClearScene
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ClearScene() {
    (*tr.draw_list).num_solid_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_trans_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_beam_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_edge_entities = 0 as libc::c_int as uint;
    // clear the scene befor start new frame
    if (*gEngfuncs.drawFuncs).R_ClearScene.is_some() {
        (*gEngfuncs.drawFuncs).R_ClearScene.expect("non-null function pointer")();
    };
}
/*
===============
R_AddEntity
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AddEntity(mut clent: *mut cl_entity_s,
                                     mut type_0: libc::c_int) -> qboolean {
    if (*r_drawentities).value == 0. {
        return false_0
    } // not allow to drawing
    if clent.is_null() || (*clent).model.is_null() {
        return false_0
    } // if set to invisible, skip
    if (*clent).curstate.effects & 128 as libc::c_int != 0 {
        return false_0
    } // done
    if !((*clent).curstate.rendermode == kRenderNormal as libc::c_int) &&
           CL_FxBlend(clent) <= 0 as libc::c_int {
        return true_0
    } // invisible
    if type_0 == 4 as libc::c_int {
        r_stats.c_client_ents = r_stats.c_client_ents.wrapping_add(1)
    }
    if R_OpaqueEntity(clent) as u64 != 0 {
        if (*(*clent).model).type_0 as libc::c_int == mod_brush as libc::c_int
           {
            if (*tr.draw_list).num_edge_entities >=
                   ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
                return false_0
            }
            (*tr.draw_list).edge_entities[(*tr.draw_list).num_edge_entities as
                                              usize] = clent;
            (*tr.draw_list).num_edge_entities =
                (*tr.draw_list).num_edge_entities.wrapping_add(1);
            return true_0
        }
        // opaque
        if (*tr.draw_list).num_solid_entities >=
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
            return false_0
        }
        (*tr.draw_list).solid_entities[(*tr.draw_list).num_solid_entities as
                                           usize] = clent;
        (*tr.draw_list).num_solid_entities =
            (*tr.draw_list).num_solid_entities.wrapping_add(1)
    } else {
        // translucent
        if (*tr.draw_list).num_trans_entities >=
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
            return false_0
        }
        (*tr.draw_list).trans_entities[(*tr.draw_list).num_trans_entities as
                                           usize] = clent;
        (*tr.draw_list).num_trans_entities =
            (*tr.draw_list).num_trans_entities.wrapping_add(1)
    }
    return true_0;
}
//=============================================================================
/*
===============
R_GetFarClip
===============
*/
unsafe extern "C" fn R_GetFarClip() -> libc::c_float {
    if !gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                             libc::c_int).is_null()
           && RI.drawWorld as libc::c_uint != 0 {
        return (*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).zmax
                   * 1.73f32
    }
    return 2048.0f32;
}
/*
===============
R_SetupFrustum
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupFrustum() {
    //ref_overview_t	*ov = gEngfuncs.GetOverviewParms();
    /*if( RP_NORMALPASS() && ( ENGINE_GET_PARM( PARM_WATER_LEVEL ) >= 3 ))
	{
		RI.fov_x = atan( tan( DEG2RAD( RI.fov_x ) / 2 ) * ( 0.97 + sin( gpGlobals->time * 1.5 ) * 0.03 )) * 2 / (M_PI / 180.0);
		RI.fov_y = atan( tan( DEG2RAD( RI.fov_y ) / 2 ) * ( 1.03 - sin( gpGlobals->time * 1.5 ) * 0.03 )) * 2 / (M_PI / 180.0);
	}*/
    // build the transformation matrix for the given view angles
    AngleVectors(RI.viewangles.as_mut_ptr() as *const vec_t,
                 RI.vforward.as_mut_ptr(), RI.vright.as_mut_ptr(),
                 RI.vup.as_mut_ptr());
    //if( !r_lockfrustum->value )
    RI.cullorigin[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize];
    RI.cullorigin[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize];
    RI.cullorigin[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize];
    RI.cull_vforward[0 as libc::c_int as usize] =
        RI.vforward[0 as libc::c_int as usize];
    RI.cull_vforward[1 as libc::c_int as usize] =
        RI.vforward[1 as libc::c_int as usize];
    RI.cull_vforward[2 as libc::c_int as usize] =
        RI.vforward[2 as libc::c_int as usize];
    RI.cull_vright[0 as libc::c_int as usize] =
        RI.vright[0 as libc::c_int as usize];
    RI.cull_vright[1 as libc::c_int as usize] =
        RI.vright[1 as libc::c_int as usize];
    RI.cull_vright[2 as libc::c_int as usize] =
        RI.vright[2 as libc::c_int as usize];
    RI.cull_vup[0 as libc::c_int as usize] =
        RI.vup[0 as libc::c_int as usize];
    RI.cull_vup[1 as libc::c_int as usize] =
        RI.vup[1 as libc::c_int as usize];
    RI.cull_vup[2 as libc::c_int as usize] =
        RI.vup[2 as libc::c_int as usize];
    //	if( RI.drawOrtho )
//		GL_FrustumInitOrtho( &RI.frustum, ov->xLeft, ov->xRight, ov->yTop, ov->yBottom, ov->zNear, ov->zFar );
//	else GL_FrustumInitProj( &RI.frustum, 0.0f, R_GetFarClip(), RI.fov_x, RI.fov_y ); // NOTE: we ignore nearplane here (mirrors only)
}
/*
=============
R_SetupProjectionMatrix
=============
*/
unsafe extern "C" fn R_SetupProjectionMatrix(mut m: *mut [vec_t; 4]) {
    let mut xMin: libc::c_float = 0.;
    let mut xMax: libc::c_float = 0.;
    let mut yMin: libc::c_float = 0.;
    let mut yMax: libc::c_float = 0.;
    let mut zNear: libc::c_float = 0.;
    let mut zFar: libc::c_float = 0.;
    if RI.drawOrtho as u64 != 0 {
        let mut ov: *const ref_overview_t =
            gEngfuncs.GetOverviewParms.expect("non-null function pointer")();
        Matrix4x4_CreateOrtho(m, (*ov).xLeft, (*ov).xRight, (*ov).yTop,
                              (*ov).yBottom, (*ov).zNear, (*ov).zFar);
        return
    }
    RI.farClip = R_GetFarClip();
    zNear = 4.0f32;
    zFar = if 256.0f32 > RI.farClip { 256.0f32 } else { RI.farClip };
    yMax =
        zNear *
            __tg_tan(RI.fov_y * 3.14159265358979323846f64 as libc::c_float /
                         360.0f32);
    yMin = -yMax;
    xMax =
        zNear *
            __tg_tan(RI.fov_x * 3.14159265358979323846f64 as libc::c_float /
                         360.0f32);
    xMin = -xMax;
    Matrix4x4_CreateProjection(m, xMax, xMin, yMax, yMin, zNear, zFar);
}
/*
=============
R_SetupModelviewMatrix
=============
*/
unsafe extern "C" fn R_SetupModelviewMatrix(mut m: *mut [vec_t; 4]) {
    Matrix4x4_CreateModelview(m);
    Matrix4x4_ConcatRotate(m, -RI.viewangles[2 as libc::c_int as usize],
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float);
    Matrix4x4_ConcatRotate(m, -RI.viewangles[0 as libc::c_int as usize],
                           0 as libc::c_int as libc::c_float,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float);
    Matrix4x4_ConcatRotate(m, -RI.viewangles[1 as libc::c_int as usize],
                           0 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float,
                           1 as libc::c_int as libc::c_float);
    Matrix4x4_ConcatTranslate(m, -RI.vieworg[0 as libc::c_int as usize],
                              -RI.vieworg[1 as libc::c_int as usize],
                              -RI.vieworg[2 as libc::c_int as usize]);
}
/*
=============
R_LoadIdentity
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_LoadIdentity() { }
/*
=============
R_RotateForEntity
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RotateForEntity(mut e: *mut cl_entity_t) { }
/*
=============
R_TranslateForEntity
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TranslateForEntity(mut e: *mut cl_entity_t) { }
/*
===============
R_FindViewLeaf
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FindViewLeaf() {
    RI.oldviewleaf = RI.viewleaf;
    RI.viewleaf =
        gEngfuncs.Mod_PointInLeaf.expect("non-null function pointer")(RI.pvsorigin.as_mut_ptr()
                                                                          as
                                                                          *const vec_t,
                                                                      (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                             as
                                                                                                                                             libc::c_int)).nodes);
}
/*
===============
R_SetupFrame
===============
*/
unsafe extern "C" fn R_SetupFrame() {
    // setup viewplane dist
    RI.viewplanedist =
        RI.vieworg[0 as libc::c_int as usize] *
            RI.vforward[0 as libc::c_int as usize] +
            RI.vieworg[1 as libc::c_int as usize] *
                RI.vforward[1 as libc::c_int as usize] +
            RI.vieworg[2 as libc::c_int as usize] *
                RI.vforward[2 as libc::c_int as usize];
    //	if( !gl_nosort->value )
    // sort translucents entities by rendermode and distance
    qsort((*tr.draw_list).trans_entities.as_mut_ptr() as *mut libc::c_void,
          (*tr.draw_list).num_trans_entities as size_t,
          ::std::mem::size_of::<*mut cl_entity_t>() as libc::c_ulong,
          ::std::mem::transmute::<*mut libc::c_void,
                                  __compar_fn_t>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                         *mut *const cl_entity_t,
                                                                                                     _:
                                                                                                         *mut *const cl_entity_t)
                                                                                    ->
                                                                                        libc::c_int>,
                                                                         *mut libc::c_void>(Some(R_TransEntityCompare
                                                                                                     as
                                                                                                     unsafe extern "C" fn(_:
                                                                                                                              *mut *const cl_entity_t,
                                                                                                                          _:
                                                                                                                              *mut *const cl_entity_t)
                                                                                                         ->
                                                                                                             libc::c_int))));
    // current viewleaf
    if RI.drawWorld as u64 != 0 {
        RI.isSkyVisible = false_0; // unknown at this moment
        R_FindViewLeaf();
    }
    // setup twice until globals fully refactored
    R_SetupFrameQ();
}
/*
=============
R_DrawEntitiesOnList
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawEntitiesOnList() {
    extern "C" {
        #[no_mangle]
        static mut d_pdrawspans:
               Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    }
    let mut i: libc::c_int = 0;
    //extern int d_aflatcolor;
	//d_aflatcolor = 0;
    tr.blend = 1.0f32;
    //	GL_CheckForErrors();
	//RI.currententity = gEngfuncs.GetEntityByIndex(0);
    d_pdrawspans =
        Some(R_PolysetFillSpans8 as
                 unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    GL_SetRenderMode(kRenderNormal as libc::c_int);
    // first draw solid entities
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_solid_entities &&
              RI.onlyClientDraw as u64 == 0 {
        RI.currententity = (*tr.draw_list).solid_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        //d_aflatcolor += 500;
        if RI.currententity.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     825 as
                                                                         libc::c_int);
        }
        if RI.currentmodel.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     826 as
                                                                         libc::c_int);
        }
        match (*RI.currentmodel).type_0 as libc::c_int {
            0 => { R_DrawBrushModel(RI.currententity); }
            3 => {
                R_SetUpWorldTransform();
                R_DrawStudioModel(RI.currententity);
            }
            2 | _ => { }
        }
        i += 1
    }
    //	GL_CheckForErrors();
    // quake-specific feature
//	R_DrawAlphaTextureChains();
    //	GL_CheckForErrors();
    R_SetUpWorldTransform();
    // draw sprites seperately, because of alpha blending
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_solid_entities &&
              RI.onlyClientDraw as u64 == 0 {
        RI.currententity = (*tr.draw_list).solid_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        if RI.currententity.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     880 as
                                                                         libc::c_int);
        }
        if RI.currentmodel.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     881 as
                                                                         libc::c_int);
        }
        match (*RI.currentmodel).type_0 as libc::c_int {
            1 => { R_DrawSpriteModel(RI.currententity); }
            _ => { }
        }
        i += 1
    }
    //	GL_CheckForErrors();
    if RI.onlyClientDraw as u64 == 0 {
        gEngfuncs.CL_DrawEFX.expect("non-null function pointer")(tr.frametime
                                                                     as
                                                                     libc::c_float,
                                                                 false_0);
    }
    //	GL_CheckForErrors();
    if RI.drawWorld as u64 != 0 {
        gEngfuncs.pfnDrawNormalTriangles.expect("non-null function pointer")();
    }
    //	GL_CheckForErrors();
    d_pdrawspans =
        Some(R_PolysetDrawSpans8_33 as
                 unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    // then draw translucent entities
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_trans_entities &&
              RI.onlyClientDraw as u64 == 0 {
        RI.currententity = (*tr.draw_list).trans_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        // handle studiomodels with custom rendermodes on texture
        if (*RI.currententity).curstate.rendermode !=
               kRenderNormal as libc::c_int {
            tr.blend =
                CL_FxBlend(RI.currententity) as libc::c_float / 255.0f32
        } else { tr.blend = 1.0f32 } // draw as solid but sorted by distance
        if !(tr.blend <= 0.0f32) {
            if RI.currententity.is_null() {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"../ref_soft/r_main.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         918
                                                                             as
                                                                             libc::c_int);
            }
            if RI.currentmodel.is_null() {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"../ref_soft/r_main.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         919
                                                                             as
                                                                             libc::c_int);
            }
            match (*RI.currentmodel).type_0 as libc::c_int {
                0 => { R_DrawBrushModel(RI.currententity); }
                3 => {
                    R_SetUpWorldTransform();
                    R_DrawStudioModel(RI.currententity);
                }
                1 => {
                    R_SetUpWorldTransform();
                    R_DrawSpriteModel(RI.currententity);
                }
                2 | _ => { }
            }
        }
        i += 1
    }
    //	GL_CheckForErrors();
    if RI.drawWorld as u64 != 0 {
        //	pglTexEnvi( GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE );
        gEngfuncs.pfnDrawTransparentTriangles.expect("non-null function pointer")();
    }
    //	GL_CheckForErrors();
    if RI.onlyClientDraw as u64 == 0 {
        R_AllowFog(false_0);
        gEngfuncs.CL_DrawEFX.expect("non-null function pointer")(tr.frametime
                                                                     as
                                                                     libc::c_float,
                                                                 true_0);
        R_AllowFog(true_0);
    }
    //GL_CheckForErrors();
    //	pglDisable( GL_BLEND );	// Trinity Render issues
    GL_SetRenderMode(kRenderNormal as libc::c_int);
    R_SetUpWorldTransform();
    if RI.onlyClientDraw as u64 == 0 { R_DrawViewModel(); }
    gEngfuncs.CL_ExtraUpdate.expect("non-null function pointer")();
    //GL_CheckForErrors();
}
#[no_mangle]
pub static mut insubmodel: qboolean = false_0;
/*
=============
R_BmodelCheckBBox
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BmodelCheckBBox(mut minmaxs: *mut libc::c_float)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pindex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut clipflags: libc::c_int = 0;
    let mut acceptpt: vec3_t = [0.; 3];
    let mut rejectpt: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    clipflags = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        // generate accept and reject points
	// FIXME: do with fast look-ups or integer tests based on the sign bit
	// of the floating point values
        pindex = qfrustum.pfrustum_indexes[i as usize];
        rejectpt[0 as libc::c_int as usize] =
            *minmaxs.offset(*pindex.offset(0 as libc::c_int as isize) as
                                isize);
        rejectpt[1 as libc::c_int as usize] =
            *minmaxs.offset(*pindex.offset(1 as libc::c_int as isize) as
                                isize);
        rejectpt[2 as libc::c_int as usize] =
            *minmaxs.offset(*pindex.offset(2 as libc::c_int as isize) as
                                isize);
        d =
            rejectpt[0 as libc::c_int as usize] *
                qfrustum.view_clipplanes[i as
                                             usize].normal[0 as libc::c_int as
                                                               usize] +
                rejectpt[1 as libc::c_int as usize] *
                    qfrustum.view_clipplanes[i as
                                                 usize].normal[1 as
                                                                   libc::c_int
                                                                   as usize] +
                rejectpt[2 as libc::c_int as usize] *
                    qfrustum.view_clipplanes[i as
                                                 usize].normal[2 as
                                                                   libc::c_int
                                                                   as usize];
        d -= qfrustum.view_clipplanes[i as usize].dist;
        if d <= 0 as libc::c_int as libc::c_float {
            return 0x10 as libc::c_int
        }
        acceptpt[0 as libc::c_int as usize] =
            *minmaxs.offset(*pindex.offset((3 as libc::c_int +
                                                0 as libc::c_int) as isize) as
                                isize);
        acceptpt[1 as libc::c_int as usize] =
            *minmaxs.offset(*pindex.offset((3 as libc::c_int +
                                                1 as libc::c_int) as isize) as
                                isize);
        acceptpt[2 as libc::c_int as usize] =
            *minmaxs.offset(*pindex.offset((3 as libc::c_int +
                                                2 as libc::c_int) as isize) as
                                isize);
        d =
            acceptpt[0 as libc::c_int as usize] *
                qfrustum.view_clipplanes[i as
                                             usize].normal[0 as libc::c_int as
                                                               usize] +
                acceptpt[1 as libc::c_int as usize] *
                    qfrustum.view_clipplanes[i as
                                                 usize].normal[1 as
                                                                   libc::c_int
                                                                   as usize] +
                acceptpt[2 as libc::c_int as usize] *
                    qfrustum.view_clipplanes[i as
                                                 usize].normal[2 as
                                                                   libc::c_int
                                                                   as usize];
        d -= qfrustum.view_clipplanes[i as usize].dist;
        if d <= 0 as libc::c_int as libc::c_float {
            clipflags |= (1 as libc::c_int) << i
        }
        i += 1
    }
    return clipflags;
}
/*
===================
R_FindTopNode
===================
*/
#[no_mangle]
pub unsafe extern "C" fn R_FindTopnode(mut mins: *mut vec_t,
                                       mut maxs: *mut vec_t) -> *mut mnode_t {
    let mut splitplane: *mut mplane_t =
        0 as *mut mplane_t; // not visible at all
    let mut sides: libc::c_int = 0; // we've reached a non-solid leaf, so it's
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    node =
        (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                               as
                                                                               libc::c_int)).nodes;
    loop  {
        if (*node).visframe != tr.visframecount { return 0 as *mut mnode_t }
        if (*node).contents < 0 as libc::c_int {
            if (*node).contents != -(2 as libc::c_int) { return node }
            // in solid, so not visible
            return 0 as *mut mnode_t
        }
        splitplane = (*node).plane;
        sides =
            if ((*splitplane).type_0 as libc::c_int) < 3 as libc::c_int {
                if (*splitplane).dist <=
                       *mins.offset((*splitplane).type_0 as isize) {
                    1 as libc::c_int
                } else if (*splitplane).dist >=
                              *maxs.offset((*splitplane).type_0 as isize) {
                    2 as libc::c_int
                } else { 3 as libc::c_int }
            } else {
                BoxOnPlaneSide(mins as *const vec_t, maxs as *const vec_t,
                               splitplane)
            };
        //  visible and not BSP clipped
        if sides == 3 as libc::c_int { return node } // this is the splitter
        // not split yet; recurse down the contacted side
        if sides & 1 as libc::c_int != 0 {
            node = (*node).children[0 as libc::c_int as usize]
        } else { node = (*node).children[1 as libc::c_int as usize] }
    };
}
/*
=============
RotatedBBox

Returns an axially aligned box that contains the input box at the given rotation
=============
*/
#[no_mangle]
pub unsafe extern "C" fn RotatedBBox(mut mins: *mut vec_t,
                                     mut maxs: *mut vec_t,
                                     mut angles: *mut vec_t,
                                     mut tmins: *mut vec_t,
                                     mut tmaxs: *mut vec_t) {
    let mut tmp: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    if *angles.offset(0 as libc::c_int as isize) == 0. &&
           *angles.offset(1 as libc::c_int as isize) == 0. &&
           *angles.offset(2 as libc::c_int as isize) == 0. {
        *tmins.offset(0 as libc::c_int as isize) =
            *mins.offset(0 as libc::c_int as isize);
        *tmins.offset(1 as libc::c_int as isize) =
            *mins.offset(1 as libc::c_int as isize);
        *tmins.offset(2 as libc::c_int as isize) =
            *mins.offset(2 as libc::c_int as isize);
        *tmaxs.offset(0 as libc::c_int as isize) =
            *maxs.offset(0 as libc::c_int as isize);
        *tmaxs.offset(1 as libc::c_int as isize) =
            *maxs.offset(1 as libc::c_int as isize);
        *tmaxs.offset(2 as libc::c_int as isize) =
            *maxs.offset(2 as libc::c_int as isize);
        return
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *tmins.offset(i as isize) = 99999 as libc::c_int as vec_t;
        *tmaxs.offset(i as isize) = -(99999 as libc::c_int) as vec_t;
        i += 1
    }
    AngleVectors(angles as *const vec_t, forward.as_mut_ptr(),
                 right.as_mut_ptr(), up.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if i & 1 as libc::c_int != 0 {
            tmp[0 as libc::c_int as usize] =
                *mins.offset(0 as libc::c_int as isize)
        } else {
            tmp[0 as libc::c_int as usize] =
                *maxs.offset(0 as libc::c_int as isize)
        }
        if i & 2 as libc::c_int != 0 {
            tmp[1 as libc::c_int as usize] =
                *mins.offset(1 as libc::c_int as isize)
        } else {
            tmp[1 as libc::c_int as usize] =
                *maxs.offset(1 as libc::c_int as isize)
        }
        if i & 4 as libc::c_int != 0 {
            tmp[2 as libc::c_int as usize] =
                *mins.offset(2 as libc::c_int as isize)
        } else {
            tmp[2 as libc::c_int as usize] =
                *maxs.offset(2 as libc::c_int as isize)
        }
        v[0 as libc::c_int as usize] =
            forward[0 as libc::c_int as usize] *
                tmp[0 as libc::c_int as usize];
        v[1 as libc::c_int as usize] =
            forward[1 as libc::c_int as usize] *
                tmp[0 as libc::c_int as usize];
        v[2 as libc::c_int as usize] =
            forward[2 as libc::c_int as usize] *
                tmp[0 as libc::c_int as usize];
        v[0 as libc::c_int as usize] =
            v[0 as libc::c_int as usize] +
                -tmp[1 as libc::c_int as usize] *
                    right[0 as libc::c_int as usize];
        v[1 as libc::c_int as usize] =
            v[1 as libc::c_int as usize] +
                -tmp[1 as libc::c_int as usize] *
                    right[1 as libc::c_int as usize];
        v[2 as libc::c_int as usize] =
            v[2 as libc::c_int as usize] +
                -tmp[1 as libc::c_int as usize] *
                    right[2 as libc::c_int as usize];
        v[0 as libc::c_int as usize] =
            v[0 as libc::c_int as usize] +
                tmp[2 as libc::c_int as usize] *
                    up[0 as libc::c_int as usize];
        v[1 as libc::c_int as usize] =
            v[1 as libc::c_int as usize] +
                tmp[2 as libc::c_int as usize] *
                    up[1 as libc::c_int as usize];
        v[2 as libc::c_int as usize] =
            v[2 as libc::c_int as usize] +
                tmp[2 as libc::c_int as usize] *
                    up[2 as libc::c_int as usize];
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if v[j as usize] < *tmins.offset(j as isize) {
                *tmins.offset(j as isize) = v[j as usize]
            }
            if v[j as usize] > *tmaxs.offset(j as isize) {
                *tmaxs.offset(j as isize) = v[j as usize]
            }
            j += 1
        }
        i += 1
    };
}
/*
=============
R_DrawBEntitiesOnList
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawBEntitiesOnList() {
    let mut i: libc::c_int = 0;
    let mut clipflags: libc::c_int = 0;
    let mut oldorigin: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut minmaxs: [libc::c_float; 6] = [0.; 6];
    let mut topnode: *mut mnode_t = 0 as *mut mnode_t;
    oldorigin[0 as libc::c_int as usize] =
        tr.modelorg[0 as libc::c_int as usize];
    oldorigin[1 as libc::c_int as usize] =
        tr.modelorg[1 as libc::c_int as usize];
    oldorigin[2 as libc::c_int as usize] =
        tr.modelorg[2 as libc::c_int as usize];
    insubmodel = true_0;
    //r_dlightframecount = r_framecount;
    i = 0 as libc::c_int; // clip brush only
    while (i as libc::c_uint) < (*tr.draw_list).num_edge_entities &&
              RI.onlyClientDraw as u64 == 0 {
        let mut k: libc::c_int = 0;
        RI.currententity = (*tr.draw_list).edge_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        if !RI.currentmodel.is_null() {
            if !((*RI.currentmodel).nummodelsurfaces == 0 as libc::c_int) {
                //if ( currententity->flags & RF_BEAM )
			//continue;
                if !((*RI.currentmodel).type_0 as libc::c_int !=
                         mod_brush as libc::c_int) {
                    // see if the bounding box lets us trivially reject, also sets
	// trivial accept status
                    RotatedBBox((*RI.currentmodel).mins.as_mut_ptr(),
                                (*RI.currentmodel).maxs.as_mut_ptr(),
                                (*RI.currententity).angles.as_mut_ptr(),
                                mins.as_mut_ptr(),
                                maxs.as_mut_ptr()); // off the edge of the screen
                    minmaxs[0 as libc::c_int as usize] =
                        mins[0 as libc::c_int as usize] +
                            (*RI.currententity).origin[0 as libc::c_int as
                                                           usize];
                    minmaxs[1 as libc::c_int as usize] =
                        mins[1 as libc::c_int as usize] +
                            (*RI.currententity).origin[1 as libc::c_int as
                                                           usize];
                    minmaxs[2 as libc::c_int as usize] =
                        mins[2 as libc::c_int as usize] +
                            (*RI.currententity).origin[2 as libc::c_int as
                                                           usize];
                    *minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                     isize).offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                        =
                        maxs[0 as libc::c_int as usize] +
                            (*RI.currententity).origin[0 as libc::c_int as
                                                           usize];
                    *minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                     isize).offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                        =
                        maxs[1 as libc::c_int as usize] +
                            (*RI.currententity).origin[1 as libc::c_int as
                                                           usize];
                    *minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                     isize).offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                        =
                        maxs[2 as libc::c_int as usize] +
                            (*RI.currententity).origin[2 as libc::c_int as
                                                           usize];
                    clipflags = R_BmodelCheckBBox(minmaxs.as_mut_ptr());
                    if !(clipflags == 0x10 as libc::c_int) {
                        //clipflags = 0;
                        topnode =
                            R_FindTopnode(minmaxs.as_mut_ptr(),
                                          minmaxs.as_mut_ptr().offset(3 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)); // no part in a visible leaf
                        if !topnode.is_null() {
                            r_entorigin[0 as libc::c_int as usize] =
                                (*RI.currententity).origin[0 as libc::c_int as
                                                               usize];
                            r_entorigin[1 as libc::c_int as usize] =
                                (*RI.currententity).origin[1 as libc::c_int as
                                                               usize];
                            r_entorigin[2 as libc::c_int as usize] =
                                (*RI.currententity).origin[2 as libc::c_int as
                                                               usize];
                            tr.modelorg[0 as libc::c_int as usize] =
                                RI.vieworg[0 as libc::c_int as usize] -
                                    r_entorigin[0 as libc::c_int as usize];
                            tr.modelorg[1 as libc::c_int as usize] =
                                RI.vieworg[1 as libc::c_int as usize] -
                                    r_entorigin[1 as libc::c_int as usize];
                            tr.modelorg[2 as libc::c_int as usize] =
                                RI.vieworg[2 as libc::c_int as usize] -
                                    r_entorigin[2 as libc::c_int as usize];
                            //VectorSubtract (r_origin, RI.currententity->origin, modelorg);
                            r_pcurrentvertbase = (*RI.currentmodel).vertexes;
                            // FIXME: stop transforming twice
                            R_RotateBmodel();
                            // calculate dynamic lighting for bmodel
		// this will reset RI.currententity, do we need this?
		//R_PushDlights ();
		/*if (clmodel->firstmodelsurface != 0)
		{
				for (k=0 ; k<r_refdef2.numDlights ; k++)
				{
						R_MarkLights (&r_refdef2.dlights[k], 1<<k,
								clmodel->nodes + clmodel->firstnode);
				}
		}*/
                            // calculate dynamic lighting for bmodel
                            k = 0 as libc::c_int;
                            while k < 32 as libc::c_int {
                                let mut l: *mut dlight_t =
                                    gEngfuncs.GetDynamicLight.expect("non-null function pointer")(k);
                                let mut origin_l: vec3_t = [0.; 3];
                                let mut oldorigin_0: vec3_t = [0.; 3];
                                if !((*l).die < (*gpGlobals).time ||
                                         (*l).radius == 0.) {
                                    // restore lightorigin*/
                                    //R_MarkLights( l, 1<<k, RI.currentmodel->nodes + RI.currentmodel->hulls[0].firstclipnode );
                                    oldorigin_0[0 as libc::c_int as usize] =
                                        (*l).origin[0 as libc::c_int as
                                                        usize]; // save lightorigin
                                    oldorigin_0[1 as libc::c_int as usize] =
                                        (*l).origin[1 as libc::c_int as
                                                        usize]; // move light in bmodel space
                                    oldorigin_0[2 as libc::c_int as usize] =
                                        (*l).origin[2 as libc::c_int as
                                                        usize];
                                    Matrix4x4_CreateFromEntity(RI.objectMatrix.as_mut_ptr(),
                                                               (*RI.currententity).angles.as_mut_ptr()
                                                                   as
                                                                   *const vec_t,
                                                               (*RI.currententity).origin.as_mut_ptr()
                                                                   as
                                                                   *const vec_t,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_float);
                                    Matrix4x4_VectorITransform(RI.objectMatrix.as_mut_ptr()
                                                                   as
                                                                   *const [vec_t; 4],
                                                               (*l).origin.as_mut_ptr()
                                                                   as
                                                                   *const libc::c_float,
                                                               origin_l.as_mut_ptr());
                                    (*l).origin[0 as libc::c_int as usize] =
                                        origin_l[0 as libc::c_int as usize];
                                    (*l).origin[1 as libc::c_int as usize] =
                                        origin_l[1 as libc::c_int as usize];
                                    (*l).origin[2 as libc::c_int as usize] =
                                        origin_l[2 as libc::c_int as usize];
                                    R_MarkLights(l, (1 as libc::c_int) << k,
                                                 (*RI.currentmodel).nodes.offset((*RI.currentmodel).hulls[0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize].firstclipnode
                                                                                     as
                                                                                     isize));
                                    (*l).origin[0 as libc::c_int as usize] =
                                        oldorigin_0[0 as libc::c_int as
                                                        usize];
                                    (*l).origin[1 as libc::c_int as usize] =
                                        oldorigin_0[1 as libc::c_int as
                                                        usize];
                                    (*l).origin[2 as libc::c_int as usize] =
                                        oldorigin_0[2 as libc::c_int as usize]
                                }
                                k += 1
                            }
                            //	RI.currentmodel = tr.draw_list->solid_entities[i]->model;
	//	RI.currententity = tr.draw_list->solid_entities[i];
                            (*RI.currententity).topnode = topnode;
                            //ASSERT( RI.currentmodel == tr.draw_list->solid_entities[i]->model );
                            if (*topnode).contents >= 0 as libc::c_int {
                                // not a leaf; has to be clipped to the world BSP
                                r_clipflags = clipflags;
                                R_DrawSolidClippedSubmodelPolygons(RI.currentmodel,
                                                                   topnode);
                            } else {
                                // falls entirely in one leaf, so we just put all the
		// edges in the edge list and let 1/z sorting handle
		// drawing order
			//ASSERT( RI.currentmodel == tr.draw_list->solid_entities[i]->model );
                                R_DrawSubmodelPolygons(RI.currentmodel,
                                                       clipflags, topnode);
                            }
                            (*RI.currententity).topnode = 0 as *mut mnode_s;
                            // put back world rotation and frustum clipping
	// FIXME: R_RotateBmodel should just work off base_vxx
                            RI.vforward[0 as libc::c_int as usize] =
                                RI.base_vpn[0 as libc::c_int as usize];
                            RI.vforward[1 as libc::c_int as usize] =
                                RI.base_vpn[1 as libc::c_int as usize];
                            RI.vforward[2 as libc::c_int as usize] =
                                RI.base_vpn[2 as libc::c_int as usize];
                            RI.vup[0 as libc::c_int as usize] =
                                RI.base_vup[0 as libc::c_int as usize];
                            RI.vup[1 as libc::c_int as usize] =
                                RI.base_vup[1 as libc::c_int as usize];
                            RI.vup[2 as libc::c_int as usize] =
                                RI.base_vup[2 as libc::c_int as usize];
                            RI.vright[0 as libc::c_int as usize] =
                                RI.base_vright[0 as libc::c_int as usize];
                            RI.vright[1 as libc::c_int as usize] =
                                RI.base_vright[1 as libc::c_int as usize];
                            RI.vright[2 as libc::c_int as usize] =
                                RI.base_vright[2 as libc::c_int as usize];
                            tr.modelorg[0 as libc::c_int as usize] =
                                oldorigin[0 as libc::c_int as usize];
                            tr.modelorg[1 as libc::c_int as usize] =
                                oldorigin[1 as libc::c_int as usize];
                            tr.modelorg[2 as libc::c_int as usize] =
                                oldorigin[2 as libc::c_int as usize];
                            R_TransformFrustum();
                        }
                    }
                }
            }
        }
        i += 1
    }
    insubmodel = false_0;
}
/*
=============
R_DrawBEntitiesOnList
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawBrushModel(mut pent: *mut cl_entity_t) {
    let mut i: libc::c_int = 0;
    let mut clipflags: libc::c_int = 0;
    let mut oldorigin: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut minmaxs: [libc::c_float; 6] = [0.; 6];
    let mut topnode: *mut mnode_t = 0 as *mut mnode_t;
    let mut k: libc::c_int = 0;
    let mut ledges: [edge_t; 4001] =
        [edge_t{u: 0,
                u_step: 0,
                prev: 0 as *mut edge_s,
                next: 0 as *mut edge_s,
                surfs: [0; 2],
                nextremove: 0 as *mut edge_s,
                nearzi: 0.,
                owner: 0 as *mut medge_t,}; 4001];
    let mut lsurfs: [surf_t; 2001] =
        [surf_t{next: 0 as *mut surf_s,
                prev: 0 as *mut surf_s,
                spans: 0 as *mut espan_s,
                key: 0,
                last_u: 0,
                spanstate: 0,
                flags: 0,
                msurf: 0 as *mut msurface_t,
                entity: 0 as *mut cl_entity_t,
                nearzi: 0.,
                insubmodel: false_0,
                d_ziorigin: 0.,
                d_zistepu: 0.,
                d_zistepv: 0.,
                pad: [0; 2],}; 2001];
    if RI.drawWorld as u64 == 0 { return }
    if !auxedges.is_null() {
        r_edges = auxedges
    } else {
        r_edges =
            (&mut *ledges.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 *mut edge_t as libc::c_long +
                 32 as libc::c_int as libc::c_long -
                 1 as libc::c_int as libc::c_long &
                 !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as
                *mut edge_t
    }
    if r_surfsonstack as u64 != 0 {
        surfaces =
            (&mut *lsurfs.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 *mut surf_t as libc::c_long +
                 32 as libc::c_int as libc::c_long -
                 1 as libc::c_int as libc::c_long &
                 !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as
                *mut surf_t;
        surf_max = &mut *surfaces.offset(r_cnumsurfs as isize) as *mut surf_t;
        // surface 0 doesn't really exist; it's just a dummy because index 0
	// is used to indicate no edge attached to surface
        memset(&mut *surfaces.offset(0 as libc::c_int as isize) as *mut surf_t
                   as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<surf_t>() as libc::c_ulong);
        surfaces = surfaces.offset(-1);
        R_SurfacePatch();
    }
    R_BeginEdgeFrame();
    oldorigin[0 as libc::c_int as usize] =
        tr.modelorg[0 as libc::c_int as usize];
    oldorigin[1 as libc::c_int as usize] =
        tr.modelorg[1 as libc::c_int as usize];
    oldorigin[2 as libc::c_int as usize] =
        tr.modelorg[2 as libc::c_int as usize];
    insubmodel = true_0;
    //r_dlightframecount = r_framecount;
    if RI.currentmodel.is_null() { return } // clip brush only
    if (*RI.currentmodel).nummodelsurfaces == 0 as libc::c_int { return }
    //if ( currententity->flags & RF_BEAM )
			//continue;
    if (*RI.currentmodel).type_0 as libc::c_int != mod_brush as libc::c_int {
        return
    }
    // see if the bounding box lets us trivially reject, also sets
	// trivial accept status
    RotatedBBox((*RI.currentmodel).mins.as_mut_ptr(),
                (*RI.currentmodel).maxs.as_mut_ptr(),
                (*RI.currententity).angles.as_mut_ptr(), mins.as_mut_ptr(),
                maxs.as_mut_ptr()); // off the edge of the screen
    minmaxs[0 as libc::c_int as usize] =
        mins[0 as libc::c_int as usize] +
            (*RI.currententity).origin[0 as libc::c_int as usize];
    minmaxs[1 as libc::c_int as usize] =
        mins[1 as libc::c_int as usize] +
            (*RI.currententity).origin[1 as libc::c_int as usize];
    minmaxs[2 as libc::c_int as usize] =
        mins[2 as libc::c_int as usize] +
            (*RI.currententity).origin[2 as libc::c_int as usize];
    *minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                     isize).offset(0 as libc::c_int as isize)
        =
        maxs[0 as libc::c_int as usize] +
            (*RI.currententity).origin[0 as libc::c_int as usize];
    *minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                     isize).offset(1 as libc::c_int as isize)
        =
        maxs[1 as libc::c_int as usize] +
            (*RI.currententity).origin[1 as libc::c_int as usize];
    *minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                     isize).offset(2 as libc::c_int as isize)
        =
        maxs[2 as libc::c_int as usize] +
            (*RI.currententity).origin[2 as libc::c_int as usize];
    clipflags = R_BmodelCheckBBox(minmaxs.as_mut_ptr());
    if clipflags == 0x10 as libc::c_int { return }
    //clipflags = 0;
    topnode =
        R_FindTopnode(minmaxs.as_mut_ptr(),
                      minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                      isize)); // no part in a visible leaf
    if topnode.is_null() { return }
    alphaspans = true_0;
    r_entorigin[0 as libc::c_int as usize] =
        (*RI.currententity).origin[0 as libc::c_int as usize];
    r_entorigin[1 as libc::c_int as usize] =
        (*RI.currententity).origin[1 as libc::c_int as usize];
    r_entorigin[2 as libc::c_int as usize] =
        (*RI.currententity).origin[2 as libc::c_int as usize];
    tr.modelorg[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize] -
            r_entorigin[0 as libc::c_int as usize];
    tr.modelorg[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize] -
            r_entorigin[1 as libc::c_int as usize];
    tr.modelorg[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize] -
            r_entorigin[2 as libc::c_int as usize];
    //VectorSubtract (r_origin, RI.currententity->origin, modelorg);
    r_pcurrentvertbase = (*RI.currentmodel).vertexes;
    // FIXME: stop transforming twice
    R_RotateBmodel();
    // calculate dynamic lighting for bmodel
		// this will reset RI.currententity, do we need this?
		//R_PushDlights ();
		/*if (clmodel->firstmodelsurface != 0)
		{
				for (k=0 ; k<r_refdef2.numDlights ; k++)
				{
						R_MarkLights (&r_refdef2.dlights[k], 1<<k,
								clmodel->nodes + clmodel->firstnode);
				}
		}*/
    // calculate dynamic lighting for bmodel
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        let mut l: *mut dlight_t =
            gEngfuncs.GetDynamicLight.expect("non-null function pointer")(k);
        let mut origin_l: vec3_t = [0.; 3];
        let mut oldorigin_0: vec3_t = [0.; 3];
        if !((*l).die < (*gpGlobals).time || (*l).radius == 0.) {
            // restore lightorigin*/
            //R_MarkLights( l, 1<<k, RI.currentmodel->nodes + RI.currentmodel->hulls[0].firstclipnode );
            oldorigin_0[0 as libc::c_int as usize] =
                (*l).origin[0 as libc::c_int as usize]; // save lightorigin
            oldorigin_0[1 as libc::c_int as usize] =
                (*l).origin[1 as libc::c_int as
                                usize]; // move light in bmodel space
            oldorigin_0[2 as libc::c_int as usize] =
                (*l).origin[2 as libc::c_int as usize];
            Matrix4x4_CreateFromEntity(RI.objectMatrix.as_mut_ptr(),
                                       (*RI.currententity).angles.as_mut_ptr()
                                           as *const vec_t,
                                       (*RI.currententity).origin.as_mut_ptr()
                                           as *const vec_t,
                                       1 as libc::c_int as libc::c_float);
            Matrix4x4_VectorITransform(RI.objectMatrix.as_mut_ptr() as
                                           *const [vec_t; 4],
                                       (*l).origin.as_mut_ptr() as
                                           *const libc::c_float,
                                       origin_l.as_mut_ptr());
            tr.modelviewIdentity = false_0;
            (*l).origin[0 as libc::c_int as usize] =
                origin_l[0 as libc::c_int as usize];
            (*l).origin[1 as libc::c_int as usize] =
                origin_l[1 as libc::c_int as usize];
            (*l).origin[2 as libc::c_int as usize] =
                origin_l[2 as libc::c_int as usize];
            R_MarkLights(l, (1 as libc::c_int) << k,
                         (*RI.currentmodel).nodes.offset((*RI.currentmodel).hulls[0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      usize].firstclipnode
                                                             as isize));
            (*l).origin[0 as libc::c_int as usize] =
                oldorigin_0[0 as libc::c_int as usize];
            (*l).origin[1 as libc::c_int as usize] =
                oldorigin_0[1 as libc::c_int as usize];
            (*l).origin[2 as libc::c_int as usize] =
                oldorigin_0[2 as libc::c_int as usize]
        }
        k += 1
    }
    //	RI.currentmodel = tr.draw_list->solid_entities[i]->model;
	//	RI.currententity = tr.draw_list->solid_entities[i];
    (*RI.currententity).topnode = topnode;
    //ASSERT( RI.currentmodel == tr.draw_list->solid_entities[i]->model );
    if (*topnode).contents >= 0 as libc::c_int {
        // not a leaf; has to be clipped to the world BSP
        r_clipflags = clipflags;
        R_DrawSolidClippedSubmodelPolygons(RI.currentmodel, topnode);
    } else {
        // falls entirely in one leaf, so we just put all the
		// edges in the edge list and let 1/z sorting handle
		// drawing order
			//ASSERT( RI.currentmodel == tr.draw_list->solid_entities[i]->model );
        R_DrawSubmodelPolygons(RI.currentmodel, clipflags, topnode);
    }
    (*RI.currententity).topnode = 0 as *mut mnode_s;
    // put back world rotation and frustum clipping
	// FIXME: R_RotateBmodel should just work off base_vxx
    RI.vforward[0 as libc::c_int as usize] =
        RI.base_vpn[0 as libc::c_int as usize];
    RI.vforward[1 as libc::c_int as usize] =
        RI.base_vpn[1 as libc::c_int as usize];
    RI.vforward[2 as libc::c_int as usize] =
        RI.base_vpn[2 as libc::c_int as usize];
    RI.vup[0 as libc::c_int as usize] =
        RI.base_vup[0 as libc::c_int as usize];
    RI.vup[1 as libc::c_int as usize] =
        RI.base_vup[1 as libc::c_int as usize];
    RI.vup[2 as libc::c_int as usize] =
        RI.base_vup[2 as libc::c_int as usize];
    RI.vright[0 as libc::c_int as usize] =
        RI.base_vright[0 as libc::c_int as usize];
    RI.vright[1 as libc::c_int as usize] =
        RI.base_vright[1 as libc::c_int as usize];
    RI.vright[2 as libc::c_int as usize] =
        RI.base_vright[2 as libc::c_int as usize];
    tr.modelorg[0 as libc::c_int as usize] =
        oldorigin[0 as libc::c_int as usize];
    tr.modelorg[1 as libc::c_int as usize] =
        oldorigin[1 as libc::c_int as usize];
    tr.modelorg[2 as libc::c_int as usize] =
        oldorigin[2 as libc::c_int as usize];
    R_TransformFrustum();
    insubmodel = false_0;
    R_ScanEdges();
    alphaspans = false_0;
}
/*
================
R_EdgeDrawing
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_EdgeDrawing() {
    let mut ledges: [edge_t; 4001] =
        [edge_t{u: 0,
                u_step: 0,
                prev: 0 as *mut edge_s,
                next: 0 as *mut edge_s,
                surfs: [0; 2],
                nextremove: 0 as *mut edge_s,
                nearzi: 0.,
                owner: 0 as *mut medge_t,}; 4001];
    let mut lsurfs: [surf_t; 2001] =
        [surf_t{next: 0 as *mut surf_s,
                prev: 0 as *mut surf_s,
                spans: 0 as *mut espan_s,
                key: 0,
                last_u: 0,
                spanstate: 0,
                flags: 0,
                msurf: 0 as *mut msurface_t,
                entity: 0 as *mut cl_entity_t,
                nearzi: 0.,
                insubmodel: false_0,
                d_ziorigin: 0.,
                d_zistepu: 0.,
                d_zistepv: 0.,
                pad: [0; 2],}; 2001];
    if RI.drawWorld as u64 == 0 { return }
    if !auxedges.is_null() {
        r_edges = auxedges
    } else {
        r_edges =
            (&mut *ledges.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 *mut edge_t as libc::c_long +
                 32 as libc::c_int as libc::c_long -
                 1 as libc::c_int as libc::c_long &
                 !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as
                *mut edge_t
    }
    if r_surfsonstack as u64 != 0 {
        surfaces =
            (&mut *lsurfs.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 *mut surf_t as libc::c_long +
                 32 as libc::c_int as libc::c_long -
                 1 as libc::c_int as libc::c_long &
                 !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as
                *mut surf_t;
        surf_max = &mut *surfaces.offset(r_cnumsurfs as isize) as *mut surf_t;
        // surface 0 doesn't really exist; it's just a dummy because index 0
	// is used to indicate no edge attached to surface
        memset(&mut *surfaces.offset(0 as libc::c_int as isize) as *mut surf_t
                   as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<surf_t>() as libc::c_ulong);
        surfaces = surfaces.offset(-1);
        R_SurfacePatch();
    }
    R_BeginEdgeFrame();
    // this will prepare edges
    R_RenderWorld();
    // move brushes to separate list to merge with edges?
    R_DrawBEntitiesOnList();
    // display all edges
    R_ScanEdges();
}
/*
===============
R_MarkLeaves
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_MarkLeaves() {
    let mut vis: *mut byte = 0 as *mut byte;
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    let mut i: libc::c_int = 0;
    if r_oldviewcluster == r_viewcluster && (*r_novis).value == 0. &&
           r_viewcluster != -(1 as libc::c_int) {
        return
    }
    tr.visframecount += 1;
    r_oldviewcluster = r_viewcluster;
    gEngfuncs.R_FatPVS.expect("non-null function pointer")(RI.pvsorigin.as_mut_ptr(),
                                                           2.0f32,
                                                           RI.visbytes.as_mut_ptr(),
                                                           (RI.params as
                                                                libc::c_uint &
                                                                (1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    1 as
                                                                        libc::c_int)
                                                               as qboolean,
                                                           false_0);
    vis = RI.visbytes.as_mut_ptr();
    i = 0 as libc::c_int;
    while i <
              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                     as
                                                                                     libc::c_int)).numleafs
          {
        if *vis.offset((i >> 3 as libc::c_int) as isize) as libc::c_int &
               (1 as libc::c_int) << (i & 7 as libc::c_int) != 0 {
            node =
                &mut *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                             as
                                                                                             libc::c_int)).leafs.offset((i
                                                                                                                             +
                                                                                                                             1
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                                                                                                                            as
                                                                                                                            isize)
                    as *mut mleaf_t as *mut mnode_t;
            while !((*node).visframe == tr.visframecount) {
                (*node).visframe = tr.visframecount;
                node = (*node).parent;
                if node.is_null() { break ; }
            }
        }
        i += 1
    };
}
/*
================
R_RenderScene

R_SetupRefParams must be called right before
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderScene() {
    if gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                            libc::c_int).is_null()
           && RI.drawWorld as libc::c_uint != 0 {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"R_RenderView: NULL worldmodel\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    // frametime is valid only for normal pass
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int ==
           0 as libc::c_int as libc::c_uint {
        tr.frametime =
            ((*gpGlobals).time - (*gpGlobals).oldtime) as libc::c_double
    } else { tr.frametime = 0.0f64 }
    // begin a new frame
    tr.framecount += 1;
    if tr.map_unload as u64 != 0 { D_FlushCaches(); tr.map_unload = false_0 }
    R_SetupFrustum();
    R_SetupFrame();
    R_PushDlights();
    R_SetupModelviewMatrix(RI.worldviewMatrix.as_mut_ptr());
    R_SetupProjectionMatrix(RI.projectionMatrix.as_mut_ptr());
    Matrix4x4_Concat(RI.worldviewProjectionMatrix.as_mut_ptr(),
                     RI.projectionMatrix.as_mut_ptr() as *const [vec_t; 4],
                     RI.worldviewMatrix.as_mut_ptr() as *const [vec_t; 4]);
    tr.modelviewIdentity = true_0;
    //	R_SetupGL( true );
	//R_Clear( ~0 );
    R_MarkLeaves();
    // R_PushDlights (r_worldmodel); ??
	//R_DrawWorld();
    R_EdgeDrawing(); // don't let sound get messed up if going slow
    gEngfuncs.CL_ExtraUpdate.expect("non-null function pointer")();
    R_DrawEntitiesOnList();
    //	R_DrawWaterSurfaces();
    //	R_EndGL();
}
/*
===============
R_DoResetGamma

gamma will be reset for
some type of screenshots
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DoResetGamma() -> qboolean {
    // FIXME: this looks ugly. apply the backward gamma changes to the output image
    return false_0;
}
/*
===============
R_BeginFrame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeginFrame(mut clearScene: qboolean) {
    // unused
    if (*vid_gamma).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 ||
           (*vid_brightness).flags & (1 as libc::c_int) << 13 as libc::c_int
               != 0 {
        gEngfuncs.BuildGammaTable.expect("non-null function pointer")((*vid_gamma).value,
                                                                      (*vid_brightness).value);
        D_FlushCaches();
        // next frame will be restored gamma
        (*vid_brightness).flags =
            (*vid_brightness).flags &
                !((1 as libc::c_int) << 13 as libc::c_int);
        (*vid_gamma).flags =
            (*vid_gamma).flags & !((1 as libc::c_int) << 13 as libc::c_int)
    }
    R_Set2DMode(true_0);
    // draw buffer stuff
	//pglDrawBuffer( GL_BACK );
    // update texture parameters
	//if( FBitSet( gl_texture_nearest->flags|gl_lightmap_nearest->flags|gl_texture_anisotropy->flags|gl_texture_lodbias->flags, FCVAR_CHANGED ))
		//R_SetTextureParameters();
    gEngfuncs.CL_ExtraUpdate.expect("non-null function pointer")();
}
/*
===============
R_SetupRefParams

set initial params for renderer
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupRefParams(mut rvp: *const ref_viewpass_t) {
    RI.params = 0 as libc::c_int;
    RI.drawWorld =
        ((*rvp).flags & (1 as libc::c_int) << 0 as libc::c_int) as qboolean;
    RI.onlyClientDraw =
        ((*rvp).flags & (1 as libc::c_int) << 3 as libc::c_int) as qboolean;
    if (*rvp).flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        RI.drawOrtho =
            ((*rvp).flags & (1 as libc::c_int) << 2 as libc::c_int) as
                qboolean
    } else { RI.drawOrtho = false_0 }
    // setup viewport
    RI.viewport[0 as libc::c_int as usize] =
        (*rvp).viewport[0 as libc::c_int as usize];
    RI.viewport[1 as libc::c_int as usize] =
        (*rvp).viewport[1 as libc::c_int as usize];
    RI.viewport[2 as libc::c_int as usize] =
        (*rvp).viewport[2 as libc::c_int as usize];
    RI.viewport[3 as libc::c_int as usize] =
        (*rvp).viewport[3 as libc::c_int as usize];
    // calc FOV
    RI.fov_x = (*rvp).fov_x;
    RI.fov_y = (*rvp).fov_y;
    RI.vieworg[0 as libc::c_int as usize] =
        (*rvp).vieworigin[0 as libc::c_int as usize];
    RI.vieworg[1 as libc::c_int as usize] =
        (*rvp).vieworigin[1 as libc::c_int as usize];
    RI.vieworg[2 as libc::c_int as usize] =
        (*rvp).vieworigin[2 as libc::c_int as usize];
    RI.viewangles[0 as libc::c_int as usize] =
        (*rvp).viewangles[0 as libc::c_int as usize];
    RI.viewangles[1 as libc::c_int as usize] =
        (*rvp).viewangles[1 as libc::c_int as usize];
    RI.viewangles[2 as libc::c_int as usize] =
        (*rvp).viewangles[2 as libc::c_int as usize];
    RI.pvsorigin[0 as libc::c_int as usize] =
        (*rvp).vieworigin[0 as libc::c_int as usize];
    RI.pvsorigin[1 as libc::c_int as usize] =
        (*rvp).vieworigin[1 as libc::c_int as usize];
    RI.pvsorigin[2 as libc::c_int as usize] =
        (*rvp).vieworigin[2 as libc::c_int as usize];
}
/*
===============
R_RenderFrame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderFrame(mut rvp: *const ref_viewpass_t) {
    if (*r_norefresh).value != 0. { return }
    // prevent cache overrun
    if (*gpGlobals).height > vid.height || (*gpGlobals).width > vid.width {
        return
    }
    // setup the initial render params
    R_SetupRefParams(rvp);
    // completely override rendering
    if (*gEngfuncs.drawFuncs).GL_RenderFrame.is_some() {
        tr.fCustomRendering = true_0;
        if (*gEngfuncs.drawFuncs).GL_RenderFrame.expect("non-null function pointer")(rvp)
               != 0 {
            //R_GatherPlayerLight();
            tr.realframecount += 1; // right called after viewmodel events
            tr.fResetVis = true_0;
            return
        }
    }
    tr.fCustomRendering = false_0;
    if RI.onlyClientDraw as u64 == 0 { R_RunViewmodelEvents(); }
    tr.realframecount += 1;
    R_RenderScene();
}
/*
===============
R_EndFrame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_EndFrame() {
    // flush any remaining 2D bits
    R_Set2DMode(false_0);
    // blit pixels
    R_BlitScreen();
}
/*
===============
R_DrawCubemapView
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawCubemapView(mut origin: *const vec_t,
                                           mut angles: *const vec_t,
                                           mut size: libc::c_int) {
    let mut rvp: ref_viewpass_t =
        ref_viewpass_t{viewport: [0; 4],
                       vieworigin: [0.; 3],
                       viewangles: [0.; 3],
                       viewentity: 0,
                       fov_x: 0.,
                       fov_y: 0.,
                       flags: 0,};
    // basic params
    rvp.viewentity = 0 as libc::c_int; // this is a final fov value
    rvp.flags = rvp.viewentity;
    rvp.flags = rvp.flags | (1 as libc::c_int) << 0 as libc::c_int;
    rvp.flags = rvp.flags | (1 as libc::c_int) << 1 as libc::c_int;
    rvp.viewport[1 as libc::c_int as usize] = 0 as libc::c_int;
    rvp.viewport[0 as libc::c_int as usize] =
        rvp.viewport[1 as libc::c_int as usize];
    rvp.viewport[3 as libc::c_int as usize] = size;
    rvp.viewport[2 as libc::c_int as usize] =
        rvp.viewport[3 as libc::c_int as usize];
    rvp.fov_y = 90.0f32;
    rvp.fov_x = rvp.fov_y;
    // setup origin & angles
    rvp.vieworigin[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize);
    rvp.vieworigin[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize);
    rvp.vieworigin[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize);
    rvp.viewangles[0 as libc::c_int as usize] =
        *angles.offset(0 as libc::c_int as isize);
    rvp.viewangles[1 as libc::c_int as usize] =
        *angles.offset(1 as libc::c_int as isize);
    rvp.viewangles[2 as libc::c_int as usize] =
        *angles.offset(2 as libc::c_int as isize);
    R_RenderFrame(&mut rvp);
    RI.viewleaf = 0 as *mut mleaf_t;
    // force markleafs next frame
}
/*
===============
R_NewMap
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_NewMap() {
    let mut i: libc::c_int = 0; // clear all level decals
    let mut world: *mut model_t =
        gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                             libc::c_int);
    r_viewcluster = -(1 as libc::c_int);
    (*tr.draw_list).num_solid_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_trans_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_beam_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_edge_entities = 0 as libc::c_int as uint;
    R_ClearDecals();
    R_StudioResetPlayerModels();
    r_cnumsurfs = (*sw_maxsurfs).value as libc::c_int;
    if r_cnumsurfs <= 2000 as libc::c_int {
        r_cnumsurfs = 2000 as libc::c_int
    }
    if r_cnumsurfs > 2000 as libc::c_int {
        surfaces =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                     (r_cnumsurfs
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<surf_t>()
                                                                                                          as
                                                                                                          libc::c_ulong),
                                                                     true_0,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     1849 as
                                                                         libc::c_int)
                as *mut surf_t;
        surface_p = surfaces;
        surf_max = &mut *surfaces.offset(r_cnumsurfs as isize) as *mut surf_t;
        r_surfsonstack = false_0;
        // surface 0 doesn't really exist; it's just a dummy because index 0
	// is used to indicate no edge attached to surface
        surfaces = surfaces.offset(-1);
        R_SurfacePatch();
    } else { r_surfsonstack = true_0 }
    r_numallocatededges = (*sw_maxedges).value as libc::c_int;
    if r_numallocatededges < 4000 as libc::c_int {
        r_numallocatededges = 4000 as libc::c_int
    }
    if r_numallocatededges <= 4000 as libc::c_int {
        auxedges = 0 as *mut edge_t
    } else {
        auxedges =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                     (r_numallocatededges
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<edge_t>()
                                                                                                          as
                                                                                                          libc::c_ulong),
                                                                     false_0,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     1874 as
                                                                         libc::c_int)
                as *mut edge_t
    }
    // clear out efrags in case the level hasn't been reloaded
    i = 0 as libc::c_int;
    while i < (*world).numleafs {
        let ref mut fresh4 =
            (*(*world).leafs.offset((i + 1 as libc::c_int) as isize)).efrags;
        *fresh4 = 0 as *mut efrag_s;
        i += 1
    }
    tr.sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(&mut *(*world).surfaces.offset(0
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               isize));
    i = 1 as libc::c_int;
    while i < (*world).numsurfaces {
        let mut sample_size: libc::c_int =
            gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(&mut *(*world).surfaces.offset(i
                                                                                                                   as
                                                                                                                   isize));
        if sample_size != tr.sample_size {
            tr.sample_size = -(1 as libc::c_int);
            break ;
        } else { i += 1 }
    }
    tr.sample_bits = -(1 as libc::c_int) as uint;
    if tr.sample_size != -(1 as libc::c_int) {
        let mut sample_pot: uint = 0;
        tr.sample_bits = 0 as libc::c_int as uint;
        sample_pot = 1 as libc::c_int as uint;
        while sample_pot < tr.sample_size as libc::c_uint {
            sample_pot <<= 1 as libc::c_int;
            tr.sample_bits = tr.sample_bits.wrapping_add(1)
        }
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"Map sample size is %d\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             tr.sample_size);
}
/*
================
R_InitTurb
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitTurb() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1280 as libc::c_int {
        sintable[i as usize] =
            ((8 as libc::c_int * 0x10000 as libc::c_int) as libc::c_double +
                 __tg_sin_0(i as libc::c_double * 3.14159f64 *
                                2 as libc::c_int as libc::c_double /
                                128 as libc::c_int as libc::c_double) *
                     8 as libc::c_int as libc::c_double *
                     0x10000 as libc::c_int as libc::c_double) as libc::c_int;
        //PGM
        intsintable[i as usize] =
            (3 as libc::c_int as libc::c_double +
                 __tg_sin_0(i as libc::c_double * 3.14159f64 *
                                2 as libc::c_int as libc::c_double /
                                128 as libc::c_int as libc::c_double) *
                     3 as libc::c_int as libc::c_double) as
                libc::c_int; // AMP2, not 20
        blanktable[i as usize] = 0 as libc::c_int;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Init() -> qboolean {
    let mut glblit: qboolean = false_0;
    gl_emboss_scale =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_emboss_scale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       30 as
                                                                           libc::c_int,
                                                               b"fake bumpmapping scale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    vid_gamma =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"gamma\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    r_norefresh =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_norefresh\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"disable 3D rendering (use with caution)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_drawentities =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"r_drawentities\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    vid_brightness =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"brightness\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    r_fullbright =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_fullbright\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   15 as
                                                                       libc::c_int,
                                                               b"disable lightmaps, get fullbright for entities\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_dynamic =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_dynamic\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"allow dynamic lighting (dlights, lightstyles)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lightmap =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lightmap\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   15 as
                                                                       libc::c_int,
                                                               b"lightmap debugging tool\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    //	sw_aliasstats = ri.Cvar_Get ("sw_polymodelstats", "0", 0);
//	sw_allow_modex = ri.Cvar_Get( "sw_allow_modex", "1", CVAR_ARCHIVE );
    sw_clearcolor =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_clearcolor\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"48999\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"screen clear color\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_drawflat =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_drawflat\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_draworder =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_draworder\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_maxedges =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_maxedges\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"32\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_maxsurfs =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_maxsurfs\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_mipscale =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_mipscale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"nothing\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_mipcap =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_mipcap\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"nothing\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_reportedgeout =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_reportedgeout\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_reportsurfout =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_reportsurfout\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_stipplealpha =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_stipplealpha\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"nothing\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_surfcacheoverride =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_surfcacheoverride\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_waterwarp =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_waterwarp\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"nothing\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_notransbrushes =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_notransbrushes\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"do not apply transparency to water/glasses (faster)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_noalphabrushes =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_noalphabrushes\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"do not draw brush holes (faster)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_traceglow =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_traceglow\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"cull flares behind models\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    sw_texfilt =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"sw_texfilt\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"texture dither\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    //r_lefthand = ri.Cvar_Get( "hand", "0", FCVAR_USERINFO | FCVAR_ARCHIVE );
//	r_speeds = ri.Cvar_Get ("r_speeds", "0", 0);
    r_decals =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"r_decals\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    //r_drawworld = ri.Cvar_Get ("r_drawworld", "1", 0);
	//r_dspeeds = ri.Cvar_Get ("r_dspeeds", "0", 0);
//	r_lightlevel = ri.Cvar_Get ("r_lightlevel", "0", 0);
	//r_lerpmodels = ri.Cvar_Get( "r_lerpmodels", "1", 0 );
    r_novis =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_novis\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    tracerred =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"tracerred\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.8\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer red component weight ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    tracergreen =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"tracergreen\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.8\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer green component weight ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    tracerblue =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"tracerblue\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.4\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer blue component weight ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    traceralpha =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"traceralpha\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.5\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer alpha amount ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_temppool =
        gEngfuncs._Mem_AllocPool.expect("non-null function pointer")(b"ref_soft zone\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_soft/r_main.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     1974 as
                                                                         libc::c_int);
    glblit =
        (gEngfuncs.Sys_CheckParm.expect("non-null function pointer")(b"-glblit\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
             != 0) as libc::c_int as qboolean;
    // create the window and set up the context
    if glblit as u64 == 0 &&
           gEngfuncs.R_Init_Video.expect("non-null function pointer")(REF_SOFTWARE
                                                                          as
                                                                          libc::c_int)
               as u64 == 0 {
        // request software blitter
        gEngfuncs.R_Free_Video.expect("non-null function pointer")();
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"failed to initialize software blitter, fallback to glblit\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        glblit = true_0
    }
    if glblit as libc::c_uint != 0 &&
           gEngfuncs.R_Init_Video.expect("non-null function pointer")(REF_GL
                                                                          as
                                                                          libc::c_int)
               as u64 == 0 {
        // request GL context
        gEngfuncs.R_Free_Video.expect("non-null function pointer")();
        return false_0
    }
    R_InitBlit(glblit);
    R_InitImages();
    // init draw stack
    tr.draw_list =
        &mut *tr.draw_stack.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut draw_list_t;
    tr.draw_stack_pos = 0 as libc::c_int;
    qfrustum.view_clipplanes[0 as libc::c_int as usize].leftedge =
        true_0 as libc::c_int as byte;
    qfrustum.view_clipplanes[1 as libc::c_int as usize].rightedge =
        true_0 as libc::c_int as byte;
    qfrustum.view_clipplanes[3 as libc::c_int as usize].leftedge =
        false_0 as libc::c_int as byte;
    qfrustum.view_clipplanes[2 as libc::c_int as usize].leftedge =
        qfrustum.view_clipplanes[3 as libc::c_int as usize].leftedge;
    qfrustum.view_clipplanes[1 as libc::c_int as usize].leftedge =
        qfrustum.view_clipplanes[2 as libc::c_int as usize].leftedge;
    qfrustum.view_clipplanes[3 as libc::c_int as usize].rightedge =
        false_0 as libc::c_int as byte;
    qfrustum.view_clipplanes[2 as libc::c_int as usize].rightedge =
        qfrustum.view_clipplanes[3 as libc::c_int as usize].rightedge;
    qfrustum.view_clipplanes[0 as libc::c_int as usize].rightedge =
        qfrustum.view_clipplanes[2 as libc::c_int as usize].rightedge;
    R_StudioInit();
    R_SpriteInit();
    R_InitTurb();
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_Shutdown() {
    R_ShutdownImages();
    gEngfuncs.R_Free_Video.expect("non-null function pointer")();
}
/*
===============
CL_FxBlend
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FxBlend(mut e: *mut cl_entity_t) -> libc::c_int {
    let mut blend: libc::c_int =
        0 as libc::c_int; // Use ent index to de-sync these fx
    let mut offset: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut tmp: vec3_t = [0.; 3];
    offset = (*e).index as libc::c_float * 363.0f32;
    match (*e).curstate.renderfx {
        3 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x40 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      2 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        4 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x40 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      8 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        1 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x10 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      2 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        2 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x10 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      8 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        5 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt > 0 as libc::c_int {
                    (*e).curstate.renderamt -= 1 as libc::c_int
                } else { (*e).curstate.renderamt = 0 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        6 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt > 3 as libc::c_int {
                    (*e).curstate.renderamt -= 4 as libc::c_int
                } else { (*e).curstate.renderamt = 0 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        7 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt < 255 as libc::c_int {
                    (*e).curstate.renderamt += 1 as libc::c_int
                } else { (*e).curstate.renderamt = 255 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        8 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt < 252 as libc::c_int {
                    (*e).curstate.renderamt += 4 as libc::c_int
                } else { (*e).curstate.renderamt = 255 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        9 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     __tg_sin((*gpGlobals).time *
                                  4 as libc::c_int as libc::c_float + offset))
                    as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        10 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     __tg_sin((*gpGlobals).time *
                                  16 as libc::c_int as libc::c_float +
                                  offset)) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        11 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     __tg_sin((*gpGlobals).time *
                                  36 as libc::c_int as libc::c_float +
                                  offset)) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        12 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     (__tg_sin((*gpGlobals).time *
                                   2 as libc::c_int as libc::c_float) +
                          __tg_sin((*gpGlobals).time *
                                       17 as libc::c_int as libc::c_float +
                                       offset))) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        13 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     (__tg_sin((*gpGlobals).time *
                                   16 as libc::c_int as libc::c_float) +
                          __tg_sin((*gpGlobals).time *
                                       23 as libc::c_int as libc::c_float +
                                       offset))) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        16 | 15 => {
            tmp[0 as libc::c_int as usize] =
                (*e).origin[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                (*e).origin[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                (*e).origin[2 as libc::c_int as usize];
            tmp[0 as libc::c_int as usize] =
                tmp[0 as libc::c_int as usize] -
                    RI.vieworg[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                tmp[1 as libc::c_int as usize] -
                    RI.vieworg[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                tmp[2 as libc::c_int as usize] -
                    RI.vieworg[2 as libc::c_int as usize];
            dist =
                tmp[0 as libc::c_int as usize] *
                    RI.vforward[0 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vforward[1 as libc::c_int as usize] +
                    tmp[2 as libc::c_int as usize] *
                        RI.vforward[2 as libc::c_int as usize];
            // turn off distance fade
            if (*e).curstate.renderfx == kRenderFxDistort as libc::c_int {
                dist = 1 as libc::c_int as libc::c_float
            }
            if dist <= 0 as libc::c_int as libc::c_float {
                blend = 0 as libc::c_int
            } else {
                (*e).curstate.renderamt = 180 as libc::c_int;
                if dist <= 100 as libc::c_int as libc::c_float {
                    blend = (*e).curstate.renderamt
                } else {
                    blend =
                        ((1.0f32 -
                              (dist - 100 as libc::c_int as libc::c_float) *
                                  (1.0f32 / 400.0f32)) *
                             (*e).curstate.renderamt as libc::c_float) as
                            libc::c_int
                }
                blend +=
                    gEngfuncs.COM_RandomLong.expect("non-null function pointer")(-(32
                                                                                       as
                                                                                       libc::c_int),
                                                                                 31
                                                                                     as
                                                                                     libc::c_int)
            }
        }
        _ => { blend = (*e).curstate.renderamt }
    }
    blend =
        if blend >= 0 as libc::c_int {
            if blend < 255 as libc::c_int {
                blend
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    return blend;
}
