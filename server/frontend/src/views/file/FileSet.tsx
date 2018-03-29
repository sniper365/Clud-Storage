import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { File as FileModel } from "../../models/File";

import FileSetItem from "./FileSetItem";

import NewFileButton from "./NewFileButton";

interface Props {
    root: number;
    on_error?: (error: ErrorModel) => void;
    on_load?: (file: FileModel[]) => void;
}

interface State {
    files: FileModel[];
}

class FileSet extends React.Component<Props, State> {
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
            }).then((response) => {
                if (response.status_code >= 400) {
                    if (this.props.on_error) {
                        this.props.on_error(response);
                    }
                } else {
                    const data: FileModel[] = response;

                    this.setState({
                        files: data
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
            <div className="file-set">
                {this.state.files.map( file => <FileSetItem file={file} key={file.file_id}/> )}

                <NewFileButton root={this.props.root} on_upload={this.load}/>
            </div>
        );
    }
}

export default FileSet;
