import * as React from "react";

import { File as FileModel } from "../../models/File";

import FileActions from "./FileActions";

class File extends React.Component<{ file: FileModel }, {}> {
    constructor() {
        super();
    }

    public render() {
        return (
            <div className="file-body container">
                <div className="row fill">
                    <div className="col-md-10 file-name m-auto">
                        { this.props.file.name}
                    </div>

                    <div className="col-md-2 p-1">
                        <FileActions file={this.props.file}/>
                    </div>
                </div>
            </div>
        );
    }
}

export default File;
