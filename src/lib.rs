#![doc = include_str!("../README.md")]

extern crate core;

mod block_cache;
mod crypto;
mod d2_shared;
mod oodle;
pub use crypto::{has_pkg_key, register_pkg_key};

mod d1_internal_alpha;
mod d1_legacy;
mod d1_roi;
mod d2_beta;
mod d2_beyondlight;
mod d2_prebl;

pub mod manager;
pub mod package;
pub mod tag;
pub mod version;

pub use binrw::Endian;

pub use d1_internal_alpha::PackageD1InternalAlpha;
pub use d1_legacy::PackageD1Legacy;
pub use d1_roi::PackageD1RiseOfIron;
pub use d2_beta::PackageD2Beta;
pub use d2_beyondlight::PackageD2BeyondLight;
pub use d2_prebl::PackageD2PreBL;

pub use d1_legacy::{
    structs::BlockHeader as BlockHeaderD1Legacy, structs::EntryHeader as EntryHeaderD1Legacy,
    structs::NamedTagEntryD1 as NamedTagEntryD1Legacy,
};

pub use d1_roi::{
    structs::BlockHeader as BlockHeaderD1RiseOfIron,
    structs::EntryHeader as EntryHeaderD1RiseOfIron,
    structs::NamedTagEntryD1 as NamedTagEntryD1RiseOfIron,
};

pub use d2_shared::{
    BlockFlags, BlockHeader as BlockHeaderD2, EntryHeader as EntryHeaderD2, PackageNamedTagEntry,
};

pub use manager::PackageManager;
pub use package::{Package, PackageLanguage, PackagePlatform};
pub use tag::{TagHash, TagHash64};
pub use version::{DestinyVersion, GameVersion, MarathonVersion, Version};

#[cfg(feature = "global_manager_instance")]
mod global_instance;

#[cfg(feature = "global_manager_instance")]
pub use global_instance::*;
