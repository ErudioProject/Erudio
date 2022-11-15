import { SessionFlow, getMe } from "@erudio/frontend/features/authentication";

export function routeData() {
    return getMe();
}

export default function Index() {
    return (
        <SessionFlow />
    )
}
