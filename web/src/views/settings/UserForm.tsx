import * as React from 'react';

import {
    Button,
    Form,
    FormGroup,
    Input,
    Label,
} from 'reactstrap';

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import User from "../../models/User";

class UserForm extends React.Component<{}, { user: User, pending: boolean }> {
    constructor() {
        super();

        this.state = {
            user: new User(),
            pending: false
        };

        this.load = this.load.bind(this);
        this.set_name = this.set_name.bind(this);
        this.set_email = this.set_email.bind(this);
        this.save = this.save.bind(this);

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

    public set_name( e: React.ChangeEvent<HTMLInputElement> ) {
        let user = this.state.user;
        user.name = e.target.value;

        this.setState({
            user: user,
        });
    }

    public set_email( e: React.ChangeEvent<HTMLInputElement> ) {
        let user = this.state.user;
        user.email = e.target.value;

        this.setState({
            user: user,
        });
    }

    public save(e) {
        e.preventDefault();

        this.setState({
            pending: true,
        });

        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id;

            fetch(path, {
                method: 'put',
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    user_id: user.user_id,
                    name: this.state.user.name,
                    email: this.state.user.email,
                }),
            }).then((response) => {
                this.setState({
                    pending: false
                });
            });
        });
    }

    public render() {
        return (
            <Form>
                <FormGroup>
                    <Label htmlFor="name">Name</Label>
                    <Input id="name" type="text" className="input" value={this.state.user && this.state.user.name} onChange={this.set_name}/>
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="email">Email</Label>
                    <Input id="email" type="text" className="input" value={this.state.user && this.state.user.email} onChange={this.set_email}/>
                </FormGroup>

                <Button className="button button-primary float-right" onClick={this.save}>
                    {this.state.pending ? "Saving..." : "Save"}
                </Button>

                <div className="clearfix"/>
            </Form>
        );
    }
}

export default UserForm;
