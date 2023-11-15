import { Command } from "obsidian";

import MainPlugin from "../main";

export class FormattoCommand {
    private plugin: MainPlugin;

    constructor(plugin: MainPlugin) {
        this.plugin = plugin;
    }

    getCommandsArr(): Command[] {
        return [
            {
                id: "format",
                name: "Format Document",
                editorCallback: (editor) => {
                    this.plugin.utils.getEventsArr(editor);
                },
            },
        ];
    }
}
