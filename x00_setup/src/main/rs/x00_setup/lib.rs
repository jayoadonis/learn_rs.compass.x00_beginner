//! Crate-wide lint configuration.
//!
//! This attribute relaxes several compiler warnings that commonly occur
//! during development, experimentation, or prototyping. It helps keep
//! build output clean while code is still evolving.
//!
//! ## Allowed lints
//! - `dead_code`: Unused functions, structs, or impls
//! - `unused_variables`: Variables that are declared but not used
//! - `unused_imports`: Imports that are not referenced
//! - `unused_parens`: Unnecessary parentheses
//!
//! **Note:** These lints should typically be re-enabled in production
//! code to maintain correctness, readability, and maintainability.
#![allow(
  dead_code, 
  unused_variables, unused_imports,
  unused_parens
)]

//! ## Denied lints (clippy plugin linter)
//! - `shadow_reuse`: Reusing a variable name.
//! - `shadow_same`: Shadowing with the same value.
//! - `shadow_unrelated`: Shadowing with an unrelated value.
//! 
//! **Note:** Enable this only if required by you or your organization
#![deny(
  clippy::shadow_reuse,
  clippy::shadow_same,
  clippy::shadow_unrelated
)]