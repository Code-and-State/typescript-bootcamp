# Vite + Svelte + Azle

## Getting started 

Make sure that [Node.js](https://nodejs.org/en/) `>= 16.x` and [`dfx`](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) `>= 0.14.x` are installed on your system.

Run the following commands in a new, empty project directory:

```sh
dfx start --clean --background # Run dfx in the background
npm run setup # Install packages, deploy canisters, and generate type bindings
npm start # Start the development server
```

## Technology Stack

- [Vite](https://vitejs.dev/): high-performance tooling for front-end web development
- [Svelte](https://svelte.dev/): a radical and innovative JavaScript framework for building user interfaces
- [TypeScript](https://www.typescriptlang.org/): JavaScript extended with syntax for types
- [Sass](https://sass-lang.com/): an extended syntax for CSS stylesheets
- [Prettier](https://prettier.io/): code formatting for a wide range of supported languages
- [Azle](https://github.com/demergent-labs/azle): a TypeScript CDK for the Internet Computer

## Documentation

- [Vite developer docs](https://vitejs.dev/guide/)
- [Learn Svelte](https://learn.svelte.dev/tutorial/welcome-to-svelte)
- [Internet Computer docs](https://internetcomputer.org/docs/current/developer-docs/ic-overview)
- [Azle Book](https://demergent-labs.github.io/azle/)
- [`dfx.json` reference schema](https://internetcomputer.org/docs/current/references/dfx-json-reference/)

## Tips and Tricks

- Customize your project's code style by editing the `.prettierrc` file and then running `npm run format`.
- Reduce the latency of update calls by passing the `--emulator` flag to `dfx start`.
- Split your frontend and backend console output by running `npm run frontend` and `npm run backend` in separate terminals.
