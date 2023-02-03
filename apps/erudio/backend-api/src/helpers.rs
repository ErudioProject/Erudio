pub mod argon;
pub mod config;
pub mod ctx;

#[derive(Serialize, Deserialize)]
pub struct IdempotentSaveState {
	pub finished: bool,
	pub data: String,
}

macro_rules! idempotent {
	// This match captures the `expr` passed in as `$e`,
	// which the macro will assume is callable (E.g. a closure or function)
	($e:expr, $c:ty, $r:ty, $ret:ty) => {{
		use crate::helpers::IdempotentSaveState;
		async fn wrap(ctx: $c, req: $r) -> RspcResult<$ret> {
			let mut redis = ctx.redis.clone();
			let idempotence_token = req.idempotence_token.clone();

			if !idempotence_token.starts_with(&ctx.config.region_id) {
				return Err(rspc::Error::new(
					ErrorCode::InternalServerError,
					"NO REGION HANDLING YET".into(),
				));
			}

			let load: Option<String> = redis
				.get(&idempotence_token)
				.await
				.map_err(Into::<InternalError>::into)?;
			if let Some(s) = load {
				let state: IdempotentSaveState = serde_json::from_str(&s).unwrap();
				return if state.finished {
					Ok(serde_json::from_str(&state.data).unwrap())
				} else {
					// TODO replace with too many requests
					Err(rspc::Error::new(ErrorCode::Conflict, "TOO MANY REQUESTS".into()))
				};
			}

			let state = IdempotentSaveState {
				finished: false,
				data: "".to_string(),
			};
			redis
				.set_ex(
					&idempotence_token,
					serde_json::to_string(&state).map_err(Into::<InternalError>::into)?,
					60 * 60,
				)
				.await
				.map_err(Into::<InternalError>::into)?;

			match $e(ctx, req).await {
				Ok(res) => {
					let json = serde_json::to_string(&res).map_err(Into::<InternalError>::into)?;
					let state = IdempotentSaveState {
						finished: true,
						data: json.clone(),
					};
					redis
						.set_ex(
							&idempotence_token,
							serde_json::to_string(&state).map_err(Into::<InternalError>::into)?,
							60 * 60,
						)
						.await
						.map_err(Into::<InternalError>::into)?;

					Ok(res)
				}
				Err(err) => {
					redis
						.expire(&idempotence_token, 0)
						.await
						.map_err(Into::<InternalError>::into)?;
					Err(err)
				}
			}
		}
		wrap
	}};
}

pub(crate) use idempotent;
use serde::{Deserialize, Serialize};
