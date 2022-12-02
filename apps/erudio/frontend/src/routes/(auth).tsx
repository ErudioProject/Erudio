import { Show } from "solid-js";
import { createRouteData, Navigate, Outlet, useRouteData } from "solid-start";
import { useClient } from "../components/contexts/ClientProvider";

export function routeData() {
    const client = useClient();
    return createRouteData(async () => {
        return client
            .getFetchClient()
            .query(['user.me'])
            .catch((e: Error) => {
                throw e;
            });
    });
}

export default function AuthLayout() {
    const me = useRouteData<typeof routeData>();
    return (
        <>
            <Show when={me.state !== "pending" && me.state === "errored"}>
                <Navigate href="/" />
            </Show>
            <Outlet />
        </>
    );
}
