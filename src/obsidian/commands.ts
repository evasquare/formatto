import { Command, Notice } from "obsidian";

import { getLocale, LOCALE_CATEGORY } from "@src/lang/lang";

import FormattoPlugin from "../main";

export class FormattoCommands {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    registerCommands() {
        this.getCommandsArr().forEach((item) => {
            this.plugin.addCommand(item);
        });
    }

    private getCommandsArr(): Command[] {
        return [
            {
                id: "formatto-logo",
                name: getLocale(LOCALE_CATEGORY.COMMANDS, "Format Document"),
                icon: "formatto-logo",
                editorCallback: (editor) => {
                    this.plugin.utils.formatDocument(editor);
                },
            },
            {
                id: "formatto-folder",
                name: getLocale(
                    LOCALE_CATEGORY.COMMANDS,
                    "Format Notes in Current Folder",
                ),
                icon: "formatto-logo",
                callback: async () => {
                    const activeFile =
                        this.plugin.app.workspace.getActiveFile();
                    if (!activeFile) {
                        new Notice(
                            getLocale(
                                LOCALE_CATEGORY.NOTICE_MESSAGES,
                                "No open document is found.",
                            ),
                        );
                        return;
                    }

                    await this.plugin.utils.formatFolderFiles(
                        activeFile.parent?.path ?? "",
                    );
                },
            },
        ];
    }
}
