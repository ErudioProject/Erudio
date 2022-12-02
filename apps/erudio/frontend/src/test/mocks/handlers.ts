import { rest, RestRequest } from "msw";
import { LoginRequest, LoginResponse, SchoolRelationType } from "../../../../bindings";

const url = "http://127.0.0.1:3001/rspc" //process.env.FRONTEND_API_URL;

export const apiTestData = {
    authorizedMail: "test@example.com",
    authorizedPassword: "Test123!",

    displayName: "Test User",
    schoolRelations: [
        {
            school_relation_type: "student" as SchoolRelationType,
            school: { name: "My School" }
        }, {
            school_relation_type: "teacher" as SchoolRelationType,
            school: { name: "My School 2" }
        }
    ]
}

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

type UserMeResponse = { pii_data: { display_name: string } | null, user_school_relation: Array<{ school_relation_type: SchoolRelationType, school: { name: string } }> }

export const handlers = [
    rest.get(`${url}/user.me`, (_, res, ctx) => {
        if (sessionStorage.getItem('is-authenticated') === 'true') {
            return res(
                ctx.status(200),
                ctx.json(wrapResponse<UserMeResponse>({
                    pii_data: {
                        display_name: apiTestData.displayName
                    },
                    user_school_relation: apiTestData.schoolRelations
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
