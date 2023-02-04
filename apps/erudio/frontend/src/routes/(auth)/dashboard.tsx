import { Component, For, Show, Suspense } from "solid-js";
import { A, Navigate } from "solid-start";
import { SchoolRelationType } from "../../../../bindings";
import rspc from "../../api-setup";

const SchoolCard: Component<{ school: [SchoolRelationType, string] }> = (props) => {
    return (
        <div class="card border border-primary text-primary hover:text-primary-focus hover:border-primary-focus">
            <div class="card-body">
                <h2 class="card-title truncate">{props.school[1]}</h2>
                <p>{props.school[0]}</p>
            </div>
        </div>
    )
}

export default function Dashboard() {
    const me = rspc.createQuery(() => ["user.me"]);
    return (
        <>
            <Show when={me.data?.school_relations.length === 1}>
                <Navigate href={`/user/${encodeURIComponent(me.data?.school_relations[0][1]!)}`} />
            </Show>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 m-3">
                <Suspense>
                    <For each={me.data?.school_relations}>
                        {school =>
                            <A href={`/user/${encodeURIComponent(school[1])}`}>
                                <SchoolCard school={school} />
                            </A>
                        }
                    </For>
                </Suspense>
            </div>
        </>
    )
}
