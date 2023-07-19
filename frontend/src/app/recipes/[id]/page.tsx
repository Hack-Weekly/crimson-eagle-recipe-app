"use client"

import { useEffect } from "react"
import { useRouter } from "next/navigation"
import { Icon } from "@iconify/react"
import InfoTabs from "@/components/InfoTabs"
import { useUserContext } from "@/context/user-state"
import useRecipeStore from "@/context/recipe-store"

type RecipeFullProps = {
    params: {
        id: string
    }
}
const RecipeFull = ({ params }: RecipeFullProps) => {

	const { userState } = useUserContext()
	const recipe = useRecipeStore(state => state.recipe)
	const isLoading = useRecipeStore(state => state.isLoading)
	const setUserState = useRecipeStore(state => state.setUserState)
	const fetchRecipe = useRecipeStore(state => state.fetchRecipe)
	const toggleBookmark = useRecipeStore(state => state.toggleBookmark)

    const router = useRouter()

    useEffect(() => {
        setUserState(userState)
		fetchRecipe(parseInt(params.id))
    }, [userState, setUserState, fetchRecipe, params.id])

    const created_at = recipe?.created_at ? new Date(recipe.created_at)
        .toLocaleDateString('en-us', { year:"numeric", month:"short", day:"numeric"})
        : 'N/A'
    const updated_at = recipe?.updated_at ? new Date(recipe.updated_at)
        .toLocaleDateString('en-us', { year:"numeric", month:"short", day:"numeric"})
        : 'N/A'

    const handleBookmark = (id: number) => {
        toggleBookmark(id)
    }

    return recipe ? (
        <div className="relative">
            <img className="w-full mb-4 object-cover aspect-video"
                src={ `https://source.unsplash.com/random/?food#${ new Date().getTime() }` }
                alt={ recipe.title } />
            <div className="absolute top-4 right-4 flex justify-end items-center gap-2">
                { userState.isLoggedIn && (
                    <button className={`${ recipe.bookmarked ? "bg-green-500" : "bg-red-500" }
                        flex items-center px-4 py-5 h-6 w-35 rounded-2xl text-white my-4`}
                        onClick={ () => handleBookmark(recipe.id) }
                    >
                        { recipe.bookmarked ? (
                        <Icon icon="carbon:checkmark-filled" />
                        ) : (
                        <Icon icon="carbon:add-filled" />
                        ) }
                        <span className="ml-2">
                            { recipe.bookmarked ? "Bookmarked" : "Bookmark" }
                        </span>
                    </button>
                ) }
                <button
                    className="bg-red-500 text-white px-4 py-2 rounded-full"
                    onClick={ () => router.back() }
                >Close</button>
            </div>
            <h2 className="text-2xl font-bold mb-2">{recipe.title}</h2>
            <div className="flex justify-between items-center">
                <p className="w-1/2 text-lg mb-2">Servings: {recipe.servings}</p>
                <p className="w-1/2 flex items-center text-gray-500">
                    <Icon icon="basil:clock-outline" className="w-7 h-7 mr-1" />
                    { recipe.timer }m
                </p>
            </div>
            <div className="flex flex-wrap my-2">
                <p className="w-1/2 mb-2 flex items-center">
                    <span className="mr-1 p-1 bg-gray-100 rounded-lg">
                        <Icon icon="fluent-emoji-high-contrast:fire" className="w-7 h-7" />
                    </span>
                    { recipe.kcal } Kcal</p>
                <p className="w-1/2 mb-2 flex items-center">
                    <span className="mr-1 p-1 bg-gray-100 rounded-lg">
                        <Icon icon="fluent-emoji-high-contrast:bread" className="w-7 h-7" />
                    </span>
                    { recipe.carbs }g carbs</p>
                <p className="w-1/2 mb-2 flex items-center">
                    <span className="mr-1 p-1 bg-gray-100 rounded-lg">
                        <Icon icon="fluent-emoji-high-contrast:cut-of-meat" className="w-7 h-7" />
                    </span>
                    { recipe.proteins }g proteins</p>
                <p className="w-1/2 mb-2 flex items-center">
                    <span className="mr-1 p-1 bg-gray-100 rounded-lg">
                        <Icon icon="fluent-emoji-high-contrast:avocado" className="w-7 h-7" />
                    </span>
                    { recipe.fats }g fats</p>
            </div>
            <p className="text-sm mb-2">Created At: { created_at }</p>
            <p className="text-sm">Updated At: { updated_at }</p>
            <InfoTabs ingredients={ recipe.ingredients } instructions={ recipe.instructions } />
        </div>
    ) : (
        <div className="w-full h-full bg-gray-800 bg-opacity-40 flex justify-center items-center">
            <div className="py-8 px-16 bg-white rounded-lg">
                <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Loading...
            </div>
        </div>
    )
}
export default RecipeFull