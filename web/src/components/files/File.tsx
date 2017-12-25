import * as React from "react";
import { File as FileModel } from "../../models/File";

class File extends React.Component<{ file_id: number, file: FileModel, key: number }, { }> {
    constructor( ) {
        super();

        this.getHeader = this.getHeader.bind(this);
    }

    public getHeader() {
        switch(this.props.file.extension) {
            case 'jpg':

        }
    }

    public render() {
        return (
            <div className="file w3-blue-gray w3-card-2">
                <div className="file-header" data-file-id={this.props.file_id}>
                    {this.getHeader()}
                    .{this.props.file.extension}
                </div>

                <div className="file-body w3-row">
                    <div className="w3-col s7 file-name">
                        {this.props.file.name}
                    </div>
                    <div className="w3-col s5 file-action">
                        <button className="w3-btn w3-ripple download-btn">Download</button>
                    </div>
                </div>
            </div>
        );
    }
}

export default File;
