import * as React from 'react';

import {
    Button,
    Form,
    FormGroup,
    Input,
    Label,
} from 'reactstrap';

import AuthService from "../../services/Auth";

import User from "../../models/User";

class UserForm extends React.Component<{}, { user: User }> {
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
            <Form>
                <FormGroup>
                    <Label htmlFor="email">Email</Label>
                    <Input id="email" type="text" className="input" />
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="password">Password</Label>
                    <Input id="password" type="password" className="input" />
                </FormGroup>

                <Button className="button button-primary float-right">

                </Button>

                <div className="clearfix"/>
            </Form>
        );
    }
}

export default UserForm;
