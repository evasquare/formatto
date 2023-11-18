import { addIcon } from "obsidian";

import formattoLogo from "../icons/formatto-logo.svg";

export class CustomIcon {
    private icons = [{ name: "formatto-logo", svg: formattoLogo }];

    registerIcons = () => {
        this.icons.forEach(({ name: id, svg }) => {
            addIcon(id, svg);
        });
    };
}
