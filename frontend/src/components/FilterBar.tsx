import { useEffect, useRef, useState } from "react"
import type { Tag } from "@/lib/types"

type FilterBarProps = {
    onFilter: (filter: Tag[]) => void,
}
const FilterBar = ({ onFilter }: FilterBarProps) => {

    const formRef = useRef<HTMLFormElement>(null)
    const [dropdownVisible, setDropdownVisible] = useState(false)
    const [tags, setTags] = useState<Tag[]>([])
    const [filteredTags, setFilteredTags] = useState<Tag[]>([])
    const [selectedTags, setSelectedTags] = useState<Tag[]>([])

    useEffect(() => {
        fetch(`${ process.env.apiUrl }/tags`)
            .then(res => res.json())
            .then(tags => {
                setTags(tags)
                setFilteredTags(tags)
            })
            .catch(console.log)       
        
        // handle esc and outside clicks for the dropdown
        const handleKeyDown = (event: KeyboardEvent) => {
            if (event.key === 'Escape') {
                setDropdownVisible(false)
            }
        }
        const handleClick = (event: MouseEvent) => {
            if (formRef.current && !formRef.current.contains(event.target as Node)) {
                setDropdownVisible(false)
            }
            else if (formRef.current && formRef.current.contains(event.target as Node)) {
                setDropdownVisible(true)
            }
        }

        document.addEventListener('keydown', handleKeyDown, false)
        document.addEventListener('mousedown', handleClick, false)

        return () => {
            document.removeEventListener('keydown', handleKeyDown, false)
            document.removeEventListener('mousedown', handleClick, false)
        }
    }, [])

    const onSearchFocus = () => {
        setDropdownVisible(true)
    }

    const onSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFilteredTags(tags.filter(tag => tag.label.includes(e.target.value)))
    }

    const onAdd = (tag: Tag) => {
        setSelectedTags(prev => [
            ...prev,
            tag,
        ])
        onFilter(selectedTags)
    }

    const onRemove = (tag: Tag) => {
        setSelectedTags(prev => prev.filter(t => t.slug != tag.slug))
        onFilter(selectedTags)
    }

    return (
        <div className="w-full sm:w-1/3 md:w-1/4 2xl:w-1/5 h-auto sm:h-screen 
            flex flex-col text-center justify-start rounded-lg p-3 bg-red-500 text-white">
            <h2 className="text-2xl font-bold mt-4">Filter by tags</h2>
            <form ref={ formRef } className="my-2 flex flex-col justify-start text-gray-700">
                <label htmlFor="tags-search" className="sr-only"></label>
                <input className="py-2 px-3 rounded-sm text-gray-700 peer"
                    type="text" name="tags-search" id="tags-search"
                    placeholder="Search tags" onFocus={ onSearchFocus }
                    onChange={ onSearch } />
                { filteredTags && filteredTags.length > 0 && dropdownVisible && (
                    <ul className="mt-1 rounded bg-white">
                        { filteredTags.map(tag => (
                            <li key={ tag.slug } className="inline-block">
                                <button type="button" onClick={ () => onAdd(tag) }
                                    className="m-1 py-1 px-2 rounded bg-gray-100 hover:bg-gray-200">
                                    { tag.label }</button>
                            </li>
                        ))}
                    </ul>
                )}
            </form>
            { selectedTags && selectedTags.length > 0 && (
                <ul className="mt-1">
                    { selectedTags.map(tag => (
                        <li key={ tag.slug } className="inline-block">
                            <button onClick={ () => onRemove(tag) }
                                className="m-1 py-1 px-2 rounded border border-white hover:border-2">
                                { tag.label }</button>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    )
}

export default FilterBar