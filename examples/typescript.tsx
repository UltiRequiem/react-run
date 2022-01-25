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
