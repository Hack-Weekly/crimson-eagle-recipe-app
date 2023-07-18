import { Icon } from "@iconify/react";
import React, { useEffect, useRef, useState } from "react";
import { Pagination, Recipe, SearchBarProps } from "@/lib/types";

const SearchBar: React.FC<SearchBarProps> = ({ onSearch }) => {
  const [query, setQuery] = useState("");
  const previousQuery = useRef(query);

  useEffect(() => {
    const delayDebounceFn = setTimeout(async () => {
      if (query !== previousQuery.current) {
        // Query has changed
        const endpoint = query
          ? `https://crimson-eagles-recipe-app.onrender.com/recipes/search/${query}?page=1&per_page=10`
          : 'https://crimson-eagles-recipe-app.onrender.com/recipes';
        try {
          const response = await fetch(endpoint);
          const data: Pagination<Recipe> = await response.json();
          onSearch(data.records);
          previousQuery.current = query;
        } catch (error) {
          console.error('An error occurred:', error);
        }
      }
    }, 300);

    return () => clearTimeout(delayDebounceFn);
  }, [query, onSearch]);

  return (
    <form className="relative text-gray-600 w-1/2">
      <input
        type="search"
        name="search"
        placeholder="Search"
        className="bg-red-500 w-full h-11 px-5 pr-10 rounded-full text-sm text-white focus:outline-none placeholder-white"
        onChange={(e) => setQuery(e.target.value)}
      />
      <button type="button" className="absolute right-0 top-0 mt-3 mr-4">
        <Icon icon={"carbon:search"} className="text-white w-6 h-6" />
      </button>
    </form>
  );
};

export default SearchBar;
