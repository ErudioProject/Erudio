import Button from '@suid/material/Button';
import Stack from '@suid/material/Stack';
import TextField from '@suid/material/TextField';
import { createRouteAction, createRouteData, Navigate, redirect, useRouteData } from 'solid-start';
import { useClient } from '../components/contexts/ClientProvider';
import { ElementType } from '@suid/types/solid';
import { Show } from 'solid-js';
import Alert from '@suid/material/Alert';
import { LoginResponse } from '../../../bindings';
import { useI18nContext } from '../i18n/i18n-solid';


export function routeData() {
    const client = useClient()!;
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
    formElement: ElementType
}

function LoginPage(props: LoginPageProps) {
    const { LL } = useI18nContext();
    return (
        <Stack component={props.formElement} textAlign="center" spacing={2}>
            <TextField type="email" required label={LL().EMAIL()} name="email" />
            <TextField
                required
                type="password"
                label={LL().PASSWORD()}
                name="password"
            />
            <Button variant="contained" component="input" type="submit">
                {LL().LOGINBUTTON()}
            </Button>
        </Stack>
    )
}

export default function Index() {
    const client = useClient()!;
    const { LL } = useI18nContext();
    const me = useRouteData<typeof routeData>();
    const [logging, { Form }] = createRouteAction(async (formData: FormData) => {
        const email = formData.get('email') as string;
        const password = formData.get('password') as string;
        client
            .getFetchClient()
            .query(['public.login', { email: email, password: password }])
            .then((res: LoginResponse) => {
                if (res.t === 'Success') return redirect('/dashboard');
            })
            .catch((e: Error) => {
                throw e;
            });
    });
    return (
        <>
            <Show when={!me.error}>
                {/* <Navigate href="/dashboard" /> */}
            </Show>
            <LoginPage formElement={Form} />
            <Show when={logging.error}>
                <Alert severity="error">{LL().INVALIDLOGIN()}</Alert>
            </Show>
        </>
    );
}
