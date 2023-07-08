import { Icon } from "@iconify/react";
import React, { useState, useEffect } from 'react';
import DeleteButton from "@/components/DeleteRecipe";
import AddRecipe from "@/components/AddRecipe";

interface UserInfo {
    username: string;
    password: string;
  }


const UserAuth: React.FC<UserInfo> = () => {
    const [userName, setUserName] = useState("");
    const [userPassword, setUserPassword] = useState("");
    const [showForm, setShowForm] = useState<boolean>(false);
    const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);

    const authStart = () => {
        setShowForm(true);
    }

    const logInUser = async () => {
        
        const UserInfo = {
            username: userName,
            password: userPassword,
          };

        try {
            const response = await fetch(`https://crimson-eagles-recipe-app.onrender.com/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(UserInfo),
            });

            if (response.ok) {
                console.log('User loged in successfully');
                const { jwt_token } = await response.json()
                localStorage.setItem('jwtToken', jwt_token);
                setIsLoggedIn(true);
            } else {
                console.error('Failed to log in user');
            }
        } catch (error) {
            console.error('Network error:', error);
        }
    };

    const registerUser = async () => {
        
        const UserInfo = {
            username: userName,
            password: userPassword,
          };

        try {
            const response = await fetch(`https://crimson-eagles-recipe-app.onrender.com/register`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(UserInfo),
            });

            if (response.ok) {
                console.log('User loged in successfully');
                const { jwt_token } = await response.json()
                localStorage.setItem('jwtToken', jwt_token);
            } else {
                console.error('Failed to log in user');
            }
        } catch (error) {
            console.error('Network error:', error);
        }
    };

    const logOutUser = () => {
        localStorage.removeItem('jwtToken');
        setIsLoggedIn(false);
        console.log('Token cleared, user logged out');
    }

    const handleLogIn = () => {
        logInUser();
        setShowForm(false);
        setUserName("");
        setUserPassword("");
    };

    const handleRegister = () => {
        registerUser();
        setShowForm(false);
        setUserName("");
        setUserPassword("");
    };
    //isLoggedIn should be adapted to show extra buttons based on if user is logged in.
    return(
        <div>
            {isLoggedIn ? (
                <button onClick={() => logOutUser()} className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:user-solid" className="w-7 h-8" />
                    <span className="text-lg font-bold"> Log Out </span>
                </button>
                ):(
                <button onClick={() => authStart() } className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:user-solid" className="w-7 h-8" />
                    <span className="text-lg font-bold"> Log In </span>
                </button>)}
            {showForm && (
                <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
                <div className="bg-white rounded-lg drop-shadow-lg p-6">
                    <h2 className="text-2xl font-bold mb-4">Delete Recipe</h2>
                    <form>
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
                            type="button" 
                            className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white"
                            onClick={() => handleLogIn()}>
                            Log In
                        </button>
                        <button 
                            type="button" 
                            className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white"
                            onClick={() => handleRegister()}>
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