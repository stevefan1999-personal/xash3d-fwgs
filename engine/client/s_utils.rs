#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type word = libc::c_ushort;
pub type C2RustUnnamed = libc::c_uint;
pub const WF_TOTALCOUNT: C2RustUnnamed = 3;
pub const WF_MPGDATA: C2RustUnnamed = 2;
pub const WF_PCMDATA: C2RustUnnamed = 1;
pub const WF_UNKNOWN: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavdata_t {
    pub rate: word,
    pub width: byte,
    pub channels: byte,
    pub loopStart: libc::c_int,
    pub samples: libc::c_int,
    pub type_0: uint,
    pub flags: uint,
    pub buffer: *mut byte,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub cache: *mut wavdata_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub hashNext: *mut sfx_s,
}
pub type sfx_t = sfx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct voxword_s {
    pub volume: libc::c_int,
    pub pitch: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub cbtrim: libc::c_int,
    pub fKeepCached: libc::c_int,
    pub samplefrac: libc::c_int,
    pub timecompress: libc::c_int,
    pub sfx: *mut sfx_t,
}
pub type voxword_t = voxword_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_s {
    pub name: [libc::c_char; 16],
    pub sfx: *mut sfx_t,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub origin: vec3_t,
    pub dist_mult: libc::c_float,
    pub master_vol: libc::c_int,
    pub isSentence: qboolean,
    pub basePitch: libc::c_int,
    pub pitch: libc::c_float,
    pub use_loop: qboolean,
    pub staticsound: qboolean,
    pub localsound: qboolean,
    pub pMixer: mixer_t,
    pub wordIndex: libc::c_int,
    pub currentWord: *mut mixer_t,
    pub words: [voxword_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixer_t {
    pub sample: libc::c_double,
    pub pData: *mut wavdata_t,
    pub forcedEndSample: libc::c_double,
    pub finished: qboolean,
}
pub type channel_t = channel_s;
//-----------------------------------------------------------------------------
// Purpose: Search backward for a zero crossing starting at sample
// Input  : sample - starting point
// Output : position of zero crossing
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn S_ZeroCrossingBefore(mut pWaveData: *mut wavdata_t,
                                              mut sample: libc::c_int)
 -> libc::c_int {
    if pWaveData.is_null() { return sample }
    if (*pWaveData).type_0 == WF_PCMDATA as libc::c_int as libc::c_uint {
        let mut sampleSize: libc::c_int = 0;
        sampleSize =
            (*pWaveData).width as libc::c_int *
                (*pWaveData).channels as libc::c_int;
        // this can never be zero -- other functions divide by this.
		// This should never happen, but avoid crashing
        if sampleSize <= 0 as libc::c_int { sampleSize = 1 as libc::c_int }
        if (*pWaveData).width as libc::c_int == 1 as libc::c_int {
            let mut pData: *mut libc::c_schar =
                (*pWaveData).buffer.offset((sample * sampleSize) as isize) as
                    *mut libc::c_schar;
            let mut zero: qboolean = false_0;
            if (*pWaveData).channels as libc::c_int == 1 as libc::c_int {
                while sample > 0 as libc::c_int && zero as u64 == 0 {
                    if (*pData as libc::c_int) < 2 as libc::c_int &&
                           *pData as libc::c_int > -(2 as libc::c_int) {
                        zero = true_0
                    } else { sample -= 1; pData = pData.offset(-1) }
                }
            } else {
                while sample > 0 as libc::c_int && zero as u64 == 0 {
                    if (*pData as libc::c_int) < 2 as libc::c_int &&
                           *pData as libc::c_int > -(2 as libc::c_int) &&
                           ((*pData.offset(1 as libc::c_int as isize) as
                                 libc::c_int) < 2 as libc::c_int &&
                                *pData.offset(1 as libc::c_int as isize) as
                                    libc::c_int > -(2 as libc::c_int)) {
                        zero = true_0
                    } else { sample -= 1; pData = pData.offset(-1) }
                }
            }
        } else {
            let mut pData_0: *mut libc::c_short =
                (*pWaveData).buffer.offset((sample * sampleSize) as isize) as
                    *mut libc::c_short;
            let mut zero_0: qboolean = false_0;
            if (*pWaveData).channels as libc::c_int == 1 as libc::c_int {
                while sample > 0 as libc::c_int && zero_0 as u64 == 0 {
                    if (*pData_0 as libc::c_int) < 512 as libc::c_int &&
                           *pData_0 as libc::c_int > -(512 as libc::c_int) {
                        zero_0 = true_0
                    } else { pData_0 = pData_0.offset(-1); sample -= 1 }
                }
            } else {
                while sample > 0 as libc::c_int && zero_0 as u64 == 0 {
                    if (*pData_0 as libc::c_int) < 512 as libc::c_int &&
                           *pData_0 as libc::c_int > -(512 as libc::c_int) &&
                           ((*pData_0.offset(1 as libc::c_int as isize) as
                                 libc::c_int) < 512 as libc::c_int &&
                                *pData_0.offset(1 as libc::c_int as isize) as
                                    libc::c_int > -(512 as libc::c_int)) {
                        zero_0 = true_0
                    } else { sample -= 1; pData_0 = pData_0.offset(-1) }
                }
            }
        }
    }
    return sample;
}
//-----------------------------------------------------------------------------
// Purpose: Search forward for a zero crossing
// Input  : sample - starting point
// Output : position of found zero crossing
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn S_ZeroCrossingAfter(mut pWaveData: *mut wavdata_t,
                                             mut sample: libc::c_int)
 -> libc::c_int {
    if pWaveData.is_null() { return sample }
    if (*pWaveData).type_0 == WF_PCMDATA as libc::c_int as libc::c_uint {
        let mut sampleSize: libc::c_int = 0;
        sampleSize =
            (*pWaveData).width as libc::c_int *
                (*pWaveData).channels as libc::c_int;
        // this can never be zero -- other functions divide by this.
		// This should never happen, but avoid crashing
        if sampleSize <= 0 as libc::c_int { sampleSize = 1 as libc::c_int }
        if (*pWaveData).width as libc::c_int == 1 as libc::c_int {
            // 8-bit
            let mut pData: *mut libc::c_schar =
                (*pWaveData).buffer.offset((sample * sampleSize) as isize) as
                    *mut libc::c_schar;
            let mut zero: qboolean = false_0;
            if (*pWaveData).channels as libc::c_int == 1 as libc::c_int {
                while sample < (*pWaveData).samples && zero as u64 == 0 {
                    if (*pData as libc::c_int) < 2 as libc::c_int &&
                           *pData as libc::c_int > -(2 as libc::c_int) {
                        zero = true_0
                    } else { sample += 1; pData = pData.offset(1) }
                }
            } else {
                while sample < (*pWaveData).samples && zero as u64 == 0 {
                    if (*pData as libc::c_int) < 2 as libc::c_int &&
                           *pData as libc::c_int > -(2 as libc::c_int) &&
                           ((*pData.offset(1 as libc::c_int as isize) as
                                 libc::c_int) < 2 as libc::c_int &&
                                *pData.offset(1 as libc::c_int as isize) as
                                    libc::c_int > -(2 as libc::c_int)) {
                        zero = true_0
                    } else { sample += 1; pData = pData.offset(1) }
                }
            }
        } else {
            let mut pData_0: *mut libc::c_short =
                (*pWaveData).buffer.offset((sample * sampleSize) as isize) as
                    *mut libc::c_short;
            let mut zero_0: qboolean = false_0;
            if (*pWaveData).channels as libc::c_int == 1 as libc::c_int {
                while sample > 0 as libc::c_int && zero_0 as u64 == 0 {
                    if (*pData_0 as libc::c_int) < 512 as libc::c_int &&
                           *pData_0 as libc::c_int > -(512 as libc::c_int) {
                        zero_0 = true_0
                    } else { pData_0 = pData_0.offset(1); sample += 1 }
                }
            } else {
                while sample > 0 as libc::c_int && zero_0 as u64 == 0 {
                    if (*pData_0 as libc::c_int) < 512 as libc::c_int &&
                           *pData_0 as libc::c_int > -(512 as libc::c_int) &&
                           ((*pData_0.offset(1 as libc::c_int as isize) as
                                 libc::c_int) < 512 as libc::c_int &&
                                *pData_0.offset(1 as libc::c_int as isize) as
                                    libc::c_int > -(512 as libc::c_int)) {
                        zero_0 = true_0
                    } else { sample += 1; pData_0 = pData_0.offset(1) }
                }
            }
        }
    }
    return sample;
}
//-----------------------------------------------------------------------------
// Purpose: wrap the position wrt looping
// Input  : samplePosition - absolute position
// Output : int - looped position
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn S_ConvertLoopedPosition(mut pSource: *mut wavdata_t,
                                                 mut samplePosition:
                                                     libc::c_int,
                                                 mut use_loop: qboolean)
 -> libc::c_int {
    // if the wave is looping and we're past the end of the sample
	// convert to a position within the loop
	// At the end of the loop, we return a short buffer, and subsequent call
	// will loop back and get the rest of the buffer
    if (*pSource).loopStart >= 0 as libc::c_int &&
           samplePosition >= (*pSource).samples &&
           use_loop as libc::c_uint != 0 {
        // size of loop
        let mut loopSize: libc::c_int =
            (*pSource).samples - (*pSource).loopStart;
        // subtract off starting bit of the wave
        samplePosition -= (*pSource).loopStart;
        if loopSize != 0 {
            // "real" position in memory (mod off extra loops)
            samplePosition = (*pSource).loopStart + samplePosition % loopSize
        }
    }
    return samplePosition;
}
#[no_mangle]
pub unsafe extern "C" fn S_GetOutputData(mut pSource: *mut wavdata_t,
                                         mut pData: *mut *mut libc::c_void,
                                         mut samplePosition: libc::c_int,
                                         mut sampleCount: libc::c_int,
                                         mut use_loop: qboolean)
 -> libc::c_int {
    let mut totalSampleCount: libc::c_int = 0;
    let mut sampleSize: libc::c_int = 0;
    // handle position looping
    samplePosition =
        S_ConvertLoopedPosition(pSource, samplePosition, use_loop);
    // how many samples are available (linearly not counting looping)
    totalSampleCount = (*pSource).samples - samplePosition;
    // may be asking for a sample out of range, clip at zero
    if totalSampleCount < 0 as libc::c_int {
        totalSampleCount = 0 as libc::c_int
    }
    // clip max output samples to max available
    if sampleCount > totalSampleCount { sampleCount = totalSampleCount }
    sampleSize =
        (*pSource).width as libc::c_int * (*pSource).channels as libc::c_int;
    // this can never be zero -- other functions divide by this.
	// This should never happen, but avoid crashing
    if sampleSize <= 0 as libc::c_int { sampleSize = 1 as libc::c_int }
    // byte offset in sample database
    samplePosition *= sampleSize;
    // if we are returning some samples, store the pointer
    if sampleCount != 0 {
        *pData =
            (*pSource).buffer.offset(samplePosition as isize) as
                *mut libc::c_void
    }
    return sampleCount;
}
// move the current position to newPosition
#[no_mangle]
pub unsafe extern "C" fn S_SetSampleStart(mut pChan: *mut channel_t,
                                          mut pSource: *mut wavdata_t,
                                          mut newPosition: libc::c_int) {
    if !pSource.is_null() {
        newPosition = S_ZeroCrossingAfter(pSource, newPosition)
    }
    (*pChan).pMixer.sample = newPosition as libc::c_double;
}
// end playback at newEndPosition
#[no_mangle]
pub unsafe extern "C" fn S_SetSampleEnd(mut pChan: *mut channel_t,
                                        mut pSource: *mut wavdata_t,
                                        mut newEndPosition: libc::c_int) {
    // forced end of zero means play the whole sample
    if newEndPosition == 0 { newEndPosition = 1 as libc::c_int }
    if !pSource.is_null() {
        newEndPosition = S_ZeroCrossingBefore(pSource, newEndPosition)
    }
    // past current position?  limit.
    if (newEndPosition as libc::c_double) < (*pChan).pMixer.sample {
        newEndPosition = (*pChan).pMixer.sample as libc::c_int
    }
    (*pChan).pMixer.forcedEndSample = newEndPosition as libc::c_double;
}
