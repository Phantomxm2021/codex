//! Measures published skill catalogs, excluding cached reads.

use crate::catalog::SkillCatalog;
use bgs_extension_api::ExtensionMetrics;
use bgs_extension_api::record_catalog_entries;

pub(crate) fn record_catalog(
    metrics: Option<&dyn ExtensionMetrics>,
    stage: &'static str,
    catalog: &SkillCatalog,
) {
    record_catalog_entries(
        metrics,
        "bgs.skills.catalog.entries",
        stage,
        catalog.entries.len(),
        catalog.is_partial,
        catalog.warnings.len(),
    );
}
