import { Plugin } from "obsidian";

import { FormattoCommands } from "@src/obsidian/commands";
import { FormattoEditorMenu } from "@src/obsidian/events/editorMenu";
import { FormattoIcons } from "@src/obsidian/icons/icons";
import { FormattoRibbonIcons } from "@src/obsidian/ribbonIcons";
import { FormattoSettingTab } from "@src/obsidian/settings/settingTab";
import { DEFAULT_SETTINGS } from "@src/obsidian/settings/settingTypes";
import { FormattoUtils } from "@src/obsidian/utils";

import __wbg_init from "../wasm/pkg/formatto_wasm";
import formatto_wasm from "../wasm/pkg/formatto_wasm_bg.wasm";

import type { FormattoPluginSettings } from "@src/obsidian/settings/settingTypes";

/** Entry Point. */
export default class FormattoPlugin extends Plugin {
    settings: FormattoPluginSettings;

    utils = new FormattoUtils(this);
    private icons = new FormattoIcons();
    private ribbonIcons = new FormattoRibbonIcons(this);
    private editorMenus = new FormattoEditorMenu(this);
    private commands = new FormattoCommands(this);

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

        this.addSettingTab(new FormattoSettingTab(this.app, this));

        this.icons.registerIcons();
        this.ribbonIcons.registerRibbonIcons();
        this.editorMenus.registerEditorMenus();
        this.commands.registerCommands();

        console.log(
            "Plugin Loaded: Formatto\n(Error details are going to be displayed here.)"
        );
    }

    /** Runs when the plugin is disabled. */
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }
}
