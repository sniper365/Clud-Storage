import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { File as FileModel } from "../../models/File";

interface Props {
    root: number;
    on_error?: (error: ErrorModel) => void;
    on_upload?: (file: FileModel) => void;
}

class NewFileButton extends React.Component<Props, {}> {
    constructor() {
        super();

        this.upload = this.upload.bind(this);
    }

    public upload(files: FileList | null) {
        if (!files) { return; }

        console.dir(files);

        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/" + this.props.root + '/files';

            for (let i = 0; i < files.length; ++i) {
                fetch(path, {
                    body: files[i],
                    headers: {
                        'Authorization': 'Bearer ' + TokenService.getToken(),
                        'Content-Type': 'text/plain'
                    },
                    method: 'post',
                }).then((response) => {
                    if (response.ok) {
                        return response.text();
                    }

                    return response.json();
                }).then((response) => {
                    if (response instanceof Object && response.status_code >= 400) {
                        if (this.props.on_error) { this.props.on_error(response); }
                    } else {
                        const file_name = files[i].name.split('.');

                        fetch(path, {
                            body: JSON.stringify({
                                extension: file_name.pop() || "text",
                                file_name: response,
                                name: file_name.join(''),
                            }),
                            headers: {
                                'Authorization': 'Bearer ' + TokenService.getToken(),
                                'Content-Type': 'application/json'
                            },
                            method: 'post',
                        }).then((file_response) => {
                            return file_response.json();
                        }).then((file_response) => {
                            if (file_response.status_code >= 400) {
                                if (this.props.on_error) { this.props.on_error(file_response); }
                            } else {
                                if (this.props.on_upload) { this.props.on_upload(file_response); }
                            }
                        });
                    }
                });
            }
        });
    }

    public render() {
        return (
            <div>
                <label htmlFor="new-file-button" className="s-floating-action-button button">
                    <img height="100%" className="s-floading-action-image"
                        src={require('../../icons/ic_add_black_24px.svg')}
                    />
                </label>
                <input id="new-file-button" type="file" hidden={true} multiple={true}
                    onChange={(e) => this.upload(e.target.files)}
                />
            </div>
        );
    }
}

export default NewFileButton;
