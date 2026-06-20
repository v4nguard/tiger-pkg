use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    sync::Arc,
};

use binrw::{BinReaderExt, Endian, VecArgs};

use crate::{
    d2_prebl::structs::PackageHeader,
    d2_shared::{
        BlockHeader, CommonPackageData, EntryHeader, HashTableEntry, PackageCommonD2,
        PackageNamedTagEntry,
    },
    package::{Package, PackageLanguage, PackagePlatform, ReadSeek, UEntryHeader, UHashTableEntry},
    DestinyVersion, TagHash,
};

pub struct PackageD2PreBL {
    common: PackageCommonD2,
    pub header: PackageHeader,
    pub named_tags: Vec<PackageNamedTagEntry>,
    pub precached_tags: Vec<[TagHash; 2]>,
}

unsafe impl Send for PackageD2PreBL {}
unsafe impl Sync for PackageD2PreBL {}

impl PackageD2PreBL {
    pub fn open(path: &str) -> anyhow::Result<PackageD2PreBL> {
        let _span = tracing::trace_span!("PackageD2PreBL::open", path);
        let reader = File::open(path)?;

        Self::from_reader(path, reader)
    }

    pub fn from_reader<R: ReadSeek + 'static>(
        path: &str,
        reader: R,
    ) -> anyhow::Result<PackageD2PreBL> {
        let _span = tracing::trace_span!("PackageD2PreBL::from_reader", path);
        let mut reader = BufReader::new(reader);
        let header: PackageHeader = reader.read_le()?;

        reader.seek(SeekFrom::Start(header.data_table_offset as u64 + 0x18))?;
        let entry_table_off: u64 = reader.read_le()?;

        reader.seek_relative(8)?;
        let block_table_off: u64 = reader.read_le()?;

        let precache_table_len: u64 = reader.read_le()?;
        let precache_table_off: u64 = reader.read_le()?;

        reader.seek(SeekFrom::Start(
            (header.data_table_offset as u64 + 0x18 + entry_table_off + 0x10) as _,
        ))?;

        let entries = reader.read_le_args(VecArgs {
            count: header.entry_table_size as _,
            inner: (),
        })?;

        reader.seek(SeekFrom::Start(
            (header.data_table_offset as u64 + 0x28 + block_table_off + 0x10) as _,
        ))?;
        let blocks = reader.read_le_args(VecArgs {
            count: header.block_table_size as _,
            inner: (),
        })?;

        let precached_tags: Vec<[TagHash; 2]> = if precache_table_len != 0 {
            reader.seek(SeekFrom::Start(
                (header.data_table_offset as u64 + 0x38 + precache_table_off + 0x10) as _,
            ))?;
            reader.read_le_args(VecArgs {
                count: precache_table_len as _,
                inner: (),
            })?
        } else {
            vec![]
        };

        let wide_hashes: Vec<HashTableEntry> = if header.misc_data_offset != 0 {
            reader.seek(SeekFrom::Start((header.misc_data_offset + 0x30) as _))?;
            let h64_table_size: u64 = reader.read_le()?;
            let real_h64_table_offset: u64 = reader.read_le()?;
            reader.seek(SeekFrom::Current(-8 + real_h64_table_offset as i64 + 16))?;
            reader.read_le_args(VecArgs {
                count: h64_table_size as _,
                inner: (),
            })?
        } else {
            vec![]
        };

        let named_tags: Vec<PackageNamedTagEntry> = if header.misc_data_offset != 0 {
            reader.seek(SeekFrom::Start((header.misc_data_offset + 0x10) as _))?;
            let named_tags_size: u64 = reader.read_le()?;
            let real_named_tags_offset: u64 = reader.read_le()?;
            reader.seek(SeekFrom::Current(-8 + real_named_tags_offset as i64 + 16))?;
            reader.read_le_args(VecArgs {
                count: named_tags_size as _,
                inner: (),
            })?
        } else {
            vec![]
        };

        Ok(PackageD2PreBL {
            common: PackageCommonD2::new(
                reader.into_inner(),
                DestinyVersion::Destiny2Shadowkeep,
                path.to_string(),
                CommonPackageData {
                    pkg_id: header.pkg_id,
                    patch_id: header.patch_id,
                    group_id: header.group_id,
                    entries,
                    blocks,
                    wide_hashes,
                    language: header.language,
                },
            )?,
            header,
            named_tags,
            precached_tags,
        })
    }

    pub fn blocks(&self) -> &[BlockHeader] {
        &self.common.blocks
    }

    pub fn blocks_mut(&mut self) -> &mut Vec<BlockHeader> {
        self.common.blocks.as_mut()
    }

    pub fn entries_raw(&self) -> &[EntryHeader] {
        &self.common._entries
    }

    pub fn entry_mut(&mut self, index: usize) -> Option<&mut EntryHeader> {
        self.common._entries.get_mut(index)
    }
}

// TODO(cohae): Can we implement this on PackageCommon?
impl Package for PackageD2PreBL {
    fn endianness(&self) -> Endian {
        Endian::Little // TODO(cohae): Not necessarily
    }

    fn pkg_id(&self) -> u16 {
        self.common.pkg_id
    }

    fn patch_id(&self) -> u16 {
        self.common.patch_id
    }

    fn language(&self) -> PackageLanguage {
        self.common.language
    }

    fn platform(&self) -> PackagePlatform {
        self.header.platform
    }

    fn hash64_table(&self) -> Vec<UHashTableEntry> {
        self.common
            .wide_hashes
            .iter()
            .map(|h| UHashTableEntry {
                hash64: h.hash64,
                hash32: h.hash32,
                reference: h.reference,
            })
            .collect()
    }

    fn named_tags(&self) -> Vec<PackageNamedTagEntry> {
        self.named_tags.clone()
    }

    fn entries(&self) -> &[UEntryHeader] {
        &self.common.entries_unified
    }

    fn entry(&self, index: usize) -> Option<UEntryHeader> {
        self.common.entries_unified.get(index).cloned()
    }

    fn get_block(&self, index: usize) -> anyhow::Result<Arc<Vec<u8>>> {
        self.common.get_block(index)
    }
}
