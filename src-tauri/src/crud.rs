use rbatis::Rbatis;
use once_cell::sync::Lazy;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

