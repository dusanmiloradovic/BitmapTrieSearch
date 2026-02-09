import { useState, useEffect } from 'react'
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
  const [value, setValue] = useState('')
  const [results, setResults] = useState<SearchResult[]>([])
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    if (!value?.trim()) {
      setResults([])
      return
    }

    const debounce = setTimeout(async () => {
      setLoading(true)
      try {
        const response = await searchBooks(value)
        setResults(response.results)
      } catch (error) {
        console.error('Search failed:', error)
        setResults([])
      } finally {
        setLoading(false)
      }
    }, 50)

    return () => clearTimeout(debounce)
  }, [value])

  return (
    <div className="min-h-screen bg-gray-50 px-4 sm:px-6 lg:px-8">
      <div className="mx-auto max-w-7xl pt-8">
        <Autocomplete value={value} onValueChange={(details) => setValue(details)}>
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
