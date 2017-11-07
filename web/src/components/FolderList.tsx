import * as React from "react";
import Folder from "./Folder";

class FolderList extends React.Component<{ }, { root: number, folders: Folder[] }> {
    constructor(root: number) {
        super();

        this.state = {
            folders: Array<Folder>(),
            root,
        };

    }

    public render() {
        const folders = [
            new Folder(1, "test"),
            new Folder(2, "test1"),
            new Folder(3, "test2"),
            new Folder(4, "test3"),
        ];

        return (
            <ul className="w3-ul folder-list">
                {folders.map( ( folder ) => {
                    return folder.render();
                })}
            </ul>
        );
    }
}

export default FolderList;
