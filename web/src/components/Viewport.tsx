import * as React from "react";
import FolderList from "./FolderList";

class Viewport extends React.Component {
    constructor() {
        super();
    }

    public render() {
        return (
            <div id="viewport">
                <FolderList />
            </div>
        );
    }
}

export default Viewport;
