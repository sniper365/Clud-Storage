import * as React from "react";

import { Switch, Route, Link } from "react-router-dom";

class NavbarItem extends React.Component<{ path: string }, {}> {
    public render() {
        return (
            <Switch>
                <Route exact path={"/" + this.props.path}>
                    <Link to={"/" + this.props.path } className="nav-link nav-item s-nav-item s-nav-item-active">
                        {this.props.children}
                    </Link>
                </Route>

                <Route path="">
                    <Link to={"/" + this.props.path } className="nav-link nav-item s-nav-item">
                        {this.props.children}
                    </Link>
                </Route>
            </Switch>
        );
    }
}

export default NavbarItem;
