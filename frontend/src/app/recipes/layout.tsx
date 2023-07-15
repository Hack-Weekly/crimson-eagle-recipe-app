"use client"

import { type Dispatch, type SetStateAction, createContext, useEffect, useState } from "react"
import Link from "next/link";
import { Icon } from "@iconify/react"
import AddRecipe from "@/components/AddRecipe"
import DeleteButton from "@/components/DeleteRecipe"
import SearchBar from "@/components/SearchBar"
import UserAuth from "@/components/UserAuth"

type UserState = {
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

export default function RecipeLayout({
  children,
}: {
  children: React.ReactNode;
}) {
	const [userState, setUserState] = useState<UserState>({
		isLoggedin: false,
		token: null,
	})

	useEffect(() => {
		const token = localStorage.getItem('jwtToken')
		setUserState({
			isLoggedin: token ? true : false,
			token: token ? token : null,
		})
	}, [])

	return (
		<UserContext.Provider value={{ userState, setUserState }}>
		<div className="h-full w-full max-w-[1920px] mx-auto flex flex-col items-center justify-center py-10">
			<div className="w-full flex justify-end items-center gap-12 pr-10 mb-8 ml-8">
				<h1 className="text-6xl font-extrabold">Foodly</h1>
				<SearchBar onSearch={ function (): void {
					throw new Error("Function not implemented.");
				} } />
				{ userState.isLoggedin && (
					<>
					<Link href="/bookmarked"
						className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white"            
					>
						<Icon icon="basil:bookmark-solid" className="w-7 h-7" />
						<span className="text-lg font-bold">Bookmarks</span>
					</Link>
					<AddRecipe onAdd={ function (): void {
						throw new Error("Function not implemented.");
					} } />
					<DeleteButton onRecipeDeleted={function (): void {
					throw new Error("Function not implemented.");
					} } />
					</>
				) }
				<UserAuth />
			</div>
			<div className="w-full flex flex-wrap sm:flex-nowrap justify-center gap-8 px-4">
				<div className="w-full sm:w-1/3 md:w-1/4 2xl:w-1/5 h-auto sm:h-screen bg-red-500 text-white flex flex-col text-center justify-start rounded-lg p-3">
					<h2 className="text-2xl font-bold mt-4">Filters</h2>
					<div className="flex flex-col justify-start">
						<label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Vegan </label>
						<label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> High Protein </label>
						<label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Low Fat </label>
						<label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Gluten Free </label>
						<label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Low Carbs </label>
					</div>
				</div>
				<main className="w-full sm:w-2/3 md:w-3/4 2xl:w-4/5">
					{ children }
				</main>
			</div>
		</div>
		</UserContext.Provider>
	)
}
