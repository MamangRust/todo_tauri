import { createSlice } from '@reduxjs/toolkit';

const initialState = {
    todos: [],
};

export const todoSlice = createSlice({
    name: 'todos',
    initialState,
    reducers: {
        setTodos: (state, action) => {
            state.todos = action.payload;
        },
        addTodo: (state, action) => {
            state.todos.push(action.payload);
        },
        updateTodo: (state, action) => {
            const { id, completed } = action.payload;
            state.todos = state.todos.map(todo =>
                todo.id === id ? { ...todo, completed } : todo
            );
        },
        removeTodo: (state, action) => {
            const idToRemove = action.payload;
            state.todos = state.todos.filter(todo => todo.id !== idToRemove);
        },
    },
});

export const { setTodos, addTodo, updateTodo, removeTodo } = todoSlice.actions;

export default todoSlice.reducer;
