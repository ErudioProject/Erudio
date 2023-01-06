mod logout;
mod me;

use crate::{
	helpers::ctx::Auth,
	routes::user::{logout::logout, me::me},
};
use rspc::{Router, RouterBuilder};

pub fn mount() -> RouterBuilder<Auth> {
	Router::<Auth>::new()
		.query("me", |t| t(me))
		.query("logout", |t| t(logout))
}
