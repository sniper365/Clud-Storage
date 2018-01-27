import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { Folder as FolderModel } from "../../models/Folder";

import Folder from "./Folder";

import { ListGroup } from 'reactstrap';

class FolderList extends React.Component<{ root: number }, { folders: FolderModel[] }> {
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
            }).then((response: FolderModel[]) => {
                this.setState({
                    folders: response
                });
            });
        });
    }

    public render() {
        return (
            <ListGroup flush={true} className="folder-list fade-in">
                {this.state.folders.map( folder => <Folder folder={folder} key={folder.folder_id}/> )}
            </ListGroup>
        );
    }
}

export default FolderList;
