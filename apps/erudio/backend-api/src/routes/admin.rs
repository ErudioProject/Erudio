use crate::helpers::ctx::SuperAdmin;
use rspc::{Router, RouterBuilder};

mod version;
use version::version;

pub fn mount() -> RouterBuilder<SuperAdmin> {
	Router::<SuperAdmin>::new().query("version", |t| t(version))
}
