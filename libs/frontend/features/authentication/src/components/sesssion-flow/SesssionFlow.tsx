import { createRouteAction, createRouteData, Navigate, redirect, useRouteData } from "solid-start";
import { Show } from "solid-js";
import { useClient } from "@erudio/frontend/data-access/api";
import { LoginPage } from "@erudio/frontend/ui/login-page";
import { useI18nContext } from "@erudio/frontend/data-access/i18n";
import Alert from "@suid/material/Alert";

export function getMe() {
    return createRouteData(
        async () => {
            const client = useClient()!;
            return (await client.getFetchClient().query(["user.me"]));
        }
    );
}

function SessionFlow() {
    const me = useRouteData<typeof getMe>();
    const { LL } = useI18nContext();
    const client = useClient()!;
    const [logging, { Form }] = createRouteAction(
        async (formData: FormData) => {
            const email = formData.get("email") as string;
            const password = formData.get("password") as string;
            const res = await client.getFetchClient().query(["public.login", { email: email, password: password }]);
            if (res.t === "Success")
                return redirect("/dashboard")
            throw new Error("Invalid login details")
        }
    );
    return (
        <>
            <Show when={me()}>
                <Navigate href="/dashboard" />
            </Show>
            <LoginPage formElement={Form} />
            <Show when={logging.error}>
                <Alert severity="error">{LL().INVALIDLOGIN()}</Alert>
            </Show>
        </>
    );
}

export default SessionFlow;
