mod logout;
mod me;

use crate::{
	helpers::ctx::AuthCtx,
	routes::user::{logout::logout, me::me},
};
use rspc::{Router, RouterBuilder};

pub(crate) fn mount() -> RouterBuilder<AuthCtx> {
	Router::<AuthCtx>::new()
		.query("me", |t| t(me))
		.query("logout", |t| t(logout))
}
