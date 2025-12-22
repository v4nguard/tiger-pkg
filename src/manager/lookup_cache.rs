use itertools::MultiUnzip;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::{error, info};

use super::{PackageManager, TagLookupIndex};
use crate::{manager::HashTableEntryShort, TagHash64, Version};

impl PackageManager {
    const LOOKUP_CACHE_MAGIC: [u8; 4] = *b"TLI0";

    #[cfg(feature = "ignore_lookup_cache")]
    pub(super) fn read_lookup_cache(&self) -> Option<TagLookupIndex> {
        info!("Not loading tag cache: ignore_lookup_cache feature flag is set");
        None
    }

    #[cfg(feature = "ignore_lookup_cache")]
    pub(super) fn write_lookup_cache(&self) -> anyhow::Result<()> {
        Ok(())
    }

    #[cfg(not(feature = "ignore_lookup_cache"))]
    pub(super) fn read_lookup_cache(&self) -> Option<TagLookupIndex> {
        use std::io::Read;

        use crate::manager::path_cache::exe_relative_path;

        let mut file = std::fs::File::open(exe_relative_path(&format!(
            "lookup_cache_{}.bin",
            self.cache_key()
        )))
        .ok()?;

        let mut cache_data = Vec::new();
        file.read_to_end(&mut cache_data).ok()?;
        if cache_data[..4] != Self::LOOKUP_CACHE_MAGIC {
            error!("Invalid lookup cache magic");
            return None;
        }

        info!("Loading index cache");

        let cache: Option<TagLookupIndex> =
            bincode::decode_from_slice(&cache_data[4..], bincode::config::legacy())
                .map(|(v, _)| v)
                .ok();

        cache
    }

    #[cfg(not(feature = "ignore_lookup_cache"))]
    pub(super) fn write_lookup_cache(&self) -> anyhow::Result<()> {
        use super::path_cache::exe_relative_path;

        let mut data = Vec::new();
        data.extend_from_slice(&Self::LOOKUP_CACHE_MAGIC);
        data.extend_from_slice(&bincode::encode_to_vec(
            &self.lookup,
            bincode::config::legacy(),
        )?);

        Ok(std::fs::write(
            exe_relative_path(&format!("lookup_cache_{}.bin", self.cache_key())),
            data,
        )?)
    }

    pub fn build_lookup_tables(&mut self) {
        let start = std::time::Instant::now();
        let tables: Vec<_> = self
            .package_paths
            .par_iter()
            .filter_map(|(_, p)| {
                let pkg = match self.version.open(&p.path) {
                    Ok(package) => package,
                    Err(e) => {
                        error!("Failed to open package '{}': {e}", p.filename);
                        return None;
                    }
                };
                let entries = (pkg.pkg_id(), pkg.entries().to_vec());

                let collect = pkg
                    .hash64_table()
                    .iter()
                    .map(|h| {
                        (
                            h.hash64,
                            HashTableEntryShort {
                                hash32: h.hash32,
                                reference: h.reference,
                            },
                        )
                    })
                    .collect::<Vec<(u64, HashTableEntryShort)>>();
                let hashes = collect;

                let named_tags = pkg.named_tags();

                Some((entries, hashes, named_tags))
            })
            .collect();

        let (entries, hashes, named_tags): (_, Vec<_>, Vec<_>) = tables.into_iter().multiunzip();

        self.lookup = TagLookupIndex {
            tag32_entries_by_pkg: entries,
            tag32_to_tag64: hashes
                .iter()
                .flatten()
                .map(|(h64, entry)| (entry.hash32, TagHash64(*h64)))
                .collect(),
            tag64_entries: hashes.into_iter().flatten().collect(),
            named_tags: named_tags.into_iter().flatten().collect(),
        };

        info!(
            "Built lookup table for {} packages in {:?}",
            self.lookup.tag32_entries_by_pkg.len(),
            start.elapsed()
        );
    }
}
