import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { useNavigate } from 'react-router-dom';

const LoginPage = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const navigate = useNavigate();

    const handleSubmit = async (e) => {
        e.preventDefault();
    
        try {
            const data = await invoke('login_user', {
                email,
                password,
            });
    
           
            alert("Response from Login: " + data);
    
          
    
            navigate("/todo");
        } catch (error) {
            alert("Error logging in: " + error);
        }
    
        setEmail('');
        setPassword('');
    };
    

    return (
        <div className="container mt-4">
            <h2>Login</h2>
            <form onSubmit={handleSubmit}>
                <div className="mb-3">
                    <label htmlFor="formEmail" className="form-label">
                        Email address
                    </label>
                    <input
                        type="email"
                        className="form-control"
                        id="formEmail"
                        placeholder="Enter email"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </div>

                <div className="mb-3">
                    <label htmlFor="formPassword" className="form-label">
                        Password
                    </label>
                    <input
                        type="password"
                        className="form-control"
                        id="formPassword"
                        placeholder="Password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </div>

                <button type="submit" className="btn btn-primary">
                    Login
                </button>
            </form>
        </div>
    );
};

export default LoginPage;
