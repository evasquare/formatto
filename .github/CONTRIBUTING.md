## Which branch do I clone/fork?
You can start by cloning or forking a branch for the version you're adding features in.

Please DO NOT make a pull request to the [main](https://github.com/polyecho/formatto) branch.


## Build Environment
To build the code, you need Node.js, NPM, Rust, and Cargo in your development environment. Please refer to following documents.

- [Node.js](https://nodejs.org/en)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [The Rust Programming Language - Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)


## NPM Scripts
You can run these scripts with the following command.

```bash
npm run <SCRIPT_NAME>
```

| Script     | Description                                              |
| ---------- | -------------------------------------------------------- |
| dev:ts     | Build every time TypeScript code is changed.             |
| dev:wasm   | Build WebAssembly files every time Rust code is changed. |
| build      | Build the plugin. (build:ts + build:wasm)                |
| build:ts   | Build only TypeScript code.                              |
| build:wasm | Build only Rust code.                                    |
| test:ts    | Run Vitest and watch changes in TypeScript code.         |
| test:rust  | Run "cargo test" and watch changes in Rust code.         |
| lint:ts    | Run ESLint.                                              |