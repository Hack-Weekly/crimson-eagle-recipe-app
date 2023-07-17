
import { UserContext } from "@/context/user-state";
import { Icon } from "@iconify/react";
import React, { useState, useContext } from 'react';
import PasswordChecklist from "react-password-checklist"

export const getJwtToken = (): string => {
    let jwtToken = localStorage.getItem('jwtToken') || "";
    return jwtToken;
};

const UserAuth: React.FC = () => {
    const [userName, setUserName] = useState("");
    const [userPassword, setUserPassword] = useState("");
    const [showForm, setShowForm] = useState<boolean>(false);
	const { userState, setUserState } = useContext(UserContext);
    const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);
    const [error, setError] = useState<string>("");

    const makeRequest = async (endpoint: string, UserInfo: {username: string, password: string}) => {
        try {
            const response = await fetch(`https://crimson-eagles-recipe-app.onrender.com/${endpoint}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(UserInfo),
            });
            return response;
        } catch (error) {
            setError('Network error: ' + error);
        }
    };
    
    const logInUser = async () => {
        setError("");
        const UserInfo = {
            username: userName,
            password: userPassword,
        };
        const response = await makeRequest('login', UserInfo);
    
        if (response && response.ok) {
            console.log('User logged in successfully');
            const { AuthToken } = await response.json();
            localStorage.setItem('jwtToken', AuthToken);
            setIsLoggedIn(true);
            setShowForm(false);
            setUserState({
                isLoggedin: true,
                token: AuthToken,
            })
          } else if (response) {
            const errorMessage = await response.text();
            setError(errorMessage);
          }
    };
    
    const registerUser = async () => {
        setError("");
        const UserInfo = {
            username: userName,
            password: userPassword,
        };
        const response = await makeRequest('register', UserInfo);
    
        // TODO you're not getting logged in in the backend
        if (response && response.ok) {
            console.log('User registered successfully');
            //setIsLoggedIn(true);
            setShowForm(false);
        } else if (response) {
            const errorMessage = await response.text();
            setError(errorMessage);
        }
    };
    

    const logOutUser = () => {
        localStorage.removeItem('jwtToken');
        setIsLoggedIn(false);
        setUserState({
            isLoggedin: false,
            token: null,
        })
        console.log('Token cleared, user logged out');
    };

    const openForm = () => {
        setShowForm(true);
        setUserName("");
        setUserPassword("");
        setError("");
    }

    // Render error message if it exists
    const renderError = error ? <div className="text-red-500 mt-2">{error}</div> : null;

    return (
        <div>
            {userState.isLoggedin || isLoggedIn ? (
                <button onClick={logOutUser} className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:user-solid" className="w-7 h-8" />
                    <span className="text-lg font-serif-extrabold"> Log Out </span>
                </button>
            ) : (
                <button onClick={openForm} className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:user-solid" className="w-7 h-8" />
                    <span className="text-lg font-serif-extrabold"> Log In </span>
                </button>
            )}

            {showForm && (
                <div className="fixed z-10 top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
                    <div className="bg-white rounded-lg drop-shadow-lg p-6">
                        <h2 className="text-2xl font-bold mb-4">Welcome to Foodly!</h2>
                        <form>
                            <div className="mb-4">
                                <label htmlFor="username" className="block font-bold mb-2">
                                    Username
                                </label>
                                <input
                                    type="text"
                                    id="username"
                                    value={userName}
                                    onChange={(e) => setUserName(e.target.value)}
                                    placeholder="Enter Username"
                                    className="border border-gray-300 rounded px-2 py-1 w-full"
                                    required
                                />
                                <label htmlFor="password" className="block font-bold mt-4 mb-2">
                                    Password
                                </label>
                                <input
                                    type="password"
                                    id="password"
                                    value={userPassword}
                                    onChange={(e) => setUserPassword(e.target.value)}
                                    placeholder="Enter Password"
                                    className="border border-gray-300 rounded px-2 py-1 w-full mb-4"
                                    required
                                />
                                <PasswordChecklist
                                    rules={["minLength","specialChar","number","capital"]}
                                    minLength={5}
                                    value={userPassword}
                                    onChange={(_isValid: any) => {}}
                                />
                                {renderError}
                            </div>
                            <div className="flex justify-start">
                                <button 
                                    type="button"
                                    className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white m-1"
                                    onClick={logInUser}>
                                    Log In
                                </button>
                                <button 
                                    type="button"
                                    className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white m-1"
                                    onClick={registerUser}>
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