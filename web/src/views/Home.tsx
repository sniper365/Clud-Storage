import * as React from "react";

import AuthService from "../services/Auth";

import Nav from "./folder/Nav";

class HomeView extends React.Component<{ }, { root?: number}> {
    constructor() {
        super();

        this.state = {
            root: undefined
        };

        AuthService.user().then((user) => {
            this.setState({
                root: user.root,
            });
        });
    }

    public render() {
        return (
            <div className="row fill">
                { this.state.root && <Nav root={this.state.root}/> }
            </div>
        );
    }
}

export default HomeView;
