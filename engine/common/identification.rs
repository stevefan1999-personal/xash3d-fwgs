#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FS_WriteFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn FS_LoadDirectFile(path: *const libc::c_char,
                         filesizeptr: *mut fs_offset_t) -> *mut byte;
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn MD5Final(digest: *mut byte, ctx: *mut MD5Context_t);
    #[no_mangle]
    fn MD5Update(ctx: *mut MD5Context_t, buf: *const byte, len: uint);
    #[no_mangle]
    fn MD5Init(ctx: *mut MD5Context_t);
    #[no_mangle]
    fn CRC32_ProcessBuffer(pulCRC: *mut dword, pBuffer: *const libc::c_void,
                           nBuffer: libc::c_int);
    #[no_mangle]
    fn CRC32_Init(pulCRC: *mut dword);
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_tolower(in_0: libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type __uint64_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint64_t = __uint64_t;
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type fs_offset_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5Context_t {
    pub buf: [uint; 4],
    pub bits: [uint; 2],
    pub in_0: [uint; 16],
}
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
/*
==========================================================

simple 64-bit one-hash-func bloom filter
should be enough to determine if device exist in identifier

==========================================================
*/
pub type bloomfilter_t = uint64_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
/*
identification.c - unique id generation
Copyright (C) 2017 mittorn

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
static mut id_md5: [libc::c_char; 33] = [0; 33];
static mut id_customid: [libc::c_char; 256] = [0; 256];
static mut id: bloomfilter_t = 0;
#[no_mangle]
pub unsafe extern "C" fn BloomFilter_Process(mut buffer: *const libc::c_char,
                                             mut size: libc::c_int)
 -> bloomfilter_t {
    let mut crc32: dword = 0;
    let mut value: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    if size <= 0 as libc::c_int || size > 512 as libc::c_int {
        return 0 as libc::c_int as bloomfilter_t
    }
    CRC32_Init(&mut crc32);
    CRC32_ProcessBuffer(&mut crc32, buffer as *const libc::c_void, size);
    while crc32 != 0 {
        value |=
            (1 as libc::c_int as uint64_t) <<
                (crc32 &
                     ((1 as libc::c_uint) <<
                          6 as
                              libc::c_int).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint));
        crc32 = crc32 >> 6 as libc::c_int
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn BloomFilter_ProcessStr(mut buffer:
                                                    *const libc::c_char)
 -> bloomfilter_t {
    return BloomFilter_Process(buffer, Q_strlen(buffer) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BloomFilter_Weight(mut value: bloomfilter_t)
 -> uint {
    let mut weight: libc::c_int = 0 as libc::c_int;
    while value != 0 {
        if value & 1 as libc::c_int as libc::c_ulong != 0 { weight += 1 }
        value = value >> 1 as libc::c_int
    }
    return weight as uint;
}
#[no_mangle]
pub unsafe extern "C" fn BloomFilter_ContainsString(mut filter: bloomfilter_t,
                                                    mut str:
                                                        *const libc::c_char)
 -> qboolean {
    let mut value: bloomfilter_t = BloomFilter_ProcessStr(str);
    return (filter & value == value) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn ID_BloomFilter_f() {
    let mut value: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        value |= BloomFilter_ProcessStr(Cmd_Argv(i));
        i += 1
    }
    Con_Printf(b"%d %016llX\n\x00" as *const u8 as *const libc::c_char,
               BloomFilter_Weight(value), value);
    // test
	// for( i = 1; i < Cmd_Argc(); i++ )
	//	Msg( "%s: %d\n", Cmd_Argv( i ), BloomFilter_ContainsString( value, Cmd_Argv( i ) ) );
}
#[no_mangle]
pub unsafe extern "C" fn ID_VerifyHEX(mut hex: *const libc::c_char)
 -> qboolean {
    let mut chars: uint = 0 as libc::c_int as uint; // detect 11:22...
    let mut prev: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut monotonic: qboolean = true_0;
    let mut weight: libc::c_int = 0 as libc::c_int;
    loop  {
        let fresh0 = hex;
        hex = hex.offset(1);
        if !(*fresh0 != 0) { break ; }
        let mut ch: libc::c_char = Q_tolower(*hex);
        if ch as libc::c_int >= 'a' as i32 && ch as libc::c_int <= 'f' as i32
               ||
               ch as libc::c_int >= '0' as i32 &&
                   ch as libc::c_int <= '9' as i32 {
            if prev as libc::c_int != 0 &&
                   ((ch as libc::c_int - prev as libc::c_int) <
                        -(1 as libc::c_int) ||
                        ch as libc::c_int - prev as libc::c_int >
                            1 as libc::c_int) {
                monotonic = false_0
            }
            if ch as libc::c_int >= 'a' as i32 {
                chars |=
                    ((1 as libc::c_int) <<
                         ch as libc::c_int - 'a' as i32 + 10 as libc::c_int)
                        as libc::c_uint
            } else {
                chars |=
                    ((1 as libc::c_int) << ch as libc::c_int - '0' as i32) as
                        libc::c_uint
            }
            prev = ch
        }
    }
    if monotonic as u64 != 0 { return false_0 }
    while chars != 0 {
        if chars & 1 as libc::c_int as libc::c_uint != 0 { weight += 1 }
        chars = chars >> 1 as libc::c_int;
        if weight > 2 as libc::c_int { return true_0 }
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn ID_VerifyHEX_f() {
    if ID_VerifyHEX(Cmd_Argv(1 as libc::c_int)) as u64 != 0 {
        Con_Printf(b"Good\n\x00" as *const u8 as *const libc::c_char);
    } else { Con_Printf(b"Bad\n\x00" as *const u8 as *const libc::c_char); };
}
#[no_mangle]
pub unsafe extern "C" fn ID_ProcessCPUInfo(mut value: *mut bloomfilter_t)
 -> qboolean {
    let mut cpuinfofd: libc::c_int =
        open(b"/proc/cpuinfo\x00" as *const u8 as *const libc::c_char,
             0 as libc::c_int);
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pbuf2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if cpuinfofd < 0 as libc::c_int { return false_0 }
    ret =
        read(cpuinfofd, buffer.as_mut_ptr() as *mut libc::c_void,
             1023 as libc::c_int as size_t) as libc::c_int;
    if ret < 0 as libc::c_int { return false_0 }
    close(cpuinfofd);
    buffer[ret as usize] = 0 as libc::c_int as libc::c_char;
    if ret == 0 { return false_0 }
    pbuf =
        Q_strstr(buffer.as_mut_ptr(),
                 b"Serial\x00" as *const u8 as *const libc::c_char);
    if pbuf.is_null() { return false_0 }
    pbuf = pbuf.offset(6 as libc::c_int as isize);
    pbuf2 = Q_strchr(pbuf, '\n' as i32 as libc::c_char);
    if !pbuf2.is_null() {
        *pbuf2 = 0 as libc::c_int as libc::c_char
    } else { pbuf2 = pbuf.offset(Q_strlen(pbuf) as isize) }
    if ID_VerifyHEX(pbuf) as u64 == 0 { return false_0 }
    *value |=
        BloomFilter_Process(pbuf,
                            pbuf2.wrapping_offset_from(pbuf) as libc::c_long
                                as libc::c_int);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn ID_ValidateNetDevice(mut dev: *const libc::c_char)
 -> qboolean {
    let mut prefix: *const libc::c_char =
        b"/sys/class/net\x00" as *const u8 as *const libc::c_char;
    let mut pfile: *mut byte = 0 as *mut byte;
    let mut assignType: libc::c_int = 0;
    // These devices are fake, their mac address is generated each boot, while assign_type is 0
    if Q_strnicmp(dev, b"ccmni\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                      as libc::c_int) == 0 ||
           Q_strnicmp(dev, b"ifb\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 4]>() as
                          libc::c_ulong as libc::c_int) == 0 {
        return false_0
    }
    pfile =
        FS_LoadDirectFile(va(b"%s/%s/addr_assign_type\x00" as *const u8 as
                                 *const libc::c_char, prefix, dev),
                          0 as *mut fs_offset_t);
    // if NULL, it may be old kernel
    if !pfile.is_null() {
        assignType = Q_atoi(pfile as *mut libc::c_char);
        _Mem_Free(pfile as *mut libc::c_void,
                  b"../engine/common/identification.c\x00" as *const u8 as
                      *const libc::c_char, 219 as libc::c_int);
        // check is MAC address is constant
        if assignType != 0 as libc::c_int { return false_0 }
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn ID_ProcessNetDevices(mut value: *mut bloomfilter_t)
 -> libc::c_int {
    let mut prefix: *const libc::c_char =
        b"/sys/class/net\x00" as *const u8 as *const libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut count: libc::c_int = 0 as libc::c_int;
    dir = opendir(prefix);
    if dir.is_null() { return 0 as libc::c_int }
    loop  {
        entry = readdir(dir);
        if !(!entry.is_null() &&
                 BloomFilter_Weight(*value) <
                     30 as libc::c_int as libc::c_uint) {
            break ;
        }
        if Q_strncmp((*entry).d_name.as_mut_ptr(),
                     b".\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 ||
               Q_strncmp((*entry).d_name.as_mut_ptr(),
                         b"..\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
            continue ;
        }
        if ID_ValidateNetDevice((*entry).d_name.as_mut_ptr()) as u64 == 0 {
            continue ;
        }
        count =
            (count as
                 libc::c_uint).wrapping_add(ID_ProcessFile(value,
                                                           va(b"%s/%s/address\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              prefix,
                                                              (*entry).d_name.as_mut_ptr()))
                                                as libc::c_uint) as
                libc::c_int as libc::c_int
    }
    closedir(dir);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn ID_CheckNetDevices(mut value: bloomfilter_t)
 -> libc::c_int {
    let mut prefix: *const libc::c_char =
        b"/sys/class/net\x00" as *const u8 as *const libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut filter: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    dir = opendir(prefix);
    if dir.is_null() { return 0 as libc::c_int }
    loop  {
        entry = readdir(dir);
        if entry.is_null() { break ; }
        if Q_strncmp((*entry).d_name.as_mut_ptr(),
                     b".\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 ||
               Q_strncmp((*entry).d_name.as_mut_ptr(),
                         b"..\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
            continue ;
        }
        if ID_ValidateNetDevice((*entry).d_name.as_mut_ptr()) as u64 == 0 {
            continue ;
        }
        if ID_ProcessFile(&mut filter,
                          va(b"%s/%s/address\x00" as *const u8 as
                                 *const libc::c_char, prefix,
                             (*entry).d_name.as_mut_ptr())) as u64 != 0 {
            count += (value & filter == filter) as libc::c_int;
            filter = 0 as libc::c_int as bloomfilter_t
        }
    }
    closedir(dir);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn ID_TestCPUInfo_f() {
    let mut value: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    if ID_ProcessCPUInfo(&mut value) as u64 != 0 {
        Con_Printf(b"Got %016llX\n\x00" as *const u8 as *const libc::c_char,
                   value);
    } else {
        Con_Printf(b"Could not get serial\n\x00" as *const u8 as
                       *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ID_ProcessFile(mut value: *mut bloomfilter_t,
                                        mut path: *const libc::c_char)
 -> qboolean {
    let mut fd: libc::c_int = open(path, 0 as libc::c_int);
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut ret: libc::c_int = 0;
    if fd < 0 as libc::c_int { return false_0 }
    ret =
        read(fd, buffer.as_mut_ptr() as *mut libc::c_void,
             255 as libc::c_int as size_t) as libc::c_int;
    if ret < 0 as libc::c_int { return false_0 }
    close(fd);
    if ret == 0 { return false_0 }
    buffer[ret as usize] = 0 as libc::c_int as libc::c_char;
    if ID_VerifyHEX(buffer.as_mut_ptr()) as u64 == 0 { return false_0 }
    *value |= BloomFilter_Process(buffer.as_mut_ptr(), ret);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn ID_ProcessFiles(mut value: *mut bloomfilter_t,
                                         mut prefix: *const libc::c_char,
                                         mut postfix: *const libc::c_char)
 -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut count: libc::c_int = 0 as libc::c_int;
    dir = opendir(prefix);
    if dir.is_null() { return 0 as libc::c_int }
    loop  {
        entry = readdir(dir);
        if !(!entry.is_null() &&
                 BloomFilter_Weight(*value) <
                     30 as libc::c_int as libc::c_uint) {
            break ;
        }
        if Q_strncmp((*entry).d_name.as_mut_ptr(),
                     b".\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 ||
               Q_strncmp((*entry).d_name.as_mut_ptr(),
                         b"..\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
            continue ;
        }
        count =
            (count as
                 libc::c_uint).wrapping_add(ID_ProcessFile(value,
                                                           va(b"%s/%s/%s\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              prefix,
                                                              (*entry).d_name.as_mut_ptr(),
                                                              postfix)) as
                                                libc::c_uint) as libc::c_int
                as libc::c_int
    }
    closedir(dir);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn ID_CheckFiles(mut value: bloomfilter_t,
                                       mut prefix: *const libc::c_char,
                                       mut postfix: *const libc::c_char)
 -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut filter: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    dir = opendir(prefix);
    if dir.is_null() { return 0 as libc::c_int }
    loop  {
        entry = readdir(dir);
        if entry.is_null() { break ; }
        if Q_strncmp((*entry).d_name.as_mut_ptr(),
                     b".\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 ||
               Q_strncmp((*entry).d_name.as_mut_ptr(),
                         b"..\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
            continue ;
        }
        if ID_ProcessFile(&mut filter,
                          va(b"%s/%s/%s\x00" as *const u8 as
                                 *const libc::c_char, prefix,
                             (*entry).d_name.as_mut_ptr(), postfix)) as u64 !=
               0 {
            count += (value & filter == filter) as libc::c_int;
            filter = 0 as libc::c_int as bloomfilter_t
        }
    }
    closedir(dir);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn ID_GenerateRawId() -> bloomfilter_t {
    let mut value: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    count =
        (count as
             libc::c_uint).wrapping_add(ID_ProcessCPUInfo(&mut value) as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    count +=
        ID_ProcessFiles(&mut value,
                        b"/sys/block\x00" as *const u8 as *const libc::c_char,
                        b"device/cid\x00" as *const u8 as
                            *const libc::c_char);
    count += ID_ProcessNetDevices(&mut value);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn ID_CheckRawId(mut filter: bloomfilter_t) -> uint {
    let mut value: bloomfilter_t = 0 as libc::c_int as bloomfilter_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    count += ID_CheckNetDevices(filter);
    count +=
        ID_CheckFiles(filter,
                      b"/sys/block\x00" as *const u8 as *const libc::c_char,
                      b"device/cid\x00" as *const u8 as *const libc::c_char);
    if ID_ProcessCPUInfo(&mut value) as u64 != 0 {
        count += (filter & value == value) as libc::c_int
    }
    return count as uint;
}
#[no_mangle]
pub unsafe extern "C" fn ID_Check() {
    let mut weight: uint = BloomFilter_Weight(id);
    let mut mincount: uint = weight >> 2 as libc::c_int;
    if mincount < 1 as libc::c_int as libc::c_uint {
        mincount = 1 as libc::c_int as uint
    }
    if weight > (30 as libc::c_int + 6 as libc::c_int) as libc::c_uint {
        id = 0 as libc::c_int as bloomfilter_t;
        return
    }
    if ID_CheckRawId(id) < mincount {
        id = 0 as libc::c_int as bloomfilter_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ID_GetMD5() -> *const libc::c_char {
    if id_customid[0 as libc::c_int as usize] != 0 {
        return id_customid.as_mut_ptr()
    }
    return id_md5.as_mut_ptr();
}
/*
===============
ID_SetCustomClientID

===============
*/
#[no_mangle]
pub unsafe extern "C" fn ID_SetCustomClientID(mut id_0: *const libc::c_char) {
    if id_0.is_null() { return }
    Q_strncpy(id_customid.as_mut_ptr(), id_0,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ID_Init() {
    let mut hash: MD5Context_t =
        {
            let mut init =
                MD5Context_t{buf: [0 as libc::c_int as uint, 0, 0, 0],
                             bits: [0; 2],
                             in_0: [0; 16],};
            init
        };
    let mut md5: [byte; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    Cmd_AddRestrictedCommand(b"bloomfilter\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(ID_BloomFilter_f as
                                      unsafe extern "C" fn() -> ()),
                             b"print bloomfilter raw value of arguments set\x00"
                                 as *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"verifyhex\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(ID_VerifyHEX_f as
                                      unsafe extern "C" fn() -> ()),
                             b"check if id source seems to be fake\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"testcpuinfo\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(ID_TestCPUInfo_f as
                                      unsafe extern "C" fn() -> ()),
                             b"try read cpu serial\x00" as *const u8 as
                                 *const libc::c_char);
    let mut home: *const libc::c_char =
        getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
    if if home.is_null() || *home == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        let mut cfg: *mut FILE =
            fopen(va(b"%s/.config/.xash_id\x00" as *const u8 as
                         *const libc::c_char, home),
                  b"r\x00" as *const u8 as *const libc::c_char);
        if cfg.is_null() {
            cfg =
                fopen(va(b"%s/.local/.xash_id\x00" as *const u8 as
                             *const libc::c_char, home),
                      b"r\x00" as *const u8 as *const libc::c_char)
        }
        if cfg.is_null() {
            cfg =
                fopen(va(b"%s/.xash_id\x00" as *const u8 as
                             *const libc::c_char, home),
                      b"r\x00" as *const u8 as *const libc::c_char)
        }
        if !cfg.is_null() {
            if fscanf(cfg, b"%016llX\x00" as *const u8 as *const libc::c_char,
                      &mut id as *mut bloomfilter_t) > 0 as libc::c_int {
                id ^= 0x10331c2dce4c91db as libc::c_long as libc::c_ulong;
                ID_Check();
            }
            fclose(cfg);
        }
    }
    if id == 0 {
        let mut buf: *const libc::c_char =
            FS_LoadFile(b".xash_id\x00" as *const u8 as *const libc::c_char,
                        0 as *mut fs_offset_t, false_0) as
                *const libc::c_char;
        if !buf.is_null() {
            sscanf(buf, b"%016llX\x00" as *const u8 as *const libc::c_char,
                   &mut id as *mut bloomfilter_t);
            id ^= 0x7ffc48fbac1711f1 as libc::c_long as libc::c_ulong;
            ID_Check();
        }
    }
    if id == 0 { id = ID_GenerateRawId() }
    MD5Init(&mut hash);
    MD5Update(&mut hash, &mut id as *mut bloomfilter_t as *mut byte,
              ::std::mem::size_of::<bloomfilter_t>() as libc::c_ulong as
                  uint);
    MD5Final(md5.as_mut_ptr(), &mut hash);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        Q_sprintf(&mut *id_md5.as_mut_ptr().offset((i * 2 as libc::c_int) as
                                                       isize) as
                      *mut libc::c_char,
                  b"%02hhx\x00" as *const u8 as *const libc::c_char,
                  md5[i as usize] as libc::c_int);
        i += 1
    }
    let mut home_0: *const libc::c_char =
        getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
    if if home_0.is_null() || *home_0 == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        let mut cfg_0: *mut FILE =
            fopen(va(b"%s/.config/.xash_id\x00" as *const u8 as
                         *const libc::c_char, home_0),
                  b"w\x00" as *const u8 as *const libc::c_char);
        if cfg_0.is_null() {
            cfg_0 =
                fopen(va(b"%s/.local/.xash_id\x00" as *const u8 as
                             *const libc::c_char, home_0),
                      b"w\x00" as *const u8 as *const libc::c_char)
        }
        if cfg_0.is_null() {
            cfg_0 =
                fopen(va(b"%s/.xash_id\x00" as *const u8 as
                             *const libc::c_char, home_0),
                      b"w\x00" as *const u8 as *const libc::c_char)
        }
        if !cfg_0.is_null() {
            fprintf(cfg_0, b"%016llX\x00" as *const u8 as *const libc::c_char,
                    id ^ 0x10331c2dce4c91db as libc::c_long as libc::c_ulong);
            fclose(cfg_0);
        }
    }
    FS_WriteFile(b".xash_id\x00" as *const u8 as *const libc::c_char,
                 va(b"%016llX\x00" as *const u8 as *const libc::c_char,
                    id ^ 0x7ffc48fbac1711f1 as libc::c_long as libc::c_ulong)
                     as *const libc::c_void,
                 16 as libc::c_int as fs_offset_t);
}
