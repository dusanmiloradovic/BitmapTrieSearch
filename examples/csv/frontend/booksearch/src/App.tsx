import { useState , useMemo } from 'react'
import debounce from 'lodash.debounce'
import {
  Autocomplete ,
  AutocompleteInput ,
  AutocompletePositioner ,
  AutocompletePopup ,
  AutocompleteList ,
  AutocompleteItem ,
  AutocompleteEmpty ,
  AutocompleteCollection ,
} from './components/ui/autocomplete'
import { searchBooks } from './api/search'
import type { SearchResult } from './types/search'
import { getBook } from "@/api/get.ts";

// Helper function to highlight search term in text
const highlightTerm = ( text: string , term: string ) => {
  if ( !term.trim() ) return text

  const regex = new RegExp( `(${term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})` , 'gi' )
  const parts = text.split( regex )

  return parts.map( ( part , index ) =>
    regex.test( part ) ? <strong key={ index }>{ part }</strong> : part
  )
}

function App() {
  const [ results , setResults ] = useState<SearchResult[]>( [] )
  const [ loading , setLoading ] = useState( false )
  const [ searchTerm , setSearchTerm ] = useState( '' )
  const [ book , setBook ] = useState<Record<string , string>>( {} )

  const debouncedSearch = useMemo(
    () =>
      debounce( async ( query: string ) => {
        setLoading( true )
        try {
          const response = await searchBooks( query )
          setResults( response.results )
        } catch ( error ) {
          console.error( 'Search failed:' , error )
          setResults( [] )
        } finally {
          setLoading( false )
        }
      } , 300 ) ,
    []
  )


  const handleItemClick =async  ( result: SearchResult ) => {
    console.log( 'Item clicked:' , result )

    // Ensure the search term is preserved after any potential component updates
    const currentTerm = searchTerm
    setTimeout( () => {
      if ( searchTerm !== currentTerm ) {
        setSearchTerm( currentTerm )
      }
    } , 0 )

    // Handle the click - you can add your custom logic here
    // For example, navigate to a detail page, add to favorites, etc.
    const res_book = await getBook( result.dictionary_index as unknown as string)
    console.log( res_book )
    setBook( res_book)
  }

  return (
    <div className="min-h-screen bg-gray-50 px-4 sm:px-6 lg:px-8">
      <div className="mx-auto max-w-7xl pt-8">
        <Autocomplete items={ results }>
          <AutocompleteInput
            placeholder="Search books..."
            value={ searchTerm }
            onChange={ ( e ) => {
              const value = e.target.value
              setSearchTerm( value )

              if ( !value?.trim() ) {
                setResults( [] )
                debouncedSearch.cancel()
                return
              }

              debouncedSearch( value )
            } }
          />
          <AutocompletePositioner>
            <AutocompletePopup>
              <AutocompleteList>
                <AutocompleteCollection>
                  { ( result: SearchResult ) => (
                    <AutocompleteItem
                      key={ result.original_entry }
                      value={ result.original_entry }
                      onClick={ ( ev ) => {
                        ev.preventDefault();
                        ev.stopPropagation();
                        handleItemClick( result );
                      } }
                    >
                      { highlightTerm( result.original_entry , searchTerm ) }
                    </AutocompleteItem>
                  ) }
                </AutocompleteCollection>
              </AutocompleteList>
              <AutocompleteEmpty>
                { loading ? 'Loading...' : 'No results found' }
              </AutocompleteEmpty>
            </AutocompletePopup>
          </AutocompletePositioner>
        </Autocomplete>
      </div>
      <div className="mx-auto max-w-7xl py-12 px-4 sm:px-6 lg:px-8">
        <div className="space-y-12">
          <div className="space-y-5 sm:space-y-4 md:max-w-xl lg:max-w-3xl xl:max-w-none">
            <h2 className="text-3xl font-extrabold tracking-tight sm:text-4xl">{book.title}</h2>
            <p className="text-xl text-gray-500">{book.author}</p>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
