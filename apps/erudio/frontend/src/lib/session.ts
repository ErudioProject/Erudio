import rspc from "../api-setup";

const createSession = (login = false) => rspc.createQuery(() => ['user.me'], {
    retry: login ? false : 3,
    refetchOnWindowFocus: !login,
    refetchInterval: login ? false : 1000 * 60 * 5
})

export default createSession;
