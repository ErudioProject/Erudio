macro_rules! idempotent {
	// This match captures the `expr` passed in as `$e`,
	// which the macro will assume is callable (E.g. a closure or function)
	($e:expr, $c:ty, $r:ty, $ret:ty) => {{
		async fn wrap(ctx: $c, req: $r) -> RspcResult<$ret> {
			let mut redis: MultiplexedConnection = ctx.redis.clone();
			let idempotence_token = req.idempotence_token.clone();

			if !idempotence_token.starts_with(&ctx.region_id) {
				return Err(rspc::Error::new(
					ErrorCode::InternalServerError,
					"NO REGION HANDLING YET".into(),
				));
			}

			let load: Option<(bool, String)> = redis
				.get(&idempotence_token)
				.await
				.map_err(Into::<InternalError>::into)?;
			if let Some(s) = load {
				return if s.0 {
					Ok(serde_json::from_str(&s.1).unwrap())
				} else {
					// TODO replace with too many requests
					Err(rspc::Error::new(ErrorCode::Conflict, "TOO MANY REQUESTS".into()))
				};
			}

			redis
				.set_ex(&idempotence_token, (false, ""), 60 * 60)
				.await
				.map_err(Into::<InternalError>::into)?;

			match $e(ctx, req).await {
				Ok(res) => {
					let json = serde_json::to_string(&res).map_err(Into::<InternalError>::into)?;
					redis
						.set_ex(&idempotence_token, (true, json.clone()), 60 * 60)
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
