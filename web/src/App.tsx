import * as React from "react";
import "./App.css";

import {
    BrowserRouter as Router,
    Redirect,
    Route,
} from "react-router-dom";

import LoginForm from "./components/LoginForm";
import Nav from "./components/Nav";
import Viewport from "./components/Viewport";

import AuthService from "./services/Auth";

const PrivateRoute  = ({ component: Component, ...rest }) => (
    <Route {...rest} render={ props => (
        AuthService.isAuthenticated
            ? <Component {...props} />
            : <Redirect to={{ pathname: "/", state: { from: props.location }}}/>
    )} />
);

class App extends React.Component<{}, {}> {
  constructor() {
    super();
  }

  public render() {
    return (
      <Router>
        <div id="app">
            <Nav />

            <Route path="/" component={LoginForm} />
            <PrivateRoute path="/home" component={Viewport} />
        </div>
      </Router>
    );
  }
}

export default App;
