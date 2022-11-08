/* import { HopeProvider, NotificationsProvider } from "@hope-ui/solid"; */
import { Outlet, Title } from "solid-start";

export default function LangLayout() {
    return (
        <HopeProvider>
            <NotificationsProvider>
                <Outlet />
            </NotificationsProvider>
        </HopeProvider>
    );
}
