POST http://localhost:3001/rspc/public.register
{
	"idempotence_token": "TEST_REGION_<randomes here   >",
	"email": "aaaaaa",
	"password": "aaa",
	"first_name": "Jan",
	"last_name": "Kowalski",
	"code": null
}
HTTP 200

[Asserts]
jsonpath "$.result.type" == "response"



GET http://localhost:3001/rspc/user.logout
HTTP 200
[Asserts]
jsonpath "$.result.type" == "response"