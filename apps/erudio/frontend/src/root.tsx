// @refresh reload
import { Suspense } from 'solid-js';
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
} from 'solid-start';
import './root.css';
import Localization from './components/contexts/Localization';
import LoadingPage from './components/LoadingPage';
import rspc, { client, queryClient } from './api-setup';

export default function Root() {
    //TODO: fix/file vite issue:
    //const lang = usePrefferedLocale("pl");
    const lang = 'pl';
    return (
        <Html lang={lang}>
            <Head>
                <Title>SolidStart - Bare</Title>
                <Meta charset="utf-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1" />
            </Head>
            <Body>
                <ErrorBoundary>
                    <Suspense fallback={LoadingPage}>
                        <rspc.Provider client={client} queryClient={queryClient}>
                            <Localization locale={lang}>
                                <div class="mx-5">
                                    <Routes>
                                        <FileRoutes />
                                    </Routes>
                                </div>
                            </Localization>
                        </rspc.Provider>
                    </Suspense>
                </ErrorBoundary>
                <Scripts />
            </Body>
        </Html>
    );
}
