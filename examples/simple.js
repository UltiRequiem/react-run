function App() {
  const [count, setCount] = React.useState(0);

  const style = { background: "red", color: "white", padding: "1em" };

  return (
    <div style={style}>
      <h1>Hello World</h1>
      <p>This is a simple example of a React app.</p>
      <p>{count}</p>
      <button onClick={() => setCount(count + Math.random())}>+ random</button>
      <button onClick={() => setCount(count + 1)}>+1</button>
    </div>
  );
}
