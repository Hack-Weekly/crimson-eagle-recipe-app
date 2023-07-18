"use client"

import { useRouter } from "next/navigation"
import React, { useState, useEffect, useContext} from "react"
import type { Recipe, Pagination } from "@/lib/types"
import RecipeThumb from "@/components/RecipeThumb"
import { UserContext } from "@/context/user-state"

export default function Home() {
	const [recipes, setRecipes] = useState<Recipe[]>([])
	const { userState } = useContext(UserContext)

	const router = useRouter()

	useEffect(() => {
		const fetchData = async () => {
			if (userState.isLoggedin) {
				fetch("https://crimson-eagles-recipe-app.onrender.com/recipes", {
					headers: {
						'Authentication': `Bearer ${ userState.token }`,
					},
					credentials: 'include',
				})
					.then(res => res.json())
					.then((res: Pagination<Recipe>) => {
						const recipesWithBookmarkedFlag = res.records.map((recipe: Recipe) => ({
							...recipe,
							bookmarked: false,
						}));
						setRecipes(recipesWithBookmarkedFlag)
					})
					.catch(console.log)
			} else {
				fetch("https://crimson-eagles-recipe-app.onrender.com/recipes")
					.then(res => res.json())
					.then((res: Pagination<Recipe>) => {
						const recipesWithBookmarkedFlag = res.records.map((recipe: Recipe) => ({
							...recipe,
							bookmarked: false,
						}));
						setRecipes(recipesWithBookmarkedFlag)
					})
					.catch(console.log)
			}
		}

		fetchData()
	}, [userState])

	const handleBookmark = (recipe: Recipe) => {
		const updatedRecipes = recipes.map((r) => {
			if (r.id === recipe.id) {
				return { ...r, bookmarked: !r.bookmarked }
			}
			return r
		})
		setRecipes(updatedRecipes)
	}

	const onSelectRecipe = (recipe: Recipe | null) => {
		if (recipe == null) {
			router.push('/recipes')
		} else {
			router.push(`/recipes/${ recipe.id }`)
		}
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-6 md:gap-8">
		{ recipes.map((recipe) => (
			<RecipeThumb key={ recipe.id }
				recipe={ recipe }
				setSelectedRecipe={ onSelectRecipe }
				isLoggedIn= { userState.isLoggedin }
				handleBookmark={ handleBookmark } />
		)) }
		</div>
	)
}
