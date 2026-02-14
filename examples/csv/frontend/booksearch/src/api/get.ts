const API_BASE_URL = 'http://127.0.0.1:8080';

export async function getBook(id:string) :Promise<Record<string,string>> {
  const response = await fetch(`${API_BASE_URL}/get?id=${encodeURIComponent(id)}`);
  if (!response.ok) {
    throw new Error(`Search failed: ${response.statusText}`);
  }

  return response.json();
}