import { SessionFlow, getMe } from '@erudio/authentication';

export function routeData() {
  return getMe();
}

export default function Index() {
  return <SessionFlow />;
}
