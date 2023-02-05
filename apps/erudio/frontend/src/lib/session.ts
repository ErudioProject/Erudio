import rspc from "../api-setup";

const createSession = (login = false) => rspc.createQuery(() => ['user.me'], { retry: login ? false : 3, refetchOnWindowFocus: !login })

export default createSession;
