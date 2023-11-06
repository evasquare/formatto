import { format_document } from "../../wasm/pkg/formatto_wasm";
import MainPlugin from "../main";

export class EditorMenuCreator {
    private plugin: MainPlugin;

    constructor(plugin: MainPlugin) {
        this.plugin = plugin;
    }

    getEventsArr() {
        return [
            this.plugin.app.workspace.on(
                "editor-menu",
                (menu, editor, view) => {
                    menu.addItem((item) =>
                        item
                            .setTitle("Format Document")
                            .setIcon("documents")
                            .onClick(() => {
                                const cursorPosition = editor.getCursor();

                                const formattedDocument = format_document(
                                    editor.getValue()
                                );

                                console.log(formattedDocument);
                                editor.setValue(formattedDocument);
                                editor.setSelection(
                                    cursorPosition,
                                    cursorPosition
                                );
                            })
                    );
                }
            ),
        ];
    }
}
