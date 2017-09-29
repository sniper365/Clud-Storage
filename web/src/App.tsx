import * as React from "react";
import "./App.css";

class App extends React.Component<{}, { api_message: string }> {
  constructor() {
    super();

    this.state = { api_message: "" };
  }

  public componentDidMount() {
    fetch("/api").then(r => r.text()).then(api_message => {
      this.setState({
        api_message
      });
    });
  }

  public render() {
    return (
      <div className="App">
      </div>
    );
  }
}

export default App;
