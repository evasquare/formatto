import { Editor, EditorPosition, Notice, TFile } from "obsidian";

import { getLocale, getWasmLocale, LOCALE_CATEGORY } from "@src/lang/lang.ts";
import FormattoPlugin from "@src/main.ts";

import { format_document } from "../../wasm/pkg/formatto_wasm.js";
import {
    FALLBACK_OPTIONS,
    FormattoPluginOptions,
} from "./options/optionTypes.ts";

export class FormattoUtils {
    private plugin: FormattoPlugin;
    private cursorPosition: EditorPosition | undefined;
    private originalDocument: string | undefined;
    private formattedDocument: string | undefined;

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
                JSON.stringify(getWasmLocale()),
            );
            this.displayMessage();
        } catch (error) {
            const stringifiedError = String(error);
            new Notice(stringifiedError);
        }

        if (!this.formattedDocument) return;
        if (this.formattedDocument !== this.originalDocument) {
            editor.setValue(this.formattedDocument);
            editor.setSelection(this.cursorPosition, this.cursorPosition);
        }

        this.clearVariables();
    }

    formatText(data: string, showErrorNotice = true): string {
        const copiedOptions = JSON.parse(JSON.stringify(this.plugin.settings));
        this.handleEmptyOptions(copiedOptions);

        this.originalDocument = data;

        try {
            this.formattedDocument = format_document(
                this.originalDocument,
                copiedOptions,
                JSON.stringify(getWasmLocale()),
            );
            return this.formattedDocument;
        } catch (error) {
            if (showErrorNotice) {
                new Notice(String(error));
            }
            return data;
        } finally {
            this.clearVariables();
        }
    }

    async formatFolderFiles(folderPath: string) {
        const filesInFolder = this.getMarkdownFilesInFolder(folderPath);
        let changedCount = 0;
        let failedCount = 0;

        for (const file of filesInFolder) {
            try {
                const originalText =
                    await this.plugin.app.vault.cachedRead(file);
                const formattedText = this.formatText(originalText, false);

                if (formattedText !== originalText) {
                    await this.plugin.app.vault.modify(file, formattedText);
                    changedCount += 1;
                }
            } catch (error) {
                failedCount += 1;
                console.error(`Failed to format file: ${file.path}`, error);
            }
        }

        const message = getLocale(
            LOCALE_CATEGORY.NOTICE_MESSAGES,
            "Folder formatting completed. Checked: {TOTAL}, Changed: {CHANGED}, Failed: {FAILED}.",
        )
            .replace("{TOTAL}", String(filesInFolder.length))
            .replace("{CHANGED}", String(changedCount))
            .replace("{FAILED}", String(failedCount));

        new Notice(message);
    }

    private getMarkdownFilesInFolder(folderPath: string): TFile[] {
        const markdownFiles = this.plugin.app.vault.getMarkdownFiles();
        if (folderPath === "/") {
            return markdownFiles;
        }

        const folderPrefix = `${folderPath}/`;
        return markdownFiles.filter((file) =>
            file.path.startsWith(folderPrefix),
        );
    }

    private displayMessage() {
        if (
            this.plugin.settings.otherOptions.notifyWhenUnchanged &&
            this.formattedDocument === this.originalDocument
        ) {
            new Notice(
                getLocale(
                    LOCALE_CATEGORY.NOTICE_MESSAGES,
                    "Document is already formatted!",
                ),
            );
        } else {
            new Notice(
                getLocale(
                    LOCALE_CATEGORY.NOTICE_MESSAGES,
                    "Document Formatted!",
                ),
            );
        }
    }

    private handleEmptyOptions(copiedOptions: FormattoPluginOptions) {
        for (const sectionKey of Object.keys(
            copiedOptions,
        ) as (keyof FormattoPluginOptions)[]) {
            for (const optionKey of Object.keys(
                copiedOptions[sectionKey] as object,
            ) as (keyof (typeof copiedOptions)[typeof sectionKey])[]) {
                if (copiedOptions[sectionKey][optionKey] === "") {
                    copiedOptions[sectionKey][optionKey] =
                        FALLBACK_OPTIONS[sectionKey][optionKey];
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
