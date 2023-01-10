import { rest, RestRequest } from "msw";
import { LoginRequest, LoginResponse, SchoolRelationType, UserMeResponse } from "../../../../bindings";
import { apiTestData } from "../data";

const url = "http://127.0.0.1:3001/rspc" //TODO: process.env.FRONTEND_API_URL;

interface RSPCError {
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
    rest.get(`${url}/public.login`, (req, res, ctx) => {
        const data = getData<LoginRequest>(req);
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
