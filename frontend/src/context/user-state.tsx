"use client"

import {
	type ReactNode,
	createContext,
	useEffect,
	useState,
	useMemo,
	useContext
} from "react"

export type UserState = {
	isLoading: boolean,
	isLoggedIn: boolean,
	token: string | null,
}
type User = {
	id: number,
	username: string,
	password: string,
}
type Auth = {
	AuthToken: string,
}
type UserContextProps = {
	userState: UserState,
	register: (username: string, password: string) => Promise<User>,
	login: (username: string, password: string) => Promise<void>,
	logout: () => void,
	//setUserState: Dispatch<SetStateAction<UserState>>
}
export const UserContext = createContext<UserContextProps>({
	userState: {
		isLoading: true,
		isLoggedIn: false,
		token: null,
	},
	register: (username: string, password: string) => Promise.reject("Not implemented."),
	login: (username: string, password: string) => Promise.reject("Not implemented."),
	logout: () => {},
})

export const UserContextProvider = ({ children }: { children: ReactNode }) => {
	const [ userState, setUserState] = useState<UserState>({
		isLoading: true,
		isLoggedIn: false,
		token: null,
	})

	useEffect(() => {
		const token = localStorage.getItem('jwtToken')
		
		if (token) {
			fetch(`${ process.env.apiUrl }/profile`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Authorization': `Bearer ${ token }`,
				},
			})
			.then(res => {
				if (!res.ok) {
					if (res.status == 401) {
						localStorage.removeItem('jwtToken')
						throw new Error("Authorization expired.")
					}
					throw new Error("Status not 200")
				}
				else return res.json()
			})
			.then(res => {
				if (res.username) {
					setUserState(prev => ({
						...prev,
						isLoggedIn: true,
						token: token,
					}))
				}
			})
			.catch(console.log) // most likely the token is not valid anymore
		}
		setUserState(prev => ({
			...prev,
			isLoading: false
		}))
	}, [])

	const register = async (username: string, password: string): Promise<User> => {
		setUserState(prev => ({
			...prev,
			isLoading: true,
		}))

		return fetch(`${ process.env.apiUrl }/register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username,
                password,
            }),
        })
		.then(res => res.json())
		.finally(() => {
			setUserState(prev => ({
				...prev,
				isLoading: false,
			}))
		})
	}

	const login = async (username: string, password: string) => {
		setUserState(prev => ({
			...prev,
			isLoading: true,
		}))

		return fetch(`${ process.env.apiUrl }/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username,
                password,
            }),
        })
		.then(res => res.json())
		.then((res: Auth) => {
			setUserState(prev => ({
				...prev,
				isLoggedIn: true,
				token: res.AuthToken,
			}))
			localStorage.setItem('jwtToken', res.AuthToken)
		})
		.finally(() => {
			setUserState(prev => ({
				...prev,
				isLoading: false,
			}))
		})
	}

	const logout = () => {
        localStorage.removeItem('jwtToken')
		setUserState({
			isLoading: false,
			isLoggedIn: false,
			token: null,
		})
	}

	const memoedValue = useMemo(() => ({
		userState,
		register,
		login,
		logout,
	}), [userState])

	return (
		<UserContext.Provider value={ memoedValue }>
			{ children }
		</UserContext.Provider>
	)
}

export const useUserContext = () => useContext(UserContext)