#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn Q_tolower(in_0: libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn COM_Hex2String(hex: uint8_t, str: *mut libc::c_char);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type uint = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type byte = libc::c_uchar;
pub type dword = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5Context_t {
    pub buf: [uint; 4],
    pub bits: [uint; 2],
    pub in_0: [uint; 16],
}
static mut crc32table: [dword; 256] =
    [0 as libc::c_int as dword, 0x77073096 as libc::c_int as dword,
     0xee0e612c as libc::c_uint, 0x990951ba as libc::c_uint,
     0x76dc419 as libc::c_int as dword, 0x706af48f as libc::c_int as dword,
     0xe963a535 as libc::c_uint, 0x9e6495a3 as libc::c_uint,
     0xedb8832 as libc::c_int as dword, 0x79dcb8a4 as libc::c_int as dword,
     0xe0d5e91e as libc::c_uint, 0x97d2d988 as libc::c_uint,
     0x9b64c2b as libc::c_int as dword, 0x7eb17cbd as libc::c_int as dword,
     0xe7b82d07 as libc::c_uint, 0x90bf1d91 as libc::c_uint,
     0x1db71064 as libc::c_int as dword, 0x6ab020f2 as libc::c_int as dword,
     0xf3b97148 as libc::c_uint, 0x84be41de as libc::c_uint,
     0x1adad47d as libc::c_int as dword, 0x6ddde4eb as libc::c_int as dword,
     0xf4d4b551 as libc::c_uint, 0x83d385c7 as libc::c_uint,
     0x136c9856 as libc::c_int as dword, 0x646ba8c0 as libc::c_int as dword,
     0xfd62f97a as libc::c_uint, 0x8a65c9ec as libc::c_uint,
     0x14015c4f as libc::c_int as dword, 0x63066cd9 as libc::c_int as dword,
     0xfa0f3d63 as libc::c_uint, 0x8d080df5 as libc::c_uint,
     0x3b6e20c8 as libc::c_int as dword, 0x4c69105e as libc::c_int as dword,
     0xd56041e4 as libc::c_uint, 0xa2677172 as libc::c_uint,
     0x3c03e4d1 as libc::c_int as dword, 0x4b04d447 as libc::c_int as dword,
     0xd20d85fd as libc::c_uint, 0xa50ab56b as libc::c_uint,
     0x35b5a8fa as libc::c_int as dword, 0x42b2986c as libc::c_int as dword,
     0xdbbbc9d6 as libc::c_uint, 0xacbcf940 as libc::c_uint,
     0x32d86ce3 as libc::c_int as dword, 0x45df5c75 as libc::c_int as dword,
     0xdcd60dcf as libc::c_uint, 0xabd13d59 as libc::c_uint,
     0x26d930ac as libc::c_int as dword, 0x51de003a as libc::c_int as dword,
     0xc8d75180 as libc::c_uint, 0xbfd06116 as libc::c_uint,
     0x21b4f4b5 as libc::c_int as dword, 0x56b3c423 as libc::c_int as dword,
     0xcfba9599 as libc::c_uint, 0xb8bda50f as libc::c_uint,
     0x2802b89e as libc::c_int as dword, 0x5f058808 as libc::c_int as dword,
     0xc60cd9b2 as libc::c_uint, 0xb10be924 as libc::c_uint,
     0x2f6f7c87 as libc::c_int as dword, 0x58684c11 as libc::c_int as dword,
     0xc1611dab as libc::c_uint, 0xb6662d3d as libc::c_uint,
     0x76dc4190 as libc::c_int as dword, 0x1db7106 as libc::c_int as dword,
     0x98d220bc as libc::c_uint, 0xefd5102a as libc::c_uint,
     0x71b18589 as libc::c_int as dword, 0x6b6b51f as libc::c_int as dword,
     0x9fbfe4a5 as libc::c_uint, 0xe8b8d433 as libc::c_uint,
     0x7807c9a2 as libc::c_int as dword, 0xf00f934 as libc::c_int as dword,
     0x9609a88e as libc::c_uint, 0xe10e9818 as libc::c_uint,
     0x7f6a0dbb as libc::c_int as dword, 0x86d3d2d as libc::c_int as dword,
     0x91646c97 as libc::c_uint, 0xe6635c01 as libc::c_uint,
     0x6b6b51f4 as libc::c_int as dword, 0x1c6c6162 as libc::c_int as dword,
     0x856530d8 as libc::c_uint, 0xf262004e as libc::c_uint,
     0x6c0695ed as libc::c_int as dword, 0x1b01a57b as libc::c_int as dword,
     0x8208f4c1 as libc::c_uint, 0xf50fc457 as libc::c_uint,
     0x65b0d9c6 as libc::c_int as dword, 0x12b7e950 as libc::c_int as dword,
     0x8bbeb8ea as libc::c_uint, 0xfcb9887c as libc::c_uint,
     0x62dd1ddf as libc::c_int as dword, 0x15da2d49 as libc::c_int as dword,
     0x8cd37cf3 as libc::c_uint, 0xfbd44c65 as libc::c_uint,
     0x4db26158 as libc::c_int as dword, 0x3ab551ce as libc::c_int as dword,
     0xa3bc0074 as libc::c_uint, 0xd4bb30e2 as libc::c_uint,
     0x4adfa541 as libc::c_int as dword, 0x3dd895d7 as libc::c_int as dword,
     0xa4d1c46d as libc::c_uint, 0xd3d6f4fb as libc::c_uint,
     0x4369e96a as libc::c_int as dword, 0x346ed9fc as libc::c_int as dword,
     0xad678846 as libc::c_uint, 0xda60b8d0 as libc::c_uint,
     0x44042d73 as libc::c_int as dword, 0x33031de5 as libc::c_int as dword,
     0xaa0a4c5f as libc::c_uint, 0xdd0d7cc9 as libc::c_uint,
     0x5005713c as libc::c_int as dword, 0x270241aa as libc::c_int as dword,
     0xbe0b1010 as libc::c_uint, 0xc90c2086 as libc::c_uint,
     0x5768b525 as libc::c_int as dword, 0x206f85b3 as libc::c_int as dword,
     0xb966d409 as libc::c_uint, 0xce61e49f as libc::c_uint,
     0x5edef90e as libc::c_int as dword, 0x29d9c998 as libc::c_int as dword,
     0xb0d09822 as libc::c_uint, 0xc7d7a8b4 as libc::c_uint,
     0x59b33d17 as libc::c_int as dword, 0x2eb40d81 as libc::c_int as dword,
     0xb7bd5c3b as libc::c_uint, 0xc0ba6cad as libc::c_uint,
     0xedb88320 as libc::c_uint, 0x9abfb3b6 as libc::c_uint,
     0x3b6e20c as libc::c_int as dword, 0x74b1d29a as libc::c_int as dword,
     0xead54739 as libc::c_uint, 0x9dd277af as libc::c_uint,
     0x4db2615 as libc::c_int as dword, 0x73dc1683 as libc::c_int as dword,
     0xe3630b12 as libc::c_uint, 0x94643b84 as libc::c_uint,
     0xd6d6a3e as libc::c_int as dword, 0x7a6a5aa8 as libc::c_int as dword,
     0xe40ecf0b as libc::c_uint, 0x9309ff9d as libc::c_uint,
     0xa00ae27 as libc::c_int as dword, 0x7d079eb1 as libc::c_int as dword,
     0xf00f9344 as libc::c_uint, 0x8708a3d2 as libc::c_uint,
     0x1e01f268 as libc::c_int as dword, 0x6906c2fe as libc::c_int as dword,
     0xf762575d as libc::c_uint, 0x806567cb as libc::c_uint,
     0x196c3671 as libc::c_int as dword, 0x6e6b06e7 as libc::c_int as dword,
     0xfed41b76 as libc::c_uint, 0x89d32be0 as libc::c_uint,
     0x10da7a5a as libc::c_int as dword, 0x67dd4acc as libc::c_int as dword,
     0xf9b9df6f as libc::c_uint, 0x8ebeeff9 as libc::c_uint,
     0x17b7be43 as libc::c_int as dword, 0x60b08ed5 as libc::c_int as dword,
     0xd6d6a3e8 as libc::c_uint, 0xa1d1937e as libc::c_uint,
     0x38d8c2c4 as libc::c_int as dword, 0x4fdff252 as libc::c_int as dword,
     0xd1bb67f1 as libc::c_uint, 0xa6bc5767 as libc::c_uint,
     0x3fb506dd as libc::c_int as dword, 0x48b2364b as libc::c_int as dword,
     0xd80d2bda as libc::c_uint, 0xaf0a1b4c as libc::c_uint,
     0x36034af6 as libc::c_int as dword, 0x41047a60 as libc::c_int as dword,
     0xdf60efc3 as libc::c_uint, 0xa867df55 as libc::c_uint,
     0x316e8eef as libc::c_int as dword, 0x4669be79 as libc::c_int as dword,
     0xcb61b38c as libc::c_uint, 0xbc66831a as libc::c_uint,
     0x256fd2a0 as libc::c_int as dword, 0x5268e236 as libc::c_int as dword,
     0xcc0c7795 as libc::c_uint, 0xbb0b4703 as libc::c_uint,
     0x220216b9 as libc::c_int as dword, 0x5505262f as libc::c_int as dword,
     0xc5ba3bbe as libc::c_uint, 0xb2bd0b28 as libc::c_uint,
     0x2bb45a92 as libc::c_int as dword, 0x5cb36a04 as libc::c_int as dword,
     0xc2d7ffa7 as libc::c_uint, 0xb5d0cf31 as libc::c_uint,
     0x2cd99e8b as libc::c_int as dword, 0x5bdeae1d as libc::c_int as dword,
     0x9b64c2b0 as libc::c_uint, 0xec63f226 as libc::c_uint,
     0x756aa39c as libc::c_int as dword, 0x26d930a as libc::c_int as dword,
     0x9c0906a9 as libc::c_uint, 0xeb0e363f as libc::c_uint,
     0x72076785 as libc::c_int as dword, 0x5005713 as libc::c_int as dword,
     0x95bf4a82 as libc::c_uint, 0xe2b87a14 as libc::c_uint,
     0x7bb12bae as libc::c_int as dword, 0xcb61b38 as libc::c_int as dword,
     0x92d28e9b as libc::c_uint, 0xe5d5be0d as libc::c_uint,
     0x7cdcefb7 as libc::c_int as dword, 0xbdbdf21 as libc::c_int as dword,
     0x86d3d2d4 as libc::c_uint, 0xf1d4e242 as libc::c_uint,
     0x68ddb3f8 as libc::c_int as dword, 0x1fda836e as libc::c_int as dword,
     0x81be16cd as libc::c_uint, 0xf6b9265b as libc::c_uint,
     0x6fb077e1 as libc::c_int as dword, 0x18b74777 as libc::c_int as dword,
     0x88085ae6 as libc::c_uint, 0xff0f6a70 as libc::c_uint,
     0x66063bca as libc::c_int as dword, 0x11010b5c as libc::c_int as dword,
     0x8f659eff as libc::c_uint, 0xf862ae69 as libc::c_uint,
     0x616bffd3 as libc::c_int as dword, 0x166ccf45 as libc::c_int as dword,
     0xa00ae278 as libc::c_uint, 0xd70dd2ee as libc::c_uint,
     0x4e048354 as libc::c_int as dword, 0x3903b3c2 as libc::c_int as dword,
     0xa7672661 as libc::c_uint, 0xd06016f7 as libc::c_uint,
     0x4969474d as libc::c_int as dword, 0x3e6e77db as libc::c_int as dword,
     0xaed16a4a as libc::c_uint, 0xd9d65adc as libc::c_uint,
     0x40df0b66 as libc::c_int as dword, 0x37d83bf0 as libc::c_int as dword,
     0xa9bcae53 as libc::c_uint, 0xdebb9ec5 as libc::c_uint,
     0x47b2cf7f as libc::c_int as dword, 0x30b5ffe9 as libc::c_int as dword,
     0xbdbdf21c as libc::c_uint, 0xcabac28a as libc::c_uint,
     0x53b39330 as libc::c_int as dword, 0x24b4a3a6 as libc::c_int as dword,
     0xbad03605 as libc::c_uint, 0xcdd70693 as libc::c_uint,
     0x54de5729 as libc::c_int as dword, 0x23d967bf as libc::c_int as dword,
     0xb3667a2e as libc::c_uint, 0xc4614ab8 as libc::c_uint,
     0x5d681b02 as libc::c_int as dword, 0x2a6f2b94 as libc::c_int as dword,
     0xb40bbe37 as libc::c_uint, 0xc30c8ea1 as libc::c_uint,
     0x5a05df1b as libc::c_int as dword, 0x2d02ef8d as libc::c_int as dword];
#[no_mangle]
pub unsafe extern "C" fn CRC32_Init(mut pulCRC: *mut dword) {
    *pulCRC = 0xffffffff as libc::c_ulong as dword; // fallthrough
}
#[no_mangle]
pub unsafe extern "C" fn CRC32_Final(mut pulCRC: dword) -> dword {
    return (pulCRC as libc::c_ulong ^ 0xffffffff as libc::c_ulong) as
               dword; // fallthrough
}
#[no_mangle]
pub unsafe extern "C" fn CRC32_ProcessByte(mut pulCRC: *mut dword,
                                           mut ch: byte) {
    let mut ulCrc: dword = *pulCRC; // fallthrough
    ulCrc ^= ch as libc::c_uint; // fallthrough
    ulCrc =
        crc32table[ulCrc as byte as usize] ^
            ulCrc >> 8 as libc::c_int; // fallthrough
    *pulCRC = ulCrc; // fallthrough
}
#[no_mangle]
pub unsafe extern "C" fn CRC32_ProcessBuffer(mut pulCRC: *mut dword,
                                             mut pBuffer: *const libc::c_void,
                                             mut nBuffer: libc::c_int) {
    let mut ulCrc: dword =
        *pulCRC; // warning, this only works on little-endian.
    let mut tmp: dword = 0;
    let mut pb: *mut byte = pBuffer as *mut byte;
    let mut nFront: uint = 0;
    let mut nMain: libc::c_int = 0;
    loop  {
        's_97:
            {
                let mut current_block_15: u64;
                match nBuffer {
                    7 => {
                        let fresh0 = pb;
                        pb = pb.offset(1);
                        ulCrc =
                            crc32table[(*fresh0 as libc::c_int ^
                                            ulCrc as byte as libc::c_int) as
                                           usize] ^ ulCrc >> 8 as libc::c_int;
                        current_block_15 = 5097651525226480783;
                    }
                    6 => { current_block_15 = 5097651525226480783; }
                    5 => { current_block_15 = 13690466564372581271; }
                    4 => { current_block_15 = 8942225378300112384; }
                    3 => {
                        let fresh3 = pb;
                        pb = pb.offset(1);
                        ulCrc =
                            crc32table[(*fresh3 as libc::c_int ^
                                            ulCrc as byte as libc::c_int) as
                                           usize] ^ ulCrc >> 8 as libc::c_int;
                        current_block_15 = 286125667863391087;
                    }
                    2 => { current_block_15 = 286125667863391087; }
                    1 => { current_block_15 = 8004985925295807200; }
                    0 => { current_block_15 = 9335634981096802304; }
                    _ => { break 's_97 ; }
                }
                match current_block_15 {
                    5097651525226480783 => {
                        let fresh1 = pb;
                        pb = pb.offset(1);
                        ulCrc =
                            crc32table[(*fresh1 as libc::c_int ^
                                            ulCrc as byte as libc::c_int) as
                                           usize] ^ ulCrc >> 8 as libc::c_int;
                        current_block_15 = 13690466564372581271;
                    }
                    286125667863391087 => {
                        let fresh4 = pb;
                        pb = pb.offset(1);
                        ulCrc =
                            crc32table[(*fresh4 as libc::c_int ^
                                            ulCrc as byte as libc::c_int) as
                                           usize] ^ ulCrc >> 8 as libc::c_int;
                        current_block_15 = 8004985925295807200;
                    }
                    _ => { }
                }
                match current_block_15 {
                    13690466564372581271 => {
                        let fresh2 = pb;
                        pb = pb.offset(1);
                        ulCrc =
                            crc32table[(*fresh2 as libc::c_int ^
                                            ulCrc as byte as libc::c_int) as
                                           usize] ^ ulCrc >> 8 as libc::c_int;
                        current_block_15 = 8942225378300112384;
                    }
                    8004985925295807200 => {
                        let fresh5 = pb;
                        pb = pb.offset(1);
                        ulCrc =
                            crc32table[(*fresh5 as libc::c_int ^
                                            ulCrc as byte as libc::c_int) as
                                           usize] ^ ulCrc >> 8 as libc::c_int;
                        current_block_15 = 9335634981096802304;
                    }
                    _ => { }
                }
                match current_block_15 {
                    8942225378300112384 => {
                        memcpy(&mut tmp as *mut dword as *mut libc::c_void,
                               pb as *const libc::c_void,
                               ::std::mem::size_of::<dword>() as
                                   libc::c_ulong);
                        ulCrc ^= tmp;
                        ulCrc =
                            crc32table[ulCrc as byte as usize] ^
                                ulCrc >> 8 as libc::c_int;
                        ulCrc =
                            crc32table[ulCrc as byte as usize] ^
                                ulCrc >> 8 as libc::c_int;
                        ulCrc =
                            crc32table[ulCrc as byte as usize] ^
                                ulCrc >> 8 as libc::c_int;
                        ulCrc =
                            crc32table[ulCrc as byte as usize] ^
                                ulCrc >> 8 as libc::c_int;
                        *pulCRC = ulCrc;
                        return
                    }
                    _ => { *pulCRC = ulCrc; return }
                }
            }
        // We may need to do some alignment work up front, and at the end, so that
	// the main loop is aligned and only has to worry about 8 byte at a time.
	// The low-order two bits of pb and nBuffer in total control the
	// upfront work.
        nFront = pb as uint & 3 as libc::c_int as libc::c_uint;
        nBuffer =
            (nBuffer as libc::c_uint).wrapping_sub(nFront) as libc::c_int as
                libc::c_int;
        let mut current_block_21: u64;
        match nFront {
            3 => {
                let fresh6 = pb;
                pb = pb.offset(1);
                ulCrc =
                    crc32table[(*fresh6 as libc::c_int ^
                                    ulCrc as byte as libc::c_int) as usize] ^
                        ulCrc >> 8 as libc::c_int;
                current_block_21 = 5806584405471629700;
                // fallthrough
            }
            2 => {
                current_block_21 = 5806584405471629700; // fallthrough
            }
            1 => {
                current_block_21 =
                    9560005304141696532; // warning, this only works on little-endian.
            }
            _ => {
                current_block_21 =
                    4068382217303356765; // warning, this only works on little-endian.
            }
        }
        match current_block_21 {
            5806584405471629700 => {
                let fresh7 = pb;
                pb = pb.offset(1);
                ulCrc =
                    crc32table[(*fresh7 as libc::c_int ^
                                    ulCrc as byte as libc::c_int) as usize] ^
                        ulCrc >> 8 as libc::c_int;
                current_block_21 = 9560005304141696532;
            }
            _ => { }
        }
        match current_block_21 {
            9560005304141696532 => {
                let fresh8 = pb;
                pb = pb.offset(1);
                ulCrc =
                    crc32table[(*fresh8 as libc::c_int ^
                                    ulCrc as byte as libc::c_int) as usize] ^
                        ulCrc >> 8 as libc::c_int
            }
            _ => { }
        }
        nMain = nBuffer >> 3 as libc::c_int;
        loop  {
            let fresh9 = nMain;
            nMain = nMain - 1;
            if !(fresh9 != 0) { break ; }
            memcpy(&mut tmp as *mut dword as *mut libc::c_void,
                   pb as *const libc::c_void,
                   ::std::mem::size_of::<dword>() as libc::c_ulong);
            ulCrc ^= tmp;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            memcpy(&mut tmp as *mut dword as *mut libc::c_void,
                   pb.offset(4 as libc::c_int as isize) as
                       *const libc::c_void,
                   ::std::mem::size_of::<dword>() as libc::c_ulong);
            ulCrc ^= tmp;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            ulCrc =
                crc32table[ulCrc as byte as usize] ^
                    ulCrc >> 8 as libc::c_int;
            pb = pb.offset(8 as libc::c_int as isize)
        }
        nBuffer &= 7 as libc::c_int
    };
}
/*
====================
CRC32_BlockSequence

For proxy protecting
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CRC32_BlockSequence(mut base: *mut byte,
                                             mut length: libc::c_int,
                                             mut sequence: libc::c_int)
 -> byte {
    let mut CRC: dword = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 64] = [0; 64];
    if sequence < 0 as libc::c_int { sequence = abs(sequence) }
    ptr =
        (crc32table.as_ptr() as
             *mut libc::c_char).offset((sequence % 0x3fc as libc::c_int) as
                                           isize);
    if length > 60 as libc::c_int { length = 60 as libc::c_int }
    memcpy(buffer.as_mut_ptr() as *mut libc::c_void,
           base as *const libc::c_void, length as libc::c_ulong);
    buffer[(length + 0 as libc::c_int) as usize] =
        *ptr.offset(0 as libc::c_int as isize);
    buffer[(length + 1 as libc::c_int) as usize] =
        *ptr.offset(1 as libc::c_int as isize);
    buffer[(length + 2 as libc::c_int) as usize] =
        *ptr.offset(2 as libc::c_int as isize);
    buffer[(length + 3 as libc::c_int) as usize] =
        *ptr.offset(3 as libc::c_int as isize);
    length += 4 as libc::c_int;
    CRC32_Init(&mut CRC);
    CRC32_ProcessBuffer(&mut CRC, buffer.as_mut_ptr() as *const libc::c_void,
                        length);
    CRC = CRC32_Final(CRC);
    return CRC as byte;
}
/*
==================
MD5Init

Start MD5 accumulation.  Set bit count to 0 and buffer to mysterious initialization constants.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context_t) {
    (*ctx).buf[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as uint;
    (*ctx).buf[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*ctx).buf[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*ctx).buf[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as uint;
    (*ctx).bits[0 as libc::c_int as usize] = 0 as libc::c_int as uint;
    (*ctx).bits[1 as libc::c_int as usize] = 0 as libc::c_int as uint;
}
/*
===================
MD5Update

Update context to reflect the concatenation of another buffer full of bytes.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn MD5Update(mut ctx: *mut MD5Context_t,
                                   mut buf: *const byte, mut len: uint) {
    let mut t: uint = 0;
    // update bitcount
    t = (*ctx).bits[0 as libc::c_int as usize]; // carry from low to high
    (*ctx).bits[0 as libc::c_int as usize] =
        t.wrapping_add(len <<
                           3 as
                               libc::c_int); // bytes already in shsInfo->data
    if (*ctx).bits[0 as libc::c_int as usize] < t {
        (*ctx).bits[1 as libc::c_int as usize] =
            (*ctx).bits[1 as libc::c_int as usize].wrapping_add(1)
    }
    (*ctx).bits[1 as libc::c_int as usize] =
        ((*ctx).bits[1 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(len >> 29 as libc::c_int) as uint as
            uint;
    t = t >> 3 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
    // handle any leading odd-sized chunks
    if t != 0 {
        let mut p: *mut byte =
            ((*ctx).in_0.as_mut_ptr() as *mut byte).offset(t as isize);
        t = (64 as libc::c_int as libc::c_uint).wrapping_sub(t);
        if len < t {
            memcpy(p as *mut libc::c_void, buf as *const libc::c_void,
                   len as libc::c_ulong);
            return
        }
        memcpy(p as *mut libc::c_void, buf as *const libc::c_void,
               t as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *const uint);
        buf = buf.offset(t as isize);
        len = (len as libc::c_uint).wrapping_sub(t) as uint as uint
    }
    // process data in 64-byte chunks
    while len >= 64 as libc::c_int as libc::c_uint {
        memcpy((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
               buf as *const libc::c_void,
               64 as libc::c_int as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *const uint);
        buf = buf.offset(64 as libc::c_int as isize);
        len =
            (len as
                 libc::c_uint).wrapping_sub(64 as libc::c_int as libc::c_uint)
                as uint as uint
    }
    // handle any remaining bytes of data.
    memcpy((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
           buf as *const libc::c_void, len as libc::c_ulong);
}
/*
===============
MD5Final

Final wrapup - pad to 64-byte boundary with the bit pattern
1 0* (64-bit count of bits processed, MSB-first)
===============
*/
#[no_mangle]
pub unsafe extern "C" fn MD5Final(mut digest: *mut byte,
                                  mut ctx: *mut MD5Context_t) {
    let mut count: uint = 0;
    let mut p: *mut byte = 0 as *mut byte;
    // compute number of bytes mod 64
    count =
        (*ctx).bits[0 as libc::c_int as usize] >> 3 as libc::c_int &
            0x3f as libc::c_int as libc::c_uint;
    // set the first char of padding to 0x80.
	// this is safe since there is always at least one byte free
    p = ((*ctx).in_0.as_mut_ptr() as *mut byte).offset(count as isize);
    let fresh10 = p;
    p = p.offset(1);
    *fresh10 = 0x80 as libc::c_int as byte;
    // bytes of padding needed to make 64 bytes
    count =
        ((64 as libc::c_int - 1 as libc::c_int) as
             libc::c_uint).wrapping_sub(count);
    // pad out to 56 mod 64
    if count < 8 as libc::c_int as libc::c_uint {
        // two lots of padding: pad the first block to 64 bytes
        memset(p as *mut libc::c_void, 0 as libc::c_int,
               count as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *const uint);
        // now fill the next block with 56 bytes
        memset((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int, 56 as libc::c_int as libc::c_ulong);
    } else {
        // pad block to 56 bytes
        memset(p as *mut libc::c_void, 0 as libc::c_int,
               count.wrapping_sub(8 as libc::c_int as libc::c_uint) as
                   libc::c_ulong);
    }
    // append length in bits and transform
    (*ctx).in_0[14 as libc::c_int as usize] =
        (*ctx).bits[0 as libc::c_int as usize];
    (*ctx).in_0[15 as libc::c_int as usize] =
        (*ctx).bits[1 as libc::c_int as usize];
    MD5Transform((*ctx).buf.as_mut_ptr(),
                 (*ctx).in_0.as_mut_ptr() as *const uint);
    memcpy(digest as *mut libc::c_void,
           (*ctx).buf.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    memset(ctx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<MD5Context_t>() as libc::c_ulong);
    // in case it's sensitive
}
// this is the central step in the MD5 algorithm.
/*
=================
MD5Transform

The core of the MD5 algorithm, this alters an existing MD5 hash to
reflect the addition of 16 longwords of new data.  MD5Update blocks
the data and converts bytes into longwords for this routine.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn MD5Transform(mut buf: *mut uint,
                                      mut in_0: *const uint) {
    let mut a: uint = 0;
    let mut b: uint = 0;
    let mut c: uint = 0;
    let mut d: uint = 0;
    a = *buf.offset(0 as libc::c_int as isize);
    b = *buf.offset(1 as libc::c_int as isize);
    c = *buf.offset(2 as libc::c_int as isize);
    d = *buf.offset(3 as libc::c_int as isize);
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xd76aa478
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xe8c7b756
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x242070db
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xc1bdceee
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(4
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xf57c0faf
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(5
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x4787c62a
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(6
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xa8304613
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(7
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xfd469501
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x698098d8
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(9
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x8b44f7af
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(10
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xffff5bb1
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(11
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x895cd7be
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(12
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x6b901122
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(13
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xfd987193
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(14
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xa679438e
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(15
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x49b40821
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xf61e2562
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(6
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xc040b340
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(11
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x265e5a51
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xe9b6c7aa
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(5
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xd62f105d
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(10
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x2441453
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(15
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xd8a1e681
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(4
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xe7d3fbc8
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(9
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x21e1cde6
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(14
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xc33707d6
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xf4d50d87
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x455a14ed
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(13
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xa9e3e905
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0xfcefa3f8
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(7
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x676f02d9
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(12
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).wrapping_add(0x8d2a4c8a
                                                                                                                 as
                                                                                                                 libc::c_uint))
            as uint as uint;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(5 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xfffa3942
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(8 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x8771f681
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(11
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x6d9d6122
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(14
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xfde5380c
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xa4beea44
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(4 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x4bdecfa9
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(7 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xf6bb4b60
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(10
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xbebfbc70
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(13
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x289b7ec6
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xeaa127fa
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(3 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xd4ef3085
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(6 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x4881d05
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(9 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xd9d4d039
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(12
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xe6db99e5
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(15
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x1fa27cf8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(2 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xc4ac5665
                                                                                                       as
                                                                                                       libc::c_uint))
            as uint as uint;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xf4292244
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(7
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x432aff97
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(14
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xab9423a7
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(5
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xfc93a039
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(12
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x655b59c3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(3
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x8f0ccc92
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xffeff47d
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x85845dd1
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x6fa87e4f
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(15
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xfe2ce6e0
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(6
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xa3014314
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(13
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x4e0811a1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(4
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xf7537e82
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint as uint;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(11
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xbd3af235
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint as uint;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x2ad7d2bb
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint as uint;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(9
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xeb86d391
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint as uint;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint as uint;
    let ref mut fresh11 = *buf.offset(0 as libc::c_int as isize);
    *fresh11 = (*fresh11 as libc::c_uint).wrapping_add(a) as uint as uint;
    let ref mut fresh12 = *buf.offset(1 as libc::c_int as isize);
    *fresh12 = (*fresh12 as libc::c_uint).wrapping_add(b) as uint as uint;
    let ref mut fresh13 = *buf.offset(2 as libc::c_int as isize);
    *fresh13 = (*fresh13 as libc::c_uint).wrapping_add(c) as uint as uint;
    let ref mut fresh14 = *buf.offset(3 as libc::c_int as isize);
    *fresh14 = (*fresh14 as libc::c_uint).wrapping_add(d) as uint as uint;
}
/*
=================
MD5_Print

transform hash to hexadecimal printable symbols
=================
*/
#[no_mangle]
pub unsafe extern "C" fn MD5_Print(mut hash: *mut byte) -> *mut libc::c_char {
    static mut szReturn: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    memset(szReturn.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           64 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        COM_Hex2String(*hash.offset(i as isize),
                       &mut *szReturn.as_mut_ptr().offset((i *
                                                               2 as
                                                                   libc::c_int)
                                                              as isize));
        i += 1
    }
    return szReturn.as_mut_ptr();
}
/*
=================
COM_HashKey

returns hash key for string
=================
*/
#[no_mangle]
pub unsafe extern "C" fn COM_HashKey(mut string: *const libc::c_char,
                                     mut hashSize: uint) -> uint {
    let mut i: uint = 0;
    let mut hashKey: uint = 0 as libc::c_int as uint;
    i = 0 as libc::c_int as uint;
    while *string.offset(i as isize) != 0 {
        hashKey =
            hashKey.wrapping_add(i).wrapping_mul(37 as libc::c_int as
                                                     libc::c_uint).wrapping_add(Q_tolower(*string.offset(i
                                                                                                             as
                                                                                                             isize))
                                                                                    as
                                                                                    libc::c_uint);
        i = i.wrapping_add(1)
    }
    return hashKey.wrapping_rem(hashSize);
}
