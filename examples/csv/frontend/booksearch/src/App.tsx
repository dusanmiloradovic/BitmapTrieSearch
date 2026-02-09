import { useState } from 'react'
import {
  Autocomplete,
  AutocompleteInput,
  AutocompletePositioner,
  AutocompletePopup,
  AutocompleteList,
  AutocompleteItem,
  AutocompleteEmpty,
} from './components/ui/autocomplete'

function App() {
  const [value, setValue] = useState('')
  const items = ['Apple', 'Banana', 'Cherry', 'Date', 'Elderberry', 'Fig', 'Grape']

  return (
    <div className="min-h-screen bg-gray-50 px-4 sm:px-6 lg:px-8">
      <div className="w-full mt-8">
        <Autocomplete value={value} onValueChange={(details) => setValue(details)}>
          <AutocompleteInput placeholder="Search..." />
          <AutocompletePositioner>
            <AutocompletePopup>
              <AutocompleteList>
                {items
                  .filter((item) => item?.toLowerCase().includes(value?.toLowerCase()))
                  .map((item) => (
                    <AutocompleteItem key={item} value={item}>
                      {item}
                    </AutocompleteItem>
                  ))}
              </AutocompleteList>
              <AutocompleteEmpty>No results found</AutocompleteEmpty>
            </AutocompletePopup>
          </AutocompletePositioner>
        </Autocomplete>
      </div>
    </div>
  )
}

export default App
