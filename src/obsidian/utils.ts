import { Editor, Notice } from "obsidian";

import { getLocale, getWasmLocale, LOCALE_CATEGORY } from "@src/lang/lang";
import FormattoPlugin from "@src/main";

import { format_document } from "../../wasm/pkg/formatto_wasm";

export class FormattoUtils {
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
                this.plugin.settings,
                JSON.stringify(getWasmLocale())
            );
        } catch (error) {
            new Notice(error);
        }
        if (!formattedDocument) return;

        if (formattedDocument !== originalDocument) {
            editor.setValue(formattedDocument);
            editor.setSelection(cursorPosition, cursorPosition);
        }

        if (
            this.plugin.settings.otherOptions.notifyWhenUnchanged &&
            formattedDocument === originalDocument
        ) {
            new Notice(
                getLocale(
                    LOCALE_CATEGORY.NOTICE_MESSAGES,
                    "Document is already formatted!"
                )
            );
        } else {
            new Notice(
                getLocale(
                    LOCALE_CATEGORY.NOTICE_MESSAGES,
                    "Document Formatted!"
                )
            );
        }
    }
}
