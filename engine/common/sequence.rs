#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type decallist_s;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
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
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type fs_offset_t = off_t;
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
pub struct client_textmessage_s {
    pub effect: libc::c_int,
    pub r1: byte,
    pub g1: byte,
    pub b1: byte,
    pub a1: byte,
    pub r2: byte,
    pub g2: byte,
    pub b2: byte,
    pub a2: byte,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub fadein: libc::c_float,
    pub fadeout: libc::c_float,
    pub holdtime: libc::c_float,
    pub fxtime: libc::c_float,
    pub pName: *mut libc::c_char,
    pub pMessage: *mut libc::c_char,
}
pub type client_textmessage_t = client_textmessage_s;
pub type sequenceCommandEnum_ = libc::c_int;
pub const SEQUENCE_MODIFIER_TEXTCHANNEL: sequenceCommandEnum_ = 22;
pub const SEQUENCE_MODIFIER_LISTENER: sequenceCommandEnum_ = 21;
pub const SEQUENCE_MODIFIER_SPEAKER: sequenceCommandEnum_ = 20;
pub const SEQUENCE_MODIFIER_FXTIME: sequenceCommandEnum_ = 19;
pub const SEQUENCE_MODIFIER_HOLDTIME: sequenceCommandEnum_ = 18;
pub const SEQUENCE_MODIFIER_FADEOUT: sequenceCommandEnum_ = 17;
pub const SEQUENCE_MODIFIER_FADEIN: sequenceCommandEnum_ = 16;
pub const SEQUENCE_MODIFIER_COLOR2: sequenceCommandEnum_ = 15;
pub const SEQUENCE_MODIFIER_COLOR: sequenceCommandEnum_ = 14;
pub const SEQUENCE_MODIFIER_POSITION: sequenceCommandEnum_ = 13;
pub const SEQUENCE_MODIFIER_EFFECT: sequenceCommandEnum_ = 12;
pub const SEQUENCE_COMMAND_NOOP: sequenceCommandEnum_ = 11;
pub const SEQUENCE_COMMAND_POSTMODIFIER: sequenceCommandEnum_ = 10;
pub const SEQUENCE_COMMAND_MODIFIER: sequenceCommandEnum_ = 9;
pub const SEQUENCE_COMMAND_SETDEFAULTS: sequenceCommandEnum_ = 8;
pub const SEQUENCE_COMMAND_REPEAT: sequenceCommandEnum_ = 7;
pub const SEQUENCE_COMMAND_SENTENCE: sequenceCommandEnum_ = 6;
pub const SEQUENCE_COMMAND_GOSUB: sequenceCommandEnum_ = 5;
pub const SEQUENCE_COMMAND_SOUND: sequenceCommandEnum_ = 4;
pub const SEQUENCE_COMMAND_TEXT: sequenceCommandEnum_ = 3;
pub const SEQUENCE_COMMAND_KILLTARGETS: sequenceCommandEnum_ = 2;
pub const SEQUENCE_COMMAND_FIRETARGETS: sequenceCommandEnum_ = 1;
pub const SEQUENCE_COMMAND_PAUSE: sequenceCommandEnum_ = 0;
pub const SEQUENCE_COMMAND_ERROR: sequenceCommandEnum_ = -1;
pub type sequenceCommandEnum_e = sequenceCommandEnum_;
pub type sequenceModifierBits = libc::c_uint;
pub const SEQUENCE_MODIFIER_TEXTCHANNEL_BIT: sequenceModifierBits = 2048;
pub const SEQUENCE_MODIFIER_LISTENER_BIT: sequenceModifierBits = 1024;
pub const SEQUENCE_MODIFIER_SPEAKER_BIT: sequenceModifierBits = 512;
pub const SEQUENCE_MODIFIER_FXTIME_BIT: sequenceModifierBits = 256;
pub const SEQUENCE_MODIFIER_HOLDTIME_BIT: sequenceModifierBits = 128;
pub const SEQUENCE_MODIFIER_FADEOUT_BIT: sequenceModifierBits = 64;
pub const SEQUENCE_MODIFIER_FADEIN_BIT: sequenceModifierBits = 32;
pub const SEQUENCE_MODIFIER_COLOR2_BIT: sequenceModifierBits = 16;
pub const SEQUENCE_MODIFIER_COLOR_BIT: sequenceModifierBits = 8;
pub const SEQUENCE_MODIFIER_POSITION_BIT: sequenceModifierBits = 4;
pub const SEQUENCE_MODIFIER_EFFECT_BIT: sequenceModifierBits = 2;
pub type sequenceCommandType_ = libc::c_uint;
pub const SEQUENCE_TYPE_MODIFIER: sequenceCommandType_ = 1;
pub const SEQUENCE_TYPE_COMMAND: sequenceCommandType_ = 0;
pub type sequenceCommandType_e = sequenceCommandType_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequenceCommandMapping_ {
    pub commandEnum: sequenceCommandEnum_e,
    pub commandName: *const libc::c_char,
    pub commandType: sequenceCommandType_e,
}
pub type sequenceCommandMapping_s = sequenceCommandMapping_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequenceCommandLine_ {
    pub commandType: libc::c_int,
    pub clientMessage: client_textmessage_t,
    pub speakerName: *mut libc::c_char,
    pub listenerName: *mut libc::c_char,
    pub soundFileName: *mut libc::c_char,
    pub sentenceName: *mut libc::c_char,
    pub fireTargetNames: *mut libc::c_char,
    pub killTargetNames: *mut libc::c_char,
    pub delay: libc::c_float,
    pub repeatCount: libc::c_int,
    pub textChannel: libc::c_int,
    pub modifierBitField: libc::c_int,
    pub nextCommandLine: *mut sequenceCommandLine_s,
}
pub type sequenceCommandLine_s = sequenceCommandLine_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequenceEntry_ {
    pub fileName: *mut libc::c_char,
    pub entryName: *mut libc::c_char,
    pub firstCommand: *mut sequenceCommandLine_s,
    pub nextEntry: *mut sequenceEntry_s,
    pub isGlobal: qboolean,
}
pub type sequenceEntry_s = sequenceEntry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentenceEntry_ {
    pub data: *mut libc::c_char,
    pub nextEntry: *mut sentenceEntry_s,
    pub isGlobal: qboolean,
    pub index: libc::c_uint,
}
pub type sentenceEntry_s = sentenceEntry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentenceGroupEntry_ {
    pub groupName: *mut libc::c_char,
    pub numSentences: libc::c_uint,
    pub firstSentence: *mut sentenceEntry_s,
    pub nextEntry: *mut sentenceGroupEntry_s,
}
pub type sentenceGroupEntry_s = sentenceGroupEntry_;
/*
sequence.c - scripted sequences for CS:CZDS
Copyright (C) 2017 a1batross

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
pub static mut g_fileScopeDefaults: sequenceCommandLine_s =
    sequenceCommandLine_s{commandType: 0,
                          clientMessage:
                              client_textmessage_t{effect: 0,
                                                   r1: 0,
                                                   g1: 0,
                                                   b1: 0,
                                                   a1: 0,
                                                   r2: 0,
                                                   g2: 0,
                                                   b2: 0,
                                                   a2: 0,
                                                   x: 0.,
                                                   y: 0.,
                                                   fadein: 0.,
                                                   fadeout: 0.,
                                                   holdtime: 0.,
                                                   fxtime: 0.,
                                                   pName:
                                                       0 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                   pMessage:
                                                       0 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,},
                          speakerName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          listenerName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          soundFileName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          sentenceName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          fireTargetNames:
                              0 as *const libc::c_char as *mut libc::c_char,
                          killTargetNames:
                              0 as *const libc::c_char as *mut libc::c_char,
                          delay: 0.,
                          repeatCount: 0,
                          textChannel: 0,
                          modifierBitField: 0,
                          nextCommandLine:
                              0 as *const sequenceCommandLine_s as
                                  *mut sequenceCommandLine_s,};
#[no_mangle]
pub static mut g_blockScopeDefaults: sequenceCommandLine_s =
    sequenceCommandLine_s{commandType: 0,
                          clientMessage:
                              client_textmessage_t{effect: 0,
                                                   r1: 0,
                                                   g1: 0,
                                                   b1: 0,
                                                   a1: 0,
                                                   r2: 0,
                                                   g2: 0,
                                                   b2: 0,
                                                   a2: 0,
                                                   x: 0.,
                                                   y: 0.,
                                                   fadein: 0.,
                                                   fadeout: 0.,
                                                   holdtime: 0.,
                                                   fxtime: 0.,
                                                   pName:
                                                       0 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                   pMessage:
                                                       0 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,},
                          speakerName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          listenerName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          soundFileName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          sentenceName:
                              0 as *const libc::c_char as *mut libc::c_char,
                          fireTargetNames:
                              0 as *const libc::c_char as *mut libc::c_char,
                          killTargetNames:
                              0 as *const libc::c_char as *mut libc::c_char,
                          delay: 0.,
                          repeatCount: 0,
                          textChannel: 0,
                          modifierBitField: 0,
                          nextCommandLine:
                              0 as *const sequenceCommandLine_s as
                                  *mut sequenceCommandLine_s,};
#[no_mangle]
pub static mut g_sequenceList: *mut sequenceEntry_s =
    0 as *const sequenceEntry_s as *mut sequenceEntry_s;
#[no_mangle]
pub static mut g_sentenceGroupList: *mut sentenceGroupEntry_s =
    0 as *const sentenceGroupEntry_s as *mut sentenceGroupEntry_s;
#[no_mangle]
pub static mut g_sequenceParseFileIsGlobal: qboolean = false_0;
#[no_mangle]
pub static mut g_nonGlobalSentences: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut g_sequenceParseFileName: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut g_lineNum: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut g_scan: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut g_lineScan: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut g_sequenceCommandMappingTable: [sequenceCommandMapping_s; 23] =
    [{
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_PAUSE,
                                     commandName:
                                         b"pause\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_TEXT,
                                     commandName:
                                         b"text\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_SOUND,
                                     commandName:
                                         b"sound\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum:
                                         SEQUENCE_COMMAND_FIRETARGETS,
                                     commandName:
                                         b"firetargets\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum:
                                         SEQUENCE_COMMAND_KILLTARGETS,
                                     commandName:
                                         b"killtargets\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_GOSUB,
                                     commandName:
                                         b"gosub\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_SENTENCE,
                                     commandName:
                                         b"sentence\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_REPEAT,
                                     commandName:
                                         b"repeat\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum:
                                         SEQUENCE_COMMAND_SETDEFAULTS,
                                     commandName:
                                         b"setdefaults\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_MODIFIER,
                                     commandName:
                                         b"modifier\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum:
                                         SEQUENCE_COMMAND_POSTMODIFIER,
                                     commandName:
                                         b"postmodifier\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_COMMAND_NOOP,
                                     commandName:
                                         b"noop\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_COMMAND,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_EFFECT,
                                     commandName:
                                         b"effect\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_POSITION,
                                     commandName:
                                         b"position\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_COLOR,
                                     commandName:
                                         b"color\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_COLOR2,
                                     commandName:
                                         b"color2\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_FADEIN,
                                     commandName:
                                         b"fadein\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_FADEOUT,
                                     commandName:
                                         b"fadeout\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_HOLDTIME,
                                     commandName:
                                         b"holdtime\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_FXTIME,
                                     commandName:
                                         b"fxtime\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_SPEAKER,
                                     commandName:
                                         b"speaker\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum: SEQUENCE_MODIFIER_LISTENER,
                                     commandName:
                                         b"listener\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     },
     {
         let mut init =
             sequenceCommandMapping_{commandEnum:
                                         SEQUENCE_MODIFIER_TEXTCHANNEL,
                                     commandName:
                                         b"channel\x00" as *const u8 as
                                             *const libc::c_char,
                                     commandType: SEQUENCE_TYPE_MODIFIER,};
         init
     }];
/*
=============
Sequence_GetCommandEnumForName

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetCommandEnumForName(mut commandName:
                                                            *const libc::c_char,
                                                        mut type_0:
                                                            sequenceCommandType_e)
 -> sequenceCommandEnum_e {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[sequenceCommandMapping_s; 23]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<sequenceCommandMapping_s>()
                                                   as libc::c_ulong) {
        let mut mapping: *const sequenceCommandMapping_s =
            g_sequenceCommandMappingTable.as_ptr().offset(i as isize);
        if (*mapping).commandType as libc::c_uint == type_0 as libc::c_uint &&
               Q_strnicmp((*mapping).commandName, commandName,
                          99999 as libc::c_int) == 0 {
            return (*mapping).commandEnum
        }
        i += 1
    }
    return SEQUENCE_COMMAND_ERROR;
}
/*
=============
Sequence_ResetDefaults

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ResetDefaults(mut destination:
                                                    *mut sequenceCommandLine_s,
                                                mut source:
                                                    *mut sequenceCommandLine_s) {
    if source.is_null() {
        static mut defaultClientMessage: client_textmessage_t =
            {
                let mut init =
                    client_textmessage_s{effect: 0 as libc::c_int,
                                         r1: 255 as libc::c_int as byte,
                                         g1: 255 as libc::c_int as byte,
                                         b1: 255 as libc::c_int as byte,
                                         a1: 255 as libc::c_int as byte,
                                         r2: 255 as libc::c_int as byte,
                                         g2: 255 as libc::c_int as byte,
                                         b2: 255 as libc::c_int as byte,
                                         a2: 255 as libc::c_int as byte,
                                         x: 0.5f64 as libc::c_float,
                                         y: 0.5f64 as libc::c_float,
                                         fadein: 0.2f64 as libc::c_float,
                                         fadeout: 0.2f64 as libc::c_float,
                                         holdtime: 1.6f64 as libc::c_float,
                                         fxtime: 1.0f64 as libc::c_float,
                                         pName:
                                             0 as *const libc::c_char as
                                                 *mut libc::c_char,
                                         pMessage:
                                             0 as *const libc::c_char as
                                                 *mut libc::c_char,};
                init
            };
        (*destination).clientMessage = defaultClientMessage;
        (*destination).textChannel = 0 as libc::c_int;
        (*destination).delay = 0 as libc::c_int as libc::c_float;
        (*destination).repeatCount = 0 as libc::c_int;
        (*destination).nextCommandLine = 0 as *mut sequenceCommandLine_s;
        (*destination).soundFileName = 0 as *mut libc::c_char;
        (*destination).speakerName = 0 as *mut libc::c_char;
        (*destination).listenerName = 0 as *mut libc::c_char;
        return
    }
    (*destination).clientMessage = (*source).clientMessage;
    (*destination).clientMessage.pName = 0 as *mut libc::c_char;
    (*destination).clientMessage.pMessage = 0 as *mut libc::c_char;
    (*destination).textChannel = (*source).textChannel;
    (*destination).delay = (*source).delay;
    (*destination).repeatCount = (*source).repeatCount;
    (*destination).nextCommandLine = 0 as *mut sequenceCommandLine_s;
    (*destination).soundFileName = 0 as *mut libc::c_char;
    if !(*destination).speakerName.is_null() {
        _Mem_Free((*destination).speakerName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 121 as libc::c_int);
    }
    (*destination).speakerName =
        _copystring(host.mempool, (*source).speakerName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 122 as libc::c_int);
    if !(*destination).listenerName.is_null() {
        _Mem_Free((*destination).listenerName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 124 as libc::c_int);
    }
    (*destination).listenerName =
        _copystring(host.mempool, (*source).listenerName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 125 as libc::c_int);
}
/*
=============
Sequence_WriteDefaults

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_WriteDefaults(mut source:
                                                    *mut sequenceCommandLine_s,
                                                mut destination:
                                                    *mut sequenceCommandLine_s) {
    if destination.is_null() {
        Con_Reportf(b"^1Error:^7 Attempt to bake defaults into a non-existant command.\x00"
                        as *const u8 as *const libc::c_char);
    }
    if source.is_null() {
        Con_Reportf(b"^1Error:^7 Attempt to bake defaults from a non-existant command.\x00"
                        as *const u8 as *const libc::c_char);
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_EFFECT_BIT as libc::c_int != 0 {
        (*destination).clientMessage.effect = (*source).clientMessage.effect
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_POSITION_BIT as libc::c_int != 0 {
        (*destination).clientMessage.x = (*source).clientMessage.x;
        (*destination).clientMessage.y = (*source).clientMessage.y
    }
    if (*source).modifierBitField & SEQUENCE_MODIFIER_COLOR_BIT as libc::c_int
           != 0 {
        (*destination).clientMessage.r1 = (*source).clientMessage.r1;
        (*destination).clientMessage.g1 = (*source).clientMessage.g1;
        (*destination).clientMessage.b1 = (*source).clientMessage.b1;
        (*destination).clientMessage.a1 = (*source).clientMessage.a1
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_COLOR2_BIT as libc::c_int != 0 {
        (*destination).clientMessage.r2 = (*source).clientMessage.r2;
        (*destination).clientMessage.g2 = (*source).clientMessage.g2;
        (*destination).clientMessage.b2 = (*source).clientMessage.b2;
        (*destination).clientMessage.a2 = (*source).clientMessage.a2
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_FADEIN_BIT as libc::c_int != 0 {
        (*destination).clientMessage.fadein = (*source).clientMessage.fadein
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_FADEOUT_BIT as libc::c_int != 0 {
        (*destination).clientMessage.fadeout = (*source).clientMessage.fadeout
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_HOLDTIME_BIT as libc::c_int != 0 {
        (*destination).clientMessage.holdtime =
            (*source).clientMessage.holdtime
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_FXTIME_BIT as libc::c_int != 0 {
        (*destination).clientMessage.fxtime = (*source).clientMessage.fxtime
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_SPEAKER_BIT as libc::c_int != 0 {
        if !(*destination).speakerName.is_null() {
            _Mem_Free((*destination).speakerName as *mut libc::c_void,
                      b"../engine/common/sequence.c\x00" as *const u8 as
                          *const libc::c_char, 191 as libc::c_int);
        }
        (*destination).speakerName =
            _copystring(host.mempool, (*source).speakerName,
                        b"../engine/common/sequence.c\x00" as *const u8 as
                            *const libc::c_char, 192 as libc::c_int)
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_LISTENER_BIT as libc::c_int != 0 {
        if !(*destination).listenerName.is_null() {
            _Mem_Free((*destination).listenerName as *mut libc::c_void,
                      b"../engine/common/sequence.c\x00" as *const u8 as
                          *const libc::c_char, 197 as libc::c_int);
        }
        (*destination).listenerName =
            _copystring(host.mempool, (*source).listenerName,
                        b"../engine/common/sequence.c\x00" as *const u8 as
                            *const libc::c_char, 198 as libc::c_int)
    }
    if (*source).modifierBitField &
           SEQUENCE_MODIFIER_TEXTCHANNEL_BIT as libc::c_int != 0 {
        (*destination).textChannel = (*source).textChannel
    };
}
/*
=============
Sequence_BakeDefaults

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_BakeDefaults(mut destination:
                                                   *mut sequenceCommandLine_s,
                                               mut source:
                                                   *mut sequenceCommandLine_s) {
    let mut saveName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saveMessage: *mut libc::c_char = 0 as *mut libc::c_char;
    if destination.is_null() {
        Con_Reportf(b"^1Error:^7 Attempt to bake defaults into a non-existant command.\x00"
                        as *const u8 as *const libc::c_char);
    }
    if source.is_null() {
        Con_Reportf(b"^1Error:^7 Attempt to bake defaults from a non-existant command.\x00"
                        as *const u8 as *const libc::c_char);
    }
    saveName = (*destination).clientMessage.pName;
    saveMessage = (*destination).clientMessage.pMessage;
    (*destination).clientMessage = (*source).clientMessage;
    (*destination).clientMessage.pName = saveName;
    (*destination).clientMessage.pMessage = saveMessage;
    (*destination).textChannel = (*source).textChannel;
    if !(*destination).speakerName.is_null() {
        _Mem_Free((*destination).speakerName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 233 as libc::c_int);
    }
    (*destination).speakerName =
        _copystring(host.mempool, (*source).speakerName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 234 as libc::c_int);
    if !(*destination).listenerName.is_null() {
        _Mem_Free((*destination).listenerName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 236 as libc::c_int);
    }
    (*destination).listenerName =
        _copystring(host.mempool, (*source).listenerName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 237 as libc::c_int);
}
/*
=============
Sequence_SkipWhitespace

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_SkipWhitespace() -> qboolean {
    let mut newLine: qboolean = false_0;
    while *(*__ctype_b_loc()).offset(*g_scan as libc::c_int as isize) as
              libc::c_int &
              _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        if *g_scan as libc::c_int == '\n' as i32 {
            g_lineScan = g_scan.offset(1 as libc::c_int as isize);
            g_lineNum += 1;
            newLine = true_0
        }
        g_scan = g_scan.offset(1)
    }
    return newLine;
}
/*
=============
Sequence_IsNameValueChar

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_IsNameValueChar(mut ch: libc::c_char)
 -> qboolean {
    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int &
           _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return true_0
    }
    match ch as libc::c_int {
        46 | 45 | 95 | 47 | 92 => { return true_0 }
        _ => { }
    }
    return false_0;
}
/*
=============
Sequence_IsSymbol

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_IsSymbol(mut ch: libc::c_char) -> qboolean {
    match ch as libc::c_int {
        34 | 35 | 36 | 37 | 44 | 61 | 64 | 123 | 125 => { return true_0 }
        _ => { }
    }
    return false_0;
}
/*
=============
Sequence_GetNameValueString

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetNameValueString(mut token:
                                                         *mut libc::c_char,
                                                     mut len: size_t)
 -> size_t {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    Sequence_SkipWhitespace();
    if Sequence_IsNameValueChar(*g_scan) as u64 == 0 {
        if *g_scan as libc::c_int == '#' as i32 ||
               *g_scan as libc::c_int == '$' as i32 {
            Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: cannot have more than one \'%c\' per line; \'%c\' must be at the beginning of the line ONLY\n\x00"
                            as *const u8 as *const libc::c_char, g_lineNum,
                        g_sequenceParseFileName.as_mut_ptr(),
                        *g_scan as libc::c_int, *g_scan as libc::c_int);
        } else {
            Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: expected name/value, found illegal character \'%c\'\n\x00"
                            as *const u8 as *const libc::c_char, g_lineNum,
                        g_sequenceParseFileName.as_mut_ptr(),
                        *g_scan as libc::c_int);
        }
    }
    p = token;
    while Sequence_IsNameValueChar(*g_scan) as libc::c_uint != 0 && len != 0 {
        *p = *g_scan;
        p = p.offset(1);
        g_scan = g_scan.offset(1);
        len = len.wrapping_sub(1)
    }
    *p = 0 as libc::c_int as libc::c_char;
    return p.wrapping_offset_from(token) as libc::c_long as size_t;
}
/*
=============
Sequence_GetSymbol

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetSymbol() -> libc::c_char {
    let mut ch: libc::c_char = 0;
    Sequence_SkipWhitespace();
    ch = *g_scan;
    if ch != 0 { g_scan = g_scan.offset(1) }
    return ch;
}
/*
=============
Sequence_ValidateNameValueString

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ValidateNameValueString(mut token:
                                                              *mut libc::c_char) {
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    scan = token;
    while *scan != 0 {
        if Sequence_IsNameValueChar(*scan) as u64 == 0 {
            Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: name/value string \"%s\" had illegal character \'%c\'\n\x00"
                            as *const u8 as *const libc::c_char, g_lineNum,
                        g_sequenceParseFileName.as_mut_ptr(), token,
                        *scan as libc::c_int);
        }
        scan = scan.offset(1)
    };
}
/*
=============
Sequence_GetToken

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetToken(mut token: *mut libc::c_char,
                                           mut size: size_t) -> size_t {
    Sequence_SkipWhitespace();
    if Sequence_IsNameValueChar(*g_scan) as u64 != 0 {
        return Sequence_GetNameValueString(token, size)
    }
    if Sequence_IsSymbol(*g_scan) as u64 == 0 {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: expected token, found \'%c\' instead\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    *g_scan as libc::c_int);
    }
    let fresh0 = g_scan;
    g_scan = g_scan.offset(1);
    *token.offset(0 as libc::c_int as isize) = *fresh0;
    *token.offset(1 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    g_scan = g_scan.offset(1);
    return 1 as libc::c_int as size_t;
    // only one symbol has copied to token
}
/*
=============
Sequence_GetLine

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetLine(mut line: *mut libc::c_char,
                                          mut lineMaxLen: libc::c_int)
 -> size_t {
    let mut lineLen: libc::c_int = 0;
    let mut read: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut write: *mut libc::c_char = line;
    Sequence_SkipWhitespace();
    read = Q_strchr(g_scan, '\n' as i32 as libc::c_char);
    if read.is_null() {
        Con_Reportf(b"^1Error:^7 Syntax Error on line %d of %s.seq: expected sentence definition or \'}\', found End-Of-File!\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr());
    }
    lineLen =
        read.wrapping_offset_from(g_scan) as libc::c_long as libc::c_int;
    if lineLen >= lineMaxLen {
        Con_Reportf(b"^1Error:^7 Syntax Error on line %d of %s.seq: line was too long (was %d chars; max is %d chars)\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(), lineLen,
                    lineMaxLen - 1 as libc::c_int);
    }
    Q_strncpy(write, g_scan, lineLen as size_t);
    *write.offset(lineLen as isize) = 0 as libc::c_int as libc::c_char;
    g_scan = read;
    return lineLen as size_t;
}
/*
=============
Sequence_StripComments

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_StripComments(mut buffer: *mut libc::c_char,
                                                mut pBufSize:
                                                    *mut libc::c_int) {
    let mut eof: *mut libc::c_char = buffer.offset(*pBufSize as isize);
    let mut read: *mut libc::c_char = buffer;
    let mut write: *mut libc::c_char = buffer;
    while read < eof {
        if *read == 0 { break ; }
        if *read as libc::c_int == '/' as i32 {
            // skip one line comments //
            if *read.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '/' as i32 {
                read = read.offset(2 as libc::c_int as isize);
                while *read != 0 {
                    if *read as libc::c_int == '\n' as i32 { break ; }
                    if *read as libc::c_int == '\r' as i32 { break ; }
                    read = read.offset(1)
                }
                continue ;
            } else if *read.offset(1 as libc::c_int as isize) as libc::c_int
                          == '*' as i32 {
                read = read.offset(2 as libc::c_int as isize);
                while *read as libc::c_int != 0 &&
                          *read.offset(1 as libc::c_int as isize) as
                              libc::c_int != 0 {
                    if *read as libc::c_int == '*' as i32 &&
                           *read.offset(1 as libc::c_int as isize) as
                               libc::c_int == '/' as i32 {
                        read = read.offset(2 as libc::c_int as isize);
                        break ;
                    } else {
                        if *read as libc::c_int == '\n' as i32 ||
                               *read as libc::c_int == '\r' as i32 {
                            let fresh1 = write;
                            write = write.offset(1);
                            *fresh1 = *read
                        }
                        read = read.offset(1)
                    }
                }
                continue ;
            }
        }
        let fresh2 = read;
        read = read.offset(1);
        let fresh3 = write;
        write = write.offset(1);
        *fresh3 = *fresh2
    }
    *write = 0 as libc::c_int as libc::c_char;
}
// skip multiline /* */
/*
=============
Sequence_ReadInt

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ReadInt() -> libc::c_int {
    let mut str: [libc::c_char; 256] = [0; 256];
    Sequence_SkipWhitespace();
    Sequence_GetNameValueString(str.as_mut_ptr(),
                                256 as libc::c_int as size_t);
    return Q_atoi(str.as_mut_ptr());
}
/*
=============
Sequence_ReadFloat

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ReadFloat() -> libc::c_float {
    let mut str: [libc::c_char; 256] = [0; 256];
    Sequence_SkipWhitespace();
    Sequence_GetNameValueString(str.as_mut_ptr(),
                                256 as libc::c_int as size_t);
    return Q_atof(str.as_mut_ptr());
}
/*
=============
Sequence_ReadFloat

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ReadString(mut dest: *mut *mut libc::c_char,
                                             mut string: *mut libc::c_char,
                                             mut len: size_t) {
    Sequence_SkipWhitespace();
    Sequence_GetNameValueString(string, len);
    if !dest.is_null() {
        *dest =
            _copystring(host.mempool, string,
                        b"../engine/common/sequence.c\x00" as *const u8 as
                            *const libc::c_char, 546 as libc::c_int)
    };
}
/*
=============
Sequence_ReadQuotedString

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ReadQuotedString(mut dest:
                                                       *mut *mut libc::c_char,
                                                   mut str: *mut libc::c_char,
                                                   mut len: size_t) {
    let mut write: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_char = 0;
    Sequence_SkipWhitespace();
    ch = Sequence_GetSymbol();
    if ch as libc::c_int != '\"' as i32 {
        Con_Reportf(b"^1Error:^7 Parsing error on or before line %d of %s.seq: expected quote (\"), found \'%c\' instead\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(), ch as libc::c_int);
    }
    write = str;
    while *g_scan as libc::c_int != 0 && len != 0 {
        if *g_scan as libc::c_int == '\"' as i32 { break ; }
        if *g_scan as libc::c_int == '\n' as i32 { g_lineNum += 1 }
        *write = *g_scan;
        write = write.offset(1);
        g_scan = g_scan.offset(1);
        len = len.wrapping_sub(1)
    }
    *write = 0 as libc::c_int as libc::c_char;
    g_scan = g_scan.offset(1);
    if !dest.is_null() {
        *dest =
            _copystring(host.mempool, str,
                        b"../engine/common/sequence.c\x00" as *const u8 as
                            *const libc::c_char, 579 as libc::c_int)
    };
}
/*
=============
Sequence_ConfirmCarriageReturnOrSymbol

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ConfirmCarriageReturnOrSymbol(mut symbol:
                                                                    libc::c_char)
 -> qboolean {
    if Sequence_SkipWhitespace() as u64 != 0 { return true_0 }
    return (*g_scan as libc::c_int == symbol as libc::c_int) as libc::c_int as
               qboolean;
}
/*
=============
Sequence_IsCommandAModifier

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_IsCommandAModifier(mut commandEnum:
                                                         sequenceCommandEnum_e)
 -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[sequenceCommandMapping_s; 23]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<sequenceCommandMapping_s>()
                                                   as libc::c_ulong) {
        if g_sequenceCommandMappingTable[i as usize].commandEnum as
               libc::c_int == commandEnum as libc::c_int {
            return (g_sequenceCommandMappingTable[i as usize].commandType as
                        libc::c_uint ==
                        SEQUENCE_TYPE_MODIFIER as libc::c_int as libc::c_uint)
                       as libc::c_int as qboolean
        }
        i += 1
    }
    Con_Reportf(b"^1Error:^7 Internal error caused by line %d of %s.seq: unknown command enum = %d\n\x00"
                    as *const u8 as *const libc::c_char, g_lineNum,
                g_sequenceParseFileName.as_mut_ptr(),
                commandEnum as libc::c_int);
    return false_0;
}
/*
=============
Sequence_ReadCommandData

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ReadCommandData(mut commandEnum:
                                                      sequenceCommandEnum_e,
                                                  mut defaults:
                                                      *mut sequenceCommandLine_s) {
    let mut temp: [libc::c_char; 1024] = [0; 1024];
    if commandEnum as libc::c_int >= SEQUENCE_MODIFIER_EFFECT as libc::c_int
           &&
           commandEnum as libc::c_int <=
               SEQUENCE_MODIFIER_TEXTCHANNEL as libc::c_int {
        (*defaults).modifierBitField =
            ((*defaults).modifierBitField as libc::c_uint |
                 (1 as libc::c_uint) <<
                     SEQUENCE_MODIFIER_EFFECT as libc::c_int -
                         SEQUENCE_COMMAND_NOOP as libc::c_int) as libc::c_int
    }
    match commandEnum as libc::c_int {
        0 => { (*defaults).delay = Sequence_ReadFloat() }
        1 => {
            Sequence_ReadQuotedString(&mut (*defaults).fireTargetNames,
                                      temp.as_mut_ptr(),
                                      ::std::mem::size_of::<[libc::c_char; 1024]>()
                                          as libc::c_ulong);
        }
        2 => {
            Sequence_ReadQuotedString(&mut (*defaults).killTargetNames,
                                      temp.as_mut_ptr(),
                                      ::std::mem::size_of::<[libc::c_char; 1024]>()
                                          as libc::c_ulong);
        }
        3 => {
            Sequence_ReadQuotedString(&mut (*defaults).clientMessage.pMessage,
                                      temp.as_mut_ptr(),
                                      ::std::mem::size_of::<[libc::c_char; 1024]>()
                                          as libc::c_ulong);
        }
        4 => {
            Sequence_ReadString(&mut (*defaults).soundFileName,
                                temp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong);
        }
        5 => {
            Sequence_ReadString(&mut (*defaults).clientMessage.pName,
                                temp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong);
        }
        6 => {
            Sequence_ReadString(&mut (*defaults).sentenceName,
                                temp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong);
        }
        7 => { (*defaults).repeatCount = Sequence_ReadInt() }
        12 => { (*defaults).clientMessage.effect = Sequence_ReadInt() }
        13 => {
            (*defaults).clientMessage.x = Sequence_ReadFloat();
            (*defaults).clientMessage.y = Sequence_ReadFloat()
        }
        14 => {
            (*defaults).clientMessage.r1 = Sequence_ReadInt() as byte;
            (*defaults).clientMessage.g1 = Sequence_ReadInt() as byte;
            (*defaults).clientMessage.b1 = Sequence_ReadInt() as byte;
            (*defaults).clientMessage.a1 = 255 as libc::c_int as byte
        }
        15 => {
            (*defaults).clientMessage.r2 = Sequence_ReadInt() as byte;
            (*defaults).clientMessage.g2 = Sequence_ReadInt() as byte;
            (*defaults).clientMessage.b2 = Sequence_ReadInt() as byte;
            (*defaults).clientMessage.a2 = 255 as libc::c_int as byte
        }
        16 => { (*defaults).clientMessage.fadein = Sequence_ReadFloat() }
        17 => { (*defaults).clientMessage.fadeout = Sequence_ReadFloat() }
        18 => { (*defaults).clientMessage.holdtime = Sequence_ReadFloat() }
        19 => { (*defaults).clientMessage.fxtime = Sequence_ReadFloat() }
        20 => {
            Sequence_ReadString(&mut (*defaults).speakerName,
                                temp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong);
        }
        21 => {
            Sequence_ReadString(&mut (*defaults).listenerName,
                                temp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong);
        }
        22 => { (*defaults).textChannel = Sequence_ReadInt() }
        _ => {
            Con_Reportf(b"^1Error:^7 Internal error caused by line %d of %s.seq: unknown command enum = %d\n\x00"
                            as *const u8 as *const libc::c_char, g_lineNum,
                        g_sequenceParseFileName.as_mut_ptr(),
                        commandEnum as libc::c_int);
        }
    };
}
/*
=============
Sequence_ParseModifier

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseModifier(mut defaults:
                                                    *mut sequenceCommandLine_s)
 -> libc::c_char {
    let mut modifierName: [libc::c_char; 256] = [0; 256];
    let mut delimiter: libc::c_char = 0;
    let mut modifierEnum: sequenceCommandEnum_e = SEQUENCE_COMMAND_PAUSE;
    Sequence_GetNameValueString(modifierName.as_mut_ptr(),
                                256 as libc::c_int as size_t);
    modifierEnum =
        Sequence_GetCommandEnumForName(modifierName.as_mut_ptr(),
                                       SEQUENCE_TYPE_MODIFIER);
    if modifierEnum as libc::c_int == SEQUENCE_COMMAND_ERROR as libc::c_int {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: unknown modifier \"%s\"\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    modifierName.as_mut_ptr());
    }
    if Sequence_IsCommandAModifier(modifierEnum) as u64 == 0 {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: \"%s\" is a #command, not a $modifier\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    modifierName.as_mut_ptr());
    }
    delimiter = Sequence_GetSymbol();
    if delimiter as libc::c_int != '=' as i32 {
        Con_Reportf(b"^1Error:^7 Parsing error on or after line %d of %s.seq: after modifier \"%s\", expected \'=\', found \'%c\'\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    modifierName.as_mut_ptr(), delimiter as libc::c_int);
    }
    Sequence_ReadCommandData(modifierEnum, defaults);
    if Sequence_ConfirmCarriageReturnOrSymbol(',' as i32 as libc::c_char) as
           u64 == 0 {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: after value(s) for modifier \"%s\", expected \',\' or End-Of-Line; found \'%c\'\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    modifierName.as_mut_ptr(), *g_scan as libc::c_int);
    }
    return Sequence_GetSymbol();
}
/*
=============
Sequence_AddCommandLineToEntry

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_AddCommandLineToEntry(mut commandLine:
                                                            *mut sequenceCommandLine_s,
                                                        mut entry:
                                                            *mut sequenceEntry_s) {
    let mut scan: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    if !(*entry).firstCommand.is_null() {
        scan = (*entry).firstCommand;
        while !(*scan).nextCommandLine.is_null() {
            scan = (*scan).nextCommandLine
        }
        (*scan).nextCommandLine = commandLine
    } else { (*entry).firstCommand = commandLine }
    (*commandLine).nextCommandLine = 0 as *mut sequenceCommandLine_s;
}
/*
=============
Sequence_ParseModifierLine

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseModifierLine(mut entry:
                                                        *mut sequenceEntry_s,
                                                    mut modifierType:
                                                        sequenceCommandType_e)
 -> libc::c_char {
    let mut newCommandLine: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    let mut delimiter: libc::c_char = ',' as i32 as libc::c_char;
    while delimiter as libc::c_int == ',' as i32 {
        match modifierType as libc::c_uint {
            0 => {
                newCommandLine =
                    _Mem_Alloc(host.mempool,
                               ::std::mem::size_of::<sequenceCommandLine_s>()
                                   as libc::c_ulong, false_0,
                               b"../engine/common/sequence.c\x00" as *const u8
                                   as *const libc::c_char, 789 as libc::c_int)
                        as *mut sequenceCommandLine_s;
                memset(newCommandLine as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<sequenceCommandLine_s>() as
                           libc::c_ulong);
                (*newCommandLine).commandType =
                    SEQUENCE_COMMAND_MODIFIER as libc::c_int;
                Sequence_AddCommandLineToEntry(newCommandLine, entry);
                delimiter = Sequence_ParseModifier(newCommandLine)
            }
            1 => {
                delimiter = Sequence_ParseModifier(&mut g_fileScopeDefaults)
            }
            _ => { }
        }
    }
    return delimiter;
}
/*
=============
Sequence_ParseCommand

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseCommand(mut newCommandLine:
                                                   *mut sequenceCommandLine_s)
 -> libc::c_char {
    let mut commandName: [libc::c_char; 256] = [0; 256];
    let mut ch: libc::c_char = 0;
    let mut commandEnum: sequenceCommandEnum_e = SEQUENCE_COMMAND_PAUSE;
    let mut modifierCommandLine: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    Sequence_GetNameValueString(commandName.as_mut_ptr(),
                                256 as libc::c_int as size_t);
    commandEnum =
        Sequence_GetCommandEnumForName(commandName.as_mut_ptr(),
                                       SEQUENCE_TYPE_COMMAND);
    if commandEnum as libc::c_int == SEQUENCE_COMMAND_ERROR as libc::c_int {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: unknown command \"%s\"\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    commandName.as_mut_ptr());
    }
    if Sequence_IsCommandAModifier(commandEnum) as u64 != 0 {
        modifierCommandLine =
            _Mem_Alloc(host.mempool,
                       ::std::mem::size_of::<sequenceCommandLine_s>() as
                           libc::c_ulong, false_0,
                       b"../engine/common/sequence.c\x00" as *const u8 as
                           *const libc::c_char, 825 as libc::c_int) as
                *mut sequenceCommandLine_s;
        memset(modifierCommandLine as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<sequenceCommandLine_s>() as
                   libc::c_ulong);
        (*modifierCommandLine).commandType =
            SEQUENCE_COMMAND_POSTMODIFIER as libc::c_int;
        while !(*newCommandLine).nextCommandLine.is_null() {
            newCommandLine = (*newCommandLine).nextCommandLine
        }
        (*newCommandLine).nextCommandLine = modifierCommandLine;
        newCommandLine = modifierCommandLine
    }
    ch = Sequence_GetSymbol();
    if ch as libc::c_int != '=' as i32 {
        Con_Reportf(b"^1Error:^7 Parsing error on or before line %d of %s.seq: after command \"%s\", expected \'=\', found \'%c\'\n\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    commandName.as_mut_ptr(), ch as libc::c_int);
    }
    Sequence_ReadCommandData(commandEnum, newCommandLine);
    return Sequence_GetSymbol();
}
/*
=============
Sequence_ParseCommandLine

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseCommandLine(mut entry:
                                                       *mut sequenceEntry_s)
 -> libc::c_char {
    let mut symbol: libc::c_char = 0;
    let mut newCommandLine: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    newCommandLine =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sequenceCommandLine_s>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 855 as libc::c_int) as
            *mut sequenceCommandLine_s;
    memset(newCommandLine as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sequenceCommandLine_s>() as libc::c_ulong);
    Sequence_ResetDefaults(newCommandLine, &mut g_blockScopeDefaults);
    Sequence_AddCommandLineToEntry(newCommandLine, entry);
    symbol = Sequence_ParseCommand(newCommandLine);
    while symbol as libc::c_int == ',' as i32 {
        symbol = Sequence_ParseCommand(newCommandLine)
    }
    return symbol;
}
/*
=============
Sequence_ParseMacro

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseMacro(mut entry: *mut sequenceEntry_s)
 -> libc::c_char {
    let mut symbol: libc::c_char = 0;
    let mut newCommandLine: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    newCommandLine =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sequenceCommandLine_s>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 882 as libc::c_int) as
            *mut sequenceCommandLine_s;
    memset(newCommandLine as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sequenceCommandLine_s>() as libc::c_ulong);
    Sequence_ResetDefaults(newCommandLine, &mut g_blockScopeDefaults);
    Sequence_AddCommandLineToEntry(newCommandLine, entry);
    Sequence_ReadCommandData(SEQUENCE_COMMAND_GOSUB, newCommandLine);
    symbol = Sequence_GetSymbol();
    while symbol as libc::c_int == ',' as i32 {
        symbol = Sequence_ParseCommand(newCommandLine)
    }
    return symbol;
}
/*
=============
Sequence_ParseLine

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseLine(mut start: libc::c_char,
                                            mut entry: *mut sequenceEntry_s)
 -> libc::c_char {
    let mut end: libc::c_char = '\u{0}' as i32 as libc::c_char;
    match start as libc::c_int {
        35 => { end = Sequence_ParseCommandLine(entry) }
        36 => {
            end = Sequence_ParseModifierLine(entry, SEQUENCE_TYPE_MODIFIER)
        }
        64 => { end = Sequence_ParseMacro(entry) }
        _ => {
            Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: line must begin with either \'#\' (command) or \'$\' (modifier); found \'%c\'\n\x00"
                            as *const u8 as *const libc::c_char, g_lineNum,
                        g_sequenceParseFileName.as_mut_ptr(),
                        start as libc::c_int);
        }
    }
    return end;
}
/*
=============
Sequence_CalcEntryDuration

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_CalcEntryDuration(mut entry:
                                                        *mut sequenceEntry_s)
 -> libc::c_float {
    let mut duration: libc::c_float = 0.;
    let mut cmd: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    duration = 0 as libc::c_int as libc::c_float;
    cmd = (*entry).firstCommand;
    while !cmd.is_null() {
        duration += (*cmd).delay;
        cmd = (*cmd).nextCommandLine
    }
    return duration;
}
/*
=============
Sequence_DoesEntryContainInfiniteLoop

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_DoesEntryContainInfiniteLoop(mut entry:
                                                                   *mut sequenceEntry_s)
 -> qboolean {
    let mut cmd: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    cmd = (*entry).firstCommand;
    while !cmd.is_null() {
        if (*cmd).repeatCount < 0 as libc::c_int { return true_0 }
        cmd = (*cmd).nextCommandLine
    }
    return false_0;
}
/*
=============
Sequence_IsEntrySafe

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_IsEntrySafe(mut entry: *mut sequenceEntry_s)
 -> qboolean {
    let mut duration: libc::c_float = 0.;
    let mut cmd: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    duration = 0 as libc::c_int as libc::c_float;
    cmd = (*entry).firstCommand;
    while !cmd.is_null() {
        duration += (*cmd).delay;
        if (*cmd).repeatCount < 0 as libc::c_int {
            if duration <= 0 as libc::c_int as libc::c_float {
                return false_0
            }
        }
        cmd = (*cmd).nextCommandLine
    }
    return true_0;
}
/*
=============
Sequence_CreateDefaultsCommand

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_CreateDefaultsCommand(mut entry:
                                                            *mut sequenceEntry_s) {
    let mut cmd: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    cmd =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sequenceCommandLine_s>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 1005 as libc::c_int) as
            *mut sequenceCommandLine_s;
    memset(cmd as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sequenceCommandLine_s>() as libc::c_ulong);
    Sequence_ResetDefaults(cmd, &mut g_fileScopeDefaults);
    (*cmd).commandType = SEQUENCE_COMMAND_SETDEFAULTS as libc::c_int;
    (*cmd).modifierBitField =
        SEQUENCE_MODIFIER_EFFECT_BIT as libc::c_int |
            SEQUENCE_MODIFIER_POSITION_BIT as libc::c_int |
            SEQUENCE_MODIFIER_COLOR_BIT as libc::c_int |
            SEQUENCE_MODIFIER_COLOR2_BIT as libc::c_int |
            SEQUENCE_MODIFIER_FADEIN_BIT as libc::c_int |
            SEQUENCE_MODIFIER_FADEOUT_BIT as libc::c_int |
            SEQUENCE_MODIFIER_HOLDTIME_BIT as libc::c_int |
            SEQUENCE_MODIFIER_FXTIME_BIT as libc::c_int;
    Sequence_AddCommandLineToEntry(cmd, entry);
}
/*
=============
Sequence_ParseEntry

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseEntry() -> libc::c_char {
    let mut symbol: libc::c_char = 0;
    let mut token: [libc::c_char; 256] = [0; 256];
    let mut entry: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    Sequence_GetNameValueString(token.as_mut_ptr(),
                                256 as libc::c_int as size_t);
    symbol = Sequence_GetSymbol();
    if symbol as libc::c_int != '{' as i32 {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: expected \'{\' to start a\n new entry block; found \'%c\' instead!\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(),
                    symbol as libc::c_int);
    }
    entry =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sequenceEntry_s>() as libc::c_ulong,
                   false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 1041 as libc::c_int) as
            *mut sequenceEntry_s;
    Sequence_ResetDefaults(&mut g_blockScopeDefaults,
                           &mut g_fileScopeDefaults);
    (*entry).entryName =
        _copystring(host.mempool, token.as_mut_ptr(),
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1043 as libc::c_int);
    (*entry).fileName =
        _copystring(host.mempool, g_sequenceParseFileName.as_mut_ptr(),
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1044 as libc::c_int);
    (*entry).isGlobal = g_sequenceParseFileIsGlobal;
    (*entry).firstCommand = 0 as *mut sequenceCommandLine_s;
    Sequence_CreateDefaultsCommand(entry);
    symbol = Sequence_GetSymbol();
    while symbol as libc::c_int != '}' as i32 {
        symbol = Sequence_ParseLine(symbol, entry)
    }
    if Sequence_IsEntrySafe(entry) as u64 == 0 {
        Con_Reportf(b"^1Error:^7 Logic error in file %s.seq before line %d: execution of entry \"%%%s\" would cause an infinite loop!\x00"
                        as *const u8 as *const libc::c_char,
                    g_sequenceParseFileName.as_mut_ptr(), g_lineNum,
                    (*entry).entryName);
    }
    (*entry).nextEntry = g_sequenceList;
    g_sequenceList = entry;
    return Sequence_GetSymbol();
}
/*
=============
Sequence_FindSentenceGroup

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FindSentenceGroup(mut groupName:
                                                        *const libc::c_char)
 -> *mut sentenceGroupEntry_s {
    let mut groupEntry: *mut sentenceGroupEntry_s =
        0 as *mut sentenceGroupEntry_s;
    groupEntry = g_sentenceGroupList;
    while !groupEntry.is_null() {
        if Q_strnicmp((*groupEntry).groupName, groupName,
                      99999 as libc::c_int) == 0 {
            return groupEntry
        }
        groupEntry = (*groupEntry).nextEntry
    }
    return 0 as *mut sentenceGroupEntry_s;
}
/*
=============
Sequence_GetSentenceByIndex

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetSentenceByIndex(mut index: libc::c_uint)
 -> *mut sentenceEntry_s {
    let mut sentenceEntry: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    let mut groupEntry: *mut sentenceGroupEntry_s =
        0 as *mut sentenceGroupEntry_s;
    let mut sentenceCount: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    groupEntry = g_sentenceGroupList;
    while !groupEntry.is_null() {
        sentenceCount =
            sentenceCount.wrapping_add((*groupEntry).numSentences);
        if index < sentenceCount {
            sentenceEntry = (*groupEntry).firstSentence;
            while !sentenceEntry.is_null() {
                if (*sentenceEntry).index == index { return sentenceEntry }
                sentenceEntry = (*sentenceEntry).nextEntry
            }
        }
        groupEntry = (*groupEntry).nextEntry
    }
    return 0 as *mut sentenceEntry_s;
}
/*
=============
Sequence_PickSentence

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_PickSentence(mut groupName:
                                                   *const libc::c_char,
                                               mut pickMethod: libc::c_int,
                                               mut picked: *mut libc::c_int)
 -> *mut sentenceEntry_s {
    let mut sentenceEntry: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    let mut groupEntry: *mut sentenceGroupEntry_s =
        0 as *mut sentenceGroupEntry_s;
    let mut pickedIdx: libc::c_uint = 0;
    let mut entryIdx: libc::c_uint = 0;
    groupEntry = Sequence_FindSentenceGroup(groupName);
    if !groupEntry.is_null() {
        pickedIdx =
            COM_RandomLong(0 as libc::c_int,
                           (*groupEntry).numSentences.wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                               as libc::c_int) as libc::c_uint;
        sentenceEntry = (*groupEntry).firstSentence;
        entryIdx = pickedIdx;
        while entryIdx != 0 {
            sentenceEntry = (*sentenceEntry).nextEntry;
            entryIdx = entryIdx.wrapping_sub(1)
        }
    } else {
        pickedIdx = 0 as libc::c_int as libc::c_uint;
        sentenceEntry = 0 as *mut sentenceEntry_s
    }
    if !picked.is_null() { *picked = pickedIdx as libc::c_int }
    return sentenceEntry;
}
/*
=============
Sequence_AddSentenceGroup

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_AddSentenceGroup(mut groupName:
                                                       *mut libc::c_char)
 -> *mut sentenceGroupEntry_s {
    let mut entry: *mut sentenceGroupEntry_s = 0 as *mut sentenceGroupEntry_s;
    let mut last: *mut sentenceGroupEntry_s = 0 as *mut sentenceGroupEntry_s;
    entry =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sentenceGroupEntry_s>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 1159 as libc::c_int) as
            *mut sentenceGroupEntry_s;
    (*entry).numSentences = 0 as libc::c_int as libc::c_uint;
    (*entry).firstSentence = 0 as *mut sentenceEntry_s;
    (*entry).nextEntry = 0 as *mut sentenceGroupEntry_s;
    (*entry).groupName =
        _copystring(host.mempool, groupName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1163 as libc::c_int);
    if !g_sentenceGroupList.is_null() {
        last = g_sentenceGroupList;
        while !(*last).nextEntry.is_null() { last = (*last).nextEntry }
        (*last).nextEntry = entry
    } else { g_sentenceGroupList = entry }
    return entry;
}
/*
=============
Sequence_AddSentenceToGroup

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_AddSentenceToGroup(mut groupName:
                                                         *mut libc::c_char,
                                                     mut data:
                                                         *mut libc::c_char) {
    let mut entry: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    let mut last: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    let mut group: *mut sentenceGroupEntry_s = 0 as *mut sentenceGroupEntry_s;
    group = Sequence_FindSentenceGroup(groupName);
    if group.is_null() {
        group = Sequence_AddSentenceGroup(groupName);
        if group.is_null() {
            Con_Reportf(b"^1Error:^7 Unable to allocate sentence group %s at line %d in file %s.seq\x00"
                            as *const u8 as *const libc::c_char, groupName,
                        g_lineNum, g_sequenceParseFileName.as_mut_ptr());
        }
    }
    entry =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sentenceEntry_s>() as libc::c_ulong,
                   false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 1199 as libc::c_int) as
            *mut sentenceEntry_s;
    (*entry).nextEntry = 0 as *mut sentenceEntry_s;
    (*entry).data =
        _copystring(host.mempool, data,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1201 as libc::c_int);
    (*entry).index = g_nonGlobalSentences;
    (*entry).isGlobal = g_sequenceParseFileIsGlobal;
    (*group).numSentences = (*group).numSentences.wrapping_add(1);
    g_nonGlobalSentences = g_nonGlobalSentences.wrapping_add(1);
    if !(*group).firstSentence.is_null() {
        last = (*group).firstSentence;
        while !(*last).nextEntry.is_null() { last = (*last).nextEntry }
        (*last).nextEntry = entry
    } else { (*group).firstSentence = entry };
}
/*
=============
Sequence_ParseSentenceLine

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseSentenceLine() -> qboolean {
    let mut data: [libc::c_char; 1024] = [0; 1024];
    let mut fullgroup: [libc::c_char; 64] = [0; 64];
    let mut groupName: [libc::c_char; 64] = [0; 64];
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastCharacterPos: libc::c_int = 0;
    let mut len: size_t = 0;
    len =
        Sequence_GetToken(fullgroup.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong);
    if *fullgroup.as_mut_ptr() as libc::c_int == '}' as i32 { return true_0 }
    c = fullgroup.as_mut_ptr().offset(len as isize);
    while *(*__ctype_b_loc()).offset(*c as libc::c_int as isize) as
              libc::c_int &
              _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0 &&
              *c as libc::c_int != '_' as i32 {
        c = c.offset(-1)
    }
    c = c.offset(1 as libc::c_int as isize);
    if *c != 0 { *c = 0 as libc::c_int as libc::c_char }
    strcpy(groupName.as_mut_ptr(), fullgroup.as_mut_ptr());
    len =
        Sequence_GetLine(data.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 1024]>() as
                             libc::c_ulong as libc::c_int);
    lastCharacterPos =
        len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if data[lastCharacterPos as usize] as libc::c_int == '\n' as i32 ||
           data[lastCharacterPos as usize] as libc::c_int == '\r' as i32 {
        data[lastCharacterPos as usize] = 0 as libc::c_int as libc::c_char
    }
    Sequence_AddSentenceToGroup(groupName.as_mut_ptr(), data.as_mut_ptr());
    return false_0;
}
/*
==============
Sequence_ParseSentenceBlock

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseSentenceBlock() -> libc::c_char {
    let mut end: qboolean = false_0;
    let mut ch: libc::c_char = Sequence_GetSymbol();
    if ch as libc::c_int != '{' as i32 {
        Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: expected \'{\' to start a\n new sentence block; found \'%c\' instead!\x00"
                        as *const u8 as *const libc::c_char, g_lineNum,
                    g_sequenceParseFileName.as_mut_ptr(), ch as libc::c_int);
    }
    while end as u64 == 0 { end = Sequence_ParseSentenceLine() }
    return Sequence_GetSymbol();
}
/*
==============
Sequence_ParseGlobalDataBlock

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseGlobalDataBlock() -> libc::c_char {
    let mut token: [libc::c_char; 256] = [0; 256];
    Sequence_GetNameValueString(token.as_mut_ptr(),
                                256 as libc::c_int as size_t);
    if Q_strnicmp(token.as_mut_ptr(),
                  b"Sentences\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        Con_Reportf(b"^1Error:^7 Syntax error in file %s.seq on line %d: found global data block symbol \'!\' with unknown data type \"%s\"\x00"
                        as *const u8 as *const libc::c_char,
                    g_sequenceParseFileName.as_mut_ptr(), g_lineNum,
                    token.as_mut_ptr());
    }
    return Sequence_ParseSentenceBlock();
}
/*
==============
Sequence_GetEntryForName

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_GetEntryForName(mut entryName:
                                                      *const libc::c_char)
 -> *mut sequenceEntry_s {
    let mut scan: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    scan = g_sequenceList;
    while !scan.is_null() {
        if Q_strnicmp(entryName, (*scan).entryName, 99999 as libc::c_int) == 0
           {
            return scan
        }
        scan = (*scan).nextEntry
    }
    return 0 as *mut sequenceEntry_s;
}
/*
==============
Sequence_CopyCommand

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_CopyCommand(mut commandOrig:
                                                  *mut sequenceCommandLine_s)
 -> *mut sequenceCommandLine_s {
    let mut commandCopy: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    commandCopy =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<sequenceCommandLine_s>() as
                       libc::c_ulong, false_0,
                   b"../engine/common/sequence.c\x00" as *const u8 as
                       *const libc::c_char, 1330 as libc::c_int) as
            *mut sequenceCommandLine_s;
    (*commandCopy).commandType = (*commandOrig).commandType;
    (*commandCopy).clientMessage = (*commandOrig).clientMessage;
    (*commandCopy).clientMessage.pMessage =
        _copystring(host.mempool, (*commandOrig).clientMessage.pMessage,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1334 as libc::c_int);
    (*commandCopy).clientMessage.pName =
        _copystring(host.mempool, (*commandOrig).clientMessage.pName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1335 as libc::c_int);
    (*commandCopy).speakerName =
        _copystring(host.mempool, (*commandOrig).speakerName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1336 as libc::c_int);
    (*commandCopy).listenerName =
        _copystring(host.mempool, (*commandOrig).listenerName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1337 as libc::c_int);
    (*commandCopy).soundFileName =
        _copystring(host.mempool, (*commandOrig).soundFileName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1338 as libc::c_int);
    (*commandCopy).sentenceName =
        _copystring(host.mempool, (*commandOrig).sentenceName,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1339 as libc::c_int);
    (*commandCopy).fireTargetNames =
        _copystring(host.mempool, (*commandOrig).fireTargetNames,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1340 as libc::c_int);
    (*commandCopy).killTargetNames =
        _copystring(host.mempool, (*commandOrig).killTargetNames,
                    b"../engine/common/sequence.c\x00" as *const u8 as
                        *const libc::c_char, 1341 as libc::c_int);
    (*commandCopy).delay = (*commandOrig).delay;
    (*commandCopy).repeatCount = (*commandOrig).repeatCount;
    (*commandCopy).textChannel = (*commandOrig).textChannel;
    (*commandCopy).modifierBitField = (*commandOrig).modifierBitField;
    (*commandCopy).nextCommandLine = 0 as *mut sequenceCommandLine_s;
    return commandCopy;
}
/*
==============
Sequence_CopyCommandList

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_CopyCommandList(mut list:
                                                      *mut sequenceCommandLine_s)
 -> *mut sequenceCommandLine_s {
    let mut scan: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    let mut copy: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    let mut new: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    let mut prev: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    copy = 0 as *mut sequenceCommandLine_s;
    prev = 0 as *mut sequenceCommandLine_s;
    scan = list;
    while !scan.is_null() {
        if (*scan).commandType != SEQUENCE_COMMAND_SETDEFAULTS as libc::c_int
           {
            new = Sequence_CopyCommand(scan);
            if !prev.is_null() {
                (*prev).nextCommandLine = new;
                prev = new
            } else { prev = new; copy = new }
        }
        scan = (*scan).nextCommandLine
    }
    return copy;
}
/*
==============
Sequence_ExpandGosubsForEntry

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ExpandGosubsForEntry(mut entry:
                                                           *mut sequenceEntry_s)
 -> qboolean {
    let mut cmd: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    let mut copyList: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    let mut scan: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    let mut gosubEntry: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    let mut foundGosubs: qboolean = false_0;
    cmd = (*entry).firstCommand;
    while !cmd.is_null() {
        if !(*cmd).clientMessage.pName.is_null() {
            if Q_strnicmp((*cmd).clientMessage.pName, (*entry).entryName,
                          99999 as libc::c_int) == 0 {
                Con_Reportf(b"^1Error:^7 Error in %s.seq: entry \"%s\" gosubs itself!\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*entry).fileName, (*entry).entryName);
            }
            gosubEntry = Sequence_GetEntryForName((*cmd).clientMessage.pName);
            if gosubEntry.is_null() {
                Con_Reportf(b"^1Error:^7 Error in %s.seq: Gosub in entry \"%s\" specified unknown entry \"%s\"\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*entry).fileName, (*entry).entryName,
                            (*cmd).clientMessage.pName);
            }
            foundGosubs = true_0;
            copyList = Sequence_CopyCommandList((*gosubEntry).firstCommand);
            if !copyList.is_null() {
                scan = (*copyList).nextCommandLine;
                while !scan.is_null() { scan = (*scan).nextCommandLine }
                (*scan).nextCommandLine = (*cmd).nextCommandLine;
                if !(*cmd).clientMessage.pName.is_null() {
                    _Mem_Free((*cmd).clientMessage.pName as *mut libc::c_void,
                              b"../engine/common/sequence.c\x00" as *const u8
                                  as *const libc::c_char,
                              1420 as libc::c_int);
                }
                (*cmd).clientMessage.pName = 0 as *mut libc::c_char;
                cmd = scan
            } else {
                if !(*cmd).clientMessage.pName.is_null() {
                    _Mem_Free((*cmd).clientMessage.pName as *mut libc::c_void,
                              b"../engine/common/sequence.c\x00" as *const u8
                                  as *const libc::c_char,
                              1426 as libc::c_int);
                }
                (*cmd).clientMessage.pName = 0 as *mut libc::c_char
            }
        }
        cmd = (*cmd).nextCommandLine
    }
    return (foundGosubs as u64 == 0) as libc::c_int as qboolean;
}
/*
==============
Sequence_ExpandAllGosubs

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ExpandAllGosubs() {
    let mut scan: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    let mut isComplete: qboolean = true_0;
    while isComplete as u64 == 0 {
        scan = g_sequenceList;
        while !scan.is_null() {
            isComplete = Sequence_ExpandGosubsForEntry(scan);
            scan = (*scan).nextEntry
        }
    };
}
/*
==============
Sequence_FlattenEntry

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FlattenEntry(mut entry:
                                                   *mut sequenceEntry_s) {
    let mut cmd: *mut sequenceCommandLine_s = 0 as *mut sequenceCommandLine_s;
    let mut last: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    cmd = (*entry).firstCommand;
    while !cmd.is_null() {
        match (*cmd).commandType {
            8 => {
                Sequence_WriteDefaults(cmd, &mut g_blockScopeDefaults);
                (*cmd).commandType = SEQUENCE_COMMAND_NOOP as libc::c_int
            }
            9 => { Sequence_WriteDefaults(cmd, &mut g_blockScopeDefaults); }
            10 => { Sequence_WriteDefaults(cmd, last); }
            _ => {
                Sequence_BakeDefaults(cmd, &mut g_blockScopeDefaults);
                last = cmd
            }
        }
        cmd = (*cmd).nextCommandLine
    };
}
/*
==============
Sequence_FlattenAllEntries

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FlattenAllEntries() {
    let mut entry: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    entry = g_sequenceList;
    while !entry.is_null() {
        Sequence_FlattenEntry(entry);
        entry = (*entry).nextEntry
    };
}
/*
==============
Sequence_ParseBuffer

==============
*/
unsafe extern "C" fn Sequence_ParseBuffer(mut buffer: *mut libc::c_char,
                                          mut bufferSize: libc::c_int) {
    let mut symbol: libc::c_char = 0;
    g_lineNum = 1 as libc::c_int;
    g_scan = buffer;
    g_lineScan = g_scan;
    Sequence_StripComments(buffer, &mut bufferSize);
    Sequence_ResetDefaults(&mut g_fileScopeDefaults,
                           0 as *mut sequenceCommandLine_s);
    symbol = Sequence_GetSymbol();
    while symbol != 0 {
        match symbol as libc::c_int {
            36 => {
                loop  {
                    symbol = Sequence_ParseModifier(&mut g_fileScopeDefaults);
                    if !(symbol as libc::c_int == ',' as i32) { break ; }
                }
            }
            37 => { symbol = Sequence_ParseEntry() }
            33 => { symbol = Sequence_ParseGlobalDataBlock() }
            _ => {
                Con_Reportf(b"^1Error:^7 Parsing error on line %d of %s.seq: At file scope, lines must begin with \'$\' (modifier) or \'%%\' (entry block) or \'!\' (sentence / global data block); found \'%c\'\n\x00"
                                as *const u8 as *const libc::c_char,
                            g_lineNum, g_sequenceParseFileName.as_mut_ptr(),
                            symbol as libc::c_int);
            }
        }
    }
    Sequence_ExpandAllGosubs();
    Sequence_FlattenAllEntries();
}
/*
==============
Sequence_ParseFile

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_ParseFile(mut fileName: *const libc::c_char,
                                            mut isGlobal: qboolean) {
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut bufSize: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    Q_strncpy(g_sequenceParseFileName.as_mut_ptr(), fileName,
              99999 as libc::c_int as size_t);
    g_sequenceParseFileIsGlobal = isGlobal;
    buffer =
        FS_LoadFile(va(b"sequences/%s.seq\x00" as *const u8 as
                           *const libc::c_char, fileName), &mut bufSize,
                    true_0);
    if buffer.is_null() { return }
    Con_Reportf(b"reading sequence file: %s\n\x00" as *const u8 as
                    *const libc::c_char, fileName);
    Sequence_ParseBuffer(buffer as *mut libc::c_char, bufSize as libc::c_int);
    _Mem_Free(buffer as *mut libc::c_void,
              b"../engine/common/sequence.c\x00" as *const u8 as
                  *const libc::c_char, 1571 as libc::c_int);
}
/*
==============
Sequence_Init

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_Init() {
    Sequence_ParseFile(b"global\x00" as *const u8 as *const libc::c_char,
                       true_0);
}
/*
==============
SequenceGet

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_Get(mut fileName: *const libc::c_char,
                                      mut entryName: *const libc::c_char)
 -> *mut sequenceEntry_s {
    let mut scan: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    scan = g_sequenceList;
    while !scan.is_null() {
        if (fileName.is_null() ||
                Q_strnicmp(fileName, (*scan).fileName, 99999 as libc::c_int)
                    == 0) &&
               Q_strnicmp(entryName, (*scan).entryName, 99999 as libc::c_int)
                   == 0 {
            return scan
        }
        scan = (*scan).nextEntry
    }
    return 0 as *mut sequenceEntry_s;
}
/*
==============
Sequence_FreeCommand

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FreeCommand(mut kill:
                                                  *mut sequenceCommandLine_s) {
    if !(*kill).fireTargetNames.is_null() {
        _Mem_Free((*kill).fireTargetNames as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1613 as libc::c_int);
    }
    if !(*kill).speakerName.is_null() {
        _Mem_Free((*kill).speakerName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1614 as libc::c_int);
    }
    if !(*kill).listenerName.is_null() {
        _Mem_Free((*kill).listenerName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1615 as libc::c_int);
    }
    if !(*kill).soundFileName.is_null() {
        _Mem_Free((*kill).soundFileName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1616 as libc::c_int);
    }
    if !(*kill).sentenceName.is_null() {
        _Mem_Free((*kill).sentenceName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1617 as libc::c_int);
    }
    if !(*kill).clientMessage.pName.is_null() {
        _Mem_Free((*kill).clientMessage.pName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1618 as libc::c_int);
    }
    if !(*kill).clientMessage.pMessage.is_null() {
        _Mem_Free((*kill).clientMessage.pMessage as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1619 as libc::c_int);
    };
}
/*
==============
Sequence_FreeEntry

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FreeEntry(mut kill: *mut sequenceEntry_s) {
    let mut dead: *mut sequenceCommandLine_s =
        0 as *mut sequenceCommandLine_s;
    if !(*kill).entryName.is_null() {
        _Mem_Free((*kill).entryName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1632 as libc::c_int);
    }
    if !(*kill).fileName.is_null() {
        _Mem_Free((*kill).fileName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1633 as libc::c_int);
    }
    dead = (*kill).firstCommand;
    while !dead.is_null() {
        (*kill).firstCommand = (*dead).nextCommandLine;
        Sequence_FreeCommand(dead);
        dead = (*dead).nextCommandLine
    }
    if !kill.is_null() {
        _Mem_Free(kill as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1641 as libc::c_int);
    };
}
/*
==============
Sequence_FreeSentence

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FreeSentence(mut sentenceEntry:
                                                   *mut sentenceEntry_s) {
    if !(*sentenceEntry).data.is_null() {
        _Mem_Free((*sentenceEntry).data as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1652 as libc::c_int);
    }
    if !sentenceEntry.is_null() {
        _Mem_Free(sentenceEntry as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1653 as libc::c_int);
    };
}
/*
==============
Sequence_FreeSentenceGroup

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FreeSentenceGroup(mut groupEntry:
                                                        *mut sentenceGroupEntry_s) {
    if !(*groupEntry).groupName.is_null() {
        _Mem_Free((*groupEntry).groupName as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1664 as libc::c_int);
    }
    if !groupEntry.is_null() {
        _Mem_Free(groupEntry as *mut libc::c_void,
                  b"../engine/common/sequence.c\x00" as *const u8 as
                      *const libc::c_char, 1665 as libc::c_int);
    };
}
/*
==============
Sequence_FreeSentenceGroupEntries

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_FreeSentenceGroupEntries(mut groupEntry:
                                                               *mut sentenceGroupEntry_s,
                                                           mut purgeGlobals:
                                                               qboolean) {
    let mut sentenceEntry: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    let mut deadSentence: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    let mut prevSentence: *mut sentenceEntry_s = 0 as *mut sentenceEntry_s;
    sentenceEntry = (*groupEntry).firstSentence;
    prevSentence = 0 as *mut sentenceEntry_s;
    while !sentenceEntry.is_null() {
        if (*sentenceEntry).isGlobal as u64 == 0 ||
               purgeGlobals as libc::c_uint != 0 {
            if !prevSentence.is_null() {
                (*prevSentence).nextEntry = (*sentenceEntry).nextEntry
            } else {
                (*groupEntry).firstSentence = (*sentenceEntry).nextEntry
            }
            (*groupEntry).numSentences =
                (*groupEntry).numSentences.wrapping_sub(1);
            g_nonGlobalSentences = g_nonGlobalSentences.wrapping_sub(1);
            deadSentence = sentenceEntry;
            sentenceEntry = (*sentenceEntry).nextEntry;
            Sequence_FreeSentence(deadSentence);
        } else {
            prevSentence = sentenceEntry;
            sentenceEntry = (*sentenceEntry).nextEntry
        }
    };
}
/*
==============
Sequence_PurgeEntries

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_PurgeEntries(mut purgeGlobals: qboolean) {
    let mut scan: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    let mut dead: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    let mut prev: *mut sequenceEntry_s = 0 as *mut sequenceEntry_s;
    let mut groupEntry: *mut sentenceGroupEntry_s =
        0 as *mut sentenceGroupEntry_s;
    let mut deadGroup: *mut sentenceGroupEntry_s =
        0 as *mut sentenceGroupEntry_s;
    let mut prevGroup: *mut sentenceGroupEntry_s =
        0 as *mut sentenceGroupEntry_s;
    dead = 0 as *mut sequenceEntry_s;
    prev = 0 as *mut sequenceEntry_s;
    scan = g_sequenceList;
    while !scan.is_null() {
        if (*scan).isGlobal as u64 == 0 || purgeGlobals as libc::c_uint != 0 {
            if !prev.is_null() {
                (*prev).nextEntry = (*scan).nextEntry
            } else { g_sequenceList = (*scan).nextEntry }
            dead = scan;
            scan = (*scan).nextEntry;
            Sequence_FreeEntry(dead);
        } else { prev = scan; scan = (*scan).nextEntry }
    }
    groupEntry = g_sentenceGroupList;
    prevGroup = 0 as *mut sentenceGroupEntry_s;
    while !groupEntry.is_null() {
        Sequence_FreeSentenceGroupEntries(groupEntry, purgeGlobals);
        if (*groupEntry).numSentences != 0 {
            prevGroup = groupEntry;
            groupEntry = (*groupEntry).nextEntry
        } else {
            if !prevGroup.is_null() {
                (*prevGroup).nextEntry = (*groupEntry).nextEntry
            } else { g_sentenceGroupList = (*groupEntry).nextEntry }
            deadGroup = groupEntry;
            groupEntry = (*groupEntry).nextEntry;
            Sequence_FreeSentenceGroup(deadGroup);
        }
    };
}
/*
==============
Sequence_OnLevelLoad

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Sequence_OnLevelLoad(mut mapName:
                                                  *const libc::c_char) {
    Sequence_PurgeEntries(false_0);
    Sequence_ParseFile(mapName, false_0);
}
