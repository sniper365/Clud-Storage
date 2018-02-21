import * as React from "react";

import { Col, Modal } from 'reactstrap';

import NewFolderForm from "./NewFolderForm";

import ErrorModel from "../../models/Error";
import { Folder as FolderModel } from "../../models/Folder";

interface Props {
    root: number;
    on_click?: () => void;
    on_save?: (response: FolderModel) => void;
    on_error?: (error: ErrorModel) => void;
}

interface State {
    modal: boolean;
}

class NewFolderButton extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            modal: false,
        };

        this.show_modal = this.show_modal.bind(this);
        this.on_click = this.on_click.bind(this);
        this.on_save = this.on_save.bind(this);
        this.on_error = this.on_error.bind(this);
    }

    public on_click() {
        if (this.props.on_click) { this.props.on_click(); }

        this.show_modal();
    }

    public on_save(response: FolderModel) {
        this.show_modal();

        if (this.props.on_save) { this.props.on_save(response); }
    }

    public on_error(response: ErrorModel) {
        this.show_modal();

        if (this.props.on_error) { this.props.on_error(response); }
    }

    public show_modal() {
        this.setState({
            modal: !this.state.modal,
        });
    }

    public render() {
        return (
            <Col md={4} className="action p-2" onClick={this.on_click}>
                <img className="" src={require('../../icons/ic_create_new_folder_black_24px.svg')}/>

                <Modal isOpen={this.state.modal} toggle={this.show_modal}>
                    <NewFolderForm root={this.props.root} on_save={this.on_save} on_error={this.on_error}/>
                </Modal>
            </Col>
        );
    }
}

export default NewFolderButton;
