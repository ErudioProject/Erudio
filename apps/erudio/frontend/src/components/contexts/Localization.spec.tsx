import { usePrefferedLocale } from './Localization';
import { Locales } from './i18n/i18n-types';
import { vi, describe, beforeEach, it, expect } from 'vitest';

let localStorageItems: Record<string, string> = {};
let navigatorLanguages: Array<string> = [];
let fallbackLocale: Locales = 'pl';

vi.spyOn(navigator, 'languages', 'get').mockImplementation(() => {
    return navigatorLanguages;
});
vi.spyOn(Storage.prototype, 'setItem');
Storage.prototype.getItem = vi.fn((key: string) => localStorageItems[key]);

describe('usePreferredLocale', () => {
    beforeEach(() => {
        localStorageItems = {};
        navigatorLanguages = [];
        fallbackLocale = 'pl';
    });

    it('detects correct locale for localStorage', () => {
        localStorageItems = { data: 'garbage', lang: 'de' };
        navigatorLanguages = ['en'];
        expect(usePrefferedLocale(fallbackLocale)).toBe('de');
    });
    it('detects correct locale for navigator', () => {
        navigatorLanguages = ['de', 'es'];
        expect(usePrefferedLocale(fallbackLocale)).toBe('de');
    });
    it('returns correct fallback locale', () => {
        expect(usePrefferedLocale(fallbackLocale)).toBe('pl');
    });
});
