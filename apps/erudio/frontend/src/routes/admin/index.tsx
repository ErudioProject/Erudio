import { createSignal, For, Suspense } from "solid-js";
import { A, Outlet } from "solid-start";
import rspc from "../../api-setup";
import { useI18nContext } from "../../i18n/i18n-solid";

const schoolSkeleton = () => (
    <>
        Loading...
    </>
)

export default function SchoolLayout() {
    const { LL } = useI18nContext()
    const [page, setPage] = createSignal(0);
    const [query, setQuery] = createSignal("");
    const perPage = 20

    const schools = rspc.createQuery(() => ['super_admin.searchSchools', {
        page: { skip: page() * perPage, take: perPage }, name: query()
    }], {
        keepPreviousData: true
    })

    return (
        <div class="grid-cols-2">
            <div>
                <div>
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">{LL().admin.schoolName()}</span>
                        </label>
                        <div class="input-group">
                            <input type="text" class="input input-primary input-bordered" onInput={(e) => setQuery(e.currentTarget.value)} />
                            <button class="btn btn-square">+</button>
                        </div>
                    </div>
                </div>
                <Suspense fallback={schoolSkeleton}>
                    <For each={schools.data}>
                        {school =>
                            <A href={`/admin/${school.id}`}>
                                <div>
                                    {school.id}
                                </div>
                            </A>
                        }
                    </For>
                </Suspense>
            </div>
            <div>
                <Outlet />
            </div>
        </div>
    )


}
