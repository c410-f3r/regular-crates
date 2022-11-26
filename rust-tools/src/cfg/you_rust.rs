use crate::Params;

const CLIPPY_FLAGS: &[&str] = &[
  "-Dclippy::restriction",
  "-Dwarnings",
  "-Aclippy::default_numeric_fallback",
  "-Aclippy::exhaustive_enums",
  "-Aclippy::exhaustive_structs",
  "-Aclippy::implicit_return",
  "-Aclippy::len_without_is_empty",
  "-Aclippy::missing_docs_in_private_items",
  "-Aclippy::multiple_inherent_impl",
  "-Aclippy::pub_use",
  "-Aclippy::self_named_module_files",
  "-Aclippy::std_instead_of_core",
  "-Aclippy::unseparated_literal_suffix",
];

const RUST_FLAGS: &[&str] = &[
  "-Dabsolute_paths_not_starting_with_crate",
  "-Danonymous_parameters",
  "-Delided_lifetimes_in_paths",
  "-Dexplicit_outlives_requirements",
  "-Dinvalid_html_tags",
  "-Dkeyword_idents",
  "-Dmacro_use_extern_crate",
  "-Dmeta_variable_misuse",
  "-Dmissing_crate_level_docs",
  "-Dmissing_debug_implementations",
  "-Dmissing_doc_code_examples",
  "-Dmissing_docs",
  "-Dnon_ascii_idents",
  "-Dpointer_structural_match",
  "-Dprivate_doc_tests",
  "-Dtrivial_casts",
  "-Dtrivial_numeric_casts",
  "-Dunaligned_references",
  "-Dunreachable_pub",
  "-Dunsafe_code",
  "-Dunstable_features",
  "-Dunused_crate_dependencies",
  "-Dunused_extern_crates",
  "-Dunused_import_braces",
  "-Dunused_lifetimes",
  "-Dunused_qualifications",
  "-Dunused_results",
  "-Dvariant_size_differences",
  "-Dwarnings",
];

const RUSTFMT_FLAGS: &[&str] = &[
  r#"edition="2021""#,
  "tab_spaces=2",
  "use_field_init_shorthand=true",
  r#"use_small_heuristics="Max""#,
];

#[derive(Debug, PartialEq)]
pub(crate) struct YouRust(pub(crate) Params);

impl Default for YouRust {
  fn default() -> Self {
    Self(Params {
      clippy_flags: CLIPPY_FLAGS.iter().map(|&e| e.into()).collect(),
      rust_flags: RUST_FLAGS.iter().map(|&e| e.into()).collect(),
      rustfmt_flags: RUSTFMT_FLAGS.iter().map(|&e| e.into()).collect(),
      toolchain: "".into(),
    })
  }
}
