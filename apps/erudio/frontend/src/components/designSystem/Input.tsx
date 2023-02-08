import { JSX, Show, VoidComponent } from "solid-js"

type TextInputProps = {
    type?: string,
    required?: boolean,
    disabled?: boolean,
    name?: string,
    onInput?: JSX.EventHandlerUnion<HTMLInputElement, InputEvent>
    display: string,
    errors?: string[] | null
}

export const TextInput: VoidComponent<TextInputProps> = (props) => (
    <div class="form-control">
        <label class="label">
            <span class="label-text">
                {props.display}
            </span>
        </label>
        <input type={props.type ?? "text"} required={props.required} disabled={props.disabled} name={props.name ?? props.display} id={props.name} classList={{ "input-error text-error": props.errors !== null }} class="input input-primary input-bordered" onInput={() => props.onInput} />
        <Show when={props.errors !== null}>
            <label class="label">
                <span class="label-text-alt text-error">{props.errors?.[0]}</span>
            </label>
        </Show>
    </div>
)
