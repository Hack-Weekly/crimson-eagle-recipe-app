import { type Dispatch, type SetStateAction, createContext } from "react"

export type UserState = {
	isLoggedin: boolean,
	token: string | null,
}
type UserContextProps = {
	userState: UserState,
	setUserState: Dispatch<SetStateAction<UserState>>
}
export const UserContext = createContext<UserContextProps>({
	userState: {
		isLoggedin: false,
		token: null,
	},
	setUserState: () => null,
})