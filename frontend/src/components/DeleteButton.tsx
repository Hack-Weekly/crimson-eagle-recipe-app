import React, { useState } from "react";
import { Icon } from "@iconify/react";

interface DeleteButtonProps {
    recipeId: number;
}

const DeleteButton: React.FC<DeleteButtonProps> = ({ recipeId }) => {
    const [showForm, setShowForm] = useState(false);
    const [inputValue, setInputValue] = useState('');

    const handleDeleteRecipe = async () => {
        try {
            const response = await fetch('/recipes/', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ id: recipeId }),
            });

            if (response.ok) {
                console.log('Recipe deleted successfully');
            } else {
                console.error('Failed to delete recipe');
            }
        } catch (error) {
            console.error('Ntwork error:', error);
        }
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        handleDeleteRecipe();
        setShowForm(false);
        setInputValue('');
    };

    return (
        <div>
            {showForm ? (
                <div className="fixed top-0 left-0 right-0 bottom-0 bg-gray-200 bg-opacity-50 flex justify-center items-center">
                    <div className="bg-white rounded-lg shadow-lg p-6">
                        <h2 className="text-2xl font-bold mb-4">Delete Recipe</h2>
                        <form onSubmit={handleSubmit}>
                            <div className="mb-4">
                                <label htmlFor="recipeID" className="block font-bold mb-2">
                                    Recipe ID
                                </label>
                                <input 
                                    type='text' 
                                    id="recipeID"
                                    value={inputValue} 
                                    onChange={(e) => setInputValue(e.target.value)} 
                                    placeholder="Enter recipe ID" 
                                    className="border border-gray-300 rounded px-2 py-1 mr-2" 
                                    required
                                />
                            </div>
                            <button 
                                type="submit" 
                                className="flex justify-between items-center px-2 py-5 h-6 w-38 bg-red-500 rounded-2xl text-white">
                                Delete
                            </button>
                        </form>
                        <button className="bg-gray-300 text-gray-700 px-4 py-2 rounded-md mt-4" onClick={() => setShowForm(false)}>
              Cancel
            </button>
                    </div>
                </div>
            ) : (
                <button onClick={() => setShowForm(true)} className="flex justify-between items-center px-2 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
                    <Icon icon="basil:trash-solid" className="w-6 h-8" />
                    <span className="text-lg font-bold"> Delete recipe</span>
                </button>
            )}
        </div>        
    );
};

export default DeleteButton;