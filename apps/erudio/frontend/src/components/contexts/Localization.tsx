import { createSignal, Show } from 'solid-js';
import { JSX } from 'solid-js/jsx-runtime';
import {
    detectLocale,
    localStorageDetector,
    navigatorDetector,
} from 'typesafe-i18n/detectors';
import TypesafeI18n from '../../i18n/i18n-solid';
import { Locales } from '../../i18n/i18n-types';
import { locales } from '../../i18n/i18n-util';
import { loadLocaleAsync } from '../../i18n/i18n-util.async';

interface LocalizationProps {
    locale: Locales
    children?: JSX.Element,
}

export default function Localization(props: LocalizationProps) {
    const [localeLoaded, setLocaleLoaded] = createSignal(false);
    loadLocaleAsync(props.locale).then(() => setLocaleLoaded(true));
    return (
        <Show when={localeLoaded()}>
            <TypesafeI18n locale={props.locale}>
                {props.children}
            </TypesafeI18n>
        </Show>
    );
}

export function usePrefferedLocale(fallbackLocale: Locales): Locales {
    let detectedLocale = detectLocale(
        fallbackLocale,
        locales,
        localStorageDetector
    );
    if (detectedLocale === fallbackLocale)
        detectedLocale = detectLocale(fallbackLocale, locales, navigatorDetector);
    return detectedLocale;
}
