"use client"

import { useEffect } from "react"
import RecipeThumb from "@/components/RecipeThumb"
import { useUserContext } from "@/context/user-state"
import useRecipeStore from "@/context/recipe-store"

const Recipes = () =>  {
	const { userState } = useUserContext()
	const recipes = useRecipeStore(state => state.recipes)
	const isLoading = useRecipeStore(state => state.isLoading)
	const setUserState = useRecipeStore(state => state.setUserState)
	const fetchRecipes = useRecipeStore(state => state.fetchRecipes)
	const toggleBookmark = useRecipeStore(state => state.toggleBookmark)

	useEffect(() => {
		setUserState(userState)
		fetchRecipes()
	}, [userState, setUserState, fetchRecipes])

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-6 md:gap-8">
		{ recipes.map((recipe) => (
			<RecipeThumb key={ recipe.id }
				recipe={ recipe }
				isLoggedIn= { userState.isLoggedIn }
				handleBookmark={ toggleBookmark } />
		)) }
		</div>
	)
}

export default Recipes