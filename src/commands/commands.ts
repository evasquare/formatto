import { Command } from "obsidian";
import { getLocale, LOCALE_CATEGORY } from "src/lang/getLocale";

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
                name: getLocale(LOCALE_CATEGORY.COMMANDS, "Format Document"),
                icon: "formatto-logo",
                editorCallback: (editor) => {
                    this.plugin.utils.formatDocument(editor);
                },
            },
        ];
    }
}
