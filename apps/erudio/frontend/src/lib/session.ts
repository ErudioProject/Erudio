import { RSPCError } from "@rspc/client";
import { CreateQueryResult } from "@tanstack/solid-query";
import { UserMeResponse } from "../../../bindings";
import rspc from "../api-setup";

const createSession = (login = false): CreateQueryResult<UserMeResponse, RSPCError> => rspc.createQuery(() => ['user.me'], {
    retry: login ? false : (retryCount, error) => error.code !== 401 && retryCount < 3 ? true : false,
    refetchOnWindowFocus: !login,
    refetchInterval: login ? false : 1000 * 60 * 5,
    retryOnMount: false
})
export const createAdminSession = (login = false) => {
    const userData = createSession(login);
    const adminData = rspc.createQuery(() => ['super_admin.version'], {
        retry: login ? false : (retryCount, error) => error.code !== 401 && retryCount < 3 ? true : false,
        refetchOnWindowFocus: !login,
        refetchInterval: login ? false : 1000 * 60 * 5,
        retryOnMount: false
    });

    return [adminData, userData] as const
}

export default createSession;
