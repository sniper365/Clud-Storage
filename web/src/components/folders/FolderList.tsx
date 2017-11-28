import * as React from "react";
import { Folder as FolderModel } from "../../models/Folder";
import AuthService from "../../services/Auth";
import Folder from "./Folder";
import Header from "./Header";

class FolderList extends React.Component<{ root?: number }, { root?: FolderModel, children?: FolderModel[] }> {
    constructor() {
        super();

        this.state = {
            children: undefined,
            root: undefined,
        };

        this.loadRoot = this.loadRoot.bind(this);
        this.loadChildren = this.loadChildren.bind(this);
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
        }).then( ( folder: FolderModel ) => {
            this.setState({
                root: folder,
            });

            this.loadChildren(folder.folder_id);
        });
    }

    public loadChildren( folder_id: number ) {
        const user = AuthService.getUser();

        const path = "/api/users/" + user.user_id + "/folders/" + folder_id + '/children';

        fetch(path, {
            headers: {
                'Authorization': 'Bearer ' + AuthService.getToken(),
                'Content-Type': 'application/json',
            },
        }).then((response) => {
            return response.json();
        }).then( ( folders: FolderModel[] ) => {
            this.setState({
                children: folders,
            });
        });
    }

    public getHeader() {
        if ( this.state.root === undefined ) { return; }

        if ( this.state.root.parent_id === null ) { return; }

        return (
            <Header name={this.state.root.name}/>
        );
    }

    public getRoot() {
        if ( this.state.root === undefined ) { return; }

        if ( this.state.root.parent_id === null ) { return; }

        return (
            <Folder folder_id={this.state.root.parent_id} folder_name="../" key={this.state.root.parent_id}/>
        );
    }

    public getChildren() {
        if ( this.state.children === undefined ) { return; }

        return this.state.children.map( folder =>
            <Folder folder_id={folder.folder_id} folder_name={folder.name} key={folder.folder_id}/>
        );
    }

    public render() {
        return (
            <div className="w3-quarter folder-list">
                <ul className="w3-ul">
                    {this.getRoot()}
                    {this.getChildren()}
                </ul>
            </div>
        );
    }
}

export default FolderList;
