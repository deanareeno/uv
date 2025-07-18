use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use uv_normalize::PackageName;
use uv_pypi_types::VerbatimParsedUrl;

use crate::ResolveError;
use crate::resolver::ResolverEnvironment;

/// See [`crate::resolver::ForkState`].
#[derive(Default, Debug, Clone)]
pub(crate) struct ForkUrls(FxHashMap<PackageName, VerbatimParsedUrl>);

impl ForkUrls {
    /// Get the URL previously used for a package in this fork.
    pub(crate) fn get(&self, package_name: &PackageName) -> Option<&VerbatimParsedUrl> {
        self.0.get(package_name)
    }

    /// Whether we use a URL for this package.
    pub(crate) fn contains_key(&self, package_name: &PackageName) -> bool {
        self.0.contains_key(package_name)
    }

    /// Check that this is the only URL used for this package in this fork.
    pub(crate) fn insert(
        &mut self,
        package_name: &PackageName,
        url: &VerbatimParsedUrl,
        env: &ResolverEnvironment,
    ) -> Result<(), ResolveError> {
        match self.0.entry(package_name.clone()) {
            Entry::Occupied(previous) => {
                if previous.get() != url {
                    let mut conflicting_url =
                        vec![previous.get().parsed_url.clone(), url.parsed_url.clone()];
                    conflicting_url.sort();
                    return Err(ResolveError::ConflictingUrls {
                        package_name: package_name.clone(),
                        urls: conflicting_url,
                        env: env.clone(),
                    });
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert(url.clone());
            }
        }
        Ok(())
    }
}
