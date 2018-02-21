import * as React from "react";

import { Col, Container, Row } from 'reactstrap';

import FolderList from './FolderList';
import NewFolderButton from "./NewFolderButton";
import ParentFolder from "./ParentFolder";

import Error from "../../components/utils/Error";

import ErrorModel from "../../models/Error";
import FolderModel from "../../models/Folder";

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
        this.on_save = this.on_save.bind(this);
    }

    public on_error(error: ErrorModel) {
        this.setState({
            error: error.message
        });
    }

    public on_save(response: FolderModel) {
        this.setState(this.state);
    }

    public render() {
        return (
            <Col md={2} className="fill p-0 s-nav">
                {this.state.error && <Error message={this.state.error}/>}

                <Container className="action-set">
                    <Row>
                        <ParentFolder root={this.props.root}/>

                        <NewFolderButton root={this.props.root} on_save={this.on_save} on_error={this.on_error}/>
                    </Row>
                </Container>

                <FolderList root={this.props.root} on_error={this.on_error} key={Math.random()}/>
            </Col>
        );
    }
}

export default Nav;
