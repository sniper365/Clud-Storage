import * as React from "react";

import { Col, Container, Row } from 'reactstrap';

import FolderList from './FolderList';
import NewFolderButton from "./NewFolderButton";
import ParentFolder from "./ParentFolder";

import Error from "../../components/utils/Error";

import ErrorModel from "../../models/Error";

interface Props {
    root: number;
}

interface State {
    error?: string;
}

class Nav extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            error: undefined
        };

        this.on_error = this.on_error.bind(this);
    }

    public on_error(error: ErrorModel) {
        this.setState({
            error: error.message
        });
    }

    public render() {
        return (
            <Col md={2} className="fill p-0 s-nav">
                {this.state.error && <Error message={this.state.error}/>}

                <Container className="action-set">
                    <Row>
                        <ParentFolder root={this.props.root}/>

                        <NewFolderButton root={this.props.root}/>
                    </Row>
                </Container>

                <FolderList root={this.props.root} on_error={this.on_error}/>
            </Col>
        );
    }
}

export default Nav;
