import { RSPCError } from "@rspc/client";
import { CreateQueryResult } from "@tanstack/solid-query";
import { UserMeResponse } from "../../../bindings";
import rspc from "../api-setup";

const createSession = (login = false): CreateQueryResult<UserMeResponse, RSPCError> => rspc.createQuery(() => ['user.me'], {
    retry: login ? false : 3,
    refetchOnWindowFocus: !login,
    refetchInterval: login ? false : 1000 * 60 * 5,
    retryOnMount: false
})
export const createAdminSession = (login = false): [CreateQueryResult<string, RSPCError>, CreateQueryResult<UserMeResponse, RSPCError>] => {
    const adminData = rspc.createQuery(() => ['super_admin.version'], {
        retry: login ? false : 3,
        refetchOnWindowFocus: !login,
        refetchInterval: login ? false : 1000 * 60 * 5,
        retryOnMount: false
    });
    const userData = createSession(login);

    return [adminData, userData]
}

export default createSession;
