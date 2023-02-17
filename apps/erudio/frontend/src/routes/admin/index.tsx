import { createForm } from "@felte/solid";
import { validator } from "@felte/validator-zod";
import { createSignal, For, Show, Suspense } from "solid-js";
import { Portal } from "solid-js/web";
import { A, Outlet } from "solid-start";
import { z } from "zod";
import rspc from "../../api-setup";
import { TextInput } from "../../components/designSystem/Input";
import { useI18nContext } from "../../i18n/i18n-solid";

const schoolSkeleton = () => (
    <span class="text-base-200">
        Loading...
    </span>
)

export default function SchoolLayout() {
    const { LL } = useI18nContext()
    const [page, setPage] = createSignal(0);
    const [query, setQuery] = createSignal("");
    const [openForm, setOpenForm] = createSignal(false);
    const [serverError, setServerError] = createSignal<string | undefined>(undefined);
    const perPage = 10

    const addSchool = rspc.createMutation("super_admin.addSchool", {
        onSuccess: () => setOpenForm(false),
        onError: (error) => {
            setErrors({ name: " " })
            setServerError(error.message)
        }
    });

    const addSchoolSchema = z.object({
        name: z.string().min(1, LL().common.errors.required())
    })
    const { form, errors, setErrors, isSubmitting } = createForm<z.infer<typeof addSchoolSchema>>({
        extend: [validator({ schema: addSchoolSchema })],
        onSubmit: (values) => addSchool.mutate({
            idempotence_token: {
                token: crypto.randomUUID(),
                region: "REGION_TEST"
            },
            name: values.name
        })
    })

    const schools = rspc.createQuery(() => ['super_admin.searchSchools', {
        page: { skip: page() * perPage, take: perPage }, name: query()
    }], {
        keepPreviousData: true,
    })

    return (
        <>
            <Portal>
                <div class="modal" classList={{ "modal-open": openForm() }}>
                    <div class="modal-box">
                        <h3 class="font-bold text-lg">{LL().admin.addSchool()}</h3>
                        <form use:form>
                            <TextInput type="text" required disabled={isSubmitting()} name="name" display="Nazwa szkoły" errors={errors().name} />
                            <Show when={serverError()}>
                                <div class="alert alert-error mt-3">
                                    <div>
                                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                        <span>{serverError()}</span>
                                    </div>
                                </div>
                            </Show>
                            <div class="modal-action">
                                <button class="btn" type="submit">{LL().admin.create()}</button>
                                <button class="btn" onClick={() => setOpenForm(false)}>{LL().admin.cancel()}</button>
                            </div>
                        </form>
                    </div>
                </div>
            </Portal>
            <div class="grid grid-cols-2 p-5">
                <div>
                    <div>
                        <div class="form-control mb-5">
                            <label class="label">
                                <span class="label-text">{LL().admin.schoolName()}</span>
                            </label>
                            <div class="input-group">
                                <input type="text" class="input input-primary input-bordered" onInput={(e) => {
                                    setQuery(e.currentTarget.value);
                                    setPage(0)
                                }} />
                                <button class="btn btn-square" onClick={() => setOpenForm(true)}>+</button>
                            </div>
                        </div>
                    </div>
                    <div class="h-screen">
                        <ul class="menu bg-base-100 border border-primary shadow-lg rounded-box max-h-3/5 flex-nowrap overflow-auto">
                            <Suspense fallback={schoolSkeleton}>
                                <For each={schools.data} fallback={<ul class="text-error m-5">{LL().admin.schoolNotFound()}</ul>}>
                                    {school =>
                                        <li class="hover-bordered" classList={{ "text-base-200": schools.isPreviousData }}>
                                            <A href={`/admin/${school.id}`} activeClass="active" replace={true}>
                                                <div>
                                                    {school.name}
                                                </div>
                                            </A>
                                        </li>
                                    }
                                </For>
                            </Suspense>
                        </ul>
                        <div class="flex justify-center mt-3">
                            <div class="btn-group">
                                <button class="btn" classList={{ "btn-disabled": page() === 0 }} disabled={page() === 0} onClick={() => setPage(p => p - 1)}>«</button>
                                <button class="btn btn-active" disabled>{LL().admin.page(page())}</button>
                                <button class="btn" onClick={() => setPage(p => p + 1)}>»</button>
                            </div>
                        </div>
                    </div>
                </div>
                <div>
                    <Outlet />
                </div>
            </div>
        </>
    )


}
