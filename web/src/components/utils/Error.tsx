import * as React from "react";

class Error extends React.Component<{ message: string }, {}> {
    constructor() {
        super();
    }

    public render() {
        return (
            <div className="alert s-alert s-alert-danger">
                {this.props.message}
            </div>
        );
    }
}

export default Error;
