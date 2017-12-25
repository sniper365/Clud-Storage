import * as React from "react";
import "./App.css";

import {
    BrowserRouter as Router,
    Redirect,
    Route,
    Switch,
} from "react-router-dom";

import LoginForm from "./components/LoginForm";
import Nav from "./components/Nav";
import Viewport from "./components/Viewport";

import AuthService from "./services/Auth";

const PrivateRoute  = ({ component: Component, ...rest }) => (
    <Route {...rest} render={ props => (
        AuthService.authenticated()
            ? <Component {...props} />
            : <Redirect to={{ pathname: "/login", state: { from: props.location }}}/>
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

            <Switch>
                <Route path="/login" component={LoginForm} />
                <PrivateRoute path="/" component={Viewport} />
                <PrivateRoute path="/folders/:folder_id" component={Viewport} />
            </Switch>
        </div>
      </Router>
    );
  }
}

export default App;
