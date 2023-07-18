import React, { useState } from "react";
import { Icon } from "@iconify/react";
import { getJwtToken } from "./UserAuth";


interface DeleteButtonProps {
  onRecipeDeleted: () => void;
}

const DeleteButton: React.FC<DeleteButtonProps> = ({ onRecipeDeleted }) => {
  const [showForm, setShowForm] = useState(false);
  const [inputValue, setInputValue] = useState<number | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleDeleteRecipe = async () => {
    setError(null);
    setIsSubmitting(true);
    try {
      const response = await fetch(`https://crimson-eagles-recipe-app.onrender.com/recipes/${inputValue}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          // Authorization: `Bearer ${getJwtToken()}`, // add this back in if the backend implements authentication 
        },
      });
      if (response.ok) {
        console.log("Recipe deleted successfully");
        setInputValue(null);
        setShowForm(false);
        onRecipeDeleted();
      } else {
        // Check if response is JSON before trying to parse it
        const contentType = response.headers.get("content-type");
        if (contentType && contentType.indexOf("application/json") !== -1) {
          const errorData = await response.json();
          setError(`Failed to delete recipe: ${errorData.message}`);
        } else {
          setError("Failed to delete recipe. Please ensure the recipe ID is correct.");
        }
      }
    } catch (error) {
      setError(`Network error: ${error}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (window.confirm("Are you sure you want to delete this recipe?")) {
      handleDeleteRecipe();
    }
  };

  return (
    <div>
      <button onClick={() => setShowForm(true)} className="flex justify-between items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
        <Icon icon="basil:trash-solid" className="w-6 h-8" />
        <span className="text-lg font-serif-extrabold"> Delete recipe</span>
      </button>
      {showForm && (
        <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-800 bg-opacity-40 flex justify-center items-center">
          <div className="bg-white rounded-lg drop-shadow-lg p-6">
            <h2 className="text-2xl font-bold mb-4">Delete Recipe</h2>
            <form onSubmit={handleSubmit}>
              <div className="mb-4">
                <label htmlFor="recipeID" className="block font-bold mb-2">
                  Recipe ID
                </label>
                <input 
                  type="number" 
                  id="recipeID"
                  value={inputValue || ""} 
                  onChange={(e) => setInputValue(Number(e.target.value))} 
                  placeholder="Enter recipe ID" 
                  className="border border-gray-300 rounded px-2 py-1 mr-2" 
                  required
                />
              </div>
              {error && <p className="text-red-500 mb-4">{error}</p>}
              <button 
                type="submit" 
                disabled={isSubmitting}
                className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white">
                Delete
              </button>
            </form>
            <button className="bg-gray-300 text-gray-700 px-4 py-2 rounded-md mt-4" onClick={() => {
                setShowForm(false);
                setInputValue(null);
                setError(null);                
                }}>
              Cancel
            </button>
          </div>
        </div>
      )}
    </div>        
  );
};

export default DeleteButton;