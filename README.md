# serde-wasm-bindgen Typescript example

This example shows an integration of
[serde-wasm-bindgen](https://github.com/cloudflare/serde-wasm-bindgen) and
[tsify](https://github.com/madonoharu/tsify).

## Build

```sh
wasm-pack build
```

## Generated Typescript definitions

```ts
export type UserState = "Active" | "Inactive";

export interface User {
  id: string;
  name: string;
  state: UserState;
}

export function getUsers(): User[];
```
