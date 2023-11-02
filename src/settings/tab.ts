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
        containerEl.createEl("span", { text: "." });

        // Format Preferences
        containerEl.createEl("h2", {
            text: "Format Preferences",
            cls: "formatto-margin-top",
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
        // - Top Heading Line Gap
        new Setting(containerEl)
            .setName("Top Heading Line Gap")
            .setDesc("A number value that is at least 0.")
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
    }
}
