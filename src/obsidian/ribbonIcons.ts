import { MarkdownView, Notice } from "obsidian";

import { getLocale, LOCALE_CATEGORY } from "@src/lang/lang";
import FormattoPlugin from "@src/main";

export class FormattoRibbonIcons {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    registerRibbonIcons = () => {
        this.plugin.addRibbonIcon(
            "formatto-logo",
            getLocale(LOCALE_CATEGORY.RIBBON_ICONS, "Format Document"),
            () => {
                const editor = this.plugin.app.workspace.activeEditor?.editor;
                const activeView =
                    this.plugin.app.workspace.getActiveViewOfType(MarkdownView);

                if (!editor) {
                    new Notice(
                        getLocale(
                            LOCALE_CATEGORY.NOTICE_MESSAGES,
                            "No open document is found."
                        )
                    );
                    return;
                }
                if (activeView.getMode() !== "source") {
                    new Notice(
                        getLocale(
                            LOCALE_CATEGORY.NOTICE_MESSAGES,
                            "You can only format in editing mode."
                        )
                    );
                    return;
                }

                this.plugin.utils.formatDocument(editor);
            }
        );
    };
}
