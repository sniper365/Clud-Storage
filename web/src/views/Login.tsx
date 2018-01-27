import * as React from "react";

import AuthService from "../services/Auth";

import LoginForm from "./login/LoginForm";

import { Redirect } from "react-router-dom";

import { Col, Container, Row, } from "reactstrap";
import { Panel, PanelBody, PanelHeader } from "../components/utils/Panel";

class LoginView extends React.Component<{}, {}> {
    public render() {
        if ( AuthService.authenticated() ) {
            return (
                <Redirect to="/"/>
            );
        }

        return (
            <Container className="mt-5">
                <Row className="justify-content-md-center">
                    <Col md={8}>
                        <Panel>
                            <PanelHeader>
                                Login
                            </PanelHeader>

                            <PanelBody>
                                <LoginForm/>
                            </PanelBody>
                        </Panel>
                    </Col>
                </Row>
            </Container>
        );
    }
}

export default LoginView;
