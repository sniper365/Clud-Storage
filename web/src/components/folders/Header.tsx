import * as React from "react";

class Header extends React.Component<{ name: string }, { }> {
    constructor( ) {
        super();
    }

    public render() {
        return (
            <h3 className="w3-rest">
                {this.props.name}
            </h3>
        );
    }
}

export default Header;
