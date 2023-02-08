use cookie::{Cookie, Expiration, SameSite};
use std::borrow::Cow;

pub fn get_cookie<'c, N, V, E>(name: N, value: V, expires: E) -> Cookie<'c>
where
	N: Into<Cow<'c, str>>,
	V: Into<Cow<'c, str>>,
	E: Into<Expiration>,
{
	Cookie::build(name, value)
		.secure(true)
		.http_only(true)
		.expires(expires)
		.same_site(SameSite::Strict)
		.finish()
}
