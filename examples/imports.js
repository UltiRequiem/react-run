import Calendar from "https://esm.sh/react-calendar";

export default function App() {
  const [value, onChange] = React.useState(new Date());

  const style = {
    background: "blue",
    color: "white",
    padding: "1em",
  };

  return (
    <div style={style}>
      <Calendar onChange={onChange} value={value} />
      <p>Date selected: {value.toString()}</p>
    </div>
  );
}
