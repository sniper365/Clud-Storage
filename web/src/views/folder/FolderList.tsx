import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { Folder as FolderModel } from "../../models/Folder";

import Folder from "./Folder";

import { ListGroup } from 'reactstrap';

interface Props {
    root: number;
    on_load?: (response: FolderModel[]) => void;
    on_error?: (error: ErrorModel) => void;
}

interface State {
    folders: FolderModel[];
}

class FolderList extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            folders: [],
        };

        this.load = this.load.bind(this);

        this.load();
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
            <ListGroup flush={true} className="folder-list">
                {this.state.folders.map( folder => <Folder folder={folder} key={folder.folder_id}/> )}
            </ListGroup>
        );
    }
}

export default FolderList;
