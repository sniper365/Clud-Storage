import * as React from "react";

import { File as FileModel } from "../../models/File";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { Card, CardImg, CardImgOverlay } from 'reactstrap';

class File extends React.Component<{ file: FileModel }, { background: string }> {
    constructor() {
        super();

        this.state = {
            background: "none",
        };

        this.load = this.load.bind(this);

        this.load();
    }

    public load() {
        AuthService.user().then((user) => {
            const path = "/api/users/" +
                user.user_id + "/folders/" +
                this.props.file.folder_id + "/files/" +
                this.props.file.file_id + "/download";

            fetch(path, {
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                }
            }).then((response) => {
                return response.blob();
            }).then((response) => {
                const url = URL.createObjectURL(response);

                this.setState({
                    background: url
                });
            });
        });
    }

    public render() {
        return (
            <Card className="file m-3">
                <CardImg src={this.state.background}/>
                <CardImgOverlay>
                    {this.props.file.name}
                </CardImgOverlay>
            </Card>
        );
    }
}

export default File;
