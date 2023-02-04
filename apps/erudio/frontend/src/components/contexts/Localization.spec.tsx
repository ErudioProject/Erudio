import { createPreferredLocale } from './Localization';
import { Locales } from '../../i18n/i18n-types';
import { vi, describe, beforeEach, it, expect } from 'vitest';
import { waitFor } from 'solid-testing-library';
import { GetOptions, GetResult } from '@capacitor/preferences';

let preferencesItems: Record<string, GetResult> = {};
let navigatorLanguages: Array<string> = [];
let fallbackLocale: Locales = 'pl';

vi.spyOn(navigator, 'languages', 'get').mockImplementation(() => {
    return navigatorLanguages;
});
vi.mock("@capacitor/preferences", () => {
    return {
        ...vi.importActual("@capacitor/preferences"),
        Preferences: {
            get: vi.fn().mockImplementation((options: GetOptions) => Promise.resolve(preferencesItems[options.key] ?? { value: null }))
        }
    }
})

describe('createPreferredLocale', () => {
    beforeEach(() => {
        preferencesItems = {};
        navigatorLanguages = [];
        fallbackLocale = 'pl';
    });

    it('detects correct locale for Preferences API', async () => {
        preferencesItems = { data: { value: 'garbage' }, lang: { value: 'de' } };
        navigatorLanguages = ['es'];
        const [locale] = createPreferredLocale(fallbackLocale);
        await waitFor(() => expect(locale()).toBe("de"));
    });
    it('detects correct locale for navigator', async () => {
        navigatorLanguages = ['de', 'es'];
        const [locale] = createPreferredLocale(fallbackLocale);
        await waitFor(() => expect(locale()).toBe("de"));
    });
    it('returns correct fallback locale', async () => {
        const [locale] = createPreferredLocale(fallbackLocale);
        await waitFor(() => expect(locale()).toBe("pl"));
    });
});
