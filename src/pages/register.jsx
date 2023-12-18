import { invoke } from '@tauri-apps/api';
import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';

const RegisterPage = () => {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();

    console.log('Name:', name);
    console.log('Email:', email);
    console.log('Password:', password);

    try {
      const response = await invoke('register_user', {
        name,
        email,
        password,
      });
      alert("Berhasil cuy");
      navigate("/login")
      console.log('Registration Response:', response);
      
    } catch (error) {
      console.error('Error registering user:', error);
     
    }

    setName('');
    setEmail('');
    setPassword('');
  };

  return (
    <div className="container mt-4">
      <h2>Register</h2>
      <form onSubmit={handleSubmit}>
        <div className="mb-3">
          <label htmlFor="formName" className="form-label">
            Name
          </label>
          <input
            type="text"
            className="form-control"
            id="formName"
            placeholder="Enter name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>

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
          Register
        </button>
      </form>
      <p className="mt-3">
        Already have an account?{' '}
        <Link to="/login">Login</Link>
      </p>
    </div>
  );
};

export default RegisterPage;
