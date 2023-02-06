use crate::cookies::get_cookie;
use crate::helpers::consts::ADMIN_COOKIE_NAME;
use crate::routes::public::login::TwoFactorAuthType;
use crate::{routes::RspcResult, Public};
use error_handler::InternalError;
use log::info;
use prisma_client::prisma::{pii_data, super_admin, user, GrammaticalForm};
use rand::RngCore;
use rspc::{ErrorCode, Type};
use services::session;

#[serde_zod::codegen]
#[derive(Type, serde::Deserialize, Debug)]
pub struct AdminLoginRequest {
	pub login: String,
	pub password: String,
}

#[derive(Type, serde::Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum AdminLoginResponse {
	Success,
	#[allow(dead_code)] // TODO
	TwoFactorAuth(TwoFactorAuthType),
}

pub async fn admin_login(ctx: Public, req: AdminLoginRequest) -> RspcResult<AdminLoginResponse> {
	info!("Login Request: {:?}", req);
	if !ctx.ip.is_loopback() {
		// allows only local host
		return Err(InternalError::IntoRspc(ErrorCode::NotFound, None).into());
	}

	let user = ctx
		.db
		.super_admin()
		.find_unique(super_admin::UniqueWhereParam::LoginEquals(req.login))
		.exec()
		.await?
		.ok_or_else(|| InternalError::IntoRspc(ErrorCode::NotFound, None))?;

	if !argon2::verify_encoded_ext(
		&user.password_hash,
		req.password.as_bytes(),
		&ctx.config.argon2.secret,
		&[],
	)
	.map_err(Into::<InternalError>::into)?
	{
		return Err(InternalError::IntoRspc(ErrorCode::NotFound, None).into());
	}

	let mut connection_secret = vec![0; ctx.config.secret_size];
	{
		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut connection_secret);
	}
	let pseudo_user = ctx
		.db
		._transaction()
		.run(|db| async move {
			let user = db
				.user()
				.upsert(
					user::UniqueWhereParam::IdEquals("070d8538-c810-41a1-ac20-dd31d76110e9".to_string()),
					user::create("Pseudo Admin User".to_string(), vec![]),
					vec![],
				)
				.exec()
				.await?;

			db.pii_data()
				.upsert(
					pii_data::UniqueWhereParam::IdEquals("070d8538-c810-41a1-ac20-dd31d76110e9".to_string()),
					pii_data::create(
						GrammaticalForm::Indeterminate,
						"Pseudo admin user".to_string(),
						"Pseudo admin user".to_string(),
						user::id::equals(user.id.clone()),
						vec![],
					),
					vec![],
				)
				.exec()
				.await
				.map_err(Into::<InternalError>::into)
				.map(|pii_data| user::Data {
					pii_data: Some(Some(Box::new(pii_data))),
					..user
				})
		})
		.await?;

	ctx.cookies.add(get_cookie(
		ADMIN_COOKIE_NAME,
		session::init::session(
			&ctx.db,
			&mut ctx.redis.clone(),
			pseudo_user,
			&connection_secret,
			Some(3600),
		)
		.await?,
	));

	Ok(AdminLoginResponse::Success)
}
