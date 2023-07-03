"use client";
import SearchBar from "@/components/SearchBar";
import { Icon } from "@iconify/react";
import Image from "next/image";
import React, { useState, useEffect} from "react";

interface Recipe {
  id: number;
  title: string;
  servings: string;
  created_at: string | null;
  updated_at: string | null;
}

export default function Home() {
  const [recipes, setRecipes] = useState<Recipe[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [recipeTitle, setRecipeTitle] = useState("");
  const [recipeId, setRecipeId] = useState("");
  const [recipeServings, setRecipeServings] = useState("");
  const [selectedRecipe, setSelectedRecipe] = useState<Recipe | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch("https://crimson-eagles-recipe-app.onrender.com/recipes");
        const data = await response.json();
        setRecipes(data);
      } catch (error) {
        console.log(error);
      }
    };

    fetchData();
    }, []);

  const handleAddRecipeClick = () => {
    setShowForm(true);
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
  
    const recipeData = {
      title: recipeTitle,
      id: recipeId,
      servings: recipeServings,
    };
  
    try {
      const response = await fetch("https://crimson-eagles-recipe-app.onrender.com/recipes", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(recipeData),
      });
  
      if (response.ok) {
        const newRecipe = await response.json();
        setRecipes((prevRecipes) => [...prevRecipes, newRecipe]);
      } else {
        console.error("Failed to add recipe:", response.status);
      }
    } catch (error) {
      console.error("Error occurred while adding recipe:", error);
    }
  
    setRecipeTitle("");
    setRecipeId("");
    setRecipeServings("");
    setShowForm(false);
  }; 

  return (
    <main className="h-full w-full flex  flex-col content-center justify-center py-10">
      <h1 className="text-6xl font-extrabold text-center mb-10">Foodly</h1>
      <div className="flex justify-end items-center gap-12 pr-10 mb-8">
        <SearchBar />
        <button className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white" onClick={handleAddRecipeClick}>
          <Icon icon="basil:add-solid" className="w-8 h-8" />
          <span className="text-lg font-bold"> Add recipe</span>
        </button>
        <button className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
          <Icon icon="subway:mark-2" className="w-8 h-6" />
          <span className="text-lg font-bold"> Bookmarks</span>
        </button>
      </div>

    <div className="flex justify-center gap-8 pl-4">
      <div className="w-1/4 bg-red-500 text-white flex flex-col items-center justify-start rounded-lg p-6 h-screen">
          <h2 className="text-2xl font-bold mt-4">Filters</h2>
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
            </div>
          ))}
        </div>
      </div>
    </div>

    {showForm && (
        <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
          <div className="bg-white rounded-lg shadow-lg p-6 w-3/4 h-3/4">
            <div className="flex justify-between items-center mb-4">
            <h2 className="text-2xl font-bold mb-4">Add Recipe</h2>
            <button className="font-bold text-2xl pr-8" onClick={() => setShowForm(false)}>
            X
            </button>
            </div>
            <form onSubmit={handleSubmit}>
              <div className="mb-4">
                <label htmlFor="title" className="block font-bold mb-2">
                  Title
                </label>
                <input
                  type="text"
                  id="title"
                  value={recipeTitle}
                  onChange={(e) => setRecipeTitle(e.target.value)}
                  className="border border-gray-300 px-3 py-2 rounded-md w-full"
                  required
                />
              </div>
              <div className="mb-4">
                <label htmlFor="id" className="block font-bold mb-2">
                  ID
                </label>
                <input
                  type="text"
                  id="id"
                  value={recipeId}
                  onChange={(e) => setRecipeId(e.target.value)}
                  className="border border-gray-300 px-3 py-2 rounded-md w-full"
                  required
                />
              </div>
              <div className="mb-4">
                <label htmlFor="title" className="block font-bold mb-2">
                  Servings
                </label>
                <input
                  type="text"
                  id="servings"
                  value={recipeServings}
                  onChange={(e) => setRecipeServings(e.target.value)}
                  className="border border-gray-300 px-3 py-2 rounded-md w-full"
                  required
                />
              </div>
              <button type="submit"className="bg-red-500 text-white px-4 py-2 rounded-md">
                Add
              </button>
            </form>
            <button className="bg-gray-300 text-gray-700 px-4 py-2 rounded-md mt-4" onClick={() => setShowForm(false)}>
              Cancel
            </button>
          </div>
        </div>
      )}

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
    </main>
  );
}
