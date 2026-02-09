export interface SearchResult {
  term: string;
  attribute: string;
  original_entry: string;
  attribute_index: number;
  position: number;
}

export interface SearchResponse {
  results: SearchResult[];
}
