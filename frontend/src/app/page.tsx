"use client";
import SearchBar from "@/components/SearchBar";
import { Icon } from "@iconify/react";
import React, { useState, useEffect} from "react";
import DeleteButton from "@/components/DeleteRecipe";
import AddRecipe from "@/components/AddRecipe";
import UserAuth from "@/components/UserAuth";
import { Recipe } from "@/lib/types";
import BookmarkButton from "@/components/BookmarkButton";
import { jwtToken } from "@/components/UserAuth";

export default function Home() {
  const [recipes, setRecipes] = useState<Recipe[]>([]);
  const [selectedRecipe, setSelectedRecipe] = useState<Recipe | null>(null);
  const [showBookmarkedRecipes, setShowBookmarkedRecipes] = useState(false);

  const fetchData = async () => {
    try {
      const response = await fetch("https://crimson-eagles-recipe-app.onrender.com/recipes");
      const data = await response.json();
      const recipesWithBookmarkedFlag = data.map((recipe: Recipe) => ({
        ...recipe,
        bookmarked: false,
      }));
      setRecipes(recipesWithBookmarkedFlag);
    } catch (error) {
      console.log(error);
    }
  };

    useEffect(() => {
      fetchData();
      }, []);

      const fetchBookmarkedRecipes = async () => {
        try {
          const response = await fetch("https://crimson-eagles-recipe-app.onrender.com/recipes/recipes/bookmark", {
            headers: {
              Authorization: `Bearer ${jwtToken}`,
            },
          });
          if (response.ok) {
            const data = await response.json();
            setRecipes(data);
          } else {
            console.error("Failed to fetch bookmarked recipes");
          }
        } catch (error) {
          console.error("Failed to fetch bookmarked recipes:", error);
        }
      };
  
  const handleSearch = (searchResults: Recipe[]) => {
    setRecipes(searchResults);
  };

  const handleAddRecipe = () => {
    fetchData();
  };

  const handleRecipeDeleted = () => {
    fetchData(); 
  };

  const handleBookmark = (recipe: Recipe) => {
    const updatedRecipes = recipes.map((r) => {
      if (r.id === recipe.id) {
        return { ...r, bookmarked: !r.bookmarked };
      }
      return r;
    });
    setRecipes(updatedRecipes);
  };

  const showBookmarked = () => {
    setShowBookmarkedRecipes(true);
    fetchBookmarkedRecipes();
  };

  const hideBookmarked = () => {
    setShowBookmarkedRecipes(false);
  };

  return (
    <main className="h-full w-full flex  flex-col content-center justify-center py-10">
      <div className="flex justify-end items-center gap-12 pr-10 mb-8 ml-8">
        <h1 className="text-6xl font-extrabold">Foodly</h1>
        <SearchBar onSearch={function (searchResults: Recipe[]): void {
          throw new Error("Function not implemented.");
        } } />
          <button
          className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white"
          onClick={showBookmarked}
        >
          <Icon icon="basil:bookmark-solid" className="w-7 h-7" />
          <span className="text-lg font-bold">Bookmarks</span>
        </button>
          <AddRecipe onAdd={function (): void {
          throw new Error("Function not implemented.");
        } }   />
          <DeleteButton onRecipeDeleted={function (): void {
          throw new Error("Function not implemented.");
        } } />
          <UserAuth />
      </div>

    <div className="flex justify-center gap-8 pl-4">
      <div className="w-1/4 bg-red-500 text-white flex flex-col text-center justify-start rounded-lg p-3 h-screen">
          <h2 className="text-2xl font-bold mt-4">Filters</h2>
          <div className="flex flex-col justify-start">
            <label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Vegan </label>
            <label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> High Protein </label>
            <label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Low Fat </label>
            <label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Gluten Free </label>
            <label className="flex items-center my-1 text-2xl"><input type = "checkbox" className="h-4 w-4 rounded-sm mr-2 ml-3 flex items-center"/> Low Carbs </label>
          </div>
      </div>
      <div className="w-3/4">
        <div className="flex flex-wrap justify-start gap-8">
          {recipes.map((recipe) => (
              <div
                key={recipe.id}
                className="bg-white rounded-lg shadow-lg p-4 sm:p-6 w-full sm:w-1/2 md:w-1/3 lg:w-1/4 border border-black mb-8"
                onClick={() => setSelectedRecipe(recipe)}
              >
                <h2 className="text-2xl font-bold mb-2">{recipe.title}</h2>
                <p className="text-lg mb-2">Servings: {recipe.servings}</p>
                <p className="text-sm mb-2">
                  Created At: {recipe.created_at || "N/A"}
                </p>
                <p className="text-sm">
                  Updated At: {recipe.updated_at || "N/A"}
                </p>
                <BookmarkButton recipe={recipe} onBookmark={handleBookmark} />
              </div>
            ))}
        </div>
      </div>
    </div>

      {selectedRecipe && (
        <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
          <div className="bg-white rounded-lg shadow-lg p-6 w-2/4 h-2/4 flex flex-col items-center justify-center">
            <h2 className="text-2xl font-bold mb-4">{selectedRecipe.title}</h2>
            <p className="text-lg mb-2">Servings: {selectedRecipe.servings}</p>
            <p className="text-sm mb-2">
              Created At: {selectedRecipe.created_at || "N/A"}
            </p>
            <p className="text-sm">
              Updated At: {selectedRecipe.updated_at || "N/A"}
            </p>
            <button
              className="bg-red-500 text-white px-4 py-2 rounded-md mt-4"
              onClick={() => setSelectedRecipe(null)}
            >
              Close
            </button>
          </div>
        </div>
      )}
      {showBookmarkedRecipes && (
        <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
          <div className="bg-white rounded-lg h-3/4 max-h-3/4 w-3/4 shadow-lg p-6 overflow-y-scroll">
            <div className="flex justify-between">
            <h2 className="text-2xl font-bold mb-4">Bookmarked Recipes</h2>
            <button className="flex justify-center items-center px-4 py-5 h-6 w-20 bg-red-500 rounded-2xl text-white" onClick={hideBookmarked}>
              <span className="text-lg font-bold">Close</span>
            </button>
            </div>
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-5 overflow-y-auto">
              {recipes
                .filter((recipe) => recipe.bookmarked)
                .map((recipe) => (
                  <div
                    key={recipe.id}
                    className="bg-white rounded-lg shadow-lg p-4 sm:p-6 border border-black mb-8"
                    onClick={() => setSelectedRecipe(recipe)}
                  >
                    <h2 className="text-2xl font-bold mb-2">{recipe.title}</h2>
                    <p className="text-lg mb-2">Servings: {recipe.servings}</p>
                    <p className="text-sm mb-2">
                      Created At: {recipe.created_at || "N/A"}
                    </p>
                    <p className="text-sm">
                      Updated At: {recipe.updated_at || "N/A"}
                    </p>
                    <button
                      className={`${
                        recipe.bookmarked ? "bg-green-500" : "bg-gray-300"
                      } text-white px-2 py-1 rounded-md mt-4`}
                      onClick={(e) => {
                        e.stopPropagation();
                        handleBookmark(recipe);
                      }}
                    >
                      {recipe.bookmarked ? (
                        <Icon icon="carbon:checkmark-filled" />
                      ) : (
                        <Icon icon="carbon:add-filled" />
                      )}
                      <span className="ml-2">Bookmark</span>
                    </button>
                  </div>
                ))}
            </div>
          </div>
        </div>
      )}
    </main>
  );
}
