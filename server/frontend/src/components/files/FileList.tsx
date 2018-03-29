import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { File as FileModel } from "../../models/File";

import File from "./File";

import { Col } from 'reactstrap';

class FileList extends React.Component<{ root: number }, { files: FileModel[] }> {
    constructor() {
        super();

        this.state = {
            files: [],
        };

        this.load = this.load.bind(this);

        this.load();
    }

    public load() {
        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/" + this.props.root + '/files';

            fetch(path, {
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                }
            }).then((response) => {
                return response.json();
            }).then((response: FileModel[]) => {
                this.setState({
                    files: response
                });
            });
        });
    }

    public render() {
        return (
            <Col md={10} className="fill">
                {this.state.files.map( file => <File file={file} key={file.file_id}/>)}
            </Col>
        );
    }
}

export default FileList;
