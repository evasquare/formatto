import { Plugin } from "obsidian";

import { FormattoEditorMenu } from "@events/editorMenu";
import { MainPluginSettingTab } from "@settings/settingTab";
import { DEFAULT_SETTINGS } from "@settings/settingTypes";

import __wbg_init from "../wasm/pkg/formatto_wasm";
import formatto_wasm from "../wasm/pkg/formatto_wasm_bg.wasm";
import { FormattoCommand } from "./commands/commands";
import { CustomIcon } from "./etc/icons/icon";
import { RibbonIcon } from "./etc/ribbonIcon";
import { FormattoUtil } from "./utils";

import type { FormattoPluginSettings } from "@settings/settingTypes";

/** Entry Point. */
export default class FormattoPlugin extends Plugin {
    settings: FormattoPluginSettings;

    utils = new FormattoUtil(this);
    private iconCreator = new CustomIcon();
    private ribbonIcon = new RibbonIcon(this);
    private eventsMenuCreator = new FormattoEditorMenu(this);
    private commandCreator = new FormattoCommand(this);

    /** Load and Save Settings */
    async loadSettings() {
        this.settings = Object.assign(
            {},
            DEFAULT_SETTINGS,
            await this.loadData()
        );
    }
    async saveSettings() {
        await this.saveData(this.settings);
    }

    /** Runs whenever the user starts using the plugin in Obsidian. */
    async onload() {
        await this.loadSettings();

        // Initialize WebAssembly
        await (async () => {
            // @ts-expect-error: formatto_wasm should be called.
            await __wbg_init(await formatto_wasm());
        })();

        this.addSettingTab(new MainPluginSettingTab(this.app, this));

        this.iconCreator.registerIcons();
        this.ribbonIcon.registerRibbonIcons();
        this.eventsMenuCreator.registerEditorMenus();
        this.commandCreator.registerCommands();

        console.log(
            "Plugin Loaded: Formatto\n(Error details are going to be displayed here.)"
        );
    }

    /** Runs when the plugin is disabled. */
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }
}
