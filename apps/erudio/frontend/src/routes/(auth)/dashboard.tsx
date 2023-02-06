import { Component, For, Match, Show, Switch } from "solid-js";
import { A, Navigate } from "solid-start";
import { SchoolRelationType } from "../../../../bindings";
import { useI18nContext } from "../../i18n/i18n-solid";
import createSession from "../../lib/session";

const SchoolCard: Component<{ school: [SchoolRelationType, string] }> = (props) => {
    const { LL } = useI18nContext()
    return (
        <div class="card border border-primary bg-base-100 text-primary hover:scale-110 hover:text-primary-focus hover:border-primary-focus shadow-xl hover:z-10 motion-safe:transition-transform">
            <div class="card-body">
                <h2 class="card-title truncate">{props.school[1]}</h2>
                <p>
                    <Switch>
                        <Match when={props.school[0] === "teacher"}>
                            {LL().dashboard.teacher()}
                        </Match>
                        <Match when={props.school[0] === "student"}>
                            {LL().dashboard.student()}
                        </Match>
                        <Match when={props.school[0] === "admin"}>
                            {LL().dashboard.admin()}
                        </Match>
                        <Match when={props.school[0] === "director"}>
                            {LL().dashboard.director()}
                        </Match>
                    </Switch>
                </p>
            </div>
        </div>
    )
}

export default function Dashboard() {
    const session = createSession();
    return (
        <>
            <Show when={session.data?.school_relations.length === 1}>
                <Navigate href={`/${encodeURIComponent(session.data!.school_relations[0][1]!)}/${session.data!.school_relations[0][0]}`} />
            </Show>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 m-3">
                <For each={session.data?.school_relations}>
                    {school =>
                        <A href={`/${encodeURIComponent(school[1])}/${school[0]}`}>
                            <SchoolCard school={school} />
                        </A>
                    }
                </For>
            </div>
        </>
    )
}
