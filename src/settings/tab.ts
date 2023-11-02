import { App, debounce, Notice, PluginSettingTab, Setting } from "obsidian";

import MainPlugin from "../plugin/main";

export class MainPluginSettingTab extends PluginSettingTab {
    plugin: MainPlugin;

    constructor(app: App, plugin: MainPlugin) {
        super(app, plugin);
        this.plugin = plugin;
    }

    display(): void {
        const { containerEl } = this;
        containerEl.empty();

        // Settings Header
        containerEl.createEl("h1", { text: "Formatto" });
        containerEl
            .createEl("span", { text: "Obsidian Formatter by " })
            .createEl("a", {
                text: "Deca",
                href: "https://github.com/decaplanet",
            });
        containerEl.createEl("span", { text: ".\n" });
        containerEl.createEl("p", {
            text: "All values should be at least 0.",
            cls: "formatto__paragraph-margin formatto__important",
        });

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

        //* Property Gaps
        containerEl.createEl("h2", {
            text: "Property Gaps",
            cls: "formatto__margin-top",
        });

        new Setting(containerEl)
            .setName("After Properties")
            .setDesc("Decides the gap after YAML properties.")
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(this.plugin.settings.topHeadingLineGap)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.topHeadingLineGap = value;
                        await this.plugin.saveSettings();
                    })
            );

        //* Heading Gaps
        containerEl.createEl("h2", {
            text: "Heading Gaps",
            cls: "formatto-margin-top",
        });
        new Setting(containerEl)
            .setName("Top Level Headings")
            .setDesc("Decides gaps between highest level of headings.")
            .addText((text) =>
                text
                    .setPlaceholder("3")
                    .setValue(this.plugin.settings.topHeadingLineGap)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.topHeadingLineGap = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("First Sub Heading")
            .setDesc(
                "Decides the gap between a parent heading and the first of its children headings."
            )
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(this.plugin.settings.topHeadingLineGap)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.topHeadingLineGap = value;
                        await this.plugin.saveSettings();
                    })
            );

        new Setting(containerEl)
            .setName("Sub Headings")
            .setDesc(
                "Decides gaps between headings that are not the highest level."
            )
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(this.plugin.settings.topHeadingLineGap)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.topHeadingLineGap = value;
                        await this.plugin.saveSettings();
                    })
            );

        //* Other Gaps
        containerEl.createEl("h2", {
            text: "Other Gaps",
            cls: "formatto-margin-top",
        });
        new Setting(containerEl)
            .setName("Contents After Headings")
            .setDesc("Decides the gap after a heading.")
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(this.plugin.settings.topHeadingLineGap)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.topHeadingLineGap = value;
                        await this.plugin.saveSettings();
                    })
            );
    }
}
