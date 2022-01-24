import Calendar from "https://esm.sh/react-calendar";

export default function App() {
  const [value, onChange] = React.useState(new Date());

  const style = {
    background: "blue",
    height: "90vh",
    alignItems: "center",
    justifyContent: "center",
    display: "flex",
    color: "white",
    flexDirection: "column",
    padding: "1em",
  };

  return (
    <div style={style}>
      <h1>React Calendar</h1>
      <Calendar onChange={onChange} value={value} />
      <div>
        <p>Date selected: {value.toDateString()}</p>
      </div>
    </div>
  );
}
