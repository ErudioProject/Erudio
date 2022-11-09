import { detectLocale, localStorageDetector, navigatorDetector } from "typesafe-i18n/detectors";
import { Locales } from "../i18n/i18n-types";
import { locales } from "../i18n/i18n-util";

export function usePrefferedLocale(fallbackLocale: Locales): Locales {
    let detectedLocale = detectLocale(fallbackLocale, locales, localStorageDetector)
    if (detectedLocale === fallbackLocale)
        detectedLocale = detectLocale(fallbackLocale, locales, navigatorDetector);
    return detectedLocale;
}
