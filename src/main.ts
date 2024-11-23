import { Plugin, TFile } from "obsidian";

import { FormattoCommands } from "@obsidian/commands";
import { FormattoEditorMenu } from "@obsidian/events/editorMenu";
import { FormattoIcons } from "@obsidian/icons/icons";
import { FormattoRibbonIcons } from "@obsidian/ribbonIcons";
import { FormattoUtils } from "@obsidian/utils";
import { FormattoOptionTab } from "@src/obsidian/options/optionTab";
import { DEFAULT_OPTIONS } from "@src/obsidian/options/optionTypes";

import __wbg_init from "../wasm/pkg/formatto_wasm";
import formatto_wasm from "../wasm/pkg/formatto_wasm_bg.wasm";

import type { FormattoPluginOptions } from "@src/obsidian/options/optionTypes";

/** Entry Point. */
export default class FormattoPlugin extends Plugin {
    settings: FormattoPluginOptions;

    utils = new FormattoUtils(this);
    private icons = new FormattoIcons();
    private ribbonIcons = new FormattoRibbonIcons(this);
    private editorMenus = new FormattoEditorMenu(this);
    private commands = new FormattoCommands(this);

    /** Load and Save Options */
    async loadOptions() {
        this.settings = Object.assign(
            {},
            DEFAULT_OPTIONS,
            await this.loadData()
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
            // @ts-expect-error: formatto_wasm should be called.
            await __wbg_init(await formatto_wasm());
        })();

        this.addSettingTab(new FormattoOptionTab(this.app, this));

        this.icons.registerIcons();
        this.ribbonIcons.registerRibbonIcons();
        this.editorMenus.registerEditorMenus();
        this.commands.registerCommands();

        this.registerEvent(
            this.app.vault.on('modify', (file) => {
                if (this.settings.otherOptions.formatOnSave && file instanceof TFile && file.extension === 'md') {
                    this.app.vault.process(file, (data) => {
                        return this.utils.formatText(data)
                    });
                }
            }),
        );

        console.log(
            "Plugin Loaded: Formatto\n(Some error details are going to be displayed here.)"
        );
    }

    /** Runs when the plugin is disabled. */
    onunload() {
        console.log("Plugin Unloaded: Formatto");
    }
}
