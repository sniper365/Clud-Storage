import * as React from "react";
import AuthService from "../../services/Auth";
import { File as FileModel } from "../../models/File";
// import FileDrop from "react-file-drop";
import File from "./File";

class FileList extends React.Component<{ root: number }, { root: number, files: FileModel[] }> {
    constructor(root) {
        super();

        this.state = {
            root: root,
            files: [],
        };

        this.loadFiles = this.loadFiles.bind(this);
    }

    public componentDidMount() {
        this.loadFiles();
    }

    public loadFiles() {
        const user = AuthService.getUser();

        const path = "/api/users/" + user.user_id + "/folders/" + this.props.root + "/files";

        return fetch(path, {
            headers: {
                'Authorization': 'Bearer ' + AuthService.getToken(),
                'Content-Type': 'application/json',
            },
        }).then((response) => {
            return response.json();
        }).then( ( files: FileModel[] ) => {
            this.setState({
                files: files,
            });
        });
    }

    public getFiles() {
        return this.state.files.map( file =>
            <File file_id={file.file_id} file={file} key={file.file_id}/>
        );
    }

    public render() {

        return (
            <div className="file-list w3-rest">
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                {this.getFiles()}
                
            </div>
        );
    }
}

export default FileList;
