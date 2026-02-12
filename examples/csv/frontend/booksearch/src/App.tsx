import { useState, useMemo } from 'react'
import debounce from 'lodash.debounce'
import {
  Autocomplete,
  AutocompleteInput,
  AutocompletePositioner,
  AutocompletePopup,
  AutocompleteList,
  AutocompleteItem,
  AutocompleteEmpty,
} from './components/ui/autocomplete'
import { searchBooks } from './api/search'
import type { SearchResult } from './types/search'

function App() {
  const [results, setResults] = useState<SearchResult[]>([])
  const [loading, setLoading] = useState(false)

  const debouncedSearch = useMemo(
    () =>
      debounce(async (query: string) => {
        setLoading(true)
        try {
          const response = await searchBooks(query)
          setResults(response.results)
        } catch (error) {
          console.error('Search failed:', error)
          setResults([])
        } finally {
          setLoading(false)
        }
      }, 300),
    []
  )

  const handleValueChange = (details: string) => {
    if (!details?.trim()) {
      setResults([])
      debouncedSearch.cancel()
      return
    }

    debouncedSearch(details)
  }

  return (
    <div className="min-h-screen bg-gray-50 px-4 sm:px-6 lg:px-8">
      <div className="mx-auto max-w-7xl pt-8">
        <Autocomplete onValueChange={handleValueChange}>
          <AutocompleteInput placeholder="Search books..." />
          <AutocompletePositioner>
            <AutocompletePopup>
              <AutocompleteList>
                {results.map((result, index) => (
                  <AutocompleteItem
                    key={`${result.original_entry}-${index}`}
                    value={result.original_entry}
                  >
                    {result.original_entry}
                  </AutocompleteItem>
                ))}
              </AutocompleteList>
              <AutocompleteEmpty>
                {loading ? 'Loading...' : 'No results found'}
              </AutocompleteEmpty>
            </AutocompletePopup>
          </AutocompletePositioner>
        </Autocomplete>
      </div>
    </div>
  )
}

export default App
