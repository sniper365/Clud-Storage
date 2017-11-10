import * as React from "react";
import { Link } from "react-router-dom";

class Folder extends React.Component<{ folder_id: number, folder_name: string, key: number }, { }> {
    constructor( ) {
        super();
    }

    public render() {
        return (
            <li className="folder">
                <Link to={"/folders/" + this.props.folder_id}>
                    {this.props.folder_name}
                </Link>
            </li>
        );
    }
}

export default Folder;
