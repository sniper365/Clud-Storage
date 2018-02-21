import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { Link } from "react-router-dom";
import { Col } from 'reactstrap';

import ErrorModel from "../../models/Error";
import FolderModel from "../../models/Folder";

interface Props {
    root: number;
    on_error?: (error: ErrorModel) => void;
}

interface State {
    root?: FolderModel;
}

class ParentFolder extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            root: undefined
        };

        this.load();
    }

    public load() {
        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/" + this.props.root;

            fetch(path, {
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                }
            }).then((response) => {
                return response.json();
            }).then((response) => {
                if (response.status_code >= 400) {
                    if (this.props.on_error) { this.props.on_error(response); }
                } else {
                    this.setState({
                        root: response
                    });
                }
            });
        });
    }

    public render() {
        if ( this.state.root && this.state.root.parent_id ) {
            return (
                <Link to={"/folders/" + this.state.root.parent_id} className="col-md-8 action p-2">
                <img className="" src={require('../../icons/ic_arrow_upward_black_24px.svg')}/>
                </Link>
            );
        }

        return (
            <Col md={8} className="p-2"/>
        );
    }
}

export default ParentFolder;
