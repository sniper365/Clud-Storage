import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

class NewFileButton extends React.Component<{ root: number, onUpload: Function }, {  }> {
    constructor() {
        super();

        this.upload = this.upload.bind(this);
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
                        this.props.onUpload();
                    });
                });
            }
        });
    }

    public render() {
        return (
            <div>
                <label htmlFor="new-file-button" className="s-floating-action-button button">
                    <img height="100%" className="s-floading-action-image" src={require('../../icons/ic_add_black_24px.svg')}/>
                </label>
                <input id="new-file-button" type="file" hidden multiple onChange={(e) => this.upload(e.target.files)}/>
            </div>
        );
    }
}

export default NewFileButton;
