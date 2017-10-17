import * as $ from "jquery";
import * as React from "react";

class LoginForm extends React.Component<{}, { }> {
    constructor() {
        super();
    }

    public login() {
        $('#login').html("Logging In...");

        fetch("/api/login", {
            body: JSON.stringify({
                'email': $('#email').val(),
                'password': $('#password').val(),
            }),
            headers: {
                'Content-Type': 'application/json'
            },
            method: 'POST'
        })
        .then((response) => {
            return response.json();
        })
        .then((response) => {
            $('[name="_token"]').attr('content', response.token);
        });
    }

    public render() {
        return (
            <div className="container">
                <div className="row">
                    <div className="col-md-8 col-md-offset-2">
                        <div className="panel panel-default">
                            <div className="panel-heading">
                                Login
                            </div>
                            <div className="panel-body">
                                <div className="form-group">
                                    <label htmlFor="email" className="control-label">Email</label>
                                    <input id="email" type="text" name="email" className="form-control"/>
                                </div>

                                <div className="form-group">
                                    <label htmlFor="password" className="control-label">Password</label>
                                    <input id="password" type="password" name="password" className="form-control"/>
                                </div>
                            </div>

                            <div className="panel-footer">
                                <button className="btn btn-primary pull-right" id="login" type="submit" onClick={this.login}>
                                    Login
                                </button>
                                <div className="clearfix"/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}

export default LoginForm;
