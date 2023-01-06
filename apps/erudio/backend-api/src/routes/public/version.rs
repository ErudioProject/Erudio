use crate::Public;

pub fn version(_: Public, _: ()) -> String {
	env!("CARGO_PKG_VERSION").to_string()
}
