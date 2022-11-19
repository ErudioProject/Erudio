import { createPalette } from "@suid/material/styles/createPalette";

export const getDesignTokens = (mode: "light" | "dark") => createPalette({
    mode,
    ...(mode === "light"
        ? {
            primary: {
                main: "#ff0000"
            },
            background: {
                paper: "rgba(0, 0, 0, 0.04)"
            }
        }
        : {
            primary: {
                main: "#00ff00"
            },
            background: {
                paper: "#000000"
            }
        })
});
