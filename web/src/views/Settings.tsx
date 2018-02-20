import * as React from "react";

import { Route, Switch } from "react-router-dom";

import { Col, Container, Row, } from "reactstrap";
import { Panel, PanelBody } from "../components/utils/Panel";

import SettingsList from "./settings/SettingsList";
import UserForm from "./settings/UserForm";
import AuthForm from "./settings/AuthForm";

class Settings extends React.Component<{}, {}> {
    public render() {
        return (
            <Container className="mt-5">
                <Row className="justify-content-md-center">
                    <SettingsList/>

                    <Col md={9}>
                        <Panel>
                            <PanelBody>
                                <Switch>
                                    <Route exact path="/settings" component={UserForm}/>
                                    <Route path="/settings/me" component={UserForm}/>
                                    <Route path="/settings/auth" component={AuthForm}/>
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
