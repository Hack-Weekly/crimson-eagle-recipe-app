import React from "react";
import { Icon } from "@iconify/react";
import { Recipe } from "@/lib/types";
import { getJwtToken } from "@/components/UserAuth";

interface BookmarkButtonProps {
  recipe: Recipe;
  onBookmark: (recipe: Recipe) => void;
}

const BookmarkButton: React.FC<BookmarkButtonProps> = ({ recipe, onBookmark }) => {
  const handleClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    try {
      const response = await fetch(`https://crimson-eagles-recipe-app.onrender.com/bookmarks/${recipe.id}`, {
        method: "PUT",
        headers: {
          Authorization: `Bearer ${getJwtToken()}`,
        },
      });
      if (response.ok) {
        onBookmark(recipe);
      } else {
        console.error("Failed to bookmark recipe");
      }
    } catch (error) {
      console.error("Failed to bookmark recipe:", error);
    }
  };

  return (
    <button
      className={`${
        recipe.bookmarked ? "bg-green-500" : "bg-gray-300"
      } flex items-center px-4 py-5 h-6 w-35 rounded-2xl text-white my-4`}
      onClick={handleClick}
    >
      {recipe.bookmarked ? (
        <Icon icon="carbon:checkmark-filled" />
      ) : (
        <Icon icon="carbon:add-filled" />
      )}
      <span className="ml-2">
        {recipe.bookmarked ? "Bookmarked" : "Bookmark"}
      </span>
    </button>
  );
};

export default BookmarkButton;