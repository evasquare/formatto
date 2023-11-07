import alias from "@rollup/plugin-alias";
import commonjs from "@rollup/plugin-commonjs";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";
import wasm from "@rollup/plugin-wasm";

export default {
    input: "src/main.ts",
    output: {
        format: "cjs",
        file: "main.js",
        exports: "default",
    },
    external: ["obsidian", "fs", "os", "path"],
    plugins: [
        wasm({
            fileName: "[name][extname]",
            maxFileSize: Number.MAX_SAFE_INTEGER,
        }),
        nodeResolve(),
        typescript({
            tsconfig: "./tsconfig.json",
        }),
        alias({
            entires: [
                { find: "@events", replacement: "./src/events" },
                { find: "@settings", replacement: "./src/settings" },
            ],
        }),
        commonjs({
            include: "node_modules/**",
        }),
    ],
};
