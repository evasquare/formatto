export interface PropertyGaps {
    afterProperties: string;
}

export interface HeadingGaps {
    topLevelHeadings: string;
    firstSubHeading: string;
    subHeadings: string;
}

export interface OtherGaps {
    contentsAfterHeadings: string;
}

export interface MainPluginSettings {
    propertyGaps: Partial<PropertyGaps>;
    headingGaps: Partial<HeadingGaps>;
    otherGaps: Partial<OtherGaps>;
}

// `Partial<Type>` is a TypeScript utility that returns a type with all properties of Type set to optional.
// It enables type checking while letting you only define the properties you want to provide defaults for.
// Source : https://docs.obsidian.md/Plugins/User+interface/Settings#Provide+default+values

export const DEFAULT_PROPERTY_GAPS: Partial<PropertyGaps> = {
    afterProperties: "2",
};

export const DEFAULT_HEADING_GAPS: Partial<HeadingGaps> = {
    topLevelHeadings: "3",
    firstSubHeading: "1",
    subHeadings: "2",
};

export const DEFAULT_OTHER_GAPS: Partial<OtherGaps> = {
    contentsAfterHeadings: "0",
};

export const DEFAULT_SETTINGS: MainPluginSettings = {
    propertyGaps: DEFAULT_PROPERTY_GAPS,
    headingGaps: DEFAULT_HEADING_GAPS,
    otherGaps: DEFAULT_OTHER_GAPS,
};
