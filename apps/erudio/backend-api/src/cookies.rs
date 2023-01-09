use cookie::{Cookie, SameSite};
use std::borrow::Cow;

pub fn get_cookie<'c, N, V>(name: N, value: V) -> Cookie<'c>
where
	N: Into<Cow<'c, str>>,
	V: Into<Cow<'c, str>>,
{
	Cookie::build(name, value)
		.secure(false) // TODO change one we have ssl set up
		.http_only(true)
		.same_site(SameSite::Strict)
		.finish()
}
