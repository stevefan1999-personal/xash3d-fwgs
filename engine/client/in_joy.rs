#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn CL_IsInConsole() -> qboolean;
    #[no_mangle]
    fn CL_IsInMenu() -> qboolean;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Platform_JoyInit(numjoy: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Key_KeynumToString(keynum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Key_IsDown(keynum: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Key_Event(key: libc::c_int, down: libc::c_int);
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
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
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
pub type C2RustUnnamed = libc::c_uint;
pub const JOY_HAT_LEFTDOWN: C2RustUnnamed = 12;
pub const JOY_HAT_LEFTUP: C2RustUnnamed = 9;
pub const JOY_HAT_RIGHTDOWN: C2RustUnnamed = 6;
pub const JOY_HAT_RIGHTUP: C2RustUnnamed = 3;
pub const JOY_HAT_LEFT: C2RustUnnamed = 8;
pub const JOY_HAT_DOWN: C2RustUnnamed = 4;
pub const JOY_HAT_RIGHT: C2RustUnnamed = 2;
pub const JOY_HAT_UP: C2RustUnnamed = 1;
pub const JOY_HAT_CENTERED: C2RustUnnamed = 0;
pub type engineAxis_e = libc::c_uint;
pub const JOY_AXIS_NULL: engineAxis_e = 6;
pub const JOY_AXIS_LT: engineAxis_e = 5;
pub const JOY_AXIS_RT: engineAxis_e = 4;
pub const JOY_AXIS_YAW: engineAxis_e = 3;
pub const JOY_AXIS_PITCH: engineAxis_e = 2;
pub const JOY_AXIS_FWD: engineAxis_e = 1;
pub const JOY_AXIS_SIDE: engineAxis_e = 0;
pub type engineAxis_t = engineAxis_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub mask: libc::c_int,
    pub key: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct joy_axis_s {
    pub val: libc::c_short,
    pub prevval: libc::c_short,
}
// index - axis num come from event
// value - inner axis
static mut joyaxesmap: [engineAxis_t; 6] =
    [JOY_AXIS_SIDE, JOY_AXIS_FWD, JOY_AXIS_PITCH, JOY_AXIS_YAW, JOY_AXIS_RT,
     JOY_AXIS_LT];
static mut joyaxis: [joy_axis_s; 6] =
    [{
         let mut init =
             joy_axis_s{val: 0 as libc::c_int as libc::c_short, prevval: 0,};
         init
     }, joy_axis_s{val: 0, prevval: 0,}, joy_axis_s{val: 0, prevval: 0,},
     joy_axis_s{val: 0, prevval: 0,}, joy_axis_s{val: 0, prevval: 0,},
     joy_axis_s{val: 0, prevval: 0,}];
// add posibility to remap keys, to place it in joykeys[]
#[no_mangle]
pub static mut joy_enable: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_pitch: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut joy_yaw: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut joy_forward: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut joy_side: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut joy_found: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut joy_index: *mut convar_t = 0 as *const convar_t as *mut convar_t;
static mut joy_lt_threshold: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_rt_threshold: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_side_deadzone: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_forward_deadzone: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_side_key_threshold: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_forward_key_threshold: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_pitch_deadzone: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_yaw_deadzone: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut joy_axis_binding: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
/*
============
Joy_IsActive
============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_IsActive() -> qboolean {
    return ((*joy_found).value != 0. && (*joy_enable).value != 0.) as
               libc::c_int as qboolean;
}
/*
============
Joy_HatMotionEvent

DPad events
============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_HatMotionEvent(mut hat: byte, mut value: byte) {
    let mut keys: [C2RustUnnamed_0; 4] =
        [{
             let mut init =
                 C2RustUnnamed_0{mask: JOY_HAT_UP as libc::c_int,
                                 key: 128 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{mask: JOY_HAT_DOWN as libc::c_int,
                                 key: 129 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{mask: JOY_HAT_LEFT as libc::c_int,
                                 key: 130 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{mask: JOY_HAT_RIGHT as libc::c_int,
                                 key: 131 as libc::c_int,};
             init
         }];
    let mut i: libc::c_int = 0;
    if (*joy_found).value == 0. { return }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[C2RustUnnamed_0; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>()
                                                   as libc::c_ulong) {
        if value as libc::c_int & keys[i as usize].mask != 0 {
            if Key_IsDown(keys[i as usize].key) == 0 {
                Key_Event(keys[i as usize].key, true_0 as libc::c_int);
            }
        } else if Key_IsDown(keys[i as usize].key) != 0 {
            Key_Event(keys[i as usize].key, false_0 as libc::c_int);
        }
        i += 1
    };
}
/*
=============
Joy_ProcessTrigger
=============
*/
unsafe extern "C" fn Joy_ProcessTrigger(engineAxis: engineAxis_t,
                                        mut value: libc::c_short) {
    let mut trigButton: libc::c_int = 0 as libc::c_int;
    let mut trigThreshold: libc::c_int = 0 as libc::c_int;
    match engineAxis as libc::c_uint {
        4 => {
            trigButton = 204 as libc::c_int;
            trigThreshold = (*joy_rt_threshold).value as libc::c_int
        }
        5 => {
            trigButton = 203 as libc::c_int;
            trigThreshold = (*joy_lt_threshold).value as libc::c_int
        }
        _ => {
            Con_Reportf(b"^1Error:^7 Joy_ProcessTrigger: invalid axis = %i\x00"
                            as *const u8 as *const libc::c_char,
                        engineAxis as libc::c_uint);
        }
    }
    // update axis values
    joyaxis[engineAxis as usize].prevval = joyaxis[engineAxis as usize].val;
    joyaxis[engineAxis as usize].val = value;
    if joyaxis[engineAxis as usize].val as libc::c_int > trigThreshold &&
           joyaxis[engineAxis as usize].prevval as libc::c_int <=
               trigThreshold {
        // ignore random press
        Key_Event(trigButton, true_0 as libc::c_int);
    } else if (joyaxis[engineAxis as usize].val as libc::c_int) <
                  trigThreshold &&
                  joyaxis[engineAxis as usize].prevval as libc::c_int >=
                      trigThreshold {
        // we're unpressing (inverted)
        Key_Event(trigButton,
                  false_0 as
                      libc::c_int); // only fwd/side axes can emit key events
    };
}
unsafe extern "C" fn Joy_GetHatValueForAxis(engineAxis: engineAxis_t)
 -> libc::c_int {
    let mut threshold: libc::c_int = 0;
    let mut negative: libc::c_int = 0;
    let mut positive: libc::c_int = 0;
    match engineAxis as libc::c_uint {
        0 => {
            threshold = (*joy_side_key_threshold).value as libc::c_int;
            negative = JOY_HAT_LEFT as libc::c_int;
            positive = JOY_HAT_RIGHT as libc::c_int
        }
        1 => {
            threshold = (*joy_side_key_threshold).value as libc::c_int;
            negative = JOY_HAT_UP as libc::c_int;
            positive = JOY_HAT_DOWN as libc::c_int
        }
        _ => {
            if false_0 as libc::c_int == 0 {
                Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/client/in_joy.c\x00" as *const u8 as
                              *const libc::c_char, 171 as libc::c_int);
            }
            return 0 as libc::c_int
        }
    }
    // similar code in Joy_ProcessTrigger
    if joyaxis[engineAxis as usize].val as libc::c_int > threshold &&
           joyaxis[engineAxis as usize].prevval as libc::c_int <= threshold {
        // ignore random press
        return positive
    }
    if (joyaxis[engineAxis as usize].val as libc::c_int) < -threshold &&
           joyaxis[engineAxis as usize].prevval as libc::c_int >= -threshold {
        // we're unpressing (inverted)
        return negative
    }
    return 0 as libc::c_int;
}
/*
=============
Joy_ProcessStick
=============
*/
unsafe extern "C" fn Joy_ProcessStick(engineAxis: engineAxis_t,
                                      mut value: libc::c_short) {
    let mut deadzone: libc::c_int =
        0 as
            libc::c_int; // caught new event in deadzone, fill it with zero(no motion)
    match engineAxis as libc::c_uint {
        1 => { deadzone = (*joy_forward_deadzone).value as libc::c_int }
        0 => { deadzone = (*joy_side_deadzone).value as libc::c_int }
        2 => { deadzone = (*joy_pitch_deadzone).value as libc::c_int }
        3 => { deadzone = (*joy_yaw_deadzone).value as libc::c_int }
        _ => {
            Con_Reportf(b"^1Error:^7 Joy_ProcessStick: invalid axis = %i\x00"
                            as *const u8 as *const libc::c_char,
                        engineAxis as libc::c_uint);
        }
    }
    if (value as libc::c_int) < deadzone && value as libc::c_int > -deadzone {
        value = 0 as libc::c_int as libc::c_short
    }
    // update axis values
    joyaxis[engineAxis as usize].prevval = joyaxis[engineAxis as usize].val;
    joyaxis[engineAxis as usize].val = value;
    // fwd/side axis simulate hat movement
    if (engineAxis as libc::c_uint ==
            JOY_AXIS_SIDE as libc::c_int as libc::c_uint ||
            engineAxis as libc::c_uint ==
                JOY_AXIS_FWD as libc::c_int as libc::c_uint) &&
           (CL_IsInMenu() as libc::c_uint != 0 ||
                CL_IsInConsole() as libc::c_uint != 0) {
        let mut val: libc::c_int = 0 as libc::c_int;
        val |= Joy_GetHatValueForAxis(JOY_AXIS_SIDE);
        val |= Joy_GetHatValueForAxis(JOY_AXIS_FWD);
        Joy_HatMotionEvent(0 as libc::c_int as byte, val as byte);
    };
}
/*
=============
Joy_AxisMotionEvent

Axis events
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_AxisMotionEvent(mut axis: byte,
                                             mut value: libc::c_short) {
    if (*joy_found).value == 0. { return } // it is not an update
    if axis as libc::c_int >= JOY_AXIS_NULL as libc::c_int {
        Con_Reportf(b"Only 6 axes is supported\n\x00" as *const u8 as
                        *const libc::c_char);
        return
    }
    return Joy_KnownAxisMotionEvent(joyaxesmap[axis as usize], value);
}
#[no_mangle]
pub unsafe extern "C" fn Joy_KnownAxisMotionEvent(mut engineAxis:
                                                      engineAxis_t,
                                                  mut value: libc::c_short) {
    if engineAxis as libc::c_uint ==
           JOY_AXIS_NULL as libc::c_int as libc::c_uint {
        return
    }
    if value as libc::c_int == joyaxis[engineAxis as usize].val as libc::c_int
       {
        return
    }
    if engineAxis as libc::c_uint >=
           JOY_AXIS_RT as libc::c_int as libc::c_uint {
        Joy_ProcessTrigger(engineAxis, value);
    } else { Joy_ProcessStick(engineAxis, value); };
}
/*
=============
Joy_BallMotionEvent

Trackball events. UNDONE
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_BallMotionEvent(mut ball: byte,
                                             mut xrel: libc::c_short,
                                             mut yrel: libc::c_short) {
    //if( !joy_found->value )
	//	return;
}
/*
=============
Joy_ButtonEvent

Button events
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_ButtonEvent(mut button: byte, mut down: byte) {
    if (*joy_found).value == 0. { return }
    // generic game button code.
    if button as libc::c_int > 32 as libc::c_int {
        let mut origbutton: libc::c_int = button as libc::c_int;
        button =
            ((button as libc::c_int & 31 as libc::c_int) + 207 as libc::c_int)
                as byte;
        Con_Reportf(b"Only 32 joybuttons is supported, converting %i button ID to %s\n\x00"
                        as *const u8 as *const libc::c_char, origbutton,
                    Key_KeynumToString(button as libc::c_int));
    } else { button = (button as libc::c_int + 207 as libc::c_int) as byte }
    Key_Event(button as libc::c_int, down as libc::c_int);
}
/*
=============
Joy_RemoveEvent

Called when joystick is removed. For future expansion
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_RemoveEvent() {
    if (*joy_found).value != 0. {
        Cvar_FullSet(b"joy_found\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
    };
}
/*
=============
Joy_RemoveEvent

Called when joystick is removed. For future expansion
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_AddEvent() {
    if (*joy_enable).value != 0. && (*joy_found).value == 0. {
        Cvar_FullSet(b"joy_found\x00" as *const u8 as *const libc::c_char,
                     b"1\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
    };
}
/*
=============
Joy_FinalizeMove

Append movement from axis. Called everyframe
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_FinalizeMove(mut fw: *mut libc::c_float,
                                          mut side: *mut libc::c_float,
                                          mut dpitch: *mut libc::c_float,
                                          mut dyaw: *mut libc::c_float) {
    if Joy_IsActive() as u64 == 0 { return } // must be form -1.0 to 1.0
    if (*joy_axis_binding).flags & (1 as libc::c_int) << 13 as libc::c_int !=
           0 {
        let mut bind: *const libc::c_char = (*joy_axis_binding).string;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while *bind.offset(i as isize) != 0 {
            match *bind.offset(i as isize) as libc::c_int {
                115 => { joyaxesmap[i as usize] = JOY_AXIS_SIDE }
                102 => { joyaxesmap[i as usize] = JOY_AXIS_FWD }
                121 => { joyaxesmap[i as usize] = JOY_AXIS_YAW }
                112 => { joyaxesmap[i as usize] = JOY_AXIS_PITCH }
                114 => { joyaxesmap[i as usize] = JOY_AXIS_RT }
                108 => { joyaxesmap[i as usize] = JOY_AXIS_LT }
                _ => { joyaxesmap[i as usize] = JOY_AXIS_NULL }
            }
            i = i.wrapping_add(1)
        }
        (*joy_axis_binding).flags =
            (*joy_axis_binding).flags &
                !((1 as libc::c_int) << 13 as libc::c_int)
    }
    *fw -=
        (*joy_forward).value *
            joyaxis[JOY_AXIS_FWD as libc::c_int as usize].val as libc::c_float
            / 32767 as libc::c_int as libc::c_float;
    *side +=
        (*joy_side).value *
            joyaxis[JOY_AXIS_SIDE as libc::c_int as usize].val as
                libc::c_float / 32767 as libc::c_int as libc::c_float;
    // HACKHACK: SDL have inverted look axis.
    *dpitch =
        (*dpitch as libc::c_double -
             ((*joy_pitch).value *
                  joyaxis[JOY_AXIS_PITCH as libc::c_int as usize].val as
                      libc::c_float / 32767 as libc::c_int as libc::c_float)
                 as libc::c_double * host.realframetime) as libc::c_float;
    *dyaw =
        (*dyaw as libc::c_double +
             ((*joy_yaw).value *
                  joyaxis[JOY_AXIS_YAW as libc::c_int as usize].val as
                      libc::c_float / 32767 as libc::c_int as libc::c_float)
                 as libc::c_double * host.realframetime) as libc::c_float;
}
/*
=============
Joy_Init

Main init procedure
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_Init() {
    joy_pitch =
        Cvar_Get(b"joy_pitch\x00" as *const u8 as *const libc::c_char,
                 b"100.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"joystick pitch sensitivity\x00" as *const u8 as
                     *const libc::c_char);
    joy_yaw =
        Cvar_Get(b"joy_yaw\x00" as *const u8 as *const libc::c_char,
                 b"100.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"joystick yaw sensitivity\x00" as *const u8 as
                     *const libc::c_char);
    joy_side =
        Cvar_Get(b"joy_side\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"joystick side sensitivity. Values from -1.0 to 1.0\x00" as
                     *const u8 as *const libc::c_char);
    joy_forward =
        Cvar_Get(b"joy_forward\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"joystick forward sensitivity. Values from -1.0 to 1.0\x00"
                     as *const u8 as *const libc::c_char);
    joy_lt_threshold =
        Cvar_Get(b"joy_lt_threshold\x00" as *const u8 as *const libc::c_char,
                 b"16384\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"left trigger threshold. Value from 0 to 32767\x00" as
                     *const u8 as *const libc::c_char);
    joy_rt_threshold =
        Cvar_Get(b"joy_rt_threshold\x00" as *const u8 as *const libc::c_char,
                 b"16384\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"right trigger threshold. Value from 0 to 32767\x00" as
                     *const u8 as *const libc::c_char);
    // emit a key event at 75% axis move
    joy_side_key_threshold =
        Cvar_Get(b"joy_side_key_threshold\x00" as *const u8 as
                     *const libc::c_char,
                 b"24576\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"side axis key event emit threshold. Value from 0 to 32767\x00"
                     as *const u8 as *const libc::c_char);
    joy_forward_key_threshold =
        Cvar_Get(b"joy_forward_key_threshold\x00" as *const u8 as
                     *const libc::c_char,
                 b"24576\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"forward axis key event emit threshold. Value from 0 to 32767\x00"
                     as *const u8 as *const libc::c_char);
    // by default, we rely on deadzone detection come from system, but some glitchy devices report false deadzones
    joy_side_deadzone =
        Cvar_Get(b"joy_side_deadzone\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"side axis deadzone. Value from 0 to 32767\x00" as *const u8
                     as *const libc::c_char);
    joy_forward_deadzone =
        Cvar_Get(b"joy_forward_deadzone\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"forward axis deadzone. Value from 0 to 32767\x00" as
                     *const u8 as *const libc::c_char);
    joy_pitch_deadzone =
        Cvar_Get(b"joy_pitch_deadzone\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"pitch axis deadzone. Value from 0 to 32767\x00" as
                     *const u8 as *const libc::c_char);
    joy_yaw_deadzone =
        Cvar_Get(b"joy_yaw_deadzone\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"yaw axis deadzone. Value from 0 to 32767\x00" as *const u8
                     as *const libc::c_char);
    joy_axis_binding =
        Cvar_Get(b"joy_axis_binding\x00" as *const u8 as *const libc::c_char,
                 b"sfpyrl\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"axis hardware id to engine inner axis binding, s - side, f - forward, y - yaw, p - pitch, r - left trigger, l - right trigger\x00"
                     as *const u8 as *const libc::c_char);
    joy_found =
        Cvar_Get(b"joy_found\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"is joystick is connected\x00" as *const u8 as
                     *const libc::c_char);
    // we doesn't loaded config.cfg yet, so this cvar is not archive.
	// change by +set joy_index in cmdline
    joy_index =
        Cvar_Get(b"joy_index\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"current active joystick\x00" as *const u8 as
                     *const libc::c_char);
    joy_enable =
        Cvar_Get(b"joy_enable\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"enable joystick\x00" as *const u8 as *const libc::c_char);
    if Sys_CheckParm(b"-nojoy\x00" as *const u8 as *const libc::c_char) != 0 {
        Cvar_FullSet(b"joy_enable\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
        return
    }
    Cvar_FullSet(b"joy_found\x00" as *const u8 as *const libc::c_char,
                 va(b"%d\x00" as *const u8 as *const libc::c_char,
                    Platform_JoyInit((*joy_index).value as libc::c_int)),
                 (1 as libc::c_int) << 17 as libc::c_int);
}
/*
===========
Joy_Shutdown

Shutdown joystick code
===========
*/
#[no_mangle]
pub unsafe extern "C" fn Joy_Shutdown() {
    Cvar_FullSet(b"joy_found\x00" as *const u8 as *const libc::c_char,
                 0 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
}
