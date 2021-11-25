#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type SDL_Window;
    pub type _SDL_GameController;
    pub type SDL_SysWMmsg;
    pub type grasshdr_s;
    #[no_mangle]
    fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_ClearError();
    #[no_mangle]
    fn SDL_GetWindowID(window: *mut SDL_Window) -> Uint32;
    #[no_mangle]
    fn SDL_GetModState() -> SDL_Keymod;
    #[no_mangle]
    fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const libc::c_char;
    #[no_mangle]
    fn SDL_IsTextInputActive() -> SDL_bool;
    #[no_mangle]
    fn SDL_GetRelativeMouseState(x: *mut libc::c_int, y: *mut libc::c_int)
     -> Uint32;
    #[no_mangle]
    fn SDL_ShowCursor(toggle: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_GameControllerOpen(joystick_index: libc::c_int)
     -> *mut SDL_GameController;
    #[no_mangle]
    fn SDL_GameControllerFromInstanceID(joyid: SDL_JoystickID)
     -> *mut SDL_GameController;
    #[no_mangle]
    fn SDL_GameControllerName(gamecontroller: *mut SDL_GameController)
     -> *const libc::c_char;
    #[no_mangle]
    fn SDL_GameControllerGetVendor(gamecontroller: *mut SDL_GameController)
     -> Uint16;
    #[no_mangle]
    fn SDL_GameControllerGetProduct(gamecontroller: *mut SDL_GameController)
     -> Uint16;
    #[no_mangle]
    fn SDL_GameControllerGetProductVersion(gamecontroller:
                                               *mut SDL_GameController)
     -> Uint16;
    #[no_mangle]
    fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SNDDMA_Activate(active: qboolean);
    #[no_mangle]
    fn VID_SetMode() -> qboolean;
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_CharEvent(key: libc::c_int);
    #[no_mangle]
    fn Sys_Quit() -> !;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut in_mouseinitialized: qboolean;
    #[no_mangle]
    fn IN_MouseEvent();
    #[no_mangle]
    fn IN_ActivateMouse(force: qboolean);
    #[no_mangle]
    fn IN_DeactivateMouse();
    #[no_mangle]
    static mut touch_emulate: *mut convar_t;
    #[no_mangle]
    fn IN_TouchEvent(type_0: touchEventType, fingerID: libc::c_int,
                     x: libc::c_float, y: libc::c_float, dx: libc::c_float,
                     dy: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn Touch_KeyEvent(key: libc::c_int, down: libc::c_int);
    #[no_mangle]
    fn Joy_IsActive() -> qboolean;
    #[no_mangle]
    fn Joy_HatMotionEvent(hat: byte, value: byte);
    #[no_mangle]
    fn Joy_AxisMotionEvent(axis: byte, value: libc::c_short);
    #[no_mangle]
    fn Joy_KnownAxisMotionEvent(engineAxis: engineAxis_t,
                                value: libc::c_short);
    #[no_mangle]
    fn Joy_BallMotionEvent(ball: byte, xrel: libc::c_short,
                           yrel: libc::c_short);
    #[no_mangle]
    fn Joy_ButtonEvent(button: byte, down: byte);
    #[no_mangle]
    fn Joy_AddEvent();
    #[no_mangle]
    fn Joy_RemoveEvent();
    #[no_mangle]
    fn Key_Event(key: libc::c_int, down: libc::c_int);
    #[no_mangle]
    fn Con_UtfProcessCharForce(in_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut cl_charset: *mut convar_t;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    static mut m_ignore: *mut convar_t;
    #[no_mangle]
    fn VID_RestoreScreenResolution();
    #[no_mangle]
    fn VID_SaveWindowSize(width: libc::c_int, height: libc::c_int);
    #[no_mangle]
    static mut dma: dma_t;
    #[no_mangle]
    static mut snd_mute_losefocus: convar_t;
    #[no_mangle]
    static mut vid_fullscreen: *mut convar_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Sint32 = int32_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_WINDOWEVENT_HIT_TEST: C2RustUnnamed = 16;
pub const SDL_WINDOWEVENT_TAKE_FOCUS: C2RustUnnamed = 15;
pub const SDL_WINDOWEVENT_CLOSE: C2RustUnnamed = 14;
pub const SDL_WINDOWEVENT_FOCUS_LOST: C2RustUnnamed = 13;
pub const SDL_WINDOWEVENT_FOCUS_GAINED: C2RustUnnamed = 12;
pub const SDL_WINDOWEVENT_LEAVE: C2RustUnnamed = 11;
pub const SDL_WINDOWEVENT_ENTER: C2RustUnnamed = 10;
pub const SDL_WINDOWEVENT_RESTORED: C2RustUnnamed = 9;
pub const SDL_WINDOWEVENT_MAXIMIZED: C2RustUnnamed = 8;
pub const SDL_WINDOWEVENT_MINIMIZED: C2RustUnnamed = 7;
pub const SDL_WINDOWEVENT_SIZE_CHANGED: C2RustUnnamed = 6;
pub const SDL_WINDOWEVENT_RESIZED: C2RustUnnamed = 5;
pub const SDL_WINDOWEVENT_MOVED: C2RustUnnamed = 4;
pub const SDL_WINDOWEVENT_EXPOSED: C2RustUnnamed = 3;
pub const SDL_WINDOWEVENT_HIDDEN: C2RustUnnamed = 2;
pub const SDL_WINDOWEVENT_SHOWN: C2RustUnnamed = 1;
pub const SDL_WINDOWEVENT_NONE: C2RustUnnamed = 0;
pub type SDL_Scancode = libc::c_uint;
pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
pub const SDL_SCANCODE_END: SDL_Scancode = 77;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
pub const SDL_SCANCODE_0: SDL_Scancode = 39;
pub const SDL_SCANCODE_9: SDL_Scancode = 38;
pub const SDL_SCANCODE_8: SDL_Scancode = 37;
pub const SDL_SCANCODE_7: SDL_Scancode = 36;
pub const SDL_SCANCODE_6: SDL_Scancode = 35;
pub const SDL_SCANCODE_5: SDL_Scancode = 34;
pub const SDL_SCANCODE_4: SDL_Scancode = 33;
pub const SDL_SCANCODE_3: SDL_Scancode = 32;
pub const SDL_SCANCODE_2: SDL_Scancode = 31;
pub const SDL_SCANCODE_1: SDL_Scancode = 30;
pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
pub const SDL_SCANCODE_X: SDL_Scancode = 27;
pub const SDL_SCANCODE_W: SDL_Scancode = 26;
pub const SDL_SCANCODE_V: SDL_Scancode = 25;
pub const SDL_SCANCODE_U: SDL_Scancode = 24;
pub const SDL_SCANCODE_T: SDL_Scancode = 23;
pub const SDL_SCANCODE_S: SDL_Scancode = 22;
pub const SDL_SCANCODE_R: SDL_Scancode = 21;
pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
pub const SDL_SCANCODE_P: SDL_Scancode = 19;
pub const SDL_SCANCODE_O: SDL_Scancode = 18;
pub const SDL_SCANCODE_N: SDL_Scancode = 17;
pub const SDL_SCANCODE_M: SDL_Scancode = 16;
pub const SDL_SCANCODE_L: SDL_Scancode = 15;
pub const SDL_SCANCODE_K: SDL_Scancode = 14;
pub const SDL_SCANCODE_J: SDL_Scancode = 13;
pub const SDL_SCANCODE_I: SDL_Scancode = 12;
pub const SDL_SCANCODE_H: SDL_Scancode = 11;
pub const SDL_SCANCODE_G: SDL_Scancode = 10;
pub const SDL_SCANCODE_F: SDL_Scancode = 9;
pub const SDL_SCANCODE_E: SDL_Scancode = 8;
pub const SDL_SCANCODE_D: SDL_Scancode = 7;
pub const SDL_SCANCODE_C: SDL_Scancode = 6;
pub const SDL_SCANCODE_B: SDL_Scancode = 5;
pub const SDL_SCANCODE_A: SDL_Scancode = 4;
pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
pub type SDL_Keycode = Sint32;
pub type SDL_Keymod = libc::c_uint;
pub const KMOD_RESERVED: SDL_Keymod = 32768;
pub const KMOD_MODE: SDL_Keymod = 16384;
pub const KMOD_CAPS: SDL_Keymod = 8192;
pub const KMOD_NUM: SDL_Keymod = 4096;
pub const KMOD_RGUI: SDL_Keymod = 2048;
pub const KMOD_LGUI: SDL_Keymod = 1024;
pub const KMOD_RALT: SDL_Keymod = 512;
pub const KMOD_LALT: SDL_Keymod = 256;
pub const KMOD_RCTRL: SDL_Keymod = 128;
pub const KMOD_LCTRL: SDL_Keymod = 64;
pub const KMOD_RSHIFT: SDL_Keymod = 2;
pub const KMOD_LSHIFT: SDL_Keymod = 1;
pub const KMOD_NONE: SDL_Keymod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub mod_0: Uint16,
    pub unused: Uint32,
}
pub type SDL_JoystickID = Sint32;
pub type SDL_GameController = _SDL_GameController;
pub type C2RustUnnamed_0 = libc::c_int;
pub const SDL_CONTROLLER_AXIS_MAX: C2RustUnnamed_0 = 6;
pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: C2RustUnnamed_0 = 5;
pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: C2RustUnnamed_0 = 4;
pub const SDL_CONTROLLER_AXIS_RIGHTY: C2RustUnnamed_0 = 3;
pub const SDL_CONTROLLER_AXIS_RIGHTX: C2RustUnnamed_0 = 2;
pub const SDL_CONTROLLER_AXIS_LEFTY: C2RustUnnamed_0 = 1;
pub const SDL_CONTROLLER_AXIS_LEFTX: C2RustUnnamed_0 = 0;
pub const SDL_CONTROLLER_AXIS_INVALID: C2RustUnnamed_0 = -1;
pub type C2RustUnnamed_1 = libc::c_int;
pub const SDL_CONTROLLER_BUTTON_MAX: C2RustUnnamed_1 = 15;
pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: C2RustUnnamed_1 = 14;
pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: C2RustUnnamed_1 = 13;
pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: C2RustUnnamed_1 = 12;
pub const SDL_CONTROLLER_BUTTON_DPAD_UP: C2RustUnnamed_1 = 11;
pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: C2RustUnnamed_1 = 10;
pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: C2RustUnnamed_1 = 9;
pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: C2RustUnnamed_1 = 8;
pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: C2RustUnnamed_1 = 7;
pub const SDL_CONTROLLER_BUTTON_START: C2RustUnnamed_1 = 6;
pub const SDL_CONTROLLER_BUTTON_GUIDE: C2RustUnnamed_1 = 5;
pub const SDL_CONTROLLER_BUTTON_BACK: C2RustUnnamed_1 = 4;
pub const SDL_CONTROLLER_BUTTON_Y: C2RustUnnamed_1 = 3;
pub const SDL_CONTROLLER_BUTTON_X: C2RustUnnamed_1 = 2;
pub const SDL_CONTROLLER_BUTTON_B: C2RustUnnamed_1 = 1;
pub const SDL_CONTROLLER_BUTTON_A: C2RustUnnamed_1 = 0;
pub const SDL_CONTROLLER_BUTTON_INVALID: C2RustUnnamed_1 = -1;
pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;
pub type SDL_GestureID = Sint64;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SDL_LASTEVENT: C2RustUnnamed_2 = 65535;
pub const SDL_USEREVENT: C2RustUnnamed_2 = 32768;
pub const SDL_RENDER_DEVICE_RESET: C2RustUnnamed_2 = 8193;
pub const SDL_RENDER_TARGETS_RESET: C2RustUnnamed_2 = 8192;
pub const SDL_SENSORUPDATE: C2RustUnnamed_2 = 4608;
pub const SDL_AUDIODEVICEREMOVED: C2RustUnnamed_2 = 4353;
pub const SDL_AUDIODEVICEADDED: C2RustUnnamed_2 = 4352;
pub const SDL_DROPCOMPLETE: C2RustUnnamed_2 = 4099;
pub const SDL_DROPBEGIN: C2RustUnnamed_2 = 4098;
pub const SDL_DROPTEXT: C2RustUnnamed_2 = 4097;
pub const SDL_DROPFILE: C2RustUnnamed_2 = 4096;
pub const SDL_CLIPBOARDUPDATE: C2RustUnnamed_2 = 2304;
pub const SDL_MULTIGESTURE: C2RustUnnamed_2 = 2050;
pub const SDL_DOLLARRECORD: C2RustUnnamed_2 = 2049;
pub const SDL_DOLLARGESTURE: C2RustUnnamed_2 = 2048;
pub const SDL_FINGERMOTION: C2RustUnnamed_2 = 1794;
pub const SDL_FINGERUP: C2RustUnnamed_2 = 1793;
pub const SDL_FINGERDOWN: C2RustUnnamed_2 = 1792;
pub const SDL_CONTROLLERDEVICEREMAPPED: C2RustUnnamed_2 = 1621;
pub const SDL_CONTROLLERDEVICEREMOVED: C2RustUnnamed_2 = 1620;
pub const SDL_CONTROLLERDEVICEADDED: C2RustUnnamed_2 = 1619;
pub const SDL_CONTROLLERBUTTONUP: C2RustUnnamed_2 = 1618;
pub const SDL_CONTROLLERBUTTONDOWN: C2RustUnnamed_2 = 1617;
pub const SDL_CONTROLLERAXISMOTION: C2RustUnnamed_2 = 1616;
pub const SDL_JOYDEVICEREMOVED: C2RustUnnamed_2 = 1542;
pub const SDL_JOYDEVICEADDED: C2RustUnnamed_2 = 1541;
pub const SDL_JOYBUTTONUP: C2RustUnnamed_2 = 1540;
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed_2 = 1539;
pub const SDL_JOYHATMOTION: C2RustUnnamed_2 = 1538;
pub const SDL_JOYBALLMOTION: C2RustUnnamed_2 = 1537;
pub const SDL_JOYAXISMOTION: C2RustUnnamed_2 = 1536;
pub const SDL_MOUSEWHEEL: C2RustUnnamed_2 = 1027;
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed_2 = 1026;
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed_2 = 1025;
pub const SDL_MOUSEMOTION: C2RustUnnamed_2 = 1024;
pub const SDL_KEYMAPCHANGED: C2RustUnnamed_2 = 772;
pub const SDL_TEXTINPUT: C2RustUnnamed_2 = 771;
pub const SDL_TEXTEDITING: C2RustUnnamed_2 = 770;
pub const SDL_KEYUP: C2RustUnnamed_2 = 769;
pub const SDL_KEYDOWN: C2RustUnnamed_2 = 768;
pub const SDL_SYSWMEVENT: C2RustUnnamed_2 = 513;
pub const SDL_WINDOWEVENT: C2RustUnnamed_2 = 512;
pub const SDL_DISPLAYEVENT: C2RustUnnamed_2 = 336;
pub const SDL_APP_DIDENTERFOREGROUND: C2RustUnnamed_2 = 262;
pub const SDL_APP_WILLENTERFOREGROUND: C2RustUnnamed_2 = 261;
pub const SDL_APP_DIDENTERBACKGROUND: C2RustUnnamed_2 = 260;
pub const SDL_APP_WILLENTERBACKGROUND: C2RustUnnamed_2 = 259;
pub const SDL_APP_LOWMEMORY: C2RustUnnamed_2 = 258;
pub const SDL_APP_TERMINATING: C2RustUnnamed_2 = 257;
pub const SDL_QUIT: C2RustUnnamed_2 = 256;
pub const SDL_FIRSTEVENT: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_CommonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub display: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_WindowEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
    pub data2: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub state: Uint8,
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub keysym: SDL_Keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
    pub start: Sint32,
    pub length: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub state: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub xrel: Sint32,
    pub yrel: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub button: Uint8,
    pub state: Uint8,
    pub clicks: Uint8,
    pub padding1: Uint8,
    pub x: Sint32,
    pub y: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub direction: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub hat: Uint8,
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Uint32,
    pub iscapture: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub fingerId: SDL_FingerID,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub pressure: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MultiGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub dTheta: libc::c_float,
    pub dDist: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub numFingers: Uint16,
    pub padding: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: Uint32,
    pub error: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub file: *mut libc::c_char,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SensorEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub data: [libc::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub code: Sint32,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint32,
    pub common: SDL_CommonEvent,
    pub display: SDL_DisplayEvent,
    pub window: SDL_WindowEvent,
    pub key: SDL_KeyboardEvent,
    pub edit: SDL_TextEditingEvent,
    pub text: SDL_TextInputEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub wheel: SDL_MouseWheelEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub jdevice: SDL_JoyDeviceEvent,
    pub caxis: SDL_ControllerAxisEvent,
    pub cbutton: SDL_ControllerButtonEvent,
    pub cdevice: SDL_ControllerDeviceEvent,
    pub adevice: SDL_AudioDeviceEvent,
    pub sensor: SDL_SensorEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
    pub tfinger: SDL_TouchFingerEvent,
    pub mgesture: SDL_MultiGestureEvent,
    pub dgesture: SDL_DollarGestureEvent,
    pub drop: SDL_DropEvent,
    pub padding: [Uint8; 56],
}
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed_3 = 1;
pub const HOST_NORMAL: C2RustUnnamed_3 = 0;
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
pub type convar_t = convar_s;
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
pub type host_parm_t = host_parm_s;
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
pub type host_redirect_t = host_redirect_s;
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
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
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
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub format: snd_format_t,
    pub samples: libc::c_int,
    pub samplepos: libc::c_int,
    pub buffer: *mut byte,
    pub initialized: qboolean,
}
pub type snd_format_t = snd_format_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_format_s {
    pub speed: libc::c_uint,
    pub width: libc::c_uint,
    pub channels: libc::c_uint,
}
pub type engineAxis_t = engineAxis_e;
pub type engineAxis_e = libc::c_uint;
pub const JOY_AXIS_NULL: engineAxis_e = 6;
pub const JOY_AXIS_LT: engineAxis_e = 5;
pub const JOY_AXIS_RT: engineAxis_e = 4;
pub const JOY_AXIS_YAW: engineAxis_e = 3;
pub const JOY_AXIS_PITCH: engineAxis_e = 2;
pub const JOY_AXIS_FWD: engineAxis_e = 1;
pub const JOY_AXIS_SIDE: engineAxis_e = 0;
pub type touchEventType = libc::c_uint;
pub const event_motion: touchEventType = 2;
pub const event_up: touchEventType = 1;
pub const event_down: touchEventType = 0;
pub type ref_globals_t = ref_globals_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedface_t {
    pub surf: *mut msurface_t,
    pub cull: libc::c_int,
}
pub type msurface_t = msurface_s;
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
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
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
pub struct mclipnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
pub type mleaf_t = mleaf_s;
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
/*
events.c - SDL event system handlers
Copyright (C) 2015-2017 a1batross

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
unsafe extern "C" fn SDLash_IsInstanceIDAGameController(mut joyId:
                                                            SDL_JoystickID)
 -> qboolean {
    if !SDL_GameControllerFromInstanceID(joyId).is_null() { return true_0 }
    return false_0;
}
/*
=============
SDLash_KeyEvent

=============
*/
unsafe extern "C" fn SDLash_KeyEvent(mut key: SDL_KeyboardEvent) {
    let mut down: libc::c_int =
        (key.state as libc::c_int != 0 as libc::c_int) as libc::c_int;
    let mut keynum: libc::c_int = key.keysym.scancode as libc::c_int;
    let mut numLock: qboolean =
        (SDL_GetModState() as libc::c_uint &
             KMOD_NUM as libc::c_int as libc::c_uint) as qboolean;
    if SDL_IsTextInputActive() as libc::c_uint != 0 && down != 0 {
        if SDL_GetModState() as libc::c_uint &
               (KMOD_LCTRL as libc::c_int | KMOD_RCTRL as libc::c_int) as
                   libc::c_uint != 0 {
            if keynum >= SDL_SCANCODE_A as libc::c_int &&
                   keynum <= SDL_SCANCODE_Z as libc::c_int {
                keynum =
                    keynum - SDL_SCANCODE_A as libc::c_int + 1 as libc::c_int;
                CL_CharEvent(keynum);
            }
            return
        }
    }
    if keynum >= SDL_SCANCODE_A as libc::c_int &&
           keynum <= SDL_SCANCODE_Z as libc::c_int {
        keynum = keynum - SDL_SCANCODE_A as libc::c_int + 'a' as i32
    } else if keynum >= SDL_SCANCODE_1 as libc::c_int &&
                  keynum <= SDL_SCANCODE_9 as libc::c_int {
        keynum = keynum - SDL_SCANCODE_1 as libc::c_int + '1' as i32
    } else if keynum >= SDL_SCANCODE_F1 as libc::c_int &&
                  keynum <= SDL_SCANCODE_F12 as libc::c_int {
        keynum = keynum - SDL_SCANCODE_F1 as libc::c_int + 135 as libc::c_int
    } else {
        match keynum {
            53 => { keynum = '`' as i32 }
            39 => { keynum = '0' as i32 }
            49 => { keynum = '\\' as i32 }
            47 => { keynum = '[' as i32 }
            48 => { keynum = ']' as i32 }
            46 => { keynum = '=' as i32 }
            45 => { keynum = '-' as i32 }
            43 => { keynum = 9 as libc::c_int }
            40 => { keynum = 13 as libc::c_int }
            41 => { keynum = 27 as libc::c_int }
            44 => { keynum = 32 as libc::c_int }
            42 => { keynum = 127 as libc::c_int }
            82 => { keynum = 128 as libc::c_int }
            80 => { keynum = 130 as libc::c_int }
            81 => { keynum = 129 as libc::c_int }
            79 => { keynum = 131 as libc::c_int }
            226 | 230 => { keynum = 132 as libc::c_int }
            224 | 228 => { keynum = 133 as libc::c_int }
            225 | 229 => { keynum = 134 as libc::c_int }
            227 | 231 => { keynum = 177 as libc::c_int }
            73 => { keynum = 147 as libc::c_int }
            76 => { keynum = 148 as libc::c_int }
            78 => { keynum = 149 as libc::c_int }
            75 => { keynum = 150 as libc::c_int }
            74 => { keynum = 151 as libc::c_int }
            77 => { keynum = 152 as libc::c_int }
            89 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '1' as i32
                    } else { 166 as libc::c_int }
            }
            90 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '2' as i32
                    } else { 167 as libc::c_int }
            }
            91 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '3' as i32
                    } else { 168 as libc::c_int }
            }
            92 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '4' as i32
                    } else { 163 as libc::c_int }
            }
            93 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '5' as i32
                    } else { 164 as libc::c_int }
            }
            94 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '6' as i32
                    } else { 165 as libc::c_int }
            }
            95 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '7' as i32
                    } else { 160 as libc::c_int }
            }
            96 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '8' as i32
                    } else { 161 as libc::c_int }
            }
            97 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '9' as i32
                    } else { 162 as libc::c_int }
            }
            98 => {
                keynum =
                    if numLock as libc::c_uint != 0 {
                        '0' as i32
                    } else { 170 as libc::c_int }
            }
            99 => { keynum = 171 as libc::c_int }
            88 => { keynum = 169 as libc::c_int }
            87 => { keynum = 174 as libc::c_int }
            86 => { keynum = 173 as libc::c_int }
            84 => { keynum = 172 as libc::c_int }
            85 => { keynum = '*' as i32 }
            83 => { keynum = 176 as libc::c_int }
            57 => { keynum = 175 as libc::c_int }
            56 => { keynum = '/' as i32 }
            55 => { keynum = '.' as i32 }
            51 => { keynum = ';' as i32 }
            52 => { keynum = '\'' as i32 }
            54 => { keynum = ',' as i32 }
            70 => {
                host.force_draw_version = true_0;
                host.force_draw_version_time =
                    (host.realtime + 5.0f32 as libc::c_double) as
                        libc::c_float
            }
            101 => { keynum = 177 as libc::c_int }
            127 | 128 | 129 | 275 | 276 => {
                // don't console spam on known functional buttons, but not used in engine
                return
            }
            0 => {
                // SDL_VERSION_ATLEAST( 2, 0, 0 )
                if down != 0 {
                    Con_Reportf(b"SDLash_KeyEvent: Unknown scancode\n\x00" as
                                    *const u8 as *const libc::c_char);
                }
                return
            }
            _ => {
                if down != 0 {
                    Con_Reportf(b"SDLash_KeyEvent: Unknown key: %s = %i\n\x00"
                                    as *const u8 as *const libc::c_char,
                                SDL_GetScancodeName(keynum as SDL_Scancode),
                                keynum);
                }
                return
            }
        }
    }
    Key_Event(keynum, down);
}
unsafe extern "C" fn SDLash_MouseKey(mut key: libc::c_int,
                                     mut down: libc::c_int,
                                     mut istouch: libc::c_int) {
    if if !touch_emulate.is_null() && (*touch_emulate).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        Touch_KeyEvent(key, down);
    } else if in_mouseinitialized as libc::c_uint != 0 &&
                  (*m_ignore).value == 0. && istouch == 0 {
        Key_Event(key, down);
    };
}
/*
=============
SDLash_MouseEvent

=============
*/
unsafe extern "C" fn SDLash_MouseEvent(mut button: SDL_MouseButtonEvent) {
    let mut down: libc::c_int =
        (button.state as libc::c_int != 0 as libc::c_int) as libc::c_int;
    let mut istouch: qboolean = false_0;
    istouch =
        (button.which == -(1 as libc::c_int) as Uint32) as libc::c_int as
            qboolean;
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    match button.button as libc::c_int {
        1 => {
            SDLash_MouseKey(241 as libc::c_int, down, istouch as libc::c_int);
        }
        3 => {
            SDLash_MouseKey(243 as libc::c_int, down, istouch as libc::c_int);
        }
        2 => {
            SDLash_MouseKey(242 as libc::c_int, down, istouch as libc::c_int);
        }
        4 => {
            SDLash_MouseKey(244 as libc::c_int, down, istouch as libc::c_int);
        }
        5 => {
            SDLash_MouseKey(245 as libc::c_int, down, istouch as libc::c_int);
        }
        _ => {
            // ! SDL_VERSION_ATLEAST( 2, 0, 0 )
            Con_Printf(b"Unknown mouse button ID: %d\n\x00" as *const u8 as
                           *const libc::c_char, button.button as libc::c_int);
        }
    };
}
/*
=============
SDLash_InputEvent

=============
*/
unsafe extern "C" fn SDLash_InputEvent(mut input: SDL_TextInputEvent) {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    text = input.text.as_mut_ptr();
    while *text != 0 {
        let mut ch: libc::c_int = 0;
        if Q_strnicmp((*cl_charset).string,
                      b"utf-8\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            ch = *text as libc::c_uchar as libc::c_int
        } else {
            ch =
                Con_UtfProcessCharForce(*text as libc::c_uchar as libc::c_int)
        }
        if !(ch == 0) { CL_CharEvent(ch); }
        text = text.offset(1)
    };
}
// SDL_VERSION_AT_LEAST( 2, 0, 0 )
unsafe extern "C" fn SDLash_ActiveEvent(mut gain: libc::c_int) {
    if gain != 0 {
        host.status = HOST_FRAME; // private to input system
        IN_ActivateMouse(true_0);
        if dma.initialized as libc::c_uint != 0 &&
               snd_mute_losefocus.value != 0. {
            SNDDMA_Activate(true_0);
        }
        host.force_draw_version = true_0;
        host.force_draw_version_time =
            (host.realtime + 5.0f32 as libc::c_double) as libc::c_float;
        if (*vid_fullscreen).value != 0. { VID_SetMode(); }
    } else {
        host.status = HOST_NOFOCUS;
        IN_DeactivateMouse();
        if dma.initialized as libc::c_uint != 0 &&
               snd_mute_losefocus.value != 0. {
            SNDDMA_Activate(false_0);
        }
        host.force_draw_version = true_0;
        host.force_draw_version_time =
            (host.realtime + 2 as libc::c_int as libc::c_double) as
                libc::c_float;
        VID_RestoreScreenResolution();
    };
}
static mut num_open_game_controllers: size_t = 0 as libc::c_int as size_t;
unsafe extern "C" fn SDLash_GameController_Add(mut index: libc::c_int) {
    extern "C" {
        #[no_mangle]
        static mut joy_enable: *mut convar_t;
    }
    let mut controller: *mut SDL_GameController =
        0 as *mut SDL_GameController;
    if (*joy_enable).value == 0. { return }
    controller = SDL_GameControllerOpen(index);
    if controller.is_null() {
        Con_Reportf(b"Failed to open SDL GameController %d: %s\n\x00" as
                        *const u8 as *const libc::c_char, index,
                    SDL_GetError());
        SDL_ClearError();
        return
    }
    Con_Reportf(b"Added controller: %s (%i:%i:%i)\n\x00" as *const u8 as
                    *const libc::c_char, SDL_GameControllerName(controller),
                SDL_GameControllerGetVendor(controller) as libc::c_int,
                SDL_GameControllerGetProduct(controller) as libc::c_int,
                SDL_GameControllerGetProductVersion(controller) as
                    libc::c_int);
    // SDL_VERSION_ATLEAST( 2, 0, 6 )
    num_open_game_controllers = num_open_game_controllers.wrapping_add(1);
    if num_open_game_controllers == 1 as libc::c_int as libc::c_ulong {
        Joy_AddEvent();
    };
}
unsafe extern "C" fn SDLash_GameController_Remove(mut joystick_id:
                                                      SDL_JoystickID) {
    Con_Reportf(b"Removed controller %i\n\x00" as *const u8 as
                    *const libc::c_char, joystick_id);
    // `Joy_RemoveEvent` sets `joy_found` to `0`.
	// We only want to do this when all the game controllers have been removed.
    num_open_game_controllers = num_open_game_controllers.wrapping_sub(1);
    if num_open_game_controllers == 0 as libc::c_int as libc::c_ulong {
        Joy_RemoveEvent();
    };
}
/*
=============
SDLash_EventFilter

=============
*/
unsafe extern "C" fn SDLash_EventFilter(mut event: *mut SDL_Event) {
    let mut current_block_64: u64;
    match (*event).type_0 {
        1024 => {
            /* Mouse events */
            if host.mouse_visible as u64 == 0 &&
                   (*event).motion.which != -(1 as libc::c_int) as Uint32 {
                IN_MouseEvent();
            }
        }
        1026 | 1025 => { SDLash_MouseEvent((*event).button); }
        768 | 769 => {
            /* Keyboard events */
            SDLash_KeyEvent((*event).key);
        }
        1536 => {
            /* Joystick events */
            if SDLash_IsInstanceIDAGameController((*event).jaxis.which) as u64
                   == 0 {
                Joy_AxisMotionEvent((*event).jaxis.axis,
                                    (*event).jaxis.value);
            }
        }
        1537 => {
            if SDLash_IsInstanceIDAGameController((*event).jball.which) as u64
                   == 0 {
                Joy_BallMotionEvent((*event).jball.ball, (*event).jball.xrel,
                                    (*event).jball.yrel);
            }
        }
        1538 => {
            if SDLash_IsInstanceIDAGameController((*event).jhat.which) as u64
                   == 0 {
                Joy_HatMotionEvent((*event).jhat.hat, (*event).jhat.value);
            }
        }
        1539 | 1540 => {
            if SDLash_IsInstanceIDAGameController((*event).jbutton.which) as
                   u64 == 0 {
                Joy_ButtonEvent((*event).jbutton.button,
                                (*event).jbutton.state);
            }
        }
        256 => { Sys_Quit(); }
        1027 => {
            let mut wheelbutton: libc::c_int =
                if (*event).wheel.y < 0 as libc::c_int {
                    239 as libc::c_int
                } else { 240 as libc::c_int };
            Key_Event(wheelbutton, true_0 as libc::c_int);
            Key_Event(wheelbutton, false_0 as libc::c_int);
        }
        1792 | 1793 | 1794 => {
            /* Touch events */
            static mut scale: libc::c_int = 0 as libc::c_int;
            let mut type_0: touchEventType = event_down;
            let mut x: libc::c_float = 0.;
            let mut y: libc::c_float = 0.;
            let mut dx: libc::c_float = 0.;
            let mut dy: libc::c_float = 0.;
            if (*event).type_0 ==
                   SDL_FINGERDOWN as libc::c_int as libc::c_uint {
                type_0 = event_down;
                current_block_64 = 7205609094909031804;
            } else if (*event).type_0 ==
                          SDL_FINGERUP as libc::c_int as libc::c_uint {
                type_0 = event_up;
                current_block_64 = 7205609094909031804;
            } else if (*event).type_0 ==
                          SDL_FINGERMOTION as libc::c_int as libc::c_uint {
                type_0 = event_motion;
                current_block_64 = 7205609094909031804;
            } else { current_block_64 = 10261677128829721533; }
            match current_block_64 {
                10261677128829721533 => { }
                _ => {
                    /*
		SDL sends coordinates in [0..width],[0..height] values
		on some devices
		*/
                    if scale == 0 {
                        if (*event).tfinger.x >
                               0 as libc::c_int as libc::c_float &&
                               (*event).tfinger.y >
                                   0 as libc::c_int as libc::c_float {
                            if (*event).tfinger.x >
                                   2 as libc::c_int as libc::c_float &&
                                   (*event).tfinger.y >
                                       2 as libc::c_int as libc::c_float {
                                scale = 2 as libc::c_int;
                                Con_Reportf(b"SDL reports screen coordinates, workaround enabled!\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                            } else { scale = 1 as libc::c_int }
                        }
                    }
                    x = (*event).tfinger.x;
                    y = (*event).tfinger.y;
                    dx = (*event).tfinger.dx;
                    dy = (*event).tfinger.dy;
                    if scale == 2 as libc::c_int {
                        x /= refState.width as libc::c_float;
                        y /= refState.height as libc::c_float;
                        dx /= refState.width as libc::c_float;
                        dy /= refState.height as libc::c_float
                    }
                    IN_TouchEvent(type_0,
                                  (*event).tfinger.fingerId as libc::c_int, x,
                                  y, dx, dy);
                }
            }
        }
        771 => {
            /* IME */
            SDLash_InputEvent((*event).text);
        }
        1541 => { Joy_AddEvent(); }
        1542 => { Joy_RemoveEvent(); }
        1616 => {
            // Swap axis to follow default axis binding:
		// LeftX, LeftY, RightX, RightY, TriggerRight, TriggerLeft
            static mut sdlControllerAxisToEngine: [libc::c_int; 6] =
                [JOY_AXIS_SIDE as libc::c_int, JOY_AXIS_FWD as libc::c_int,
                 JOY_AXIS_PITCH as libc::c_int, JOY_AXIS_YAW as libc::c_int,
                 JOY_AXIS_LT as libc::c_int, JOY_AXIS_RT as libc::c_int];
            if Joy_IsActive() as libc::c_uint != 0 &&
                   (*event).caxis.axis as libc::c_int !=
                       SDL_CONTROLLER_AXIS_INVALID as libc::c_int as Uint8 as
                           libc::c_int {
                Joy_KnownAxisMotionEvent(sdlControllerAxisToEngine[(*event).caxis.axis
                                                                       as
                                                                       usize]
                                             as engineAxis_t,
                                         (*event).caxis.value);
            }
        }
        1617 | 1618 => {
            static mut sdlControllerButtonToEngine: [libc::c_int; 15] =
                [207 as libc::c_int, 208 as libc::c_int, 209 as libc::c_int,
                 210 as libc::c_int, 213 as libc::c_int, 214 as libc::c_int,
                 215 as libc::c_int, 216 as libc::c_int, 217 as libc::c_int,
                 211 as libc::c_int, 212 as libc::c_int, 222 as libc::c_int,
                 223 as libc::c_int, 224 as libc::c_int, 225 as libc::c_int];
            // TODO: Use joyinput funcs, for future multiple gamepads support
            if Joy_IsActive() as libc::c_uint != 0 &&
                   (*event).cbutton.button as libc::c_int !=
                       SDL_CONTROLLER_BUTTON_INVALID as libc::c_int as Uint8
                           as libc::c_int {
                Key_Event(sdlControllerButtonToEngine[(*event).cbutton.button
                                                          as usize],
                          (*event).cbutton.state as
                              libc::c_int); // no need to activate
            }
        }
        1619 => { SDLash_GameController_Add((*event).cdevice.which); }
        1620 => { SDLash_GameController_Remove((*event).cdevice.which); }
        512 => {
            if (*event).window.windowID !=
                   SDL_GetWindowID(host.hWnd as *mut SDL_Window) {
                return
            }
            if !(host.status as libc::c_uint ==
                     HOST_SHUTDOWN as libc::c_int as libc::c_uint ||
                     host.type_0 ==
                         HOST_DEDICATED as libc::c_int as libc::c_uint) {
                match (*event).window.event as libc::c_int {
                    4 => {
                        if (*vid_fullscreen).value == 0. {
                            Cvar_SetValue(b"_window_xpos\x00" as *const u8 as
                                              *const libc::c_char,
                                          (*event).window.data1 as
                                              libc::c_float);
                            Cvar_SetValue(b"_window_ypos\x00" as *const u8 as
                                              *const libc::c_char,
                                          (*event).window.data2 as
                                              libc::c_float);
                        }
                    }
                    7 => {
                        host.status = HOST_SLEEP;
                        VID_RestoreScreenResolution();
                    }
                    9 => {
                        host.status = HOST_FRAME;
                        host.force_draw_version = true_0;
                        host.force_draw_version_time =
                            (host.realtime + 5.0f32 as libc::c_double) as
                                libc::c_float;
                        if (*vid_fullscreen).value != 0. { VID_SetMode(); }
                    }
                    12 => { SDLash_ActiveEvent(true_0 as libc::c_int); }
                    13 => { SDLash_ActiveEvent(false_0 as libc::c_int); }
                    5 => {
                        if !((*vid_fullscreen).value != 0.) {
                            VID_SaveWindowSize((*event).window.data1,
                                               (*event).window.data2);
                        }
                    }
                    _ => { }
                }
            }
        }
        _ => { }
    };
}
/*
=============
SDLash_RunEvents

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_RunEvents() {
    let mut event: SDL_Event = SDL_Event{type_0: 0,};
    while host.crashed as u64 == 0 && host.shutdown_issued as u64 == 0 &&
              SDL_PollEvent(&mut event) != 0 {
        SDLash_EventFilter(&mut event);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Platform_GetNativeObject(mut name:
                                                      *const libc::c_char)
 -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
    // SDL don't have it
}
/*
========================
Platform_PreCreateMove

this should disable mouse look on client when m_ignore enabled
TODO: kill mouse in win32 clients too
========================
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_PreCreateMove() {
    if if !m_ignore.is_null() && (*m_ignore).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        SDL_GetRelativeMouseState(0 as *mut libc::c_int,
                                  0 as *mut libc::c_int);
        SDL_ShowCursor(SDL_TRUE as libc::c_int);
    };
}
//  defined( XASH_SDL ) && !XASH_DEDICATED
