import Image from "next/image";
"use client";
import {useEffect, useState} from "react";

export default function Home() {

  const [recipes, setRecipes] = useState([]);

  useEffect(() => {
    const fetchRecipes = async () => {
      try {
        const response = await fetch("/recipes");
        if (response.ok) {
          const data = await response.json();
          setRecipes(data.results);
        } else {
          throw new Error("request failed");
        }
      } catch (error) {
        console.error(error);
      } 
    };

    fetchRecipes();
  }, []);

  return (
    <main>
      <h1 className="text-3xl font-extrabold">Recipe App</h1>
      <ul>
        {recipes.map((recipe) => (
          <li key={recipe.id}> {recipe.title} </li>
        ))}
      </ul>
    </main>
  );
}
