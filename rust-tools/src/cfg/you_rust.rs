use crate::Params;

const CLIPPY_FLAGS: &[&str] = &[
  "-Dclippy::pedantic",
  "-Dclippy::restriction",
  "-Dwarnings",
  "-Aclippy::absolute_paths",
  "-Aclippy::big_endian_bytes",
  "-Aclippy::blanket-clippy-restriction-lints",
  "-Aclippy::decimal_literal_representation",
  "-Aclippy::default_numeric_fallback",
  "-Aclippy::error_impl_error",
  "-Aclippy::exhaustive_enums",
  "-Aclippy::exhaustive_structs",
  "-Aclippy::host_endian_bytes",
  "-Aclippy::ignored_unit_patterns",
  "-Aclippy::impl_trait_in_params",
  "-Aclippy::implicit_return",
  "-Aclippy::inline_always",
  "-Aclippy::integer_division",
  "-Aclippy::integer_division_remainder_used",
  "-Aclippy::into_iter_without_iter",
  "-Aclippy::len_without_is_empty",
  "-Aclippy::let_underscore_untyped",
  "-Aclippy::many_single_char_names",
  "-Aclippy::min_ident_chars",
  "-Aclippy::missing_assert_message",
  "-Aclippy::missing_docs_in_private_items",
  "-Aclippy::missing_errors_doc",
  "-Aclippy::missing_trait_methods",
  "-Aclippy::module_name_repetitions",
  "-Aclippy::multiple_inherent_impl",
  "-Aclippy::must_use_candidate",
  "-Aclippy::needless_doctest_main",
  "-Aclippy::pattern_type_mismatch",
  "-Aclippy::pub_use",
  "-Aclippy::pub_with_shorthand",
  "-Aclippy::question_mark_used",
  "-Aclippy::self_named_module_files",
  "-Aclippy::semicolon_outside_block",
  "-Aclippy::similar_names",
  "-Aclippy::single_call_fn",
  "-Aclippy::struct_field_names",
  "-Aclippy::unseparated_literal_suffix",
  "-Aclippy::used_underscore_binding",
  "-Aclippy::wildcard_enum_match_arm",
];

const RUST_FLAGS: &[&str] = &[
  "-Ddeprecated_in_future",
  "-Ddeprecated_safe",
  "-Delided_lifetimes_in_paths",
  "-Dexplicit_outlives_requirements",
  "-Dffi_unwind_calls",
  "-Dkeyword_idents_2018",
  "-Dkeyword_idents_2024",
  "-Dlet_underscore_drop",
  "-Dmacro_use_extern_crate",
  "-Dmeta_variable_misuse",
  "-Dmissing_abi",
  "-Dmissing_debug_implementations",
  "-Dmissing_docs",
  "-Dmissing_unsafe_on_extern",
  "-Dnon_ascii_idents",
  "-Dredundant_lifetimes",
  "-Drust_2021_incompatible_closure_captures",
  "-Drust_2021_incompatible_or_patterns",
  "-Drust_2021_prefixes_incompatible_syntax",
  "-Drust_2021_prelude_collisions",
  "-Dsingle_use_lifetimes",
  "-Dtrivial_casts",
  "-Dtrivial_numeric_casts",
  "-Dunit_bindings",
  "-Dunnameable_types",
  "-Dunreachable_pub",
  "-Dunsafe_code",
  "-Dunused_extern_crates",
  "-Dunused_import_braces",
  "-Dunused_lifetimes",
  "-Dunused_macro_rules",
  "-Dunused_qualifications",
  "-Dunused_results",
  "-Dvariant_size_differences",
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
