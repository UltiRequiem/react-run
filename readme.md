# react-run

[![CI](https://github.com/UltiRequiem/react-run/actions/workflows/ci.yaml/badge.svg)](https://github.com/UltiRequiem/react-run/actions/workflows/ci.yaml)

Run React code snippets/components from your command-line without config.

## Usage

The entry point is a component called `App`, example:

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

You could run this with:

```sh
react-run example.js
```

This will open your default browser with your component loaded.

Check the [examples/](./examples) to see what is supported.

## Installation

With `cargo`:

```sh
cargo install react-run
```

Or use a binary from
[releases](https://github.com/UltiRequiem/react-run/releases/latest).

## License

Licensed under the MIT licence.
