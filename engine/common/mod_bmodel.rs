#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn ceilf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strnlwr(in_0: *const libc::c_char, out: *mut libc::c_char,
                 size_out: size_t);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_toupper(in_0: libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atov(vec: *mut libc::c_float, str: *const libc::c_char, siz: size_t);
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn COM_ReplaceExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Realloc(poolptr: poolhandle_t, memptr: *mut libc::c_void,
                    size: size_t, clear: qboolean,
                    filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_AllocPool(name: *const libc::c_char,
                      filename: *const libc::c_char, fileline: libc::c_int)
     -> poolhandle_t;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_GetDiskPath(name: *const libc::c_char, gamedironly: qboolean)
     -> *const libc::c_char;
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn COM_CompareFileTime(filename1: *const libc::c_char,
                           filename2: *const libc::c_char,
                           iCompare: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Write(file: *mut file_t, data: *const libc::c_void,
                datasize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Seek(file: *mut file_t, offset: fs_offset_t, whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileTime(filename: *const libc::c_char, gamedironly: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Tell(file: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Host_IsQuakeCompatible() -> qboolean;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut loadmodel: *mut model_t;
    #[no_mangle]
    static mut r_wadtextures: *mut convar_t;
    #[no_mangle]
    fn Mod_FindName(name: *const libc::c_char, trackCRC: qboolean)
     -> *mut model_t;
    #[no_mangle]
    fn Mod_InitDebugHulls();
    #[no_mangle]
    fn BoxOnPlaneSide(emins: *const vec_t, emaxs: *const vec_t,
                      p: *const mplane_t) -> libc::c_int;
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn AddPointToBounds(v: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn PlaneTypeForNormal(normal: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn RadiusFromBounds(mins: *const vec_t, maxs: *const vec_t)
     -> libc::c_float;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed = 1;
pub const HOST_NORMAL: C2RustUnnamed = 0;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec2_t = [vec_t; 2];
pub type vec3_t = [vec_t; 3];
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
pub type fs_offset_t = off_t;
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
pub struct dlump_t {
    pub fileofs: libc::c_int,
    pub filelen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dheader_t {
    pub version: libc::c_int,
    pub lumps: [dlump_t; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dextrahdr_t {
    pub id: libc::c_int,
    pub version: libc::c_int,
    pub lumps: [dlump_t; 12],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmiptexlump_t {
    pub nummiptex: libc::c_int,
    pub dataofs: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvertex_t {
    pub point: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dplane_t {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
    pub mins: [libc::c_short; 3],
    pub maxs: [libc::c_short; 3],
    pub firstface: word,
    pub numfaces: word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnode32_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_int; 2],
    pub mins: [libc::c_float; 3],
    pub maxs: [libc::c_float; 3],
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dleaf_t {
    pub contents: libc::c_int,
    pub visofs: libc::c_int,
    pub mins: [libc::c_short; 3],
    pub maxs: [libc::c_short; 3],
    pub firstmarksurface: word,
    pub nummarksurfaces: word,
    pub ambient_level: [byte; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dleaf32_t {
    pub contents: libc::c_int,
    pub visofs: libc::c_int,
    pub mins: [libc::c_float; 3],
    pub maxs: [libc::c_float; 3],
    pub firstmarksurface: libc::c_int,
    pub nummarksurfaces: libc::c_int,
    pub ambient_level: [byte; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dclipnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dclipnode32_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtexinfo_t {
    pub vecs: [[libc::c_float; 4]; 2],
    pub miptex: libc::c_int,
    pub flags: libc::c_short,
    pub faceinfo: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfaceinfo_t {
    pub landname: [libc::c_char; 16],
    pub texture_step: libc::c_ushort,
    pub max_extent: libc::c_ushort,
    pub groupid: libc::c_short,
}
pub type dmarkface_t = word;
pub type dmarkface32_t = libc::c_int;
pub type dsurfedge_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dedge_t {
    pub v: [word; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dedge32_t {
    pub v: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dface_t {
    pub planenum: word,
    pub side: libc::c_short,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_short,
    pub texinfo: libc::c_short,
    pub styles: [byte; 4],
    pub lightofs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dface32_t {
    pub planenum: libc::c_int,
    pub side: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub texinfo: libc::c_int,
    pub styles: [byte; 4],
    pub lightofs: libc::c_int,
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
pub struct convar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut convar_s,
    pub desc: *mut libc::c_char,
    pub def_string: *mut libc::c_char,
}
pub type convar_t = convar_s;
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_status_t {
    pub curstate: host_state_t,
    pub nextstate: host_state_t,
    pub levelName: [libc::c_char; 64],
    pub landmarkName: [libc::c_char; 64],
    pub backgroundMap: qboolean,
    pub loadGame: qboolean,
    pub newGame: qboolean,
}
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_redirect_s {
    pub target: rdtype_t,
    pub buffer: *mut libc::c_char,
    pub buffersize: size_t,
    pub address: netadr_t,
    pub flush: Option<unsafe extern "C" fn(_: netadr_t, _: rdtype_t,
                                           _: *mut libc::c_char) -> ()>,
    pub lines: libc::c_int,
}
pub type host_redirect_t = host_redirect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_parm_s {
    pub hInst: HINSTANCE,
    pub hMutex: HANDLE,
    pub status: host_status_t,
    pub game: game_status_t,
    pub type_0: uint,
    pub abortframe: jmp_buf,
    pub errorframe: dword,
    pub mempool: poolhandle_t,
    pub finalmsg: string,
    pub downloadfile: string,
    pub downloadcount: libc::c_int,
    pub deferred_cmd: [libc::c_char; 128],
    pub rd: host_redirect_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub realframetime: libc::c_double,
    pub framecount: uint,
    pub draw_decals: [[libc::c_char; 64]; 512],
    pub player_mins: [vec3_t; 4],
    pub player_maxs: [vec3_t; 4],
    pub hWnd: *mut libc::c_void,
    pub allow_console: qboolean,
    pub allow_console_init: qboolean,
    pub key_overstrike: qboolean,
    pub stuffcmds_pending: qboolean,
    pub allow_cheats: qboolean,
    pub con_showalways: qboolean,
    pub change_game: qboolean,
    pub mouse_visible: qboolean,
    pub shutdown_issued: qboolean,
    pub force_draw_version: qboolean,
    pub force_draw_version_time: libc::c_float,
    pub apply_game_config: qboolean,
    pub apply_opengl_config: qboolean,
    pub config_executed: qboolean,
    pub sv_cvars_restored: libc::c_int,
    pub crashed: qboolean,
    pub daemonized: qboolean,
    pub enabledll: qboolean,
    pub textmode: qboolean,
    pub userinfo_changed: qboolean,
    pub movevars_changed: qboolean,
    pub renderinfo_changed: qboolean,
    pub rootdir: [libc::c_char; 260],
    pub rodir: [libc::c_char; 260],
    pub gamefolder: [libc::c_char; 64],
    pub imagepool: poolhandle_t,
    pub soundpool: poolhandle_t,
    pub features: uint,
    pub window_center_x: libc::c_int,
    pub window_center_y: libc::c_int,
    pub decalList: *mut decallist_s,
    pub numdecals: libc::c_int,
}
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
pub type modelstate_t = modelstate_s;
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
pub type host_parm_t = host_parm_s;
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
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
pub type cl_entity_t = cl_entity_s;
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
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
pub type ref_screen_rotation_t = ref_screen_rotation_e;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mip_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
}
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
pub type world_static_t = world_static_s;
/*
mod_bmodel.c - loading & handling world and brushmodels
Copyright (C) 2016 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// LUMP_ error codes
pub type wadlist_t = wadlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wadlist_s {
    pub wadnames: [[libc::c_char; 32]; 256],
    pub wadusage: [libc::c_int; 256],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbspmodel_t {
    pub submodels: *mut dmodel_t,
    pub numsubmodels: size_t,
    pub vertexes: *mut dvertex_t,
    pub numvertexes: size_t,
    pub planes: *mut dplane_t,
    pub numplanes: size_t,
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub numnodes: size_t,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
    pub numleafs: size_t,
    pub c2rust_unnamed_1: C2RustUnnamed_3,
    pub numclipnodes: size_t,
    pub texinfo: *mut dtexinfo_t,
    pub numtexinfo: size_t,
    pub c2rust_unnamed_2: C2RustUnnamed_2,
    pub nummarkfaces: size_t,
    pub surfedges: *mut dsurfedge_t,
    pub numsurfedges: size_t,
    pub c2rust_unnamed_3: C2RustUnnamed_1,
    pub numedges: size_t,
    pub c2rust_unnamed_4: C2RustUnnamed_0,
    pub numsurfaces: size_t,
    pub faceinfo: *mut dfaceinfo_t,
    pub numfaceinfo: size_t,
    pub visdata: *mut byte,
    pub visdatasize: size_t,
    pub lightdata: *mut byte,
    pub lightdatasize: size_t,
    pub deluxdata: *mut byte,
    pub deluxdatasize: size_t,
    pub shadowdata: *mut byte,
    pub shadowdatasize: size_t,
    pub entdata: *mut byte,
    pub entdatasize: size_t,
    pub textures: *mut dmiptexlump_t,
    pub texdatasize: size_t,
    pub deluxedata_out: *mut color24,
    pub shadowdata_out: *mut byte,
    pub clipnodes_out: *mut dclipnode32_t,
    pub wadlist: wadlist_t,
    pub lightmap_samples: libc::c_int,
    pub version: libc::c_int,
    pub isworld: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub surfaces: *mut dface_t,
    pub surfaces32: *mut dface32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub edges: *mut dedge_t,
    pub edges32: *mut dedge32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub markfaces: *mut dmarkface_t,
    pub markfaces32: *mut dmarkface32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub clipnodes: *mut dclipnode_t,
    pub clipnodes32: *mut dclipnode32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub leafs: *mut dleaf_t,
    pub leafs32: *mut dleaf32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub nodes: *mut dnode_t,
    pub nodes32: *mut dnode32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_state_s {
    pub initialized: qboolean,
    pub hInstance: HINSTANCE,
    pub dllFuncs: ref_interface_t,
    pub numRenderers: libc::c_int,
    pub shortNames: [string; 5],
    pub readableNames: [string; 5],
}
pub type mip_t = mip_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loadstat_t {
    pub name: [libc::c_char; 64],
    pub numerrors: libc::c_int,
    pub numwarnings: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlumpstat_t {
    pub lumpname: *const libc::c_char,
    pub entrysize: size_t,
    pub maxcount: size_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlumpinfo_t {
    pub lumpnumber: libc::c_int,
    pub mincount: size_t,
    pub maxcount: size_t,
    pub entrysize: libc::c_int,
    pub entrysize32: libc::c_int,
    pub loadname: *const libc::c_char,
    pub flags: libc::c_int,
    pub dataptr: *mut *const libc::c_void,
    pub count: *mut size_t,
}
pub type leaflist_t = leaflist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct leaflist_s {
    pub count: libc::c_int,
    pub maxcount: libc::c_int,
    pub overflowed: qboolean,
    pub list: *mut libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub topnode: libc::c_int,
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_float) -> libc::c_float {
    return floorf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_ceil(mut __x: libc::c_float) -> libc::c_float {
    return ceilf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[no_mangle]
pub static mut world: world_static_t =
    world_static_t{loading: false_0,
                   flags: 0,
                   message: [0; 2048],
                   compiler: [0; 256],
                   generator: [0; 256],
                   hull_models: 0 as *const hull_model_t as *mut hull_model_t,
                   num_hull_models: 0,
                   deluxedata: 0 as *const color24 as *mut color24,
                   shadowdata: 0 as *const byte as *mut byte,
                   visbytes: 0,
                   fatbytes: 0,
                   mins: [0.; 3],
                   maxs: [0.; 3],
                   size: [0.; 3],
                   recursion_level: 0,
                   max_recursion: 0,};
static mut srcmodel: dbspmodel_t =
    dbspmodel_t{submodels: 0 as *const dmodel_t as *mut dmodel_t,
                numsubmodels: 0,
                vertexes: 0 as *const dvertex_t as *mut dvertex_t,
                numvertexes: 0,
                planes: 0 as *const dplane_t as *mut dplane_t,
                numplanes: 0,
                c2rust_unnamed:
                    C2RustUnnamed_5{nodes:
                                        0 as *const dnode_t as *mut dnode_t,},
                numnodes: 0,
                c2rust_unnamed_0:
                    C2RustUnnamed_4{leafs:
                                        0 as *const dleaf_t as *mut dleaf_t,},
                numleafs: 0,
                c2rust_unnamed_1:
                    C2RustUnnamed_3{clipnodes:
                                        0 as *const dclipnode_t as
                                            *mut dclipnode_t,},
                numclipnodes: 0,
                texinfo: 0 as *const dtexinfo_t as *mut dtexinfo_t,
                numtexinfo: 0,
                c2rust_unnamed_2:
                    C2RustUnnamed_2{markfaces:
                                        0 as *const dmarkface_t as
                                            *mut dmarkface_t,},
                nummarkfaces: 0,
                surfedges: 0 as *const dsurfedge_t as *mut dsurfedge_t,
                numsurfedges: 0,
                c2rust_unnamed_3:
                    C2RustUnnamed_1{edges:
                                        0 as *const dedge_t as *mut dedge_t,},
                numedges: 0,
                c2rust_unnamed_4:
                    C2RustUnnamed_0{surfaces:
                                        0 as *const dface_t as *mut dface_t,},
                numsurfaces: 0,
                faceinfo: 0 as *const dfaceinfo_t as *mut dfaceinfo_t,
                numfaceinfo: 0,
                visdata: 0 as *const byte as *mut byte,
                visdatasize: 0,
                lightdata: 0 as *const byte as *mut byte,
                lightdatasize: 0,
                deluxdata: 0 as *const byte as *mut byte,
                deluxdatasize: 0,
                shadowdata: 0 as *const byte as *mut byte,
                shadowdatasize: 0,
                entdata: 0 as *const byte as *mut byte,
                entdatasize: 0,
                textures: 0 as *const dmiptexlump_t as *mut dmiptexlump_t,
                texdatasize: 0,
                deluxedata_out: 0 as *const color24 as *mut color24,
                shadowdata_out: 0 as *const byte as *mut byte,
                clipnodes_out:
                    0 as *const dclipnode32_t as *mut dclipnode32_t,
                wadlist:
                    wadlist_t{wadnames: [[0; 32]; 256],
                              wadusage: [0; 256],
                              count: 0,},
                lightmap_samples: 0,
                version: 0,
                isworld: false_0,};
static mut loadstat: loadstat_t =
    loadstat_t{name: [0; 64], numerrors: 0, numwarnings: 0,};
static mut worldmodel: *mut model_t = 0 as *const model_t as *mut model_t;
static mut g_visdata: [byte; 4096] = [0; 4096];
// for overflows where each leaf can't be stored individually
// intermediate buffer
static mut worldstats: [mlumpstat_t; 27] =
    [mlumpstat_t{lumpname: 0 as *const libc::c_char,
                 entrysize: 0,
                 maxcount: 0,
                 count: 0,}; 27];
// Initialized in run_static_initializers
static mut srclumps: [mlumpinfo_t; 15] =
    [mlumpinfo_t{lumpnumber: 0,
                 mincount: 0,
                 maxcount: 0,
                 entrysize: 0,
                 entrysize32: 0,
                 loadname: 0 as *const libc::c_char,
                 flags: 0,
                 dataptr: 0 as *mut *const libc::c_void,
                 count: 0 as *mut size_t,}; 15];
// Initialized in run_static_initializers
static mut extlumps: [mlumpinfo_t; 12] =
    [mlumpinfo_t{lumpnumber: 0,
                 mincount: 0,
                 maxcount: 0,
                 entrysize: 0,
                 entrysize32: 0,
                 loadname: 0 as *const libc::c_char,
                 flags: 0,
                 dataptr: 0 as *mut *const libc::c_void,
                 count: 0 as *mut size_t,}; 12];
/*
===============================================================================

			MAP PROCESSING

===============================================================================
*/
/*
=================
Mod_LoadLump

generic loader
=================
*/
unsafe extern "C" fn Mod_LoadLump(mut in_0: *const byte,
                                  mut info: *mut mlumpinfo_t,
                                  mut stat: *mut mlumpstat_t,
                                  mut flags: libc::c_int) {
    let mut version: libc::c_int = (*(in_0 as *mut dheader_t)).version;
    let mut numelems: size_t = 0;
    let mut real_entrysize: size_t = 0;
    let mut msg1: [libc::c_char; 32] = [0; 32];
    let mut msg2: [libc::c_char; 32] = [0; 32];
    let mut l: *mut dlump_t = 0 as *mut dlump_t;
    if (*info).flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           != 0 {
        let mut header: *mut dextrahdr_t =
            (in_0 as
                 *mut byte).offset(::std::mem::size_of::<dheader_t>() as
                                       libc::c_ulong as isize) as
                *mut dextrahdr_t;
        if (*header).id !=
               (('H' as i32) << 24 as libc::c_int) +
                   (('S' as i32) << 16 as libc::c_int) +
                   (('A' as i32) << 8 as libc::c_int) + 'X' as i32 ||
               (*header).version != 4 as libc::c_int {
            return
        }
        l =
            &mut *(*header).lumps.as_mut_ptr().offset((*info).lumpnumber as
                                                          isize) as
                *mut dlump_t
    } else {
        let mut header_0: *mut dheader_t = in_0 as *mut dheader_t;
        l =
            &mut *(*header_0).lumps.as_mut_ptr().offset((*info).lumpnumber as
                                                            isize) as
                *mut dlump_t
    }
    // lump is unused by engine for some reasons ?
    if l.is_null() || (*info).entrysize <= 0 as libc::c_int ||
           (*info).maxcount <= 0 as libc::c_int as libc::c_ulong {
        return
    } // default
    real_entrysize = (*info).entrysize as size_t;
    // analyze real entrysize
    if version ==
           ('B' as i32) << 0 as libc::c_int | ('S' as i32) << 8 as libc::c_int
               | ('P' as i32) << 16 as libc::c_int |
               ('2' as i32) << 24 as libc::c_int &&
           (*info).entrysize32 > 0 as libc::c_int {
        // always use alternate entrysize for BSP2
        real_entrysize = (*info).entrysize32 as size_t
    } else if (*info).lumpnumber == 9 as libc::c_int &&
                  version != 29 as libc::c_int {
        // never run this check for BSP29 because Arguire QBSP 'broken' clipnodes!
        if (*l).filelen % (*info).entrysize != 0 ||
               (*l).filelen / (*info).entrysize >= 32767 as libc::c_int {
            real_entrysize = (*info).entrysize32 as size_t;
            flags =
                (flags as libc::c_uint |
                     (1 as libc::c_uint) << 2 as libc::c_int) as libc::c_int
            // shut up warning
        }
    }
    // bmodels not required the visibility
    if flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int == 0 &&
           world.loading as u64 == 0 && (*info).lumpnumber == 4 as libc::c_int
       {
        flags =
            (flags as libc::c_uint | (1 as libc::c_uint) << 2 as libc::c_int)
                as libc::c_int
    } // shut up warning
    // fill the stats for world
    if flags as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        (*stat).lumpname = (*info).loadname; // first letter in cap
        (*stat).entrysize = real_entrysize;
        (*stat).maxcount = (*info).maxcount;
        if real_entrysize != 0 as libc::c_int as libc::c_ulong {
            (*stat).count =
                ((*l).filelen as libc::c_ulong).wrapping_div(real_entrysize)
        }
    }
    Q_strncpy(msg1.as_mut_ptr(), (*info).loadname,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy(msg2.as_mut_ptr(), (*info).loadname,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    msg2[0 as libc::c_int as usize] =
        Q_toupper(msg2[0 as libc::c_int as usize]);
    // lump is not present
    if (*l).filelen <= 0 as libc::c_int {
        // don't warn about extra lumps - it's optional
        if (*info).flags as libc::c_uint &
               (1 as libc::c_uint) << 1 as libc::c_int == 0 {
            // some data array that may be optional
            if real_entrysize ==
                   ::std::mem::size_of::<byte>() as libc::c_ulong {
                if flags as libc::c_uint &
                       (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                    Con_DPrintf(b"^3Warning:^7 map ^2%s^7 has no %s\n\x00" as
                                    *const u8 as *const libc::c_char,
                                loadstat.name.as_mut_ptr(),
                                msg1.as_mut_ptr());
                    loadstat.numwarnings += 1
                }
            } else if (*info).mincount > 0 as libc::c_int as libc::c_ulong {
                // it has the mincount and the lump is completely missed!
                if flags as libc::c_uint &
                       (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                    Con_DPrintf(b"^1Error:^7 map ^2%s^7 has no %s\n\x00" as
                                    *const u8 as *const libc::c_char,
                                loadstat.name.as_mut_ptr(),
                                msg1.as_mut_ptr());
                }
                loadstat.numerrors += 1
            }
        }
        return
    }
    if ((*l).filelen as libc::c_ulong).wrapping_rem(real_entrysize) != 0 {
        if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int ==
               0 {
            Con_DPrintf(b"^1Error:^7 Mod_Load%s: Lump size %d was not a multiple of %lu bytes\n\x00"
                            as *const u8 as *const libc::c_char,
                        msg2.as_mut_ptr(), (*l).filelen, real_entrysize);
        }
        loadstat.numerrors += 1;
        return
    }
    numelems = ((*l).filelen as libc::c_ulong).wrapping_div(real_entrysize);
    if numelems < (*info).mincount {
        // it has the mincount and it's smaller than this limit
        if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int ==
               0 {
            Con_DPrintf(b"^1Error:^7 map ^2%s^7 has no %s\n\x00" as *const u8
                            as *const libc::c_char,
                        loadstat.name.as_mut_ptr(), msg1.as_mut_ptr());
        }
        loadstat.numerrors += 1;
        return
    }
    if numelems > (*info).maxcount {
        // it has the maxcount and it's overflowed
        if (*info).flags as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int != 0 {
            if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
                   == 0 {
                Con_DPrintf(b"^1Error:^7 map ^2%s^7 has too many %s\n\x00" as
                                *const u8 as *const libc::c_char,
                            loadstat.name.as_mut_ptr(), msg1.as_mut_ptr());
            }
            loadstat.numerrors += 1;
            return
        } else {
            if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
                   == 0 {
                // just throw warning
                Con_DPrintf(b"^3Warning:^7 map ^2%s^7 has too many %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            loadstat.name.as_mut_ptr(),
                            msg1.as_mut_ptr()); // don't fill the intermediate struct
                loadstat.numwarnings += 1
            }
        }
    }
    if flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        return
    }
    // all checks are passed, store pointers
    if !(*info).dataptr.is_null() {
        *(*info).dataptr =
            in_0.offset((*l).fileofs as isize) as *mut libc::c_void
    }
    if !(*info).count.is_null() { *(*info).count = numelems };
}
/*
================
Mod_ArrayUsage
================
*/
unsafe extern "C" fn Mod_ArrayUsage(mut szItem: *const libc::c_char,
                                    mut items: libc::c_int,
                                    mut maxitems: libc::c_int,
                                    mut itemsize: libc::c_int)
 -> libc::c_int {
    let mut percentage: libc::c_float =
        if maxitems != 0 {
            (items as libc::c_float * 100.0f32) / maxitems as libc::c_float
        } else { 0.0f32 };
    Con_Printf(b"%-12s  %7i/%-7i  %8i/%-8i  (%4.1f%%) \x00" as *const u8 as
                   *const libc::c_char, szItem, items, maxitems,
               items * itemsize, maxitems * itemsize,
               percentage as libc::c_double);
    if percentage > 99.99f32 {
        Con_Printf(b"^1SIZE OVERFLOW!!!^7\n\x00" as *const u8 as
                       *const libc::c_char);
    } else if percentage > 95.0f32 {
        Con_Printf(b"^3SIZE DANGER!^7\n\x00" as *const u8 as
                       *const libc::c_char);
    } else if percentage > 80.0f32 {
        Con_Printf(b"^2VERY FULL!^7\n\x00" as *const u8 as
                       *const libc::c_char);
    } else { Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char); }
    return items * itemsize;
}
/*
================
Mod_GlobUsage
================
*/
unsafe extern "C" fn Mod_GlobUsage(mut szItem: *const libc::c_char,
                                   mut itemstorage: libc::c_int,
                                   mut maxstorage: libc::c_int)
 -> libc::c_int {
    let mut percentage: libc::c_float =
        if maxstorage != 0 {
            (itemstorage as libc::c_float * 100.0f32) /
                maxstorage as libc::c_float
        } else { 0.0f32 };
    Con_Printf(b"%-15s  %-12s  %8i/%-8i  (%4.1f%%) \x00" as *const u8 as
                   *const libc::c_char, szItem,
               b"[variable]\x00" as *const u8 as *const libc::c_char,
               itemstorage, maxstorage, percentage as libc::c_double);
    if percentage > 99.99f32 {
        Con_Printf(b"^1SIZE OVERFLOW!!!^7\n\x00" as *const u8 as
                       *const libc::c_char);
    } else if percentage > 95.0f32 {
        Con_Printf(b"^3SIZE DANGER!^7\n\x00" as *const u8 as
                       *const libc::c_char);
    } else if percentage > 80.0f32 {
        Con_Printf(b"^2VERY FULL!^7\n\x00" as *const u8 as
                       *const libc::c_char);
    } else { Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char); }
    return itemstorage;
}
/*
=============
Mod_PrintWorldStats_f

Dumps info about world
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_PrintWorldStats_f() {
    let mut i: libc::c_int = 0; // unused or lump is empty
    let mut totalmemory: libc::c_int = 0 as libc::c_int;
    let mut w: *mut model_t = worldmodel;
    if w.is_null() || (*w).numsubmodels == 0 {
        Con_Printf(b"No map loaded\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Con_Printf(b"Object names  Objects/Maxobjs  Memory / Maxmem  Fullness\n\x00"
                   as *const u8 as *const libc::c_char);
    Con_Printf(b"------------  ---------------  ---------------  --------\n\x00"
                   as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[mlumpstat_t; 27]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpstat_t>()
                                                   as libc::c_ulong) {
        let mut stat: *mut mlumpstat_t =
            &mut *worldstats.as_mut_ptr().offset(i as isize) as
                *mut mlumpstat_t;
        if !((*stat).lumpname.is_null() || (*stat).maxcount == 0 ||
                 (*stat).count == 0) {
            if (*stat).entrysize ==
                   ::std::mem::size_of::<byte>() as libc::c_ulong {
                totalmemory +=
                    Mod_GlobUsage((*stat).lumpname,
                                  (*stat).count as libc::c_int,
                                  (*stat).maxcount as libc::c_int)
            } else {
                totalmemory +=
                    Mod_ArrayUsage((*stat).lumpname,
                                   (*stat).count as libc::c_int,
                                   (*stat).maxcount as libc::c_int,
                                   (*stat).entrysize as libc::c_int)
            }
        }
        i += 1
    }
    Con_Printf(b"=== Total BSP file data space used: %s ===\n\x00" as
                   *const u8 as *const libc::c_char,
               Q_pretifymem(totalmemory as libc::c_float, 2 as libc::c_int));
    Con_Printf(b"World size ( %g %g %g ) units\n\x00" as *const u8 as
                   *const libc::c_char,
               world.size[0 as libc::c_int as usize] as libc::c_double,
               world.size[1 as libc::c_int as usize] as libc::c_double,
               world.size[2 as libc::c_int as usize] as libc::c_double);
    Con_Printf(b"Supports transparency world water: %s\n\x00" as *const u8 as
                   *const libc::c_char,
               if world.flags as libc::c_uint &
                      (1 as libc::c_uint) << 2 as libc::c_int != 0 {
                   b"Yes\x00" as *const u8 as *const libc::c_char
               } else { b"No\x00" as *const u8 as *const libc::c_char });
    Con_Printf(b"Lighting: %s\n\x00" as *const u8 as *const libc::c_char,
               if (*w).flags as libc::c_uint &
                      (1 as libc::c_uint) << 4 as libc::c_int != 0 {
                   b"colored\x00" as *const u8 as *const libc::c_char
               } else {
                   b"monochrome\x00" as *const u8 as *const libc::c_char
               });
    Con_Printf(b"World total leafs: %d\n\x00" as *const u8 as
                   *const libc::c_char,
               (*worldmodel).numleafs + 1 as libc::c_int);
    Con_Printf(b"original name: ^1%s\n\x00" as *const u8 as
                   *const libc::c_char, (*worldmodel).name.as_mut_ptr());
    Con_Printf(b"internal name: %s\n\x00" as *const u8 as *const libc::c_char,
               if world.message[0 as libc::c_int as usize] as libc::c_int != 0
                  {
                   va(b"^2%s\x00" as *const u8 as *const libc::c_char,
                      world.message.as_mut_ptr()) as *const libc::c_char
               } else { b"none\x00" as *const u8 as *const libc::c_char });
    Con_Printf(b"map compiler: %s\n\x00" as *const u8 as *const libc::c_char,
               if world.compiler[0 as libc::c_int as usize] as libc::c_int !=
                      0 {
                   va(b"^3%s\x00" as *const u8 as *const libc::c_char,
                      world.compiler.as_mut_ptr()) as *const libc::c_char
               } else { b"unknown\x00" as *const u8 as *const libc::c_char });
    Con_Printf(b"map editor: %s\n\x00" as *const u8 as *const libc::c_char,
               if world.generator[0 as libc::c_int as usize] as libc::c_int !=
                      0 {
                   va(b"^2%s\x00" as *const u8 as *const libc::c_char,
                      world.generator.as_mut_ptr()) as *const libc::c_char
               } else { b"unknown\x00" as *const u8 as *const libc::c_char });
}
/*
===============================================================================

			COMMON ROUTINES

===============================================================================
*/
/*
===================
Mod_DecompressPVS
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_DecompressPVS(mut in_0: *const byte,
                                           mut visbytes: libc::c_int)
 -> *mut byte {
    let mut out: *mut byte = 0 as *mut byte;
    let mut c: libc::c_int = 0;
    out = g_visdata.as_mut_ptr();
    if in_0.is_null() {
        // no vis info, so make all visible
        while visbytes != 0 {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = 0xff as libc::c_int as byte;
            visbytes -= 1
        }
        return g_visdata.as_mut_ptr()
    }
    loop  {
        if *in_0 != 0 {
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = *fresh1
        } else {
            c = *in_0.offset(1 as libc::c_int as isize) as libc::c_int;
            in_0 = in_0.offset(2 as libc::c_int as isize);
            while c != 0 {
                let fresh3 = out;
                out = out.offset(1);
                *fresh3 = 0 as libc::c_int as byte;
                c -= 1
            }
        }
        if !((out.wrapping_offset_from(g_visdata.as_mut_ptr()) as
                  libc::c_long) < visbytes as libc::c_long) {
            break ;
        }
    }
    return g_visdata.as_mut_ptr();
}
/*
==================
Mod_PointInLeaf

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_PointInLeaf(mut p: *const vec_t,
                                         mut node: *mut mnode_t)
 -> *mut mleaf_t {
    loop  {
        if (*node).contents < 0 as libc::c_int { return node as *mut mleaf_t }
        node =
            (*node).children[((if ((*(*node).plane).type_0 as libc::c_int) <
                                      3 as libc::c_int {
                                   *p.offset((*(*node).plane).type_0 as isize)
                               } else {
                                   (*p.offset(0 as libc::c_int as isize) *
                                        (*(*node).plane).normal[0 as
                                                                    libc::c_int
                                                                    as usize]
                                        +
                                        *p.offset(1 as libc::c_int as isize) *
                                            (*(*node).plane).normal[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize])
                                       +
                                       *p.offset(2 as libc::c_int as isize) *
                                           (*(*node).plane).normal[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                               }) - (*(*node).plane).dist <=
                                  0 as libc::c_int as libc::c_float) as
                                 libc::c_int as usize]
    };
}
/*
==================
Mod_GetPVSForPoint

Returns PVS data for a given point
NOTE: can return NULL
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_GetPVSForPoint(mut p: *const vec_t)
 -> *mut byte {
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if worldmodel.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 527 as libc::c_int);
    }
    node = (*worldmodel).nodes;
    loop  {
        if (*node).contents < 0 as libc::c_int {
            leaf = node as *mut mleaf_t;
            break ;
            // we found a leaf
        } else {
            node =
                (*node).children[((if ((*(*node).plane).type_0 as libc::c_int)
                                          < 3 as libc::c_int {
                                       *p.offset((*(*node).plane).type_0 as
                                                     isize)
                                   } else {
                                       (*p.offset(0 as libc::c_int as isize) *
                                            (*(*node).plane).normal[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                            +
                                            *p.offset(1 as libc::c_int as
                                                          isize) *
                                                (*(*node).plane).normal[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize])
                                           +
                                           *p.offset(2 as libc::c_int as
                                                         isize) *
                                               (*(*node).plane).normal[2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                   }) - (*(*node).plane).dist <=
                                      0 as libc::c_int as libc::c_float) as
                                     libc::c_int as usize]
        }
    }
    if !leaf.is_null() && (*leaf).cluster >= 0 as libc::c_int {
        return Mod_DecompressPVS((*leaf).compressed_vis,
                                 world.visbytes as libc::c_int)
    }
    return 0 as *mut byte;
}
/*
==================
Mod_FatPVS_RecursiveBSPNode

==================
*/
unsafe extern "C" fn Mod_FatPVS_RecursiveBSPNode(mut org: *const vec_t,
                                                 mut radius: libc::c_float,
                                                 mut visbuffer: *mut byte,
                                                 mut visbytes: libc::c_int,
                                                 mut node: *mut mnode_t) {
    let mut i: libc::c_int = 0;
    while (*node).contents >= 0 as libc::c_int {
        let mut d: libc::c_float =
            (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *org.offset((*(*node).plane).type_0 as isize)
             } else {
                 (*org.offset(0 as libc::c_int as isize) *
                      (*(*node).plane).normal[0 as libc::c_int as usize] +
                      *org.offset(1 as libc::c_int as isize) *
                          (*(*node).plane).normal[1 as libc::c_int as usize])
                     +
                     *org.offset(2 as libc::c_int as isize) *
                         (*(*node).plane).normal[2 as libc::c_int as usize]
             }) - (*(*node).plane).dist;
        if d > radius {
            node = (*node).children[0 as libc::c_int as usize]
        } else if d < -radius {
            node = (*node).children[1 as libc::c_int as usize]
        } else {
            // go down both sides
            Mod_FatPVS_RecursiveBSPNode(org, radius, visbuffer, visbytes,
                                        (*node).children[0 as libc::c_int as
                                                             usize]);
            node = (*node).children[1 as libc::c_int as usize]
        }
    }
    // if this leaf is in a cluster, accumulate the vis bits
    if (*(node as *mut mleaf_t)).cluster >= 0 as libc::c_int {
        let mut vis: *mut byte =
            Mod_DecompressPVS((*(node as *mut mleaf_t)).compressed_vis,
                              world.visbytes as libc::c_int);
        i = 0 as libc::c_int;
        while i < visbytes {
            let ref mut fresh4 = *visbuffer.offset(i as isize);
            *fresh4 =
                (*fresh4 as libc::c_int |
                     *vis.offset(i as isize) as libc::c_int) as byte;
            i += 1
        }
    };
}
/*
==================
Mod_FatPVS_RecursiveBSPNode

Calculates a PVS that is the inclusive or of all leafs
within radius pixels of the given point.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_FatPVS(mut org: *const vec_t,
                                    mut radius: libc::c_float,
                                    mut visbuffer: *mut byte,
                                    mut visbytes: libc::c_int,
                                    mut merge: qboolean,
                                    mut fullvis: qboolean) -> libc::c_int {
    let mut bytes: libc::c_int = world.visbytes as libc::c_int;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if worldmodel.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 595 as libc::c_int);
    }
    leaf = Mod_PointInLeaf(org, (*worldmodel).nodes);
    bytes = if bytes < visbytes { bytes } else { visbytes };
    // enable full visibility for some reasons
    if fullvis as libc::c_uint != 0 || (*worldmodel).visdata.is_null() ||
           leaf.is_null() || (*leaf).cluster < 0 as libc::c_int {
        memset(visbuffer as *mut libc::c_void, 0xff as libc::c_int,
               bytes as libc::c_ulong);
        return bytes
    }
    if merge as u64 == 0 {
        memset(visbuffer as *mut libc::c_void, 0 as libc::c_int,
               bytes as libc::c_ulong);
    }
    Mod_FatPVS_RecursiveBSPNode(org, radius, visbuffer, bytes,
                                (*worldmodel).nodes);
    return bytes;
}
/*
======================================================================

LEAF LISTING

======================================================================
*/
unsafe extern "C" fn Mod_BoxLeafnums_r(mut ll: *mut leaflist_t,
                                       mut node: *mut mnode_t) {
    let mut sides: libc::c_int = 0;
    loop  {
        if (*node).contents == -(2 as libc::c_int) { return }
        if (*node).contents < 0 as libc::c_int {
            let mut leaf: *mut mleaf_t = node as *mut mleaf_t;
            // it's a leaf!
            if (*ll).count >= (*ll).maxcount {
                (*ll).overflowed = true_0;
                return
            }
            let fresh5 = (*ll).count;
            (*ll).count = (*ll).count + 1;
            *(*ll).list.offset(fresh5 as isize) = (*leaf).cluster;
            return
        }
        sides =
            if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                if (*(*node).plane).dist <=
                       (*ll).mins[(*(*node).plane).type_0 as usize] {
                    1 as libc::c_int
                } else if (*(*node).plane).dist >=
                              (*ll).maxs[(*(*node).plane).type_0 as usize] {
                    2 as libc::c_int
                } else { 3 as libc::c_int }
            } else {
                BoxOnPlaneSide((*ll).mins.as_mut_ptr() as *const vec_t,
                               (*ll).maxs.as_mut_ptr() as *const vec_t,
                               (*node).plane)
            };
        if sides == 1 as libc::c_int {
            node = (*node).children[0 as libc::c_int as usize]
        } else if sides == 2 as libc::c_int {
            node = (*node).children[1 as libc::c_int as usize]
        } else {
            // go down both
            if (*ll).topnode == -(1 as libc::c_int) {
                (*ll).topnode =
                    node.wrapping_offset_from((*worldmodel).nodes) as
                        libc::c_long as libc::c_int
            }
            Mod_BoxLeafnums_r(ll,
                              (*node).children[0 as libc::c_int as usize]);
            node = (*node).children[1 as libc::c_int as usize]
        }
    };
}
/*
==================
Mod_BoxLeafnums
==================
*/
unsafe extern "C" fn Mod_BoxLeafnums(mut mins: *const vec_t,
                                     mut maxs: *const vec_t,
                                     mut list: *mut libc::c_int,
                                     mut listsize: libc::c_int,
                                     mut topnode: *mut libc::c_int)
 -> libc::c_int {
    let mut ll: leaflist_t =
        leaflist_t{count: 0,
                   maxcount: 0,
                   overflowed: false_0,
                   list: 0 as *mut libc::c_int,
                   mins: [0.; 3],
                   maxs: [0.; 3],
                   topnode: 0,};
    if worldmodel.is_null() { return 0 as libc::c_int }
    ll.mins[0 as libc::c_int as usize] =
        *mins.offset(0 as libc::c_int as isize);
    ll.mins[1 as libc::c_int as usize] =
        *mins.offset(1 as libc::c_int as isize);
    ll.mins[2 as libc::c_int as usize] =
        *mins.offset(2 as libc::c_int as isize);
    ll.maxs[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize);
    ll.maxs[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize);
    ll.maxs[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize);
    ll.maxcount = listsize;
    ll.overflowed = false_0;
    ll.topnode = -(1 as libc::c_int);
    ll.list = list;
    ll.count = 0 as libc::c_int;
    Mod_BoxLeafnums_r(&mut ll, (*worldmodel).nodes);
    if !topnode.is_null() { *topnode = ll.topnode }
    return ll.count;
}
/*
=============
Mod_BoxVisible

Returns true if any leaf in boxspace
is potentially visible
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_BoxVisible(mut mins: *const vec_t,
                                        mut maxs: *const vec_t,
                                        mut visbits: *const byte)
 -> qboolean {
    let mut leafList: [libc::c_int; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if visbits.is_null() || mins.is_null() || maxs.is_null() { return true_0 }
    count =
        Mod_BoxLeafnums(mins, maxs, leafList.as_mut_ptr(), 256 as libc::c_int,
                        0 as *mut libc::c_int);
    i = 0 as libc::c_int;
    while i < count {
        if if leafList[i as usize] >= 0 as libc::c_int {
               (*visbits.offset((leafList[i as usize] >> 3 as libc::c_int) as
                                    isize) as libc::c_int &
                    (1 as libc::c_int) <<
                        (leafList[i as usize] & 7 as libc::c_int)) as byte as
                   libc::c_int
           } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
            return true_0
        }
        i += 1
    }
    return false_0;
}
/*
=============
Mod_HeadnodeVisible
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_HeadnodeVisible(mut node: *mut mnode_t,
                                             mut visbits: *const byte,
                                             mut lastleaf: *mut libc::c_int)
 -> qboolean {
    if node.is_null() || (*node).contents == -(2 as libc::c_int) {
        return false_0
    }
    if (*node).contents < 0 as libc::c_int {
        if if (*(node as *mut mleaf_t)).cluster >= 0 as libc::c_int {
               (*visbits.offset(((*(node as *mut mleaf_t)).cluster >>
                                     3 as libc::c_int) as isize) as
                    libc::c_int &
                    (1 as libc::c_int) <<
                        ((*(node as *mut mleaf_t)).cluster &
                             7 as libc::c_int)) as byte as libc::c_int
           } else { false_0 as libc::c_int as byte as libc::c_int } == 0 {
            return false_0
        }
        if !lastleaf.is_null() {
            *lastleaf = (*(node as *mut mleaf_t)).cluster
        }
        return true_0
    }
    if Mod_HeadnodeVisible((*node).children[0 as libc::c_int as usize],
                           visbits, lastleaf) as u64 != 0 {
        return true_0
    }
    if Mod_HeadnodeVisible((*node).children[1 as libc::c_int as usize],
                           visbits, lastleaf) as u64 != 0 {
        return true_0
    }
    return false_0;
}
/*
==================
Mod_AmbientLevels

grab the ambient sound levels for current point
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_AmbientLevels(mut p: *const vec_t,
                                           mut pvolumes: *mut byte) {
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if worldmodel.is_null() || p.is_null() || pvolumes.is_null() { return }
    leaf = Mod_PointInLeaf(p, (*worldmodel).nodes);
    *(pvolumes as *mut libc::c_int) =
        *((*leaf).ambient_sound_level.as_mut_ptr() as *mut libc::c_int);
}
/*
=================
Mod_FindModelOrigin

routine to detect bmodels with origin-brush
=================
*/
unsafe extern "C" fn Mod_FindModelOrigin(mut entities: *const libc::c_char,
                                         mut modelname: *const libc::c_char,
                                         mut origin: *mut vec_t) {
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keyname: string = [0; 256];
    let mut token: [libc::c_char; 2048] = [0; 2048];
    let mut model_found: qboolean = false_0;
    let mut origin_found: qboolean = false_0;
    if entities.is_null() || modelname.is_null() || *modelname == 0 { return }
    if origin.is_null() ||
           !(*origin.offset(0 as libc::c_int as isize) == 0.0f32 &&
                 *origin.offset(1 as libc::c_int as isize) == 0.0f32 &&
                 *origin.offset(2 as libc::c_int as isize) == 0.0f32) {
        return
    }
    pfile = entities as *mut libc::c_char;
    loop  {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 2048]>()
                                   as libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        if token[0 as libc::c_int as usize] as libc::c_int != '{' as i32 {
            Host_Error(b"Mod_FindModelOrigin: found %s when expecting {\n\x00"
                           as *const u8 as *const libc::c_char,
                       token.as_mut_ptr());
        }
        origin_found = false_0;
        model_found = origin_found;
        let ref mut fresh6 = *origin.offset(2 as libc::c_int as isize);
        *fresh6 = 0 as libc::c_int as vec_t;
        let ref mut fresh7 = *origin.offset(1 as libc::c_int as isize);
        *fresh7 = *fresh6;
        *origin.offset(0 as libc::c_int as isize) = *fresh7;
        loop  {
            // parse key
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int); // end of desc
            if pfile.is_null() {
                Host_Error(b"Mod_FindModelOrigin: EOF without closing brace\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if token[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
                break ;
            }
            Q_strncpy(keyname.as_mut_ptr(), token.as_mut_ptr(),
                      ::std::mem::size_of::<string>() as libc::c_ulong);
            // parse value
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if pfile.is_null() {
                Host_Error(b"Mod_FindModelOrigin: EOF without closing brace\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if token[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
                Host_Error(b"Mod_FindModelOrigin: closing brace without data\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if Q_strnicmp(keyname.as_mut_ptr(),
                          b"model\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 &&
                   Q_strnicmp(modelname, token.as_mut_ptr(),
                              99999 as libc::c_int) == 0 {
                model_found = true_0
            }
            if Q_strnicmp(keyname.as_mut_ptr(),
                          b"origin\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                Q_atov(origin, token.as_mut_ptr(),
                       3 as libc::c_int as size_t);
                origin_found = true_0
            }
        }
        if model_found as u64 != 0 { break ; }
    };
}
/*
==================
Mod_CheckWaterAlphaSupport

converted maps potential may don't
support water transparency
==================
*/
unsafe extern "C" fn Mod_CheckWaterAlphaSupport(mut bmod: *mut dbspmodel_t)
 -> qboolean {
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pvs: *const byte = 0 as *const byte;
    if (*bmod).visdatasize <= 0 as libc::c_int as libc::c_ulong {
        return true_0
    }
    // check all liquid leafs to see if they can see into empty leafs, if any
	// can we can assume this map supports r_wateralpha
    i = 0 as libc::c_int;
    leaf = (*loadmodel).leafs;
    while i < (*loadmodel).numleafs {
        if ((*leaf).contents == -(3 as libc::c_int) ||
                (*leaf).contents == -(4 as libc::c_int)) &&
               (*leaf).cluster >= 0 as libc::c_int {
            pvs =
                Mod_DecompressPVS((*leaf).compressed_vis,
                                  world.visbytes as libc::c_int);
            j = 0 as libc::c_int;
            while j < (*loadmodel).numleafs {
                if (if (*(*loadmodel).leafs.offset(j as isize)).cluster >=
                           0 as libc::c_int {
                        (*pvs.offset(((*(*loadmodel).leafs.offset(j as
                                                                      isize)).cluster
                                          >> 3 as libc::c_int) as isize) as
                             libc::c_int &
                             (1 as libc::c_int) <<
                                 ((*(*loadmodel).leafs.offset(j as
                                                                  isize)).cluster
                                      & 7 as libc::c_int)) as byte as
                            libc::c_int
                    } else { false_0 as libc::c_int as byte as libc::c_int })
                       != 0 &&
                       (*(*loadmodel).leafs.offset(j as isize)).contents ==
                           -(1 as libc::c_int) {
                    return true_0
                }
                j += 1
            }
        }
        i += 1;
        leaf = leaf.offset(1)
    }
    return false_0;
}
/*
==================
Mod_SampleSizeForFace

return the current lightmap resolution per face
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_SampleSizeForFace(mut surf: *mut msurface_t)
 -> libc::c_int {
    if surf.is_null() || (*surf).texinfo.is_null() {
        return 16 as libc::c_int
    }
    // world luxels has more priority
    if (*(*surf).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    if (*(*surf).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        return 8 as libc::c_int
    }
    if !(*(*surf).texinfo).faceinfo.is_null() {
        return (*(*(*surf).texinfo).faceinfo).texture_step as libc::c_int
    }
    return 16 as libc::c_int;
}
/*
==================
Mod_GetFaceContents

determine face contents by name
==================
*/
unsafe extern "C" fn Mod_GetFaceContents(mut name: *const libc::c_char)
 -> libc::c_int {
    if Q_strnicmp(name, b"SKY\x00" as *const u8 as *const libc::c_char,
                  3 as libc::c_int) == 0 {
        return -(6 as libc::c_int)
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 ||
           *name.offset(0 as libc::c_int as isize) as libc::c_int ==
               '*' as i32 {
        if Q_strnicmp(name.offset(1 as libc::c_int as isize),
                      b"lava\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int) == 0 {
            return -(5 as libc::c_int)
        } else {
            if Q_strnicmp(name.offset(1 as libc::c_int as isize),
                          b"slime\x00" as *const u8 as *const libc::c_char,
                          5 as libc::c_int) == 0 {
                return -(4 as libc::c_int)
            }
        }
        return -(3 as libc::c_int)
        // otherwise it's water
    }
    if Q_strnicmp(name, b"water\x00" as *const u8 as *const libc::c_char,
                  5 as libc::c_int) == 0 {
        return -(3 as libc::c_int)
    }
    return -(2 as libc::c_int);
}
/*
==================
Mod_GetFaceContents

determine face contents by name
==================
*/
unsafe extern "C" fn Mod_GetVertexByNumber(mut mod_0: *mut model_t,
                                           mut surfedge: libc::c_int)
 -> *mut mvertex_t {
    let mut lindex: libc::c_int = 0;
    let mut edge: *mut medge_t = 0 as *mut medge_t;
    lindex = *(*mod_0).surfedges.offset(surfedge as isize);
    if lindex > 0 as libc::c_int {
        edge = &mut *(*mod_0).edges.offset(lindex as isize) as *mut medge_t;
        return &mut *(*mod_0).vertexes.offset(*(*edge).v.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
                                                  as isize) as *mut mvertex_t
    } else {
        edge = &mut *(*mod_0).edges.offset(-lindex as isize) as *mut medge_t;
        return &mut *(*mod_0).vertexes.offset(*(*edge).v.as_mut_ptr().offset(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
                                                  as isize) as *mut mvertex_t
    };
}
/*
==================
Mod_MakeNormalAxial

remove jitter from near-axial normals
==================
*/
unsafe extern "C" fn Mod_MakeNormalAxial(mut normal: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    type_0 = 0 as libc::c_int;
    while type_0 < 3 as libc::c_int {
        if __tg_fabs(*normal.offset(type_0 as isize)) > 0.9999f32 { break ; }
        type_0 += 1
    }
    // make positive and pure axial
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int && type_0 != 3 as libc::c_int {
        if i == type_0 {
            *normal.offset(i as isize) = 1.0f32
        } else { *normal.offset(i as isize) = 0.0f32 }
        i += 1
    };
}
/*
==================
Mod_LightMatrixFromTexMatrix

compute lightmap matrix based on texture matrix
==================
*/
unsafe extern "C" fn Mod_LightMatrixFromTexMatrix(mut tx: *const mtexinfo_t,
                                                  mut lmvecs:
                                                      *mut [libc::c_float; 4]) {
    let mut lmscale: libc::c_float = 16 as libc::c_int as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // this is can't be possible but who knews
    if (*tx).flags as libc::c_uint & (1 as libc::c_uint) << 3 as libc::c_int
           != 0 {
        lmscale = 8 as libc::c_int as libc::c_float
    }
    if !(*tx).faceinfo.is_null() {
        lmscale = (*(*tx).faceinfo).texture_step as libc::c_float
    }
    // copy texmatrix into lightmap matrix fisrt
    i = 0 as libc::c_int; // just use texmatrix
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            (*lmvecs.offset(i as isize))[j as usize] =
                (*tx).vecs[i as usize][j as usize];
            j += 1
        }
        i += 1
    }
    if (*tx).flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           == 0 {
        return
    }
    let mut ilength: libc::c_float =
        __tg_sqrt((*lmvecs.offset(0 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                      (*lmvecs.offset(0 as libc::c_int as
                                          isize))[0 as libc::c_int as usize] +
                      (*lmvecs.offset(0 as libc::c_int as
                                          isize))[1 as libc::c_int as usize] *
                          (*lmvecs.offset(0 as libc::c_int as
                                              isize))[1 as libc::c_int as
                                                          usize] +
                      (*lmvecs.offset(0 as libc::c_int as
                                          isize))[2 as libc::c_int as usize] *
                          (*lmvecs.offset(0 as libc::c_int as
                                              isize))[2 as libc::c_int as
                                                          usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    (*lmvecs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *=
        ilength;
    (*lmvecs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] *=
        ilength;
    (*lmvecs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] *=
        ilength;
    let mut ilength_0: libc::c_float =
        __tg_sqrt((*lmvecs.offset(1 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                      (*lmvecs.offset(1 as libc::c_int as
                                          isize))[0 as libc::c_int as usize] +
                      (*lmvecs.offset(1 as libc::c_int as
                                          isize))[1 as libc::c_int as usize] *
                          (*lmvecs.offset(1 as libc::c_int as
                                              isize))[1 as libc::c_int as
                                                          usize] +
                      (*lmvecs.offset(1 as libc::c_int as
                                          isize))[2 as libc::c_int as usize] *
                          (*lmvecs.offset(1 as libc::c_int as
                                              isize))[2 as libc::c_int as
                                                          usize]);
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    (*lmvecs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *=
        ilength_0;
    (*lmvecs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] *=
        ilength_0;
    (*lmvecs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] *=
        ilength_0;
    if (*tx).flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
           != 0 {
        Mod_MakeNormalAxial((*lmvecs.offset(0 as libc::c_int as
                                                isize)).as_mut_ptr());
        Mod_MakeNormalAxial((*lmvecs.offset(1 as libc::c_int as
                                                isize)).as_mut_ptr());
    }
    // put the lighting origin at center the of poly
    (*lmvecs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*lmvecs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (1.0f32 / lmscale);
    (*lmvecs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*lmvecs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (1.0f32 / lmscale);
    (*lmvecs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*lmvecs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (1.0f32 / lmscale);
    (*lmvecs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*lmvecs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            * -(1.0f32 / lmscale);
    (*lmvecs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*lmvecs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * -(1.0f32 / lmscale);
    (*lmvecs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*lmvecs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * -(1.0f32 / lmscale);
    (*lmvecs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        lmscale * 0.5f32;
    (*lmvecs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -lmscale * 0.5f32;
}
/*
=================
Mod_CalcSurfaceExtents

Fills in surf->texturemins[] and surf->extents[]
=================
*/
unsafe extern "C" fn Mod_CalcSurfaceExtents(mut surf: *mut msurface_t) {
    let mut mins: [libc::c_float; 2] = [0.; 2];
    let mut maxs: [libc::c_float; 2] = [0.; 2];
    let mut val: libc::c_float = 0.;
    let mut lmmins: [libc::c_float; 2] = [0.; 2];
    let mut lmmaxs: [libc::c_float; 2] = [0.; 2];
    let mut bmins: [libc::c_int; 2] = [0; 2];
    let mut bmaxs: [libc::c_int; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut sample_size: libc::c_int = 0;
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut facenum: libc::c_int =
        surf.wrapping_offset_from((*loadmodel).surfaces) as libc::c_long as
            libc::c_int;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut v: *mut mvertex_t = 0 as *mut mvertex_t;
    sample_size = Mod_SampleSizeForFace(surf);
    tex = (*surf).texinfo;
    Mod_LightMatrixFromTexMatrix(tex, (*info).lmvecs.as_mut_ptr());
    lmmins[1 as libc::c_int as usize] =
        999999 as libc::c_int as libc::c_float;
    mins[1 as libc::c_int as usize] = lmmins[1 as libc::c_int as usize];
    lmmins[0 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
    mins[0 as libc::c_int as usize] = lmmins[0 as libc::c_int as usize];
    lmmaxs[1 as libc::c_int as usize] =
        -(999999 as libc::c_int) as libc::c_float;
    maxs[1 as libc::c_int as usize] = lmmaxs[1 as libc::c_int as usize];
    lmmaxs[0 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
    maxs[0 as libc::c_int as usize] = lmmaxs[0 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < (*surf).numedges {
        e = *(*loadmodel).surfedges.offset(((*surf).firstedge + i) as isize);
        if e >= (*loadmodel).numedges || e <= -(*loadmodel).numedges {
            Host_Error(b"Mod_CalcSurfaceExtents: bad edge\n\x00" as *const u8
                           as *const libc::c_char);
        }
        if e >= 0 as libc::c_int {
            v =
                &mut *(*loadmodel).vertexes.offset(*(*(*loadmodel).edges.offset(e
                                                                                    as
                                                                                    isize)).v.as_mut_ptr().offset(0
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                       as isize) as
                    *mut mvertex_t
        } else {
            v =
                &mut *(*loadmodel).vertexes.offset(*(*(*loadmodel).edges.offset(-e
                                                                                    as
                                                                                    isize)).v.as_mut_ptr().offset(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                       as isize) as
                    *mut mvertex_t
        }
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            val =
                (*v).position[0 as libc::c_int as usize] *
                    (*(*surf).texinfo).vecs[j as
                                                usize][0 as libc::c_int as
                                                           usize] +
                    (*v).position[1 as libc::c_int as usize] *
                        (*(*surf).texinfo).vecs[j as
                                                    usize][1 as libc::c_int as
                                                               usize] +
                    (*v).position[2 as libc::c_int as usize] *
                        (*(*surf).texinfo).vecs[j as
                                                    usize][2 as libc::c_int as
                                                               usize] +
                    (*(*surf).texinfo).vecs[j as
                                                usize][3 as libc::c_int as
                                                           usize];
            mins[j as usize] =
                if val < mins[j as usize] { val } else { mins[j as usize] };
            maxs[j as usize] =
                if val > maxs[j as usize] { val } else { maxs[j as usize] };
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            val =
                (*v).position[0 as libc::c_int as usize] *
                    (*info).lmvecs[j as usize][0 as libc::c_int as usize] +
                    (*v).position[1 as libc::c_int as usize] *
                        (*info).lmvecs[j as usize][1 as libc::c_int as usize]
                    +
                    (*v).position[2 as libc::c_int as usize] *
                        (*info).lmvecs[j as usize][2 as libc::c_int as usize]
                    + (*info).lmvecs[j as usize][3 as libc::c_int as usize];
            lmmins[j as usize] =
                if val < lmmins[j as usize] {
                    val
                } else { lmmins[j as usize] };
            lmmaxs[j as usize] =
                if val > lmmaxs[j as usize] {
                    val
                } else { lmmaxs[j as usize] };
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        bmins[i as usize] =
            __tg_floor(mins[i as usize] / sample_size as libc::c_float) as
                libc::c_int;
        bmaxs[i as usize] =
            __tg_ceil(maxs[i as usize] / sample_size as libc::c_float) as
                libc::c_int;
        (*surf).texturemins[i as usize] =
            (bmins[i as usize] * sample_size) as libc::c_short;
        (*surf).extents[i as usize] =
            ((bmaxs[i as usize] - bmins[i as usize]) * sample_size) as
                libc::c_short;
        if (*tex).flags as libc::c_uint &
               (1 as libc::c_uint) << 1 as libc::c_int != 0 {
            lmmins[i as usize] = __tg_floor(lmmins[i as usize]);
            lmmaxs[i as usize] = __tg_ceil(lmmaxs[i as usize]);
            (*info).lightmapmins[i as usize] =
                lmmins[i as usize] as libc::c_short;
            (*info).lightextents[i as usize] =
                (lmmaxs[i as usize] - lmmins[i as usize]) as libc::c_short
        } else {
            // REFTODO:
            // XASH_DEDICATED
            // just copy texturemins
            (*info).lightmapmins[i as usize] =
                (*surf).texturemins[i as usize];
            (*info).lightextents[i as usize] = (*surf).extents[i as usize]
        }
        i += 1
    };
}
/*
=================
Mod_CalcSurfaceBounds

fills in surf->mins and surf->maxs
=================
*/
unsafe extern "C" fn Mod_CalcSurfaceBounds(mut surf: *mut msurface_t) {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut v: *mut mvertex_t = 0 as *mut mvertex_t;
    ClearBounds((*(*surf).info).mins.as_mut_ptr(),
                (*(*surf).info).maxs.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < (*surf).numedges {
        e = *(*loadmodel).surfedges.offset(((*surf).firstedge + i) as isize);
        if e >= (*loadmodel).numedges || e <= -(*loadmodel).numedges {
            Host_Error(b"Mod_CalcSurfaceBounds: bad edge\n\x00" as *const u8
                           as *const libc::c_char);
        }
        if e >= 0 as libc::c_int {
            v =
                &mut *(*loadmodel).vertexes.offset(*(*(*loadmodel).edges.offset(e
                                                                                    as
                                                                                    isize)).v.as_mut_ptr().offset(0
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                       as isize) as
                    *mut mvertex_t
        } else {
            v =
                &mut *(*loadmodel).vertexes.offset(*(*(*loadmodel).edges.offset(-e
                                                                                    as
                                                                                    isize)).v.as_mut_ptr().offset(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                       as isize) as
                    *mut mvertex_t
        }
        AddPointToBounds((*v).position.as_mut_ptr() as *const vec_t,
                         (*(*surf).info).mins.as_mut_ptr(),
                         (*(*surf).info).maxs.as_mut_ptr());
        i += 1
    }
    (*(*surf).info).origin[0 as libc::c_int as usize] =
        ((*(*surf).info).mins[0 as libc::c_int as usize] +
             (*(*surf).info).maxs[0 as libc::c_int as usize]) * 0.5f32;
    (*(*surf).info).origin[1 as libc::c_int as usize] =
        ((*(*surf).info).mins[1 as libc::c_int as usize] +
             (*(*surf).info).maxs[1 as libc::c_int as usize]) * 0.5f32;
    (*(*surf).info).origin[2 as libc::c_int as usize] =
        ((*(*surf).info).mins[2 as libc::c_int as usize] +
             (*(*surf).info).maxs[2 as libc::c_int as usize]) * 0.5f32;
}
/*
=================
Mod_CreateFaceBevels
=================
*/
unsafe extern "C" fn Mod_CreateFaceBevels(mut surf: *mut msurface_t) {
    let mut delta: vec3_t = [0.; 3];
    let mut edgevec: vec3_t = [0.; 3];
    let mut facebevel: *mut byte = 0 as *mut byte;
    let mut faceNormal: vec3_t = [0.; 3];
    let mut v0: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut v1: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut contents: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut radius: vec_t = 0.;
    let mut fb: *mut mfacebevel_t = 0 as *mut mfacebevel_t;
    if !(*surf).texinfo.is_null() && !(*(*surf).texinfo).texture.is_null() {
        contents =
            Mod_GetFaceContents((*(*(*surf).texinfo).texture).name.as_mut_ptr())
    } else { contents = -(2 as libc::c_int) }
    size =
        (::std::mem::size_of::<mfacebevel_t>() as
             libc::c_ulong).wrapping_add(((*surf).numedges as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mplane_t>()
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int;
    facebevel =
        _Mem_Alloc((*loadmodel).mempool, size as size_t, true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1145 as libc::c_int) as *mut byte;
    fb = facebevel as *mut mfacebevel_t;
    facebevel =
        facebevel.offset(::std::mem::size_of::<mfacebevel_t>() as
                             libc::c_ulong as isize);
    (*fb).edges = facebevel as *mut mplane_t;
    (*fb).numedges = (*surf).numedges;
    (*fb).contents = contents;
    (*(*surf).info).bevel = fb;
    if (*surf).flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           != 0 {
        faceNormal[0 as libc::c_int as usize] =
            -(*(*surf).plane).normal[0 as libc::c_int as usize];
        faceNormal[1 as libc::c_int as usize] =
            -(*(*surf).plane).normal[1 as libc::c_int as usize];
        faceNormal[2 as libc::c_int as usize] =
            -(*(*surf).plane).normal[2 as libc::c_int as usize]
    } else {
        faceNormal[0 as libc::c_int as usize] =
            (*(*surf).plane).normal[0 as libc::c_int as usize];
        faceNormal[1 as libc::c_int as usize] =
            (*(*surf).plane).normal[1 as libc::c_int as usize];
        faceNormal[2 as libc::c_int as usize] =
            (*(*surf).plane).normal[2 as libc::c_int as usize]
    }
    // compute face origin and plane edges
    i = 0 as libc::c_int;
    while i < (*surf).numedges {
        let mut dest: *mut mplane_t =
            &mut *(*fb).edges.offset(i as isize) as *mut mplane_t;
        v0 = Mod_GetVertexByNumber(loadmodel, (*surf).firstedge + i);
        v1 =
            Mod_GetVertexByNumber(loadmodel,
                                  (*surf).firstedge +
                                      (i + 1 as libc::c_int) %
                                          (*surf).numedges);
        edgevec[0 as libc::c_int as usize] =
            (*v1).position[0 as libc::c_int as usize] -
                (*v0).position[0 as libc::c_int as usize];
        edgevec[1 as libc::c_int as usize] =
            (*v1).position[1 as libc::c_int as usize] -
                (*v0).position[1 as libc::c_int as usize];
        edgevec[2 as libc::c_int as usize] =
            (*v1).position[2 as libc::c_int as usize] -
                (*v0).position[2 as libc::c_int as usize];
        (*dest).normal[0 as libc::c_int as usize] =
            faceNormal[1 as libc::c_int as usize] *
                edgevec[2 as libc::c_int as usize] -
                faceNormal[2 as libc::c_int as usize] *
                    edgevec[1 as libc::c_int as usize];
        (*dest).normal[1 as libc::c_int as usize] =
            faceNormal[2 as libc::c_int as usize] *
                edgevec[0 as libc::c_int as usize] -
                faceNormal[0 as libc::c_int as usize] *
                    edgevec[2 as libc::c_int as usize];
        (*dest).normal[2 as libc::c_int as usize] =
            faceNormal[0 as libc::c_int as usize] *
                edgevec[1 as libc::c_int as usize] -
                faceNormal[1 as libc::c_int as usize] *
                    edgevec[0 as libc::c_int as usize];
        let mut ilength: libc::c_float =
            __tg_sqrt((*dest).normal[0 as libc::c_int as usize] *
                          (*dest).normal[0 as libc::c_int as usize] +
                          (*dest).normal[1 as libc::c_int as usize] *
                              (*dest).normal[1 as libc::c_int as usize] +
                          (*dest).normal[2 as libc::c_int as usize] *
                              (*dest).normal[2 as libc::c_int as usize]);
        if ilength != 0. { ilength = 1.0f32 / ilength }
        (*dest).normal[0 as libc::c_int as usize] *= ilength;
        (*dest).normal[1 as libc::c_int as usize] *= ilength;
        (*dest).normal[2 as libc::c_int as usize] *= ilength;
        (*dest).dist =
            (*dest).normal[0 as libc::c_int as usize] *
                (*v0).position[0 as libc::c_int as usize] +
                (*dest).normal[1 as libc::c_int as usize] *
                    (*v0).position[1 as libc::c_int as usize] +
                (*dest).normal[2 as libc::c_int as usize] *
                    (*v0).position[2 as libc::c_int as usize];
        (*dest).type_0 =
            PlaneTypeForNormal((*dest).normal.as_mut_ptr() as *const vec_t) as
                byte;
        (*fb).origin[0 as libc::c_int as usize] =
            (*fb).origin[0 as libc::c_int as usize] +
                (*v0).position[0 as libc::c_int as usize];
        (*fb).origin[1 as libc::c_int as usize] =
            (*fb).origin[1 as libc::c_int as usize] +
                (*v0).position[1 as libc::c_int as usize];
        (*fb).origin[2 as libc::c_int as usize] =
            (*fb).origin[2 as libc::c_int as usize] +
                (*v0).position[2 as libc::c_int as usize];
        i += 1
    }
    (*fb).origin[0 as libc::c_int as usize] =
        (*fb).origin[0 as libc::c_int as usize] *
            (1.0f32 / (*surf).numedges as libc::c_float);
    (*fb).origin[1 as libc::c_int as usize] =
        (*fb).origin[1 as libc::c_int as usize] *
            (1.0f32 / (*surf).numedges as libc::c_float);
    (*fb).origin[2 as libc::c_int as usize] =
        (*fb).origin[2 as libc::c_int as usize] *
            (1.0f32 / (*surf).numedges as libc::c_float);
    // compute face radius
    i = 0 as libc::c_int;
    while i < (*surf).numedges {
        v0 = Mod_GetVertexByNumber(loadmodel, (*surf).firstedge + i);
        delta[0 as libc::c_int as usize] =
            (*v0).position[0 as libc::c_int as usize] -
                (*fb).origin[0 as libc::c_int as usize];
        delta[1 as libc::c_int as usize] =
            (*v0).position[1 as libc::c_int as usize] -
                (*fb).origin[1 as libc::c_int as usize];
        delta[2 as libc::c_int as usize] =
            (*v0).position[2 as libc::c_int as usize] -
                (*fb).origin[2 as libc::c_int as usize];
        radius =
            delta[0 as libc::c_int as usize] *
                delta[0 as libc::c_int as usize] +
                delta[1 as libc::c_int as usize] *
                    delta[1 as libc::c_int as usize] +
                delta[2 as libc::c_int as usize] *
                    delta[2 as libc::c_int as usize];
        (*fb).radius =
            if radius > (*fb).radius { radius } else { (*fb).radius };
        i += 1
    };
}
/*
=================
Mod_SetParent
=================
*/
unsafe extern "C" fn Mod_SetParent(mut node: *mut mnode_t,
                                   mut parent: *mut mnode_t) {
    (*node).parent = parent; // it's leaf
    if (*node).contents < 0 as libc::c_int { return }
    Mod_SetParent((*node).children[0 as libc::c_int as usize], node);
    Mod_SetParent((*node).children[1 as libc::c_int as usize], node);
}
/*
==================
CountClipNodes_r
==================
*/
unsafe extern "C" fn CountClipNodes_r(mut src: *mut mclipnode_t,
                                      mut hull: *mut hull_t,
                                      mut nodenum: libc::c_int) {
    // leaf?
    if nodenum < 0 as libc::c_int { return }
    if (*hull).lastclipnode == 32767 as libc::c_int {
        Host_Error(b"MAX_MAP_CLIPNODES limit exceeded\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    (*hull).lastclipnode += 1;
    CountClipNodes_r(src, hull,
                     (*src.offset(nodenum as
                                      isize)).children[0 as libc::c_int as
                                                           usize] as
                         libc::c_int);
    CountClipNodes_r(src, hull,
                     (*src.offset(nodenum as
                                      isize)).children[1 as libc::c_int as
                                                           usize] as
                         libc::c_int);
}
/*
==================
CountClipNodes32_r
==================
*/
unsafe extern "C" fn CountClipNodes32_r(mut src: *mut dclipnode32_t,
                                        mut hull: *mut hull_t,
                                        mut nodenum: libc::c_int) {
    // leaf?
    if nodenum < 0 as libc::c_int { return }
    if (*hull).lastclipnode == 32767 as libc::c_int {
        Host_Error(b"MAX_MAP_CLIPNODES limit exceeded\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    (*hull).lastclipnode += 1;
    CountClipNodes32_r(src, hull,
                       (*src.offset(nodenum as
                                        isize)).children[0 as libc::c_int as
                                                             usize]);
    CountClipNodes32_r(src, hull,
                       (*src.offset(nodenum as
                                        isize)).children[1 as libc::c_int as
                                                             usize]);
}
/*
==================
RemapClipNodes_r
==================
*/
unsafe extern "C" fn RemapClipNodes_r(mut srcnodes: *mut dclipnode32_t,
                                      mut hull: *mut hull_t,
                                      mut nodenum: libc::c_int)
 -> libc::c_int {
    let mut src: *mut dclipnode32_t = 0 as *mut dclipnode32_t;
    let mut out: *mut mclipnode_t = 0 as *mut mclipnode_t;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    // leaf?
    if nodenum < 0 as libc::c_int { return nodenum }
    // emit a clipnode
    if (*hull).lastclipnode == 32767 as libc::c_int {
        Host_Error(b"MAX_MAP_CLIPNODES limit exceeded\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    src = srcnodes.offset(nodenum as isize);
    c = (*hull).lastclipnode;
    out = &mut *(*hull).clipnodes.offset(c as isize) as *mut mclipnode_t;
    (*hull).lastclipnode += 1;
    (*out).planenum = (*src).planenum;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        (*out).children[i as usize] =
            RemapClipNodes_r(srcnodes, hull, (*src).children[i as usize]) as
                libc::c_short;
        i += 1
    }
    return c;
}
/*
=================
Mod_MakeHull0

Duplicate the drawing hull structure as a clipping hull
=================
*/
unsafe extern "C" fn Mod_MakeHull0() {
    let mut in_0: *mut mnode_t = 0 as *mut mnode_t;
    let mut child: *mut mnode_t = 0 as *mut mnode_t;
    let mut out: *mut mclipnode_t = 0 as *mut mclipnode_t;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    hull =
        &mut *(*loadmodel).hulls.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
            *mut hull_t;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   ((*loadmodel).numnodes as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<mclipnode_t>()
                                                        as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1281 as libc::c_int) as
            *mut mclipnode_t;
    (*hull).clipnodes = out;
    in_0 = (*loadmodel).nodes;
    (*hull).firstclipnode = 0 as libc::c_int;
    (*hull).lastclipnode = (*loadmodel).numnodes - 1 as libc::c_int;
    (*hull).planes = (*loadmodel).planes;
    i = 0 as libc::c_int;
    while i < (*loadmodel).numnodes {
        (*out).planenum =
            (*in_0).plane.wrapping_offset_from((*loadmodel).planes) as
                libc::c_long as libc::c_int;
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            child = (*in_0).children[j as usize];
            if (*child).contents < 0 as libc::c_int {
                (*out).children[j as usize] =
                    (*child).contents as libc::c_short
            } else {
                (*out).children[j as usize] =
                    child.wrapping_offset_from((*loadmodel).nodes) as
                        libc::c_long as libc::c_short
            }
            j += 1
        }
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1)
    };
}
/*
=================
Mod_SetupHull
=================
*/
unsafe extern "C" fn Mod_SetupHull(mut bmod: *mut dbspmodel_t,
                                   mut mod_0: *mut model_t,
                                   mut mempool: poolhandle_t,
                                   mut headnode: libc::c_int,
                                   mut hullnum: libc::c_int) {
    let mut hull: *mut hull_t =
        &mut *(*mod_0).hulls.as_mut_ptr().offset(hullnum as isize) as
            *mut hull_t;
    let mut count: libc::c_int = 0;
    // assume no hull
    (*hull).lastclipnode = 0 as libc::c_int; // hull is missed
    (*hull).firstclipnode = (*hull).lastclipnode; // hull missed
    (*hull).planes = 0 as *mut mplane_t; // ZHLT weird empty hulls
    if headnode == -(1 as libc::c_int) ||
           hullnum != 1 as libc::c_int && headnode == 0 as libc::c_int {
        return
    } // copy human hull
    if headnode >= (*mod_0).numclipnodes { return } // copy large hull
    match hullnum {
        1 => {
            (*hull).clip_mins[0 as libc::c_int as usize] =
                host.player_mins[0 as libc::c_int as
                                     usize][0 as libc::c_int as
                                                usize]; // copy head hull
            (*hull).clip_mins[1 as libc::c_int as usize] =
                host.player_mins[0 as libc::c_int as
                                     usize][1 as libc::c_int as
                                                usize]; // no hull specified
            (*hull).clip_mins[2 as libc::c_int as usize] =
                host.player_mins[0 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            (*hull).clip_maxs[0 as libc::c_int as usize] =
                host.player_maxs[0 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            (*hull).clip_maxs[1 as libc::c_int as usize] =
                host.player_maxs[0 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            (*hull).clip_maxs[2 as libc::c_int as usize] =
                host.player_maxs[0 as libc::c_int as
                                     usize][2 as libc::c_int as usize]
        }
        2 => {
            (*hull).clip_mins[0 as libc::c_int as usize] =
                host.player_mins[3 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            (*hull).clip_mins[1 as libc::c_int as usize] =
                host.player_mins[3 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            (*hull).clip_mins[2 as libc::c_int as usize] =
                host.player_mins[3 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            (*hull).clip_maxs[0 as libc::c_int as usize] =
                host.player_maxs[3 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            (*hull).clip_maxs[1 as libc::c_int as usize] =
                host.player_maxs[3 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            (*hull).clip_maxs[2 as libc::c_int as usize] =
                host.player_maxs[3 as libc::c_int as
                                     usize][2 as libc::c_int as usize]
        }
        3 => {
            (*hull).clip_mins[0 as libc::c_int as usize] =
                host.player_mins[1 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            (*hull).clip_mins[1 as libc::c_int as usize] =
                host.player_mins[1 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            (*hull).clip_mins[2 as libc::c_int as usize] =
                host.player_mins[1 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            (*hull).clip_maxs[0 as libc::c_int as usize] =
                host.player_maxs[1 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            (*hull).clip_maxs[1 as libc::c_int as usize] =
                host.player_maxs[1 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            (*hull).clip_maxs[2 as libc::c_int as usize] =
                host.player_maxs[1 as libc::c_int as
                                     usize][2 as libc::c_int as usize]
        }
        _ => {
            Host_Error(b"Mod_SetupHull: bad hull number %i\n\x00" as *const u8
                           as *const libc::c_char, hullnum);
        }
    }
    if (*hull).clip_mins[0 as libc::c_int as usize] == 0.0f32 &&
           (*hull).clip_mins[1 as libc::c_int as usize] == 0.0f32 &&
           (*hull).clip_mins[2 as libc::c_int as usize] == 0.0f32 &&
           ((*hull).clip_maxs[0 as libc::c_int as usize] == 0.0f32 &&
                (*hull).clip_maxs[1 as libc::c_int as usize] == 0.0f32 &&
                (*hull).clip_maxs[2 as libc::c_int as usize] == 0.0f32) {
        return
    }
    CountClipNodes32_r((*bmod).clipnodes_out, hull, headnode);
    count = (*hull).lastclipnode;
    // fit array to real count
    (*hull).clipnodes =
        _Mem_Alloc(mempool,
                   (::std::mem::size_of::<mclipnode_t>() as
                        libc::c_ulong).wrapping_mul((*hull).lastclipnode as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1349 as libc::c_int) as
            *mut mclipnode_t; // share planes
    (*hull).planes = (*mod_0).planes; // restart counting
    (*hull).lastclipnode = 0 as libc::c_int;
    // remap clipnodes to 16-bit indexes
    RemapClipNodes_r((*bmod).clipnodes_out, hull, headnode);
}
/*
=================
Mod_LoadColoredLighting
=================
*/
unsafe extern "C" fn Mod_LoadColoredLighting(mut bmod: *mut dbspmodel_t)
 -> qboolean {
    let mut modelname: [libc::c_char; 64] = [0; 64];
    let mut path: [libc::c_char; 64] = [0; 64];
    let mut iCompare: libc::c_int = 0;
    let mut litdatasize: fs_offset_t = 0;
    let mut in_0: *mut byte = 0 as *mut byte;
    COM_FileBase((*loadmodel).name.as_mut_ptr(), modelname.as_mut_ptr());
    Q_snprintf(path.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"maps/%s.lit\x00" as *const u8 as *const libc::c_char,
               modelname.as_mut_ptr());
    // make sure what deluxemap is actual
    if COM_CompareFileTime(path.as_mut_ptr(), (*loadmodel).name.as_mut_ptr(),
                           &mut iCompare) == 0 {
        return false_0
    }
    if iCompare < 0 as libc::c_int {
        // this may happens if level-designer used -onlyents key for hlcsg
        Con_Printf(b"^3Warning:^7 %s probably is out of date\n\x00" as
                       *const u8 as *const libc::c_char, path.as_mut_ptr());
    }
    in_0 = FS_LoadFile(path.as_mut_ptr(), &mut litdatasize, false_0);
    if *(in_0 as *mut uint) !=
           ((('T' as i32) << 24 as libc::c_int) +
                (('I' as i32) << 16 as libc::c_int) +
                (('L' as i32) << 8 as libc::c_int) + 'Q' as i32) as
               libc::c_uint ||
           *(in_0 as *mut uint).offset(1 as libc::c_int as isize) !=
               1 as libc::c_int as libc::c_uint {
        _Mem_Free(in_0 as *mut libc::c_void,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 1386 as libc::c_int);
        return false_0
    }
    // skip header bytes
    litdatasize -= 8 as libc::c_int as libc::c_long;
    if litdatasize as libc::c_ulong !=
           (*bmod).lightdatasize.wrapping_mul(3 as libc::c_int as
                                                  libc::c_ulong) {
        Con_Printf(b"^1Error:^7 %s has mismatched size (%li should be %lu)\n\x00"
                       as *const u8 as *const libc::c_char, path.as_mut_ptr(),
                   litdatasize,
                   (*bmod).lightdatasize.wrapping_mul(3 as libc::c_int as
                                                          libc::c_ulong));
        _Mem_Free(in_0 as *mut libc::c_void,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 1396 as libc::c_int);
        return false_0
    }
    (*loadmodel).lightdata =
        _Mem_Alloc((*loadmodel).mempool, litdatasize as size_t, false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1400 as libc::c_int) as
            *mut color24;
    memcpy((*loadmodel).lightdata as *mut libc::c_void,
           in_0.offset(8 as libc::c_int as isize) as *const libc::c_void,
           litdatasize as libc::c_ulong);
    (*loadmodel).flags =
        ((*loadmodel).flags as libc::c_uint |
             (1 as libc::c_uint) << 4 as libc::c_int) as libc::c_int;
    (*bmod).lightdatasize = litdatasize as size_t;
    _Mem_Free(in_0 as *mut libc::c_void,
              b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                  *const libc::c_char, 1404 as libc::c_int);
    return true_0;
}
/*
=================
Mod_LoadDeluxemap
=================
*/
unsafe extern "C" fn Mod_LoadDeluxemap(mut bmod: *mut dbspmodel_t) {
    let mut modelname: [libc::c_char; 64] = [0; 64];
    let mut deluxdatasize: fs_offset_t = 0;
    let mut path: [libc::c_char; 64] = [0; 64];
    let mut iCompare: libc::c_int = 0;
    let mut in_0: *mut byte = 0 as *mut byte;
    if host.features &
           ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint == 0 {
        return
    }
    COM_FileBase((*loadmodel).name.as_mut_ptr(), modelname.as_mut_ptr());
    Q_snprintf(path.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"maps/%s.dlit\x00" as *const u8 as *const libc::c_char,
               modelname.as_mut_ptr());
    // make sure what deluxemap is actual
    if COM_CompareFileTime(path.as_mut_ptr(), (*loadmodel).name.as_mut_ptr(),
                           &mut iCompare) == 0 {
        return
    }
    if iCompare < 0 as libc::c_int {
        // this may happens if level-designer used -onlyents key for hlcsg
        Con_Printf(b"^3Warning:^7 %s probably is out of date\n\x00" as
                       *const u8 as *const libc::c_char, path.as_mut_ptr());
    }
    in_0 = FS_LoadFile(path.as_mut_ptr(), &mut deluxdatasize, false_0);
    if *(in_0 as *mut uint) !=
           ((('T' as i32) << 24 as libc::c_int) +
                (('I' as i32) << 16 as libc::c_int) +
                (('L' as i32) << 8 as libc::c_int) + 'Q' as i32) as
               libc::c_uint ||
           *(in_0 as *mut uint).offset(1 as libc::c_int as isize) !=
               1 as libc::c_int as libc::c_uint {
        _Mem_Free(in_0 as *mut libc::c_void,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 1441 as libc::c_int);
        return
    }
    // skip header bytes
    deluxdatasize -= 8 as libc::c_int as libc::c_long;
    if deluxdatasize as libc::c_ulong != (*bmod).lightdatasize {
        Con_Reportf(b"^1Error:^7 %s has mismatched size (%li should be %lu)\n\x00"
                        as *const u8 as *const libc::c_char,
                    path.as_mut_ptr(), deluxdatasize, (*bmod).lightdatasize);
        _Mem_Free(in_0 as *mut libc::c_void,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 1451 as libc::c_int);
        return
    }
    (*bmod).deluxedata_out =
        _Mem_Alloc((*loadmodel).mempool, deluxdatasize as size_t, false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1455 as libc::c_int) as
            *mut color24;
    memcpy((*bmod).deluxedata_out as *mut libc::c_void,
           in_0.offset(8 as libc::c_int as isize) as *const libc::c_void,
           deluxdatasize as libc::c_ulong);
    (*bmod).deluxdatasize = deluxdatasize as size_t;
    _Mem_Free(in_0 as *mut libc::c_void,
              b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                  *const libc::c_char, 1458 as libc::c_int);
}
/*
=================
Mod_SetupSubmodels

duplicate the basic information
for embedded submodels
=================
*/
unsafe extern "C" fn Mod_SetupSubmodels(mut bmod: *mut dbspmodel_t) {
    let mut colored: qboolean = false_0; // regular and alternate animation
    let mut mempool: poolhandle_t = 0;
    let mut ents: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut bm: *mut dmodel_t = 0 as *mut dmodel_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    ents = (*loadmodel).entities;
    mempool = (*loadmodel).mempool;
    if (*loadmodel).flags as libc::c_uint &
           (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        colored = true_0
    }
    mod_0 = loadmodel;
    (*loadmodel).numframes = 2 as libc::c_int;
    // set up the submodels
    i = 0 as libc::c_int;
    while i < (*mod_0).numsubmodels {
        bm = &mut *(*mod_0).submodels.offset(i as isize) as *mut dmodel_t;
        // hull 0 is just shared across all bmodels
        (*mod_0).hulls[0 as libc::c_int as usize].firstclipnode =
            (*bm).headnode[0 as libc::c_int as
                               usize]; // need to be real count
        (*mod_0).hulls[0 as libc::c_int as usize].lastclipnode =
            (*bm).headnode[0 as libc::c_int as usize];
        // counting a real number of clipnodes per each submodel
        CountClipNodes_r((*mod_0).hulls[0 as libc::c_int as usize].clipnodes,
                         &mut *(*mod_0).hulls.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                         (*bm).headnode[0 as libc::c_int as usize]);
        // but hulls1-3 is build individually for a each given submodel
        j = 1 as libc::c_int;
        while j < 4 as libc::c_int {
            Mod_SetupHull(bmod, mod_0, mempool, (*bm).headnode[j as usize],
                          j);
            j += 1
        }
        (*mod_0).firstmodelsurface = (*bm).firstface;
        (*mod_0).nummodelsurfaces = (*bm).numfaces;
        (*mod_0).mins[0 as libc::c_int as usize] =
            (*bm).mins[0 as libc::c_int as usize];
        (*mod_0).mins[1 as libc::c_int as usize] =
            (*bm).mins[1 as libc::c_int as usize];
        (*mod_0).mins[2 as libc::c_int as usize] =
            (*bm).mins[2 as libc::c_int as usize];
        (*mod_0).maxs[0 as libc::c_int as usize] =
            (*bm).maxs[0 as libc::c_int as usize];
        (*mod_0).maxs[1 as libc::c_int as usize] =
            (*bm).maxs[1 as libc::c_int as usize];
        (*mod_0).maxs[2 as libc::c_int as usize] =
            (*bm).maxs[2 as libc::c_int as usize];
        (*mod_0).radius =
            RadiusFromBounds((*mod_0).mins.as_mut_ptr() as *const vec_t,
                             (*mod_0).maxs.as_mut_ptr() as *const vec_t);
        (*mod_0).numleafs = (*bm).visleafs;
        (*mod_0).flags = 0 as libc::c_int;
        // this bit will be shared between all the submodels include worldmodel
        if colored as u64 != 0 {
            (*mod_0).flags =
                ((*mod_0).flags as libc::c_uint |
                     (1 as libc::c_uint) << 4 as libc::c_int) as libc::c_int
        }
        if i != 0 as libc::c_int {
            Mod_FindModelOrigin(ents,
                                va(b"*%i\x00" as *const u8 as
                                       *const libc::c_char, i),
                                (*bm).origin.as_mut_ptr());
            // mark models that have origin brushes
            if !((*bm).origin[0 as libc::c_int as usize] == 0.0f32 &&
                     (*bm).origin[1 as libc::c_int as usize] == 0.0f32 &&
                     (*bm).origin[2 as libc::c_int as usize] == 0.0f32) {
                (*mod_0).flags =
                    ((*mod_0).flags as libc::c_uint |
                         (1 as libc::c_uint) << 1 as libc::c_int) as
                        libc::c_int
            }
            // c2a1 doesn't have origin brush it's just placed at center of the level
            if Q_strnicmp((*loadmodel).name.as_mut_ptr(),
                          b"maps/c2a1.bsp\x00" as *const u8 as
                              *const libc::c_char, 99999 as libc::c_int) == 0
                   && i == 11 as libc::c_int {
                (*mod_0).flags =
                    ((*mod_0).flags as libc::c_uint |
                         (1 as libc::c_uint) << 1 as libc::c_int) as
                        libc::c_int
            }
        }
        // sets the model flags
        j = 0 as libc::c_int;
        while i != 0 as libc::c_int && j < (*mod_0).nummodelsurfaces {
            let mut surf: *mut msurface_t =
                (*mod_0).surfaces.offset((*mod_0).firstmodelsurface as
                                             isize).offset(j as isize);
            if (*surf).flags as libc::c_uint &
                   (1 as libc::c_uint) << 6 as libc::c_int != 0 {
                (*mod_0).flags =
                    ((*mod_0).flags as libc::c_uint |
                         (1 as libc::c_uint) << 0 as libc::c_int) as
                        libc::c_int
            }
            if (*surf).flags as libc::c_uint &
                   (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                (*mod_0).flags =
                    ((*mod_0).flags as libc::c_uint |
                         (1 as libc::c_uint) << 3 as libc::c_int) as
                        libc::c_int
            }
            if (*surf).flags as libc::c_uint &
                   (1 as libc::c_uint) << 4 as libc::c_int != 0 {
                (*mod_0).flags =
                    ((*mod_0).flags as libc::c_uint |
                         (1 as libc::c_uint) << 2 as libc::c_int) as
                        libc::c_int
            }
            j += 1
        }
        if i < (*mod_0).numsubmodels - 1 as libc::c_int {
            let mut name: [libc::c_char; 8] = [0; 8];
            // duplicate the basic information
            Q_snprintf(name.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 8]>() as
                           libc::c_ulong,
                       b"*%i\x00" as *const u8 as *const libc::c_char,
                       i + 1 as libc::c_int);
            loadmodel = Mod_FindName(name.as_mut_ptr(), true_0);
            *loadmodel = *mod_0;
            Q_strncpy((*loadmodel).name.as_mut_ptr(), name.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong);
            (*loadmodel).mempool = 0 as libc::c_int as poolhandle_t;
            mod_0 = loadmodel
        }
        i += 1
    }
    if !(*bmod).clipnodes_out.is_null() {
        _Mem_Free((*bmod).clipnodes_out as *mut libc::c_void,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 1559 as libc::c_int);
    };
}
/*
===============================================================================

			MAP LOADING

===============================================================================
*/
/*
=================
Mod_LoadSubmodels
=================
*/
unsafe extern "C" fn Mod_LoadSubmodels(mut bmod: *mut dbspmodel_t) {
    let mut in_0: *mut dmodel_t = 0 as *mut dmodel_t;
    let mut out: *mut dmodel_t = 0 as *mut dmodel_t;
    let mut oldmaxfaces: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // allocate extradata for each dmodel_t
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numsubmodels.wrapping_mul(::std::mem::size_of::<dmodel_t>()
                                                         as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1581 as libc::c_int) as
            *mut dmodel_t;
    (*loadmodel).numsubmodels = (*bmod).numsubmodels as libc::c_int;
    (*loadmodel).submodels = out;
    in_0 = (*bmod).submodels;
    if (*bmod).isworld as u64 != 0 {
        refState.max_surfaces = 0 as libc::c_int
    }
    oldmaxfaces = refState.max_surfaces;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numsubmodels {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            // reset empty bounds to prevent error
            if (*in_0).mins[j as usize] == 999999.0f32 {
                (*in_0).mins[j as usize] = 0.0f32
            }
            if (*in_0).maxs[j as usize] == -999999.0f32 {
                (*in_0).maxs[j as usize] = 0.0f32
            }
            // spread the mins / maxs by a unit
            (*out).mins[j as usize] =
                (*in_0).mins[j as usize] -
                    1.0f32; // skip the world to save mem
            (*out).maxs[j as usize] = (*in_0).maxs[j as usize] + 1.0f32;
            (*out).origin[j as usize] = (*in_0).origin[j as usize];
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            (*out).headnode[j as usize] = (*in_0).headnode[j as usize];
            j += 1
        }
        (*out).visleafs = (*in_0).visleafs;
        (*out).firstface = (*in_0).firstface;
        (*out).numfaces = (*in_0).numfaces;
        if !(i == 0 as libc::c_int && (*bmod).isworld as libc::c_uint != 0) {
            oldmaxfaces =
                if oldmaxfaces > (*out).numfaces {
                    oldmaxfaces
                } else { (*out).numfaces }
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
    // these array used to sort translucent faces in bmodels
    if oldmaxfaces > refState.max_surfaces {
        refState.draw_surfaces =
            _Mem_Realloc(host.mempool,
                         refState.draw_surfaces as *mut libc::c_void,
                         (oldmaxfaces as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<sortedface_t>()
                                                              as
                                                              libc::c_ulong),
                         true_0,
                         b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                             *const libc::c_char, 1622 as libc::c_int) as
                *mut sortedface_t;
        refState.max_surfaces = oldmaxfaces
    };
}
/*
=================
Mod_LoadEntities
=================
*/
unsafe extern "C" fn Mod_LoadEntities(mut bmod: *mut dbspmodel_t) {
    let mut entpatch: *mut byte = 0 as *mut byte;
    let mut token: [libc::c_char; 2048] = [0; 2048];
    let mut wadstring: [libc::c_char; 2048] = [0; 2048];
    let mut keyname: string = [0; 256];
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*bmod).isworld as u64 != 0 {
        let mut entfilename: [libc::c_char; 64] = [0; 64];
        let mut entpatchsize: fs_offset_t = 0;
        let mut ft1: size_t = 0;
        let mut ft2: size_t = 0;
        // world is check for entfile too
        Q_strncpy(entfilename.as_mut_ptr(), (*loadmodel).name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        COM_ReplaceExtension(entfilename.as_mut_ptr(),
                             b".ent\x00" as *const u8 as *const libc::c_char);
        // make sure what entity patch is never than bsp
        ft1 = FS_FileTime((*loadmodel).name.as_mut_ptr(), false_0) as size_t;
        ft2 = FS_FileTime(entfilename.as_mut_ptr(), true_0) as size_t;
        if ft2 != -(1 as libc::c_int) as libc::c_ulong {
            if ft1 > ft2 {
                Con_Printf(b"^3Warning:^7 Entity patch is older than bsp. Ignored.\n\x00"
                               as *const u8 as *const libc::c_char);
            } else {
                entpatch =
                    FS_LoadFile(entfilename.as_mut_ptr(), &mut entpatchsize,
                                true_0);
                if !entpatch.is_null() {
                    Con_Printf(b"^2Read entity patch:^7 %s\n\x00" as *const u8
                                   as *const libc::c_char,
                               entfilename.as_mut_ptr());
                    (*bmod).entdatasize = entpatchsize as size_t;
                    (*bmod).entdata = entpatch
                }
            }
        }
    }
    // make sure what we really has terminator
    (*loadmodel).entities =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).entdatasize.wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1670 as libc::c_int) as
            *mut libc::c_char; // moving to private model pool
    memcpy((*loadmodel).entities as *mut libc::c_void,
           (*bmod).entdata as *const libc::c_void,
           (*bmod).entdatasize); // release entpatch if present
    if !entpatch.is_null() {
        _Mem_Free(entpatch as *mut libc::c_void,
                  b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                      *const libc::c_char, 1672 as libc::c_int);
    }
    if (*bmod).isworld as u64 == 0 { return }
    pfile = (*loadmodel).entities;
    world.generator[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    world.compiler[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    world.message[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*bmod).wadlist.count = 0 as libc::c_int;
    // parse all the wads for loading textures in right ordering
    pfile =
        _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 2048]>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if !pfile.is_null() {
        if token[0 as libc::c_int as usize] as libc::c_int != '{' as i32 {
            Host_Error(b"Mod_LoadEntities: found %s when expecting {\n\x00" as
                           *const u8 as *const libc::c_char,
                       token.as_mut_ptr());
        }
        loop  {
            // all done
            // parse key
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int); // end of desc
            if pfile.is_null() {
                Host_Error(b"Mod_LoadEntities: EOF without closing brace\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if token[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
                break ;
            }
            Q_strncpy(keyname.as_mut_ptr(), token.as_mut_ptr(),
                      ::std::mem::size_of::<string>() as libc::c_ulong);
            // parse value
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if pfile.is_null() {
                Host_Error(b"Mod_LoadEntities: EOF without closing brace\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if token[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
                Host_Error(b"Mod_LoadEntities: closing brace without data\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if Q_strnicmp(keyname.as_mut_ptr(),
                          b"wad\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                let mut pszWadFile: *mut libc::c_char =
                    0 as *mut libc::c_char;
                Q_strncpy(wadstring.as_mut_ptr(), token.as_mut_ptr(),
                          (2048 as libc::c_int - 2 as libc::c_int) as size_t);
                wadstring[(2048 as libc::c_int - 2 as libc::c_int) as usize] =
                    0 as libc::c_int as libc::c_char;
                if Q_strchr(wadstring.as_mut_ptr(),
                            ';' as i32 as libc::c_char).is_null() {
                    Q_strncat(wadstring.as_mut_ptr(),
                              b";\x00" as *const u8 as *const libc::c_char,
                              99999 as libc::c_int as size_t);
                }
                // parse wad pathes
                pszWadFile =
                    strtok(wadstring.as_mut_ptr(),
                           b";\x00" as *const u8 as *const libc::c_char);
                while !pszWadFile.is_null() {
                    COM_FixSlashes(pszWadFile);
                    COM_FileBase(pszWadFile, token.as_mut_ptr());
                    // too many wads...
                    // make sure what wad is really exist
                    if FS_FileExists(va(b"%s.wad\x00" as *const u8 as
                                            *const libc::c_char,
                                        token.as_mut_ptr()),
                                     false_0 as libc::c_int) != 0 {
                        let fresh8 = (*bmod).wadlist.count;
                        (*bmod).wadlist.count = (*bmod).wadlist.count + 1;
                        let mut num: libc::c_int = fresh8;
                        Q_strncpy((*bmod).wadlist.wadnames[num as
                                                               usize].as_mut_ptr(),
                                  token.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 32]>()
                                      as libc::c_ulong);
                        (*bmod).wadlist.wadusage[num as usize] =
                            0 as libc::c_int
                    }
                    if (*bmod).wadlist.count >= 256 as libc::c_int { break ; }
                    pszWadFile =
                        strtok(0 as *mut libc::c_char,
                               b";\x00" as *const u8 as *const libc::c_char)
                }
            } else if Q_strnicmp(keyname.as_mut_ptr(),
                                 b"message\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 {
                Q_strncpy(world.message.as_mut_ptr(), token.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 2048]>() as
                              libc::c_ulong);
            } else if Q_strnicmp(keyname.as_mut_ptr(),
                                 b"compiler\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 ||
                          Q_strnicmp(keyname.as_mut_ptr(),
                                     b"_compiler\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
                Q_strncpy(world.compiler.as_mut_ptr(), token.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong);
            } else if Q_strnicmp(keyname.as_mut_ptr(),
                                 b"generator\x00" as *const u8 as
                                     *const libc::c_char,
                                 99999 as libc::c_int) == 0 ||
                          Q_strnicmp(keyname.as_mut_ptr(),
                                     b"_generator\x00" as *const u8 as
                                         *const libc::c_char,
                                     99999 as libc::c_int) == 0 {
                Q_strncpy(world.generator.as_mut_ptr(), token.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong);
            }
        }
        return
    };
}
/*
=================
Mod_LoadPlanes
=================
*/
unsafe extern "C" fn Mod_LoadPlanes(mut bmod: *mut dbspmodel_t) {
    let mut in_0: *mut dplane_t = 0 as *mut dplane_t;
    let mut out: *mut mplane_t = 0 as *mut mplane_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    in_0 = (*bmod).planes;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numplanes.wrapping_mul(::std::mem::size_of::<mplane_t>()
                                                      as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1754 as libc::c_int) as
            *mut mplane_t;
    (*loadmodel).planes = out;
    (*loadmodel).numplanes = (*bmod).numplanes as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numplanes {
        (*out).signbits = 0 as libc::c_int as byte;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out).normal[j as usize] = (*in_0).normal[j as usize];
            if (*out).normal[j as usize] < 0.0f32 {
                (*out).signbits =
                    ((*out).signbits as libc::c_uint |
                         (1 as libc::c_uint) << j) as byte
            }
            j += 1
        }
        if __tg_sqrt((*out).normal[0 as libc::c_int as usize] *
                         (*out).normal[0 as libc::c_int as usize] +
                         (*out).normal[1 as libc::c_int as usize] *
                             (*out).normal[1 as libc::c_int as usize] +
                         (*out).normal[2 as libc::c_int as usize] *
                             (*out).normal[2 as libc::c_int as usize]) <
               0.5f32 {
            Con_Printf(b"^1Error:^7 bad normal for plane #%i\n\x00" as
                           *const u8 as *const libc::c_char, i);
        }
        (*out).dist = (*in_0).dist;
        (*out).type_0 = (*in_0).type_0 as byte;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    };
}
/*
=================
Mod_LoadVertexes
=================
*/
unsafe extern "C" fn Mod_LoadVertexes(mut bmod: *mut dbspmodel_t) {
    let mut in_0: *mut dvertex_t = 0 as *mut dvertex_t;
    let mut out: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut i: libc::c_int = 0;
    in_0 = (*bmod).vertexes;
    (*loadmodel).vertexes =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numvertexes.wrapping_mul(::std::mem::size_of::<mvertex_t>()
                                                        as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1788 as libc::c_int) as
            *mut mvertex_t;
    out = (*loadmodel).vertexes;
    (*loadmodel).numvertexes = (*bmod).numvertexes as libc::c_int;
    if (*bmod).isworld as u64 != 0 {
        ClearBounds(world.mins.as_mut_ptr(), world.maxs.as_mut_ptr());
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numvertexes {
        if (*bmod).isworld as u64 != 0 {
            AddPointToBounds((*in_0).point.as_mut_ptr() as *const vec_t,
                             world.mins.as_mut_ptr(),
                             world.maxs.as_mut_ptr());
        }
        (*out).position[0 as libc::c_int as usize] =
            (*in_0).point[0 as libc::c_int as usize];
        (*out).position[1 as libc::c_int as usize] =
            (*in_0).point[1 as libc::c_int as usize];
        (*out).position[2 as libc::c_int as usize] =
            (*in_0).point[2 as libc::c_int as usize];
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
    if (*bmod).isworld as u64 == 0 { return }
    world.size[0 as libc::c_int as usize] =
        world.maxs[0 as libc::c_int as usize] -
            world.mins[0 as libc::c_int as usize];
    world.size[1 as libc::c_int as usize] =
        world.maxs[1 as libc::c_int as usize] -
            world.mins[1 as libc::c_int as usize];
    world.size[2 as libc::c_int as usize] =
        world.maxs[2 as libc::c_int as usize] -
            world.mins[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        // spread the mins / maxs by a pixel
        world.mins[i as usize] -= 1.0f32;
        world.maxs[i as usize] += 1.0f32;
        i += 1
    };
}
/*
=================
Mod_LoadEdges
=================
*/
unsafe extern "C" fn Mod_LoadEdges(mut bmod: *mut dbspmodel_t) {
    let mut out: *mut medge_t = 0 as *mut medge_t;
    let mut i: libc::c_int = 0;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numedges.wrapping_mul(::std::mem::size_of::<medge_t>()
                                                     as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1822 as libc::c_int) as
            *mut medge_t;
    (*loadmodel).edges = out;
    (*loadmodel).numedges = (*bmod).numedges as libc::c_int;
    if (*bmod).version ==
           ('B' as i32) << 0 as libc::c_int | ('S' as i32) << 8 as libc::c_int
               | ('P' as i32) << 16 as libc::c_int |
               ('2' as i32) << 24 as libc::c_int {
        let mut in_0: *mut dedge32_t = (*bmod).c2rust_unnamed_3.edges32;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*bmod).numedges {
            (*out).v[0 as libc::c_int as usize] =
                (*in_0).v[0 as libc::c_int as usize] as libc::c_ushort;
            (*out).v[1 as libc::c_int as usize] =
                (*in_0).v[1 as libc::c_int as usize] as libc::c_ushort;
            i += 1;
            in_0 = in_0.offset(1);
            out = out.offset(1)
        }
    } else {
        let mut in_1: *mut dedge_t = (*bmod).c2rust_unnamed_3.edges;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*bmod).numedges {
            (*out).v[0 as libc::c_int as usize] =
                (*in_1).v[0 as libc::c_int as usize];
            (*out).v[1 as libc::c_int as usize] =
                (*in_1).v[1 as libc::c_int as usize];
            i += 1;
            in_1 = in_1.offset(1);
            out = out.offset(1)
        }
    };
}
/*
=================
Mod_LoadSurfEdges
=================
*/
unsafe extern "C" fn Mod_LoadSurfEdges(mut bmod: *mut dbspmodel_t) {
    (*loadmodel).surfedges =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numsurfedges.wrapping_mul(::std::mem::size_of::<dsurfedge_t>()
                                                         as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1854 as libc::c_int) as
            *mut libc::c_int;
    memcpy((*loadmodel).surfedges as *mut libc::c_void,
           (*bmod).surfedges as *const libc::c_void,
           (*bmod).numsurfedges.wrapping_mul(::std::mem::size_of::<dsurfedge_t>()
                                                 as libc::c_ulong));
    (*loadmodel).numsurfedges = (*bmod).numsurfedges as libc::c_int;
}
/*
=================
Mod_LoadMarkSurfaces
=================
*/
unsafe extern "C" fn Mod_LoadMarkSurfaces(mut bmod: *mut dbspmodel_t) {
    let mut out: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
    let mut i: libc::c_int = 0;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).nummarkfaces.wrapping_mul(::std::mem::size_of::<*mut msurface_t>()
                                                         as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1869 as libc::c_int) as
            *mut *mut msurface_t;
    (*loadmodel).marksurfaces = out;
    (*loadmodel).nummarksurfaces = (*bmod).nummarkfaces as libc::c_int;
    if (*bmod).version ==
           ('B' as i32) << 0 as libc::c_int | ('S' as i32) << 8 as libc::c_int
               | ('P' as i32) << 16 as libc::c_int |
               ('2' as i32) << 24 as libc::c_int {
        let mut in_0: *mut dmarkface32_t =
            (*bmod).c2rust_unnamed_2.markfaces32;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*bmod).nummarkfaces {
            if *in_0 < 0 as libc::c_int || *in_0 >= (*loadmodel).numsurfaces {
                Host_Error(b"Mod_LoadMarkFaces: bad surface number in \'%s\'\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*loadmodel).name.as_mut_ptr());
            }
            let ref mut fresh9 = *out.offset(i as isize);
            *fresh9 = (*loadmodel).surfaces.offset(*in_0 as isize);
            i += 1;
            in_0 = in_0.offset(1)
        }
    } else {
        let mut in_1: *mut dmarkface_t = (*bmod).c2rust_unnamed_2.markfaces;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*bmod).nummarkfaces {
            if (*in_1 as libc::c_int) < 0 as libc::c_int ||
                   *in_1 as libc::c_int >= (*loadmodel).numsurfaces {
                Host_Error(b"Mod_LoadMarkFaces: bad surface number in \'%s\'\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*loadmodel).name.as_mut_ptr());
            }
            let ref mut fresh10 = *out.offset(i as isize);
            *fresh10 =
                (*loadmodel).surfaces.offset(*in_1 as libc::c_int as isize);
            i += 1;
            in_1 = in_1.offset(1)
        }
    };
}
/*
=================
Mod_LoadTextures
=================
*/
unsafe extern "C" fn Mod_LoadTextures(mut bmod: *mut dbspmodel_t) {
    let mut in_0: *mut dmiptexlump_t = 0 as *mut dmiptexlump_t;
    let mut tx: *mut texture_t = 0 as *mut texture_t;
    let mut tx2: *mut texture_t = 0 as *mut texture_t;
    let mut anims: [*mut texture_t; 10] = [0 as *mut texture_t; 10];
    let mut altanims: [*mut texture_t; 10] = [0 as *mut texture_t; 10];
    let mut num: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut altmax: libc::c_int = 0;
    let mut custom_palette: qboolean = false_0;
    let mut texname: [libc::c_char; 64] = [0; 64];
    let mut mt: *mut mip_t = 0 as *mut mip_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*bmod).isworld as u64 != 0 {
        // release old sky layers first
        if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
            ref_0.dllFuncs.GL_FreeTexture.expect("non-null function pointer")(ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"alpha_sky\x00"
                                                                                                                                                    as
                                                                                                                                                    *const u8
                                                                                                                                                    as
                                                                                                                                                    *const libc::c_char,
                                                                                                                                                0
                                                                                                                                                    as
                                                                                                                                                    *const byte,
                                                                                                                                                0
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    size_t,
                                                                                                                                                0
                                                                                                                                                    as
                                                                                                                                                    libc::c_int)
                                                                                  as
                                                                                  libc::c_uint);
            ref_0.dllFuncs.GL_FreeTexture.expect("non-null function pointer")(ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"solid_sky\x00"
                                                                                                                                                    as
                                                                                                                                                    *const u8
                                                                                                                                                    as
                                                                                                                                                    *const libc::c_char,
                                                                                                                                                0
                                                                                                                                                    as
                                                                                                                                                    *const byte,
                                                                                                                                                0
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    size_t,
                                                                                                                                                0
                                                                                                                                                    as
                                                                                                                                                    libc::c_int)
                                                                                  as
                                                                                  libc::c_uint);
        }
    }
    if (*bmod).texdatasize == 0 {
        // no textures
        (*loadmodel).textures = 0 as *mut *mut texture_t;
        return
    }
    in_0 = (*bmod).textures;
    (*loadmodel).textures =
        _Mem_Alloc((*loadmodel).mempool,
                   ((*in_0).nummiptex as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut texture_t>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 1933 as libc::c_int) as
            *mut *mut texture_t;
    (*loadmodel).numtextures = (*in_0).nummiptex;
    i = 0 as libc::c_int;
    while i < (*loadmodel).numtextures {
        let mut txFlags: libc::c_int = 0 as libc::c_int;
        if (*in_0).dataofs[i as usize] == -(1 as libc::c_int) {
            // create default texture (some mods requires this)
            tx =
                _Mem_Alloc((*loadmodel).mempool,
                           ::std::mem::size_of::<texture_t>() as
                               libc::c_ulong, true_0,
                           b"../engine/common/mod_bmodel.c\x00" as *const u8
                               as *const libc::c_char, 1943 as libc::c_int) as
                    *mut texture_t;
            let ref mut fresh11 = *(*loadmodel).textures.offset(i as isize);
            *fresh11 = tx;
            Q_strncpy((*tx).name.as_mut_ptr(),
                      b"*default\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong);
            if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint)
               {
                (*tx).gl_texturenum =
                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*default\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      0
                                                                                          as
                                                                                          *const byte,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          size_t,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int);
                (*tx).height = 16 as libc::c_int as libc::c_uint;
                (*tx).width = (*tx).height
            }
            // missed
        } else {
            mt =
                (in_0 as
                     *mut byte).offset((*in_0).dataofs[i as usize] as isize)
                    as *mut mip_t;
            if (*mt).name[0 as libc::c_int as usize] == 0 {
                Q_snprintf((*mt).name.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 16]>() as
                               libc::c_ulong,
                           b"miptex_%i\x00" as *const u8 as
                               *const libc::c_char, i);
            }
            tx =
                _Mem_Alloc((*loadmodel).mempool,
                           ::std::mem::size_of::<texture_t>() as
                               libc::c_ulong, true_0,
                           b"../engine/common/mod_bmodel.c\x00" as *const u8
                               as *const libc::c_char, 1961 as libc::c_int) as
                    *mut texture_t;
            let ref mut fresh12 = *(*loadmodel).textures.offset(i as isize);
            *fresh12 = tx;
            // convert to lowercase
            Q_strncpy((*tx).name.as_mut_ptr(), (*mt).name.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong); // Paranoia2 texture alpha-tracing
            Q_strnlwr((*tx).name.as_mut_ptr(), (*tx).name.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong);
            custom_palette = false_0;
            (*tx).width = (*mt).width;
            (*tx).height = (*mt).height;
            if host.features &
                   ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint !=
                   0 &&
                   (*mt).name[0 as libc::c_int as usize] as libc::c_int ==
                       '{' as i32 {
                txFlags = txFlags | TF_KEEP_SOURCE as libc::c_int
            }
            if (*mt).offsets[0 as libc::c_int as usize] >
                   0 as libc::c_int as libc::c_uint {
                let mut size: libc::c_int =
                    (::std::mem::size_of::<mip_t>() as libc::c_ulong as
                         libc::c_int as
                         libc::c_uint).wrapping_add((*mt).width.wrapping_mul((*mt).height).wrapping_mul(85
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint)
                                                        >> 6 as libc::c_int)
                        as libc::c_int;
                let mut next_dataofs: libc::c_int = 0 as libc::c_int;
                let mut remaining: libc::c_int = 0;
                // compute next dataofset to determine allocated miptex space
                j = i + 1 as libc::c_int;
                while j < (*loadmodel).numtextures {
                    next_dataofs = (*in_0).dataofs[j as usize];
                    if next_dataofs != -(1 as libc::c_int) { break ; }
                    j += 1
                }
                if j == (*loadmodel).numtextures {
                    next_dataofs = (*bmod).texdatasize as libc::c_int
                }
                // NOTE: imagelib detect miptex version by size
			// 770 additional bytes is indicated custom palette
                remaining =
                    next_dataofs - ((*in_0).dataofs[i as usize] + size);
                if remaining >= 770 as libc::c_int { custom_palette = true_0 }
            }
            if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint)
               {
                // check for multi-layered sky texture (quake1 specific)
                if (*bmod).isworld as libc::c_uint != 0 &&
                       Q_strncmp((*mt).name.as_mut_ptr(),
                                 b"sky\x00" as *const u8 as
                                     *const libc::c_char, 3 as libc::c_int) ==
                           0 &&
                       (*mt).width.wrapping_div((*mt).height) ==
                           2 as libc::c_int as libc::c_uint {
                    ref_0.dllFuncs.R_InitSkyClouds.expect("non-null function pointer")(mt,
                                                                                       tx,
                                                                                       custom_palette); // load quake sky
                    if ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"solid_sky\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         0
                                                                                             as
                                                                                             *const byte,
                                                                                         0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             size_t,
                                                                                         0
                                                                                             as
                                                                                             libc::c_int)
                           != 0 &&
                           ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"alpha_sky\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             0
                                                                                                 as
                                                                                                 *const byte,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 size_t,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int)
                               != 0 {
                        world.flags =
                            (world.flags as libc::c_uint |
                                 (1 as libc::c_uint) << 0 as libc::c_int) as
                                libc::c_int
                    }
                } else {
                    // texture loading order:
			// 1. from wad
			// 2. internal from map
                    // trying wad texture (force while r_wadtextures is 1)
                    if (*r_wadtextures).value != 0. &&
                           (*bmod).wadlist.count > 0 as libc::c_int ||
                           (*mt).offsets[0 as libc::c_int as usize] <=
                               0 as libc::c_int as libc::c_uint {
                        Q_snprintf(texname.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong,
                                   b"%s.mip\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*mt).name.as_mut_ptr());
                        // check wads in reverse order
                        j =
                            (*bmod).wadlist.count -
                                1 as libc::c_int; // this wad are really used
                        while j >= 0 as libc::c_int {
                            let mut texpath: *mut libc::c_char =
                                va(b"%s.wad/%s\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*bmod).wadlist.wadnames[j as
                                                                usize].as_mut_ptr(),
                                   texname.as_mut_ptr());
                            if FS_FileExists(texpath, false_0 as libc::c_int)
                                   != 0 {
                                (*tx).gl_texturenum =
                                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(texpath,
                                                                                                      0
                                                                                                          as
                                                                                                          *const byte,
                                                                                                      0
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          size_t,
                                                                                                      TF_ALLOW_EMBOSS
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          |
                                                                                                          txFlags);
                                (*bmod).wadlist.wadusage[j as usize] += 1;
                                break ;
                            } else { j -= 1 }
                        }
                    }
                    // wad failed, so use internal texture (if present)
                    if (*mt).offsets[0 as libc::c_int as usize] >
                           0 as libc::c_int as libc::c_uint &&
                           (*tx).gl_texturenum == 0 {
                        // NOTE: imagelib detect miptex version by size
				// 770 additional bytes is indicated custom palette
                        let mut size_0: libc::c_int =
                            (::std::mem::size_of::<mip_t>() as libc::c_ulong
                                 as libc::c_int as
                                 libc::c_uint).wrapping_add((*mt).width.wrapping_mul((*mt).height).wrapping_mul(85
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint)
                                                                >>
                                                                6 as
                                                                    libc::c_int)
                                as libc::c_int;
                        if custom_palette as u64 != 0 {
                            size_0 =
                                (size_0 as
                                     libc::c_ulong).wrapping_add((::std::mem::size_of::<libc::c_short>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(768
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_int as libc::c_int
                        }
                        Q_snprintf(texname.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong,
                                   b"#%s:%s.mip\x00" as *const u8 as
                                       *const libc::c_char,
                                   loadstat.name.as_mut_ptr(),
                                   (*mt).name.as_mut_ptr());
                        (*tx).gl_texturenum =
                            ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(texname.as_mut_ptr(),
                                                                                              mt
                                                                                                  as
                                                                                                  *mut byte,
                                                                                              size_0
                                                                                                  as
                                                                                                  size_t,
                                                                                              TF_ALLOW_EMBOSS
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  |
                                                                                                  txFlags)
                    }
                    // if texture is completely missed
                    if (*tx).gl_texturenum == 0 {
                        Con_DPrintf(b"^1Error:^7 unable to find %s.mip\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    (*mt).name.as_mut_ptr());
                        (*tx).gl_texturenum =
                            ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*default\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char,
                                                                                              0
                                                                                                  as
                                                                                                  *const byte,
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  size_t,
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int)
                    }
                    // check for luma texture
                    if ref_0.dllFuncs.RefGetParm.expect("non-null function pointer")(10
                                                                                         as
                                                                                         libc::c_int,
                                                                                     (*tx).gl_texturenum)
                           & TF_HAS_LUMA as libc::c_int != 0 {
                        Q_snprintf(texname.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 64]>()
                                       as libc::c_ulong,
                                   b"#%s:%s_luma.mip\x00" as *const u8 as
                                       *const libc::c_char,
                                   loadstat.name.as_mut_ptr(),
                                   (*mt).name.as_mut_ptr());
                        if (*mt).offsets[0 as libc::c_int as usize] >
                               0 as libc::c_int as libc::c_uint {
                            // NOTE: imagelib detect miptex version by size
					// 770 additional bytes is indicated custom palette
                            let mut size_1: libc::c_int =
                                (::std::mem::size_of::<mip_t>() as
                                     libc::c_ulong as libc::c_int as
                                     libc::c_uint).wrapping_add((*mt).width.wrapping_mul((*mt).height).wrapping_mul(85
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)
                                                                    >>
                                                                    6 as
                                                                        libc::c_int)
                                    as libc::c_int;
                            if custom_palette as u64 != 0 {
                                size_1 =
                                    (size_1 as
                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<libc::c_short>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(768
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong))
                                        as libc::c_int as libc::c_int
                            }
                            (*tx).fb_texturenum =
                                ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(texname.as_mut_ptr(),
                                                                                                  mt
                                                                                                      as
                                                                                                      *mut byte,
                                                                                                  size_1
                                                                                                      as
                                                                                                      size_t,
                                                                                                  TF_MAKELUMA
                                                                                                      as
                                                                                                      libc::c_int)
                                    as libc::c_ushort
                        } else {
                            let mut srcSize: fs_offset_t =
                                0 as libc::c_int as fs_offset_t;
                            let mut src: *mut byte = 0 as *mut byte;
                            // NOTE: we can't loading it from wad as normal because _luma texture doesn't exist
					// and not be loaded. But original texture is already loaded and can't be modified
					// So load original texture manually and convert it to luma
                            // check wads in reverse order
                            j =
                                (*bmod).wadlist.count -
                                    1 as
                                        libc::c_int; // this wad are really used
                            while j >= 0 as libc::c_int {
                                let mut texpath_0: *mut libc::c_char =
                                    va(b"%s.wad/%s.mip\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*bmod).wadlist.wadnames[j as
                                                                    usize].as_mut_ptr(),
                                       (*tx).name.as_mut_ptr());
                                if FS_FileExists(texpath_0,
                                                 false_0 as libc::c_int) != 0
                                   {
                                    src =
                                        FS_LoadFile(texpath_0, &mut srcSize,
                                                    false_0);
                                    (*bmod).wadlist.wadusage[j as usize] += 1;
                                    break ;
                                } else { j -= 1 }
                            }
                            // okay, loading it from wad or hi-res version
                            (*tx).fb_texturenum =
                                ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(texname.as_mut_ptr(),
                                                                                                  src,
                                                                                                  srcSize
                                                                                                      as
                                                                                                      size_t,
                                                                                                  TF_MAKELUMA
                                                                                                      as
                                                                                                      libc::c_int)
                                    as libc::c_ushort;
                            if !src.is_null() {
                                _Mem_Free(src as *mut libc::c_void,
                                          b"../engine/common/mod_bmodel.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          2090 as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    // sequence the animations and detail textures
    i = 0 as libc::c_int; // already sequenced
    while i < (*loadmodel).numtextures {
        tx = *(*loadmodel).textures.offset(i as isize);
        if !(tx.is_null() ||
                 (*tx).name[0 as libc::c_int as usize] as libc::c_int !=
                     '-' as i32 &&
                     (*tx).name[0 as libc::c_int as usize] as libc::c_int !=
                         '+' as i32) {
            if (*tx).anim_next.is_null() {
                // find the number of frames in the animation
                memset(anims.as_mut_ptr() as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<[*mut texture_t; 10]>() as
                           libc::c_ulong);
                memset(altanims.as_mut_ptr() as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<[*mut texture_t; 10]>() as
                           libc::c_ulong);
                max = (*tx).name[1 as libc::c_int as usize] as libc::c_int;
                altmax = 0 as libc::c_int;
                if max >= '0' as i32 && max <= '9' as i32 {
                    max -= '0' as i32;
                    altmax = 0 as libc::c_int;
                    anims[max as usize] = tx;
                    max += 1
                } else if max >= 'a' as i32 && max <= 'j' as i32 {
                    altmax = max - 'a' as i32;
                    max = 0 as libc::c_int;
                    altanims[altmax as usize] = tx;
                    altmax += 1
                } else {
                    Con_Printf(b"^1Error:^7 Mod_LoadTextures: bad animating texture %s\n\x00"
                                   as *const u8 as *const libc::c_char,
                               (*tx).name.as_mut_ptr());
                }
                j = i + 1 as libc::c_int;
                while j < (*loadmodel).numtextures {
                    tx2 = *(*loadmodel).textures.offset(j as isize);
                    if !(tx2.is_null() ||
                             (*tx2).name[0 as libc::c_int as usize] as
                                 libc::c_int != '-' as i32 &&
                                 (*tx2).name[0 as libc::c_int as usize] as
                                     libc::c_int != '+' as i32) {
                        if !(Q_strncmp((*tx2).name.as_mut_ptr().offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                                       (*tx).name.as_mut_ptr().offset(2 as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                                       99999 as libc::c_int) != 0) {
                            num =
                                (*tx2).name[1 as libc::c_int as usize] as
                                    libc::c_int;
                            if num >= '0' as i32 && num <= '9' as i32 {
                                num -= '0' as i32;
                                anims[num as usize] = tx2;
                                if num + 1 as libc::c_int > max {
                                    max = num + 1 as libc::c_int
                                }
                            } else if num >= 'a' as i32 && num <= 'j' as i32 {
                                num = num - 'a' as i32;
                                altanims[num as usize] = tx2;
                                if num + 1 as libc::c_int > altmax {
                                    altmax = num + 1 as libc::c_int
                                }
                            } else {
                                Con_Printf(b"^1Error:^7 Mod_LoadTextures: bad animating texture %s\n\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           (*tx).name.as_mut_ptr());
                            }
                        }
                    }
                    j += 1
                }
                // link them all together
                j = 0 as libc::c_int;
                while j < max {
                    tx2 = anims[j as usize];
                    if tx2.is_null() {
                        Con_Printf(b"^1Error:^7 Mod_LoadTextures: missing frame %i of %s\n\x00"
                                       as *const u8 as *const libc::c_char, j,
                                   (*tx).name.as_mut_ptr());
                        (*tx).anim_total = 0 as libc::c_int;
                        break ;
                    } else {
                        (*tx2).anim_total = max * 2 as libc::c_int;
                        (*tx2).anim_min = j * 2 as libc::c_int;
                        (*tx2).anim_max =
                            (j + 1 as libc::c_int) * 2 as libc::c_int;
                        (*tx2).anim_next =
                            anims[((j + 1 as libc::c_int) % max) as usize];
                        if altmax != 0 {
                            (*tx2).alternate_anims =
                                altanims[0 as libc::c_int as usize]
                        }
                        j += 1
                    }
                }
                j = 0 as libc::c_int;
                while j < altmax {
                    tx2 = altanims[j as usize];
                    if tx2.is_null() {
                        Con_Printf(b"^1Error:^7 Mod_LoadTextures: missing frame %i of %s\n\x00"
                                       as *const u8 as *const libc::c_char, j,
                                   (*tx).name.as_mut_ptr());
                        (*tx).anim_total = 0 as libc::c_int;
                        break ;
                    } else {
                        (*tx2).anim_total = altmax * 2 as libc::c_int;
                        (*tx2).anim_min = j * 2 as libc::c_int;
                        (*tx2).anim_max =
                            (j + 1 as libc::c_int) * 2 as libc::c_int;
                        (*tx2).anim_next =
                            altanims[((j + 1 as libc::c_int) % altmax) as
                                         usize];
                        if max != 0 {
                            (*tx2).alternate_anims =
                                anims[0 as libc::c_int as usize]
                        }
                        j += 1
                    }
                }
            }
        }
        i += 1
    };
}
/*
=================
Mod_LoadTexInfo
=================
*/
unsafe extern "C" fn Mod_LoadTexInfo(mut bmod: *mut dbspmodel_t) {
    let mut fout: *mut mfaceinfo_t = 0 as *mut mfaceinfo_t;
    let mut faceinfo: *mut mfaceinfo_t = 0 as *mut mfaceinfo_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut miptex: libc::c_int = 0;
    let mut fin: *mut dfaceinfo_t = 0 as *mut dfaceinfo_t;
    let mut out: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut in_0: *mut dtexinfo_t = 0 as *mut dtexinfo_t;
    // trying to load faceinfo
    fout =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numfaceinfo.wrapping_mul(::std::mem::size_of::<mfaceinfo_t>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2213 as libc::c_int) as
            *mut mfaceinfo_t; // this is possible?
    faceinfo = fout;
    fin = (*bmod).faceinfo;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numfaceinfo {
        Q_strncpy((*fout).landname.as_mut_ptr(), (*fin).landname.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 16]>() as
                      libc::c_ulong);
        (*fout).texture_step = (*fin).texture_step;
        (*fout).max_extent = (*fin).max_extent;
        (*fout).groupid = (*fin).groupid;
        i += 1;
        fin = fin.offset(1);
        fout = fout.offset(1)
    }
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numtexinfo.wrapping_mul(::std::mem::size_of::<mtexinfo_t>()
                                                       as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2224 as libc::c_int) as
            *mut mtexinfo_t;
    (*loadmodel).texinfo = out;
    (*loadmodel).numtexinfo = (*bmod).numtexinfo as libc::c_int;
    in_0 = (*bmod).texinfo;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numtexinfo {
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                (*out).vecs[j as usize][k as usize] =
                    (*in_0).vecs[j as usize][k as usize];
                k += 1
            }
            j += 1
        }
        miptex = (*in_0).miptex;
        if miptex < 0 as libc::c_int || miptex > (*loadmodel).numtextures {
            miptex = 0 as libc::c_int
        }
        (*out).texture = *(*loadmodel).textures.offset(miptex as isize);
        (*out).flags = (*in_0).flags as libc::c_int;
        // make sure what faceinfo is really exist
        if !faceinfo.is_null() &&
               (*in_0).faceinfo as libc::c_int != -(1 as libc::c_int) &&
               ((*in_0).faceinfo as libc::c_ulong) < (*bmod).numfaceinfo {
            (*out).faceinfo =
                &mut *faceinfo.offset((*in_0).faceinfo as isize) as
                    *mut mfaceinfo_t
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    };
}
/*
=================
Mod_LoadSurfaces
=================
*/
unsafe extern "C" fn Mod_LoadSurfaces(mut bmod: *mut dbspmodel_t) {
    let mut test_lightsize: libc::c_int = -(1 as libc::c_int);
    let mut next_lightofs: libc::c_int = -(1 as libc::c_int);
    let mut prev_lightofs: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lightofs: libc::c_int = 0;
    let mut info: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut out: *mut msurface_t = 0 as *mut msurface_t;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numsurfaces.wrapping_mul(::std::mem::size_of::<msurface_t>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2260 as libc::c_int) as
            *mut msurface_t;
    (*loadmodel).surfaces = out;
    info =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numsurfaces.wrapping_mul(::std::mem::size_of::<mextrasurf_t>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2261 as libc::c_int) as
            *mut mextrasurf_t;
    (*loadmodel).numsurfaces = (*bmod).numsurfaces as libc::c_int;
    // predict samplecount based on bspversion
    if (*bmod).version == 29 as libc::c_int ||
           (*bmod).version ==
               ('B' as i32) << 0 as libc::c_int |
                   ('S' as i32) << 8 as libc::c_int |
                   ('P' as i32) << 16 as libc::c_int |
                   ('2' as i32) << 24 as libc::c_int {
        (*bmod).lightmap_samples = 1 as libc::c_int
    } else { (*bmod).lightmap_samples = 3 as libc::c_int }
    let mut current_block_66: u64;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numsurfaces {
        let mut tex: *mut texture_t = 0 as *mut texture_t;
        // cut up polygon for warps
        // setup crosslinks between two parts of msurface_t
        (*out).info = info; // corrupted level?
        (*info).surf = out;
        if (*bmod).version ==
               ('B' as i32) << 0 as libc::c_int |
                   ('S' as i32) << 8 as libc::c_int |
                   ('P' as i32) << 16 as libc::c_int |
                   ('2' as i32) << 24 as libc::c_int {
            let mut in_0: *mut dface32_t =
                &mut *(*bmod).c2rust_unnamed_4.surfaces32.offset(i as isize)
                    as *mut dface32_t;
            if (*in_0).firstedge + (*in_0).numedges >
                   (*loadmodel).numsurfedges {
                current_block_66 = 13183875560443969876;
            } else {
                (*out).firstedge = (*in_0).firstedge;
                (*out).numedges = (*in_0).numedges;
                if (*in_0).side != 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 1 as libc::c_int) as
                            libc::c_int
                }
                (*out).plane =
                    (*loadmodel).planes.offset((*in_0).planenum as isize);
                (*out).texinfo =
                    (*loadmodel).texinfo.offset((*in_0).texinfo as isize);
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    (*out).styles[j as usize] = (*in_0).styles[j as usize];
                    j += 1
                }
                lightofs = (*in_0).lightofs;
                current_block_66 = 15597372965620363352;
            }
        } else {
            let mut in_1: *mut dface_t =
                &mut *(*bmod).c2rust_unnamed_4.surfaces.offset(i as isize) as
                    *mut dface_t;
            if (*in_1).firstedge + (*in_1).numedges as libc::c_int >
                   (*loadmodel).numsurfedges {
                Con_Reportf(b"^1Error:^7 bad surface %i from %lu\n\x00" as
                                *const u8 as *const libc::c_char, i,
                            (*bmod).numsurfaces);
                current_block_66 = 13183875560443969876;
            } else {
                (*out).firstedge = (*in_1).firstedge;
                (*out).numedges = (*in_1).numedges as libc::c_int;
                if (*in_1).side != 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 1 as libc::c_int) as
                            libc::c_int
                }
                (*out).plane =
                    (*loadmodel).planes.offset((*in_1).planenum as libc::c_int
                                                   as isize);
                (*out).texinfo =
                    (*loadmodel).texinfo.offset((*in_1).texinfo as libc::c_int
                                                    as isize);
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    (*out).styles[j as usize] = (*in_1).styles[j as usize];
                    j += 1
                }
                lightofs = (*in_1).lightofs;
                current_block_66 = 15597372965620363352;
            }
        }
        match current_block_66 {
            15597372965620363352 => {
                tex = (*(*out).texinfo).texture;
                if Q_strncmp((*tex).name.as_mut_ptr(),
                             b"sky\x00" as *const u8 as *const libc::c_char,
                             3 as libc::c_int) == 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 2 as libc::c_int) as
                            libc::c_int
                }
                if (*tex).name[0 as libc::c_int as usize] as libc::c_int ==
                       '*' as i32 &&
                       Q_strnicmp((*tex).name.as_mut_ptr(),
                                  b"*default\x00" as *const u8 as
                                      *const libc::c_char,
                                  99999 as libc::c_int) != 0 ||
                       (*tex).name[0 as libc::c_int as usize] as libc::c_int
                           == '!' as i32 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 4 as libc::c_int) as
                            libc::c_int
                }
                if Host_IsQuakeCompatible() as u64 == 0 {
                    if Q_strncmp((*tex).name.as_mut_ptr(),
                                 b"water\x00" as *const u8 as
                                     *const libc::c_char, 5 as libc::c_int) ==
                           0 ||
                           Q_strnicmp((*tex).name.as_mut_ptr(),
                                      b"laser\x00" as *const u8 as
                                          *const libc::c_char,
                                      5 as libc::c_int) == 0 {
                        (*out).flags =
                            ((*out).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 4 as libc::c_int) as
                                libc::c_int
                    }
                }
                if Q_strncmp((*tex).name.as_mut_ptr(),
                             b"scroll\x00" as *const u8 as
                                 *const libc::c_char, 6 as libc::c_int) == 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 6 as libc::c_int) as
                            libc::c_int
                }
                if (*(*out).texinfo).flags as libc::c_uint &
                       (1 as libc::c_uint) << 6 as libc::c_int != 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 6 as libc::c_int) as
                            libc::c_int
                }
                // g-cont. added a combined conveyor-transparent
                if Q_strncmp((*tex).name.as_mut_ptr(),
                             b"{scroll\x00" as *const u8 as
                                 *const libc::c_char, 7 as libc::c_int) == 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             ((1 as libc::c_uint) << 6 as libc::c_int |
                                  (1 as libc::c_uint) << 8 as libc::c_int)) as
                            libc::c_int
                }
                if (*tex).name[0 as libc::c_int as usize] as libc::c_int ==
                       '{' as i32 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 8 as libc::c_int) as
                            libc::c_int
                }
                if (*(*out).texinfo).flags as libc::c_uint &
                       (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                    (*out).flags =
                        ((*out).flags as libc::c_uint |
                             (1 as libc::c_uint) << 5 as libc::c_int) as
                            libc::c_int
                }
                Mod_CalcSurfaceBounds(out);
                Mod_CalcSurfaceExtents(out);
                Mod_CreateFaceBevels(out);
                // grab the second sample to detect colored lighting
                if test_lightsize > 0 as libc::c_int &&
                       lightofs != -(1 as libc::c_int) {
                    if lightofs > prev_lightofs && lightofs < next_lightofs {
                        next_lightofs = lightofs
                    }
                }
                // grab the first sample to determine lightmap size
                if lightofs != -(1 as libc::c_int) &&
                       test_lightsize == -(1 as libc::c_int) {
                    let mut sample_size: libc::c_int =
                        Mod_SampleSizeForFace(out);
                    let mut smax: libc::c_int =
                        (*info).lightextents[0 as libc::c_int as usize] as
                            libc::c_int / sample_size + 1 as libc::c_int;
                    let mut tmax: libc::c_int =
                        (*info).lightextents[1 as libc::c_int as usize] as
                            libc::c_int / sample_size + 1 as libc::c_int;
                    let mut lightstyles: libc::c_int = 0 as libc::c_int;
                    test_lightsize = smax * tmax;
                    // count styles to right compute test_lightsize
                    j = 0 as libc::c_int;
                    while j < 4 as libc::c_int &&
                              (*out).styles[j as usize] as libc::c_int !=
                                  255 as libc::c_int {
                        lightstyles += 1;
                        j += 1
                    }
                    test_lightsize *= lightstyles;
                    prev_lightofs = lightofs;
                    next_lightofs = 99999999 as libc::c_int
                }
                // TODO: Do we need subdivide on server?
                if (*out).flags as libc::c_uint &
                       (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
                       !(host.type_0 ==
                             HOST_DEDICATED as libc::c_int as libc::c_uint) {
                    ref_0.dllFuncs.GL_SubdivideSurface.expect("non-null function pointer")(out);
                }
            }
            _ => { }
        }
        i += 1;
        out = out.offset(1);
        info = info.offset(1)
    }
    // now we have enough data to trying determine samplecount per lightmap pixel
    if test_lightsize > 0 as libc::c_int &&
           prev_lightofs != -(1 as libc::c_int) &&
           next_lightofs != -(1 as libc::c_int) &&
           next_lightofs != 99999999 as libc::c_int {
        let mut samples: libc::c_float =
            (next_lightofs - prev_lightofs) as libc::c_float /
                test_lightsize as
                    libc::c_float; // align datasize and try again
        if samples != samples as libc::c_int as libc::c_float {
            test_lightsize =
                test_lightsize + 3 as libc::c_int & !(3 as libc::c_int);
            samples =
                (next_lightofs - prev_lightofs) as libc::c_float /
                    test_lightsize as libc::c_float
        }
        if samples == 1 as libc::c_int as libc::c_float ||
               samples == 3 as libc::c_int as libc::c_float {
            (*bmod).lightmap_samples = samples as libc::c_int;
            Con_Reportf(b"lighting: %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        if (*bmod).lightmap_samples == 1 as libc::c_int {
                            b"monochrome\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"colored\x00" as *const u8 as *const libc::c_char
                        });
            (*bmod).lightmap_samples =
                if (*bmod).lightmap_samples > 1 as libc::c_int {
                    (*bmod).lightmap_samples
                } else { 1 as libc::c_int }
            // avoid division by zero
        } else {
            Con_DPrintf(b"^3Warning:^7 lighting invalid samplecount: %g, defaulting to %i\n\x00"
                            as *const u8 as *const libc::c_char,
                        samples as libc::c_double, (*bmod).lightmap_samples);
        }
    };
}
/*
=================
Mod_LoadNodes
=================
*/
unsafe extern "C" fn Mod_LoadNodes(mut bmod: *mut dbspmodel_t) {
    let mut out: *mut mnode_t = 0 as *mut mnode_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numnodes.wrapping_mul(::std::mem::size_of::<mnode_t>()
                                                     as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2410 as libc::c_int) as
            *mut mnode_t;
    (*loadmodel).nodes = out;
    (*loadmodel).numnodes = (*bmod).numnodes as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*loadmodel).numnodes {
        if (*bmod).version ==
               ('B' as i32) << 0 as libc::c_int |
                   ('S' as i32) << 8 as libc::c_int |
                   ('P' as i32) << 16 as libc::c_int |
                   ('2' as i32) << 24 as libc::c_int {
            let mut in_0: *mut dnode32_t =
                &mut *(*bmod).c2rust_unnamed.nodes32.offset(i as isize) as
                    *mut dnode32_t;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*out).minmaxs[(j + 0 as libc::c_int) as usize] =
                    (*in_0).mins[j as usize];
                (*out).minmaxs[(j + 3 as libc::c_int) as usize] =
                    (*in_0).maxs[j as usize];
                j += 1
            }
            p = (*in_0).planenum;
            (*out).plane = (*loadmodel).planes.offset(p as isize);
            (*out).firstsurface = (*in_0).firstface as libc::c_ushort;
            (*out).numsurfaces = (*in_0).numfaces as libc::c_ushort;
            j = 0 as libc::c_int;
            while j < 2 as libc::c_int {
                p = (*in_0).children[j as usize];
                if p >= 0 as libc::c_int {
                    (*out).children[j as usize] =
                        (*loadmodel).nodes.offset(p as isize)
                } else {
                    (*out).children[j as usize] =
                        (*loadmodel).leafs.offset((-(1 as libc::c_int) - p) as
                                                      isize) as *mut mnode_t
                }
                j += 1
            }
        } else {
            let mut in_1: *mut dnode_t =
                &mut *(*bmod).c2rust_unnamed.nodes.offset(i as isize) as
                    *mut dnode_t;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*out).minmaxs[(j + 0 as libc::c_int) as usize] =
                    (*in_1).mins[j as usize] as libc::c_float;
                (*out).minmaxs[(j + 3 as libc::c_int) as usize] =
                    (*in_1).maxs[j as usize] as libc::c_float;
                j += 1
            }
            p = (*in_1).planenum;
            (*out).plane = (*loadmodel).planes.offset(p as isize);
            (*out).firstsurface = (*in_1).firstface;
            (*out).numsurfaces = (*in_1).numfaces;
            j = 0 as libc::c_int;
            while j < 2 as libc::c_int {
                p = (*in_1).children[j as usize] as libc::c_int;
                if p >= 0 as libc::c_int {
                    (*out).children[j as usize] =
                        (*loadmodel).nodes.offset(p as isize)
                } else {
                    (*out).children[j as usize] =
                        (*loadmodel).leafs.offset((-(1 as libc::c_int) - p) as
                                                      isize) as *mut mnode_t
                }
                j += 1
            }
        }
        i += 1;
        out = out.offset(1)
    }
    // sets nodes and leafs
    Mod_SetParent((*loadmodel).nodes, 0 as *mut mnode_t);
}
/*
=================
Mod_LoadLeafs
=================
*/
unsafe extern "C" fn Mod_LoadLeafs(mut bmod: *mut dbspmodel_t) {
    let mut out: *mut mleaf_t =
        0 as *mut mleaf_t; // no visclusters on bmodels
    let mut i: libc::c_int = 0; // solid leaf 0 has no visdata
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut visclusters: libc::c_int = 0 as libc::c_int;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numleafs.wrapping_mul(::std::mem::size_of::<mleaf_t>()
                                                     as libc::c_ulong),
                   true_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2476 as libc::c_int) as
            *mut mleaf_t;
    (*loadmodel).leafs = out;
    (*loadmodel).numleafs = (*bmod).numleafs as libc::c_int;
    if (*bmod).isworld as u64 != 0 {
        visclusters =
            (*(*loadmodel).submodels.offset(0 as libc::c_int as
                                                isize)).visleafs;
        world.visbytes =
            (visclusters + 7 as libc::c_int >> 3 as libc::c_int) as size_t;
        world.fatbytes =
            (visclusters + 31 as libc::c_int >> 3 as libc::c_int) as size_t;
        refState.visbytes = world.visbytes
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*bmod).numleafs {
        if (*bmod).version ==
               ('B' as i32) << 0 as libc::c_int |
                   ('S' as i32) << 8 as libc::c_int |
                   ('P' as i32) << 16 as libc::c_int |
                   ('2' as i32) << 24 as libc::c_int {
            let mut in_0: *mut dleaf32_t =
                &mut *(*bmod).c2rust_unnamed_0.leafs32.offset(i as isize) as
                    *mut dleaf32_t;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*out).minmaxs[(j + 0 as libc::c_int) as usize] =
                    (*in_0).mins[j as usize];
                (*out).minmaxs[(j + 3 as libc::c_int) as usize] =
                    (*in_0).maxs[j as usize];
                j += 1
            }
            (*out).contents = (*in_0).contents;
            p = (*in_0).visofs;
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                (*out).ambient_sound_level[j as usize] =
                    (*in_0).ambient_level[j as usize];
                j += 1
            }
            (*out).firstmarksurface =
                (*loadmodel).marksurfaces.offset((*in_0).firstmarksurface as
                                                     isize);
            (*out).nummarksurfaces = (*in_0).nummarksurfaces
        } else {
            let mut in_1: *mut dleaf_t =
                &mut *(*bmod).c2rust_unnamed_0.leafs.offset(i as isize) as
                    *mut dleaf_t;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*out).minmaxs[(j + 0 as libc::c_int) as usize] =
                    (*in_1).mins[j as usize] as libc::c_float;
                (*out).minmaxs[(j + 3 as libc::c_int) as usize] =
                    (*in_1).maxs[j as usize] as libc::c_float;
                j += 1
            }
            (*out).contents = (*in_1).contents;
            p = (*in_1).visofs;
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                (*out).ambient_sound_level[j as usize] =
                    (*in_1).ambient_level[j as usize];
                j += 1
            }
            (*out).firstmarksurface =
                (*loadmodel).marksurfaces.offset((*in_1).firstmarksurface as
                                                     libc::c_int as isize);
            (*out).nummarksurfaces = (*in_1).nummarksurfaces as libc::c_int
        }
        if (*bmod).isworld as u64 != 0 {
            (*out).cluster = i - 1 as libc::c_int;
            if (*out).cluster >= visclusters {
                (*out).cluster = -(1 as libc::c_int)
            }
            // ignore visofs errors on leaf 0 (solid)
            if p >= 0 as libc::c_int && (*out).cluster >= 0 as libc::c_int &&
                   !(*loadmodel).visdata.is_null() {
                if (p as libc::c_ulong) < (*bmod).visdatasize {
                    (*out).compressed_vis =
                        (*loadmodel).visdata.offset(p as isize)
                } else {
                    Con_Reportf(b"^3Warning:^7 Mod_LoadLeafs: invalid visofs for leaf #%i\n\x00"
                                    as *const u8 as *const libc::c_char, i);
                }
            }
        } else { (*out).cluster = -(1 as libc::c_int) }
        if p == -(1 as libc::c_int) {
            (*out).compressed_vis = 0 as *mut byte
        } else {
            (*out).compressed_vis = (*loadmodel).visdata.offset(p as isize)
        }
        // gl underwater warp
        if (*out).contents != -(1 as libc::c_int) {
            j = 0 as libc::c_int;
            while j < (*out).nummarksurfaces {
                // mark underwater surfaces
                (**(*out).firstmarksurface.offset(j as isize)).flags =
                    ((**(*out).firstmarksurface.offset(j as isize)).flags as
                         libc::c_uint |
                         (1 as libc::c_uint) << 7 as libc::c_int) as
                        libc::c_int;
                j += 1
            }
        }
        i += 1;
        out = out.offset(1)
    }
    if (*bmod).isworld as libc::c_uint != 0 &&
           (*(*loadmodel).leafs.offset(0 as libc::c_int as isize)).contents !=
               -(2 as libc::c_int) {
        Host_Error(b"Mod_LoadLeafs: Map %s has leaf 0 is not CONTENTS_SOLID\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*loadmodel).name.as_mut_ptr());
    }
    // do some final things for world
    if (*bmod).isworld as libc::c_uint != 0 &&
           Mod_CheckWaterAlphaSupport(bmod) as libc::c_uint != 0 {
        world.flags =
            (world.flags as libc::c_uint |
                 (1 as libc::c_uint) << 2 as libc::c_int) as libc::c_int
    };
}
/*
=================
Mod_LoadClipnodes
=================
*/
unsafe extern "C" fn Mod_LoadClipnodes(mut bmod: *mut dbspmodel_t) {
    let mut out: *mut dclipnode32_t = 0 as *mut dclipnode32_t;
    let mut i: libc::c_int = 0;
    out =
        _Mem_Alloc((*loadmodel).mempool,
                   (*bmod).numclipnodes.wrapping_mul(::std::mem::size_of::<dclipnode32_t>()
                                                         as libc::c_ulong),
                   false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2577 as libc::c_int) as
            *mut dclipnode32_t;
    (*bmod).clipnodes_out = out;
    if (*bmod).version ==
           ('B' as i32) << 0 as libc::c_int | ('S' as i32) << 8 as libc::c_int
               | ('P' as i32) << 16 as libc::c_int |
               ('2' as i32) << 24 as libc::c_int ||
           (*bmod).version == 30 as libc::c_int &&
               (*bmod).numclipnodes >= 32767 as libc::c_int as libc::c_ulong {
        let mut in_0: *mut dclipnode32_t =
            (*bmod).c2rust_unnamed_1.clipnodes32;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*bmod).numclipnodes {
            (*out).planenum = (*in_0).planenum;
            (*out).children[0 as libc::c_int as usize] =
                (*in_0).children[0 as libc::c_int as usize];
            (*out).children[1 as libc::c_int as usize] =
                (*in_0).children[1 as libc::c_int as usize];
            i += 1;
            out = out.offset(1);
            in_0 = in_0.offset(1)
        }
    } else {
        let mut in_1: *mut dclipnode_t = (*bmod).c2rust_unnamed_1.clipnodes;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*bmod).numclipnodes {
            (*out).planenum = (*in_1).planenum;
            (*out).children[0 as libc::c_int as usize] =
                (*in_1).children[0 as libc::c_int as usize] as libc::c_ushort
                    as libc::c_int;
            (*out).children[1 as libc::c_int as usize] =
                (*in_1).children[1 as libc::c_int as usize] as libc::c_ushort
                    as libc::c_int;
            // Arguire QBSP 'broken' clipnodes
            if (*out).children[0 as libc::c_int as usize] as libc::c_ulong >=
                   (*bmod).numclipnodes {
                (*out).children[0 as libc::c_int as usize] -=
                    65536 as libc::c_int
            }
            if (*out).children[1 as libc::c_int as usize] as libc::c_ulong >=
                   (*bmod).numclipnodes {
                (*out).children[1 as libc::c_int as usize] -=
                    65536 as libc::c_int
            }
            i += 1;
            out = out.offset(1);
            in_1 = in_1.offset(1)
        }
    }
    // FIXME: fill loadmodel->clipnodes?
    (*loadmodel).numclipnodes = (*bmod).numclipnodes as libc::c_int;
}
/*
=================
Mod_LoadVisibility
=================
*/
unsafe extern "C" fn Mod_LoadVisibility(mut bmod: *mut dbspmodel_t) {
    (*loadmodel).visdata =
        _Mem_Alloc((*loadmodel).mempool, (*bmod).visdatasize, false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2620 as libc::c_int) as *mut byte;
    memcpy((*loadmodel).visdata as *mut libc::c_void,
           (*bmod).visdata as *const libc::c_void, (*bmod).visdatasize);
}
/*
=================
Mod_LoadLightVecs
=================
*/
unsafe extern "C" fn Mod_LoadLightVecs(mut bmod: *mut dbspmodel_t) {
    if (*bmod).deluxdatasize != (*bmod).lightdatasize {
        if (*bmod).deluxdatasize > 0 as libc::c_int as libc::c_ulong {
            Con_Printf(b"^1Error:^7 Mod_LoadLightVecs: has mismatched size (%lu should be %i)\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*bmod).deluxdatasize,
                       (*bmod).lightdatasize); // old method
        } else { Mod_LoadDeluxemap(bmod); }
        return
    }
    (*bmod).deluxedata_out =
        _Mem_Alloc((*loadmodel).mempool, (*bmod).deluxdatasize, false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2639 as libc::c_int) as
            *mut color24;
    memcpy((*bmod).deluxedata_out as *mut libc::c_void,
           (*bmod).deluxdata as *const libc::c_void, (*bmod).deluxdatasize);
}
/*
=================
Mod_LoadShadowmap
=================
*/
unsafe extern "C" fn Mod_LoadShadowmap(mut bmod: *mut dbspmodel_t) {
    if (*bmod).shadowdatasize !=
           (*bmod).lightdatasize.wrapping_div(3 as libc::c_int as
                                                  libc::c_ulong) {
        if (*bmod).shadowdatasize > 0 as libc::c_int as libc::c_ulong {
            Con_Printf(b"^1Error:^7 Mod_LoadShadowmap: has mismatched size (%i should be %lu)\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*bmod).shadowdatasize,
                       (*bmod).lightdatasize.wrapping_div(3 as libc::c_int as
                                                              libc::c_ulong));
        }
        return
    }
    (*bmod).shadowdata_out =
        _Mem_Alloc((*loadmodel).mempool, (*bmod).shadowdatasize, false_0,
                   b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                       *const libc::c_char, 2657 as libc::c_int) as *mut byte;
    memcpy((*bmod).shadowdata_out as *mut libc::c_void,
           (*bmod).shadowdata as *const libc::c_void, (*bmod).shadowdatasize);
}
/*
=================
Mod_LoadLighting
=================
*/
unsafe extern "C" fn Mod_LoadLighting(mut bmod: *mut dbspmodel_t) {
    let mut i: libc::c_int = 0;
    let mut lightofs: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut out: *mut color24 = 0 as *mut color24;
    let mut in_0: *mut byte = 0 as *mut byte;
    if (*bmod).lightdatasize == 0 { return }
    match (*bmod).lightmap_samples {
        1 => {
            if Mod_LoadColoredLighting(bmod) as u64 == 0 {
                out =
                    _Mem_Alloc((*loadmodel).mempool,
                               (*bmod).lightdatasize.wrapping_mul(::std::mem::size_of::<color24>()
                                                                      as
                                                                      libc::c_ulong),
                               false_0,
                               b"../engine/common/mod_bmodel.c\x00" as
                                   *const u8 as *const libc::c_char,
                               2681 as libc::c_int) as *mut color24;
                (*loadmodel).lightdata = out;
                in_0 = (*bmod).lightdata;
                // expand the white lighting data
                i = 0 as libc::c_int;
                while (i as libc::c_ulong) < (*bmod).lightdatasize {
                    let fresh13 = in_0;
                    in_0 = in_0.offset(1);
                    (*out).b = *fresh13;
                    (*out).g = (*out).b;
                    (*out).r = (*out).g;
                    i += 1;
                    out = out.offset(1)
                }
            }
        }
        3 => {
            // load colored lighting
            (*loadmodel).lightdata =
                _Mem_Alloc((*loadmodel).mempool, (*bmod).lightdatasize,
                           false_0,
                           b"../engine/common/mod_bmodel.c\x00" as *const u8
                               as *const libc::c_char, 2690 as libc::c_int) as
                    *mut color24;
            memcpy((*loadmodel).lightdata as *mut libc::c_void,
                   (*bmod).lightdata as *const libc::c_void,
                   (*bmod).lightdatasize);
            (*loadmodel).flags =
                ((*loadmodel).flags as libc::c_uint |
                     (1 as libc::c_uint) << 4 as libc::c_int) as libc::c_int
        }
        _ => {
            Host_Error(b"Mod_LoadLighting: bad lightmap sample count %i\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*bmod).lightmap_samples);
        }
    }
    // not supposed to be load ?
    if host.features &
           ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        Mod_LoadLightVecs(bmod);
        Mod_LoadShadowmap(bmod);
        if (*bmod).isworld as libc::c_uint != 0 && (*bmod).deluxdatasize != 0
           {
            world.flags =
                (world.flags as libc::c_uint |
                     (1 as libc::c_uint) << 3 as libc::c_int) as libc::c_int
        }
    }
    surf = (*loadmodel).surfaces;
    // setup lightdata pointers
    i = 0 as libc::c_int;
    while i < (*loadmodel).numsurfaces {
        if (*bmod).version ==
               ('B' as i32) << 0 as libc::c_int |
                   ('S' as i32) << 8 as libc::c_int |
                   ('P' as i32) << 16 as libc::c_int |
                   ('2' as i32) << 24 as libc::c_int {
            lightofs =
                (*(*bmod).c2rust_unnamed_4.surfaces32.offset(i as
                                                                 isize)).lightofs
        } else {
            lightofs =
                (*(*bmod).c2rust_unnamed_4.surfaces.offset(i as
                                                               isize)).lightofs
        }
        if !(*loadmodel).lightdata.is_null() &&
               lightofs != -(1 as libc::c_int) {
            let mut offset: libc::c_int = lightofs / (*bmod).lightmap_samples;
            // NOTE: we divide offset by three because lighting and deluxemap keep their pointers
			// into three-bytes structs and shadowmap just monochrome
            (*surf).samples = (*loadmodel).lightdata.offset(offset as isize);
            // if deluxemap is present setup it too
            if !(*bmod).deluxedata_out.is_null() {
                (*(*surf).info).deluxemap =
                    (*bmod).deluxedata_out.offset(offset as isize)
            }
            // will be used by mods
            if !(*bmod).shadowdata_out.is_null() {
                (*(*surf).info).shadowmap =
                    (*bmod).shadowdata_out.offset(offset as isize)
            }
        }
        i += 1;
        surf = surf.offset(1)
    };
}
/*
=================
Mod_LoadBmodelLumps

loading and processing bmodel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadBmodelLumps(mut mod_base: *const byte,
                                             mut isworld: qboolean)
 -> qboolean {
    let mut header: *mut dheader_t = mod_base as *mut dheader_t;
    let mut extrahdr: *mut dextrahdr_t =
        (mod_base as
             *mut byte).offset(::std::mem::size_of::<dheader_t>() as
                                   libc::c_ulong as isize) as
            *mut dextrahdr_t;
    let mut bmod: *mut dbspmodel_t = &mut srcmodel;
    let mut mod_0: *mut model_t = loadmodel;
    let mut wadvalue: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    // always reset the intermediate struct
    memset(bmod as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dbspmodel_t>() as
               libc::c_ulong); // share up global
    memset(&mut loadstat as *mut loadstat_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<loadstat_t>() as
               libc::c_ulong); // clear world settings
    Q_strncpy(loadstat.name.as_mut_ptr(), (*loadmodel).name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    wadvalue[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if (*header).version ==
           ('B' as i32) << 0 as libc::c_int | ('S' as i32) << 8 as libc::c_int
               | ('P' as i32) << 16 as libc::c_int |
               ('2' as i32) << 24 as libc::c_int {
        Con_Printf(b"^1Error:^7 %s can\'t be loaded in this build. Please rebuild engine with enabled SUPPORT_BSP2_FORMAT\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*loadmodel).name.as_mut_ptr());
        return false_0
    }
    match (*header).version {
        29 | 30 | 844124994 => { }
        _ => {
            Con_Printf(b"^1Error:^7 %s has wrong version number (%i should be %i)\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*loadmodel).name.as_mut_ptr(), (*header).version,
                       30 as libc::c_int);
            loadstat.numerrors += 1;
            return false_0
        }
    }
    (*bmod).version = (*header).version;
    if isworld as u64 != 0 { world.flags = 0 as libc::c_int }
    (*bmod).isworld = isworld;
    if (*header).version == 30 as libc::c_int &&
           (*header).lumps[0 as libc::c_int as usize].fileofs <=
               1024 as libc::c_int &&
           ((*header).lumps[0 as libc::c_int as usize].filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dplane_t>()
                                                as libc::c_ulong) ==
               0 as libc::c_int as libc::c_ulong {
        // blue-shift swapped lumps
        srclumps[0 as libc::c_int as usize].lumpnumber = 1 as libc::c_int;
        srclumps[1 as libc::c_int as usize].lumpnumber = 0 as libc::c_int
    }
    // loading base lumps
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[mlumpinfo_t; 15]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpinfo_t>()
                                                   as libc::c_ulong) {
        Mod_LoadLump(mod_base, &mut *srclumps.as_mut_ptr().offset(i as isize),
                     &mut *worldstats.as_mut_ptr().offset(i as isize),
                     if isworld as libc::c_uint != 0 {
                         ((1 as libc::c_uint) << 0 as libc::c_int) |
                             (1 as libc::c_uint) << 2 as libc::c_int
                     } else { 0 as libc::c_int as libc::c_uint } as
                         libc::c_int);
        i += 1
    }
    // loading extralumps
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[mlumpinfo_t; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpinfo_t>()
                                                   as libc::c_ulong) {
        Mod_LoadLump(mod_base, &mut *extlumps.as_mut_ptr().offset(i as isize),
                     &mut *worldstats.as_mut_ptr().offset((::std::mem::size_of::<[mlumpinfo_t; 15]>()
                                                               as
                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpinfo_t>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_add(i
                                                                                                                               as
                                                                                                                               libc::c_ulong)
                                                              as isize),
                     if isworld as libc::c_uint != 0 {
                         ((1 as libc::c_uint) << 0 as libc::c_int) |
                             (1 as libc::c_uint) << 2 as libc::c_int
                     } else { 0 as libc::c_int as libc::c_uint } as
                         libc::c_int);
        i += 1
    }
    if (*bmod).isworld as u64 == 0 && loadstat.numerrors != 0 {
        Con_DPrintf(b"Mod_Load%s: %i error(s), %i warning(s)\n\x00" as
                        *const u8 as *const libc::c_char,
                    if isworld as libc::c_uint != 0 {
                        b"World\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Brush\x00" as *const u8 as *const libc::c_char
                    }, loadstat.numerrors, loadstat.numwarnings);
        return false_0
        // there were errors, we can't load this map
    } else {
        if (*bmod).isworld as u64 == 0 && loadstat.numwarnings != 0 {
            Con_DPrintf(b"Mod_Load%s: %i warning(s)\n\x00" as *const u8 as
                            *const libc::c_char,
                        if isworld as libc::c_uint != 0 {
                            b"World\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"Brush\x00" as *const u8 as *const libc::c_char
                        }, loadstat.numwarnings);
        }
    }
    // load into heap
    Mod_LoadEntities(bmod);
    Mod_LoadPlanes(bmod);
    Mod_LoadSubmodels(bmod);
    Mod_LoadVertexes(bmod);
    Mod_LoadEdges(bmod);
    Mod_LoadSurfEdges(bmod);
    Mod_LoadTextures(bmod);
    Mod_LoadVisibility(bmod);
    Mod_LoadTexInfo(bmod);
    Mod_LoadSurfaces(bmod);
    Mod_LoadLighting(bmod);
    Mod_LoadMarkSurfaces(bmod);
    Mod_LoadLeafs(bmod);
    Mod_LoadNodes(bmod);
    Mod_LoadClipnodes(bmod);
    // preform some post-initalization
    Mod_MakeHull0(); // restore pointer to world
    Mod_SetupSubmodels(bmod);
    if isworld as u64 != 0 {
        loadmodel = mod_0;
        // occlusion data pointer
        // XASH_DEDICATED
        Mod_InitDebugHulls(); // FIXME: build hulls for separate bmodels (shells, medkits etc)
        world.deluxedata = (*bmod).deluxedata_out; // deluxemap data pointer
        world.shadowdata = (*bmod).shadowdata_out
    } // kill the last semicolon
    i = 0 as libc::c_int;
    while i < (*bmod).wadlist.count {
        if !((*bmod).wadlist.wadusage[i as usize] == 0) {
            Q_strncat(wadvalue.as_mut_ptr(),
                      va(b"%s.wad; \x00" as *const u8 as *const libc::c_char,
                         (*bmod).wadlist.wadnames[i as usize].as_mut_ptr()),
                      ::std::mem::size_of::<[libc::c_char; 2048]>() as
                          libc::c_ulong);
        }
        i += 1
    }
    if if wadvalue.as_mut_ptr().is_null() || *wadvalue.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        wadvalue[Q_strlen(wadvalue.as_mut_ptr()).wrapping_sub(2 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                     as usize] = '\u{0}' as i32 as libc::c_char;
        Con_Reportf(b"Wad files required to run the map: \"%s\"\n\x00" as
                        *const u8 as *const libc::c_char,
                    wadvalue.as_mut_ptr());
    }
    return true_0;
}
/*
=================
Mod_TestBmodelLumps

check for possible errors
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_TestBmodelLumps(mut name: *const libc::c_char,
                                             mut mod_base: *const byte,
                                             mut silent: qboolean)
 -> qboolean {
    let mut header: *mut dheader_t = mod_base as *mut dheader_t;
    let mut i: libc::c_int = 0;
    let mut flags: libc::c_int =
        ((1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
    // always reset the intermediate struct
    memset(&mut loadstat as *mut loadstat_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<loadstat_t>() as libc::c_ulong);
    // store the name to correct show errors and warnings
    Q_strncpy(loadstat.name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    if silent as u64 != 0 {
        flags =
            (flags as libc::c_uint | (1 as libc::c_uint) << 2 as libc::c_int)
                as libc::c_int
    }
    if (*header).version ==
           ('B' as i32) << 0 as libc::c_int | ('S' as i32) << 8 as libc::c_int
               | ('P' as i32) << 16 as libc::c_int |
               ('2' as i32) << 24 as libc::c_int {
        if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int ==
               0 {
            Con_Printf(b"^1Error:^7 %s can\'t be loaded in this build. Please rebuild engine with enabled SUPPORT_BSP2_FORMAT\n\x00"
                           as *const u8 as *const libc::c_char, name);
        }
        return false_0
    }
    match (*header).version {
        29 | 30 | 844124994 => { }
        _ => {
            // don't early out: let me analyze errors
            if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
                   == 0 {
                Con_Printf(b"^1Error:^7 %s has wrong version number (%i should be %i)\n\x00"
                               as *const u8 as *const libc::c_char, name,
                           (*header).version, 30 as libc::c_int);
            }
            loadstat.numerrors += 1
        }
    }
    if (*header).version == 30 as libc::c_int &&
           (*header).lumps[0 as libc::c_int as usize].fileofs <=
               1024 as libc::c_int &&
           ((*header).lumps[0 as libc::c_int as usize].filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dplane_t>()
                                                as libc::c_ulong) ==
               0 as libc::c_int as libc::c_ulong {
        // blue-shift swapped lumps
        srclumps[0 as libc::c_int as usize].lumpnumber = 1 as libc::c_int;
        srclumps[1 as libc::c_int as usize].lumpnumber = 0 as libc::c_int
    }
    // loading base lumps
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[mlumpinfo_t; 15]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpinfo_t>()
                                                   as libc::c_ulong) {
        Mod_LoadLump(mod_base, &mut *srclumps.as_mut_ptr().offset(i as isize),
                     &mut *worldstats.as_mut_ptr().offset(i as isize), flags);
        i += 1
    }
    // loading extralumps
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[mlumpinfo_t; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpinfo_t>()
                                                   as libc::c_ulong) {
        Mod_LoadLump(mod_base, &mut *extlumps.as_mut_ptr().offset(i as isize),
                     &mut *worldstats.as_mut_ptr().offset((::std::mem::size_of::<[mlumpinfo_t; 15]>()
                                                               as
                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<mlumpinfo_t>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_add(i
                                                                                                                               as
                                                                                                                               libc::c_ulong)
                                                              as isize),
                     flags);
        i += 1
    }
    if loadstat.numerrors != 0 {
        if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int ==
               0 {
            Con_Printf(b"Mod_LoadWorld: %i error(s), %i warning(s)\n\x00" as
                           *const u8 as *const libc::c_char,
                       loadstat.numerrors, loadstat.numwarnings);
        }
        return false_0
        // there were errors, we can't load this map
    } else {
        if loadstat.numwarnings != 0 {
            if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
                   == 0 {
                Con_Printf(b"Mod_LoadWorld: %i warning(s)\n\x00" as *const u8
                               as *const libc::c_char, loadstat.numwarnings);
            }
        }
    }
    return true_0;
}
/*
=================
Mod_LoadBrushModel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadBrushModel(mut mod_0: *mut model_t,
                                            mut buffer: *const libc::c_void,
                                            mut loaded: *mut qboolean) {
    if !loaded.is_null() { *loaded = false_0 }
    (*loadmodel).mempool =
        _Mem_AllocPool(va(b"^2%s^7\x00" as *const u8 as *const libc::c_char,
                          (*loadmodel).name.as_mut_ptr()),
                       b"../engine/common/mod_bmodel.c\x00" as *const u8 as
                           *const libc::c_char, 2937 as libc::c_int);
    (*loadmodel).type_0 = mod_brush;
    // loading all the lumps into heap
    if Mod_LoadBmodelLumps(buffer as *const byte, world.loading) as u64 == 0 {
        return
    } // there were errors
    if world.loading as u64 != 0 { worldmodel = mod_0 }
    if !loaded.is_null() { *loaded = true_0 };
    // all done
}
/*
==================
Mod_CheckLump

check lump for existing
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_CheckLump(mut filename: *const libc::c_char,
                                       lump: libc::c_int,
                                       mut lumpsize: *mut libc::c_int)
 -> libc::c_int {
    let mut f: *mut file_t =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    let mut buffer: [byte; 228] = [0; 228];
    let mut prefetch_size: size_t =
        ::std::mem::size_of::<[byte; 228]>() as libc::c_ulong;
    let mut extrahdr: *mut dextrahdr_t = 0 as *mut dextrahdr_t;
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    if f.is_null() { return 1 as libc::c_int }
    if FS_Read(f, buffer.as_mut_ptr() as *mut libc::c_void, prefetch_size) as
           libc::c_ulong != prefetch_size {
        FS_Close(f);
        return 2 as libc::c_int
    }
    header = buffer.as_mut_ptr() as *mut dheader_t;
    if (*header).version != 30 as libc::c_int {
        FS_Close(f);
        return 3 as libc::c_int
    }
    extrahdr =
        buffer.as_mut_ptr().offset(::std::mem::size_of::<dheader_t>() as
                                       libc::c_ulong as isize) as
            *mut dextrahdr_t;
    if (*extrahdr).id !=
           (('H' as i32) << 24 as libc::c_int) +
               (('S' as i32) << 16 as libc::c_int) +
               (('A' as i32) << 8 as libc::c_int) + 'X' as i32 ||
           (*extrahdr).version != 4 as libc::c_int {
        FS_Close(f);
        return 4 as libc::c_int
    }
    if lump < 0 as libc::c_int || lump >= 12 as libc::c_int {
        FS_Close(f);
        return 5 as libc::c_int
    }
    if (*extrahdr).lumps[lump as usize].filelen <= 0 as libc::c_int {
        FS_Close(f);
        return 6 as libc::c_int
    }
    if !lumpsize.is_null() {
        *lumpsize = (*extrahdr).lumps[lump as usize].filelen
    }
    FS_Close(f);
    return 0 as libc::c_int;
}
/*
==================
Mod_ReadLump

reading random lump by user request
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_ReadLump(mut filename: *const libc::c_char,
                                      lump: libc::c_int,
                                      mut lumpdata: *mut *mut libc::c_void,
                                      mut lumpsize: *mut libc::c_int)
 -> libc::c_int {
    let mut f: *mut file_t =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0); // write term
    let mut buffer: [byte; 228] = [0; 228];
    let mut prefetch_size: size_t =
        ::std::mem::size_of::<[byte; 228]>() as libc::c_ulong;
    let mut extrahdr: *mut dextrahdr_t = 0 as *mut dextrahdr_t;
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    let mut data: *mut byte = 0 as *mut byte;
    let mut length: libc::c_int = 0;
    if f.is_null() { return 1 as libc::c_int }
    if FS_Read(f, buffer.as_mut_ptr() as *mut libc::c_void, prefetch_size) as
           libc::c_ulong != prefetch_size {
        FS_Close(f);
        return 2 as libc::c_int
    }
    header = buffer.as_mut_ptr() as *mut dheader_t;
    if (*header).version != 30 as libc::c_int {
        FS_Close(f);
        return 3 as libc::c_int
    }
    extrahdr =
        buffer.as_mut_ptr().offset(::std::mem::size_of::<dheader_t>() as
                                       libc::c_ulong as isize) as
            *mut dextrahdr_t;
    if (*extrahdr).id !=
           (('H' as i32) << 24 as libc::c_int) +
               (('S' as i32) << 16 as libc::c_int) +
               (('A' as i32) << 8 as libc::c_int) + 'X' as i32 ||
           (*extrahdr).version != 4 as libc::c_int {
        FS_Close(f);
        return 4 as libc::c_int
    }
    if lump < 0 as libc::c_int || lump >= 12 as libc::c_int {
        FS_Close(f);
        return 5 as libc::c_int
    }
    if (*extrahdr).lumps[lump as usize].filelen <= 0 as libc::c_int {
        FS_Close(f);
        return 6 as libc::c_int
    }
    data =
        malloc(((*extrahdr).lumps[lump as usize].filelen + 1 as libc::c_int)
                   as libc::c_ulong) as *mut byte;
    length = (*extrahdr).lumps[lump as usize].filelen;
    if data.is_null() { FS_Close(f); return 7 as libc::c_int }
    FS_Seek(f, (*extrahdr).lumps[lump as usize].fileofs as fs_offset_t,
            0 as libc::c_int);
    if FS_Read(f, data as *mut libc::c_void, length as size_t) !=
           length as libc::c_long {
        free(data as *mut libc::c_void);
        FS_Close(f);
        return 8 as libc::c_int
    }
    *data.offset(length as isize) = 0 as libc::c_int as byte;
    FS_Close(f);
    if !lumpsize.is_null() { *lumpsize = length }
    *lumpdata = data as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
==================
Mod_SaveLump

writing lump by user request
only empty lumps is allows
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_SaveLump(mut filename: *const libc::c_char,
                                      lump: libc::c_int,
                                      mut lumpdata: *mut libc::c_void,
                                      mut lumpsize: libc::c_int)
 -> libc::c_int {
    let mut buffer: [byte; 228] = [0; 228];
    let mut prefetch_size: size_t =
        ::std::mem::size_of::<[byte; 228]>() as libc::c_ulong;
    let mut result: libc::c_int = 0;
    let mut dummy: libc::c_int = lumpsize;
    let mut extrahdr: *mut dextrahdr_t = 0 as *mut dextrahdr_t;
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    let mut f: *mut file_t = 0 as *mut file_t;
    if lumpdata.is_null() || lumpsize <= 0 as libc::c_int {
        return 7 as libc::c_int
    }
    // make sure what .bsp is placed into gamedir and not in pak
    if FS_GetDiskPath(filename, true_0).is_null() { return 1 as libc::c_int }
    // first we should sure what we allow to rewrite this .bsp
    result = Mod_CheckLump(filename, lump, &mut dummy);
    if result != 6 as libc::c_int { return result }
    f =
        FS_Open(filename, b"e+b\x00" as *const u8 as *const libc::c_char,
                true_0);
    if f.is_null() { return 1 as libc::c_int }
    if FS_Read(f, buffer.as_mut_ptr() as *mut libc::c_void, prefetch_size) as
           libc::c_ulong != prefetch_size {
        FS_Close(f);
        return 2 as libc::c_int
    }
    header = buffer.as_mut_ptr() as *mut dheader_t;
    // these checks below are redundant
    if (*header).version != 30 as libc::c_int {
        FS_Close(f);
        return 3 as libc::c_int
    }
    extrahdr =
        buffer.as_mut_ptr().offset(::std::mem::size_of::<dheader_t>() as
                                       libc::c_ulong as isize) as
            *mut dextrahdr_t;
    if (*extrahdr).id !=
           (('H' as i32) << 24 as libc::c_int) +
               (('S' as i32) << 16 as libc::c_int) +
               (('A' as i32) << 8 as libc::c_int) + 'X' as i32 ||
           (*extrahdr).version != 4 as libc::c_int {
        FS_Close(f);
        return 4 as libc::c_int
    }
    if lump < 0 as libc::c_int || lump >= 12 as libc::c_int {
        FS_Close(f);
        return 5 as libc::c_int
    }
    if (*extrahdr).lumps[lump as usize].filelen != 0 as libc::c_int {
        FS_Close(f);
        return 6 as libc::c_int
    }
    FS_Seek(f, 0 as libc::c_int as fs_offset_t, 2 as libc::c_int);
    // will be saved later
    (*extrahdr).lumps[lump as usize].fileofs = FS_Tell(f) as libc::c_int;
    (*extrahdr).lumps[lump as usize].filelen = lumpsize;
    if FS_Write(f, lumpdata, lumpsize as size_t) != lumpsize as libc::c_long {
        FS_Close(f);
        return 8 as libc::c_int
    }
    // update the header
    FS_Seek(f,
            ::std::mem::size_of::<dheader_t>() as libc::c_ulong as
                fs_offset_t, 0 as libc::c_int);
    if FS_Write(f, extrahdr as *const libc::c_void,
                ::std::mem::size_of::<dextrahdr_t>() as libc::c_ulong) as
           libc::c_ulong !=
           ::std::mem::size_of::<dextrahdr_t>() as libc::c_ulong {
        FS_Close(f);
        return 8 as libc::c_int
    }
    FS_Close(f);
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    srclumps =
        [{
             let mut init =
                 mlumpinfo_t{lumpnumber: 0 as libc::c_int,
                             mincount: 32 as libc::c_int as size_t,
                             maxcount: 0x100000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<byte>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"entities\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.entdata as *mut *mut byte as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.entdatasize,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 1 as libc::c_int,
                             mincount: 1 as libc::c_int as size_t,
                             maxcount: 65536 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dplane_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"planes\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.planes as *mut *mut dplane_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numplanes,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 2 as libc::c_int,
                             mincount: 1 as libc::c_int as size_t,
                             maxcount: 0x2000000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<byte>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"textures\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.textures as
                                     *mut *mut dmiptexlump_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.texdatasize,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 3 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 65535 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dvertex_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"vertexes\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.vertexes as *mut *mut dvertex_t
                                     as *mut *const libc::c_void,
                             count: &mut srcmodel.numvertexes,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 4 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 0x1000000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<byte>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"visibility\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.visdata as *mut *mut byte as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.visdatasize,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 5 as libc::c_int,
                             mincount: 1 as libc::c_int as size_t,
                             maxcount: 32767 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dnode_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32:
                                 ::std::mem::size_of::<dnode32_t>() as
                                     libc::c_ulong as libc::c_int,
                             loadname:
                                 b"nodes\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 0 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.c2rust_unnamed.nodes as
                                     *mut *mut dnode_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numnodes,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 6 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 65535 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dtexinfo_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"texinfo\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 0 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.texinfo as *mut *mut dtexinfo_t
                                     as *mut *const libc::c_void,
                             count: &mut srcmodel.numtexinfo,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 7 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 65535 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dface_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32:
                                 ::std::mem::size_of::<dface32_t>() as
                                     libc::c_ulong as libc::c_int,
                             loadname:
                                 b"faces\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 0 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.c2rust_unnamed_4.surfaces as
                                     *mut *mut dface_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numsurfaces,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 8 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 0x2000000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<byte>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"lightmaps\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.lightdata as *mut *mut byte as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.lightdatasize,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 9 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 32767 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dclipnode_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32:
                                 ::std::mem::size_of::<dclipnode32_t>() as
                                     libc::c_ulong as libc::c_int,
                             loadname:
                                 b"clipnodes\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.c2rust_unnamed_1.clipnodes as
                                     *mut *mut dclipnode_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numclipnodes,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 10 as libc::c_int,
                             mincount: 1 as libc::c_int as size_t,
                             maxcount: 32767 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dleaf_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32:
                                 ::std::mem::size_of::<dleaf32_t>() as
                                     libc::c_ulong as libc::c_int,
                             loadname:
                                 b"leafs\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 0 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.c2rust_unnamed_0.leafs as
                                     *mut *mut dleaf_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numleafs,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 11 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 65535 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dmarkface_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32:
                                 ::std::mem::size_of::<dmarkface32_t>() as
                                     libc::c_ulong as libc::c_int,
                             loadname:
                                 b"markfaces\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.c2rust_unnamed_2.markfaces as
                                     *mut *mut dmarkface_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.nummarkfaces,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 12 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 0x100000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dedge_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32:
                                 ::std::mem::size_of::<dedge32_t>() as
                                     libc::c_ulong as libc::c_int,
                             loadname:
                                 b"edges\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.c2rust_unnamed_3.edges as
                                     *mut *mut dedge_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numedges,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 13 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 0x200000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dsurfedge_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"surfedges\x00" as *const u8 as
                                     *const libc::c_char,
                             flags: 0 as libc::c_int,
                             dataptr:
                                 &mut srcmodel.surfedges as
                                     *mut *mut dsurfedge_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numsurfedges,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 14 as libc::c_int,
                             mincount: 1 as libc::c_int as size_t,
                             maxcount: 768 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dmodel_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"models\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 0 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.submodels as *mut *mut dmodel_t
                                     as *mut *const libc::c_void,
                             count: &mut srcmodel.numsubmodels,};
             init
         }];
    extlumps =
        [{
             let mut init =
                 mlumpinfo_t{lumpnumber: 0 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 0x2000000 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<byte>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"deluxmaps\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 1 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.deluxdata as *mut *mut byte as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.deluxdatasize,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 1 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount: 8192 as libc::c_int as size_t,
                             entrysize:
                                 ::std::mem::size_of::<dfaceinfo_t>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"faceinfos\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 0 as libc::c_int |
                                      (1 as libc::c_uint) << 1 as libc::c_int)
                                     as libc::c_int,
                             dataptr:
                                 &mut srcmodel.faceinfo as
                                     *mut *mut dfaceinfo_t as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.numfaceinfo,};
             init
         },
         {
             let mut init =
                 mlumpinfo_t{lumpnumber: 8 as libc::c_int,
                             mincount: 0 as libc::c_int as size_t,
                             maxcount:
                                 (0x2000000 as libc::c_int / 3 as libc::c_int)
                                     as size_t,
                             entrysize:
                                 ::std::mem::size_of::<byte>() as
                                     libc::c_ulong as libc::c_int,
                             entrysize32: -(1 as libc::c_int),
                             loadname:
                                 b"shadowmap\x00" as *const u8 as
                                     *const libc::c_char,
                             flags:
                                 ((1 as libc::c_uint) << 1 as libc::c_int) as
                                     libc::c_int,
                             dataptr:
                                 &mut srcmodel.shadowdata as *mut *mut byte as
                                     *mut *const libc::c_void,
                             count: &mut srcmodel.shadowdatasize,};
             init
         },
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,},
         mlumpinfo_t{lumpnumber: 0,
                     mincount: 0,
                     maxcount: 0,
                     entrysize: 0,
                     entrysize32: 0,
                     loadname: 0 as *const libc::c_char,
                     flags: 0,
                     dataptr: 0 as *mut *const libc::c_void,
                     count: 0 as *mut size_t,}]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
