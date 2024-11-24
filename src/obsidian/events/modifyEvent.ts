import { TFile } from "obsidian";

import FormattoPlugin from "@src/main";

export class FormattoModifyEvent {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    private timer = null;
    private timerMs = 1500;

    registerEvents() {
        this.getEventsArr().forEach((item) => {
            this.plugin.registerEvent(item);
        });
    }

    private getEventsArr() {
        return [
            this.plugin.app.vault.on("modify", (file) => {
                this.timer = setTimeout(() => {
                    if (
                        this.plugin.settings.otherOptions.formatOnSave &&
                        file instanceof TFile &&
                        file.extension === "md"
                    ) {
                        this.plugin.app.vault.process(file, (data) => {
                            return this.plugin.utils.formatText(data);
                        });
                    }
                }, this.timerMs);
            }),
            this.plugin.app.workspace.on("editor-change", (editor, info) => {
                clearTimeout(this.timer);
            }),
        ];
    }
}
