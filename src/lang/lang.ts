import de from "./locale/de.json";
import en from "./locale/en.json";
import hu from "./locale/hu.json";
import ko from "./locale/ko.json";

const detectedLanguage = window.localStorage.getItem("language");

export const LOCALE_CATEGORY = {
    COMMANDS: "commands",
    EDITOR_MENU: "editorMenu",
    RIBBON_ICONS: "ribbonIcons",
    NOTICE_MESSAGES: "noticeMessages",
    OPTION_WARNINGS: "optionWarnings",
    PLACEHOLDERS: "placeholders",
    OPTION_SECTIONS: "optionSections",
    HEADING_GAPS: "headingGaps",
    OTHER_GAPS: "otherGaps",
    FORMAT_OPTIONS: "formatOptions",
    OTHER_OPTIONS: "otherOptions",
} as const;

type ObjectValues<T> = T[keyof T];
type LocaleCategory = ObjectValues<typeof LOCALE_CATEGORY>;

const locales: { [key: string]: typeof en } = {
    en: en,
    de: de,
    hu: hu,
    ko: ko,
};

/** @example getLocale(LOCALE_CATEGORY.COMMANDS, "Format Document") */
export const getLocale = (category: LocaleCategory, key: string) => {
    const usingLocale = locales[detectedLanguage] ?? locales.en;
    const message = usingLocale[category][key];

    if (message === "") {
        const usingLocale = locales.en;
        return usingLocale[category][key];
    }

    return usingLocale[category][key];
};

/** Returns the "wasm" object in the locale file. */
export const getWasmLocale = () => {
    const usingLocale = locales[detectedLanguage] ?? locales.en;
    return usingLocale.wasm;
};
