import * as React from "react";
import AuthService from "../../services/Auth";

import {
    Button,
    Form,
    FormGroup,
    Input,
    Label,
} from 'reactstrap';

import { Redirect } from "react-router-dom";

class LoginForm extends React.Component<
    {},
    {
        email: string,
        password: string,
        authenticated: boolean,
        pending: boolean
    }> {

    constructor() {
        super();

        this.login = this.login.bind(this);
        this.set_email = this.set_email.bind(this);
        this.set_password = this.set_password.bind(this);

        this.state = {
            authenticated: false,
            email: '',
            password: '',
            pending: false,
        };
    }

    public login( e: React.MouseEvent<HTMLButtonElement> ) {
        this.setState({
            pending: true,
        });

        AuthService.authenticate( this.state.email, this.state.password )
            .then(() => {
                this.setState({
                    authenticated: true,
                });
            });
    }

    public render() {
        if ( AuthService.authenticated() ) {
            return (
                <Redirect to="/"/>
            );
        }

        return (
            <Form>
                <FormGroup>
                    <Label htmlFor="email">Email</Label>
                    <Input id="email" type="text" className="input" onChange={this.set_email}/>
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="password">Password</Label>
                    <Input id="password" type="password" className="input" onChange={this.set_password}/>
                </FormGroup>

                <Button className="button button-primary float-right" onClick={this.login}>
                    {this.state.pending ? 'Logging In...' : 'Login'}
                </Button>

                <div className="clearfix"/>
            </Form>
        );
    }

    private set_email(e: React.ChangeEvent<HTMLInputElement>) {
        this.setState({
            email: e.target.value,
        });
    }

    private set_password(e: React.ChangeEvent<HTMLInputElement>) {
        this.setState({
            password: e.target.value,
        });
    }
}

export default LoginForm;
