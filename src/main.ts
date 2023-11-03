import { Plugin } from "obsidian";

import getEditorMenus from "@events/getEditorMenus";
import { MainPluginSettingTab } from "@settings/settingTab";
import { DEFAULT_SETTINGS } from "@settings/settingTypes";

import type { MainPluginSettings } from "@settings/settingTypes";

//* ENTRY POINT
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
        this.addSettingTab(new MainPluginSettingTab(this.app, this));

        this.events.forEach((item) => {
            this.registerEvent(item);
        });

        console.log("Plugin Loaded: Formatto");
    }

    // Runs when the plugin is disabled.
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }

    // Dynamically load events.
    events = getEditorMenus(this);
}
