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
