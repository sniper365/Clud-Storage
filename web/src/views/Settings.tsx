import * as React from "react";

import { Route, Switch } from "react-router-dom";

import { Col, Container, Row, } from "reactstrap";
import { Panel, PanelBody } from "../components/utils/Panel";

import AuthForm from "./settings/AuthForm";
import SettingsList from "./settings/SettingsList";
import UserForm from "./settings/UserForm";

import Error from "../components/utils/Error";

import ErrorModel from "../models/Error";

interface State {
    error?: string;
}

class Settings extends React.Component<{}, State> {
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
            <Container className="mt-5">
                <Row className="justify-content-md-center">
                    <SettingsList/>

                    <Col md={9}>
                        {this.state.error && <Error message={this.state.error}/>}

                        <Panel>
                            <PanelBody>
                                <Switch>
                                    <Route exact={true} path="/settings" render={() => (
                                        <UserForm on_error={this.on_error}/>
                                    )}/>
                                    <Route path="/settings/me" render={() => (
                                        <UserForm on_error={this.on_error}/>
                                    )}/>
                                    <Route path="/settings/auth" render={() => (
                                        <AuthForm on_error={this.on_error}/>
                                    )}/>
                                </Switch>
                            </PanelBody>
                        </Panel>
                    </Col>
                </Row>
            </Container>
        );
    }
}

export default Settings;
