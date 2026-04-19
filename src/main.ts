import { Plugin } from "obsidian";

import { FormattoCommands } from "@obsidian/commands.ts";
import { FormattoIcons } from "@obsidian/icons/icons.ts";
import { FormattoRibbonIcons } from "@obsidian/ribbonIcons.ts";
import { FormattoUtils } from "@obsidian/utils.ts";
import { FormattoEditorMenuEvent } from "@src/obsidian/events/editorMenuEvent.ts";
import { FormattoModifyEvent } from "@src/obsidian/events/modifyEvent.ts";
import { FormattoOptionTab } from "@src/obsidian/options/optionTab.ts";
import { DEFAULT_OPTIONS } from "@src/obsidian/options/optionTypes.ts";

import __wbg_init from "../wasm/pkg/formatto_wasm.js";
import formatto_wasm from "../wasm/pkg/formatto_wasm.js";

import type { FormattoPluginOptions } from "@src/obsidian/options/optionTypes.ts";

/** Entry Point. */
export default class FormattoPlugin extends Plugin {
    // @ts-expect-error A value injected by the Obsidian client.
    settings: FormattoPluginOptions;

    utils = new FormattoUtils(this);
    private icons = new FormattoIcons();
    private ribbonIcons = new FormattoRibbonIcons(this);
    private editorMenus = new FormattoEditorMenuEvent(this);
    private modify = new FormattoModifyEvent(this);
    private commands = new FormattoCommands(this);

    /** Load and Save Options */
    async loadOptions() {
        this.settings = Object.assign(
            {},
            DEFAULT_OPTIONS,
            await this.loadData(),
        );
    }
    async saveOptions() {
        await this.saveData(this.settings);
    }

    /** Runs whenever the user starts using the plugin in Obsidian. */
    async onload() {
        await this.loadOptions();

        // Initialize WebAssembly
        await (async () => {
            await __wbg_init(await formatto_wasm());
        })();

        this.addSettingTab(new FormattoOptionTab(this.app, this));

        this.icons.registerIcons();
        this.ribbonIcons.registerRibbonIcons();
        this.editorMenus.registerEvents();
        this.modify.registerEvents();
        this.commands.registerCommands();

        console.log(
            "Plugin Loaded: Formatto\n(Some error details are going to be displayed here.)",
        );
    }

    /** Runs when the plugin is disabled. */
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }
}
