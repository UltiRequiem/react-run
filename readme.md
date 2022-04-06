# react-run

Run React code snippets without config 🚀

## Usage

The entry point is always a component called `App` 👇

```tsx
// example.tsx

function ProductList({ products }: { products: string[] }) {
  return products.map((p) => <li>{p}</li>);
}

export default function App() {
  const style = {
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

Run it 🏃

```sh
react-run example.tsx
```

Or run a hosted file 🤖

```sh
react-run https://raw.githubusercontent.com/UltiRequiem/react-run/main/examples/typescript.tsx
```

> This will open your default browser with your component loaded 🤯

## Features

- JSX/TSX

- Functional or Class Components

- Import External Libraries

- Run via URL

Check the [examples/](./examples) to see what is supported.

## Installation

> [Cargo](https://doc.rust-lang.org/cargo) is the Rust package manager.

```sh
cargo install react-run
```

Or use a binary from
[releases](https://github.com/UltiRequiem/react-run/releases/latest).

## Standing on the shoulders of giants

- 🗼 [tokio-rs](https://github.com/tokio-rs/tokio): A runtime for writing
  reliable asynchronous applications with Rust.

- 🤗 [colored](https://github.com/mackwic/colored): The easier way to have text
  on your term!

- 👏 [clap](https://github.com/clap-rs/clap): A full featured, fast Command Line
  Argument Parser for Rust

- ⚡ [minireq](https://github.com/neonmoe/minreq): Simple, minimal-dependency
  HTTP client.

- 🏎️ [swc](https://swc.rs): SWC is 20x faster than Babel on a single thread and
  70x faster on four cores.

## Roadmap

- [x] Integrate with SWC [#4](https://github.com/UltiRequiem/react-run/issues/4)
- [x] Support TypeScript [#2](https://github.com/UltiRequiem/react-run/issues/2)
- [x] Support running URLs
      [#3](https://github.com/UltiRequiem/react-run/issues/3)
- [ ] Live Reload [#1](https://github.com/UltiRequiem/react-run/issues/1)

## Support

Open an Issue, I will check it a soon as possible 👀

If you want to hurry me up a bit
[send me a tweet](https://twitter.com/UltiRequiem) 😆

Consider [supporting me on Patreon](https://patreon.com/UltiRequiem) if you like
my work 🙏

Don't forget to start the repo ⭐

## Versioning

We use [Semantic Versioning](http://semver.org). For the versions available, see
the [tags](https://github.com/UltiRequiem/react-run/tags) 🏷️

## Authors

[Eliaz Bobadilla](https://ultirequiem.com) - Creator and Maintainer 💪

See also the full list of
[contributors](https://github.com/UltiRequiem/react-run/contributors) who
participated in this project ✨

## Licence

Licensed under the MIT License 📄
