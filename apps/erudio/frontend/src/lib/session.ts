import { RSPCError } from "@rspc/client";
import { CreateQueryResult } from "@tanstack/solid-query";
import { UserMeResponse } from "../../../bindings";
import rspc from "../api-setup";

const retryFunction = (retryCount: number, error: RSPCError) => {
    if (retryCount >= 3 || (error.code >= 300 && error.code < 500))
        return false;
    return true
}

const createSession = (): CreateQueryResult<UserMeResponse, RSPCError> => rspc.createQuery(() => ['user.me'], {
    retry: retryFunction,
    refetchInterval: 1000 * 60 * 5,
    retryOnMount: false
})
export const createAdminSession = () => {
    const userData = createSession();
    const adminData = rspc.createQuery(() => ['super_admin.version'], {
        retry: retryFunction,
        refetchInterval: 1000 * 60 * 5,
        retryOnMount: false
    });

    return [adminData, userData] as const
}

export default createSession;
