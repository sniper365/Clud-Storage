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

import ErrorModel from "../../models/Error";
import { User as UserModel } from "../../models/User";

interface Props {
    on_error?: (error: ErrorModel) => void;
    on_success?: () => void;
}

interface State {
    user: UserModel;
    pending: boolean;
}

class UserForm extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            pending: false,
            user: new UserModel(),
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
                    'user': user
                });
            });
    }

    public set_name( e: React.ChangeEvent<HTMLInputElement> ) {
        const user = this.state.user;
        user.name = e.target.value;

        this.setState({
            'user': user,
        });
    }

    public set_email( e: React.ChangeEvent<HTMLInputElement> ) {
        const user = this.state.user;
        user.email = e.target.value;

        this.setState({
            'user': user,
        });
    }

    public save(e: React.MouseEvent<HTMLInputElement>) {
        e.preventDefault();

        this.setState({
            pending: true,
        });

        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id;

            fetch(path, {
                body: JSON.stringify({
                    email: this.state.user.email,
                    name: this.state.user.name,
                    user_id: user.user_id,
                }),
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                },
                method: 'put',
            }).then((response) => {
                if (response.ok) {
                    return response;
                }

                return response.json();
            }).then((response) => {
                this.setState({
                    pending: false
                });

                if (response.status_code >= 400) {
                    if (this.props.on_error) { this.props.on_error(response); }
                } else {
                    if (this.props.on_success) { this.props.on_success(); }
                }
            });
        });
    }

    public render() {
        return (
            <Form>
                <FormGroup>
                    <Label htmlFor="name">Name</Label>
                    <Input id="name"
                        type="text"
                        className="input"
                        value={this.state.user && this.state.user.name}
                        onChange={this.set_name}
                    />
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="email">Email</Label>
                    <Input
                        id="email"
                        type="text"
                        className="input"
                        value={this.state.user && this.state.user.email}
                        onChange={this.set_email}
                    />
                </FormGroup>

                <Button className="button button-primary float-right" type="submit" onClick={this.save}>
                    {this.state.pending ? "Saving..." : "Save"}
                </Button>

                <div className="clearfix"/>
            </Form>
        );
    }
}

export default UserForm;
