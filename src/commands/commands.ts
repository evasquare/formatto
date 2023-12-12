import { Command } from "obsidian";

import MainPlugin from "../main";

export class FormattoCommand {
    private plugin: MainPlugin;

    constructor(plugin: MainPlugin) {
        this.plugin = plugin;
    }

    registerCommands() {
        this.getCommandsArr().forEach((item) => {
            this.plugin.addCommand(item);
        });
    }

    getCommandsArr(): Command[] {
        return [
            {
                id: "formatto-logo",
                name: "Format Document",
                icon: "formatto-logo",
                editorCallback: (editor) => {
                    this.plugin.utils.formatDocument(editor);
                },
            },
        ];
    }
}
