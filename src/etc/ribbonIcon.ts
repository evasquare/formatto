import { MarkdownView, Notice } from "obsidian";
import FormattoPlugin from "src/main";

export class RibbonIcon {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    registerRibbonIcons = () => {
        this.plugin.addRibbonIcon("formatto-logo", "Format Document", () => {
            const editor = this.plugin.app.workspace.activeEditor?.editor;
            const activeView =
                this.plugin.app.workspace.getActiveViewOfType(MarkdownView);

            if (!editor) {
                new Notice("No open document is found.");
            } else if (activeView.getMode() !== "source") {
                new Notice("You can only format in editing mode.");
            } else {
                this.plugin.utils.formatDocument(editor);
            }
        });
    };
}
