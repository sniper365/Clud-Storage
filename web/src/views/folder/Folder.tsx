import * as React from "react";

import { Link } from "react-router-dom";

import { Folder as FolderModel } from "../../models/Folder";

class Folder extends React.Component<{ folder: FolderModel }, {}> {
    constructor() {
        super();
    }

    public render() {
        return (
            <Link to={"/folders/" + this.props.folder.folder_id} className="list-group-item folder">
                <img className="" src={require('../../icons/ic_folder_black_24px.svg')}/>&nbsp;&nbsp;{this.props.folder.name}
            </Link>
        );
    }
}

export default Folder;
