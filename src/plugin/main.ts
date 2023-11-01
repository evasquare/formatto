import { Plugin } from "obsidian";

import getEditorMenus from "../events/getEditorMenus";
import { MainPluginSettingTab } from "../settings/tab";

interface MainPluginSettings {
    topHeadingLineGap: string;
}

/**
 * `Partial<Type>` is a TypeScript utility that returns a type with all properties of Type set to optional.
 * It enables type checking while letting you only define the properties you want to provide defaults for.
 *
 * Source : https://docs.obsidian.md/Plugins/User+interface/Settings#Provide+default+values
 */
const DEFAULT_SETTINGS: Partial<MainPluginSettings> = {
    topHeadingLineGap: "3",
};

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
    events = getEditorMenus(this.app.workspace);
}
