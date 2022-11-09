import { usePrefferedLocale } from "./data-access-i18n";
import { Locales } from "../i18n/i18n-types";

let localStorageItems: Record<string, string> = {};
let navigatorLanguages: Array<string> = [];
let fallbackLocale: Locales = "pl";

jest.spyOn(navigator, 'languages', 'get').mockImplementation(() => { return navigatorLanguages });
jest.spyOn(Storage.prototype, 'setItem');
Storage.prototype.getItem = jest.fn((key: string) => localStorageItems[key]);

describe('usePreferredLocale', () => {
  beforeEach(() => {
    localStorageItems = {};
    navigatorLanguages = [];
    fallbackLocale = "pl";
  })

  it('detects correct locale for localStorage', () => {
    localStorageItems = { data: "garbage", lang: "de" };
    navigatorLanguages = ["en"];
    expect(usePrefferedLocale(fallbackLocale)).toBe("de");
  });
  it('detects correct locale for navigator', () => {
    navigatorLanguages = ["de", "es"];
    expect(usePrefferedLocale(fallbackLocale)).toBe("de");
  });
  it('returns correct fallback locale', () => {
    expect(usePrefferedLocale(fallbackLocale)).toBe("pl");
  });
});
