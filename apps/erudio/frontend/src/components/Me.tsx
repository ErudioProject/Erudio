import { Component } from "solid-js";
import rspc from "../api-setup";
import { useI18nContext } from "../i18n/i18n-solid";

type MeProps = {
    displayName: string
    bgColor: string
}

const Me: Component<MeProps> = (props) => {
    const { LL } = useI18nContext()
    const utils = rspc.useContext();
    const logout = rspc.createMutation(['user.logout'], {
        onSuccess: () => utils.queryClient.invalidateQueries(['user.me'])
    });

    return (
        <div class="dropdown dropdown-left">
            <label tabindex="0" class="btn btn-ghost rounded-btn">
                <div class="avatar placeholder">
                    <div class="text-neutral-content rounded-full w-10" style={{ "background-color": props.bgColor }}>
                        <span class="text-xl">{props.displayName[0]}</span>
                    </div>
                </div>
                <label class="hidden md:inline ml-3">{props.displayName}</label>
            </label>
            <ul tabindex="0" class="dropdown-content menu p-2 mr-2 shadow bg-base-200 rounded-box w-52">
                <li><button onClick={() => logout.mutate(undefined)}>{LL().user.logout()}</button></li>
            </ul>
        </div>
    )
}


export default Me;
