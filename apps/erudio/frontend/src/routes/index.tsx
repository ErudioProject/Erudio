import { createRouteAction, createRouteData, FormError, FormProps, Navigate, redirect, useRouteData } from 'solid-start';
import { Component, ParentComponent, Show } from 'solid-js';
import { useI18nContext } from '../i18n/i18n-solid';
import { FetchClient } from '../api-setup';
import LoadingPage from '../components/LoadingPage';

export function routeData() {
    return createRouteData(async () => {
        return await FetchClient
            .query(['user.me'])
    });
}

type LoginPageProps = {
    FormElement: ParentComponent<FormProps>
    loading: boolean
    error: FormError
}

const LoginPage: Component<LoginPageProps> = (props) => {
    const { LL } = useI18nContext();
    return (
        <>
            <props.FormElement>
                <div class="mx-auto flex flex-col justify-center items-center h-screen gap-4">
                    <picture>
                        <source srcset="logo.svg" />
                        <img src="logo.svg" alt="Logo" style={{ "width": "200px", "height": "auto" }} />
                    </picture>
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">{LL().EMAIL()}</span>
                        </label>
                        <input type="email" required disabled={props.loading} name="email" class="input input-primary input-bordered" />
                    </div>
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">{LL().PASSWORD()}</span>
                        </label>
                        <input type="password" required disabled={props.loading} name="password" class="input input-primary input-bordered" />
                    </div>
                    <button type="submit" class="btn btn-primary" classList={{ 'loading': props.loading }}>
                        {LL().LOGINBUTTON()}
                    </button>
                    <Show when={props.error}>
                        <div class="alert alert-error w-auto">
                            <div class="flex-1">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-6 h-6 mx-2 stroke-current">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
                                </svg>
                                <p>{LL().INVALIDLOGIN()}</p>
                            </div>
                        </div>
                    </Show>
                </div>
            </props.FormElement>
        </>
    )
}

export default function Index() {
    const me = useRouteData<typeof routeData>();
    const [logging, login] = createRouteAction(async (formData: FormData) => {
        const email = formData.get('email') as string;
        const password = formData.get('password') as string;
        try {
            let response = await FetchClient
                .query(['public.login', { email: email, password: password }])
            if (response.t === 'Success') return redirect('/dashboard');
        }
        catch (e) {
            throw new FormError('Invalid login data')
        }
    });
    return (
        <>
            <Show when={me.state === "pending"}
                fallback={<LoginPage FormElement={login.Form} loading={logging.pending} error={logging.error} />}>
                <LoadingPage />
            </Show>
            <Show when={me.state !== "pending" && me.state !== "errored"}>
                <Navigate href="/dashboard" />
            </Show>
        </>
    );
}
