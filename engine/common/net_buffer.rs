#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type decallist_s;
    #[no_mangle]
    fn fmodf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut host: host_parm_t;
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
pub type word = libc::c_ushort;
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
pub type netsrc_t = libc::c_uint;
pub const NS_COUNT: netsrc_t = 2;
pub const NS_SERVER: netsrc_t = 1;
pub const NS_CLIENT: netsrc_t = 0;
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
pub struct sizebuf_s {
    pub bOverflow: qboolean,
    pub pDebugName: *const libc::c_char,
    pub pData: *mut byte,
    pub iCurBit: libc::c_int,
    pub nDataBits: libc::c_int,
}
pub type sizebuf_t = sizebuf_s;
#[inline]
unsafe extern "C" fn MSG_GetMaxBytes(mut sb: *mut sizebuf_t) -> libc::c_int {
    return (*sb).nDataBits >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline(always)]
unsafe extern "C" fn __tg_fmod(mut __x: libc::c_float, mut __y: libc::c_float)
 -> libc::c_float {
    return fmodf(__x, __y);
}
/*
net_buffer.c - network bitbuffer io functions
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
//#define DEBUG_NET_MESSAGES_SEND
//#define DEBUG_NET_MESSAGES_READ
// precalculated bit masks for WriteUBitLong.
// Using these tables instead of doing the calculations
// gives a 33% speedup in WriteUBitLong.
static mut BitWriteMasks: [[dword; 33]; 32] = [[0; 33]; 32];
static mut ExtraMasks: [dword; 32] = [0; 32];
#[no_mangle]
pub unsafe extern "C" fn MSG_BigShort(mut swap: libc::c_ushort)
 -> libc::c_ushort {
    return (swap as libc::c_int >> 8 as libc::c_int |
                (swap as libc::c_int) << 8 as libc::c_int) as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_InitMasks() {
    let mut startbit: uint = 0;
    let mut endbit: uint = 0;
    let mut maskBit: uint = 0;
    let mut nBitsLeft: uint = 0;
    startbit = 0 as libc::c_int as uint;
    while startbit < 32 as libc::c_int as libc::c_uint {
        nBitsLeft = 0 as libc::c_int as uint;
        while nBitsLeft < 33 as libc::c_int as libc::c_uint {
            endbit = startbit.wrapping_add(nBitsLeft);
            BitWriteMasks[startbit as usize][nBitsLeft as usize] =
                ((1 as libc::c_uint) <<
                     startbit).wrapping_sub(1 as libc::c_int as libc::c_uint);
            if endbit < 32 as libc::c_int as libc::c_uint {
                BitWriteMasks[startbit as usize][nBitsLeft as usize] |=
                    !((1 as libc::c_uint) <<
                          endbit).wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint)
            }
            nBitsLeft = nBitsLeft.wrapping_add(1)
        }
        startbit = startbit.wrapping_add(1)
    }
    maskBit = 0 as libc::c_int as uint;
    while maskBit < 32 as libc::c_int as libc::c_uint {
        ExtraMasks[maskBit as usize] =
            ((1 as libc::c_uint) <<
                 maskBit).wrapping_sub(1 as libc::c_int as libc::c_uint);
        maskBit = maskBit.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_InitExt(mut sb: *mut sizebuf_t,
                                     mut pDebugName: *const libc::c_char,
                                     mut pData: *mut libc::c_void,
                                     mut nBytes: libc::c_int,
                                     mut nMaxBits: libc::c_int) {
    MSG_StartWriting(sb, pData, nBytes, 0 as libc::c_int, nMaxBits);
    (*sb).pDebugName = pDebugName;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_StartWriting(mut sb: *mut sizebuf_t,
                                          mut pData: *mut libc::c_void,
                                          mut nBytes: libc::c_int,
                                          mut iStartBit: libc::c_int,
                                          mut nBits: libc::c_int) {
    // make sure it's dword aligned and padded.
    (*sb).pDebugName = b"Unnamed\x00" as *const u8 as *const libc::c_char;
    (*sb).pData = pData as *mut byte;
    if nBits == -(1 as libc::c_int) {
        (*sb).nDataBits = nBytes << 3 as libc::c_int
    } else { (*sb).nDataBits = nBits }
    (*sb).iCurBit = iStartBit;
    (*sb).bOverflow = false_0;
}
/*
=======================
MSG_Clear

for clearing overflowed buffer
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_Clear(mut sb: *mut sizebuf_t) {
    (*sb).iCurBit = 0 as libc::c_int;
    (*sb).bOverflow = false_0;
}
unsafe extern "C" fn MSG_Overflow(mut sb: *mut sizebuf_t,
                                  mut nBits: libc::c_int) -> qboolean {
    if (*sb).iCurBit + nBits > (*sb).nDataBits { (*sb).bOverflow = true_0 }
    return (*sb).bOverflow;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_CheckOverflow(mut sb: *mut sizebuf_t)
 -> qboolean {
    return MSG_Overflow(sb, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_SeekToBit(mut sb: *mut sizebuf_t,
                                       mut bitPos: libc::c_int,
                                       mut whence: libc::c_int)
 -> libc::c_int {
    // compute the file offset
    match whence {
        1 => { bitPos += (*sb).iCurBit }
        0 => { }
        2 => { bitPos += (*sb).nDataBits }
        _ => { return -(1 as libc::c_int) }
    }
    if bitPos < 0 as libc::c_int || bitPos > (*sb).nDataBits {
        return -(1 as libc::c_int)
    }
    (*sb).iCurBit = bitPos;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_SeekToByte(mut sb: *mut sizebuf_t,
                                        mut bytePos: libc::c_int) {
    (*sb).iCurBit = bytePos << 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteOneBit(mut sb: *mut sizebuf_t,
                                         mut nValue: libc::c_int) {
    if MSG_Overflow(sb, 1 as libc::c_int) as u64 == 0 {
        if nValue != 0 {
            let ref mut fresh0 =
                *(*sb).pData.offset(((*sb).iCurBit >> 3 as libc::c_int) as
                                        isize);
            *fresh0 =
                (*fresh0 as libc::c_uint |
                     (1 as libc::c_uint) <<
                         ((*sb).iCurBit & 7 as libc::c_int)) as byte
        } else {
            let ref mut fresh1 =
                *(*sb).pData.offset(((*sb).iCurBit >> 3 as libc::c_int) as
                                        isize);
            *fresh1 =
                (*fresh1 as libc::c_uint &
                     !((1 as libc::c_uint) <<
                           ((*sb).iCurBit & 7 as libc::c_int))) as byte
        }
        (*sb).iCurBit += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteUBitLong(mut sb: *mut sizebuf_t,
                                           mut curData: uint,
                                           mut numbits: libc::c_int) {
    // bounds checking..
    if (*sb).iCurBit + numbits > (*sb).nDataBits {
        (*sb).bOverflow = true_0; // Mask in a dword.
        (*sb).iCurBit = (*sb).nDataBits
    } else {
        let mut nBitsLeft: libc::c_int = numbits;
        let mut iCurBit: libc::c_int = (*sb).iCurBit;
        let mut iDWord: uint = (iCurBit >> 5 as libc::c_int) as uint;
        let mut iCurBitMasked: dword = 0;
        let mut nBitsWritten: libc::c_int = 0;
        iCurBitMasked = (iCurBit & 31 as libc::c_int) as dword;
        let ref mut fresh2 =
            *((*sb).pData as *mut dword).offset(iDWord as isize);
        *fresh2 &= BitWriteMasks[iCurBitMasked as usize][nBitsLeft as usize];
        let ref mut fresh3 =
            *((*sb).pData as *mut dword).offset(iDWord as isize);
        *fresh3 |= curData << iCurBitMasked;
        // did it span a dword?
        nBitsWritten =
            (32 as libc::c_int as libc::c_uint).wrapping_sub(iCurBitMasked) as
                libc::c_int;
        if nBitsWritten < nBitsLeft {
            nBitsLeft -= nBitsWritten;
            iCurBit += nBitsWritten;
            curData >>= nBitsWritten;
            iCurBitMasked = (iCurBit & 31 as libc::c_int) as dword;
            let ref mut fresh4 =
                *((*sb).pData as
                      *mut dword).offset(iDWord.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                             as isize);
            *fresh4 &=
                BitWriteMasks[iCurBitMasked as usize][nBitsLeft as usize];
            let ref mut fresh5 =
                *((*sb).pData as
                      *mut dword).offset(iDWord.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                             as isize);
            *fresh5 |= curData << iCurBitMasked
        }
        (*sb).iCurBit += numbits
    };
}
/*
=======================
MSG_WriteSBitLong

sign bit comes first
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteSBitLong(mut sb: *mut sizebuf_t,
                                           mut data: libc::c_int,
                                           mut numbits: libc::c_int) {
    // do we have a valid # of bits to encode with?
    // NOTE: it does this wierdness here so it's bit-compatible with regular integer data in the buffer.
	// (Some old code writes direct integers right into the buffer).
    if data < 0 as libc::c_int {
        MSG_WriteUBitLong(sb,
                          (0x80000000 as
                               libc::c_uint).wrapping_add(data as
                                                              libc::c_uint),
                          numbits - 1 as libc::c_int);
        MSG_WriteOneBit(sb, 1 as libc::c_int);
    } else {
        MSG_WriteUBitLong(sb, data as uint, numbits - 1 as libc::c_int);
        MSG_WriteOneBit(sb, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBitLong(mut sb: *mut sizebuf_t,
                                          mut data: uint,
                                          mut numbits: libc::c_int,
                                          mut bSigned: qboolean) {
    if bSigned as u64 != 0 {
        MSG_WriteSBitLong(sb, data as libc::c_int, numbits);
    } else { MSG_WriteUBitLong(sb, data, numbits); };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBits(mut sb: *mut sizebuf_t,
                                       mut pData: *const libc::c_void,
                                       mut nBits: libc::c_int) -> qboolean {
    let mut pOut: *mut byte = pData as *mut byte;
    let mut nBitsLeft: libc::c_int = nBits;
    // get output dword-aligned.
    while pOut as dword & 3 as libc::c_int as libc::c_uint !=
              0 as libc::c_int as libc::c_uint &&
              nBitsLeft >= 8 as libc::c_int {
        MSG_WriteUBitLong(sb, *pOut as uint, 8 as libc::c_int);
        nBitsLeft -= 8 as libc::c_int;
        pOut = pOut.offset(1)
    }
    // read dwords.
    while nBitsLeft >= 32 as libc::c_int {
        MSG_WriteUBitLong(sb, *(pOut as *mut dword), 32 as libc::c_int);
        pOut =
            pOut.offset(::std::mem::size_of::<dword>() as libc::c_ulong as
                            isize);
        nBitsLeft -= 32 as libc::c_int
    }
    // read the remaining bytes.
    while nBitsLeft >= 8 as libc::c_int {
        MSG_WriteUBitLong(sb, *pOut as uint, 8 as libc::c_int);
        nBitsLeft -= 8 as libc::c_int;
        pOut = pOut.offset(1)
    }
    // Read the remaining bits.
    if nBitsLeft != 0 { MSG_WriteUBitLong(sb, *pOut as uint, nBitsLeft); }
    return ((*sb).bOverflow as u64 == 0) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBitAngle(mut sb: *mut sizebuf_t,
                                           mut fAngle: libc::c_float,
                                           mut numbits: libc::c_int) {
    let mut mask: uint = 0;
    let mut shift: uint = 0;
    let mut d: libc::c_int = 0;
    // clamp the angle before receiving
    fAngle = __tg_fmod(fAngle, 360.0f32);
    if fAngle < 0 as libc::c_int as libc::c_float { fAngle += 360.0f32 }
    shift = ((1 as libc::c_int) << numbits) as uint;
    mask = shift.wrapping_sub(1 as libc::c_int as libc::c_uint);
    d = (fAngle * shift as libc::c_float / 360.0f32) as libc::c_int;
    d = (d as libc::c_uint & mask) as libc::c_int;
    MSG_WriteUBitLong(sb, d as uint, numbits);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteCoord(mut sb: *mut sizebuf_t,
                                        mut val: libc::c_float) {
    // g-cont. we loose precision here but keep old size of coord variable!
    if host.features &
           ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        MSG_WriteShort(sb,
                       if val < 0.0f32 {
                           (val - 0.5f32) as libc::c_int
                       } else { (val + 0.5f32) as libc::c_int });
    } else { MSG_WriteShort(sb, (val * 8.0f32) as libc::c_int); };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteVec3Coord(mut sb: *mut sizebuf_t,
                                            mut fa: *const libc::c_float) {
    MSG_WriteCoord(sb, *fa.offset(0 as libc::c_int as isize));
    MSG_WriteCoord(sb, *fa.offset(1 as libc::c_int as isize));
    MSG_WriteCoord(sb, *fa.offset(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteVec3Angles(mut sb: *mut sizebuf_t,
                                             mut fa: *const libc::c_float) {
    MSG_WriteBitAngle(sb, *fa.offset(0 as libc::c_int as isize),
                      16 as libc::c_int);
    MSG_WriteBitAngle(sb, *fa.offset(1 as libc::c_int as isize),
                      16 as libc::c_int);
    MSG_WriteBitAngle(sb, *fa.offset(2 as libc::c_int as isize),
                      16 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBitFloat(mut sb: *mut sizebuf_t,
                                           mut val: libc::c_float) {
    let mut intVal: libc::c_int = 0;
    intVal = *(&mut val as *mut libc::c_float as *mut libc::c_int);
    MSG_WriteUBitLong(sb, intVal as uint, 32 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteCmdExt(mut sb: *mut sizebuf_t,
                                         mut cmd: libc::c_int,
                                         mut type_0: netsrc_t,
                                         mut name: *const libc::c_char) {
    MSG_WriteUBitLong(sb, cmd as uint,
                      ((::std::mem::size_of::<byte>() as libc::c_ulong) <<
                           3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteChar(mut sb: *mut sizebuf_t,
                                       mut val: libc::c_int) {
    MSG_WriteSBitLong(sb, val,
                      ((::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong) << 3 as libc::c_int) as
                          libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteByte(mut sb: *mut sizebuf_t,
                                       mut val: libc::c_int) {
    MSG_WriteUBitLong(sb, val as uint,
                      ((::std::mem::size_of::<byte>() as libc::c_ulong) <<
                           3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteShort(mut sb: *mut sizebuf_t,
                                        mut val: libc::c_int) {
    MSG_WriteSBitLong(sb, val,
                      ((::std::mem::size_of::<libc::c_short>() as
                            libc::c_ulong) << 3 as libc::c_int) as
                          libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteWord(mut sb: *mut sizebuf_t,
                                       mut val: libc::c_int) {
    MSG_WriteUBitLong(sb, val as uint,
                      ((::std::mem::size_of::<word>() as libc::c_ulong) <<
                           3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteLong(mut sb: *mut sizebuf_t,
                                       mut val: libc::c_int) {
    MSG_WriteSBitLong(sb, val,
                      ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                           << 3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDword(mut sb: *mut sizebuf_t,
                                        mut val: dword) {
    MSG_WriteUBitLong(sb, val,
                      ((::std::mem::size_of::<dword>() as libc::c_ulong) <<
                           3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteFloat(mut sb: *mut sizebuf_t,
                                        mut val: libc::c_float) {
    MSG_WriteBits(sb, &mut val as *mut libc::c_float as *const libc::c_void,
                  ((::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                       << 3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBytes(mut sb: *mut sizebuf_t,
                                        mut pBuf: *const libc::c_void,
                                        mut nBytes: libc::c_int) -> qboolean {
    return MSG_WriteBits(sb, pBuf, nBytes << 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteString(mut sb: *mut sizebuf_t,
                                         mut pStr: *const libc::c_char)
 -> qboolean {
    if !pStr.is_null() {
        loop  {
            MSG_WriteChar(sb, *pStr as libc::c_schar as libc::c_int);
            pStr = pStr.offset(1);
            if !(*pStr.offset(-(1 as libc::c_int as isize)) != 0) { break ; }
        }
    } else { MSG_WriteChar(sb, 0 as libc::c_int); }
    return ((*sb).bOverflow as u64 == 0) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadOneBit(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    if MSG_Overflow(sb, 1 as libc::c_int) as u64 == 0 {
        let mut value: libc::c_int =
            *(*sb).pData.offset(((*sb).iCurBit >> 3 as libc::c_int) as isize)
                as libc::c_int &
                (1 as libc::c_int) << ((*sb).iCurBit & 7 as libc::c_int);
        (*sb).iCurBit += 1;
        return (value != 0) as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadUBitLong(mut sb: *mut sizebuf_t,
                                          mut numbits: libc::c_int) -> uint {
    let mut idword1: libc::c_int = 0;
    let mut dword1: uint = 0;
    let mut ret: uint = 0;
    if numbits == 8 as libc::c_int {
        let mut leftBits: libc::c_int = MSG_GetNumBitsLeft(sb);
        if leftBits >= 0 as libc::c_int && leftBits < 8 as libc::c_int {
            return 0 as libc::c_int as uint
        }
        // end of message
    }
    if (*sb).iCurBit + numbits > (*sb).nDataBits {
        (*sb).bOverflow = true_0;
        (*sb).iCurBit = (*sb).nDataBits;
        return 0 as libc::c_int as uint
    }
    // Read the current dword.
    idword1 =
        (*sb).iCurBit >>
            5 as libc::c_int; // get the bits we're interested in.
    dword1 = *((*sb).pData as *mut uint).offset(idword1 as isize);
    dword1 >>= (*sb).iCurBit & 31 as libc::c_int;
    (*sb).iCurBit += numbits;
    ret = dword1;
    // Does it span this dword?
    if (*sb).iCurBit - 1 as libc::c_int >> 5 as libc::c_int == idword1 {
        if numbits != 32 as libc::c_int {
            ret &= ExtraMasks[numbits as usize]
        }
    } else {
        let mut nExtraBits: libc::c_int = (*sb).iCurBit & 31 as libc::c_int;
        let mut dword2: uint =
            *((*sb).pData as
                  *mut uint).offset((idword1 + 1 as libc::c_int) as isize) &
                ExtraMasks[nExtraBits as usize];
        // no need to mask since we hit the end of the dword.
		// shift the second dword's part into the high bits.
        ret |= dword2 << numbits - nExtraBits
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBitFloat(mut sb: *mut sizebuf_t)
 -> libc::c_float {
    let mut val: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut byte: libc::c_int = 0;
    if MSG_Overflow(sb, 32 as libc::c_int) as u64 != 0 { return 0.0f32 }
    bit = (*sb).iCurBit & 0x7 as libc::c_int;
    byte = (*sb).iCurBit >> 3 as libc::c_int;
    val = *(*sb).pData.offset(byte as isize) as libc::c_int >> bit;
    val |=
        (*(*sb).pData.offset((byte + 1 as libc::c_int) as isize) as
             libc::c_int) << 8 as libc::c_int - bit;
    val |=
        (*(*sb).pData.offset((byte + 2 as libc::c_int) as isize) as
             libc::c_int) << 16 as libc::c_int - bit;
    val |=
        (*(*sb).pData.offset((byte + 3 as libc::c_int) as isize) as
             libc::c_int) << 24 as libc::c_int - bit;
    if bit != 0 as libc::c_int {
        val |=
            (*(*sb).pData.offset((byte + 4 as libc::c_int) as isize) as
                 libc::c_int) << 32 as libc::c_int - bit
    }
    (*sb).iCurBit += 32 as libc::c_int;
    return *(&mut val as *mut libc::c_int as *mut libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBits(mut sb: *mut sizebuf_t,
                                      mut pOutData: *mut libc::c_void,
                                      mut nBits: libc::c_int) -> qboolean {
    let mut pOut: *mut byte = pOutData as *mut byte;
    let mut nBitsLeft: libc::c_int = nBits;
    // get output dword-aligned.
    while pOut as dword & 3 as libc::c_int as libc::c_uint !=
              0 as libc::c_int as libc::c_uint &&
              nBitsLeft >= 8 as libc::c_int {
        *pOut = MSG_ReadUBitLong(sb, 8 as libc::c_int) as byte;
        pOut = pOut.offset(1);
        nBitsLeft -= 8 as libc::c_int
    }
    // read dwords.
    while nBitsLeft >= 32 as libc::c_int {
        *(pOut as *mut dword) = MSG_ReadUBitLong(sb, 32 as libc::c_int);
        pOut =
            pOut.offset(::std::mem::size_of::<dword>() as libc::c_ulong as
                            isize);
        nBitsLeft -= 32 as libc::c_int
    }
    // read the remaining bytes.
    while nBitsLeft >= 8 as libc::c_int {
        *pOut = MSG_ReadUBitLong(sb, 8 as libc::c_int) as byte;
        pOut = pOut.offset(1);
        nBitsLeft -= 8 as libc::c_int
    }
    // read the remaining bits.
    if nBitsLeft != 0 { *pOut = MSG_ReadUBitLong(sb, nBitsLeft) as byte }
    return ((*sb).bOverflow as u64 == 0) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBitAngle(mut sb: *mut sizebuf_t,
                                          mut numbits: libc::c_int)
 -> libc::c_float {
    let mut fReturn: libc::c_float = 0.;
    let mut shift: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    shift = ((1 as libc::c_int) << numbits) as libc::c_float;
    i = MSG_ReadUBitLong(sb, numbits) as libc::c_int;
    fReturn = i as libc::c_float * (360.0f32 / shift);
    // clamp the finale angle
    if fReturn < -180.0f32 {
        fReturn += 360.0f32
    } else if fReturn > 180.0f32 { fReturn -= 360.0f32 }
    return fReturn;
}
// Append numbits least significant bits from data to the current bit stream
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadSBitLong(mut sb: *mut sizebuf_t,
                                          mut numbits: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    r = MSG_ReadUBitLong(sb, numbits - 1 as libc::c_int) as libc::c_int;
    // NOTE: it does this wierdness here so it's bit-compatible with regular integer data in the buffer.
	// (Some old code writes direct integers right into the buffer).
    sign = MSG_ReadOneBit(sb);
    if sign != 0 {
        r =
            ((1 as libc::c_uint) <<
                 numbits -
                     1 as
                         libc::c_int).wrapping_sub(r as
                                                       libc::c_uint).wrapping_neg()
                as libc::c_int
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBitLong(mut sb: *mut sizebuf_t,
                                         mut numbits: libc::c_int,
                                         mut bSigned: qboolean) -> uint {
    if bSigned as u64 != 0 { return MSG_ReadSBitLong(sb, numbits) as uint }
    return MSG_ReadUBitLong(sb, numbits);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadCmd(mut sb: *mut sizebuf_t,
                                     mut type_0: netsrc_t) -> libc::c_int {
    let mut cmd: libc::c_int =
        MSG_ReadUBitLong(sb,
                         ((::std::mem::size_of::<byte>() as libc::c_ulong) <<
                              3 as libc::c_int) as libc::c_int) as
            libc::c_int;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadChar(mut sb: *mut sizebuf_t) -> libc::c_int {
    return MSG_ReadSBitLong(sb,
                            ((::std::mem::size_of::<libc::c_char>() as
                                  libc::c_ulong) << 3 as libc::c_int) as
                                libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadByte(mut sb: *mut sizebuf_t) -> libc::c_int {
    return MSG_ReadUBitLong(sb,
                            ((::std::mem::size_of::<byte>() as libc::c_ulong)
                                 << 3 as libc::c_int) as libc::c_int) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadShort(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return MSG_ReadSBitLong(sb,
                            ((::std::mem::size_of::<libc::c_short>() as
                                  libc::c_ulong) << 3 as libc::c_int) as
                                libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadWord(mut sb: *mut sizebuf_t) -> libc::c_int {
    return MSG_ReadUBitLong(sb,
                            ((::std::mem::size_of::<word>() as libc::c_ulong)
                                 << 3 as libc::c_int) as libc::c_int) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadCoord(mut sb: *mut sizebuf_t)
 -> libc::c_float {
    // g-cont. we loose precision here but keep old size of coord variable!
    if host.features &
           ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        return MSG_ReadShort(sb) as libc::c_float
    }
    return MSG_ReadShort(sb) as libc::c_float * (1.0f32 / 8.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadVec3Coord(mut sb: *mut sizebuf_t,
                                           mut fa: *mut vec_t) {
    *fa.offset(0 as libc::c_int as isize) = MSG_ReadCoord(sb);
    *fa.offset(1 as libc::c_int as isize) = MSG_ReadCoord(sb);
    *fa.offset(2 as libc::c_int as isize) = MSG_ReadCoord(sb);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadVec3Angles(mut sb: *mut sizebuf_t,
                                            mut fa: *mut vec_t) {
    *fa.offset(0 as libc::c_int as isize) =
        MSG_ReadBitAngle(sb, 16 as libc::c_int);
    *fa.offset(1 as libc::c_int as isize) =
        MSG_ReadBitAngle(sb, 16 as libc::c_int);
    *fa.offset(2 as libc::c_int as isize) =
        MSG_ReadBitAngle(sb, 16 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadLong(mut sb: *mut sizebuf_t) -> libc::c_int {
    return MSG_ReadSBitLong(sb,
                            ((::std::mem::size_of::<libc::c_int>() as
                                  libc::c_ulong) << 3 as libc::c_int) as
                                libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDword(mut sb: *mut sizebuf_t) -> dword {
    return MSG_ReadUBitLong(sb,
                            ((::std::mem::size_of::<dword>() as libc::c_ulong)
                                 << 3 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadFloat(mut sb: *mut sizebuf_t)
 -> libc::c_float {
    let mut ret: libc::c_float = 0.;
    MSG_ReadBits(sb, &mut ret as *mut libc::c_float as *mut libc::c_void,
                 32 as libc::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBytes(mut sb: *mut sizebuf_t,
                                       mut pOut: *mut libc::c_void,
                                       mut nBytes: libc::c_int) -> qboolean {
    return MSG_ReadBits(sb, pOut, nBytes << 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadStringExt(mut sb: *mut sizebuf_t,
                                           mut bLine: qboolean)
 -> *mut libc::c_char {
    static mut string: [libc::c_char; 4096] = [0; 4096];
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    loop  {
        // use MSG_ReadByte so -1 is out of bounds
        c = MSG_ReadByte(sb);
        if c == 0 as libc::c_int { break ; }
        if bLine as libc::c_uint != 0 && c == '\n' as i32 { break ; }
        // translate all fmt spec to avoid crash bugs
		// NOTE: but game strings leave unchanged. see pfnWriteString for details
        if c == '%' as i32 { c = '.' as i32 } // terminator
        string[l as usize] = c as libc::c_char;
        l += 1;
        if !((l as libc::c_ulong) <
                 (::std::mem::size_of::<[libc::c_char; 4096]>() as
                      libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulong)) {
            break ;
        }
    }
    string[l as usize] = 0 as libc::c_int as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ExciseBits(mut sb: *mut sizebuf_t,
                                        mut startbit: libc::c_int,
                                        mut bitstoremove: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut endbit: libc::c_int = startbit + bitstoremove;
    let mut remaining_to_end: libc::c_int = (*sb).nDataBits - endbit;
    let mut temp: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    MSG_StartWriting(&mut temp, (*sb).pData as *mut libc::c_void,
                     MSG_GetMaxBytes(sb), startbit, -(1 as libc::c_int));
    MSG_SeekToBit(sb, endbit, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < remaining_to_end {
        MSG_WriteOneBit(&mut temp, MSG_ReadOneBit(sb));
        i += 1
    }
    MSG_SeekToBit(sb, startbit, 0 as libc::c_int);
    (*sb).nDataBits -= bitstoremove;
}
