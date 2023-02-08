import { Show } from "solid-js";
import { Navigate, Outlet } from "solid-start";
import Nav from "../components/Nav";
import { createAdminSession } from "../lib/session";

export default function AdminLayout() {
    const [admin, session] = createAdminSession();
    return (
        <>
            <Show when={admin.isError}>
                <Navigate href="/adminlogin" />
            </Show>
            <Show when={session.data}>
                <Nav displayName={session.data!.display_name} userId={session.data!.id} />
                <Outlet />
            </Show>
        </>
    );
}
