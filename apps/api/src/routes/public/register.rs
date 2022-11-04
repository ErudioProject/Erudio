use crate::{
	error_mapping::RspcError,
	prisma::{pii_data, user},
	routes::{
		public::{ARGON_CONFIG, SALT_SIZE, SECRET_SIZE},
		RspcResult, SESSION_COOKIE_NAME,
	},
	Ctx, GrammaticalForm,
};
use log::debug;
use rand::RngCore;
use redis::AsyncCommands;
use rspc::{RouterBuilder, Type};
use tower_cookies::Cookie;

pub(crate) trait RegisterBuilder {
	fn register_query(self, key: &'static str) -> Self;
}

impl RegisterBuilder for RouterBuilder<Ctx> {
	fn register_query(self, key: &'static str) -> Self {
		self.query(key, |t| {
			#[derive(Type, serde::Deserialize, Debug)]
			pub struct RegisterRequest {
				pub email: String,
				pub password: String,
				pub code: (),
			}

			t(|ctx: Ctx, req: RegisterRequest| async move {
				debug!("Register Request : {:?}", req);
				let mut salt = [0].repeat(SALT_SIZE);
				let mut connection_secret = [0].repeat(SECRET_SIZE);
				{
					let mut rng = rand::thread_rng(); // Maybe change
					rng.fill_bytes(&mut salt);
					rng.fill_bytes(&mut connection_secret);
				}
				let user = ctx
					.db
					.user()
					.create(
						argon2::hash_raw(req.password.as_bytes(), &salt, &ARGON_CONFIG)
							.map_err(Into::<RspcError>::into)?,
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

				let json = serde_json::to_string(&user).map_err(Into::<RspcError>::into)?;
				{
					let mut conn = ctx.redis.lock().await;
					conn.set(&connection_secret, json)
						.await
						.map_err(Into::<RspcError>::into)?;
					conn.lpush(user.id, connection_secret.clone())
						.await
						.map_err(Into::<RspcError>::into)?; // Reverse record for fast logout
				}
				ctx.cookies
					.add(Cookie::build(SESSION_COOKIE_NAME, hex::encode(&connection_secret)).finish());

				Ok(()) as RspcResult<()>
			})
		})
	}
}
