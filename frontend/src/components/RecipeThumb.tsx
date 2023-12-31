"use client"

import Link from "next/link"
import { Icon } from "@iconify/react"
import type { Recipe } from "@/lib/types"

type RecipeThumbProps = {
    recipe: Recipe,
    isLoggedIn: boolean,
    handleBookmark: (id: number) => void,
}
const RecipeThumb = ({ recipe, isLoggedIn, handleBookmark }: RecipeThumbProps) => {

    const updated_at = recipe.updated_at ? new Date(recipe.updated_at)
        .toLocaleDateString('en-us', { year:"numeric", month:"short", day:"numeric"})
        : 'N/A'

    return (
        <Link href={ `/recipes/${ recipe.id }` }
            key={recipe.id}
            title={ recipe.title }
            className="flex flex-col justify-start bg-white rounded-lg shadow-lg border border-black
                relative px-4 pt-4 pb-16 sm:px-6 sm:t-6 text-left"
        >
            <img className="w-full aspect-square mb-4 object-cover"
                src={ `https://source.unsplash.com/random/?food#${ new Date().getTime() }` }
                alt={ recipe.title } />
            <h2 className="w-full text-2xl font-bold mb-2 truncate">{ recipe.title }</h2>
            <div className="flex justify-between items-center">
                <p className="text-lg mb-2">Servings: { recipe.servings }</p>
                <p className="flex items-center text-gray-500">
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
            <p className="text-sm">
                Last update: { updated_at }
            </p>
            { isLoggedIn && (
                <div className="absolute bottom-0 right-4">
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
                </div>
            ) }
        </Link>
    )
}

export default RecipeThumb