import * as React from "react";

import { Col, Modal } from 'reactstrap';

import NewFolderForm from "./NewFolderForm";

class NewFolderButton extends React.Component< {
            root: number,
            onClick?: Function,
            onSave?: Function,
        }, {
            modal: boolean
        }> {

    constructor() {
        super();

        this.state = {
            modal: false,
        };

        this.show_modal = this.show_modal.bind(this);
        this.on_click = this.on_click.bind(this);
        this.on_save = this.on_save.bind(this);
    }

    public on_click() {
        this.props.onClick && this.props.onClick();

        this.show_modal();
    }

    public on_save() {
        this.show_modal();

        this.props.onSave && this.props.onSave();
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
                    <NewFolderForm root={this.props.root} onSave={this.on_save}/>
                </Modal>
            </Col>
        );
    }
}

export default NewFolderButton;
