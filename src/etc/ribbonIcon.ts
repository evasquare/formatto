import { debounce, MarkdownView, Notice } from "obsidian";
import FormattoPlugin from "src/main";

export class RibbonIcon {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    debounceMsg = debounce((text: string) => {
        new Notice(text);
    }, 1000);

    registerRibbonIcons = () => {
        this.plugin.addRibbonIcon("formatto-logo", "Format Document", () => {
            const editor = this.plugin.app.workspace.activeEditor?.editor;
            const activeView =
                this.plugin.app.workspace.getActiveViewOfType(MarkdownView);

            if (!editor) {
                this.debounceMsg("Please make sure that the editor is open.");
                return;
            }
            if (activeView.getMode() !== "source") {
                this.debounceMsg("You can only format in editing mode.");
                return;
            }

            this.plugin.utils.getEventsArr(editor);
        });
    };
}
