"use client";
import SearchBar from "@/components/SearchBar";
import { Icon } from "@iconify/react";
import Image from "next/image";
export default function Home() {
  return (
    <main className="h-full w-full flex  flex-col content-center justify-center py-10">
      <h1 className="text-6xl font-extrabold text-center mb-10">Foodly</h1>
      <div className="flex justify-end items-center gap-12 pr-10">
        <SearchBar />
        <button className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
          <Icon icon="basil:add-solid" className="w-8 h-8" />
          <span className="text-lg font-bold"> Add recipe</span>
        </button>
        <button className="flex justify-between items-center px-4 py-5 h-6 w-40 bg-red-500 rounded-2xl text-white">
          <Icon icon="subway:mark-2" className="w-8 h-6" />
          <span className="text-lg font-bold"> Bookmarks</span>
        </button>
      </div>
    </main>
  );
}
