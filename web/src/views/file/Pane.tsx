import * as React from "react";

import { Col } from 'reactstrap';

import FileSet from './FileSet';

class Pane extends React.Component<{ root: number }, {  }> {
    constructor() {
        super();
    }

    public render() {
        return (
            <Col md={10} className="fill p-0">
                <FileSet root={this.props.root}/>
            </Col>
        );
    }
}

export default Pane;
