import * as React from "react";

import { Col, Container, Row } from 'reactstrap';

import FolderList from './FolderList';
import NewFolderButton from "./NewFolderButton";
import ParentFolder from "./ParentFolder";


class Nav extends React.Component<{ root: number }, { }> {
    constructor() {
        super();

    }

    public render() {
        return (
            <Col md={2} className="fill p-0">
                <Container className="action-set p-2">
                    <Row>
                        <ParentFolder root={this.props.root}/>

                        <NewFolderButton root={this.props.root}/>
                    </Row>
                </Container>

                <FolderList root={this.props.root}/>
            </Col>
        );
    }
}

export default Nav;
