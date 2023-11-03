import alias from "@rollup/plugin-alias";
import commonjs from "@rollup/plugin-commonjs";
import typescript from "@rollup/plugin-typescript";

export default {
    input: "src/main.ts",
    output: {
        format: "cjs",
        file: "main.js",
        exports: "default",
    },
    external: ["obsidian", "fs", "os", "path"],
    plugins: [
        typescript({
            tsconfig: "./tsconfig.json",
        }),
        commonjs({
            include: "node_modules/**",
        }),
        alias({
            entires: [
                { find: "@settings", replacement: "./src/settings" },
                { find: "@events", replacement: "./src/events" },
            ],
        }),
    ],
};
