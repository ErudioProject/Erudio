import { Component } from "solid-js";

interface MeProps {
    displayName: string
    bgColor: string
}

const Me: Component<MeProps> = (props) => {
    return (
        <>
            <div class="avatar placeholder mr-3">
                <div class="text-neutral-content rounded-full w-10" style={{ "background-color": props.bgColor }}>
                    <span class="text-xl">{props.displayName[0]}
                    </span>
                </div>
            </div>
            <label>{props.displayName}</label>
        </>
    )
}


export default Me;
