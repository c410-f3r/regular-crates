use crate::{cfg::YouRust, Params};

create_enum_with_list! {
    crate::Error::UnknownCfg;

    #[derive(Debug)]
    pub(crate) enum CfgOption {
      YouRust, "you-rust";
    }
}

impl CfgOption {
  pub(crate) fn into_params(self) -> Params {
    match self {
      Self::YouRust => YouRust::default().0,
    }
  }
}
