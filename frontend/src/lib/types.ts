export interface Recipe {
  id: number;
  title: string;
  servings: string;
  timer: number | null;
  kcal: number | null;
  carbs: number | null;
  proteins: number | null;
  fats: number | null;
  image: string | null;
  instructions: string[];
  ingredients: {
  unit: string | null;
  label: string;
  amount: number | null;
}[];
  tags: string[];
  bookmarked: boolean | null;
  owned: boolean | null;
  created_at: string | null;
  updated_at: string | null;
}
  