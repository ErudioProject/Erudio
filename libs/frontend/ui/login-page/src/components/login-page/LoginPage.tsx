import Box from "@suid/material/Box";
import TextField from "@suid/material/TextField";
import { useI18nContext } from "@erudio/i18n";
import { ElementType } from "@suid/types/solid";
import Button from "@suid/material/Button";

interface LoginPageProps {
    formElement: ElementType
}

function LoginPage(props: LoginPageProps) {
    const { LL } = useI18nContext()
    return (
        <Box
            component={props.formElement}
            textAlign="center">
            <TextField
                type="email"
                required
                label={LL().EMAIL()}
                name="email" />
            <TextField
                required
                type="password"
                label={LL().PASSWORD()}
                name="password"
            />
            <Button variant="contained" component="input" type="submit">
                {LL().LOGINBUTTON()}
            </Button>
        </Box>
    )
}

export default LoginPage;
