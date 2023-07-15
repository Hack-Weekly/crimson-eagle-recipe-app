export interface Recipe {
    tags: any;
    bookmarked: boolean;
    id: number;
    title: string;
    servings: string;
    created_at: string | null;
    updated_at: string | null;
  }
  
export type SearchBarProps = {
    onSearch: (searchResults: Recipe[]) => void;
};
  