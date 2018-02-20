import * as React from "react";
import { Link } from "react-router-dom";

import AuthService from "../../services/Auth";

import { Col, ListGroup } from 'reactstrap';

import User from "../../models/User";

class SettingsList extends React.Component<{}, { user: User }> {
    constructor() {
        super();

        this.state = {
            user: new User()
        };

        this.load = this.load.bind(this);

        this.load();
    }

    public load() {
        AuthService.user()
            .then((user) => {
                this.setState({
                    user: user
                });
            });
    }

    public render() {
        return (
            <Col md={3}>
                <ListGroup flush={true} className="s-list-group">
                    <Link to="/settings/me" className="list-group-item">Account</Link>
                    <Link to="/settings/auth" className="list-group-item">Authentication</Link>
                </ListGroup>
            </Col>
        );
    }
}

export default SettingsList;
