export interface HeadingGaps {
    topLevelHeadings: string;
    firstSubHeading: string;
    subHeadings: string;
}

export interface OtherGaps {
    afterProperties: string;
    contentsAfterHeadings: string;
    beforeCodeBlocks: string;
    codeBlocksAfterHeadings: string;
}

export interface MainPluginSettings {
    headingGaps: Partial<HeadingGaps>;
    otherGaps: Partial<OtherGaps>;
}

// `Partial<Type>` is a TypeScript utility that returns a type with all properties of Type set to optional.
// It enables type checking while letting you only define the properties you want to provide defaults for.
// Source : https://docs.obsidian.md/Plugins/User+interface/Settings#Provide+default+values

export const DEFAULT_HEADING_GAPS: Partial<HeadingGaps> = {
    topLevelHeadings: "3",
    firstSubHeading: "1",
    subHeadings: "2",
};

export const DEFAULT_OTHER_GAPS: Partial<OtherGaps> = {
    afterProperties: "2",
    contentsAfterHeadings: "0",
    beforeCodeBlocks: "1",
    codeBlocksAfterHeadings: "0",
};

export const DEFAULT_SETTINGS: MainPluginSettings = {
    headingGaps: DEFAULT_HEADING_GAPS,
    otherGaps: DEFAULT_OTHER_GAPS,
};
