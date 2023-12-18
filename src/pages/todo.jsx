import React, { useState, useEffect } from 'react';
import { setTodos, addTodo, updateTodo, removeTodo } from '../store/todo';
import { invoke } from '@tauri-apps/api/tauri';
import { useDispatch, useSelector } from 'react-redux';

function TodoList() {
    const dispatch = useDispatch();
    const todos = useSelector((state) => state.todos.todos);
    const [newTodo, setNewTodo] = useState('');

    useEffect(() => {
        loadTodos();
    }, []);

    const loadTodos = async () => {
        try {
            const todos = await invoke('get_todos');
            dispatch(setTodos(todos));
        } catch (error) {
            console.error('Error fetching todos:', error);
        }
    };

    const handleAddTodo = async () => {
        try {
            await invoke('add_todo', { title: newTodo, completed: false });
            dispatch(addTodo({ title: newTodo, completed: false }));
            setNewTodo('');

            loadTodos();
        } catch (error) {
            console.error('Error adding todo:', error);
        }
    };

    const handleUpdateTodo = async (id, completed) => {
        try {
            await invoke('update_todo', { id, completed });
            dispatch(updateTodo({ id, completed }));
            loadTodos();
        } catch (error) {
            console.error('Error updating todo:', error);
        }
    };

    const handleRemoveTodo = async (id) => {
        try {
            await invoke('remove_todo', { id });
            dispatch(removeTodo(id)); 
            loadTodos();
        } catch (error) {
            console.error('Error removing todo:', error);
        }
    };

    return (
        <div className="container mt-5">
            <h1>Todo List</h1>
            <div className="input-group mb-3">
                <input
                    type="text"
                    className="form-control"
                    placeholder="Add new todo"
                    value={newTodo}
                    onChange={(e) => setNewTodo(e.target.value)}
                />
                <button className="btn btn-primary" onClick={handleAddTodo}>
                    Add
                </button>
            </div>
            <ul className="list-group">
                {todos.map((todo) => (
                    <li key={todo.id} className="list-group-item d-flex justify-content-between align-items-center">
                        <div>
                            <input
                                type="checkbox"
                                checked={todo.completed}
                                onChange={(e) => handleUpdateTodo(todo.id, e.target.checked)}
                                className="form-check-input me-2"
                            />
                            {todo.title}
                        </div>
                        <button className="btn btn-danger" onClick={() => handleRemoveTodo(todo.id)}>
                            Remove
                        </button>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default TodoList;
