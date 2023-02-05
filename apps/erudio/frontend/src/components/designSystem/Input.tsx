import { ValidationMessage } from "@felte/reporter-solid"
import { VoidComponent } from "solid-js"

type TextInputProps = {
    type: string,
    required: boolean,
    disabled: boolean,
    name: string,
    display: string,
    touched: boolean
}

export const TextInput: VoidComponent<TextInputProps> = (props) => (
    <div class="form-control">
        <label class="label">
            <span class="label-text">
                {props.display}
            </span>
        </label>
        <input type={props.type} required={props.required} disabled={props.disabled} name={props.name} id={props.name} classList={{ "invalid:input-error invalid:text-error": props.touched }} class="input input-primary input-bordered" />
        <label class="label">
            <ValidationMessage for={props.name}>
                {(messages) =>
                    <span class="label-text-alt text-error">{messages?.[0]}</span>
                }
            </ValidationMessage>
        </label>
    </div>
)
