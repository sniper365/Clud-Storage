import * as React from "react";
import { Navbar } from 'reactstrap';

import AuthService from "./services/Auth";

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

                { AuthService.authenticated() &&
                    <ul className="navbar-nav flex-row">
                        <NavbarItem path="home">
                            Home
                        </NavbarItem>

                        <NavbarItem path="settings">
                            Settings
                        </NavbarItem>
                    </ul>
                }

                { AuthService.authenticated() &&
                    <ul className="navbar-nav flex-row ml-md-auto">
                        <NavbarItem path="logout">
                            Logout
                        </NavbarItem>
                    </ul>
                }
            </Navbar>
        );
    }
}

export default Nav;
