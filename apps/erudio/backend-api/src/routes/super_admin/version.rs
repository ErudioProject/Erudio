use crate::helpers::ctx::SuperAdmin;

pub fn version(_: SuperAdmin, _: ()) -> String {
	env!("CARGO_PKG_VERSION").to_string()
}
