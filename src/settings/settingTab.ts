import { debounce, Notice, PluginSettingTab, Setting } from "obsidian";
import { getLocale, LOCALE_CATEGORY } from "src/lang/getLocale";

import type { App } from "obsidian";
import type FormattoPlugin from "../main";
export class MainPluginSettingTab extends PluginSettingTab {
    private plugin: FormattoPlugin;
    private noticeMessages = {
        invalidNumberMessage: getLocale(
            LOCALE_CATEGORY.NOTICE_MESSAGES,
            "Please enter a valid number.\nIt should be at least 0."
        ),
        notWholeNumberMessage: getLocale(
            LOCALE_CATEGORY.NOTICE_MESSAGES,
            "Please enter a valid number.\nIt should be a whole number."
        ),
    };

    constructor(app: App, plugin: FormattoPlugin) {
        super(app, plugin);
        this.plugin = plugin;
    }

    private checkDecimal(value: string): boolean {
        return value !== "0" && value !== "1" && parseFloat(value) % 1 !== 0;
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

        // Heading Gaps
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.SETTING_SECTIONS, "Heading gaps"),
        });
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Before top level headings"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Decides gaps before top level of headings."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("3")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeTopLevelHeadings
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.headingGaps.beforeTopLevelHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Before first sub heading"
                )
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Decides the child heading gap right before a parent heading."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeFirstSubHeading
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.headingGaps.beforeFirstSubHeading =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(LOCALE_CATEGORY.HEADING_GAPS, "Before sub headings")
            )
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.HEADING_GAPS,
                    "Decides gaps before headings that are not in the top level."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeSubHeadings
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.headingGaps.beforeSubHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );

        // Other Gaps
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.SETTING_SECTIONS, "Other gaps"),
        });
        new Setting(containerEl)
            .setName(getLocale(LOCALE_CATEGORY.OTHER_GAPS, "After properties"))
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Decides the gap after the property section."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(this.plugin.settings.otherGaps.afterProperties)
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.afterProperties = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName(getLocale(LOCALE_CATEGORY.OTHER_GAPS, "Before contents"))
            .setDesc(
                getLocale(
                    LOCALE_CATEGORY.OTHER_GAPS,
                    "Decides gaps before contents. (ex: Text section before headings)"
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(this.plugin.settings.otherGaps.beforeContents)
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeContents = value;
                        await this.plugin.saveSettings();
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
                    "Decides gaps before 'contents that are after code blocks'."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeContentsAfterCodeBlocks
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeContentsAfterCodeBlocks =
                            value;
                        await this.plugin.saveSettings();
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
                    .setPlaceholder("1")
                    .setValue(this.plugin.settings.otherGaps.beforeCodeBlocks)
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeCodeBlocks = value;
                        await this.plugin.saveSettings();
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
                    "Decides gaps before 'code blocks that are after headings'."
                )
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeCodeBlocksAfterHeadings
                    )
                    .onChange(async (value) => {
                        debounceMsg(value);

                        this.plugin.settings.otherGaps.beforeCodeBlocksAfterHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );

        // Format Settings
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.SETTING_SECTIONS, "Format options"),
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
                        await this.plugin.saveSettings();
                    })
            );

        // Other Settings
        containerEl.createEl("h2", {
            text: getLocale(LOCALE_CATEGORY.SETTING_SECTIONS, "Other options"),
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
                    "Displays a different message when no change was made."
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
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName(
                getLocale(
                    LOCALE_CATEGORY.OTHER_OPTIONS,
                    "Show more detailed error messages"
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
                        await this.plugin.saveSettings();
                    })
            );
    }
}
