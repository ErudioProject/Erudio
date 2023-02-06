mod admin_login;
pub mod login;
pub mod register;
pub mod version;

use crate::routes::public::admin_login::admin_login;
use crate::routes::public::login::login;
use crate::routes::public::register::register;
use crate::routes::public::version::version;
use crate::{helpers::idempotent, Public};
use rspc::{Router, RouterBuilder};

pub fn mount() -> RouterBuilder<Public> {
	Router::<Public>::new()
		.query("version", |t| t(version))
		.mutation("login", |t| t(login))
		.mutation("login.admin", |t| t(admin_login))
		.mutation("register", |t| {
			t(idempotent!(register, Public, register::RegisterRequest, ()))
		})
}
