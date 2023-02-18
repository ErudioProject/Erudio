pub mod argon;
pub mod consts;
pub mod ctx;
pub mod pagination;
pub mod seed;

#[derive(Serialize, Deserialize)]
pub struct IdempotentSaveState {
	pub finished: bool,
	pub data: String,
}

#[derive(rspc::Type, serde::Serialize, Deserialize, Debug, Clone)]
pub struct IdempotenceToken {
	pub region: String,
	pub token: String,
}

macro_rules! idempotent {
	// This match captures the `expr` passed in as `$e`,
	// which the macro will assume is callable (E.g. a closure or function)
	($e:expr, $c:ty, $r:ty, $ret:ty) => {{
		use redis::AsyncCommands;
		async fn wrap(ctx: $c, req: $r) -> crate::routes::RspcResult<$ret> {
			let mut redis = ctx.redis.clone();
			let idempotence_token: crate::helpers::IdempotenceToken = req.idempotence_token.clone();

			if !idempotence_token.region.eq(&ctx.config.region_id) {
				return Err(error_handler::InternalError::ServerError(
					"No region handling YET".to_string(),
					color_eyre::eyre::eyre!(""),
				)
				.into());
			}

			let load: Option<String> = redis
				.get(&idempotence_token.token)
				.await
				.map_err(Into::<error_handler::InternalError>::into)?;
			if let Some(s) = load {
				let state: crate::helpers::IdempotentSaveState = serde_json::from_str(&s).unwrap();
				return if state.finished {
					Ok(serde_json::from_str(&state.data).unwrap())
				} else {
					// TODO replace with too many requests
					Err(rspc::Error::new(rspc::ErrorCode::Conflict, "TOO MANY REQUESTS".into()))
				};
			}

			let state = crate::helpers::IdempotentSaveState {
				finished: false,
				data: "".to_string(),
			};
			redis
				.set_ex(
					&idempotence_token.token,
					serde_json::to_string(&state).map_err(Into::<error_handler::InternalError>::into)?,
					60 * 60,
				)
				.await
				.map_err(Into::<error_handler::InternalError>::into)?;

			match $e(ctx, req).await {
				Ok(res) => {
					let json = serde_json::to_string(&res).map_err(Into::<error_handler::InternalError>::into)?;
					let state = crate::helpers::IdempotentSaveState {
						finished: true,
						data: json.clone(),
					};
					redis
						.set_ex(
							&idempotence_token.token,
							serde_json::to_string(&state).map_err(Into::<error_handler::InternalError>::into)?,
							60 * 60,
						)
						.await
						.map_err(Into::<error_handler::InternalError>::into)?;

					Ok(res)
				}
				Err(err) => {
					redis
						.expire(&idempotence_token.token, 0)
						.await
						.map_err(Into::<error_handler::InternalError>::into)?;
					Err(err)
				}
			}
		}
		wrap
	}};
}

pub(crate) use idempotent;
use serde::{Deserialize, Serialize};
