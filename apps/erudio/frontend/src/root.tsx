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
import LoadingPage from './components/placeholders/LoadingPage';
import Theme, { ModeSwitch } from './components/contexts/Theme';
import { CssBaseline } from '@suid/material';

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
                <Suspense fallback={LoadingPage}>
                    <ErrorBoundary>
                        <Localization locale={lang}>
                            <ClientProvider url={import.meta.env.FRONTEND_API_URL}>
                                <Theme defaultMode='dark'>
                                    <CssBaseline />
                                    <ModeSwitch />
                                    <Container sx={{ height: "100vh" }}>
                                        <Routes>
                                            <FileRoutes />
                                        </Routes>
                                    </Container>
                                </Theme>
                            </ClientProvider>
                        </Localization>
                    </ErrorBoundary>
                </Suspense>
                <Scripts />
            </Body>
        </Html>
    );
}
