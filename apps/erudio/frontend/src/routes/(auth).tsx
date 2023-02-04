import { Show } from "solid-js";
import { Navigate, Outlet } from "solid-start";
import rspc from "../api-setup";
import Nav from "../components/Nav";

export default function AuthLayout() {
    const me = rspc.createQuery(() => ["user.me"], { retry: false });
    return (
        <>
            <Show when={me.isError}>
                <Navigate href="/" />
            </Show>
            <Nav displayName={me.data?.display_name ?? ""} userId={me.data?.id ?? ""} />
            <Outlet />
        </>
    );
}
