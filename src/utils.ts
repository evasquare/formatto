import { addIcon, Editor, Notice } from "obsidian";

import formattoLogo from "../icons/formatto-logo.svg";
import { format_document } from "../wasm/pkg/formatto_wasm";
import FormattoPlugin from "./main";

export class FormattoUtil {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    getEventsArr(editor: Editor) {
        const cursorPosition = editor.getCursor();

        let formattedDocument: string;
        try {
            formattedDocument = format_document(
                editor.getValue(),
                this.plugin.settings
            );
        } catch (error) {
            new Notice(error);
        }

        if (formattedDocument === undefined) return;

        editor.setValue(formattedDocument);
        editor.setSelection(cursorPosition, cursorPosition);

        new Notice("Document Formatted!");
    }
}
