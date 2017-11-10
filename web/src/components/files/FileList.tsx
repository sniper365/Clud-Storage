import * as React from "react";
import AuthService from "../../services/Auth";

class FileList extends React.Component<{ root?: number }, { }> {
    constructor() {
        super();

        this.state = {
            root: undefined,
        };

        this.loadRoot = this.loadRoot.bind(this);
    }

    public componentDidMount() {
        this.loadRoot();
    }

    public loadRoot() {
        const root = this.props.root ? this.props.root : undefined;

        const user = AuthService.getUser();

        const path = ( root === undefined )
            ? "/api/users/" + user.user_id + "/root"
            : "/api/users/" + user.user_id + "/folders/" + root;

        return fetch(path, {
            headers: {
                'Authorization': 'Bearer ' + AuthService.getToken(),
                'Content-Type': 'application/json',
            },
        }).then((response) => {
            return response.json();
        });
    }

    public render() {
        return (
            <div/>
        );
    }
}

export default FileList;
