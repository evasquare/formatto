import { Command } from "obsidian";

import MainPlugin from "../main";

export class FormattoCommands {
    private plugin: MainPlugin;

    constructor(plugin: MainPlugin) {
        this.plugin = plugin;
    }

    getCommandsArr(): Command[] {
        return [
            {
                id: "formatto-format__format",
                name: "Format Document",
                editorCallback: (editor) => {
                    this.plugin.utils.getEventsArr(editor);
                },
            },
        ];
    }
}
