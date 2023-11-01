import { Workspace } from "obsidian";

const getEditorMenus = (workspace: Workspace) => {
    // Array of events to register.
    const events = [
        workspace.on("editor-menu", (menu, editor, view) => {
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
