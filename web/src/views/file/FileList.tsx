import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { File as FileModel } from "../../models/File";

import FileDrop from 'react-file-drop';

class FileList extends React.Component<{ root: number }, { files: FileModel[] }> {
    constructor() {
        super();

        this.state = {
            files: [],
        };

        this.load = this.load.bind(this);
        this.upload = this.upload.bind(this);

        this.load();
    }

    public upload(files) {
        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/" + this.props.root + '/files';

            for(let i = 0; i < files.length; i++) {
                fetch(path, {
                    method: 'post',
                    headers: {
                        'Authorization': 'Bearer ' + TokenService.getToken(),
                        'Content-Type': 'text/plain'
                    },
                    body: files[i],
                }).then((response) => {
                    return response.text();
                }).then((response) => {
                    console.log(files[i]);
                    console.log(response);

                    let file_name = files[i].name.split('.');

                    fetch(path, {
                        method: 'post',
                        headers: {
                            'Authorization': 'Bearer ' + TokenService.getToken(),
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify({
                            extension: file_name.pop() || "text",
                            name: file_name.join(''),
                            file_name: response,
                        }),
                    }).then((response) => {
                        return response.json();
                    }).then((response) => {
                        this.load();
                    });
                });
            }
        });
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
            <FileDrop className="fill" frame={document} onDrop={this.upload}>
                aaaa
            </FileDrop>
        );
    }
}

export default FileList;
