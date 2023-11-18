import FormattoPlugin from "src/main";

export class RibbonIcon {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    registerIcons = () => {
        this.plugin.addRibbonIcon("formatto-logo", "Format Document", () => {
            this.plugin.utils.getEventsArr(
                this.plugin.app.workspace.activeEditor.editor
            );
        });
    };
}
