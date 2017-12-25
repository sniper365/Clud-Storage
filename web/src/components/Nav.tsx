// import * as $ from "jquery";
import * as React from "react";
import AuthService from "../services/Auth";

class Nav extends React.Component<{}, { }> {
    constructor() {
        super();
    }

    public getUserName() {
        if ( !AuthService.authenticated() ) {
            return "Login";
        }

        return AuthService.getUser().name;
    }

    public render() {
        return (
            <div id="nav" className="w3-bar w3-blue-gray w3-medium">

                <ul className="w3-ul w3-margin-left">
                    <h2 id="title" className="w3-margin-0">
                        {this.getUserName()}
                    </h2>
                </ul>

            </div>
        );
    }
}

export default Nav;
