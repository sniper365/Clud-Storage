import * as $ from "jquery";
import * as React from "react";
import { Redirect } from "react-router-dom";

import AuthService from "../services/Auth";

class LoginForm extends React.Component<{ }, { email: string, password: string, authenticated: boolean }> {
    constructor() {
        super();

        this.login = this.login.bind(this);
        this.set_email = this.set_email.bind(this);
        this.set_password = this.set_password.bind(this);

        this.state = {
            authenticated: false,
            email: '',
            password: '',
        };
    }

    public login( e: React.MouseEvent<HTMLButtonElement> ) {
        e.preventDefault();

        $('#login').html("Logging In...");

        const authenticated = AuthService.login( this.state.email, this.state.password );

        if ( authenticated ) {
            $('[id=title]').html( AuthService.getUser().name );

            this.setState({
                authenticated: true,
            });
        } else {
            $('#login').html("Login");
        }
    }

    public render() {
        if ( this.state.authenticated ) {
            return (
                <Redirect to="/folders"/>
            );
        }

        return (
            <div className="w3-panel w3-blue-gray login-form">
                <form>

                    <div className="w3-margin-top w3-margin-bottom">
                        <input id="email" type="text" className="w3-input w3-border-0" onChange={this.set_email}/>
                        <label htmlFor="email">Email</label>
                    </div>

                    <div className="w3-margin-top w3-margin-bottom">
                        <input id="password" type="password"
                            className="w3-input w3-border-0" onChange={this.set_password}/>
                        <label htmlFor="password">Password</label>
                    </div>

                    <button className="w3-btn login-btn" id="login" type="submit" onClick={this.login}>
                        Login
                    </button>

                </form>
            </div>
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
