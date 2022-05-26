create_enum_with_list! {
    crate::Error::UnknownAction;

    #[derive(Debug)]
    pub(crate) enum ActionOption {
        BuildGeneric, "build-generic";
        BuildWithFeatures, "build-with-features";
        CheckGeneric, "check-generic";
        CheckWithFeatures, "check-with-features";
        Clippy, "clippy";
        RustFlags, "rust-flags";
        Rustfmt, "rustfmt";
        SetUp, "set-up";
        TargetDir, "target-dir";
        TestGeneric, "test-generic";
        TestWithFeatures, "test-with-features";
    }
}
