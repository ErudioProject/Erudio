import { Show } from "solid-js";
import { Navigate, Outlet } from "solid-start";
import Nav from "../components/Nav";
import createSession from "../lib/session";

export default function AuthLayout() {
    const session = createSession();
    return (
        <>
            <Show when={session.isError}>
                <Navigate href="/" />
            </Show>
            <Show when={session.data}>
                <Nav displayName={session.data!.display_name} userId={session.data!.id} />
            </Show>
            <Outlet />
        </>
    );
}
