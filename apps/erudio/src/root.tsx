// @refresh reload
import { createSignal, Show, Suspense } from "solid-js";
import {
    Body,
    ErrorBoundary,
    FileRoutes,
    Head,
    Html,
    Meta,
    Routes,
    Scripts,
    Title,
} from "solid-start";
import "./root.css";
import { loadLocaleAsync, TypesafeI18n, usePrefferedLocale } from "@erudio/frontend/data-access/i18n";
import Container from "@suid/material/Container";
import { ClientProvider } from "@erudio/frontend/data-access/api";

export default function Root() {
    //TODO: fix/file vite issue:
    //const lang = usePrefferedLocale("pl");
    const lang = "pl";
    const [localeLoaded, setLocaleLoaded] = createSignal(false);
    loadLocaleAsync(lang).then(() => setLocaleLoaded(true));
    return (
        <Html lang={lang}>
            <Head>
                <Title>SolidStart - Bare</Title>
                <Meta charset="utf-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1" />
            </Head>
            <Body>
                <Suspense>
                    <ErrorBoundary>
                        <Show when={localeLoaded()}>
                            <TypesafeI18n locale={lang}>
                                <ClientProvider url="https://localhost:3001">
                                    <Container>
                                        <Routes>
                                            <FileRoutes />
                                        </Routes>
                                    </Container>
                                </ClientProvider>
                            </TypesafeI18n>
                        </Show>
                    </ErrorBoundary>
                </Suspense>
                <Scripts />
            </Body>
        </Html>
    );
}
