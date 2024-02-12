module.exports = {
    env: {
        es2021: true,
        node: true,
    },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        // Remove this line if you're not using Prettier.
        "prettier",
    ],
    overrides: [],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
    },
    plugins: ["@typescript-eslint"],
    rules: {
        quotes: ["warn", "double"],
        semi: ["error", "always"],

        // Unused vars
        "no-unused-vars": ["warn"],
        "@typescript-eslint/no-unused-vars": ["warn"],

        // Other configs
        "prefer-const": ["warn"],
        "@typescript-eslint/no-explicit-any": ["warn"],
    },
};
