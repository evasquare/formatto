import { debounce, Notice, PluginSettingTab, Setting } from "obsidian";

import type { App } from "obsidian";
import type FormattoPlugin from "../main";

export class MainPluginSettingTab extends PluginSettingTab {
    private plugin: FormattoPlugin;
    private invalidNumberMessage =
        "Please enter a valid number.\nIt should be at least 0.";

    constructor(app: App, plugin: FormattoPlugin) {
        super(app, plugin);
        this.plugin = plugin;
    }

    display(): void {
        const { containerEl } = this;

        containerEl.empty();
        const debounceMsg = debounce(
            (text: string, value: string) => {
                if (
                    value !== "" &&
                    (isNaN(parseInt(value)) || parseInt(value) < 0)
                ) {
                    new Notice(text);
                }
            },
            1000,
            true
        );

        // Heading Gaps
        containerEl.createEl("h2", {
            text: "Heading gaps",
        });
        new Setting(containerEl)
            .setName("Before top level headings")
            .setDesc("Decides gaps before top level of headings.")
            .addText((text) =>
                text
                    .setPlaceholder("3")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeTopLevelHeadings
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.headingGaps.beforeTopLevelHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before first sub heading")
            .setDesc(
                "Decides the child heading gap right before a parent heading."
            )
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeFirstSubHeading
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.headingGaps.beforeFirstSubHeading =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before sub headings")
            .setDesc(
                "Decides gaps before headings that are not in the top level."
            )
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeSubHeadings
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.headingGaps.beforeSubHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );

        // Other Gaps
        containerEl.createEl("h2", {
            text: "Other gaps",
        });
        new Setting(containerEl)
            .setName("After properties")
            .setDesc("Decides the gap right after the property section.")
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(this.plugin.settings.otherGaps.afterProperties)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.otherGaps.afterProperties = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before contents")
            .setDesc(
                "Decides gaps before contents (ex: Text section before headings)."
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(this.plugin.settings.otherGaps.beforeContents)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.otherGaps.beforeContents = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before contents after code blocks")
            .setDesc("Decides gaps before contents that are after code blocks.")
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeContentsAfterCodeBlocks
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.otherGaps.beforeContentsAfterCodeBlocks =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before code blocks")
            .setDesc("Decides gaps before code blocks.")
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(this.plugin.settings.otherGaps.beforeCodeBlocks)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.otherGaps.beforeCodeBlocks = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before code blocks after headings")
            .setDesc(
                "Decides gaps before code blocks that are right after headings."
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeCodeBlocksAfterHeadings
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(this.invalidNumberMessage, value);
                        }

                        this.plugin.settings.otherGaps.beforeCodeBlocksAfterHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );

        // Format Settings
        containerEl.createEl("h2", {
            text: "Format settings",
        });
        new Setting(containerEl)
            .setName("Newline at the end of a document")
            .setDesc("Inserts a newline at the end of a document.")
            .addToggle((text) =>
                text
                    .setValue(this.plugin.settings.formatSettings.insertNewline)
                    .onChange(async (value) => {
                        this.plugin.settings.formatSettings.insertNewline =
                            value;
                        await this.plugin.saveSettings();
                    })
            );

        // Other Settings
        containerEl.createEl("h2", {
            text: "Other settings",
        });
        new Setting(containerEl)
            .setName("Notify when no change is needed")
            .setDesc("Displays a different message when no change was made.")
            .addToggle((text) =>
                text
                    .setValue(
                        this.plugin.settings.otherSettings.notifyWhenUnchanged
                    )
                    .onChange(async (value) => {
                        this.plugin.settings.otherSettings.notifyWhenUnchanged =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
    }
}
