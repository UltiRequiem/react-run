function sum(...numbers) {
  return numbers.reduce((acc, curr) => acc + curr, 0);
}

function App() {
  const [count, setCount] = React.useState(0);

  return (
    <div>
      <h1>Hello World</h1>
      <p>This is a simple example of a React app.</p>
      <p> {sum(1, 3, 4, 10, 4)}</p>
      <p>{count}</p>
      <button onClick={() => setCount(count + 1)}>+1</button>
    </div>
  );
}
