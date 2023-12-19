import { Editor, EditorPosition, Notice } from "obsidian";

import {
    EditorPosition as WasmEditorPosition,
    formatDocument,
    FormattedDocument,
} from "../wasm/pkg/formatto_wasm";
import FormattoPlugin from "./main";

export class FormattoUtil {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    formatDocument(editor: Editor) {
        const cursorPosition = editor.getCursor();
        const originalDocument = editor.getValue();

        let formattedDocument: FormattedDocument;
        const editorPosition: EditorPosition = {
            line: cursorPosition.line,
            ch: cursorPosition.ch,
        };

        try {
            formattedDocument = formatDocument(
                originalDocument,
                this.plugin.settings,
                new WasmEditorPosition(editorPosition.line, editorPosition.ch)
            );
        } catch (error) {
            new Notice(error);
        }
        if (!formattedDocument.document) return;

        editor.setValue(formattedDocument.document);
        editor.setSelection(
            formattedDocument.editorPosition,
            formattedDocument.editorPosition
        );

        if (originalDocument === formattedDocument.document) {
            new Notice("Document is already formatted!");
        } else {
            new Notice("Document Formatted!");
        }
    }
}
