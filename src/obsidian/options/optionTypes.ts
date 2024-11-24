/*
  Type Declarations
*/

export interface HeadingGaps {
    /** "Decides the gap before a top-level heading." */
    beforeTopLevelHeadings: string;
    /** Decides the child heading gap right before a parent heading. */
    beforeFirstSubHeading: string;
    /** Decides gaps before headings that are not top-level. */
    beforeSubHeadings: string;
}

export interface OtherGaps {
    /** Decides the gap after a property section. */
    afterProperties: string;
    /** Decides gaps before content sections. (ex: Text before headings) */
    beforeContents: string;
    /** Decides gaps before \"contents that are after code blocks.\" */
    beforeContentsAfterCodeBlocks: string;
    /** Decides gaps before code blocks. */
    beforeCodeBlocks: string;
    /** Decides gaps before \"code blocks that are after headings.\" */
    beforeCodeBlocksAfterHeadings: string;
}

export interface FormatOptions {
    /** Inserts a newline at the end of a document. */
    insertNewline: boolean;
}

export interface OtherOptions {
    /** Displays a different message when no change is needed. */
    notifyWhenUnchanged: boolean;
    /** Displays additional information when parsing fails. */
    showMoreDetailedErrorMessages: boolean;
    /** Format document after each modification. */
    formatOnSave: boolean;
}

export interface FormattoPluginOptions {
    headingGaps: Partial<HeadingGaps>;
    otherGaps: Partial<OtherGaps>;
    formatOptions: Partial<FormatOptions>;
    otherOptions: Partial<OtherOptions>;
}

/*
  Fallback Option Values
*/

export const FALLBACK_HEADING_GAPS: Partial<HeadingGaps> = {
    beforeTopLevelHeadings: "3",
    beforeFirstSubHeading: "1",
    beforeSubHeadings: "2",
};

export const FALLBACK_OTHER_GAPS: Partial<OtherGaps> = {
    afterProperties: "2",
    beforeContents: "0",
    beforeContentsAfterCodeBlocks: "1",
    beforeCodeBlocks: "1",
    beforeCodeBlocksAfterHeadings: "0",
};

export const FALLBACK_FORMAT_OPTIONS: Partial<FormatOptions> = {
    insertNewline: true,
};

export const FALLBACK_OTHER_OPTIONS: Partial<OtherOptions> = {
    notifyWhenUnchanged: true,
    showMoreDetailedErrorMessages: false,
    formatOnSave: false,
};

export const FALLBACK_OPTIONS: FormattoPluginOptions = {
    headingGaps: FALLBACK_HEADING_GAPS,
    otherGaps: FALLBACK_OTHER_GAPS,
    formatOptions: FALLBACK_FORMAT_OPTIONS,
    otherOptions: FALLBACK_OTHER_OPTIONS,
};

/*
  Default Option Values
*/

export const EMPTY_HEADING_GAPS: Partial<HeadingGaps> = {
    beforeTopLevelHeadings: "",
    beforeFirstSubHeading: "",
    beforeSubHeadings: "",
};

export const EMPTY_OTHER_GAPS: Partial<OtherGaps> = {
    afterProperties: "",
    beforeContents: "",
    beforeContentsAfterCodeBlocks: "",
    beforeCodeBlocks: "",
    beforeCodeBlocksAfterHeadings: "",
};

export const DEFAULT_OPTIONS: FormattoPluginOptions = {
    headingGaps: EMPTY_HEADING_GAPS,
    otherGaps: EMPTY_OTHER_GAPS,
    formatOptions: FALLBACK_FORMAT_OPTIONS,
    otherOptions: FALLBACK_OTHER_OPTIONS,
};
