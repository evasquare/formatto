import { Plugin } from "obsidian";

export default class MainPlugin extends Plugin {
    async onload() {
        console.log("Hello World!");
    }

    onunload() {}
}
