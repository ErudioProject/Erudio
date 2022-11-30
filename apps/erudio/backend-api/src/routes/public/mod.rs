mod login;
mod register;
mod version;

use login::*;
use register::*;
use version::*;

use crate::{helpers::idempotent, routes::RspcResult, Ctx};
use error_handler::InternalError;
use redis::AsyncCommands;
use rspc::{ErrorCode, Router, RouterBuilder};

pub(crate) fn mount() -> RouterBuilder<Ctx> {
	Router::<Ctx>::new()
		.query("version", |t| t(version))
		.query("login", |t| t(login))
		.mutation("register", |t| t(idempotent!(register, Ctx, RegisterRequest, ())))
}
