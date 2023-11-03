import MainPlugin from "../main";

const getEditorMenus = (plugin: MainPlugin) => {
    // Array of events to register.
    const events = [
        plugin.app.workspace.on("editor-menu", (menu, editor, view) => {
            menu.addItem((item) =>
                item
                    .setTitle("Format Document")
                    .setIcon("documents")
                    .onClick(() => {
                        // TODO: Get and format document.
                    })
            );
        }),
    ];

    return events;
};

export default getEditorMenus;
