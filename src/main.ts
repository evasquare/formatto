import { Plugin } from "obsidian";

import { FormattoEditorMenu } from "@events/editorMenu";
import { MainPluginSettingTab } from "@settings/settingTab";
import { DEFAULT_SETTINGS } from "@settings/settingTypes";

import __wbg_init from "../wasm/pkg/formatto_wasm";
import formatto_wasm from "../wasm/pkg/formatto_wasm_bg.wasm";
import { FormattoCommand } from "./commands/commands";
import { FormattoUtil } from "./utils";

import type { FormattoPluginSettings } from "@settings/settingTypes";

// * ENTRY POINT
export default class FormattoPlugin extends Plugin {
    settings: FormattoPluginSettings;

    // Load and Save Settings
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

    // Runs whenever the user starts using the plugin in Obsidian.
    async onload() {
        await this.loadSettings();

        // Initialize WebAssembly
        await (async () => {
            // @ts-ignore
            await __wbg_init(await formatto_wasm());
        })();

        this.addSettingTab(new MainPluginSettingTab(this.app, this));

        this.utils.registerIcons();
        this.eventsMenuCreator.registerEditorMenus();
        this.commandsCreator.registerCommands();

        console.log("Plugin Loaded: Formatto");
    }

    // Runs when the plugin is disabled.
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }

    utils = new FormattoUtil(this);
    private eventsMenuCreator = new FormattoEditorMenu(this);
    private commandsCreator = new FormattoCommand(this);
}
