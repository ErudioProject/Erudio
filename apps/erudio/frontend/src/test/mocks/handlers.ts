import { rest, RestRequest } from "msw";
import { AdminLoginRequest, AdminLoginResponse, LoginRequest, LoginResponse, School, UserMeResponse } from "../../../../bindings";
import clientEnv from "../../lib/env";
import { apiTestData } from "../data";

const url = clientEnv.VITE_API_URL;

type RSPCError = {
    code: number,
    message: string,
    data: object | null
}

function wrapResponse<T>(data: T) {
    return {
        jsonrpc: "2.0",
        id: null,
        result: {
            type: "response",
            data
        }
    }
}

function wrapError(data: RSPCError) {
    return {
        jsonrpc: "2.0",
        id: null,
        result: {
            type: "error",
            data
        }
    }
}

function getData<T>(req: RestRequest): T {
    return JSON.parse(req.url.searchParams.get("input") ?? "{}") as T
}

async function postData<T>(req: RestRequest): Promise<T> {
    return await req.json()
}

export const handlers = [
    rest.get(`${url}/super_admin.searchSchools`, (_, res, ctx) => {
        if (sessionStorage.getItem('is-admin') === 'true') {
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<Array<School>>(apiTestData.schools))
            )
        }
        return res(
            ctx.status(200),
            ctx.json(wrapError({
                code: 401,
                message: "Unauthorized",
                data: null
            }))
        )
    }),
    rest.get(`${url}/super_admin.version`, (_, res, ctx) => {
        if (sessionStorage.getItem('is-admin') === 'true') {
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<string>("0.0.1"))
            )
        }
        return res(
            ctx.status(200),
            ctx.json(wrapError({
                code: 401,
                message: "Unauthorized",
                data: null
            }))
        )
    }),
    rest.get(`${url}/user.me`, (_, res, ctx) => {
        if (sessionStorage.getItem('is-authenticated') === 'true') {
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<UserMeResponse>({
                    display_name: apiTestData.displayName,
                    school_relations: apiTestData.schoolRelations,
                    id: apiTestData.userId
                }))
            )
        }
        return res(
            ctx.status(200),
            ctx.json(wrapError({
                code: 401,
                message: "Unauthorized",
                data: null
            }))
        )
    }),
    rest.post(`${url}/public.login`, async (req, res, ctx) => {
        const data = await postData<LoginRequest>(req);
        if (data.email === apiTestData.authorizedMail && data.password === apiTestData.authorizedPassword) {
            sessionStorage.setItem('is-authenticated', 'true')
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<LoginResponse>({ t: "Success" }))
            );
        }
        else
            return res(
                ctx.status(200),
                ctx.json(wrapError({
                    code: 404,
                    message: "Email not found",
                    data: null
                }))
            );
    }),
    rest.post(`${url}/public.login.admin`, async (req, res, ctx) => {
        const data = await postData<AdminLoginRequest>(req);
        if (data.login === apiTestData.authorizedMail && data.password === apiTestData.authorizedPassword) {
            sessionStorage.setItem('is-admin', 'true')
            sessionStorage.setItem('is-authenticated', 'true')
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<AdminLoginResponse>({ t: "Success" }))
            );
        }
        else
            return res(
                ctx.status(200),
                ctx.json(wrapError({
                    code: 404,
                    message: "Email not found",
                    data: null
                }))
            );
    }),
]
