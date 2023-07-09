import { Icon } from "@iconify/react";
import React, { useState } from 'react';
import { Recipe } from "@/lib/types";

const addRecipe: React.FC<Recipe> = () => {
    const [showForm, setShowForm] = useState(false);
    const [recipeTitle, setRecipeTitle] = useState("");
    const [recipeId, setRecipeId] = useState("");
    const [recipeServings, setRecipeServings] = useState("");
    const [recipes, setRecipes] = useState<Recipe[]>([]);

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
          <div>
          <button className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white" onClick={handleAddRecipeClick}>
          <Icon icon="basil:add-solid" className="w-8 h-8" />
          <span className="text-lg font-bold"> Add recipe</span>
          </button>
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
        </div>
      );
  };
  export default addRecipe;

