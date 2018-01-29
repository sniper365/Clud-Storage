import * as React from "react";

import { Col } from 'reactstrap';

import FileList from './FileList';

class Pane extends React.Component<{ root: number }, {  }> {
    constructor() {
        super();
    }

    public render() {
        return (
            <Col md={10} className="fill p-0">
                <FileList root={this.props.root}/>
            </Col>
        );
    }
}

export default Pane;
