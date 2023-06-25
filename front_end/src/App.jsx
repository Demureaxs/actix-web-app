import { Component } from 'react';
import axios from 'axios';
import './App.css';
import ToDoItem from './components/ToDoItem';
import CreateToDoItem from './components/CreateToDoItem';

class App extends Component {
    state = {
        pending_items: [],
        done_items: [],
        pending_items_count: 0,
        done_items_count: 0,
    };

    // makes the API call
    getItems() {
        axios
            .get('http://127.0.0.1:8080/v1/item/get', {
                headers: { token: 'some_token' },
            })
            .then((response) => {
                let pending_items = response.data['pending_items'];
                let done_items = response.data['done_items'];

                this.setState({
                    pending_items: this.processItemValues(pending_items),
                    done_items: this.processItemValues(done_items),
                    pending_item_count: response.data['pending_item_count'],
                    done_item_count: response.data['done_item_count'],
                });
            });
        console.log(this.pending_items);
    }
    // ensures the api call is updated when mounted
    componentDidMount() {
        this.getItems();
    }

    handleReturnedState = (response) => {
        let pending_items = response.data['pending_items'];
        let done_items = response.data['done_items'];

        this.setState({
            pending_items: this.processItemValues(pending_items),
            done_items: this.processItemValues(done_items),
            pending_item_count: response.data['pending_item_count'],
            done_item_count: response.data['done_item_count'],
        });
    };

    // convert items from API to html
    processItemValues(items) {
        let itemList = [];
        items.forEach((item, _) => {
            itemList.push(
                <ToDoItem
                    key={item.title + item.status}
                    title={item.title}
                    status={item.status}
                    passBackResponse={this.handleReturnedState}
                />
            );
        });
        return itemList;
    }

    render() {
        return (
            <div className='App'>
                <div className='mainContainer'>
                    <div className='header'>
                        <p>Complete Tasks: {this.state.done_item_count}</p>
                        <p>Pending Tasks: {this.state.pending_item_count}</p>
                    </div>
                    <h1>Pending Items</h1>
                    <p>Pending Item Count: {this.state.pending_item_count}</p>
                    {this.state.pending_items}
                    <h1>Done Items</h1>
                    <p>Done Item Count: {this.state.done_item_count}</p>
                    {this.state.done_items}
                    <CreateToDoItem
                        passBackResponse={this.handleReturnedState}
                    />
                </div>
            </div>
        );
    }
}

export default App;
