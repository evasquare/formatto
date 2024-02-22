import { addIcon } from "obsidian";

import formattoLogo from "./img/formatto-logo.svg";

export class FormattoIcons {
    private icons = [{ iconId: "formatto-logo", svg: formattoLogo }];

    registerIcons = () => {
        this.icons.forEach(({ iconId, svg }) => {
            addIcon(iconId, svg);
        });
    };
}
