import de from "./locale/de.json";
import en from "./locale/en.json";
import ko from "./locale/ko.json";

const detectedLanguage = window.localStorage.getItem("language");

export const LOCALE_CATEGORY = {
    COMMANDS: "commands",
    NOTICE_MESSAGES: "noticeMessages",
    SETTING_SECTIONS: "settingSections",
    HEADING_GAPS: "headingGaps",
    OTHER_GAPS: "otherGaps",
    FORMAT_OPTIONS: "formatOptions",
    OTHER_OPTIONS: "otherOptions",
} as const;

type ObjectValues<T> = T[keyof T];
type LocaleCategory = ObjectValues<typeof LOCALE_CATEGORY>;

/** @example getLocale(LOCALE_CATEGORY.COMMANDS, "Format Document") */
export const getLocale = (category: LocaleCategory, key: string) => {
    let usingLocale: typeof en = en;
    switch (detectedLanguage) {
        case "de":
            usingLocale = de;
            break;
        case "en":
            usingLocale = en;
            break;
        case "ko":
            usingLocale = ko;
            break;

        default:
            usingLocale = en;
            break;
    }

    return usingLocale[category][key];
};
