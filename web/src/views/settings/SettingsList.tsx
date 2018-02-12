import * as React from "react";

import AuthService from "../../services/Auth";

import { Col, ListGroup, ListGroupItem } from 'reactstrap';

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
                    <ListGroupItem>aaaa
                    </ListGroupItem>
                </ListGroup>
            </Col>
        );
    }
}

export default SettingsList;
