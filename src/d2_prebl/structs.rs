use std::{fmt::Debug, io::SeekFrom};

use binrw::{BinRead, BinWrite};

use crate::package::{PackageLanguage, PackagePlatform};

#[derive(BinRead, BinWrite, Debug)]
pub struct PackageHeader {
    #[br(assert(version == 38))]
    pub version: u16,
    pub platform: PackagePlatform,

    pub pkg_id: u16,
    pub _unk6: u16,
    pub group_id: u64,
    pub build_time: u64, // 0x10
    pub _unk18: u32,
    pub _unk1c: u32,
    pub patch_id: u16, // 0x20
    pub language: PackageLanguage,

    #[br(count = 128)]
    #[bw(pad_size_to = 128)]
    #[br(map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    #[bw(map = |s| s.clone().into_bytes())]
    pub tool_string: String, // 0x24

    pub _unka4: u32,
    pub _unka8: u32,
    pub _unkac: u32,
    pub header_signature_offset: u32, // 0xb0
    pub entry_table_size: u32,

    #[brw(seek_before = SeekFrom::Start(0xd0))]
    pub block_table_size: u32,

    #[brw(seek_before = SeekFrom::Start(0xf0))]
    pub misc_data_offset: u32,
    pub misc_data_size: u32,
    /// SHA1 hash of the data in range `misc_data_offset..misc_data_offset+misc_data_size`
    pub _misc_data_hash: [u8; 20],

    #[brw(seek_before = SeekFrom::Start(0x110))]
    pub data_table_offset: u32,
    pub data_table_size: u32,
    /// SHA1 hash of the data in range `data_table_offset..data_table_offset+data_table_size`
    pub _table_hash: [u8; 20],

    #[brw(seek_before = SeekFrom::Start(0x160))]
    pub deadbeef_off: u32,
    pub file_size: u32,
    pub unk168: u32,
}
