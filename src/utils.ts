import { Editor, Notice } from "obsidian";

import { format_document } from "../wasm/pkg/formatto_wasm";
import FormattoPlugin from "./main";

export class FormattoUtil {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    formatDocument(editor: Editor) {
        const cursorPosition = editor.getCursor();
        const originalDocument = editor.getValue();

        let formattedDocument: string;
        try {
            formattedDocument = format_document(
                originalDocument,
                this.plugin.settings
            );
        } catch (error) {
            new Notice(error);
        }

        if (formattedDocument === undefined) return;

        editor.setValue(formattedDocument);
        editor.setSelection(cursorPosition, cursorPosition);

        if (originalDocument == editor.getValue()) {
            new Notice("There is nothing to format.");
        } else {
            new Notice("Document Formatted.");
        }
    }
}
