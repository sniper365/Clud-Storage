import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { File as FileModel } from "../../models/File";

interface Props {
    file: FileModel;
}

interface State {
    background: string;
}

class File extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            background: ""
        };

        this.isImage = this.isImage.bind(this);
        this.load = this.load.bind(this);
    }

    public componentDidMount() {
        this.load();
    }

    public load() {
        if (!this.isImage()) {
            return;
        }

        AuthService.user()
            .then((user) => {
                const file = this.props.file;

                const path = '/api/users/' + user.user_id +
                    '/folders/' + file.folder_id +
                    '/files/' + file.file_id +
                    '/download';

                fetch(path, {
                    headers: {
                        'Authorization': 'Bearer ' + TokenService.getToken(),
                        'Content-Type': 'application/json'
                    },
                    method: 'GET',
                }).then((response) => {
                    return response.blob();
                }).then((image) => {
                    const url = URL.createObjectURL(image);

                    this.setState({
                        background: "url(" + url + ")"
                    });
                });
            });
    }

    public isImage() {
        return ['jpg', 'jpeg', 'png', 'svg', 'png'].indexOf(this.props.file.extension) !== -1;
    }

    public render() {
        return (
            <div className="file-header" style={{ backgroundImage: this.state.background }} />
        );
    }
}

export default File;
