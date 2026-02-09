import type { SearchResponse } from '../types/search';

const API_BASE_URL = 'http://127.0.0.1:8080';

export async function searchBooks(term: string): Promise<SearchResponse> {
  const response = await fetch(`${API_BASE_URL}/search?term=${encodeURIComponent(term)}`);
  
  if (!response.ok) {
    throw new Error(`Search failed: ${response.statusText}`);
  }
  
  return response.json();
}
