import { addIcon } from "obsidian";

import formattoLogo from "../icons/formatto-logo.svg";
import FormattoPlugin from "../main";

export class CustomIcon {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    private icons = [{ name: "formatto-logo", svg: formattoLogo }];

    registerIcons = () => {
        this.icons.forEach(({ name: id, svg }) => {
            addIcon(id, svg);
        });
    };
}
