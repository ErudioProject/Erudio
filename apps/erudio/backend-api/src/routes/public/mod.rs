mod login;
mod register;
mod version;

use crate::routes::public::login::login;
use crate::routes::public::register::register;
use crate::routes::public::version::version;
use crate::{helpers::idempotent, Public};
use rspc::{Router, RouterBuilder};

pub fn mount() -> RouterBuilder<Public> {
	Router::<Public>::new()
		.query("version", |t| t(version))
		.query("login", |t| t(login))
		.mutation("register", |t| {
			t(idempotent!(register, Public, register::RegisterRequest, ()))
		})
}
