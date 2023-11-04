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
                                const value = editor.getValue();

                                // TODO: Format the document ("value" variable).
                            })
                    );
                }
            ),
        ];
    }
}
