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
import Container from '@suid/material/Container';
import ClientProvider from './components/contexts/ClientProvider';

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
                <Suspense>
                    <ErrorBoundary>
                        <Localization locale={lang}>
                            <ClientProvider url={import.meta.env.FRONTEND_API_URL}>
                                <Container>
                                    <Routes>
                                        <FileRoutes />
                                    </Routes>
                                </Container>
                            </ClientProvider>
                        </Localization>
                    </ErrorBoundary>
                </Suspense>
                <Scripts />
            </Body>
        </Html>
    );
}
