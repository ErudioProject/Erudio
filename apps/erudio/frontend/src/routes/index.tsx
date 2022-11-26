import Button from '@suid/material/Button';
import { createRouteAction, createRouteData, FormError, FormProps, Navigate, redirect, useRouteData } from 'solid-start';
import { useClient } from '../components/contexts/ClientProvider';
import { Component, ParentComponent, Show } from 'solid-js';
import Alert from '@suid/material/Alert';
import { useI18nContext } from '../i18n/i18n-solid';
import { Stack, TextField } from '@suid/material';

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

interface LoginPageProps {
    FormElement: ParentComponent<FormProps>
    loading: boolean
    error: FormError
}

const LoginPage: Component<LoginPageProps> = (props) => {
    const { LL } = useI18nContext();
    return (
        <>
            <props.FormElement>
                <Stack textAlign="center" spacing={3} alignItems="center" justifyContent="center" sx={{ height: "100vh" }}>
                    <picture>
                        <source srcset="logo.svg" />
                        <img src="logo.svg" alt="Logo" style="width:200px;height:auto" />
                    </picture>
                    <TextField type="email" required label={LL().EMAIL()} name="email" disabled={props.loading} />
                    <TextField
                        required
                        type="password"
                        label={LL().PASSWORD()}
                        name="password"
                        disabled={props.loading}
                    />
                    <Button variant="contained" type="submit" disabled={props.loading}>
                        {LL().LOGINBUTTON()}
                    </Button>
                    <Show when={props.error}>
                        <Alert severity="error">{LL().INVALIDLOGIN()}</Alert>
                    </Show>
                </Stack>
            </props.FormElement>
        </>
    )
}

export default function Index() {
    const client = useClient();
    const me = useRouteData<typeof routeData>();
    const [logging, login] = createRouteAction(async (formData: FormData) => {
        const email = formData.get('email') as string;
        const password = formData.get('password') as string;
        try {
            let response = await client
                .getFetchClient()
                .query(['public.login', { email: email, password: password }])
            if (response.t === 'Success') return redirect('/dashboard');
        }
        catch (e) {
            throw new FormError('Invalid login data')
        }
    });
    return (
        <>
            <Show when={me.state !== "pending" && me.state !== "errored"}>
                <Navigate href="/dashboard" />
            </Show>
            <LoginPage FormElement={login.Form} loading={logging.pending} error={logging.error} />
        </>
    );
}
