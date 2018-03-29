import * as React from "react";

export class Panel extends React.Component<{}, {}> {
    constructor() {
        super();
    }

    public render() {
        return (
            <div className="panel panel-default panel-blue-gray">
                {this.props.children}
            </div>
        );
    }
}

export class PanelHeader extends React.Component<{}, {}> {
    public render() {
        return (
            <div className="panel-heading panel-heading-blue-gray">
                {this.props.children}
            </div>
        );
    }
}

export class PanelBody extends React.Component<{}, {}> {
    public render() {
        return (
            <div className="panel-body panel-body-blue-gray">
                {this.props.children}
            </div>
        );
    }
}

export class PanelFooter extends React.Component<{}, {}> {
    public render() {
        return (
            <div className="panel-footer panel-footer-blue-gray">
                {this.props.children}
            </div>
        );
    }
}
