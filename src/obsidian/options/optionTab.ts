import { debounce, Notice, PluginSettingTab, Setting } from "obsidian";

import { getLocale, LOCALE_CATEGORY } from "@src/lang/lang";

import { FALLBACK_OPTIONS } from "./optionTypes";

import type { App } from "obsidian";
import type FormattoPlugin from "@src/main";

export class FormattoOptionTab extends PluginSettingTab {
    private plugin: FormattoPlugin;
    private noticeMessages = {
        invalidNumberMessage: getLocale(
            LOCALE_CATEGORY.NOTICE_MESSAGES,
            "Please enter a valid number.\nIt must be at least 0."
        ),
        notWholeNumberMessage: getLocale(
            LOCALE_CATEGORY.NOTICE_MESSAGES,
            "Please enter a valid number.\nIt must be a whole number."
        ),
    };

    constructor(app: App, plugin: FormattoPlugin) {
        super(app, plugin);
        this.plugin = plugin;
    }

    private checkDecimal(value: string): boolean {
        return value !== "0" && value !== "1" && parseFloat(value) % 1 !== 0;
    }

    private putDefaultIndicator(value: string): string {
        return `${value} ${getLocale(
            LOCALE_CATEGORY.PLACEHOLDERS,
            "(Default)"
        )}`;
    }

    display(): void {
        const { containerEl } = this;
        containerEl.empty();

        const debounceMsg = debounce(
            (value: string) => {
                if (value !== "") {
                    // Check if the value is a valid number
                    if (isNaN(parseInt(value)) || parseInt(value) < 0) {
                        new Notice(this.noticeMessages.invalidNumberMessage);
                        return;
                    }
                    // Check if the value is a whole number
                    if (this.checkDecimal(value)) {
                        new Notice(this.noticeMessages.notWholeNumberMessage);
                        return;
                    }
                }
            },
            1000,
            true
        );

        containerEl.createDiv({}, (div) => {
            div.innerHTML = `<div style="color: var(--text-accent)">
                ${getLocale(
                    LOCALE_CATEGORY.OPTION_WARNINGS,
                    "Gap value must be a whole number and it needs to be at least 0."
                )}
            </div>`;
            div.className = "setting-item setting-item-description";
        });

        // Heading Gaps
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.OPTION_SECTIONS, "Heading gaps"),
        });
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Before top-level headings"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Decides the gap before a top-level heading."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.headingGaps.beforeTopLevelHeadings
                        )
                    )
                    .setValue(
                        this.plugin.settings.headingGaps.beforeTopLevelHeadings
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.headingGaps.beforeTopLevelHeadings =
                            value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Before the first sub-level heading"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Decides the child heading gap right after a parent heading."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.headingGaps.beforeFirstSubHeading
                        )
                    )
                    .setValue(
                        this.plugin.settings.headingGaps.beforeFirstSubHeading
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.headingGaps.beforeFirstSubHeading =
                            value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Before sub-level headings"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Decides gaps before headings that are not top-level."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.headingGaps.beforeSubHeadings
                        )
                    )
                    .setValue(
                        this.plugin.settings.headingGaps.beforeSubHeadings
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.headingGaps.beforeSubHeadings =
                            value;
                        await this.plugin.saveOptions();
                    })
            );

        // Other Gaps
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.OPTION_SECTIONS, "Other gaps"),
        });
        new Setting(containerEl)
            .setName(getLocale(LOCALE_CATEGORY.OTHER_GAPS, "After properties"))
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Decides the gap after a property section."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.otherGaps.afterProperties
                        )
                    )
                    .setValue(this.plugin.settings.otherGaps.afterProperties)
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.afterProperties = value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(getLocale(LOCALE_CATEGORY.OTHER_GAPS, "Before contents"))
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Decides gaps before content sections. (ex: Text before headings)"
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.otherGaps.beforeContents
                        )
                    )
                    .setValue(this.plugin.settings.otherGaps.beforeContents)
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeContents = value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Before contents after code blocks"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    'Decides gaps before "contents that are after code blocks."' // eslint-disable-line
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.otherGaps
                                .beforeContentsAfterCodeBlocks
                        )
                    )
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeContentsAfterCodeBlocks
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeContentsAfterCodeBlocks =
                            value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(LOCALE_CATEGORY.OTHER_GAPS, "Before code blocks")
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Decides gaps before code blocks."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.otherGaps.beforeCodeBlocks
                        )
                    )
                    .setValue(this.plugin.settings.otherGaps.beforeCodeBlocks)
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeCodeBlocks = value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Before code blocks after headings"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    'Decides gaps before "code blocks that are after headings."'
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder(
                        this.putDefaultIndicator(
                            FALLBACK_OPTIONS.otherGaps
                                .beforeCodeBlocksAfterHeadings
                        )
                    )
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeCodeBlocksAfterHeadings
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeCodeBlocksAfterHeadings =
                            value;
                        await this.plugin.saveOptions();
                    })
            );

        // Format Options
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.OPTION_SECTIONS, "Format options"),
        });
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.FORMAT_OPTIONS,
                    "Newline at the end of a document"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.FORMAT_OPTIONS,
                    "Inserts a newline at the end of a document."
                )
            )
            .addToggle((text) =>
                text
                    .setValue(this.plugin.settings.formatOptions.insertNewline)
                    .onChange(async (value) => {
                        this.plugin.settings.formatOptions.insertNewline =
                            value;
                        await this.plugin.saveOptions();
                    })
            );

        // Other Options
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.OPTION_SECTIONS, "Other options"),
        });
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.OTHER_OPTIONS,
                    "Notify when no change is needed"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_OPTIONS,
                    "Displays a different message when no change is needed."
                )
            )
            .addToggle((text) =>
                text
                    .setValue(
                        this.plugin.settings.otherOptions.notifyWhenUnchanged
                    )
                    .onChange(async (value) => {
                        this.plugin.settings.otherOptions.notifyWhenUnchanged =
                            value;
                        await this.plugin.saveOptions();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.OTHER_OPTIONS,
                    "More detailed error message"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_OPTIONS,
                    "Displays additional information when parsing fails."
                )
            )
            .addToggle((text) =>
                text
                    .setValue(
                        this.plugin.settings.otherOptions
                            .showMoreDetailedErrorMessages
                    )
                    .onChange(async (value) => {
                        this.plugin.settings.otherOptions.showMoreDetailedErrorMessages =
                            value;
                        await this.plugin.saveOptions();
                    })
            );
    }
}
