
import React, { FormEvent, useState } from 'react'
import PasswordChecklist from "react-password-checklist"
import { Icon } from "@iconify/react"
import { useUserContext } from "@/context/user-state"

const UserAuthWithContext: React.FC = () => {
    const [userName, setUserName] = useState("")
    const [userPassword, setUserPassword] = useState("")
    const [showForm, setShowForm] = useState<boolean>(false)
    const [error, setError] = useState<string>("")
    const [submitType, setSubmitType] = useState<"login" | 'register'>("login")

	const { userState, register, login, logout } = useUserContext()
    
    const onLoginSubmit = (e: FormEvent) => {
        e.preventDefault()

        login(userName, userPassword)
            .then(_ => setShowForm(false))
            .catch(setError)
    }
    
    const onRegisterSubmit = (e: FormEvent) => {
        e.preventDefault()

        register(userName, userPassword)
            // log in at once
            .then(_ => login(userName, userPassword))
            .then(_ => setShowForm(false))
            .catch(setError)
    }

    const onGuestLogin = () => {
        login("Guest", "p4ssW0Rd!")
            .then(_ => setShowForm(false))
            .catch(setError)
    }

    const openForm = () => {
        setShowForm(true)
        setUserName("")
        setUserPassword("")
        setError("")
    }

    // Render error message if it exists
    const renderError = error ? <div className="text-red-500 mt-2">{error}</div> : null;

    return (
        <div>
            { userState.isLoggedIn ? (
                <button onClick={ logout } className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:user-solid" className="w-7 h-8" />
                    <span className="text-lg font-serif-extrabold"> Log Out </span>
                </button>
            ) : (
                <button onClick={ openForm } className="flex justify-center items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:user-solid" className="w-7 h-8" />
                    <span className="text-lg font-serif-extrabold"> Log In </span>
                </button>
            )}

            { showForm && (
                <div className="fixed z-10 top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
                    <div className="relative bg-white rounded-lg drop-shadow-lg p-6">                        
                        <button onClick={() => setShowForm(false)}
                            className="absolute top-3 right-4 p-1 text-xl text-gray-700 hover:text-gray-800">
                            &times;
                        </button>
                        <h2 className="text-2xl font-bold mb-2">Welcome to Foodly!</h2>
                        <form onSubmit={ submitType == 'login' ? onLoginSubmit : onRegisterSubmit }>
                            <div className="mb-4">
                                <label htmlFor="username" className="block font-bold mb-2">
                                    Username
                                </label>
                                <input
                                    type="text"
                                    id="username"
                                    value={ userName }
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
                                    value={ userPassword }
                                    onChange={(e) => setUserPassword(e.target.value)}
                                    placeholder="Enter Password"
                                    className="border border-gray-300 rounded px-2 py-1 w-full mb-4"
                                    required
                                />
                                <PasswordChecklist
                                    rules={["minLength","specialChar","number","capital"]}
                                    minLength={5}
                                    value={ userPassword }
                                    onChange={(_isValid: any) => {}}
                                />
                                { renderError }
                            </div>
                            <div className="flex justify-start items-center">
                                <button type="submit"
                                    className="px-4 py-2 mr-2 bg-red-500 rounded-2xl text-white">
                                    { submitType == 'login' ? 'Log In' : 'Sign up'}
                                </button>
                                { submitType == 'login' ? (
                                    <p>No account yet? <button type="button" className="p-1 underline"
                                        onClick={ () => setSubmitType("register")}>Register</button></p>
                                ) : (
                                    <p>Already registered? <button type="button" className="p-1 underline"
                                    onClick={ () => setSubmitType("login")}>Log in</button></p>
                                )}
                            </div>
                        </form>
                        <div className="relative flex flex-col justify-center items-center">
                            <hr className="w-full h-px my-8 bg-gray-300 border-0" />
                            <span className="absolute top-2 p-3 text-gray-400 bg-white">OR</span>
                            <button type="button" onClick={ onGuestLogin }
                                className="px-4 py-2 bg-red-500 rounded-2xl text-white">
                                Log in as Guest</button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    )
}

export default UserAuthWithContext