import axios from 'axios';
import React, { Component } from 'react';
import '../App.css';

class CreateToDoItem extends Component {
    state = {
        title: '',
    };

    createItem = () => {
        axios
            .post(
                'http://127.0.0.1:8080/v1/item/create/' + this.state.title,
                {},
                {
                    headers: { token: localStorage.getItem('user-token') },
                }
            )
            .then((response) => {
                this.setState({ title: '' });
                this.props.passBackResponse(response);
            });
    };

    handleTitleChange = (event) => {
        this.setState({ title: event.target.value });
    };

    render() {
        return (
            <div className='inputContainer'>
                <input
                    type='text'
                    id='name'
                    placeholder='Create To Do Item'
                    value={this.state.title}
                    onChange={this.handleTitleChange}
                />
                <div
                    className='actionButton'
                    id='create-button'
                    onClick={this.createItem}
                >
                    Create
                </div>
            </div>
        );
    }
}

export default CreateToDoItem;
