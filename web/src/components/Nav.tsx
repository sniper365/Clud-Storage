import * as $ from "jquery";
import * as React from "react";

class Nav extends React.Component<{}, { }> {
    constructor() {
        super();
    }

    public logout() {
        // It's JSON web token auth; keeping authentication is client side
        // All we do to de-auth the user is destroy the token
        $('[name="_token"]').attr('content', '');
    }

    public render() {
        return (
            <div id="nav" className="w3-bar w3-blue-gray w3-medium">

                <ul className="w3-ul w3-margin-left">
                    <h2 id="title" className="w3-margin-0">Login</h2>
                </ul>

            </div>
        );
    }
}

export default Nav;
