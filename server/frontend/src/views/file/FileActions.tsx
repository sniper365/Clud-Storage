import * as React from "react";
import fileDownload from "react-file-download";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { File as FileModel } from "../../models/File";

import { Dropdown, DropdownItem, DropdownMenu, DropdownToggle } from 'reactstrap';

interface Props {
    file: FileModel;
    on_error?: (error: ErrorModel) => void;
    on_download?: (file: FileModel) => void;
}

interface State {
    download_link: string;
    is_open: boolean;
}

class File extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            download_link: "",
            is_open: false,
        };

        this.toggle = this.toggle.bind(this);
        this.download = this.download.bind(this);
    }

    public toggle() {
        this.setState({
            is_open: !this.state.is_open
        });
    }

    public download() {
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
                    if (response.ok) {
                        return response.blob();
                    }

                    return response.json();
                }).then((response) => {
                    if (response instanceof Object && response.status_code >= 400) {
                        if (this.props.on_error) { this.props.on_error(response); }
                    } else {
                        const file_name = this.props.file.name + '.' + this.props.file.extension;
                        fileDownload(response, file_name, this.props.file.extension);

                        if (this.props.on_download) { this.props.on_download(response); }
                    }
                });
            });
    }

    public render() {
        return (
            <Dropdown className="file-actions" isOpen={this.state.is_open} toggle={this.toggle}>
                <DropdownToggle className="s-dropdown-toggle button">
                    <img src={require('../../icons/ic_more_vert_black_24px.svg')}/>
                </DropdownToggle>

                <DropdownMenu className="s-dropdown-menu">
                    <DropdownItem className="s-dropdown-item" onClick={this.download}>
                        Download
                    </DropdownItem>
                </DropdownMenu>
            </Dropdown>
        );
    }
}

export default File;
