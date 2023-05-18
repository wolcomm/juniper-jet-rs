//! A rust client for the Juniper JET gRPC API
#![doc(html_root_url = "https://docs.rs/juniper-jet/0.1.0-alpha.1")]
// clippy lints
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![allow(clippy::redundant_pub_crate)]
// rustc lints
#![allow(box_pointers)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(deprecated_in_future)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(keyword_idents)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
// docs.rs build config
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "latest")]
/// Generated JET gRPC service definitions for the latest Junos version.
pub mod latest {
    pub use super::junos_23_1::jnx;
}

junos_versions! {
    "23.1": #[cfg(feature = "junos-23-1")] junos_23_1,
    "22.4": #[cfg(feature = "junos-22-4")] junos_22_4,
    "22.3": #[cfg(feature = "junos-22-3")] junos_22_3,
    "22.2": #[cfg(feature = "junos-22-2")] junos_22_2,
    "21.4": #[cfg(feature = "junos-21-4")] junos_21_4,
    "21.3": #[cfg(feature = "junos-21-3")] junos_21_3,
    "21.2": #[cfg(feature = "junos-21-2")] junos_21_2,
    "21.1": #[cfg(feature = "junos-21-1")] junos_21_1,
    "20.4": #[cfg(feature = "junos-20-4")] junos_20_4,
    "20.3": #[cfg(feature = "junos-20-3")] junos_20_3,
    "20.2": #[cfg(feature = "junos-20-2")] junos_20_2,
    "20.1": #[cfg(feature = "junos-20-1")] junos_20_1,
    "19.4": #[cfg(feature = "junos-19-4")] junos_19_4,
    "19.3": #[cfg(feature = "junos-19-3")] junos_19_3,
    "19.2": #[cfg(feature = "junos-19-2")] junos_19_2,
    "19.1": #[cfg(feature = "junos-19-1")] junos_19_1,
}

macro_rules! junos_versions {
    ( $( $version:literal: $( #[$attr:meta] )+ $mod:ident ),* $(,)? ) => {
        $(
            $( #[$attr] )+
            #[allow(clippy::doc_markdown)]
            #[allow(clippy::future_not_send)]
            #[allow(clippy::missing_const_for_fn)]
            #[allow(clippy::missing_errors_doc)]
            #[allow(clippy::module_name_repetitions)]
            #[allow(clippy::must_use_candidate)]
            #[allow(clippy::struct_excessive_bools)]
            #[allow(clippy::tabs_in_doc_comments)]
            #[allow(clippy::use_self)]
            #[allow(clippy::wildcard_imports)]
            #[allow(rustdoc::invalid_html_tags)]
            #[allow(rustdoc::broken_intra_doc_links)]
            #[allow(unused_results)]
            #[allow(unused_qualifications)]
            #[allow(missing_copy_implementations)]
            #[allow(missing_docs)]
            #[allow(variant_size_differences)]
            #[doc = "Generated JET gRPC service definitions for Junos `"]
            #[doc = $version]
            #[doc = "`."]
            pub mod $mod {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($mod), "/jnx.jet.rs"));
            }
        )*
    };
}
use junos_versions;

// silence unused dev-dependency warnings
#[cfg(test)]
mod deps {
    use version_sync as _;
}
