import * as React from "react";
import AuthService from "../../services/Auth";

import {
    Button,
    Form,
    FormGroup,
    Input,
    Label,
} from 'reactstrap';

import Error from "../../models/Error";
import Session from "../../models/Session";

interface Props {
    on_success?: (response: Session) => void;
    on_error?: (error: Error) => void;
}

interface State {
    email: string;
    password: string;
    pending: boolean;
}

class LoginForm extends React.Component<Props, State> {
    constructor(_props: Props) {
        super(_props);

        this.login = this.login.bind(this);
        this.set_email = this.set_email.bind(this);
        this.set_password = this.set_password.bind(this);

        this.state = {
            email: '',
            password: '',
            pending: false,
        };
    }

    public login( e: React.MouseEvent<HTMLButtonElement> ) {
        e.preventDefault();

        this.setState({
            pending: true,
        });

        AuthService.authenticate( this.state.email, this.state.password )
            .then((response) => {
                this.setState({
                    pending: false,
                });

                if ( AuthService.authenticated() ) {
                    if ( this.props.on_success ) {
                        this.props.on_success(response);
                    }
                } else {
                    if ( this.props.on_error ) {
                        this.props.on_error(response);
                    }
                }
            });
    }

    public render() {

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

                <Button className="button button-primary float-right" type="submit" onClick={this.login}>
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
