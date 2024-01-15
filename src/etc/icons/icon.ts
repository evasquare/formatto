import { addIcon } from "obsidian";

import formattoLogo from "./img/formatto-logo.svg";

export class CustomIcon {
    private icons = [{ iconId: "formatto-logo", svg: formattoLogo }];

    registerIcons = () => {
        this.icons.forEach(({ iconId, svg }) => {
            addIcon(iconId, svg);
        });
    };
}
