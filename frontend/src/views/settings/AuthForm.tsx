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

interface Props {
    on_error?: (error: ErrorModel) => void;
    on_success?: () => void;
}

interface State {
    pending: boolean;
    curr_password: string;
    new_password: string;
    conf_password: string;
}

class AuthForm extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            conf_password: "",
            curr_password: "",
            new_password: "",
            pending: false,
        };

        this.set_curr_password = this.set_curr_password.bind(this);
        this.set_new_password = this.set_new_password.bind(this);
        this.set_conf_password = this.set_conf_password.bind(this);
        this.save = this.save.bind(this);
    }

    public set_curr_password( e: React.ChangeEvent<HTMLInputElement> ) {
        this.setState({
            curr_password: e.target.value,
        });
    }

    public set_new_password( e: React.ChangeEvent<HTMLInputElement> ) {
        this.setState({
            new_password: e.target.value,
        });
    }

    public set_conf_password( e: React.ChangeEvent<HTMLInputElement> ) {
        this.setState({
            conf_password: e.target.value,
        });
    }

    public save(e: React.MouseEvent<HTMLInputElement>) {
        e.preventDefault();

        this.setState({
            pending: true,
        });

        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/password";

            fetch(path, {
                body: JSON.stringify({
                    current_password: this.state.curr_password,
                    password: this.state.new_password,
                    password_confirmation: this.state.conf_password,
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
                    <Label htmlFor="current_password">Current Password</Label>
                    <Input id="current_password"
                        name="current_password"
                        type="password"
                        className="input"
                        value={this.state.curr_password}
                        onChange={this.set_curr_password}
                    />
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="password">New Password</Label>
                    <Input
                        id="password"
                        name="password"
                        type="password"
                        className="input"
                        value={this.state.new_password}
                        onChange={this.set_new_password}
                    />
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="password_confirmation">Confirm New Password</Label>
                    <Input
                        id="password_confirmation"
                        name="password_confirmation"
                        type="password"
                        className="input"
                        value={this.state.conf_password}
                        onChange={this.set_conf_password}
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

export default AuthForm;
