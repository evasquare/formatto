import { addIcon } from "obsidian";

import formattoLogo from "./img/formatto-logo.svg";

export class CustomIcons {
    private icons = [{ iconId: "formatto-logo", svg: formattoLogo }];

    registerIcons = () => {
        this.icons.forEach(({ iconId, svg }) => {
            addIcon(iconId, svg);
        });
    };
}
