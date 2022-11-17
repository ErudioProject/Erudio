import { createRouteAction, createRouteData, Navigate, redirect, useRouteData } from "solid-start";
import { Show } from "solid-js";
import { useClient } from "@erudio/api";
import { LoginPage } from "@erudio/login-page";
import { useI18nContext } from "@erudio/i18n";
import Alert from "@suid/material/Alert";

export function getMe() {
    return createRouteData(
        async () => {
            const client = useClient()!;
            return client.getFetchClient().query(["user.me"]).catch(e => {
                throw e;
            });
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
            client.getFetchClient().query(["public.login", { email: email, password: password }])
                .then(res => {
                    if (res.t === "Success")
                        return redirect("/dashboard")
                })
                .catch(e => {
                    throw e
                })
        }
    );
    return (
        <>
            <Show when={!me.error}>
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
