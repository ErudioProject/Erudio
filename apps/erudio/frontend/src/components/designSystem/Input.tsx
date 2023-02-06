import { Show, VoidComponent } from "solid-js"

type TextInputProps = {
    type: string,
    required: boolean,
    disabled: boolean,
    name: string,
    display: string,
    errors: string[] | null
}

export const TextInput: VoidComponent<TextInputProps> = (props) => (
    <div class="form-control">
        <label class="label">
            <span class="label-text">
                {props.display}
            </span>
        </label>
        <input type={props.type} required={props.required} disabled={props.disabled} name={props.name} id={props.name} classList={{ "input-error text-error": props.errors !== null }} class="input input-primary input-bordered" />
        <Show when={props.errors !== null}>
            <label class="label">
                <span class="label-text-alt text-error">{props.errors?.[0]}</span>
            </label>
        </Show>
    </div>
)
