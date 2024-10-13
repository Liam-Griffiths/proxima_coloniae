// src/pages/Home.jsx
import React, { useContext } from 'react';
import { AuthContext } from '../context/AuthContext.jsx';
import { useNavigate } from "react-router-dom";

function Home() {
    const { login } = useContext(AuthContext);
    const navigate = useNavigate();

    const handleLogin = (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);

        fetch('http://localhost:8080/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                email: formData.get('email'),
                password: formData.get('password'),
            }),
        })
            .then((response) => response.json())
            .then((data) => {
                if (data.token) {
                    login(data.token);
                    navigate('/account');
                } else {
                    // Handle login error
                }
            })
            .catch((error) => {
                console.error('Error logging in:', error);
            });
    };

    const handleRegister = (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);

        fetch('http://localhost:8080/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username: formData.get('username'),
                email: formData.get('email'),
                password: formData.get('password'),
            }),
        })
            .then((response) => {
                if (!response.ok) {
                    // Handle HTTP errors
                    return response.json().then((errorData) => {
                        throw new Error(errorData.message || 'Registration failed');
                    });
                }
                return response.json();
            })
            .then((data) => {
                if (data.token) {
                    // Automatically log in the user
                    login(data.token);
                    navigate('/account');
                } else {
                    // Handle registration error
                    console.error('Registration failed:', data.message);
                }
            })
            .catch((error) => {
                console.error('Error registering:', error.message);
            });
    };

    return (
        <div className="min-h-screen bg-gradient-to-b from-gray-900 via-black to-gray-900 text-white flex items-center justify-center p-4">
            <div className="max-w-4xl w-full">
                <h1 className="text-5xl font-extrabold mb-4 text-center">Proxima Coloniae</h1>
                <p className="text-xl mb-8 text-center">A space strategy game.</p>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                    {/* Registration Form */}
                    <div className="bg-gray-800 bg-opacity-75 p-6 rounded-lg shadow-lg">
                        <h2 className="text-2xl font-semibold mb-4">Register</h2>
                        <form onSubmit={handleRegister} className="space-y-4">
                            <input
                                type="text"
                                name="username"
                                placeholder="Username"
                                className="w-full p-3 bg-gray-700 text-white border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                                required
                            />
                            <input
                                type="email"
                                name="email"
                                placeholder="Email"
                                className="w-full p-3 bg-gray-700 text-white border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                                required
                            />
                            <input
                                type="password"
                                name="password"
                                placeholder="Password"
                                className="w-full p-3 bg-gray-700 text-white border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                                required
                            />
                            <button
                                type="submit"
                                className="w-full p-3 bg-blue-600 text-white rounded hover:bg-blue-700 transition duration-200 focus:outline-none focus:ring-4 focus:ring-blue-300"
                            >
                                Register
                            </button>
                        </form>
                    </div>

                    {/* Login Form */}
                    <div className="bg-gray-800 bg-opacity-75 p-6 rounded-lg shadow-lg">
                        <h2 className="text-2xl font-semibold mb-4">Login</h2>
                        <form onSubmit={handleLogin} className="space-y-4">
                            <input
                                type="email"
                                name="email"
                                placeholder="Email"
                                className="w-full p-3 bg-gray-700 text-white border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
                                required
                            />
                            <input
                                type="password"
                                name="password"
                                placeholder="Password"
                                className="w-full p-3 bg-gray-700 text-white border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
                                required
                            />
                            <button
                                type="submit"
                                className="w-full p-3 bg-green-600 text-white rounded hover:bg-green-700 transition duration-200 focus:outline-none focus:ring-4 focus:ring-green-300"
                            >
                                Login
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Home;
