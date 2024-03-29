import { Editor, EditorPosition, Notice } from "obsidian";

import { getLocale, getWasmLocale, LOCALE_CATEGORY } from "@src/lang/lang";
import FormattoPlugin from "@src/main";

import { format_document } from "../../wasm/pkg/formatto_wasm";
import {
    FALLBACK_SETTINGS,
    FormattoPluginSettings,
} from "./settings/settingTypes";

export class FormattoUtils {
    private plugin: FormattoPlugin;
    private cursorPosition: EditorPosition;
    private originalDocument: string;
    private formattedDocument: string;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    formatDocument(editor: Editor) {
        const copiedSettings = JSON.parse(JSON.stringify(this.plugin.settings));
        this.handleEmptyOptions(copiedSettings);

        this.cursorPosition = editor.getCursor();
        this.originalDocument = editor.getValue();

        try {
            this.formattedDocument = format_document(
                this.originalDocument,
                copiedSettings,
                JSON.stringify(getWasmLocale())
            );
        } catch (error) {
            new Notice(error);
        }

        this.displayMessage();

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

    private handleEmptyOptions(copiedSettings: FormattoPluginSettings) {
        for (const optionSection of Object.keys(copiedSettings)) {
            for (const optionKey of Object.keys(
                copiedSettings[optionSection]
            )) {
                if (copiedSettings[optionSection][optionKey] === "") {
                    copiedSettings[optionSection][optionKey] =
                        FALLBACK_SETTINGS[optionSection][optionKey];
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
