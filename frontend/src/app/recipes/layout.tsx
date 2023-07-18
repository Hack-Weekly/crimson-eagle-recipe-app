"use client"

import { type ReactNode } from "react"
import Link from "next/link"
import { Icon } from "@iconify/react"
import AddRecipe from "@/components/AddRecipe"
import DeleteButton from "@/components/DeleteRecipe"
import SearchBarWithDebounce from "@/components/SearchBarWithDebounce"
import UserAuthWithContext from "@/components/UserAuthWithContext"
import FilterBar from "@/components/FilterBar"
import { useUserContext } from "@/context/user-state"
import type { Tag } from "@/lib/types"
import useRecipeStore from "@/context/recipe-store"

const RecipeLayout = ({ children }: { children: ReactNode }) => {
	const { userState } = useUserContext()
	const setQuery = useRecipeStore(state => state.setQuery)
	const setFilter = useRecipeStore(state => state.setFilter)

	const onSearch = (query: string) => {
		setQuery(query)
	}

	const onFilter = (filter: Tag[]) => {
		setFilter(filter)
	}

	return (
		<div className="h-full w-full max-w-[1920px] mx-auto flex flex-col items-center justify-center py-10">
			<div className="w-full flex justify-end items-center gap-12 pr-10 mb-8 ml-8">
				<h1 className="text-6xl font-extrabold">Foodly</h1>
				<SearchBarWithDebounce onSearch={ onSearch } />
				{ userState.isLoggedIn && (
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
						throw new Error("Function not implemented.")
					} } />
					</>
				) }
				<UserAuthWithContext />
			</div>
			<div className="w-full flex flex-wrap sm:flex-nowrap justify-center gap-8 px-4">
				<FilterBar onFilter={ onFilter } />
				<main className="w-full sm:w-2/3 md:w-3/4 2xl:w-4/5">
					{ children }
				</main>
			</div>
		</div>
	)
}

export default RecipeLayout