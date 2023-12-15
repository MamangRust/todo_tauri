import React, { useState } from 'react';
import { Input, Button, List, Card } from 'antd';


const TodoList = () => {
    const [todos, setTodos] = useState([]);
    const [inputValue, setInputValue] = useState('');

    const handleInputChange = (e) => {
        setInputValue(e.target.value);
    };

    const handleAddTodo = () => {
        if (inputValue.trim() !== '') {
            setTodos([...todos, inputValue]);
            setInputValue('');
        }
    };

    const handleDeleteTodo = (index) => {
        const newTodos = todos.filter((_, i) => i !== index);
        setTodos(newTodos);
    };

    return (
        <div style={{ width: "500px", height: "500px", margin: 'auto', marginTop: 50 }}>
            <Card className="home-card">

                <h1 className="home-card-title">Todo List</h1>
                <Input
                    placeholder="Add a todo"
                    value={inputValue}
                    onChange={handleInputChange}
                    style={{ marginBottom: 10 }}
                />
                <Button type="primary" onClick={handleAddTodo}>
                    Add Todo
                </Button>
                <List
                    style={{ marginTop: 20 }}
                    bordered
                    dataSource={todos}
                    renderItem={(item, index) => (
                        <List.Item
                            actions={[
                                <Button type="link" onClick={() => handleDeleteTodo(index)}>
                                    Delete
                                </Button>,
                            ]}
                        >
                            {item}
                        </List.Item>
                    )}
                />
            </Card>
        </div>
    );
};

export default TodoList;
