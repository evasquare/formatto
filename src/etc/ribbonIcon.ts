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
<<<<<<< HEAD
                new Notice("No open document is found.");
                return;
            }
            if (activeView.getMode() !== "source") {
                new Notice("You can only format in editing mode.");
            } else {
                this.plugin.utils.formatDocument(editor);
=======
                new Notice("Please make sure that the editor is open.");
                return;
>>>>>>> parent of 8c194da (simplify code)
            }
            if (activeView.getMode() !== "source") {
                new Notice("You can only format in editing mode.");
                return;
            }

            this.plugin.utils.formatDocument(editor);
        });
    };
}
