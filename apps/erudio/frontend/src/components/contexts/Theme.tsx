import { createTheme, FormControlLabel, Switch, ThemeProvider } from "@suid/material";
import { createPalette } from "@suid/material/styles/createPalette";
import { Accessor, createContext, createSignal, ParentComponent, ParentProps, useContext } from "solid-js";
import { useI18nContext } from "../../i18n/i18n-solid";
import { getDesignTokens } from "../../theme";

type ModeProviderProps = {
    defaultMode: "light" | "dark",
}

interface ModeContextType {
    switchMode(): void,
    mode: Accessor<"light" | "dark">
}

const ModeContext = createContext<ModeContextType>();
const useMode = () => useContext(ModeContext)!

const ModeProvider: ParentComponent<ModeProviderProps> = (props) => {
    const [mode, setMode] = createSignal(props.defaultMode);
    const modeContext: ModeContextType = {
        mode,
        switchMode() {
            if (mode() === "light")
                setMode("dark")
            else setMode("light")
        },
    }
    return (
        <ModeContext.Provider value={modeContext}>
            {props.children}
        </ModeContext.Provider>
    )
}

const ErudioThemeProvider: ParentComponent = (props) => {
    const mode = useMode();
    const palette = () => createPalette(getDesignTokens(mode.mode()))
    const theme = createTheme({ palette });
    return (
        <ThemeProvider theme={theme}>
            {props.children}
        </ThemeProvider>
    )
}

type ThemeProps = ModeProviderProps;
export default function Theme(props: ParentProps<ThemeProps>) {
    return (
        <ModeProvider defaultMode={props.defaultMode}>
            <ErudioThemeProvider>
                {props.children}
            </ErudioThemeProvider>
        </ModeProvider>
    )
}

function NonLabeledSwitch() {
    const mode = useMode();
    const isChecked = () => mode.mode() === "dark";

    const switchHandler = () => {
        mode.switchMode()
    }

    return (
        <Switch checked={isChecked()} onChange={switchHandler} />
    )
}

export function ModeSwitch() {
    const { LL } = useI18nContext();

    return (
        <FormControlLabel control={NonLabeledSwitch} label={LL().DARKMODE()} labelPlacement="start" />
    )

}
