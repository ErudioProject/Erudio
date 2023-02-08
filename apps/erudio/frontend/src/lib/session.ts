import { RSPCError } from "@rspc/client";
import { CreateQueryResult } from "@tanstack/solid-query";
import { UserMeResponse } from "../../../bindings";
import rspc from "../api-setup";

const createSession = (): CreateQueryResult<UserMeResponse, RSPCError> => rspc.createQuery(() => ['user.me'], {
    retry: (retryCount, error) => error.code !== 401 && retryCount < 3 ? true : false,
    refetchInterval: 1000 * 60 * 5,
    retryOnMount: false
})
export const createAdminSession = () => {
    const userData = createSession();
    const adminData = rspc.createQuery(() => ['super_admin.version'], {
        retry: (retryCount, error) => error.code !== 401 && retryCount < 3 ? true : false,
        refetchInterval: 1000 * 60 * 5,
        retryOnMount: false
    });

    return [adminData, userData] as const
}

export default createSession;
