export interface MainPluginSettings {
    topHeadingLineGap: string;
}

/**
 * `Partial<Type>` is a TypeScript utility that returns a type with all properties of Type set to optional.
 * It enables type checking while letting you only define the properties you want to provide defaults for.
 *
 * Source : https://docs.obsidian.md/Plugins/User+interface/Settings#Provide+default+values
 */
export const DEFAULT_SETTINGS: Partial<MainPluginSettings> = {
    topHeadingLineGap: "3",
};
