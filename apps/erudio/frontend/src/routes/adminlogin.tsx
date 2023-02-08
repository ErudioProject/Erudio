import { createForm } from "@felte/solid";
import { validator } from "@felte/validator-zod";
import { createSignal, Show } from "solid-js";
import { Navigate } from "solid-start";
import { z } from "zod";
import rspc from "../api-setup";
import { TextInput } from "../components/designSystem/Input";
import { useI18nContext } from "../i18n/i18n-solid";
import { createAdminSession } from "../lib/session";

export default function AdminLogin() {
    const { LL } = useI18nContext();
    const [serverError, setServerError] = createSignal<string | null>(null);
    const [admin] = createAdminSession(true);
    const utils = rspc.useContext();

    const login = rspc.createMutation("public.login.admin", {
        onSuccess: () => utils.queryClient.invalidateQueries({ predicate: (query) => query.queryKey[0] === 'user.me' || query.queryKey[0] === 'super_admin.version' }),
        onError: () => {
            setErrors({ login: " ", password: " " });
            setServerError(LL().index.invalid());
        }
    });


    const AdminLoginRequestSchema = z.object({
        login: z.string().min(1, LL().index.errors.required()),
        password: z.string().min(1, LL().index.errors.required())
    })
    const { form, errors, setErrors, isSubmitting } = createForm<z.infer<typeof AdminLoginRequestSchema>>({
        extend: [validator({ schema: AdminLoginRequestSchema })],
        onSubmit: (values) => {
            login.mutate(values)
        },
    })

    return (
        <>
            <Show when={admin.data}>
                <Navigate href="/admin" />
            </Show>
            <Show when={admin.isError}>
                <form use:form>
                    <div class="mx-auto flex flex-col justify-center items-center h-screen gap-4">
                        <picture>
                            <source srcset="logo.svg" />
                            <img src="logo.svg" alt="Logo" style={{ "width": "200px", "height": "auto" }} />
                        </picture>
                        <TextInput type='text' name='login' required disabled={isSubmitting()} display={LL().index.login()} errors={errors().login} />
                        <TextInput type='password' name='password' required disabled={isSubmitting()} display={LL().index.password()} errors={errors().password} />
                        <button type="submit" class="btn btn-primary" classList={{ 'loading': isSubmitting() }}>
                            {LL().index.loginButton()}
                        </button>
                        <Show when={serverError()}>
                            <div class="alert alert-error w-auto">
                                <div class="flex-1">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-6 h-6 mx-2 stroke-current">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
                                    </svg>
                                    <p>{serverError()}</p>
                                </div>
                            </div>
                        </Show>
                    </div>
                </form>
            </Show>
        </>
    )
}
