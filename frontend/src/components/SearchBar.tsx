import { Icon } from "@iconify/react";
import React, { useState } from "react";
import { SearchBarProps } from "../lib/types";

const SearchBar: React.FC<SearchBarProps> = ({ onSearch }) => {
  const [query, setQuery] = useState("");

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      const response = await fetch(`https://crimson-eagles-recipe-app.onrender.com/recipes/search/${query}?page=1&per_page=10`);
      const data = await response.json();
      onSearch(data); // Call the callback function with the search results
    } catch (error) {
      console.error("An error occurred:", error);
    }
  };

  return (
    <form onSubmit={handleSearch} className="relative text-gray-600 w-1/2">
      <input
        type="search"
        name="search"
        placeholder="Search"
        className="bg-red-500 w-full h-11 px-5 pr-10 rounded-full text-sm text-white focus:outline-none placeholder-white"
        onChange={(e) => setQuery(e.target.value)}
      />
      <button type="submit" className="absolute right-0 top-0 mt-3 mr-4">
        <Icon icon={"carbon:search"} className="text-white w-6 h-6" />
      </button>
    </form>
  );
};

export default SearchBar;
