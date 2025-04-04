macro_rules! new_pub_const {
  ($n:ident, $v:expr) => {
    pub const $n: &'static str = $v;
  };
}

macro_rules! default_url {
  ($i:ident) => {
    impl $i {
      pub fn default() -> Self {
        Self { url: None }
      }
    }
  };
  ($i:ident, $v:expr) => {
    impl $i {
      pub fn default() -> crate::Result<Self> {
        Ok(Self {
          url: Url::parse($v)?,
        })
      }
    }
  };
}

macro_rules! default_url_w_token {
  ($i:ident) => {
    impl $i {
      pub fn default() -> Self {
        Self {
          url: None,
          token: None,
        }
      }
    }
  };
  ($i:ident, $v:expr) => {
    impl $i {
      pub fn default() -> crate::Result<Self> {
        Ok(Self {
          url: Url::parse($v)?,
          token: None,
        })
      }
    }
  };
}
