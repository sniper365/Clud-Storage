import * as React from "react";
import { Navbar } from 'reactstrap';

import NavbarBrand from "./components/navbar/Brand";
import NavbarItem from "./components/navbar/Item";

class Nav extends React.Component<{}, { }> {
    constructor() {
        super();
    }

    public render() {
        return (
            <Navbar className="top-bar box-shadow-bottom flex-column flex-md-row navbar-expand">
                <NavbarBrand/>

                <ul className="navbar-nav flex-row">
                    <NavbarItem path="home">
                        Home
                    </NavbarItem>

                    <NavbarItem path="settings">
                        Settings
                    </NavbarItem>
                </ul>

                <ul className="navbar-nav flex-row ml-md-auto">
                    <NavbarItem path="logout">
                        Logout
                    </NavbarItem>
                </ul>
            </Navbar>
        );
    }
}

export default Nav;
