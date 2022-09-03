#![doc = include_str!("../README.md")]
//!
//! To get started, see the [`static_router!`] macro.

#![warn(clippy::pedantic)]
#![warn(
	missing_copy_implementations,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	variant_size_differences
)]
#![forbid(unsafe_code)]

/// Create a static files router with the given name and static directory path.
///
/// The static directory path is relative to the crate root, not the caller file.
///
/// The resulting router serves the static files at the root. Use axum's router nesting feature if you want the static files to be scoped to something like `/static`.
///
/// # Syntax
///
/// The macro expects as arguments an identifier (the router name), then a comma, then a string literal (the static directory path).
///
/// # Examples
///
/// Create a router named `router` that serves the files in `static`.
///
/// ```no_compile
/// static_router!(router, "static");
/// ```
pub use static_router_macros::static_router;

#[doc(hidden)]
pub mod __private {
	pub use {axum, http, std, tower_http};
}
