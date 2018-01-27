import * as React from "react";
import { Navbar, NavbarBrand } from 'reactstrap';

class Nav extends React.Component<{}, { }> {
    constructor() {
        super();
    }

    public render() {
        return (
            <Navbar className="blue-bar box-shadow-bottom">
                    <NavbarBrand href="/">
                        Storage
                    </NavbarBrand>
            </Navbar>
        );
    }
}

export default Nav;
