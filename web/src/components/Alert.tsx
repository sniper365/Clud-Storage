import * as React from "react";

class Alert extends React.Component<{ alert: string, msg: string }, { }> {
    constructor( ) {
        super();
    }

    public render() {
        return (
            <div className="w3-panel w3-dark-grey w3-text-red alert">
                <p>
                    <strong>{this.props.alert}</strong>
                    <br/>
                    <span>{this.props.msg}</span>
                </p>
            </div>
        );
    }
}

export default Alert;
