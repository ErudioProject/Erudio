import { rest, RestRequest } from "msw";
import { LoginRequest, LoginResponse, SchoolRelationType, UserMeResponse } from "../../../../bindings";
import { apiTestData } from "../data";

const url = "http://localhost:4000/rspc" //TODO: process.env.FRONTEND_API_URL;

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
    rest.get(`${url}/user.me`, (_, res, ctx) => {
        if (sessionStorage.getItem('is-authenticated') === 'true') {
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<UserMeResponse>({
                    display_name: apiTestData.displayName,
                    school_relations: apiTestData.schoolRelations as [SchoolRelationType, string][],
                    id: apiTestData.userId
                }))
            )
        }
        else {
            return res(
                ctx.status(200),
                ctx.json(wrapError({
                    code: 401,
                    message: "Unauthorized",
                    data: null
                }))
            )
        }
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
]
