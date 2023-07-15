"use client"

import { useState } from "react"

type InfoTabsProps = {
    ingredients: {
		unit: string | null;
		label: string;
		amount: number | null;
	}[],
    instructions: string[],
}
const InfoTabs = ({ ingredients, instructions }: InfoTabsProps) => {

    const [selected, setSelected] = useState("ingredients")

    return (
        <div className="w-full my-6 px-2">
            <div className="flex justify-between items-center bg-gray-200 p-2 rounded-xl -mx-2 gap-2">
                <button onClick={ () => setSelected("ingredients") } className={ `w-1/2 rounded-xl px-6 py-3
                    ${ selected == 'ingredients' ? 'bg-gray-800 text-white' : 'bg-transparent text-gray-700 hover:bg-gray-300'}` }>
                    Ingredients</button>
                <button onClick={ () => setSelected("instructions") } className={ `w-1/2 rounded-xl px-6 py-3
                    ${ selected == 'instructions' ? 'bg-gray-800 text-white' : 'bg-transparent text-gray-700 hover:bg-gray-300'}` }>
                    Instructions</button>
            </div>
            { selected == "ingredients" ? (
            <ul className="ml-8 mr-2 my-6 leading-loose list-outside
                list-image-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTIiIHZpZXdCb3g9IjAgMCAxNCAxMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiBmaWxsPSIjMzhiZGY4Ij48cGF0aCBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0xMy42ODUuMTUzYS43NTIuNzUyIDAgMCAxIC4xNDMgMS4wNTJsLTggMTAuNWEuNzUuNzUgMCAwIDEtMS4xMjcuMDc1bC00LjUtNC41YS43NS43NSAwIDAgMSAxLjA2LTEuMDZsMy44OTQgMy44OTMgNy40OC05LjgxN2EuNzUuNzUgMCAwIDEgMS4wNS0uMTQzWiIgLz48L3N2Zz4=')]">
                { ingredients.map(({ unit, label, amount }, k) => (
                    <li key={ k }>{ amount } { unit } { label }</li>
                ))}
            </ul>
            ) : (
            <ul className="ml-8 mr-2 my-6 leading-relaxed list-outside list-decimal">
                { instructions.map((i, k) => (
                    <li className="my-2" key={ k }>{ i }</li>
                ))}
            </ul>
            )}
        </div>
    )
}

export default InfoTabs