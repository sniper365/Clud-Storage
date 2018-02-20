import * as React from "react";

import Nav from "./folder/Nav";
import Pane from "./file/Pane";

interface Props {
    root: number;
}

class FolderView extends React.Component<Props, {}> {
    constructor() {
        super();
    }

    public render() {
        return (
            <div className="row fill">
                <Nav root={this.props.root}/>

                <Pane root={this.props.root}/>
            </div>
        );
    }
}

export default FolderView;
