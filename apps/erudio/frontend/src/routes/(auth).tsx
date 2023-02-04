import { Show } from "solid-js";
import { createRouteData, Navigate, Outlet, useRouteData } from "solid-start";
import { FetchClient } from "../api-setup";
import Nav from "../components/Nav";

export function routeData() {
    return createRouteData(async () => {
        return await FetchClient
            .query(['user.me'])
    });
}

export default function AuthLayout() {
    const me = useRouteData<typeof routeData>();
    return (
        <>
            <Show when={me.state !== "pending" && me.state === "errored"}>
                <Navigate href="/" />
            </Show>
            <Nav displayName={me()?.display_name ?? ""} userId={me()?.id ?? ""} />
            <Outlet />
        </>
    );
}
