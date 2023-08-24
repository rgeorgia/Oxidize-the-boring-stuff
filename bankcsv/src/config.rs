use serde::Deserialize;
use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize)]
struct ShortName {
    search_string: String,
    category: String,
    short_name: String,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        [
            {
                "search_string": "BARNES AND NOBL",
                "category": "Books",
                "short_name": "Barns-n-Noble"
            },
            {
                "search_string": "BOOKS A MILL",
                "category": "Books",
                "short_name": "Books A Million"
            }
        ]
    "#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Vec<ShortName> = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    // println!("Search for: {:?}", p);
    // println!("Category: {}", p.category);
    // println!("Short Name: {}", p.short_name);
    for data in p.iter() {
        println!("{}", data.short_name);
    }

    Ok(())
}
