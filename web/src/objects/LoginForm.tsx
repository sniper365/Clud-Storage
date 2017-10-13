import * as React from "react";

class LoginForm extends React.Component<{}, { }> {
    constructor() {
        super();
    }

// public componentDidMount() {
//     fetch("/api").then(r => r.text()).then(api_message => {
//         this.setState({
//             api_message
//         });
//     });
// }

    public render() {
        return (
            <div className="container">
                <div className="row">
                    <div className="col-md-8 col-md-offset-2">
                        <div className="panel panel-default">
                            <div className="panel-heading">
                                Login
                            </div>

                            <form action="/api/login" method="post">
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
                                    <button className="btn btn-primary pull-right" type="submit">Login</button>
                                    <div className="clearfix"/>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}

export default LoginForm;
