# react-run

[![CI](https://github.com/UltiRequiem/react-run/actions/workflows/ci.yaml/badge.svg)](https://github.com/UltiRequiem/react-run/actions/workflows/ci.yaml)

Run React(JS/TS) code snippets/components from your command-line without config.

## Usage

The entry point is a component called `App`, example:

```tsx
// example.tsx

function ProductList({ products }: { products: string[] }) {
  return products.map((p: string) => <li>{p}</li>);
}

export default function App() {
  const style: { [key: string]: string } = {
    background: "lightgray",
    padding: "1em",
  };

  return (
    <div style={style}>
      <h1>Our Products</h1>
      <ProductList products={["Apple", "Banana"]} />
    </div>
  );
}
```

<details>

  <summary>JavaScript</summary>

```javascript
// example.js
export default function App() {
  const [count, setCount] = React.useState(0);

  return (
    <div style={{ background: "purple", color: "white", padding: "10px" }}>
      <h1>Hello World</h1>
      <p>{count}</p>
      <button onClick={() => setCount(count + Math.random())}>+ random</button>
    </div>
  );
}
```

</details>

You could run this with:

```sh
react-run example.tsx # or example.js
```

Or If you don't want to have the file in your local machine:

```sh
react-run https://raw.githubusercontent.com/UltiRequiem/react-run/main/examples/typescript.tsx
```

> Yep, just like [Deno](https://deno.land)

This will open your default browser with your component loaded.

## Features

- TypeScript

- Function/Class Component

- Import External Libraries

- Run via URL

Check the [examples/](./examples) to see what is supported.

## Installation

With `cargo`:

```sh
cargo install react-run
```

Or use a binary from
[releases](https://github.com/UltiRequiem/react-run/releases/latest).

## Roadmap

- [x] Support running URL [#3](https://github.com/UltiRequiem/react-run/issues/3)
- [x] Support TypeScript [#2](https://github.com/UltiRequiem/react-run/issues/2)
- [ ] Live Reload [#1](https://github.com/UltiRequiem/react-run/issues/1)

## License

Licensed under the MIT licence.
