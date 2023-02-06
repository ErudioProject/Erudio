import { Component } from "solid-js";
import Me from "./Me";

type NavProps = {
    displayName: string,
    userId: string
}

const getAvatarColor = (id: string) => {
    let hash = 0;
    for (let i = 0; i < id.length; i++) {
        hash = id.charCodeAt(i) + ((hash << 5) - hash);
    }
    var colour = '#';
    for (let i = 0; i < 3; i++) {
        let value = (hash >> (i * 8)) & 0xFF;
        colour += ('00' + value.toString(16)).substr(-2);
    }
    return colour;
}
const Nav: Component<NavProps> = (props) => {
    return (
        <div class="navbar bg-base-100 w-full bg-opacity-90 backdrop-blur shadow-sm">
            <div class="flex-1">
                <picture>
                    <source srcset="logo.svg" />
                    <img src="logo.svg" alt="Logo" style={{ "width": "200px", "height": "auto" }} />
                </picture>
            </div>
            <div class="flex-none">
                <Me displayName={props.displayName} bgColor={getAvatarColor(props.userId)} />
            </div>
        </div>
    )
}

export default Nav;
