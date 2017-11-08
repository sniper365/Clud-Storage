import * as React from "react";
import { Link } from "react-router-dom";
import { Folder as FolderModel } from "../models/Folder";
import Folder from "./Folder";

class FolderList extends React.Component<{ root?: FolderModel }, { folders: FolderModel[] }> {
    constructor() {
        super();

        this.state = {
            folders: [],
        };
    }

    public getRoot() {
        if ( this.props.root !== undefined && this.props.root.parent_id ) {
            return (
                <Link to={"/folders/" + this.props.root.parent_id}>
                    <Folder folder_id={1} folder_name="../"/>
                </Link>
            );
        }

        return;
    }

    public render() {
        return (
            <ul className="w3-ul folder-list">
                {this.getRoot()}
            </ul>
        );
    }
}

export default FolderList;
