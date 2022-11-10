import { useI18nContext } from "@erudio/frontend/data-access/i18n";
import Button from "@suid/material/Button";

export default function Index() {
    const { LL } = useI18nContext();
    return (
        <Button variant="contained">{LL().HI({ name: "Patryk" })}</Button>
    )
}
