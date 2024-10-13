// src/pages/Account.jsx
import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import GalaxyVisualization from '../components/GalaxyVisualisation.jsx';

function Account() {
    const navigate = useNavigate();
    const [user, setUser] = useState(null);

    useEffect(() => {
        const token = localStorage.getItem('token');
        if (!token) {
            navigate('/');
        } else {
            fetchAccountInfo();
        }
    }, []);

    const fetchAccountInfo = () => {
        const token = localStorage.getItem('token');
        fetch('http://localhost:8080/account', {
            headers: {
                Authorization: 'Bearer ' + token,
            },
        })
            .then((response) => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then((user) => setUser(user))
            .catch((error) => {
                console.error('Error:', error);
                logout();
            });
    };

    const logout = () => {
        localStorage.removeItem('token');
        navigate('/');
    };

    const formatDate = (dateString) => {
        if (!dateString) return 'N/A';
        const date = new Date(dateString);
        return date.toLocaleString();
    };

    return (
        <div className="min-h-screen bg-gradient-to-b from-gray-900 via-black to-gray-900 text-white p-8 flex flex-col items-center">
            <h1 className="text-4xl font-extrabold mb-6 text-center">Proxima Coloniae - Account</h1>

            <div className="max-w-3xl w-full space-y-8">
                {/* Account Information */}
                <div className="bg-gray-800 bg-opacity-75 p-6 rounded-lg shadow-lg">
                    <h2 className="text-2xl font-semibold mb-4">Account Information</h2>
                    <div id="account-details">
                        {user ? (
                            <div className="space-y-2">
                                <p><strong>Username:</strong> {user.username}</p>
                                <p><strong>Email:</strong> {user.email}</p>
                                <p><strong>Created At:</strong> {formatDate(user.created_at)}</p>
                                <p><strong>Last Login:</strong> {formatDate(user.last_login)}</p>
                            </div>
                        ) : (
                            <p>Loading account information...</p>
                        )}
                    </div>
                </div>

                {/* Galaxy Visualization */}
                <div className="bg-gray-800 bg-opacity-75 p-6 rounded-lg shadow-lg">
                    <h2 className="text-2xl font-semibold mb-4">Galaxy Visualization</h2>
                    <GalaxyVisualization />
                </div>
            </div>

            <button
                onClick={logout}
                className="mt-8 bg-red-600 hover:bg-red-700 text-white font-bold py-3 px-6 rounded shadow-lg transition-transform transform hover:scale-105 focus:outline-none focus:ring-4 focus:ring-red-300"
            >
                Logout
            </button>
        </div>
    );
}

export default Account;
