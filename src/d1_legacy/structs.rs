use std::io::SeekFrom;

use binrw::binrw;

use crate::{
    package::{PackageLanguage, PackagePlatform},
    TagHash,
};

#[binrw]
#[derive(Debug)]
#[br(big)]
#[allow(dead_code)]
pub struct PackageHeader {
    #[br(assert(version == 24))]
    pub version: u16,
    pub platform: PackagePlatform,

    pub pkg_id: u16,
    pub _unk6: u16,
    pub group_id: u64,
    pub build_time: u64,
    pub _unk_buildid: u32,
    pub _unk1c: u32,
    pub patch_id: u16,
    pub language: PackageLanguage,

    #[br(count = 128)]
    #[bw(pad_size_to = 128)]
    #[br(map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    #[bw(map = |s| s.clone().into_bytes())]
    pub tool_string: String,

    pub _unka4: u32,
    pub _unka8: u32,
    pub _unkac: u32,
    pub header_signature_offset: u32,

    pub entry_table_size: u32,
    pub entry_table_offset: u32,
    pub entry_table_hash: [u8; 20],

    pub block_table_size: u32,
    pub block_table_offset: u32,
    pub block_table_hash: [u8; 20],

    pub named_tag_table_size: u32,
    pub named_tag_table_offset: u32,
    pub named_tag_table_hash: [u8; 20],

    #[brw(seek_before = SeekFrom::Start(0x13c))]
    pub file_size: u32,
}

#[binrw]
#[derive(Debug)]
#[brw(big)]
pub struct EntryHeader {
    pub reference: u32,

    #[bw(calc = (*file_type as u32) << 16 | (*unk_type2 as u32) << 8 | *file_subtype as u32)]
    _types: u32,

    #[br(calc = (_types >> 8) as u8)]
    #[bw(ignore)]
    pub unk_type2: u8,
    #[br(calc = (_types >> 16) as u8)]
    #[bw(ignore)]
    pub file_type: u8,
    #[br(calc = (_types & 0xff) as u8)]
    #[bw(ignore)]
    pub file_subtype: u8,

    #[bw(calc = (*file_size as u64) << 28 | ((*starting_block_offset as u64) >> 4) << 14 | *starting_block as u64)]
    _block_info: u64,

    #[br(calc = _block_info as u32 & 0x3fff)]
    #[bw(ignore)]
    pub starting_block: u32,

    #[br(calc = ((_block_info >> 14) as u32 & 0x3FFF) << 4)]
    #[bw(ignore)]
    pub starting_block_offset: u32,

    #[br(calc = (_block_info >> 28) as u32 & 0x3FFFFFFF)]
    #[bw(ignore)]
    pub file_size: u32,
}

#[binrw]
#[derive(Debug)]
#[brw(big)]
pub struct BlockHeader {
    pub offset: u32,
    pub size: u32,
    pub patch_id: u16,
    pub flags: u16,
    pub hash: [u8; 20],
}

#[binrw]
#[derive(Debug, Clone)]
#[brw(big)]
pub struct NamedTagEntryD1 {
    pub hash: TagHash,
    pub class_hash: u32,
    pub name: [u8; 60],
}
