import { Plugin } from "obsidian";

import { EditorMenuCreator } from "@events/editorMenuCreator";
import { MainPluginSettingTab } from "@settings/settingTab";
import { DEFAULT_SETTINGS } from "@settings/settingTypes";

import __wbg_init, { status } from "../wasm/pkg/formatto_wasm";
import formatto_wasm from "../wasm/pkg/formatto_wasm_bg.wasm";

import type { MainPluginSettings } from "@settings/settingTypes";

// ENTRY POINT
export default class MainPlugin extends Plugin {
    settings: MainPluginSettings;

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

        // Register Events
        this.events.forEach((item) => {
            this.registerEvent(item);
        });
    }

    // Runs when the plugin is disabled.
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }

    private eventsMenuCreator = new EditorMenuCreator(this);
    private events = this.eventsMenuCreator.getEventsArr();
}
