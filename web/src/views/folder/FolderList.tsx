import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { Folder as FolderModel } from "../../models/Folder";

import Error from "../../components/utils/Error";
import Folder from "./Folder";

import { ListGroup } from 'reactstrap';

interface Props {
    root: number;
    on_load?: (response: FolderModel[]) => void;
    on_error?: (error: ErrorModel) => void;
}

interface State {
    folders: FolderModel[];
    error?: string;
}

class FolderList extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            error: undefined,
            folders: [],
        };

        this.load = this.load.bind(this);

        this.load();
    }

    public on_error(error: ErrorModel) {
        this.setState({
            error: error.message
        });
    }

    public load() {
        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/" + this.props.root + '/children';

            fetch(path, {
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                }
            }).then((response) => {
                return response.json();
            }).then((response) => {
                if (response.status_code >= 400) {
                    this.on_error(response);

                    if (this.props.on_error) {
                        this.props.on_error(response);
                    }
                } else {
                    const data: FolderModel[] = response;

                    this.setState({
                        folders: data
                    });

                    if (this.props.on_load) {
                        this.props.on_load(data);
                    }
                }
            });
        });
    }

    public render() {
        return (
            <div>
                {this.state.error && <Error message={this.state.error}/>}

                <ListGroup flush={true} className="folder-list">
                    {this.state.folders.map( folder => <Folder folder={folder} key={folder.folder_id}/> )}
                </ListGroup>
            </div>
        );
    }
}

export default FolderList;
