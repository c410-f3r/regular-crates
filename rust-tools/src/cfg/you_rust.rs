// DNMS: Does Not Make Sense
// HP: Has Problems
// MSTS: Make Sense To Some
// PCV: Prefer Clippy Variant
// PO: Personal Opinion
// U: Unstable

use crate::Params;

const CLIPPY_FLAGS: &[&str] = &[
  "-Dclippy::pedantic",
  "-Dclippy::restriction",
  "-Dwarnings",
  // There are places where absolute paths are needed to avoid conflicts (DNMS)
  "-Aclippy::absolute_paths",
  // `allow`s are required when `cfg` flags are involved (MSTS)
  "-Aclippy::allow_attributes",
  // Trips with top-most public modules and bottom-most private modules (HP)
  "-Aclippy::arbitrary_source_item_ordering",
  // Very niche (DNMS)
  "-Aclippy::big_endian_bytes",
  // `rust-tools` makes this lint irrelevant (DNMS)
  "-Aclippy::blanket_clippy_restriction_lints",
  // Too many false positives (PO)
  "-Aclippy::cfg_not_test",
  // For some people, some numbers are better represented by literals (MSTS)
  "-Aclippy::decimal_literal_representation",
  // Integers default to `u32` and floats to `f64`. This is predictable and set in stone (DNMS)
  "-Aclippy::default_numeric_fallback",
  // Conflicts with `missing-docs` (HP)
  "-Aclippy::doc_include_without_cfg",
  // Can't distinguish between titles and paragraphs (HP)
  "-Aclippy::doc_paragraphs_missing_punctuation",
  // Allows the future insertion of fields without breaking user-code (DNMS)
  "-Aclippy::empty_structs_with_brackets",
  // Sometimes it is more readable to use inline modules when they are small (PO)
  "-Aclippy::inline_modules",
  // `arithmetic_side_effects` makes the use of these operators predictable by the compiler (DNMS)
  "-Aclippy::integer_division_remainder_used",
  // It is better to not have an empty `else` block (PO)
  "-Aclippy::else_if_without_else",
  // adhoc module prefixes should be used to distinguish different error enums (PO)
  "-Aclippy::error_impl_error",
  // Exhaustiveness is a good thing (PO)
  "-Aclippy::exhaustive_enums",
  // Exhaustiveness is a good thing (PO)
  "-Aclippy::exhaustive_structs",
  // Extra work that leads to the same end result (DNMS)
  "-Aclippy::field_scoped_visibility_modifiers",
  // Very niche (MSTS)
  "-Aclippy::float_arithmetic",
  // Not very aesthetic (PO)
  "-Aclippy::ignored_unit_patterns",
  // For example, functions and iterators are common candidates (DNMS)
  "-Aclippy::impl_trait_in_params",
  // It is idiomatic to not use `return` (DNMS)
  "-Aclippy::implicit_return",
  // Essential in high performance (DNMS)
  "-Aclippy::inline_always",
  // Does not make sense (PO)
  "-Aclippy::integer_division",
  // Extra work that leads to the same end result (DNMS)
  "-Aclippy::into_iter_without_iter",
  // Does not work with `cfg_select` (HP)
  "-Aclippy::items_after_statements",
  // Not very useful (MSTS)
  "-Aclippy::len_without_is_empty",
  // Conflicts with other lints and also generates additional work (PO)
  "-Aclippy::let_underscore_untyped",
  // Very niche (MSTS)
  "-Aclippy::little_endian_bytes",
  // It is actually more readable to use ranges (PO)
  "-Aclippy::map_with_unused_argument_over_ranges",
  // The code itself should provide enough context (PO)
  "-Aclippy::missing_assert_message",
  // It is highly impractical to document private functions in large codebases (MSTS)
  "-Aclippy::missing_docs_in_private_items",
  // Callers can look into the codebase to gather more info about the error (PO)
  "-Aclippy::missing_errors_doc",
  // Provided methods are highly useful and common (MSTS)
  "-Aclippy::missing_trait_methods",
  // Doesn't make sense (PO)
  "-Aclippy::module_name_repetitions",
  // Very niche (MSTS)
  "-Aclippy::modulo_arithmetic",
  // Different blocks can be used as logical divisors (MSTS)
  "-Aclippy::multiple_inherent_impl",
  // The usefulness is doubtful (PO)
  "-Aclippy::must_use_candidate",
  // Lifetime errors because inner references are shorter (MSTS)
  "-Aclippy::ref_option",
  // Pattern matching used to be more restrict in older versions (PO)
  "-Aclippy::pattern_type_mismatch",
  // Just plain wrong (DNMS)
  "-Aclippy::pub_use",
  // The `in` prefix is unnecessary (PO)
  "-Aclippy::pub_with_shorthand",
  // `?` is idiomatic and there are third-party libraries that offer backtraces when working
  // with `?` (DNMS)
  "-Aclippy::question_mark_used",
  // The required methods of an external trait might conflict with internal methods (DNMS)
  "-Aclippy::same_name_method",
  // The new syntax is better (PO)
  "-Aclippy::self_named_module_files",
  // Not very aesthetic (PO)
  "-Aclippy::semicolon_outside_block",
  // Single responsibility and self documentation (DNMS)
  "-Aclippy::single_call_fn",
  // Too restrictive (PO)
  "-Aclippy::struct_field_names",
  // Sometimes a new field is not evaluated in established places because of the wildcard (PO)
  "-Aclippy::unneeded_field_pattern",
  // Unseparated is better than separated (PO)
  "-Aclippy::unseparated_literal_suffix",
  // New extension with lots of false-positives (HP)
  "-Aclippy::unused_async_trait_impl",
  // Variables with starting underscore are required when `cfg` flags are involved (MSTS)
  "-Aclippy::used_underscore_binding",
];

const RUST_FLAGS: &[&str] = &[
  "-Dabsolute_paths_not_starting_with_crate",
  "-Dambiguous_negative_literals",
  "-Dclosure_returning_async_block",
  "-Ddeprecated_in_future",
  //"-Ddeprecated_llvm_intrinsic", (U)
  "-Ddeprecated_safe_2024",
  "-Dderef_into_dyn_supertrait",
  "-Dedition_2024_expr_fragment_specifier",
  "-Delided_lifetimes_in_paths",
  "-Dexplicit_outlives_requirements",
  "-Dffi_unwind_calls",
  //"-Dfuzzy_provenance_casts", (U)
  "-Dif_let_rescope",
  "-Dimpl_trait_overcaptures",
  "-Dimpl_trait_redundant_captures",
  "-Dkeyword_idents_2018",
  "-Dkeyword_idents_2024",
  "-Dlet_underscore_drop",
  "-Dlinker_info",
  //"-Dlossy_provenance_casts", (U)
  "-Dmacro_use_extern_crate",
  "-Dmeta_variable_misuse",
  //"-Dmissing_copy_implementations", (PO)
  "-Dmissing_debug_implementations",
  "-Dmissing_docs",
  "-Dmissing_unsafe_on_extern",
  //"-Dmultiple_supertrait_upcastable", (U)
  //"-Dmust_not_suspend", (U)
  "-Dnon_ascii_idents",
  //"-Dnon_exhaustive_omitted_patterns", (U)
  "-Dredundant_imports",
  "-Dredundant_lifetimes",
  //"-Dresolving_to_items_shadowing_supertrait_items", (U)
  "-Drust_2021_incompatible_closure_captures",
  "-Drust_2021_incompatible_or_patterns",
  "-Drust_2021_prefixes_incompatible_syntax",
  "-Drust_2021_prelude_collisions",
  "-Drust_2024_guarded_string_incompatible_syntax",
  "-Drust_2024_incompatible_pat",
  "-Drust_2024_prelude_collisions",
  // "-Dshadowing_supertrait_items", (U)
  "-Dsingle_use_lifetimes",
  "-Dtail_expr_drop_order",
  "-Dtrivial_casts",
  "-Dtrivial_numeric_casts",
  "-Dunit_bindings",
  "-Dunnameable_types",
  //"-Dunqualified_local_imports", (U)
  "-Dunreachable_pub",
  "-Dunsafe_attr_outside_unsafe",
  "-Dunsafe_code",
  "-Dunsafe_op_in_unsafe_fn",
  //"-Dunstable_features", (MSTS)
  //"-Dunused_crate_dependencies", (HP)
  "-Dunused_extern_crates",
  "-Dunused_import_braces",
  "-Dunused_lifetimes",
  "-Dunused_macro_rules",
  "-Dunused_qualifications",
  "-Dunused_results",
  //"-Dvariant_size_differences", (PCV)
  "-Dwarnings",
];

const RUSTFMT_FLAGS: &[&str] =
  &[r#"edition="2021""#, "tab_spaces=2", r#"use_small_heuristics="Max""#];

#[derive(Debug, PartialEq)]
pub(crate) struct YouRust(pub(crate) Params);

impl Default for YouRust {
  fn default() -> Self {
    Self(Params {
      clippy_flags: CLIPPY_FLAGS.iter().map(|&err| err.into()).collect(),
      rust_flags: RUST_FLAGS.iter().map(|&err| err.into()).collect(),
      rustfmt_flags: RUSTFMT_FLAGS.iter().map(|&err| err.into()).collect(),
      toolchain: String::new(),
    })
  }
}
