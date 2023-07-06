import { Icon } from "@iconify/react";
import React, { useState, useEffect } from 'react';

interface UserInfo {
    username: string;
    password: string;
  }


const UserAuth: React.FC<UserInfo> = () => {
    const [userName, setUserName] = useState("");
    const [userPassword, setUserPassword] = useState("");
    const [showForm, setShowForm] = useState<boolean>(false);

    const authStart = () => {
        setShowForm(true);
    }

    const handleLogIn = async () => {
        
        const UserInfo = {
            username: userName,
            password: userPassword,
          };

        try {
            const response = await fetch(`/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(UserInfo),
            });

            if (response.ok) {
                console.log('User loged in successfully');
            } else {
                console.error('Failed to log in user');
            }
        } catch (error) {
            console.error('Network error:', error);
        }
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        handleLogIn();
        setShowForm(false);
        setUserName("");
        setUserPassword("");
    };

    return(
        <div>
            <button onClick={() => authStart() } className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                <Icon icon="basil:user-solid" className="w-7 h-8" />
                <span className="text-lg font-bold"> Log In </span>
            </button>
            {showForm && (
                <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
                <div className="bg-white rounded-lg drop-shadow-lg p-6">
                    <h2 className="text-2xl font-bold mb-4">Delete Recipe</h2>
                    <form onSubmit={handleSubmit}>
                        <div className="mb-4">
                            <label htmlFor="recipeID" className="block font-bold mb-2 text-center">
                                User Name
                            </label>
                            <input 
                                type='text' 
                                id="username"
                                value={userName} 
                                onChange={(e) => setUserName(e.target.value)} 
                                placeholder="Enter Username" 
                                className="border border-gray-300 rounded px-2 py-1 mr-2" 
                                required
                            />
                            <label htmlFor="userpassword" className="block font-bold mb-2 text-center">
                                Password
                            </label>
                            <input 
                                type='password' 
                                id="recipeID"
                                value={userPassword} 
                                onChange={(e) => setUserPassword(e.target.value)} 
                                placeholder="Enter Password" 
                                className="border border-gray-300 rounded px-2 py-1 mr-2" 
                                required
                            />
                        </div>
                        <div className="flex justify-between mr-8 ml-8">
                        <button 
                            type="submit" 
                            className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white">
                            Log In
                        </button>
                        <button 
                            type="submit" 
                            className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white">
                            Sign Up
                        </button>
                        </div>
                    </form>
                    <button className="bg-gray-300 text-gray-700 px-4 py-2 rounded-md mt-4" onClick={() => setShowForm(false)}>
          Cancel
        </button>
                </div>
            </div>
        )}
        </div>
    );
}

export default UserAuth;