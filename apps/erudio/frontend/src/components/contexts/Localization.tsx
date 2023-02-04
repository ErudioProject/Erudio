import { Preferences } from '@capacitor/preferences';
import { createEffect, createResource, createSignal, ParentProps, Show } from 'solid-js';
import {
    detectLocale,
    navigatorDetector,
} from 'typesafe-i18n/detectors';
import TypesafeI18n from '../../i18n/i18n-solid';
import { Locales } from '../../i18n/i18n-types';
import { baseLocale, isLocale, locales } from '../../i18n/i18n-util';
import { loadLocaleAsync } from '../../i18n/i18n-util.async';

export default function Localization(props: ParentProps) {
    const [localeLoaded, setLocaleLoaded] = createSignal(false);
    const [locale] = createPreferredLocale(baseLocale)
    createEffect(() => locale.state === "ready" && loadLocaleAsync(locale()).then(() => setLocaleLoaded(true)))
    return (
        <Show when={localeLoaded()}>
            <TypesafeI18n locale={locale()!}>
                {props.children}
            </TypesafeI18n>
        </Show>
    );
}

async function detectPreferencesLocale(fallbackLocale: Locales): Promise<Locales> {
    const { value } = await Preferences.get({ key: "lang" })
    if (isLocale(value ?? ""))
        return value as Locales
    return fallbackLocale
}

export function createPreferredLocale(fallbackLocale: Locales) {
    return createResource(async () => {
        let detectedLocale = await detectPreferencesLocale(fallbackLocale);
        if (detectedLocale === fallbackLocale)
            detectedLocale = detectLocale(fallbackLocale, locales, navigatorDetector);
        return detectedLocale
    });
}
