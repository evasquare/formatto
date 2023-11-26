module.exports = {
    env: {
        es2021: true,
        node: true,
    },
    extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
    overrides: [],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
    },
    plugins: ["@typescript-eslint"],
    rules: {
        indent: ["warn", 4],
        quotes: ["warn", "double"],
        semi: ["error", "always"],

        // Unused vars
        "no-unused-vars": ["warn"],
        "@typescript-eslint/no-unused-vars": ["warn"],

        // Etc
        "prefer-const": ["warn"],
    },
};
