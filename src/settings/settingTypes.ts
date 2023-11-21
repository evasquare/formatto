export interface HeadingGaps {
    /** Decides gaps before highest level of headings. */
    beforeTopLevelHeadings: string;
    /** Decides the child heading gap right before a parent heading. */
    beforeFirstSubHeading: string;
    /** Decides gaps before headings that are not in the highest level. */
    beforeSubHeadings: string;
}

export interface OtherGaps {
    /** Decides the gap after a YAML properties. */
    afterProperties: string;
    /** Decides gaps before contents (ex: Text section right before headings). */
    beforeContents: string;
    /** Decides gaps before contents that are right after code blocks. */
    beforeContentsAfterCodeBlocks: string;
    /** Decides gaps before code blocks. */
    beforeCodeBlocks: string;
    /** Decides gaps before code blocks that are right after headings. */
    beforeCodeBlocksAfterHeadings: string;
}

export interface FormattoPluginSettings {
    headingGaps: Partial<HeadingGaps>;
    otherGaps: Partial<OtherGaps>;
}

// `Partial<Type>` is a TypeScript utility that returns a type with all properties of Type set to optional.
// It enables type checking while letting you only define the properties you want to provide defaults for.
// Source : https://docs.obsidian.md/Plugins/User+interface/Settings#Provide+default+values

export const DEFAULT_HEADING_GAPS: Partial<HeadingGaps> = {
    beforeTopLevelHeadings: "3",
    beforeFirstSubHeading: "1",
    beforeSubHeadings: "2",
};

export const DEFAULT_OTHER_GAPS: Partial<OtherGaps> = {
    afterProperties: "2",
    beforeContents: "0",
    beforeContentsAfterCodeBlocks: "1",
    beforeCodeBlocks: "1",
    beforeCodeBlocksAfterHeadings: "0",
};

export const DEFAULT_SETTINGS: FormattoPluginSettings = {
    headingGaps: DEFAULT_HEADING_GAPS,
    otherGaps: DEFAULT_OTHER_GAPS,
};
