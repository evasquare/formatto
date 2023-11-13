import FormattoPlugin from "../main";

export class FormattoEditorMenu {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
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
                                this.plugin.utils.getEventsArr(editor);
                            })
                    );
                }
            ),
        ];
    }
}
