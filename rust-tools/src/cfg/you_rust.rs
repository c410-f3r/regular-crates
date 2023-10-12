use crate::Params;

const CLIPPY_FLAGS: &[&str] = &[
  "-Dclippy::pedantic",
  "-Dclippy::restriction",
  "-Dwarnings",
  "-Aclippy::big_endian_bytes",
  "-Aclippy::blanket-clippy-restriction-lints",
  "-Aclippy::default_numeric_fallback",
  "-Aclippy::doc_markdown",
  "-Aclippy::absolute_paths",
  "-Aclippy::error_impl_error",
  "-Aclippy::exhaustive_enums",
  "-Aclippy::exhaustive_structs",
  "-Aclippy::host_endian_bytes",
  "-Aclippy::ignored_unit_patterns",
  "-Aclippy::implicit_return",
  "-Aclippy::integer_division",
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
  "-Aclippy::multiple_unsafe_ops_per_block",
  "-Aclippy::must_use_candidate",
  "-Aclippy::needless_else",
  "-Aclippy::non_ascii_literal",
  "-Aclippy::partial_pub_fields",
  "-Aclippy::pattern_type_mismatch",
  "-Aclippy::pub_use",
  "-Aclippy::pub_with_shorthand",
  "-Aclippy::question_mark_used",
  "-Aclippy::self_named_module_files",
  "-Aclippy::semicolon_outside_block",
  "-Aclippy::similar_names",
  "-Aclippy::single_call_fn",
  "-Aclippy::std_instead_of_core",
  "-Aclippy::struct_excessive_bools",
  "-Aclippy::unseparated_literal_suffix",
  "-Aclippy::unused_self",
];

const RUST_FLAGS: &[&str] = &[
  "-Dabsolute_paths_not_starting_with_crate",
  "-Ddeprecated_in_future",
  "-Delided_lifetimes_in_paths",
  "-Dexplicit_outlives_requirements",
  "-Dkeyword_idents",
  "-Dlet_underscore_drop",
  "-Dmacro_use_extern_crate",
  "-Dmeta_variable_misuse",
  "-Dmissing_abi",
  "-Dmissing_debug_implementations",
  "-Dmissing_docs",
  "-Dnon_ascii_idents",
  "-Dnoop_method_call",
  "-Dpointer_structural_match",
  "-Drust_2021_incompatible_closure_captures",
  "-Drust_2021_incompatible_or_patterns",
  "-Drust_2021_prefixes_incompatible_syntax",
  "-Drust_2021_prelude_collisions",
  "-Dsingle_use_lifetimes",
  "-Dtrivial_casts",
  "-Dtrivial_numeric_casts",
  "-Dunreachable_pub",
  "-Dunsafe_code",
  "-Dunsafe_op_in_unsafe_fn",
  "-Dunused_extern_crates",
  "-Dunused_import_braces",
  "-Dunused_lifetimes",
  "-Dunused_macro_rules",
  "-Dunused_qualifications",
  "-Dunused_results",
  "-Dunused_tuple_struct_fields",
  "-Dwarnings",
];

const RUSTFMT_FLAGS: &[&str] =
  &[r#"edition="2021""#, "tab_spaces=2", r#"use_small_heuristics="Max""#];

#[derive(Debug, PartialEq)]
pub(crate) struct YouRust(pub(crate) Params);

impl Default for YouRust {
  fn default() -> Self {
    Self(Params {
      clippy_flags: CLIPPY_FLAGS.iter().map(|&e| e.into()).collect(),
      rust_flags: RUST_FLAGS.iter().map(|&e| e.into()).collect(),
      rustfmt_flags: RUSTFMT_FLAGS.iter().map(|&e| e.into()).collect(),
      toolchain: String::new(),
    })
  }
}
