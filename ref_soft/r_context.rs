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
    pub type world_static_s;
    pub type mip_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Draw_Fill(x: libc::c_int, y: libc::c_int, w: libc::c_int,
                 h: libc::c_int);
    #[no_mangle]
    fn R_SetDisplayTransform(rotate: ref_screen_rotation_t,
                             offset_x: libc::c_int, offset_y: libc::c_int,
                             scale_x: libc::c_float, scale_y: libc::c_float)
     -> qboolean;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    fn CL_DrawBeams(fTrans: libc::c_int, active_beams: *mut BEAM);
    #[no_mangle]
    fn R_BeamCull(start: *const vec_t, end: *const vec_t, pvsOnly: qboolean)
     -> qboolean;
    #[no_mangle]
    fn R_DecalSetupVerts(pDecal: *mut decal_t, surf: *mut msurface_t,
                         texture: libc::c_int, outCount: *mut libc::c_int)
     -> *mut libc::c_float;
    #[no_mangle]
    fn R_EntityRemoveDecals(mod_0: *mut model_t);
    #[no_mangle]
    fn GL_Bind(tmu: libc::c_int, texnum: libc::c_uint);
    #[no_mangle]
    fn R_Set2DMode(enable: qboolean);
    #[no_mangle]
    fn R_DrawTileClear(texnum: libc::c_int, x: libc::c_int, y: libc::c_int,
                       w: libc::c_int, h: libc::c_int);
    #[no_mangle]
    fn R_UploadStretchRaw(texture: libc::c_int, cols: libc::c_int,
                          rows: libc::c_int, width: libc::c_int,
                          height: libc::c_int, data: *const byte);
    #[no_mangle]
    fn R_GetTexture(texnum: libc::c_uint) -> *mut image_t;
    #[no_mangle]
    fn GL_LoadTexture(name: *const libc::c_char, buf: *const byte,
                      size: size_t, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn GL_LoadTextureArray(names: *mut *const libc::c_char,
                           flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn GL_LoadTextureFromBuffer(name: *const libc::c_char,
                                pic: *mut rgbdata_t, flags: texFlags_t,
                                update: qboolean) -> libc::c_int;
    #[no_mangle]
    fn GL_CreateTexture(name: *const libc::c_char, width: libc::c_int,
                        height: libc::c_int, buffer: *const libc::c_void,
                        flags: texFlags_t) -> libc::c_int;
    #[no_mangle]
    fn GL_CreateTextureArray(name: *const libc::c_char, width: libc::c_int,
                             height: libc::c_int, depth: libc::c_int,
                             buffer: *const libc::c_void, flags: texFlags_t)
     -> libc::c_int;
    #[no_mangle]
    fn GL_ProcessTexture(texnum: libc::c_int, gamma: libc::c_float,
                         topColor: libc::c_int, bottomColor: libc::c_int);
    #[no_mangle]
    fn GL_UpdateTexSize(texnum: libc::c_int, width: libc::c_int,
                        height: libc::c_int, depth: libc::c_int);
    #[no_mangle]
    fn GL_FindTexture(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn GL_FreeTexture(texnum: libc::c_uint);
    #[no_mangle]
    fn CL_RunLightStyles();
    #[no_mangle]
    fn R_LightVec(start: *const vec_t, end: *const vec_t,
                  lightspot: *mut vec_t, lightvec: *mut vec_t) -> colorVec;
    #[no_mangle]
    fn R_LightPoint(p0: *const vec_t) -> colorVec;
    #[no_mangle]
    fn R_ClearScene();
    #[no_mangle]
    fn R_RenderScene();
    #[no_mangle]
    fn R_AllowFog(allowed: qboolean);
    #[no_mangle]
    fn R_PushScene();
    #[no_mangle]
    fn R_PopScene();
    #[no_mangle]
    fn CL_DrawParticlesExternal(rvp: *const ref_viewpass_t,
                                trans_pass: qboolean,
                                frametime: libc::c_float);
    #[no_mangle]
    fn CL_DrawParticles(frametime: libc::c_double,
                        cl_active_particles: *mut particle_t,
                        partsize: libc::c_float);
    #[no_mangle]
    fn CL_DrawTracers(frametime: libc::c_double,
                      cl_active_tracers: *mut particle_t);
    #[no_mangle]
    fn Mod_LoadSpriteModel(mod_0: *mut model_t, buffer: *const libc::c_void,
                           loaded: *mut qboolean, texFlags: uint);
    #[no_mangle]
    fn R_StudioLerpMovement(e: *mut cl_entity_t, time: libc::c_double,
                            origin: *mut vec_t, angles: *mut vec_t);
    #[no_mangle]
    fn R_StudioGetTexture(e: *mut cl_entity_t) -> *mut mstudiotex_s;
    #[no_mangle]
    fn R_StudioEstimateFrame(e: *mut cl_entity_t,
                             pseqdesc: *mut mstudioseqdesc_t)
     -> libc::c_float;
    #[no_mangle]
    fn CL_InitStudioAPI();
    #[no_mangle]
    fn Mod_StudioLoadTextures(mod_0: *mut model_t, data: *mut libc::c_void);
    #[no_mangle]
    fn VGUI_DrawInit();
    #[no_mangle]
    fn VGUI_DrawShutdown();
    #[no_mangle]
    fn VGUI_SetupDrawingText(pColor: *mut libc::c_int);
    #[no_mangle]
    fn VGUI_SetupDrawingRect(pColor: *mut libc::c_int);
    #[no_mangle]
    fn VGUI_SetupDrawingImage(pColor: *mut libc::c_int);
    #[no_mangle]
    fn VGUI_BindTexture(id: libc::c_int);
    #[no_mangle]
    fn VGUI_EnableTexture(enable: qboolean);
    #[no_mangle]
    fn VGUI_CreateTexture(id: libc::c_int, width: libc::c_int,
                          height: libc::c_int);
    #[no_mangle]
    fn VGUI_UploadTexture(id: libc::c_int, buffer: *const libc::c_char,
                          width: libc::c_int, height: libc::c_int);
    #[no_mangle]
    fn VGUI_UploadTextureBlock(id: libc::c_int, drawX: libc::c_int,
                               drawY: libc::c_int, rgba: *const byte,
                               blockWidth: libc::c_int,
                               blockHeight: libc::c_int);
    #[no_mangle]
    fn VGUI_DrawQuad(ul: *const vpoint_t, lr: *const vpoint_t);
    #[no_mangle]
    fn VGUI_GetTextureSizes(width: *mut libc::c_int,
                            height: *mut libc::c_int);
    #[no_mangle]
    fn VGUI_GenerateTexture() -> libc::c_int;
    #[no_mangle]
    fn R_Init() -> qboolean;
    #[no_mangle]
    fn R_Shutdown();
    #[no_mangle]
    fn GL_SetupAttributes(safegl: libc::c_int);
    #[no_mangle]
    fn GL_InitExtensions();
    #[no_mangle]
    fn GL_ClearExtensions();
    #[no_mangle]
    fn R_BeginFrame(clearScene: qboolean);
    #[no_mangle]
    fn R_RenderFrame(vp: *const ref_viewpass_s);
    #[no_mangle]
    fn R_EndFrame();
    #[no_mangle]
    fn R_GetSpriteParms(frameWidth: *mut libc::c_int,
                        frameHeight: *mut libc::c_int,
                        numFrames: *mut libc::c_int, curFrame: libc::c_int,
                        pSprite: *const model_s);
    #[no_mangle]
    fn R_DrawStretchRaw(x: libc::c_float, y: libc::c_float, w: libc::c_float,
                        h: libc::c_float, cols: libc::c_int,
                        rows: libc::c_int, data: *const byte,
                        dirty: qboolean);
    #[no_mangle]
    fn R_DrawStretchPic(x: libc::c_float, y: libc::c_float, w: libc::c_float,
                        h: libc::c_float, s1: libc::c_float,
                        t1: libc::c_float, s2: libc::c_float,
                        t2: libc::c_float, texnum: libc::c_int);
    #[no_mangle]
    fn R_WorldToScreen(point: *const vec_t, screen: *mut vec_t)
     -> libc::c_int;
    #[no_mangle]
    fn R_ScreenToWorld(screen: *const vec_t, point: *mut vec_t);
    #[no_mangle]
    fn R_AddEntity(pRefEntity: *mut cl_entity_s, entityType: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn Mod_LoadMapSprite(mod_0: *mut model_s, buffer: *const libc::c_void,
                         size: size_t, loaded: *mut qboolean);
    #[no_mangle]
    fn Mod_SpriteUnloadTextures(data: *mut libc::c_void);
    #[no_mangle]
    fn R_GetSpriteTexture(m_pSpriteModel: *const model_s, frame: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn R_DecalShoot(textureIndex: libc::c_int, entityIndex: libc::c_int,
                    modelIndex: libc::c_int, pos: *mut vec_t,
                    flags: libc::c_int, scale: libc::c_float);
    #[no_mangle]
    fn R_DecalRemoveAll(texture: libc::c_int);
    #[no_mangle]
    fn R_CreateDecalList(pList: *mut decallist_t) -> libc::c_int;
    #[no_mangle]
    fn R_ClearAllDecals();
    #[no_mangle]
    fn R_NewMap();
    #[no_mangle]
    fn CL_AddCustomBeam(pEnvBeam: *mut cl_entity_t);
    #[no_mangle]
    fn TriRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn TriBegin(mode: libc::c_int);
    #[no_mangle]
    fn TriEnd();
    #[no_mangle]
    fn TriTexCoord2f(u: libc::c_float, v: libc::c_float);
    #[no_mangle]
    fn TriVertex3fv(v: *const libc::c_float);
    #[no_mangle]
    fn TriVertex3f(x: libc::c_float, y: libc::c_float, z: libc::c_float);
    #[no_mangle]
    fn _TriColor4f(r: libc::c_float, g: libc::c_float, b: libc::c_float,
                   a: libc::c_float);
    #[no_mangle]
    fn _TriColor4ub(r: byte, g: byte, b: byte, a: byte);
    #[no_mangle]
    fn TriFog(flFogColor: *mut libc::c_float, flStart: libc::c_float,
              flEnd: libc::c_float, bOn: libc::c_int);
    #[no_mangle]
    fn TriGetMatrix(pname: libc::c_int, matrix: *mut libc::c_float);
    #[no_mangle]
    fn TriFogParams(flDensity: libc::c_float, iFogSkybox: libc::c_int);
    #[no_mangle]
    fn TriCullFace(mode: TRICULLSTYLE);
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
pub type vec2_t = [vec_t; 2];
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
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
}
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
pub struct beam_s {
    pub next: *mut BEAM,
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub source: vec3_t,
    pub target: vec3_t,
    pub delta: vec3_t,
    pub t: libc::c_float,
    pub freq: libc::c_float,
    pub die: libc::c_float,
    pub width: libc::c_float,
    pub amplitude: libc::c_float,
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub brightness: libc::c_float,
    pub speed: libc::c_float,
    pub frameRate: libc::c_float,
    pub frame: libc::c_float,
    pub segments: libc::c_int,
    pub startEntity: libc::c_int,
    pub endEntity: libc::c_int,
    pub modelIndex: libc::c_int,
    pub frameCount: libc::c_int,
    pub pFollowModel: *mut model_s,
    pub particles: *mut particle_s,
}
pub type BEAM = beam_s;
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
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
pub type ref_screen_rotation_t = ref_screen_rotation_e;
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
pub type ref_api_t = ref_api_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_interface_s {
    pub R_Init: Option<unsafe extern "C" fn() -> qboolean>,
    pub R_Shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetConfigName: Option<unsafe extern "C" fn()
                                    -> *const libc::c_char>,
    pub R_SetDisplayTransform: Option<unsafe extern "C" fn(_:
                                                               ref_screen_rotation_t,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_float,
                                                           _: libc::c_float)
                                          -> qboolean>,
    pub GL_SetupAttributes: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub GL_InitExtensions: Option<unsafe extern "C" fn() -> ()>,
    pub GL_ClearExtensions: Option<unsafe extern "C" fn() -> ()>,
    pub R_BeginFrame: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_RenderScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_EndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub R_PushScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_PopScene: Option<unsafe extern "C" fn() -> ()>,
    pub GL_BackendStartFrame: Option<unsafe extern "C" fn() -> ()>,
    pub GL_BackendEndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScreen: Option<unsafe extern "C" fn() -> ()>,
    pub R_AllowFog: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub GL_SetRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_AddEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                 _: libc::c_int) -> qboolean>,
    pub CL_AddCustomBeam: Option<unsafe extern "C" fn(_: *mut cl_entity_t)
                                     -> ()>,
    pub R_ProcessEntData: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_ShowTextures: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetTextureOriginalBuffer: Option<unsafe extern "C" fn(_:
                                                                    libc::c_uint)
                                               -> *const byte>,
    pub GL_LoadTextureFromBuffer: Option<unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _:
                                                                  *mut rgbdata_t,
                                                              _: texFlags_t,
                                                              _: qboolean)
                                             -> libc::c_int>,
    pub GL_ProcessTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub R_SetupSky: Option<unsafe extern "C" fn(_: *const libc::c_char)
                               -> ()>,
    pub R_Set2DMode: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_DrawStretchRaw: Option<unsafe extern "C" fn(_: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const byte,
                                                      _: qboolean) -> ()>,
    pub R_DrawStretchPic: Option<unsafe extern "C" fn(_: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int) -> ()>,
    pub R_DrawTileClear: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub FillRGBA: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_int, _: libc::c_int,
                                              _: libc::c_int, _: libc::c_int)
                             -> ()>,
    pub FillRGBABlend: Option<unsafe extern "C" fn(_: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub WorldToScreen: Option<unsafe extern "C" fn(_: *const vec_t,
                                                   _: *mut vec_t)
                                  -> libc::c_int>,
    pub VID_ScreenShot: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> qboolean>,
    pub VID_CubemapShot: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: uint,
                                                     _: *const libc::c_float,
                                                     _: qboolean)
                                    -> qboolean>,
    pub R_LightPoint: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> colorVec>,
    pub R_DecalShoot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut vec_t,
                                                  _: libc::c_int,
                                                  _: libc::c_float) -> ()>,
    pub R_DecalRemoveAll: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_CreateDecalList: Option<unsafe extern "C" fn(_: *mut decallist_s)
                                      -> libc::c_int>,
    pub R_ClearAllDecals: Option<unsafe extern "C" fn() -> ()>,
    pub R_StudioEstimateFrame: Option<unsafe extern "C" fn(_:
                                                               *mut cl_entity_t,
                                                           _:
                                                               *mut mstudioseqdesc_t)
                                          -> libc::c_float>,
    pub R_StudioLerpMovement: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                          _: libc::c_double,
                                                          _: *mut vec_t,
                                                          _: *mut vec_t)
                                         -> ()>,
    pub CL_InitStudioAPI: Option<unsafe extern "C" fn() -> ()>,
    pub R_InitSkyClouds: Option<unsafe extern "C" fn(_: *mut mip_s,
                                                     _: *mut texture_s,
                                                     _: qboolean) -> ()>,
    pub GL_SubdivideSurface: Option<unsafe extern "C" fn(_: *mut msurface_t)
                                        -> ()>,
    pub CL_RunLightStyles: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetSpriteParms: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const model_t)
                                     -> ()>,
    pub R_GetSpriteTexture: Option<unsafe extern "C" fn(_: *const model_t,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
    pub Mod_LoadMapSprite: Option<unsafe extern "C" fn(_: *mut model_s,
                                                       _: *const libc::c_void,
                                                       _: size_t,
                                                       _: *mut qboolean)
                                      -> ()>,
    pub Mod_ProcessRenderData: Option<unsafe extern "C" fn(_: *mut model_t,
                                                           _: qboolean,
                                                           _: *const byte)
                                          -> qboolean>,
    pub Mod_StudioLoadTextures: Option<unsafe extern "C" fn(_: *mut model_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    pub CL_DrawParticles: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: *mut particle_t,
                                                      _: libc::c_float)
                                     -> ()>,
    pub CL_DrawTracers: Option<unsafe extern "C" fn(_: libc::c_double,
                                                    _: *mut particle_t)
                                   -> ()>,
    pub CL_DrawBeams: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut BEAM) -> ()>,
    pub R_BeamCull: Option<unsafe extern "C" fn(_: *const vec_t,
                                                _: *const vec_t, _: qboolean)
                               -> qboolean>,
    pub RefGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub GetDetailScaleForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> ()>,
    pub GetExtraParmsForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte)
                                            -> ()>,
    pub GetFrameTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub R_SetCurrentEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                       -> ()>,
    pub R_SetCurrentModel: Option<unsafe extern "C" fn(_: *mut model_s)
                                      -> ()>,
    pub GL_FindTexture: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
    pub GL_TextureName: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const libc::c_char>,
    pub GL_TextureData: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const byte>,
    pub GL_LoadTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const byte, _: size_t,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GL_CreateTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_void,
                                                      _: texFlags_t)
                                     -> libc::c_int>,
    pub GL_LoadTextureArray: Option<unsafe extern "C" fn(_:
                                                             *mut *const libc::c_char,
                                                         _: libc::c_int)
                                        -> libc::c_int>,
    pub GL_CreateTextureArray: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_char,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _:
                                                               *const libc::c_void,
                                                           _: texFlags_t)
                                          -> libc::c_int>,
    pub GL_FreeTexture: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub DrawSingleDecal: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                     _: *mut msurface_s)
                                    -> ()>,
    pub R_DecalSetupVerts: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                       _: *mut msurface_s,
                                                       _: libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> *mut libc::c_float>,
    pub R_EntityRemoveDecals: Option<unsafe extern "C" fn(_: *mut model_s)
                                         -> ()>,
    pub AVI_UploadRawFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte)
                                       -> ()>,
    pub GL_Bind: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_uint)
                            -> ()>,
    pub GL_SelectTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub GL_LoadTextureMatrix: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float)
                                         -> ()>,
    pub GL_TexMatrixIdentity: Option<unsafe extern "C" fn() -> ()>,
    pub GL_CleanUpTextureUnits: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    pub GL_TexGen: Option<unsafe extern "C" fn(_: libc::c_uint,
                                               _: libc::c_uint) -> ()>,
    pub GL_TextureTarget: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub GL_TexCoordArrayMode: Option<unsafe extern "C" fn(_: libc::c_uint)
                                         -> ()>,
    pub GL_UpdateTexSize: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub GL_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub GL_Reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub GL_DrawParticles: Option<unsafe extern "C" fn(_:
                                                          *const ref_viewpass_s,
                                                      _: qboolean,
                                                      _: libc::c_float)
                                     -> ()>,
    pub LightVec: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: *const libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> colorVec>,
    pub StudioGetTexture: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                     -> *mut mstudiotex_s>,
    pub GL_RenderFrame: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> ()>,
    pub GL_OrthoBounds: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_SpeedsMessage: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: size_t) -> qboolean>,
    pub Mod_GetCurrentVis: Option<unsafe extern "C" fn() -> *mut byte>,
    pub R_NewMap: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *mut libc::c_void>,
    pub TriRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub Begin: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub End: Option<unsafe extern "C" fn() -> ()>,
    pub Color4f: Option<unsafe extern "C" fn(_: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float) -> ()>,
    pub Color4ub: Option<unsafe extern "C" fn(_: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar) -> ()>,
    pub TexCoord2f: Option<unsafe extern "C" fn(_: libc::c_float,
                                                _: libc::c_float) -> ()>,
    pub Vertex3fv: Option<unsafe extern "C" fn(_: *const libc::c_float)
                              -> ()>,
    pub Vertex3f: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float) -> ()>,
    pub Fog: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                         _: libc::c_float, _: libc::c_float,
                                         _: libc::c_int) -> ()>,
    pub ScreenToWorld: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub GetMatrix: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_float) -> ()>,
    pub FogParams: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int) -> ()>,
    pub CullFace: Option<unsafe extern "C" fn(_: TRICULLSTYLE) -> ()>,
    pub VGUI_DrawInit: Option<unsafe extern "C" fn() -> ()>,
    pub VGUI_DrawShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub VGUI_SetupDrawingText: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_int)
                                          -> ()>,
    pub VGUI_SetupDrawingRect: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_int)
                                          -> ()>,
    pub VGUI_SetupDrawingImage: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_int)
                                           -> ()>,
    pub VGUI_BindTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub VGUI_EnableTexture: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub VGUI_CreateTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub VGUI_UploadTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *const libc::c_char,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub VGUI_UploadTextureBlock: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: libc::c_int,
                                                             _: libc::c_int,
                                                             _: *const byte,
                                                             _: libc::c_int,
                                                             _: libc::c_int)
                                            -> ()>,
    pub VGUI_DrawQuad: Option<unsafe extern "C" fn(_: *const vpoint_t,
                                                   _: *const vpoint_t) -> ()>,
    pub VGUI_GetTextureSizes: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                          _: *mut libc::c_int)
                                         -> ()>,
    pub VGUI_GenerateTexture: Option<unsafe extern "C" fn() -> libc::c_int>,
}
pub type ref_interface_t = ref_interface_s;
pub type ref_viewpass_t = ref_viewpass_s;
pub type mip_t = mip_s;
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
vid_sdl.c - SDL vid component
Copyright (C) 2018 a1batross

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
pub static mut gEngfuncs: ref_api_t =
    ref_api_t{EngineGetParm: None,
              Cvar_Get: None,
              pfnGetCvarPointer: None,
              pfnGetCvarFloat: None,
              pfnGetCvarString: None,
              Cvar_SetValue: None,
              Cvar_Set: None,
              Cvar_RegisterVariable: None,
              Cvar_FullSet: None,
              Cmd_AddCommand: None,
              Cmd_RemoveCommand: None,
              Cmd_Argc: None,
              Cmd_Argv: None,
              Cmd_Args: None,
              Cbuf_AddText: None,
              Cbuf_InsertText: None,
              Cbuf_Execute: None,
              Con_Printf: None,
              Con_DPrintf: None,
              Con_Reportf: None,
              Con_NPrintf: None,
              Con_NXPrintf: None,
              CL_CenterPrint: None,
              Con_DrawStringLen: None,
              Con_DrawString: None,
              CL_DrawCenterPrint: None,
              GetLocalPlayer: None,
              GetViewModel: None,
              GetEntityByIndex: None,
              R_BeamGetEntity: None,
              CL_GetWaterEntity: None,
              CL_AddVisibleEntity: None,
              Mod_SampleSizeForFace: None,
              Mod_BoxVisible: None,
              GetWorld: None,
              Mod_PointInLeaf: None,
              Mod_CreatePolygonsForHull: None,
              R_StudioSlerpBones: None,
              R_StudioCalcBoneQuaternion: None,
              R_StudioCalcBonePosition: None,
              R_StudioGetAnim: None,
              pfnStudioEvent: None,
              CL_DrawEFX: None,
              CL_ThinkParticle: None,
              R_FreeDeadParticles: None,
              CL_AllocParticleFast: None,
              CL_AllocElight: None,
              GetDefaultSprite: None,
              R_StoreEfrags: None,
              Mod_ForName: None,
              Mod_Extradata: None,
              pfnGetModelByIndex: None,
              Mod_GetCurrentLoadingModel: None,
              Mod_SetCurrentLoadingModel: None,
              CL_GetRemapInfoForEntity: None,
              CL_AllocRemapInfo: None,
              CL_FreeRemapInfo: None,
              CL_UpdateRemapInfo: None,
              CL_ExtraUpdate: None,
              Host_Error: None,
              COM_SetRandomSeed: None,
              COM_RandomFloat: None,
              COM_RandomLong: None,
              GetScreenFade: None,
              pfnTextMessageGet: None,
              GetPredictedOrigin: None,
              CL_GetPaletteColor: None,
              CL_GetScreenInfo: None,
              SetLocalLightLevel: None,
              Sys_CheckParm: None,
              pfnPlayerInfo: None,
              pfnGetPlayerState: None,
              Mod_CacheCheck: None,
              Mod_LoadCacheFile: None,
              Mod_Calloc: None,
              pfnGetStudioModelInterface: None,
              _Mem_AllocPool: None,
              _Mem_FreePool: None,
              _Mem_Alloc: None,
              _Mem_Realloc: None,
              _Mem_Free: None,
              COM_LoadLibrary: None,
              COM_FreeLibrary: None,
              COM_GetProcAddress: None,
              COM_LoadFile: None,
              FS_FileExists: None,
              FS_AllowDirectPaths: None,
              R_Init_Video: None,
              R_Free_Video: None,
              GL_SetAttribute: None,
              GL_GetAttribute: None,
              GL_GetProcAddress: None,
              GL_SwapBuffers: None,
              SW_CreateBuffer: None,
              SW_LockBuffer: None,
              SW_UnlockBuffer: None,
              BuildGammaTable: None,
              LightToTexGamma: None,
              R_DoResetGamma: None,
              GetLightStyle: None,
              GetDynamicLight: None,
              GetEntityLight: None,
              R_FatPVS: None,
              GetOverviewParms: None,
              pfnTime: None,
              EV_GetPhysent: None,
              EV_TraceSurface: None,
              PM_TraceLine: None,
              EV_VisTraceLine: None,
              CL_TraceLine: None,
              pfnGetMoveVars: None,
              Image_AddCmdFlags: None,
              Image_SetForceFlags: None,
              Image_ClearForceFlags: None,
              Image_CustomPalette: None,
              Image_Process: None,
              FS_LoadImage: None,
              FS_SaveImage: None,
              FS_CopyImage: None,
              FS_FreeImage: None,
              Image_SetMDLPointer: None,
              Image_GetPool: None,
              Image_GetPFDesc: None,
              pfnDrawNormalTriangles: None,
              pfnDrawTransparentTriangles: None,
              drawFuncs:
                  0 as *const render_interface_t as *mut render_interface_t,};
#[no_mangle]
pub static mut gpGlobals: *mut ref_globals_t =
    0 as *const ref_globals_t as *mut ref_globals_t;
#[no_mangle]
pub static mut tr: gl_globals_t =
    gl_globals_t{defaultTexture: 0,
                 particleTexture: 0,
                 whiteTexture: 0,
                 grayTexture: 0,
                 blackTexture: 0,
                 solidskyTexture: 0,
                 alphaskyTexture: 0,
                 lightmapTextures: [0; 256],
                 dlightTexture: 0,
                 skyboxTextures: [0; 6],
                 cinTexture: 0,
                 draw_stack:
                     [draw_list_t{edge_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  solid_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  trans_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  beam_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  num_edge_entities: 0,
                                  num_solid_entities: 0,
                                  num_trans_entities: 0,
                                  num_beam_entities: 0,}; 2],
                 draw_stack_pos: 0,
                 draw_list: 0 as *const draw_list_t as *mut draw_list_t,
                 draw_decals:
                     [0 as *const msurface_t as *mut msurface_t; 4096],
                 num_draw_decals: 0,
                 modelviewIdentity: false_0,
                 visframecount: 0,
                 dlightframecount: 0,
                 realframecount: 0,
                 framecount: 0,
                 ignore_lightgamma: false_0,
                 fCustomRendering: false_0,
                 fResetVis: false_0,
                 fFlipViewModel: false_0,
                 recursion_level: 0,
                 max_recursion: 0,
                 visbytes: [0; 4096],
                 lightstylevalue: [0; 64],
                 block_size: 0,
                 frametime: 0.,
                 blend: 0.,
                 modelorg: [0.; 3],
                 fCustomSkybox: false_0,
                 sample_size: 0,
                 sample_bits: 0,
                 map_unload: false_0,};
#[no_mangle]
pub static mut r_stats: ref_speeds_t =
    ref_speeds_t{c_world_polys: 0,
                 c_studio_polys: 0,
                 c_sprite_polys: 0,
                 c_alias_polys: 0,
                 c_world_leafs: 0,
                 c_view_beams_count: 0,
                 c_active_tents_count: 0,
                 c_alias_models_drawn: 0,
                 c_studio_models_drawn: 0,
                 c_sprite_models_drawn: 0,
                 c_particle_count: 0,
                 c_client_ents: 0,
                 t_world_node: 0.,
                 t_world_draw: 0.,};
#[no_mangle]
pub static mut r_temppool: poolhandle_t = 0;
#[no_mangle]
pub static mut gl_emboss_scale: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_norefresh: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_brightness: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid: viddef_t =
    viddef_t{buffer: 0 as *const pixel_t as *mut pixel_t,
             colormap: [0; 262144],
             screen: [0; 65536],
             screen32: [0; 65536],
             addmap: [0; 65536],
             modmap: [0; 65536],
             alphamap: [0; 786432],
             color: 0,
             is2d: false_0,
             alpha: 0,
             rendermode: 0,
             rowbytes: 0,
             width: 0,
             height: 0,};
unsafe extern "C" fn R_ClearScreen() { }
unsafe extern "C" fn R_GetTextureOriginalBuffer(mut idx: libc::c_uint)
 -> *const byte {
    let mut glt: *mut image_t = R_GetTexture(idx);
    if glt.is_null() || (*glt).original.is_null() ||
           (*(*glt).original).buffer.is_null() {
        return 0 as *const byte
    }
    return (*(*glt).original).buffer;
}
/*
=============
CL_FillRGBA

=============
*/
unsafe extern "C" fn CL_FillRGBA(mut _x: libc::c_float, mut _y: libc::c_float,
                                 mut _w: libc::c_float, mut _h: libc::c_float,
                                 mut r: libc::c_int, mut g: libc::c_int,
                                 mut b: libc::c_int, mut a: libc::c_int) {
    vid.rendermode = kRenderTransAdd as libc::c_int;
    _TriColor4ub(r as byte, g as byte, b as byte, a as byte);
    Draw_Fill(_x as libc::c_int, _y as libc::c_int, _w as libc::c_int,
              _h as libc::c_int);
}
/*
=============
pfnFillRGBABlend

=============
*/
unsafe extern "C" fn CL_FillRGBABlend(mut _x: libc::c_float,
                                      mut _y: libc::c_float,
                                      mut _w: libc::c_float,
                                      mut _h: libc::c_float,
                                      mut r: libc::c_int, mut g: libc::c_int,
                                      mut b: libc::c_int,
                                      mut a: libc::c_int) {
    vid.rendermode = kRenderTransAlpha as libc::c_int; //glt->format;
    _TriColor4ub(r as byte, g as byte, b as byte, a as byte); //glt->encode;
    Draw_Fill(_x as libc::c_int, _y as libc::c_int, _w as libc::c_int,
              _h as libc::c_int); //tr.skytexturenum;
}
#[no_mangle]
pub unsafe extern "C" fn Mod_ProcessRenderData(mut mod_0: *mut model_t,
                                               mut create: qboolean,
                                               mut buf: *const byte)
 -> qboolean {
    let mut loaded: qboolean = true_0; //glt->target;
    if create as u64 != 0 {
        match (*mod_0).type_0 as libc::c_int {
            3 => { }
            1 => {
                Mod_LoadSpriteModel(mod_0, buf as *const libc::c_void,
                                    &mut loaded,
                                    (*mod_0).numtexinfo as
                                        uint); //glt->texnum;
            }
            2 => { }
            0 => { }
            _ => {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"Mod_LoadModel: unsupported type %d\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         (*mod_0).type_0
                                                                             as
                                                                             libc::c_int); //glState.activeTMU;
            }
        }
    } //GL_MaxTextureUnits();
    if loaded as libc::c_uint != 0 &&
           (*gEngfuncs.drawFuncs).Mod_ProcessUserData.is_some() {
        (*gEngfuncs.drawFuncs).Mod_ProcessUserData.expect("non-null function pointer")(mod_0,
                                                                                       create,
                                                                                       buf); //glConfig.context;
    } //glConfig.wrapper;
    if create as u64 == 0 {
        Mod_UnloadTextures(mod_0); //glState.stencilEnabled;
    }
    return loaded;
}
unsafe extern "C" fn GL_RefGetParm(mut parm: libc::c_int,
                                   mut arg: libc::c_int) -> libc::c_int {
    let mut glt: *mut image_t = 0 as *mut image_t;
    match parm {
        1 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).width as libc::c_int
        }
        2 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).height as libc::c_int
        }
        3 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).srcWidth as libc::c_int
        }
        4 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).srcHeight as libc::c_int
        }
        13 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return 0 as libc::c_int
        }
        14 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return 0 as libc::c_int
        }
        15 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).numMips as libc::c_int
        }
        11 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).depth as libc::c_int
        }
        5 => {
            if !(arg >= 0 as libc::c_int && arg < 6 as libc::c_int) {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"../ref_soft/r_context.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         154
                                                                             as
                                                                             libc::c_int);
            }
            return tr.skyboxTextures[arg as usize]
        }
        6 => { return 0 as libc::c_int }
        7 => {
            arg =
                if arg >= 0 as libc::c_int {
                    if arg < 256 as libc::c_int - 1 as libc::c_int {
                        arg
                    } else { (256 as libc::c_int) - 1 as libc::c_int }
                } else { 0 as libc::c_int };
            return tr.lightmapTextures[arg as usize]
        }
        21 => { return (*gpGlobals).wideScreen as libc::c_int }
        22 => { return (*gpGlobals).fullScreen as libc::c_int }
        23 => { return (*gpGlobals).width }
        24 => { return (*gpGlobals).height }
        8 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return 0 as libc::c_int
        }
        9 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return 0 as libc::c_int
        }
        10 => {
            glt = R_GetTexture(arg as libc::c_uint);
            return (*glt).flags as libc::c_int
        }
        27 => { return 0 as libc::c_int }
        28 => {
            arg =
                if arg >= 0 as libc::c_int {
                    if arg < 64 as libc::c_int - 1 as libc::c_int {
                        arg
                    } else { (64 as libc::c_int) - 1 as libc::c_int }
                } else { 0 as libc::c_int };
            return tr.lightstylevalue[arg as usize]
        }
        29 => { return 0 as libc::c_int }
        31 => { return 0 as libc::c_int }
        33 => {
            if arg >= 0 as libc::c_int &&
                   arg <
                       (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                              as
                                                                                              libc::c_int)).numsurfaces
               {
                return gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(&mut *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int)).surfaces.offset(arg
                                                                                                                                                                                                          as
                                                                                                                                                                                                          isize))
            }
            return 16 as libc::c_int
        }
        34 => { return 0 as libc::c_int }
        35 => { return 0 as libc::c_int }
        36 => { return 0 as libc::c_int }
        17 => {
            return (Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(parm,
                                                                                                                          arg)
                        != 0 && tr.fCustomSkybox as u64 == 0) as libc::c_int
        }
        _ => {
            return Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(parm,
                                                                                                                         arg)
        }
    };
}
unsafe extern "C" fn R_GetDetailScaleForTexture(mut texture: libc::c_int,
                                                mut xScale:
                                                    *mut libc::c_float,
                                                mut yScale:
                                                    *mut libc::c_float) {
    let mut glt: *mut image_t = R_GetTexture(texture as libc::c_uint);
    if !xScale.is_null() { *xScale = (*glt).xscale }
    if !yScale.is_null() { *yScale = (*glt).yscale };
}
unsafe extern "C" fn R_GetExtraParmsForTexture(mut texture: libc::c_int,
                                               mut red: *mut byte,
                                               mut green: *mut byte,
                                               mut blue: *mut byte,
                                               mut density: *mut byte) {
    let mut glt: *mut image_t = R_GetTexture(texture as libc::c_uint);
    if !red.is_null() { *red = (*glt).fogParams[0 as libc::c_int as usize] }
    if !green.is_null() {
        *green = (*glt).fogParams[1 as libc::c_int as usize]
    }
    if !blue.is_null() { *blue = (*glt).fogParams[2 as libc::c_int as usize] }
    if !density.is_null() {
        *density = (*glt).fogParams[3 as libc::c_int as usize]
    };
}
unsafe extern "C" fn R_SetCurrentEntity(mut ent: *mut cl_entity_t) {
    RI.currententity = ent;
    // set model also
    if !RI.currententity.is_null() {
        RI.currentmodel = (*RI.currententity).model
    };
}
unsafe extern "C" fn R_SetCurrentModel(mut mod_0: *mut model_t) {
    RI.currentmodel = mod_0;
}
unsafe extern "C" fn R_GetFrameTime() -> libc::c_float {
    return tr.frametime as libc::c_float;
}
unsafe extern "C" fn GL_TextureName(mut texnum: libc::c_uint)
 -> *const libc::c_char {
    return (*R_GetTexture(texnum)).name.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn GL_TextureData(mut texnum: libc::c_uint)
 -> *const byte {
    let mut pic: *mut rgbdata_t = (*R_GetTexture(texnum)).original;
    if !pic.is_null() { return (*pic).buffer }
    return 0 as *const byte;
}
#[no_mangle]
pub unsafe extern "C" fn Mod_BrushUnloadTextures(mut mod_0: *mut model_t) {
    let mut i: libc::c_int = 0;
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"Unloading world\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
    tr.map_unload = true_0;
    i = 0 as libc::c_int;
    while i < (*mod_0).numtextures {
        let mut tx: *mut texture_t = *(*mod_0).textures.offset(i as isize);
        // luma texture
        if !(tx.is_null() || (*tx).gl_texturenum == tr.defaultTexture)
           { // free slot
            GL_FreeTexture((*tx).gl_texturenum as
                               libc::c_uint); // main texture
            GL_FreeTexture((*tx).fb_texturenum as libc::c_uint);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Mod_UnloadTextures(mut mod_0: *mut model_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if mod_0.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_context.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 282 as
                                                                     libc::c_int);
    }
    match (*mod_0).type_0 as libc::c_int {
        3 => { }
        2 => { }
        0 => { Mod_BrushUnloadTextures(mod_0); }
        1 => { Mod_SpriteUnloadTextures((*mod_0).cache.data); }
        _ => {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"Mod_UnloadModel: unsupported type %d\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*mod_0).type_0
                                                                         as
                                                                         libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_ProcessEntData(mut allocate: qboolean) { }
// stubs
#[no_mangle]
pub unsafe extern "C" fn GL_SetTexCoordArrayMode(mut mode: uint) { }
#[no_mangle]
pub unsafe extern "C" fn GL_BackendStartFrame() { }
#[no_mangle]
pub unsafe extern "C" fn GL_BackendEndFrame() { }
#[no_mangle]
pub unsafe extern "C" fn GL_SetRenderMode(mut mode: libc::c_int) {
    vid.rendermode = mode;
    // / TODO: table shading/blending???
	// / maybe, setup block drawing function pointers here
}
#[no_mangle]
pub unsafe extern "C" fn R_ShowTextures() {
    // textures undone too
}
#[no_mangle]
pub unsafe extern "C" fn R_ShowTree() {
    // do we really need this here???
}
#[no_mangle]
pub unsafe extern "C" fn R_SetupSky(mut skyboxname: *const libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn VID_ScreenShot(mut filename: *const libc::c_char,
                                        mut shot_type: libc::c_int)
 -> qboolean {
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn VID_CubemapShot(mut base: *const libc::c_char,
                                         mut size: uint,
                                         mut vieworg: *const libc::c_float,
                                         mut skyshot: qboolean) -> qboolean {
    // cubemaps? in my softrender???
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_InitSkyClouds(mut mt: *mut mip_t,
                                         mut tx: *mut texture_t,
                                         mut custom_palette: qboolean) {
}
#[no_mangle]
pub unsafe extern "C" fn GL_SubdivideSurface(mut fa: *mut msurface_t) { }
#[no_mangle]
pub unsafe extern "C" fn DrawSingleDecal(mut pDecal: *mut decal_t,
                                         mut fa: *mut msurface_t) {
}
#[no_mangle]
pub unsafe extern "C" fn GL_SelectTexture(mut texture: libc::c_int) { }
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTexMatrixExt(mut glmatrix:
                                                 *const libc::c_float) {
}
#[no_mangle]
pub unsafe extern "C" fn GL_LoadIdentityTexMatrix() { }
#[no_mangle]
pub unsafe extern "C" fn GL_CleanUpTextureUnits(mut last: libc::c_int) { }
#[no_mangle]
pub unsafe extern "C" fn GL_TexGen(mut coord: libc::c_uint,
                                   mut mode: libc::c_uint) {
}
#[no_mangle]
pub unsafe extern "C" fn GL_TextureTarget(mut target: uint) { }
#[no_mangle]
pub unsafe extern "C" fn GL_BuildLightmaps() { CL_RunLightStyles(); }
#[no_mangle]
pub unsafe extern "C" fn Mod_SetOrthoBounds(mut mins: *const libc::c_float,
                                            mut maxs: *const libc::c_float) {
}
#[no_mangle]
pub unsafe extern "C" fn R_SpeedsMessage(mut out: *mut libc::c_char,
                                         mut size: size_t) -> qboolean {
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Mod_GetCurrentVis() -> *mut byte {
    return 0 as *mut byte;
}
#[no_mangle]
pub unsafe extern "C" fn R_GetConfigName() -> *const libc::c_char {
    return b"ref_soft\x00" as *const u8 as *const libc::c_char;
    // software specific cvars will go to ref_soft.cfg
}
unsafe extern "C" fn R_GetProcAddress(mut name: *const libc::c_char)
 -> *mut libc::c_void {
    return gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(name);
}
#[no_mangle]
pub static mut gReffuncs: ref_interface_t =
    unsafe {
        {
            let mut init =
                ref_interface_s{R_Init:
                                    Some(R_Init as
                                             unsafe extern "C" fn()
                                                 -> qboolean),
                                R_Shutdown:
                                    Some(R_Shutdown as
                                             unsafe extern "C" fn() -> ()),
                                R_GetConfigName:
                                    Some(R_GetConfigName as
                                             unsafe extern "C" fn()
                                                 -> *const libc::c_char),
                                R_SetDisplayTransform:
                                    Some(R_SetDisplayTransform as
                                             unsafe extern "C" fn(_:
                                                                      ref_screen_rotation_t,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float)
                                                 -> qboolean),
                                GL_SetupAttributes:
                                    Some(GL_SetupAttributes as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                GL_InitExtensions:
                                    Some(GL_InitExtensions as
                                             unsafe extern "C" fn() -> ()),
                                GL_ClearExtensions:
                                    Some(GL_ClearExtensions as
                                             unsafe extern "C" fn() -> ()),
                                R_BeginFrame:
                                    Some(R_BeginFrame as
                                             unsafe extern "C" fn(_: qboolean)
                                                 -> ()),
                                R_RenderScene:
                                    Some(R_RenderScene as
                                             unsafe extern "C" fn() -> ()),
                                R_EndFrame:
                                    Some(R_EndFrame as
                                             unsafe extern "C" fn() -> ()),
                                R_PushScene:
                                    Some(R_PushScene as
                                             unsafe extern "C" fn() -> ()),
                                R_PopScene:
                                    Some(R_PopScene as
                                             unsafe extern "C" fn() -> ()),
                                GL_BackendStartFrame:
                                    Some(GL_BackendStartFrame as
                                             unsafe extern "C" fn() -> ()),
                                GL_BackendEndFrame:
                                    Some(GL_BackendEndFrame as
                                             unsafe extern "C" fn() -> ()),
                                R_ClearScreen:
                                    Some(R_ClearScreen as
                                             unsafe extern "C" fn() -> ()),
                                R_AllowFog:
                                    Some(R_AllowFog as
                                             unsafe extern "C" fn(_: qboolean)
                                                 -> ()),
                                GL_SetRenderMode:
                                    Some(GL_SetRenderMode as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                R_AddEntity:
                                    Some(R_AddEntity as
                                             unsafe extern "C" fn(_:
                                                                      *mut cl_entity_s,
                                                                  _:
                                                                      libc::c_int)
                                                 -> qboolean),
                                CL_AddCustomBeam:
                                    Some(CL_AddCustomBeam as
                                             unsafe extern "C" fn(_:
                                                                      *mut cl_entity_t)
                                                 -> ()),
                                R_ProcessEntData:
                                    Some(R_ProcessEntData as
                                             unsafe extern "C" fn(_: qboolean)
                                                 -> ()),
                                R_ShowTextures:
                                    Some(R_ShowTextures as
                                             unsafe extern "C" fn() -> ()),
                                R_GetTextureOriginalBuffer:
                                    Some(R_GetTextureOriginalBuffer as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_uint)
                                                 -> *const byte),
                                GL_LoadTextureFromBuffer:
                                    Some(GL_LoadTextureFromBuffer as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      *mut rgbdata_t,
                                                                  _:
                                                                      texFlags_t,
                                                                  _: qboolean)
                                                 -> libc::c_int),
                                GL_ProcessTexture:
                                    Some(GL_ProcessTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                R_SetupSky:
                                    Some(R_SetupSky as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char)
                                                 -> ()),
                                R_Set2DMode:
                                    Some(R_Set2DMode as
                                             unsafe extern "C" fn(_: qboolean)
                                                 -> ()),
                                R_DrawStretchRaw:
                                    Some(R_DrawStretchRaw as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const byte,
                                                                  _: qboolean)
                                                 -> ()),
                                R_DrawStretchPic:
                                    Some(R_DrawStretchPic as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                R_DrawTileClear:
                                    Some(R_DrawTileClear as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                FillRGBA:
                                    Some(CL_FillRGBA as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                FillRGBABlend:
                                    Some(CL_FillRGBABlend as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                WorldToScreen:
                                    Some(R_WorldToScreen as
                                             unsafe extern "C" fn(_:
                                                                      *const vec_t,
                                                                  _:
                                                                      *mut vec_t)
                                                 -> libc::c_int),
                                VID_ScreenShot:
                                    Some(VID_ScreenShot as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      libc::c_int)
                                                 -> qboolean),
                                VID_CubemapShot:
                                    Some(VID_CubemapShot as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: uint,
                                                                  _:
                                                                      *const libc::c_float,
                                                                  _: qboolean)
                                                 -> qboolean),
                                R_LightPoint:
                                    Some(R_LightPoint as
                                             unsafe extern "C" fn(_:
                                                                      *const vec_t)
                                                 -> colorVec),
                                R_DecalShoot:
                                    Some(R_DecalShoot as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut vec_t,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_float)
                                                 -> ()),
                                R_DecalRemoveAll:
                                    Some(R_DecalRemoveAll as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                R_CreateDecalList:
                                    Some(R_CreateDecalList as
                                             unsafe extern "C" fn(_:
                                                                      *mut decallist_t)
                                                 -> libc::c_int),
                                R_ClearAllDecals:
                                    Some(R_ClearAllDecals as
                                             unsafe extern "C" fn() -> ()),
                                R_StudioEstimateFrame:
                                    Some(R_StudioEstimateFrame as
                                             unsafe extern "C" fn(_:
                                                                      *mut cl_entity_t,
                                                                  _:
                                                                      *mut mstudioseqdesc_t)
                                                 -> libc::c_float),
                                R_StudioLerpMovement:
                                    Some(R_StudioLerpMovement as
                                             unsafe extern "C" fn(_:
                                                                      *mut cl_entity_t,
                                                                  _:
                                                                      libc::c_double,
                                                                  _:
                                                                      *mut vec_t,
                                                                  _:
                                                                      *mut vec_t)
                                                 -> ()),
                                CL_InitStudioAPI:
                                    Some(CL_InitStudioAPI as
                                             unsafe extern "C" fn() -> ()),
                                R_InitSkyClouds:
                                    Some(R_InitSkyClouds as
                                             unsafe extern "C" fn(_:
                                                                      *mut mip_s,
                                                                  _:
                                                                      *mut texture_s,
                                                                  _: qboolean)
                                                 -> ()),
                                GL_SubdivideSurface:
                                    Some(GL_SubdivideSurface as
                                             unsafe extern "C" fn(_:
                                                                      *mut msurface_t)
                                                 -> ()),
                                CL_RunLightStyles:
                                    Some(CL_RunLightStyles as
                                             unsafe extern "C" fn() -> ()),
                                R_GetSpriteParms:
                                    Some(R_GetSpriteParms as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_int,
                                                                  _:
                                                                      *mut libc::c_int,
                                                                  _:
                                                                      *mut libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const model_s)
                                                 -> ()),
                                R_GetSpriteTexture:
                                    Some(R_GetSpriteTexture as
                                             unsafe extern "C" fn(_:
                                                                      *const model_s,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                Mod_LoadMapSprite:
                                    Some(Mod_LoadMapSprite as
                                             unsafe extern "C" fn(_:
                                                                      *mut model_s,
                                                                  _:
                                                                      *const libc::c_void,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut qboolean)
                                                 -> ()),
                                Mod_ProcessRenderData:
                                    Some(Mod_ProcessRenderData as
                                             unsafe extern "C" fn(_:
                                                                      *mut model_t,
                                                                  _: qboolean,
                                                                  _:
                                                                      *const byte)
                                                 -> qboolean),
                                Mod_StudioLoadTextures:
                                    Some(Mod_StudioLoadTextures as
                                             unsafe extern "C" fn(_:
                                                                      *mut model_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                CL_DrawParticles:
                                    Some(CL_DrawParticles as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_double,
                                                                  _:
                                                                      *mut particle_t,
                                                                  _:
                                                                      libc::c_float)
                                                 -> ()),
                                CL_DrawTracers:
                                    Some(CL_DrawTracers as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_double,
                                                                  _:
                                                                      *mut particle_t)
                                                 -> ()),
                                CL_DrawBeams:
                                    Some(CL_DrawBeams as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut BEAM)
                                                 -> ()),
                                R_BeamCull:
                                    Some(R_BeamCull as
                                             unsafe extern "C" fn(_:
                                                                      *const vec_t,
                                                                  _:
                                                                      *const vec_t,
                                                                  _: qboolean)
                                                 -> qboolean),
                                RefGetParm:
                                    Some(GL_RefGetParm as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                GetDetailScaleForTexture:
                                    Some(R_GetDetailScaleForTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_float,
                                                                  _:
                                                                      *mut libc::c_float)
                                                 -> ()),
                                GetExtraParmsForTexture:
                                    Some(R_GetExtraParmsForTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut byte,
                                                                  _:
                                                                      *mut byte,
                                                                  _:
                                                                      *mut byte,
                                                                  _:
                                                                      *mut byte)
                                                 -> ()),
                                GetFrameTime:
                                    Some(R_GetFrameTime as
                                             unsafe extern "C" fn()
                                                 -> libc::c_float),
                                R_SetCurrentEntity:
                                    Some(R_SetCurrentEntity as
                                             unsafe extern "C" fn(_:
                                                                      *mut cl_entity_t)
                                                 -> ()),
                                R_SetCurrentModel:
                                    Some(R_SetCurrentModel as
                                             unsafe extern "C" fn(_:
                                                                      *mut model_t)
                                                 -> ()),
                                GL_FindTexture:
                                    Some(GL_FindTexture as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char)
                                                 -> libc::c_int),
                                GL_TextureName:
                                    Some(GL_TextureName as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_uint)
                                                 -> *const libc::c_char),
                                GL_TextureData:
                                    Some(GL_TextureData as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_uint)
                                                 -> *const byte),
                                GL_LoadTexture:
                                    Some(GL_LoadTexture as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      *const byte,
                                                                  _: size_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                GL_CreateTexture:
                                    Some(GL_CreateTexture as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const libc::c_void,
                                                                  _:
                                                                      texFlags_t)
                                                 -> libc::c_int),
                                GL_LoadTextureArray:
                                    Some(GL_LoadTextureArray as
                                             unsafe extern "C" fn(_:
                                                                      *mut *const libc::c_char,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                GL_CreateTextureArray:
                                    Some(GL_CreateTextureArray as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const libc::c_void,
                                                                  _:
                                                                      texFlags_t)
                                                 -> libc::c_int),
                                GL_FreeTexture:
                                    Some(GL_FreeTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_uint)
                                                 -> ()),
                                DrawSingleDecal:
                                    Some(DrawSingleDecal as
                                             unsafe extern "C" fn(_:
                                                                      *mut decal_t,
                                                                  _:
                                                                      *mut msurface_t)
                                                 -> ()),
                                R_DecalSetupVerts:
                                    Some(R_DecalSetupVerts as
                                             unsafe extern "C" fn(_:
                                                                      *mut decal_t,
                                                                  _:
                                                                      *mut msurface_t,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> *mut libc::c_float),
                                R_EntityRemoveDecals:
                                    Some(R_EntityRemoveDecals as
                                             unsafe extern "C" fn(_:
                                                                      *mut model_t)
                                                 -> ()),
                                AVI_UploadRawFrame:
                                    Some(R_UploadStretchRaw as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const byte)
                                                 -> ()),
                                GL_Bind:
                                    Some(GL_Bind as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_uint)
                                                 -> ()),
                                GL_SelectTexture:
                                    Some(GL_SelectTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                GL_LoadTextureMatrix:
                                    Some(GL_LoadTexMatrixExt as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_float)
                                                 -> ()),
                                GL_TexMatrixIdentity:
                                    Some(GL_LoadIdentityTexMatrix as
                                             unsafe extern "C" fn() -> ()),
                                GL_CleanUpTextureUnits:
                                    Some(GL_CleanUpTextureUnits as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                GL_TexGen:
                                    Some(GL_TexGen as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_uint,
                                                                  _:
                                                                      libc::c_uint)
                                                 -> ()),
                                GL_TextureTarget:
                                    Some(GL_TextureTarget as
                                             unsafe extern "C" fn(_: uint)
                                                 -> ()),
                                GL_TexCoordArrayMode:
                                    Some(GL_SetTexCoordArrayMode as
                                             unsafe extern "C" fn(_: uint)
                                                 -> ()),
                                GL_UpdateTexSize:
                                    Some(GL_UpdateTexSize as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                GL_Reserved0: None,
                                GL_Reserved1: None,
                                GL_DrawParticles:
                                    Some(CL_DrawParticlesExternal as
                                             unsafe extern "C" fn(_:
                                                                      *const ref_viewpass_t,
                                                                  _: qboolean,
                                                                  _:
                                                                      libc::c_float)
                                                 -> ()),
                                LightVec:
                                    Some(R_LightVec as
                                             unsafe extern "C" fn(_:
                                                                      *const vec_t,
                                                                  _:
                                                                      *const vec_t,
                                                                  _:
                                                                      *mut vec_t,
                                                                  _:
                                                                      *mut vec_t)
                                                 -> colorVec),
                                StudioGetTexture:
                                    Some(R_StudioGetTexture as
                                             unsafe extern "C" fn(_:
                                                                      *mut cl_entity_t)
                                                 -> *mut mstudiotex_s),
                                GL_RenderFrame:
                                    Some(R_RenderFrame as
                                             unsafe extern "C" fn(_:
                                                                      *const ref_viewpass_s)
                                                 -> ()),
                                GL_OrthoBounds:
                                    Some(Mod_SetOrthoBounds as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_float,
                                                                  _:
                                                                      *const libc::c_float)
                                                 -> ()),
                                R_SpeedsMessage:
                                    Some(R_SpeedsMessage as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_char,
                                                                  _: size_t)
                                                 -> qboolean),
                                Mod_GetCurrentVis:
                                    Some(Mod_GetCurrentVis as
                                             unsafe extern "C" fn()
                                                 -> *mut byte),
                                R_NewMap:
                                    Some(R_NewMap as
                                             unsafe extern "C" fn() -> ()),
                                R_ClearScene:
                                    Some(R_ClearScene as
                                             unsafe extern "C" fn() -> ()),
                                R_GetProcAddress:
                                    Some(R_GetProcAddress as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char)
                                                 -> *mut libc::c_void),
                                TriRenderMode:
                                    Some(TriRenderMode as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                Begin:
                                    Some(TriBegin as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                End:
                                    Some(TriEnd as
                                             unsafe extern "C" fn() -> ()),
                                Color4f:
                                    Some(_TriColor4f as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float)
                                                 -> ()),
                                Color4ub:
                                    Some(_TriColor4ub as
                                             unsafe extern "C" fn(_: byte,
                                                                  _: byte,
                                                                  _: byte,
                                                                  _: byte)
                                                 -> ()),
                                TexCoord2f:
                                    Some(TriTexCoord2f as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float)
                                                 -> ()),
                                Vertex3fv:
                                    Some(TriVertex3fv as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_float)
                                                 -> ()),
                                Vertex3f:
                                    Some(TriVertex3f as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float)
                                                 -> ()),
                                Fog:
                                    Some(TriFog as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                ScreenToWorld:
                                    Some(R_ScreenToWorld as
                                             unsafe extern "C" fn(_:
                                                                      *const vec_t,
                                                                  _:
                                                                      *mut vec_t)
                                                 -> ()),
                                GetMatrix:
                                    Some(TriGetMatrix as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_float)
                                                 -> ()),
                                FogParams:
                                    Some(TriFogParams as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_float,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                CullFace:
                                    Some(TriCullFace as
                                             unsafe extern "C" fn(_:
                                                                      TRICULLSTYLE)
                                                 -> ()),
                                VGUI_DrawInit:
                                    Some(VGUI_DrawInit as
                                             unsafe extern "C" fn() -> ()),
                                VGUI_DrawShutdown:
                                    Some(VGUI_DrawShutdown as
                                             unsafe extern "C" fn() -> ()),
                                VGUI_SetupDrawingText:
                                    Some(VGUI_SetupDrawingText as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_int)
                                                 -> ()),
                                VGUI_SetupDrawingRect:
                                    Some(VGUI_SetupDrawingRect as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_int)
                                                 -> ()),
                                VGUI_SetupDrawingImage:
                                    Some(VGUI_SetupDrawingImage as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_int)
                                                 -> ()),
                                VGUI_BindTexture:
                                    Some(VGUI_BindTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> ()),
                                VGUI_EnableTexture:
                                    Some(VGUI_EnableTexture as
                                             unsafe extern "C" fn(_: qboolean)
                                                 -> ()),
                                VGUI_CreateTexture:
                                    Some(VGUI_CreateTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                VGUI_UploadTexture:
                                    Some(VGUI_UploadTexture as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                VGUI_UploadTextureBlock:
                                    Some(VGUI_UploadTextureBlock as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *const byte,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()),
                                VGUI_DrawQuad:
                                    Some(VGUI_DrawQuad as
                                             unsafe extern "C" fn(_:
                                                                      *const vpoint_t,
                                                                  _:
                                                                      *const vpoint_t)
                                                 -> ()),
                                VGUI_GetTextureSizes:
                                    Some(VGUI_GetTextureSizes as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_int,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> ()),
                                VGUI_GenerateTexture:
                                    Some(VGUI_GenerateTexture as
                                             unsafe extern "C" fn()
                                                 -> libc::c_int),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn GetRefAPI(mut version: libc::c_int,
                                   mut funcs: *mut ref_interface_t,
                                   mut engfuncs: *mut ref_api_t,
                                   mut globals: *mut ref_globals_t)
 -> libc::c_int {
    if version != 1 as libc::c_int { return 0 as libc::c_int }
    // fill in our callbacks
    memcpy(funcs as *mut libc::c_void,
           &mut gReffuncs as *mut ref_interface_t as *const libc::c_void,
           ::std::mem::size_of::<ref_interface_t>() as libc::c_ulong);
    memcpy(&mut gEngfuncs as *mut ref_api_t as *mut libc::c_void,
           engfuncs as *const libc::c_void,
           ::std::mem::size_of::<ref_api_t>() as libc::c_ulong);
    gpGlobals = globals;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetRefHumanReadableName(mut out: *mut libc::c_char,
                                                 mut size: size_t) {
    Q_strncpy(out, b"Software\x00" as *const u8 as *const libc::c_char, size);
}
