import * as React from "react";

import { Col } from 'reactstrap';

import FileSet from './FileSet';

import Error from "../../components/utils/Error";

import ErrorModel from "../../models/Error";

interface Props {
    root: number;
}

interface State {
    error?: string;
}

class Pane extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            error: undefined,
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
            <Col md={10} className="fill p-0">
                {this.state.error && <Error message={this.state.error}/>}

                <FileSet root={this.props.root} on_error={this.on_error}/>
            </Col>
        );
    }
}

export default Pane;
