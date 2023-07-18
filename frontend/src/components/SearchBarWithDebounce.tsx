import { Icon } from "@iconify/react"
import React, { useEffect, useMemo } from "react"
import debounce from "lodash.debounce"

type SearchBarProps = {
  onSearch: (query: string) => void,
}
const SearchBarWithDebounce: React.FC<SearchBarProps> = ({ onSearch }) => {

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onSearch(e.target.value)
  }
  const debouncedQuery = useMemo(() => {
    return debounce(handleChange, 500)
  }, [])

  useEffect(() => {
    return () => {
      debouncedQuery.cancel()
    }
  }, [debouncedQuery])

  return (
    <form className="relative text-gray-600 w-1/2">
      <input
        type="search"
        name="search"
        placeholder="Search"
        className="bg-red-500 w-full h-11 px-5 pr-10 rounded-full text-sm text-white focus:outline-none placeholder-white"
        onChange={ debouncedQuery }
      />
      <button type="button" className="absolute right-0 top-0 mt-3 mr-4">
        <Icon icon={"carbon:search"} className="text-white w-6 h-6" />
      </button>
    </form>
  );
};

export default SearchBarWithDebounce
