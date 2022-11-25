import { rest } from "msw";
import { LoginRequest, LoginResponse } from "../../../../bindings";

const url = process.env.VITE_API_URL;

const testData = {
    authorizedMail: "Email",
    authorizedPassword: "Password"
}

export const handlers = [
    rest.get(`${url}/public.login`, async (req, res, ctx) => {
        const data = await req.json() as LoginRequest;
        if (data.email === testData.authorizedMail && data.password === testData.authorizedPassword) {
            sessionStorage.setItem('is-authenticated', 'true')
            return res(
                ctx.status(200),
                ctx.json({
                    t: "Success"
                } as LoginResponse)
            );
        }
        else
            return res(
                ctx.status(404)
            );
    }),

    rest.get(`${url}/user.me`, (_, res,) => {
    }),
]