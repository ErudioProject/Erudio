mod me;

use crate::routes::{user::me::me, AuthCtx};
use rspc::{Router, RouterBuilder};

pub fn mount() -> RouterBuilder<AuthCtx> {
	Router::<AuthCtx>::new().query("me", |t| t(me))
}
