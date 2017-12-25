import * as React from "react";
import { Route, Switch } from "react-router-dom";

import Folder from "../models/Folder";
import FileList from "./files/FileList";
import FolderList from "./folders/FolderList";

class Viewport extends React.Component<{}, { loaded: boolean, root?: Folder }> {
    constructor() {
        super();

        this.state = {
            loaded: false,
            root: undefined,
        };
    }

    public render() {
        return (
            <Switch>
                <Route path="/folders/:folder_id" component={({ match }) => (
                    <div id="viewport">
                        <FolderList root={match.params.folder_id}/>

                        <FileList root={match.params.folder_id}/>
                    </div>
                )}/>
            </Switch>
        );
    }
}

export default Viewport;
