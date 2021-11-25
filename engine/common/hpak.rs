#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn COM_ReplaceExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn MD5Init(ctx: *mut MD5Context_t);
    #[no_mangle]
    fn MD5Update(ctx: *mut MD5Context_t, buf: *const byte, len: uint);
    #[no_mangle]
    fn MD5Final(digest: *mut byte, ctx: *mut MD5Context_t);
    #[no_mangle]
    fn MD5_Print(hash: *mut byte) -> *mut libc::c_char;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_WriteFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: fs_offset_t) -> qboolean;
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
    fn FS_FileSize(filename: *const libc::c_char, gamedironly: qboolean)
     -> fs_offset_t;
    #[no_mangle]
    fn FS_Rename(oldname: *const libc::c_char, newname: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn FS_FileCopy(pOutput: *mut file_t, pInput: *mut file_t,
                   fileSize: libc::c_int) -> qboolean;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn FS_Tell(file: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn FS_FileLength(f: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint32_t = __uint32_t;
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
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
pub type fs_offset_t = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5Context_t {
    pub buf: [uint; 4],
    pub bits: [uint; 2],
    pub in_0: [uint; 16],
}
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
pub type host_parm_t = host_parm_s;
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
pub type hash_pack_queue_t = hash_pack_queue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_pack_queue_s {
    pub name: *mut libc::c_char,
    pub resource: resource_t,
    pub size: size_t,
    pub data: *mut libc::c_void,
    pub next: *mut hash_pack_queue_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hpak_lump_t {
    pub resource: resource_t,
    pub filepos: libc::c_int,
    pub disksize: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hpak_info_t {
    pub count: libc::c_int,
    pub entries: *mut hpak_lump_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hpak_header_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub infotableofs: libc::c_int,
}
#[no_mangle]
pub static mut hpk_maxsize: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut gp_hpak_queue: *mut hash_pack_queue_t =
    0 as *const hash_pack_queue_t as *mut hash_pack_queue_t;
#[no_mangle]
pub static mut hash_pack_header: hpak_header_t =
    hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
#[no_mangle]
pub static mut hash_pack_info: hpak_info_t =
    hpak_info_t{count: 0,
                entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
#[no_mangle]
pub unsafe extern "C" fn HPAK_TypeFromIndex(mut type_0: libc::c_int)
 -> *const libc::c_char {
    match type_0 {
        0 => { return b"decal\x00" as *const u8 as *const libc::c_char }
        1 => { return b"skin\x00" as *const u8 as *const libc::c_char }
        2 => { return b"model\x00" as *const u8 as *const libc::c_char }
        3 => { return b"decal\x00" as *const u8 as *const libc::c_char }
        4 => { return b"generic\x00" as *const u8 as *const libc::c_char }
        5 => { return b"event\x00" as *const u8 as *const libc::c_char }
        6 => { return b"map\x00" as *const u8 as *const libc::c_char }
        _ => { }
    }
    return b"?\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn HPAK_AddToQueue(mut name: *const libc::c_char,
                                     mut pResource: *mut resource_t,
                                     mut data: *mut libc::c_void,
                                     mut f: *mut file_t) {
    let mut p: *mut hash_pack_queue_t = 0 as *mut hash_pack_queue_t;
    p =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<hash_pack_queue_t>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 56 as libc::c_int) as
            *mut hash_pack_queue_t;
    (*p).name =
        _copystring(host.mempool, name,
                    b"../engine/common/hpak.c\x00" as *const u8 as
                        *const libc::c_char, 57 as libc::c_int);
    (*p).resource = *pResource;
    (*p).size = (*pResource).nDownloadSize as size_t;
    (*p).data =
        _Mem_Alloc(host.mempool, (*p).size, false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 60 as libc::c_int);
    if !data.is_null() {
        memcpy((*p).data, data, (*p).size);
    } else if !f.is_null() {
        FS_Read(f, (*p).data, (*p).size);
    } else {
        Host_Error(b"HPAK_AddToQueue: data == NULL.\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    (*p).next = gp_hpak_queue;
    gp_hpak_queue = p;
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_FlushHostQueue() {
    let mut p: *mut hash_pack_queue_t = 0 as *mut hash_pack_queue_t;
    p = gp_hpak_queue;
    while !p.is_null() {
        gp_hpak_queue = (*p).next;
        HPAK_AddLump(false_0, (*p).name, &mut (*p).resource,
                     (*p).data as *mut byte, 0 as *mut file_t);
        if !(*p).name.is_null() {
            _Mem_Free((*p).name as *mut libc::c_void,
                      b"../engine/common/hpak.c\x00" as *const u8 as
                          *const libc::c_char, 78 as libc::c_int);
            (*p).name = 0 as *mut libc::c_char
        }
        _Mem_Free((*p).data,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 79 as libc::c_int);
        _Mem_Free(p as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 80 as libc::c_int);
        p = gp_hpak_queue
    }
    gp_hpak_queue = 0 as *mut hash_pack_queue_t;
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_CreatePak(mut filename: *const libc::c_char,
                                        mut pResource: *mut resource_t,
                                        mut pData: *mut byte,
                                        mut fin: *mut file_t) {
    let mut filelocation: libc::c_int = 0;
    let mut pakname: string = [0; 256];
    let mut md5: [byte; 16] = [0; 16];
    let mut fout: *mut file_t = 0 as *mut file_t;
    let mut ctx: MD5Context_t =
        MD5Context_t{buf: [0; 4], bits: [0; 2], in_0: [0; 16],};
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    if !fin.is_null() && !pData.is_null() || fin.is_null() && pData.is_null()
       {
        return
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    Con_Printf(b"creating HPAK %s.\n\x00" as *const u8 as *const libc::c_char,
               pakname.as_mut_ptr());
    fout =
        FS_Open(pakname.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char, false_0);
    if fout.is_null() {
        Con_DPrintf(b"^1Error:^7 HPAK_CreatePak: can\'t write %s.\n\x00" as
                        *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr());
        return
    }
    // let's hash it.
    memset(&mut ctx as *mut MD5Context_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<MD5Context_t>() as libc::c_ulong);
    MD5Init(&mut ctx);
    if pData.is_null() {
        let mut temp: *mut byte = 0 as *mut byte;
        // there are better ways
        filelocation = FS_Tell(fin) as libc::c_int;
        temp =
            _Mem_Alloc(host.mempool, (*pResource).nDownloadSize as size_t,
                       false_0,
                       b"../engine/common/hpak.c\x00" as *const u8 as
                           *const libc::c_char, 121 as libc::c_int) as
                *mut byte;
        FS_Read(fin, temp as *mut libc::c_void,
                (*pResource).nDownloadSize as size_t);
        FS_Seek(fin, filelocation as fs_offset_t, 0 as libc::c_int);
        MD5Update(&mut ctx, temp, (*pResource).nDownloadSize as uint);
        _Mem_Free(temp as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 125 as libc::c_int);
    } else { MD5Update(&mut ctx, pData, (*pResource).nDownloadSize as uint); }
    MD5Final(md5.as_mut_ptr(), &mut ctx);
    if memcmp(md5.as_mut_ptr() as *const libc::c_void,
              (*pResource).rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
              16 as libc::c_int as libc::c_ulong) != 0 {
        Con_DPrintf(b"^1Error:^7 HPAK_CreatePak: bad checksum for %s. Ignored\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr());
        return
    }
    hash_pack_header.ident =
        (('K' as i32) << 24 as libc::c_int) +
            (('A' as i32) << 16 as libc::c_int) +
            (('P' as i32) << 8 as libc::c_int) + 'H' as i32;
    hash_pack_header.version = 1 as libc::c_int;
    hash_pack_header.infotableofs = 0 as libc::c_int;
    FS_Write(fout,
             &mut hash_pack_header as *mut hpak_header_t as
                 *const libc::c_void,
             ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    hash_pack_info.count = 1 as libc::c_int;
    hash_pack_info.entries =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<hpak_lump_t>() as libc::c_ulong,
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 147 as libc::c_int) as
            *mut hpak_lump_t;
    (*hash_pack_info.entries.offset(0 as libc::c_int as isize)).resource =
        *pResource;
    (*hash_pack_info.entries.offset(0 as libc::c_int as isize)).filepos =
        FS_Tell(fout) as libc::c_int;
    (*hash_pack_info.entries.offset(0 as libc::c_int as isize)).disksize =
        (*pResource).nDownloadSize;
    if pData.is_null() {
        FS_FileCopy(fout, fin,
                    (*hash_pack_info.entries.offset(0 as libc::c_int as
                                                        isize)).disksize);
    } else {
        FS_Write(fout, pData as *const libc::c_void,
                 (*hash_pack_info.entries.offset(0 as libc::c_int as
                                                     isize)).disksize as
                     size_t);
    }
    filelocation = FS_Tell(fout) as libc::c_int;
    FS_Write(fout,
             &mut hash_pack_info.count as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(fout,
             &mut *hash_pack_info.entries.offset(0 as libc::c_int as isize) as
                 *mut hpak_lump_t as *const libc::c_void,
             ::std::mem::size_of::<hpak_lump_t>() as libc::c_ulong);
    if !hash_pack_info.entries.is_null() {
        _Mem_Free(hash_pack_info.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 166 as libc::c_int);
    }
    memset(&mut hash_pack_info as *mut hpak_info_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<hpak_info_t>() as libc::c_ulong);
    hash_pack_header.infotableofs = filelocation;
    FS_Seek(fout, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_Write(fout,
             &mut hash_pack_header as *mut hpak_header_t as
                 *const libc::c_void,
             ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    FS_Close(fout);
}
unsafe extern "C" fn HPAK_FindResource(mut hpk: *mut hpak_info_t,
                                       mut hash: *mut byte,
                                       mut pResource: *mut resource_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*hpk).count {
        if memcmp((*(*hpk).entries.offset(i as
                                              isize)).resource.rgucMD5_hash.as_mut_ptr()
                      as *const libc::c_void, hash as *const libc::c_void,
                  16 as libc::c_int as libc::c_ulong) == 0 {
            if !pResource.is_null() {
                *pResource = (*(*hpk).entries.offset(i as isize)).resource
            }
            return true_0
        }
        i += 1
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_AddLump(mut bUseQueue: qboolean,
                                      mut name: *const libc::c_char,
                                      mut pResource: *mut resource_t,
                                      mut pData: *mut byte,
                                      mut pFile: *mut file_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut position: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut pCurrentEntry: *mut hpak_lump_t = 0 as *mut hpak_lump_t;
    let mut srcname: string = [0; 256];
    let mut dstname: string = [0; 256];
    let mut srcpak: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut dstpak: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut file_src: *mut file_t = 0 as *mut file_t;
    let mut file_dst: *mut file_t = 0 as *mut file_t;
    let mut md5: [byte; 16] = [0; 16];
    let mut ctx: MD5Context_t =
        MD5Context_t{buf: [0; 4], bits: [0; 2], in_0: [0; 16],};
    if pData.is_null() && pFile.is_null() { return }
    if (*pResource).nDownloadSize < 1 as libc::c_int * 1024 as libc::c_int ||
           (*pResource).nDownloadSize >
               128 as libc::c_int * 1024 as libc::c_int {
        Con_Printf(b"^1Error:^7 %s: invalid size %s\n\x00" as *const u8 as
                       *const libc::c_char, name,
                   Q_pretifymem((*pResource).nDownloadSize as libc::c_float,
                                2 as libc::c_int));
        return
    }
    // hash it
    memset(&mut ctx as *mut MD5Context_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<MD5Context_t>() as libc::c_ulong);
    MD5Init(&mut ctx);
    if pData.is_null() {
        let mut temp: *mut byte = 0 as *mut byte;
        // there are better ways
        position = FS_Tell(pFile) as libc::c_int;
        temp =
            _Mem_Alloc(host.mempool, (*pResource).nDownloadSize as size_t,
                       false_0,
                       b"../engine/common/hpak.c\x00" as *const u8 as
                           *const libc::c_char, 222 as libc::c_int) as
                *mut byte;
        FS_Read(pFile, temp as *mut libc::c_void,
                (*pResource).nDownloadSize as size_t);
        FS_Seek(pFile, position as fs_offset_t, 0 as libc::c_int);
        MD5Update(&mut ctx, temp, (*pResource).nDownloadSize as uint);
        _Mem_Free(temp as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 226 as libc::c_int);
    } else { MD5Update(&mut ctx, pData, (*pResource).nDownloadSize as uint); }
    MD5Final(md5.as_mut_ptr(), &mut ctx);
    if memcmp(md5.as_mut_ptr() as *const libc::c_void,
              (*pResource).rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
              16 as libc::c_int as libc::c_ulong) != 0 {
        Con_DPrintf(b"^1Error:^7 HPAK_AddLump: bad checksum for %s. Ignored\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*pResource).szFileName.as_mut_ptr());
        return
    }
    if bUseQueue as u64 != 0 {
        HPAK_AddToQueue(name, pResource, pData as *mut libc::c_void, pFile);
        return
    }
    Q_strncpy(srcname.as_mut_ptr(), name,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(srcname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    file_src =
        FS_Open(srcname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if file_src.is_null() {
        // just create new pack
        HPAK_CreatePak(name, pResource, pData, pFile);
        return
    }
    Q_strncpy(dstname.as_mut_ptr(), srcname.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(dstname.as_mut_ptr(),
                         b".hp2\x00" as *const u8 as *const libc::c_char);
    file_dst =
        FS_Open(dstname.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char, false_0);
    if file_dst.is_null() {
        Con_DPrintf(b"^1Error:^7 HPAK_AddLump: couldn\'t open %s.\n\x00" as
                        *const u8 as *const libc::c_char,
                    srcname.as_mut_ptr());
        FS_Close(file_src);
        return
    }
    // load headers
    FS_Read(file_src,
            &mut hash_pack_header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if hash_pack_header.version != 1 as libc::c_int {
        // we don't check the HPAK bit for some reason.
        Con_DPrintf(b"^1Error:^7 HPAK_AddLump: %s does not have a valid header.\n\x00"
                        as *const u8 as *const libc::c_char,
                    srcname.as_mut_ptr()); // rewind to start of file
        FS_Close(file_src);
        FS_Close(file_dst);
        return
    }
    length = FS_FileLength(file_src) as libc::c_int;
    FS_Seek(file_src, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_FileCopy(file_dst, file_src, length);
    FS_Seek(file_src, hash_pack_header.infotableofs as fs_offset_t,
            0 as libc::c_int);
    FS_Read(file_src,
            &mut srcpak.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if srcpak.count < 1 as libc::c_int || srcpak.count > 0x8000 as libc::c_int
       {
        Con_DPrintf(b"^1Error:^7 HPAK_AddLump: %s contain too many lumps.\n\x00"
                        as *const u8 as *const libc::c_char,
                    srcname.as_mut_ptr());
        FS_Close(file_src);
        FS_Close(file_dst);
        return
    }
    // load the data
    srcpak.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(srcpak.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 299 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(file_src, srcpak.entries as *mut libc::c_void,
            (::std::mem::size_of::<hpak_lump_t>() as
                 libc::c_ulong).wrapping_mul(srcpak.count as libc::c_ulong));
    // check if already exists
    if HPAK_FindResource(&mut srcpak, (*pResource).rgucMD5_hash.as_mut_ptr(),
                         0 as *mut resource_t) as u64 != 0 {
        if !srcpak.entries.is_null() {
            _Mem_Free(srcpak.entries as *mut libc::c_void,
                      b"../engine/common/hpak.c\x00" as *const u8 as
                          *const libc::c_char, 305 as libc::c_int);
        }
        FS_Close(file_src);
        FS_Close(file_dst);
        FS_Delete(dstname.as_mut_ptr());
        return
    }
    // make a new container
    dstpak.count = srcpak.count + 1 as libc::c_int;
    dstpak.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(dstpak.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 314 as libc::c_int) as
            *mut hpak_lump_t;
    memcpy(dstpak.entries as *mut libc::c_void,
           srcpak.entries as *const libc::c_void,
           srcpak.count as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < srcpak.count {
        if memcmp(md5.as_mut_ptr() as *const libc::c_void,
                  (*srcpak.entries.offset(i as
                                              isize)).resource.rgucMD5_hash.as_mut_ptr()
                      as *const libc::c_void,
                  16 as libc::c_int as libc::c_ulong) != 0 {
            pCurrentEntry =
                &mut *dstpak.entries.offset(i as isize) as *mut hpak_lump_t;
            j = i;
            while j < srcpak.count {
                *dstpak.entries.offset((j + 1 as libc::c_int) as isize) =
                    *srcpak.entries.offset(j as isize);
                j += 1
            }
        }
        i += 1
    }
    if pCurrentEntry.is_null() {
        pCurrentEntry =
            &mut *dstpak.entries.offset((dstpak.count - 1 as libc::c_int) as
                                            isize) as *mut hpak_lump_t
    }
    memset(pCurrentEntry as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<hpak_lump_t>() as libc::c_ulong);
    FS_Seek(file_dst, hash_pack_header.infotableofs as fs_offset_t,
            0 as libc::c_int);
    (*pCurrentEntry).resource = *pResource;
    (*pCurrentEntry).filepos = FS_Tell(file_dst) as libc::c_int;
    (*pCurrentEntry).disksize = (*pResource).nDownloadSize;
    if pData.is_null() {
        FS_FileCopy(file_dst, file_src, (*pCurrentEntry).disksize);
    } else {
        FS_Write(file_dst, pData as *const libc::c_void,
                 (*pCurrentEntry).disksize as size_t);
    }
    hash_pack_header.infotableofs = FS_Tell(file_dst) as libc::c_int;
    FS_Write(file_dst,
             &mut dstpak.count as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < dstpak.count {
        FS_Write(file_dst,
                 &mut *dstpak.entries.offset(i as isize) as *mut hpak_lump_t
                     as *const libc::c_void,
                 ::std::mem::size_of::<hpak_lump_t>() as libc::c_ulong);
        i += 1
    }
    // finalize
    if !srcpak.entries.is_null() {
        _Mem_Free(srcpak.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 350 as libc::c_int);
    }
    if !dstpak.entries.is_null() {
        _Mem_Free(dstpak.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 352 as libc::c_int);
    }
    FS_Seek(file_dst, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_Write(file_dst,
             &mut hash_pack_header as *mut hpak_header_t as
                 *const libc::c_void,
             ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    FS_Close(file_src);
    FS_Close(file_dst);
    FS_Delete(srcname.as_mut_ptr());
    FS_Rename(dstname.as_mut_ptr(), srcname.as_mut_ptr());
}
unsafe extern "C" fn HPAK_Validate(mut filename: *const libc::c_char,
                                   mut quiet: qboolean) -> qboolean {
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut dataDir: *mut hpak_lump_t = 0 as *mut hpak_lump_t;
    let mut hdr: hpak_header_t =
        hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
    let mut dataPak: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    let mut num_lumps: libc::c_int = 0;
    let mut MD5_Hash: MD5Context_t =
        MD5Context_t{buf: [0; 4], bits: [0; 2], in_0: [0; 16],};
    let mut pakname: string = [0; 256];
    let mut pRes: *mut resource_t = 0 as *mut resource_t;
    let mut md5: [byte; 16] = [0; 16];
    if quiet as u64 != 0 { HPAK_FlushHostQueue(); }
    // not an error - just flush queue
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return true_0
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    f =
        FS_Open(pakname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() {
        Con_DPrintf(b"^1Error:^7 Couldn\'t find %s.\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        return true_0
    }
    if quiet as u64 == 0 {
        Con_Printf(b"Validating %s\n\x00" as *const u8 as *const libc::c_char,
                   pakname.as_mut_ptr());
    }
    FS_Read(f, &mut hdr as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if hdr.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 ||
           hdr.version != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 HPAK_ValidatePak: %s does not have a valid HPAK header.\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr());
        FS_Close(f);
        return false_0
    }
    FS_Seek(f, hdr.infotableofs as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut num_lumps as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if num_lumps < 1 as libc::c_int || num_lumps > 65535 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 HPAK_ValidatePak: %s has too many lumps %u.\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr(), num_lumps);
        FS_Close(f);
        return false_0
    }
    if quiet as u64 == 0 {
        Con_Printf(b"# of Entries:  %i\n\x00" as *const u8 as
                       *const libc::c_char, num_lumps);
    }
    dataDir =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(num_lumps as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 414 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(f, dataDir as *mut libc::c_void,
            (::std::mem::size_of::<hpak_lump_t>() as
                 libc::c_ulong).wrapping_mul(num_lumps as libc::c_ulong));
    if quiet as u64 == 0 {
        Con_Printf(b"# Type Size FileName : MD5 Hash\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < num_lumps {
        if (*dataDir.offset(i as isize)).disksize < 1 as libc::c_int ||
               (*dataDir.offset(i as isize)).disksize > 131071 as libc::c_int
           {
            // odd max size
            Con_DPrintf(b"^1Error:^7 HPAK_ValidatePak: lump %i has invalid size %s\n\x00"
                            as *const u8 as *const libc::c_char, i,
                        Q_pretifymem((*dataDir.offset(i as isize)).disksize as
                                         libc::c_float, 2 as libc::c_int));
            _Mem_Free(dataDir as *mut libc::c_void,
                      b"../engine/common/hpak.c\x00" as *const u8 as
                          *const libc::c_char, 425 as libc::c_int);
            FS_Close(f);
            return false_0
        }
        dataPak =
            _Mem_Alloc(host.mempool,
                       (*dataDir.offset(i as isize)).disksize as size_t,
                       false_0,
                       b"../engine/common/hpak.c\x00" as *const u8 as
                           *const libc::c_char, 430 as libc::c_int) as
                *mut byte;
        FS_Seek(f, (*dataDir.offset(i as isize)).filepos as fs_offset_t,
                0 as libc::c_int);
        FS_Read(f, dataPak as *mut libc::c_void,
                (*dataDir.offset(i as isize)).disksize as size_t);
        memset(&mut MD5_Hash as *mut MD5Context_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<MD5Context_t>() as libc::c_ulong);
        MD5Init(&mut MD5_Hash);
        MD5Update(&mut MD5_Hash, dataPak,
                  (*dataDir.offset(i as isize)).disksize as uint);
        MD5Final(md5.as_mut_ptr(), &mut MD5_Hash);
        pRes = &mut (*dataDir.offset(i as isize)).resource;
        Con_Printf(b"%i:      %s %s %s:   \x00" as *const u8 as
                       *const libc::c_char, i,
                   HPAK_TypeFromIndex((*pRes).type_0 as libc::c_int),
                   Q_pretifymem((*pRes).nDownloadSize as libc::c_float,
                                2 as libc::c_int),
                   (*pRes).szFileName.as_mut_ptr());
        if memcmp(md5.as_mut_ptr() as *const libc::c_void,
                  (*pRes).rgucMD5_hash.as_mut_ptr() as *const libc::c_void,
                  0x10 as libc::c_int as libc::c_ulong) != 0 {
            if quiet as u64 != 0 {
                Con_DPrintf(b"^1Error:^7 HPAK_ValidatePak: %s has invalid checksum.\n\x00"
                                as *const u8 as *const libc::c_char,
                            pakname.as_mut_ptr());
                _Mem_Free(dataPak as *mut libc::c_void,
                          b"../engine/common/hpak.c\x00" as *const u8 as
                              *const libc::c_char, 449 as libc::c_int);
                _Mem_Free(dataDir as *mut libc::c_void,
                          b"../engine/common/hpak.c\x00" as *const u8 as
                              *const libc::c_char, 450 as libc::c_int);
                FS_Close(f);
                return false_0
            } else {
                Con_DPrintf(b"^1Error:^7 failed\n\x00" as *const u8 as
                                *const libc::c_char);
            }
        } else if quiet as u64 == 0 {
            Con_Printf(b"OK\n\x00" as *const u8 as *const libc::c_char);
        }
        // at this point, it's passed our checks.
        _Mem_Free(dataPak as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 462 as libc::c_int);
        i += 1
    }
    _Mem_Free(dataDir as *mut libc::c_void,
              b"../engine/common/hpak.c\x00" as *const u8 as
                  *const libc::c_char, 465 as libc::c_int);
    FS_Close(f);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_ValidatePak(mut filename: *const libc::c_char) {
    HPAK_Validate(filename, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_CheckIntegrity(mut filename:
                                                 *const libc::c_char) {
    let mut pakname: string = [0; 256];
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    HPAK_ValidatePak(pakname.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_CheckSize(mut filename: *const libc::c_char) {
    let mut pakname: string = [0; 256];
    let mut maxsize: libc::c_int = 0;
    maxsize = (*hpk_maxsize).value as libc::c_int;
    if maxsize <= 0 as libc::c_int { return }
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    if FS_FileSize(pakname.as_mut_ptr(), false_0) >
           (maxsize * 1000000 as libc::c_int) as libc::c_long {
        Con_Printf(b"Server: Size of %s > %f MB, deleting.\n\x00" as *const u8
                       as *const libc::c_char, filename,
                   (*hpk_maxsize).value as libc::c_double);
        Log_Printf(b"Server: Size of %s > %f MB, deleting.\n\x00" as *const u8
                       as *const libc::c_char, filename,
                   (*hpk_maxsize).value as libc::c_double);
        FS_Delete(filename);
    };
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_ResourceForHash(mut filename:
                                                  *const libc::c_char,
                                              mut hash: *mut byte,
                                              mut pResource: *mut resource_t)
 -> qboolean {
    let mut directory: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut header: hpak_header_t =
        hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
    let mut pakname: string = [0; 256];
    let mut bFound: qboolean = false_0;
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut p: *mut hash_pack_queue_t = 0 as *mut hash_pack_queue_t;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    p = gp_hpak_queue;
    while !p.is_null() {
        if Q_strnicmp((*p).name, filename, 99999 as libc::c_int) == 0 &&
               memcmp((*p).resource.rgucMD5_hash.as_mut_ptr() as
                          *const libc::c_void, hash as *const libc::c_void,
                      16 as libc::c_int as libc::c_ulong) == 0 {
            if !pResource.is_null() { *pResource = (*p).resource }
            return true_0
        }
        p = (*p).next
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    f =
        FS_Open(pakname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() { return false_0 }
    FS_Read(f, &mut header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 {
        FS_Close(f);
        return false_0
    }
    if header.version != 1 as libc::c_int { FS_Close(f); return false_0 }
    FS_Seek(f, header.infotableofs as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut directory.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if directory.count < 1 as libc::c_int ||
           directory.count > 0x8000 as libc::c_int {
        FS_Close(f);
        return false_0
    }
    directory.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(directory.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 561 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(f, directory.entries as *mut libc::c_void,
            (::std::mem::size_of::<hpak_lump_t>() as
                 libc::c_ulong).wrapping_mul(directory.count as
                                                 libc::c_ulong));
    bFound = HPAK_FindResource(&mut directory, hash, pResource);
    _Mem_Free(directory.entries as *mut libc::c_void,
              b"../engine/common/hpak.c\x00" as *const u8 as
                  *const libc::c_char, 564 as libc::c_int);
    FS_Close(f);
    return bFound;
}
unsafe extern "C" fn HPAK_ResourceForIndex(mut filename: *const libc::c_char,
                                           mut index: libc::c_int,
                                           mut pResource: *mut resource_t)
 -> qboolean {
    let mut header: hpak_header_t =
        hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
    let mut directory: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut pakname: string = [0; 256];
    let mut f: *mut file_t = 0 as *mut file_t;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    f =
        FS_Open(pakname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() {
        Con_DPrintf(b"^1Error:^7 couldn\'t open %s.\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        return false_0
    }
    FS_Read(f, &mut header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 {
        Con_DPrintf(b"^1Error:^7 %s is not an HPAK file\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        FS_Close(f);
        return false_0
    }
    if header.version != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has invalid version (%i should be %i).\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr(), header.version, 1 as libc::c_int);
        FS_Close(f);
        return false_0
    }
    FS_Seek(f, header.infotableofs as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut directory.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if directory.count < 1 as libc::c_int ||
           directory.count > 0x8000 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has too many lumps %u.\n\x00" as *const u8
                        as *const libc::c_char, pakname.as_mut_ptr(),
                    directory.count);
        FS_Close(f);
        return false_0
    }
    if index < 1 as libc::c_int || index > directory.count {
        Con_DPrintf(b"^1Error:^7 %s, lump with index %i doesn\'t exist.\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr(), index);
        FS_Close(f);
        return false_0
    }
    directory.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(directory.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 622 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(f, directory.entries as *mut libc::c_void,
            (::std::mem::size_of::<hpak_lump_t>() as
                 libc::c_ulong).wrapping_mul(directory.count as
                                                 libc::c_ulong));
    *pResource =
        (*directory.entries.offset((index - 1 as libc::c_int) as
                                       isize)).resource;
    if !directory.entries.is_null() {
        _Mem_Free(directory.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 625 as libc::c_int);
    }
    FS_Close(f);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_GetDataPointer(mut filename:
                                                 *const libc::c_char,
                                             mut pResource: *mut resource_t,
                                             mut buffer: *mut *mut byte,
                                             mut bufsize: *mut libc::c_int)
 -> qboolean {
    let mut tmpbuf: *mut byte = 0 as *mut byte;
    let mut pakname: string = [0; 256];
    let mut header: hpak_header_t =
        hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
    let mut directory: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut entry: *mut hpak_lump_t = 0 as *mut hpak_lump_t;
    let mut p: *mut hash_pack_queue_t = 0 as *mut hash_pack_queue_t;
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    if !buffer.is_null() { *buffer = 0 as *mut byte }
    if !bufsize.is_null() { *bufsize = 0 as libc::c_int }
    p = gp_hpak_queue;
    while !p.is_null() {
        if Q_strnicmp((*p).name, filename, 99999 as libc::c_int) == 0 &&
               memcmp((*p).resource.rgucMD5_hash.as_mut_ptr() as
                          *const libc::c_void,
                      (*pResource).rgucMD5_hash.as_mut_ptr() as
                          *const libc::c_void,
                      16 as libc::c_int as libc::c_ulong) == 0 {
            if !buffer.is_null() {
                tmpbuf =
                    _Mem_Alloc(host.mempool, (*p).size, false_0,
                               b"../engine/common/hpak.c\x00" as *const u8 as
                                   *const libc::c_char, 654 as libc::c_int) as
                        *mut byte;
                memcpy(tmpbuf as *mut libc::c_void, (*p).data, (*p).size);
                *buffer = tmpbuf
            }
            if !bufsize.is_null() { *bufsize = (*p).size as libc::c_int }
            return true_0
        }
        p = (*p).next
    }
    Q_strncpy(pakname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    f =
        FS_Open(pakname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() { return false_0 }
    FS_Read(f, &mut header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 {
        Con_DPrintf(b"^1Error:^7 %s it\'s not a HPK file.\n\x00" as *const u8
                        as *const libc::c_char, pakname.as_mut_ptr());
        FS_Close(f);
        return false_0
    }
    if header.version != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has invalid version (%i should be %i).\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr(), header.version, 1 as libc::c_int);
        FS_Close(f);
        return false_0
    }
    FS_Seek(f, header.infotableofs as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut directory.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if directory.count < 1 as libc::c_int ||
           directory.count > 0x8000 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 HPAK_GetDataPointer: %s has too many lumps %u.\n\x00"
                        as *const u8 as *const libc::c_char, filename,
                    directory.count);
        FS_Close(f);
        return false_0
    }
    directory.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(directory.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 698 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(f, directory.entries as *mut libc::c_void,
            (::std::mem::size_of::<hpak_lump_t>() as
                 libc::c_ulong).wrapping_mul(directory.count as
                                                 libc::c_ulong));
    i = 0 as libc::c_int;
    while i < directory.count {
        entry =
            &mut *directory.entries.offset(i as isize) as *mut hpak_lump_t;
        if memcmp((*entry).resource.rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void,
                  (*pResource).rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void, 16 as libc::c_int as libc::c_ulong)
               == 0 {
            FS_Seek(f, (*entry).filepos as fs_offset_t, 0 as libc::c_int);
            if !buffer.is_null() && (*entry).disksize > 0 as libc::c_int {
                tmpbuf =
                    _Mem_Alloc(host.mempool, (*entry).disksize as size_t,
                               false_0,
                               b"../engine/common/hpak.c\x00" as *const u8 as
                                   *const libc::c_char, 711 as libc::c_int) as
                        *mut byte;
                FS_Read(f, tmpbuf as *mut libc::c_void,
                        (*entry).disksize as size_t);
                *buffer = tmpbuf
            }
            if !bufsize.is_null() { *bufsize = (*entry).disksize }
            _Mem_Free(directory.entries as *mut libc::c_void,
                      b"../engine/common/hpak.c\x00" as *const u8 as
                          *const libc::c_char, 719 as libc::c_int);
            FS_Close(f);
            return true_0
        }
        i += 1
    }
    _Mem_Free(directory.entries as *mut libc::c_void,
              b"../engine/common/hpak.c\x00" as *const u8 as
                  *const libc::c_char, 726 as libc::c_int);
    FS_Close(f);
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_RemoveLump(mut name: *const libc::c_char,
                                         mut pResource: *mut resource_t) {
    let mut read_path: string = [0; 256];
    let mut save_path: string = [0; 256];
    let mut file_src: *mut file_t = 0 as *mut file_t;
    let mut file_dst: *mut file_t = 0 as *mut file_t;
    let mut hpak_read: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut hpak_save: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (if name.is_null() || *name == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 || pResource.is_null() {
        return
    }
    HPAK_FlushHostQueue();
    Q_strncpy(read_path.as_mut_ptr(), name,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(read_path.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    file_src =
        FS_Open(read_path.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if file_src.is_null() {
        Con_DPrintf(b"^1Error:^7 %s couldn\'t open.\n\x00" as *const u8 as
                        *const libc::c_char, read_path.as_mut_ptr());
        return
    }
    Q_strncpy(save_path.as_mut_ptr(), read_path.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(save_path.as_mut_ptr(),
                         b".hp2\x00" as *const u8 as *const libc::c_char);
    file_dst =
        FS_Open(save_path.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char, false_0);
    if file_dst.is_null() {
        Con_DPrintf(b"^1Error:^7 %s couldn\'t open.\n\x00" as *const u8 as
                        *const libc::c_char, save_path.as_mut_ptr());
        FS_Close(file_src);
        return
    }
    FS_Seek(file_src, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_Seek(file_dst, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    // header copy
    FS_Read(file_src,
            &mut hash_pack_header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as
                libc::c_ulong); // delete temp file
    FS_Write(file_dst,
             &mut hash_pack_header as *mut hpak_header_t as
                 *const libc::c_void,
             ::std::mem::size_of::<hpak_header_t>() as
                 libc::c_ulong); // delete temp file
    if hash_pack_header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 ||
           hash_pack_header.version != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has invalid header.\n\x00" as *const u8 as
                        *const libc::c_char, read_path.as_mut_ptr());
        FS_Close(file_src);
        FS_Close(file_dst);
        FS_Delete(save_path.as_mut_ptr());
        return
    }
    FS_Seek(file_src, hash_pack_header.infotableofs as fs_offset_t,
            0 as libc::c_int);
    FS_Read(file_src,
            &mut hpak_read.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if hpak_read.count < 1 as libc::c_int ||
           hpak_read.count > 0x8000 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has invalid number of lumps.\n\x00" as
                        *const u8 as *const libc::c_char,
                    read_path.as_mut_ptr());
        FS_Close(file_src);
        FS_Close(file_dst);
        FS_Delete(save_path.as_mut_ptr());
        return
    }
    if hpak_read.count == 1 as libc::c_int {
        Con_DPrintf(b"^3Warning:^7 %s only has one element, so HPAK will be removed\n\x00"
                        as *const u8 as *const libc::c_char,
                    read_path.as_mut_ptr());
        FS_Close(file_src);
        FS_Close(file_dst);
        FS_Delete(read_path.as_mut_ptr());
        FS_Delete(save_path.as_mut_ptr());
        return
    }
    hpak_save.count = hpak_read.count - 1 as libc::c_int;
    hpak_read.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(hpak_read.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 807 as libc::c_int) as
            *mut hpak_lump_t;
    hpak_save.entries =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<hpak_lump_t>() as
                        libc::c_ulong).wrapping_mul(hpak_save.count as
                                                        libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 808 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(file_src, hpak_read.entries as *mut libc::c_void,
            (::std::mem::size_of::<hpak_lump_t>() as
                 libc::c_ulong).wrapping_mul(hpak_read.count as
                                                 libc::c_ulong));
    if HPAK_FindResource(&mut hpak_read,
                         (*pResource).rgucMD5_hash.as_mut_ptr(),
                         0 as *mut resource_t) as u64 == 0 {
        Con_DPrintf(b"^1Error:^7 HPAK %s doesn\'t contain specified lump: %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    read_path.as_mut_ptr(),
                    (*pResource).szFileName.as_mut_ptr());
        _Mem_Free(hpak_read.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 815 as libc::c_int);
        _Mem_Free(hpak_save.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 816 as libc::c_int);
        FS_Close(file_src);
        FS_Close(file_dst);
        FS_Delete(save_path.as_mut_ptr());
        return
    }
    Con_Printf(b"Removing %s from HPAK %s.\n\x00" as *const u8 as
                   *const libc::c_char, (*pResource).szFileName.as_mut_ptr(),
               read_path.as_mut_ptr());
    // If there's a collision, we've just corrupted this hpak.
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < hpak_read.count {
        if !(memcmp((*hpak_read.entries.offset(i as
                                                   isize)).resource.rgucMD5_hash.as_mut_ptr()
                        as *const libc::c_void,
                    (*pResource).rgucMD5_hash.as_mut_ptr() as
                        *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong) == 0) {
            *hpak_save.entries.offset(j as isize) =
                *hpak_read.entries.offset(i as isize);
            (*hpak_save.entries.offset(j as isize)).filepos =
                FS_Tell(file_dst) as libc::c_int;
            FS_Seek(file_src,
                    (*hpak_read.entries.offset(j as isize)).filepos as
                        fs_offset_t, 0 as libc::c_int);
            FS_FileCopy(file_dst, file_src,
                        (*hpak_save.entries.offset(j as isize)).disksize);
            j += 1
        }
        i += 1
    }
    hash_pack_header.infotableofs = FS_Tell(file_dst) as libc::c_int;
    FS_Write(file_dst,
             &mut hpak_save.count as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < hpak_save.count {
        FS_Write(file_dst,
                 &mut *hpak_save.entries.offset(i as isize) as
                     *mut hpak_lump_t as *const libc::c_void,
                 ::std::mem::size_of::<hpak_lump_t>() as libc::c_ulong);
        i += 1
    }
    FS_Seek(file_dst, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_Write(file_dst,
             &mut hash_pack_header as *mut hpak_header_t as
                 *const libc::c_void,
             ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    _Mem_Free(hpak_read.entries as *mut libc::c_void,
              b"../engine/common/hpak.c\x00" as *const u8 as
                  *const libc::c_char, 847 as libc::c_int);
    _Mem_Free(hpak_save.entries as *mut libc::c_void,
              b"../engine/common/hpak.c\x00" as *const u8 as
                  *const libc::c_char, 848 as libc::c_int);
    FS_Close(file_src);
    FS_Close(file_dst);
    FS_Delete(read_path.as_mut_ptr());
    FS_Rename(save_path.as_mut_ptr(), read_path.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_List_f() {
    let mut nCurrent: libc::c_int = 0;
    let mut header: hpak_header_t =
        hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
    let mut directory: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut entry: *mut hpak_lump_t = 0 as *mut hpak_lump_t;
    let mut lumpname: string = [0; 256];
    let mut pakname: string = [0; 256];
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: *const libc::c_char = 0 as *const libc::c_char;
    let mut f: *mut file_t = 0 as *mut file_t;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: hpklist <hpk>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    HPAK_FlushHostQueue();
    Q_strncpy(pakname.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    Con_Printf(b"Contents for %s.\n\x00" as *const u8 as *const libc::c_char,
               pakname.as_mut_ptr());
    f =
        FS_Open(pakname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() {
        Con_DPrintf(b"^1Error:^7 couldn\'t open %s.\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        return
    }
    FS_Read(f, &mut header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 {
        Con_DPrintf(b"^1Error:^7 %s is not an HPAK file\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        FS_Close(f);
        return
    }
    if header.version != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has invalid version (%i should be %i).\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr(), header.version, 1 as libc::c_int);
        FS_Close(f);
        return
    }
    FS_Seek(f, header.infotableofs as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut directory.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if directory.count < 1 as libc::c_int ||
           directory.count > 0x8000 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has too many lumps %u.\n\x00" as *const u8
                        as *const libc::c_char, pakname.as_mut_ptr(),
                    directory.count);
        FS_Close(f);
        return
    }
    Con_Printf(b"# of Entries:  %i\n\x00" as *const u8 as *const libc::c_char,
               directory.count);
    Con_Printf(b"# Type Size FileName : MD5 Hash\n\x00" as *const u8 as
                   *const libc::c_char);
    directory.entries =
        _Mem_Alloc(host.mempool,
                   (directory.count as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<hpak_lump_t>()
                                                        as libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 916 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(f, directory.entries as *mut libc::c_void,
            (directory.count as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<hpak_lump_t>()
                                                 as libc::c_ulong));
    nCurrent = 0 as libc::c_int;
    while nCurrent < directory.count {
        entry =
            &mut *directory.entries.offset(nCurrent as isize) as
                *mut hpak_lump_t;
        COM_FileBase((*entry).resource.szFileName.as_mut_ptr(),
                     lumpname.as_mut_ptr());
        type_0 = HPAK_TypeFromIndex((*entry).resource.type_0 as libc::c_int);
        size =
            Q_pretifymem((*entry).resource.nDownloadSize as libc::c_float,
                         2 as libc::c_int);
        Con_Printf(b"%i: %10s %s %s\n  :  %s\n\x00" as *const u8 as
                       *const libc::c_char, nCurrent + 1 as libc::c_int,
                   type_0, size, lumpname.as_mut_ptr(),
                   MD5_Print((*entry).resource.rgucMD5_hash.as_mut_ptr()));
        nCurrent += 1
    }
    if !directory.entries.is_null() {
        _Mem_Free(directory.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 930 as libc::c_int);
    }
    FS_Close(f);
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_Extract_f() {
    let mut nCurrent: libc::c_int = 0;
    let mut header: hpak_header_t =
        hpak_header_t{ident: 0, version: 0, infotableofs: 0,};
    let mut directory: hpak_info_t =
        hpak_info_t{count: 0,
                    entries: 0 as *const hpak_lump_t as *mut hpak_lump_t,};
    let mut entry: *mut hpak_lump_t = 0 as *mut hpak_lump_t;
    let mut lumpname: string = [0; 256];
    let mut pakname: string = [0; 256];
    let mut szFileOut: string = [0; 256];
    let mut nIndex: libc::c_int = 0;
    let mut pData: *mut byte = 0 as *mut byte;
    let mut nDataSize: libc::c_int = 0;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: *const libc::c_char = 0 as *const libc::c_char;
    let mut f: *mut file_t = 0 as *mut file_t;
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: hpkextract hpkname [all | single index]\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    if Q_strnicmp(Cmd_Argv(2 as libc::c_int),
                  b"all\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        nIndex = -(1 as libc::c_int)
    } else { nIndex = Q_atoi(Cmd_Argv(2 as libc::c_int)) }
    HPAK_FlushHostQueue();
    Q_strncpy(pakname.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(pakname.as_mut_ptr(),
                         b".hpk\x00" as *const u8 as *const libc::c_char);
    Con_Printf(b"Contents for %s.\n\x00" as *const u8 as *const libc::c_char,
               pakname.as_mut_ptr());
    f =
        FS_Open(pakname.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, false_0);
    if f.is_null() {
        Con_DPrintf(b"^1Error:^7 couldn\'t open %s.\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        return
    }
    FS_Read(f, &mut header as *mut hpak_header_t as *mut libc::c_void,
            ::std::mem::size_of::<hpak_header_t>() as libc::c_ulong);
    if header.ident !=
           (('K' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('P' as i32) << 8 as libc::c_int) + 'H' as i32 {
        Con_DPrintf(b"^1Error:^7 %s is not an HPAK file\n\x00" as *const u8 as
                        *const libc::c_char, pakname.as_mut_ptr());
        FS_Close(f);
        return
    }
    if header.version != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has invalid version (%i should be %i).\n\x00"
                        as *const u8 as *const libc::c_char,
                    pakname.as_mut_ptr(), header.version, 1 as libc::c_int);
        FS_Close(f);
        return
    }
    FS_Seek(f, header.infotableofs as fs_offset_t, 0 as libc::c_int);
    FS_Read(f, &mut directory.count as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if directory.count < 1 as libc::c_int ||
           directory.count > 0x8000 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has too many lumps %u.\n\x00" as *const u8
                        as *const libc::c_char, pakname.as_mut_ptr(),
                    directory.count);
        FS_Close(f);
        return
    }
    if nIndex == -(1 as libc::c_int) {
        Con_Printf(b"Extracting all lumps from %s.\n\x00" as *const u8 as
                       *const libc::c_char, pakname.as_mut_ptr());
    } else {
        Con_Printf(b"Extracting lump %i from %s\n\x00" as *const u8 as
                       *const libc::c_char, nIndex, pakname.as_mut_ptr());
    }
    directory.entries =
        _Mem_Alloc(host.mempool,
                   (directory.count as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<hpak_lump_t>()
                                                        as libc::c_ulong),
                   false_0,
                   b"../engine/common/hpak.c\x00" as *const u8 as
                       *const libc::c_char, 1007 as libc::c_int) as
            *mut hpak_lump_t;
    FS_Read(f, directory.entries as *mut libc::c_void,
            (directory.count as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<hpak_lump_t>()
                                                 as libc::c_ulong));
    nCurrent = 0 as libc::c_int;
    while nCurrent < directory.count {
        entry =
            &mut *directory.entries.offset(nCurrent as isize) as
                *mut hpak_lump_t;
        if !(nIndex != -(1 as libc::c_int) && nIndex != nCurrent) {
            COM_FileBase((*entry).resource.szFileName.as_mut_ptr(),
                         lumpname.as_mut_ptr());
            type_0 =
                HPAK_TypeFromIndex((*entry).resource.type_0 as libc::c_int);
            size =
                Q_pretifymem((*entry).resource.nDownloadSize as libc::c_float,
                             2 as libc::c_int);
            Con_Printf(b"Extracting %i: %10s %s %s\n\x00" as *const u8 as
                           *const libc::c_char, nCurrent + 1 as libc::c_int,
                       type_0, size, lumpname.as_mut_ptr());
            if (*entry).disksize <= 0 as libc::c_int ||
                   (*entry).disksize >=
                       128 as libc::c_int * 1024 as libc::c_int {
                Con_DPrintf(b"^3Warning:^7 Unable to extract data, size invalid:  %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            Q_pretifymem((*entry).disksize as libc::c_float,
                                         2 as libc::c_int));
            } else {
                nDataSize = (*entry).disksize;
                pData =
                    _Mem_Alloc(host.mempool,
                               (nDataSize + 1 as libc::c_int) as size_t,
                               false_0,
                               b"../engine/common/hpak.c\x00" as *const u8 as
                                   *const libc::c_char, 1030 as libc::c_int)
                        as *mut byte;
                FS_Seek(f, (*entry).filepos as fs_offset_t, 0 as libc::c_int);
                FS_Read(f, pData as *mut libc::c_void, nDataSize as size_t);
                Q_snprintf(szFileOut.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong,
                           b"hpklmps\\lmp%04i.bmp\x00" as *const u8 as
                               *const libc::c_char, nCurrent);
                FS_WriteFile(szFileOut.as_mut_ptr(),
                             pData as *const libc::c_void,
                             nDataSize as fs_offset_t);
                if !pData.is_null() {
                    _Mem_Free(pData as *mut libc::c_void,
                              b"../engine/common/hpak.c\x00" as *const u8 as
                                  *const libc::c_char, 1036 as libc::c_int);
                }
            }
        }
        nCurrent += 1
    }
    if !directory.entries.is_null() {
        _Mem_Free(directory.entries as *mut libc::c_void,
                  b"../engine/common/hpak.c\x00" as *const u8 as
                      *const libc::c_char, 1040 as libc::c_int);
    }
    FS_Close(f);
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_Remove_f() {
    let mut resource: resource_t =
        resource_t{szFileName: [0; 64],
                   type_0: t_sound,
                   nIndex: 0,
                   nDownloadSize: 0,
                   ucFlags: 0,
                   rgucMD5_hash: [0; 16],
                   playernum: 0,
                   rguc_reserved: [0; 32],
                   pNext: 0 as *mut resource_s,
                   pPrev: 0 as *mut resource_s,};
    HPAK_FlushHostQueue();
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: hpkremove <hpk> <index>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if HPAK_ResourceForIndex(Cmd_Argv(1 as libc::c_int),
                             Q_atoi(Cmd_Argv(2 as libc::c_int)),
                             &mut resource) as u64 != 0 {
        HPAK_RemoveLump(Cmd_Argv(1 as libc::c_int), &mut resource);
    } else {
        Con_DPrintf(b"^1Error:^7 Could not locate resource %i in %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    Q_atoi(Cmd_Argv(2 as libc::c_int)),
                    Cmd_Argv(1 as libc::c_int));
    };
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_Validate_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: hpkval <filename>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    HPAK_Validate(Cmd_Argv(1 as libc::c_int), false_0);
}
#[no_mangle]
pub unsafe extern "C" fn HPAK_Init() {
    Cmd_AddRestrictedCommand(b"hpklist\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HPAK_List_f as
                                      unsafe extern "C" fn() -> ()),
                             b"list all files in specified HPK-file\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"hpkremove\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HPAK_Remove_f as
                                      unsafe extern "C" fn() -> ()),
                             b"remove specified file from HPK-file\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"hpkval\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HPAK_Validate_f as
                                      unsafe extern "C" fn() -> ()),
                             b"validate specified HPK-file\x00" as *const u8
                                 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"hpkextract\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(HPAK_Extract_f as
                                      unsafe extern "C" fn() -> ()),
                             b"extract all lumps from specified HPK-file\x00"
                                 as *const u8 as *const libc::c_char);
    hpk_maxsize =
        Cvar_Get(b"hpk_maxsize\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"set limit by size for all HPK-files ( 0 - unlimited )\x00"
                     as *const u8 as *const libc::c_char);
    gp_hpak_queue = 0 as *mut hash_pack_queue_t;
}
