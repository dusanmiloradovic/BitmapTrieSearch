use bitmap_trie::dictionary::AttributeSearch;
use csvexample::CsvDictionary;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CSV Dictionary Example");
    println!("======================");

    // Sample CSV data (in a real application, this would come from a file)
    let csv_data = r#"name,position,company,location,salary
John Doe,Software Engineer,Tech Corp,San Francisco,120000
Jane Smith,Data Scientist,AI Labs,New York,135000
Bob Johnson,Product Manager,StartupXYZ,Austin,110000
Alice Brown,DevOps Engineer,Cloud Co,Seattle,125000
Charlie Wilson,Frontend Developer,Web Solutions,Boston,95000
Diana Martinez,Backend Developer,Data Systems,Chicago,115000
Eve Thompson,Full Stack Developer,Innovation Inc,Denver,105000
Frank Garcia,Machine Learning Engineer,AI Research,Los Angeles,140000
Grace Lee,UX Designer,Design Studio,Portland,85000
Henry Davis,Security Engineer,CyberSafe,Miami,130000"#;

    // Configure attributes for different types of searching
    let attributes = vec![
        ("name".to_string(), AttributeSearch::Multiple), // Search by words in name
        ("position".to_string(), AttributeSearch::Multiple), // Search by words in position
        ("company".to_string(), AttributeSearch::Exact), // Exact prefix match for company
        ("location".to_string(), AttributeSearch::Exact), // Exact prefix match for location
        ("salary".to_string(), AttributeSearch::None),   // Salary as metadata only
    ];

    // Create and populate dictionary
    let mut dict = CsvDictionary::new(attributes);
    let reader = Cursor::new(csv_data);

    let count = dict.load_from_csv(reader, true)?;
    println!("Loaded {} records from CSV\n", count);

    // Demonstrate different types of searches
    let searches = vec![
        ("John", "Search by first name"),
        ("Engineer", "Search by job title word"),
        ("Tech", "Search by company prefix"),
        ("San", "Search by location prefix"),
        ("Data", "Search across multiple fields"),
    ];

    for (term, description) in searches {
        println!("🔍 {} - '{}':", description, term);
        let results = dict.search(term);

        if results.is_empty() {
            println!("   No results found\n");
        } else {
            for result in results {
                println!(
                    "   ✓ Found '{}' in {} field: '{}'",
                    result.term, result.attribute, result.original_entry
                );
            }
            println!();
        }
    }

    // Demonstrate case insensitive search
    println!("🔍 Case insensitive search - 'john' (lowercase):");
    let results = dict.search("john");
    for result in results {
        println!(
            "   ✓ Found '{}' in {} field: '{}'",
            result.term, result.attribute, result.original_entry
        );
    }

    Ok(())
}
