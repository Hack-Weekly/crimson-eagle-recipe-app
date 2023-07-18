import { create } from 'zustand'
import type { Recipe, Tag, SearchState, Pagination } from "@/lib/types"
import type { UserState } from './user-state'

const buildurl = (query: string | null, filter: Tag[]) => {
    let queries: string[] = []
    if (query && query != "") {
        queries.push(`page=1`) // TODO add pagination
        queries.push('per_page=12')
    }
    if (filter && filter.length > 0) {
        for (const tag of filter) {
            queries.push(`tags[]=${tag.slug}`)
        }
    }
    return queries.length > 0 ? `/search/${ query }?${ queries.join('&') }` : ''
}

type RecipeState = {
    recipes: Recipe[],
    recipe: Recipe | null,
    search: SearchState,
    isLoading: boolean,
    lastUrl: string | null,
    lastUpdate: number | null,
    userState: UserState,
    setUserState: (userState: UserState) => void,
    fetchRecipes: () => void,
    fetchRecipe: (id: number) => void,
    setQuery: (query: string) => void,
    setFilter: (filter: Tag[]) => void,
    toggleBookmark: (id: number) => void,
}
const useRecipeStore = create<RecipeState>((set, get) => ({
    recipes: [] as Recipe[],
    recipe: null,
    search: {
        query: null,
        filter: [] as Tag[],
    },
    lastUrl: null,
    lastUpdate: null,
    userState: {
        isLoading: true,
        isLoggedIn: false,
        token: null,
    },
    isLoading: false,
    setUserState: (userState: UserState) => {
        set({ userState })
    },
    fetchRecipes: async () => {
        // don't reload if not needed
        const lastUrl = get().lastUrl
        const userState = get().userState

        const url = `${ process.env.apiUrl }/recipes` + buildurl(get().search.query, get().search.filter)

        if (lastUrl) {
            let reload = false
            const [uri, token, time] = lastUrl.split("|")
            
            if (url != uri) reload = true
            if (userState.isLoggedIn) {
                if (userState.token != token) reload = true                
            } else {
                if (token != "" && token != "null") reload = true
            }
            // 5 minutes
            if (Date.now() - parseInt(time) > 1000 * 60 * 5) reload = true

            if (!reload) return
        }

        // debounce
        const lastUpdate = get().lastUpdate
        if (get().isLoading && lastUpdate && Date.now() - lastUpdate < 200)
            return

        set({
            isLoading: true,
            lastUpdate: Date.now()
        })
		
        const fetching = (userState.isLoggedIn) ? fetch(url, {
            headers: {
                'Authorization': `Bearer ${ userState.token }`,
            },
            credentials: 'include',
        }) : fetch(url)

        // TODO error handling
        fetching
            .then(res => {
                if (!res.ok) throw new Error("Status not 200")
                else return res.json()
            })
            .then((res: Pagination<Recipe>) => set({
                recipes: res.records,
                lastUrl: `${ url }|${ userState.token }|${ Date.now() }`,
            }))
            .catch(err => {
                set({
                    lastUrl: null,
                })
                console.log(err)
            })
            .finally(() => {
                set({ isLoading: false })
            })
    },
    fetchRecipe: async (id: number) => {        
        const recipes = get().recipes
        if (recipes.length > 0) {
            const recipe = recipes.find(r => r.id == id)
            if (recipe) {
                set({ recipe })
                return
            }
        }
        
        // debounce
        const lastUpdate = get().lastUpdate
        if (get().isLoading && lastUpdate && Date.now() - lastUpdate < 200)
        return
        
        set({
            isLoading: true,
            lastUpdate: Date.now()
        })

        const url = `${ process.env.apiUrl }/recipes/${ id }`
		
        const userState = get().userState
        const fetching = (userState.isLoggedIn) ? fetch(url, {
            headers: {
                'Authorization': `Bearer ${ userState.token }`,
            },
            credentials: 'include',
        }) : fetch(url)

        // TODO error handling
        fetching
            .then(res => {
                if (!res.ok) {
                    if (res.status == 404) throw new Error("Recipe not found.")
                    throw new Error("Status not 200")
                }
                else return res.json()
            })
            .then((res: Recipe) => set({ recipe: res }))
            .catch(console.log)
            .finally(() => {
                set({ isLoading: false })
            })
    },
    setQuery: (query: string) => {
        set(state => ({
            search: {
                ...state.search,
                query,
            }
        }))
        get().fetchRecipes()
    },
    setFilter: (filter: Tag[]) => {
        set(state => ({
            search: {
                ...state.search,
                filter,
            }
        }))
        get().fetchRecipes()
    },
    toggleBookmark: (id: number) => {
        const userState = get().userState
        if (!userState.isLoggedIn)
            return

        fetch(`${ process.env.apiUrl }/bookmarks/${ id }`, {
            method: 'PUT',
            headers: {
                'Authorization': `Bearer ${ userState.token }`,
            },
            credentials: 'include',
        })
            .then(res => res.json())
            .then(res => {
                set(state => ({
                    recipe: state.recipe && state.recipe.id == id ? {
                        ...state.recipe,
                        bookmarked: res,
                    } : null,
                    recipes: state.recipes && state.recipes.length > 0 ?
                        state.recipes.map(r => r.id == id ? {
                            ...r,
                            bookmarked: res,
                        } : r)
                        : [],
                }))})
    }
}))

export default useRecipeStore