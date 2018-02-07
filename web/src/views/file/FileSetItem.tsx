import * as React from "react";

import { File as FileModel } from "../../models/File";

import FileHeader from "./FileHeader";
import FileBody from "./FileBody";

class File extends React.Component<{ file: FileModel }, { background: string }> {
    constructor() {
        super();
    }

    public render() {
        const tooltip = this.props.file.name + '.' + this.props.file.extension;

        return (
            <div className="file fade-in" data-toggle="tooltip" data-placement="bottom" title={tooltip}>
                <FileHeader file={this.props.file}/>

                <FileBody file={this.props.file}/>
            </div>
        );
    }
}

export default File;
