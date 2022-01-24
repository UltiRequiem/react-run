export default class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      content: "Reject common sense to make the impossible possible!",
      author: "Kamina",
    };
    this.updateQuote = this.updateQuote.bind(this);
  }

  updateQuote() {
    fetch("https://api.quotable.io/random")
      .then((response) => response.json())
      .then((data) => this.setState(data));
  }

  render() {
    const { content, author } = this.state;
    return (
      <>
        <figure>
          <blockquote>&quot;{content}&quot;</blockquote>
          <figcaption>
            &mdash; <cite>{author}</cite>
          </figcaption>
        </figure>
        <button type="button" variant="primary" onClick={this.updateQuote}>
          Random Quote
        </button>
      </>
    );
  }
}
