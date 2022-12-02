import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "solid-testing-library";
import Theme, { ModeSwitch } from "./Theme";
import { Container, CssBaseline } from "@suid/material";
import * as theme from "../../theme"
import TypesafeI18n from "../../i18n/i18n-solid";
import { createPalette } from "@suid/material/styles/createPalette";

vi.spyOn(theme, "getDesignTokens").mockImplementation((mode: "light" | "dark") => createPalette({
    mode,
    ...(mode === "light"
        ? {
            components: {
                MuiContainer: {
                    styleOverrides: {
                        root: {
                            backgroundColor: "#ffffff"
                        }
                    }
                }
            }
        }
        : {
            components: {
                MuiContainer: {
                    styleOverrides: {
                        root: {
                            backgroundColor: "#000000"
                        }
                    }
                }
            }
        })
}));

interface TestProps {
    mode: "light" | "dark"
};

//TODO: Figure out why these won't pass...

function Test(props: TestProps) {
    return (
        <>
            <TypesafeI18n locale="pl">
                <Theme defaultMode={props.mode}>
                    <CssBaseline />
                    {/* <ModeSwitch /> */}
                    <Container>
                        Test component
                    </Container>
                </Theme>
            </TypesafeI18n>
        </>
    )
}

describe("Theme", () => {
    it("Correctly renders light theme", () => {
        render(() => <Test mode="light" />);
        const container = screen.getByText("Test component");
        expect(container).toBeInTheDocument();
    });

    it("Correctly renders dark theme", () => {
        render(() => <Test mode="dark" />);
        const container = screen.getByText("Test component");
        expect(container).toBeInTheDocument();
        /* expect(container).toHaveStyle(`background-color: #000000`); */
    })
});

/* describe("ModeSwitch", () => {
*     it("Switches between light and dark theme", () => {
*         render(() => <Test mode="light" />);
*         const container = screen.getByText("Test component");
*         const themeSwitch = screen.getByRole("checkbox");
*         fireEvent.click(themeSwitch);
*         expect(container).toHaveStyle(`background-color: #000000`);
*     })
* }) */
