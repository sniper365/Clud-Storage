import * as React from "react";
import fileDownload from "react-file-download";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import { File as FileModel } from "../../models/File";

import { Dropdown, DropdownToggle, DropdownMenu, DropdownItem } from 'reactstrap';

class File extends React.Component<{ file: FileModel }, { is_open: boolean, download_link: string }> {
    constructor() {
        super();

        this.state = {
            is_open: false,
            download_link: "",
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
        console.log('hit');
        AuthService.user()
            .then((user) => {
                const file = this.props.file;

                const path = '/api/users/' + user.user_id + '/folders/' + file.folder_id + '/files/' + file.file_id + '/download';

                fetch(path, {
                    method: 'GET',
                    headers: {
                        'Authorization': 'Bearer ' + TokenService.getToken(),
                        'Content-Type': 'application/json'
                    },
                }).then((response) => {
                    return response.blob();
                }).then((response) => {
                    fileDownload(response, this.props.file.name + '.' + this.props.file.extension, this.props.file.extension);
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
