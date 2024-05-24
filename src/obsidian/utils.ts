import { Editor, EditorPosition, Notice } from "obsidian";

import { getLocale, getWasmLocale, LOCALE_CATEGORY } from "@src/lang/lang";
import FormattoPlugin from "@src/main";

import { format_document } from "../../wasm/pkg/formatto_wasm";
import { FALLBACK_OPTIONS, FormattoPluginOptions } from "./options/optionTypes";

export class FormattoUtils {
    private plugin: FormattoPlugin;
    private cursorPosition: EditorPosition;
    private originalDocument: string;
    private formattedDocument: string;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    formatDocument(editor: Editor) {
        const copiedOptions = JSON.parse(JSON.stringify(this.plugin.settings));
        this.handleEmptyOptions(copiedOptions);

        this.cursorPosition = editor.getCursor();
        this.originalDocument = editor.getValue();

        try {
            this.formattedDocument = format_document(
                this.originalDocument,
                copiedOptions,
                JSON.stringify(getWasmLocale())
            );
            this.displayMessage();
        } catch (error) {
            new Notice(error);
        }

        if (!this.formattedDocument) return;
        if (this.formattedDocument !== this.originalDocument) {
            editor.setValue(this.formattedDocument);
            editor.setSelection(this.cursorPosition, this.cursorPosition);
        }

        this.clearVariables();
    }

    private displayMessage() {
        if (
            this.plugin.settings.otherOptions.notifyWhenUnchanged &&
            this.formattedDocument === this.originalDocument
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

    private handleEmptyOptions(copiedOptions: FormattoPluginOptions) {
        for (const optionSection of Object.keys(copiedOptions)) {
            for (const optionKey of Object.keys(copiedOptions[optionSection])) {
                if (copiedOptions[optionSection][optionKey] === "") {
                    copiedOptions[optionSection][optionKey] =
                        FALLBACK_OPTIONS[optionSection][optionKey];
                }
            }
        }
    }

    private clearVariables() {
        this.cursorPosition = undefined;
        this.originalDocument = undefined;
        this.formattedDocument = undefined;
    }
}
