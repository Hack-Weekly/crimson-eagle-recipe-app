import { Icon } from "@iconify/react";
import React from "react";

type SearchBarProps = {};

const SearchBar: React.FC<SearchBarProps> = () => {
  return (
    <div className="relative text-gray-600 w-1/2">
      <input
        type="search"
        name="serch"
        placeholder="Search"
        className="bg-red-500 w-full h-11 px-5 pr-10 rounded-full text-sm text-white focus:outline-none placeholder-white "
      />
      <button type="submit" className="absolute right-0 top-0 mt-3 mr-4 ">
        <Icon icon={"carbon:search"} className="text-white w-6 h-6" />
      </button>
    </div>
  );
};
export default SearchBar;
