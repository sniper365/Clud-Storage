import * as React from "react";

import LoginForm from "./login/LoginForm";

import { Redirect } from "react-router-dom";

import { Col, Container, Row, } from "reactstrap";
import { Panel, PanelBody, PanelHeader } from "../components/utils/Panel";

import Error from "../components/utils/Error";

import ErrorMessage from "../models/Error";

interface Props {

}

interface State {
    authenticated: boolean,
    error?: string,
}

class LoginView extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            authenticated: false,
            error: undefined
        };

        this.on_error = this.on_error.bind(this);
        this.on_success = this.on_success.bind(this);
    }

    public on_error(error: ErrorMessage) {
        this.setState({
            error: error.message
        });
    }

    public on_success(_response) {
        this.setState({
            authenticated: true
        })
    }

    public render() {
        if ( this.state.authenticated ) {
            return (
                <Redirect to="/"/>
            );
        }

        return (
            <Container className="mt-5">
                <Row className="justify-content-md-center">
                    <Col md={8}>
                        {this.state.error && <Error message={this.state.error}/>}

                        <Panel>
                            <PanelHeader>
                                Login
                            </PanelHeader>

                            <PanelBody>
                                <LoginForm on_error={this.on_error}/>
                            </PanelBody>
                        </Panel>
                    </Col>
                </Row>
            </Container>
        );
    }
}

export default LoginView;
