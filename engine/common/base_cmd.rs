#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type decallist_s;
    pub type cmd_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Cmd_GetName(cmd: *mut cmd_s) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_AliasGetList() -> *mut cmdalias_s;
    #[no_mangle]
    fn Cmd_GetNextFunctionHandle(cmd: *mut cmd_s) -> *mut cmd_s;
    #[no_mangle]
    fn Cmd_GetFirstFunctionHandle() -> *mut cmd_s;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_LookupVars(checkbit: libc::c_int, buffer: *mut libc::c_void,
                       ptr: *mut libc::c_void, callback: setpair_t);
    #[no_mangle]
    fn COM_HashKey(string: *const libc::c_char, hashSize: uint) -> uint;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
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
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type setpair_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char,
                                _: *const libc::c_void, _: *mut libc::c_void,
                                _: *mut libc::c_void) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: [libc::c_char; 32],
    pub value: *mut libc::c_char,
}
pub type base_command_type = libc::c_uint;
pub const HM_CMDALIAS: base_command_type = 3;
pub const HM_CMD: base_command_type = 2;
pub const HM_CVAR: base_command_type = 1;
pub const HM_DONTCARE: base_command_type = 0;
pub type base_command_type_e = base_command_type;
pub type base_command_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base_command_hashmap_s {
    pub basecmd: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub type_0: base_command_type_e,
    pub next: *mut base_command_hashmap_s,
}
pub type base_command_hashmap_t = base_command_hashmap_s;
pub type cmdalias_t = cmdalias_s;
// 128 * 4 * 4 == 2048 bytes
static mut hashed_cmds: [*mut base_command_hashmap_t; 128] =
    [0 as *const base_command_hashmap_t as *mut base_command_hashmap_t; 128];
/*
============
BaseCmd_FindInBucket

Find base command in bucket
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_FindInBucket(mut bucket:
                                                  *mut base_command_hashmap_t,
                                              mut type_0: base_command_type_e,
                                              mut name: *const libc::c_char)
 -> *mut base_command_hashmap_t {
    let mut i: *mut base_command_hashmap_t = bucket;
    while !i.is_null() &&
              ((*i).type_0 as libc::c_uint != type_0 as libc::c_uint ||
                   Q_strnicmp(name, (*i).name, 99999 as libc::c_int) != 0) {
        i = (*i).next
    }
    return i;
}
/*
============
BaseCmd_GetBucket

Get bucket which contain basecmd by given name
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_GetBucket(mut name: *const libc::c_char)
 -> *mut base_command_hashmap_t {
    return hashed_cmds[COM_HashKey(name, 128 as libc::c_int as uint) as
                           usize];
}
/*
============
BaseCmd_Find

Find base command in hashmap
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Find(mut type_0: base_command_type_e,
                                      mut name: *const libc::c_char)
 -> *mut libc::c_void {
    let mut base: *mut base_command_hashmap_t = BaseCmd_GetBucket(name);
    let mut found: *mut base_command_hashmap_t =
        BaseCmd_FindInBucket(base, type_0, name);
    if !found.is_null() { return (*found).basecmd }
    return 0 as *mut libc::c_void;
}
/*
============
BaseCmd_Find

Find every type of base command and write into arguments
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_FindAll(mut name: *const libc::c_char,
                                         mut cmd: *mut *mut libc::c_void,
                                         mut alias: *mut *mut libc::c_void,
                                         mut cvar: *mut *mut libc::c_void) {
    let mut base: *mut base_command_hashmap_t = BaseCmd_GetBucket(name);
    let mut i: *mut base_command_hashmap_t = base;
    if !(!cmd.is_null() && !alias.is_null() && !cvar.is_null()) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/base_cmd.c\x00" as *const u8 as
                      *const libc::c_char, 81 as libc::c_int);
    }
    *cvar = 0 as *mut libc::c_void;
    *alias = *cvar;
    *cmd = *alias;
    while !i.is_null() {
        if Q_strnicmp((*i).name, name, 99999 as libc::c_int) == 0 {
            match (*i).type_0 as libc::c_uint {
                2 => { *cmd = (*i).basecmd }
                3 => { *alias = (*i).basecmd }
                1 => { *cvar = (*i).basecmd }
                _ => { }
            }
        }
        i = (*i).next
    };
}
/*
============
BaseCmd_Insert

Add new typed base command to hashmap
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Insert(mut type_0: base_command_type_e,
                                        mut basecmd: *mut libc::c_void,
                                        mut name: *const libc::c_char) {
    let mut hash: uint = COM_HashKey(name, 128 as libc::c_int as uint);
    let mut elem: *mut base_command_hashmap_t =
        0 as *mut base_command_hashmap_t;
    elem =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<base_command_hashmap_t>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/base_cmd.c\x00" as *const u8 as
                       *const libc::c_char, 119 as libc::c_int) as
            *mut base_command_hashmap_t;
    (*elem).basecmd = basecmd;
    (*elem).type_0 = type_0;
    (*elem).name = name;
    (*elem).next = hashed_cmds[hash as usize];
    hashed_cmds[hash as usize] = elem;
}
/*
============
BaseCmd_Replace

Used in case, when basecmd has been registered, but gamedll wants to register it's own
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Replace(mut type_0: base_command_type_e,
                                         mut basecmd: *mut libc::c_void,
                                         mut name: *const libc::c_char)
 -> qboolean {
    let mut i: *mut base_command_hashmap_t =
        BaseCmd_GetBucket(name); // may be freed after
    while !i.is_null() &&
              ((*i).type_0 as libc::c_uint != type_0 as libc::c_uint ||
                   Q_strnicmp(name, (*i).name, 99999 as libc::c_int) != 0) {
        i = (*i).next
    }
    if i.is_null() {
        Con_Reportf(b"^1Error:^7 BaseCmd_Replace: couldn\'t find %s\n\x00" as
                        *const u8 as *const libc::c_char, name);
        return false_0
    }
    (*i).basecmd = basecmd;
    (*i).name = name;
    return true_0;
}
/*
============
BaseCmd_Remove

Remove base command from hashmap
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Remove(mut type_0: base_command_type_e,
                                        mut name: *const libc::c_char) {
    let mut hash: uint = COM_HashKey(name, 128 as libc::c_int as uint);
    let mut i: *mut base_command_hashmap_t = 0 as *mut base_command_hashmap_t;
    let mut prev: *mut base_command_hashmap_t =
        0 as *mut base_command_hashmap_t;
    prev = 0 as *mut base_command_hashmap_t;
    i = hashed_cmds[hash as usize];
    while !i.is_null() &&
              (Q_strncmp((*i).name, name, 99999 as libc::c_int) != 0 ||
                   (*i).type_0 as libc::c_uint != type_0 as libc::c_uint) {
        prev = i;
        i = (*i).next
    }
    if i.is_null() {
        Con_Reportf(b"^1Error:^7 Couldn\'t find %s in buckets\n\x00" as
                        *const u8 as *const libc::c_char, name);
        return
    }
    if !prev.is_null() {
        (*prev).next = (*i).next
    } else { hashed_cmds[hash as usize] = (*i).next }
    if !i.is_null() {
        _Mem_Free(i as *mut libc::c_void,
                  b"../engine/common/base_cmd.c\x00" as *const u8 as
                      *const libc::c_char, 180 as libc::c_int);
    };
}
/*
============
BaseCmd_Init

initialize base command hashmap system
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Init() {
    memset(hashed_cmds.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut base_command_hashmap_t; 128]>() as
               libc::c_ulong);
}
/*
============
BaseCmd_Stats_f

============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Stats_f() {
    let mut i: libc::c_int = 0;
    let mut minsize: libc::c_int = 99999 as libc::c_int;
    let mut maxsize: libc::c_int = -(1 as libc::c_int);
    let mut empty: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        let mut hm: *mut base_command_hashmap_t =
            0 as *mut base_command_hashmap_t;
        let mut len: libc::c_int = 0 as libc::c_int;
        // count bucket length
        hm = hashed_cmds[i as usize];
        while !hm.is_null() { hm = (*hm).next; len += 1 }
        if len == 0 as libc::c_int {
            empty += 1
        } else {
            if len < minsize { minsize = len }
            if len > maxsize { maxsize = len }
        }
        i += 1
    }
    Con_Printf(b"Base command stats:\n\x00" as *const u8 as
                   *const libc::c_char);
    Con_Printf(b"Bucket minimal length: %d\n\x00" as *const u8 as
                   *const libc::c_char, minsize);
    Con_Printf(b"Bucket maximum length: %d\n\x00" as *const u8 as
                   *const libc::c_char, maxsize);
    Con_Printf(b"Empty buckets: %d\n\x00" as *const u8 as *const libc::c_char,
               empty);
}
unsafe extern "C" fn BaseCmd_CheckCvars(mut key: *const libc::c_char,
                                        mut value: *const libc::c_char,
                                        mut buffer: *mut libc::c_void,
                                        mut ptr: *mut libc::c_void) {
    let mut v: *mut libc::c_void = BaseCmd_Find(HM_CVAR, key);
    let mut invalid: *mut qboolean = ptr as *mut qboolean;
    if v.is_null() {
        Con_Printf(b"Cvar %s is missing in basecmd\n\x00" as *const u8 as
                       *const libc::c_char, key);
        *invalid = true_0
    };
}
/*
============
BaseCmd_Stats_f

testing order matches cbuf execute
============
*/
#[no_mangle]
pub unsafe extern "C" fn BaseCmd_Test_f() {
    let mut cmd: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut invalid: qboolean = false_0;
    // Cmd_LookupCmds don't allows to check alias, so just iterate
    a = Cmd_AliasGetList();
    while !a.is_null() {
        let mut v: *mut libc::c_void =
            BaseCmd_Find(HM_CMDALIAS, (*a).name.as_mut_ptr());
        if v.is_null() {
            Con_Printf(b"Alias %s is missing in basecmd\n\x00" as *const u8 as
                           *const libc::c_char, (*a).name.as_mut_ptr());
            invalid = true_0
        }
        a = (*a).next
    }
    cmd = Cmd_GetFirstFunctionHandle() as *mut libc::c_void;
    while !cmd.is_null() {
        let mut v_0: *mut libc::c_void =
            BaseCmd_Find(HM_CMD, Cmd_GetName(cmd as *mut cmd_s));
        if v_0.is_null() {
            Con_Printf(b"Command %s is missing in basecmd\n\x00" as *const u8
                           as *const libc::c_char,
                       Cmd_GetName(cmd as *mut cmd_s));
            invalid = true_0
        }
        cmd =
            Cmd_GetNextFunctionHandle(cmd as *mut cmd_s) as *mut libc::c_void
    }
    Cvar_LookupVars(0 as libc::c_int, 0 as *mut libc::c_void,
                    &mut invalid as *mut qboolean as *mut libc::c_void,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *const libc::c_char,
                                                                        _:
                                                                            *const libc::c_char,
                                                                        _:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> ()>,
                                            setpair_t>(Some(BaseCmd_CheckCvars
                                                                as
                                                                unsafe extern "C" fn(_:
                                                                                         *const libc::c_char,
                                                                                     _:
                                                                                         *const libc::c_char,
                                                                                     _:
                                                                                         *mut libc::c_void,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    -> ())));
    if invalid as u64 == 0 {
        Con_Printf(b"BaseCmd is valid\n\x00" as *const u8 as
                       *const libc::c_char);
    };
}
