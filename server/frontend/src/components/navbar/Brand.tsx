import * as React from "react";

import { Link } from "react-router-dom";

class Brand extends React.Component<{}, {}> {
    public render() {
        return (
            <Link to={"/home"} className="navbar-brand s-navbar-brand">
                Storage
            </Link>
        );
    }
}

export default Brand;
