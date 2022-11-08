use crate::{
	routes::{
		public::{ARGON_CONFIG, SALT_SIZE, SECRET_SIZE},
		RspcResult, SESSION_COOKIE_NAME,
	},
	Ctx,
};
use backend_error_handler::ApiError;
use backend_prisma_client::prisma::{pii_data, user, GrammaticalForm};
use backend_session_manager::init_session;
use cookie::SameSite;
use log::debug;
use rand::RngCore;
use rspc::Type;
use tower_cookies::Cookie;

#[derive(Type, serde::Deserialize, Debug)]
pub struct RegisterRequest {
	pub email: String,
	pub password: String,
	pub code: (),
}

pub(crate) async fn register(ctx: Ctx, req: RegisterRequest) -> RspcResult<()> {
	debug!("Register Request : {:?}", req);
	let mut salt = [0].repeat(SALT_SIZE);
	let mut connection_secret = [0].repeat(SECRET_SIZE);
	{
		let mut rng = rand::thread_rng(); // TODO Maybe change
		rng.fill_bytes(&mut salt);
		rng.fill_bytes(&mut connection_secret);
	}
	let user = ctx
		.db
		.user()
		.create(
			argon2::hash_raw(req.password.as_bytes(), &salt, &ARGON_CONFIG).map_err(Into::<ApiError>::into)?,
			false,
			GrammaticalForm::Indeterminate,
			vec![],
		)
		.exec()
		.await?;

	let pii_data = ctx
		.db
		.pii_data()
		.create(user::id::equals(user.id.clone()), vec![pii_data::email::Set(Some(
			req.email,
		))
		.into()])
		.exec()
		.await?;

	let user = user::Data {
		pii_data: Some(Some(Box::new(pii_data))),
		..user
	};

	ctx.cookies.add(
		Cookie::build(
			SESSION_COOKIE_NAME,
			init_session(&ctx.db, &ctx.redis, &user, &connection_secret, None).await?,
		)
		.secure(false) // TODO change one we have ssl set up
		.http_only(true)
		.same_site(SameSite::Strict)
		.finish(),
	);

	Ok(())
}
