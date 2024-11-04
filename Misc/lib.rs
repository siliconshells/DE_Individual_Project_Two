use csv::ReaderBuilder; //for loading from csv
use rusqlite::{params, Connection, Result}; 
use std::error::Error;
use std::fs::File; //for loading csv //for capturing errors from loading

pub mod my_lib;


// Here we will have a function for each of the commands

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(()) //returns nothing except an error if it occurs
}

//Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        // Assuming the `users` table has an `id` and `name` column
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        Ok((id, name,age))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (id, name,age) = row?;
        println!("ID: {}, Name: {}, Age: {}", id, name, age);
    }

    Ok(())
}

//delete
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

//load data from a file path to a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> { //Box<dyn Error> is a trait object that can represent any error type
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (id, name, age) VALUES (?, ?, ?)",
        table_name
    );
    //this is a loop that expects a specific schema, you will need to change this if you have a different schema
    for result in rdr.records() {
        let record = result?;
        let id: i32 = record[0].parse()?; //.parse() is a method that converts a string into a number
        let name: &str = &record[1];
        let age: i32 = record[2].parse()?;

        conn.execute(&insert_query, params![id, name, age])?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}




// pub fn read_all_data(database_name: &str, table_name: &str) -> Result<Option<Vec<String>>> {
//     let conn = Connection::open(database_name)?;
//     let query = format!("SELECT * FROM {}", table_name);
//     let _  = log_tests("Executing query...", false, false, false, false);

//     let mut stmt = conn.prepare(&query)?;
//     let result: Result<Vec<String>> = stmt.query_map([], |row| row.get(0))?.collect();

//     Ok(result.ok())
// }

// pub fn read_data(
//     database_name: &str,
//     table_name: &str,
//     data_id: i64,
// ) -> Result<Option<Vec<String>>> {
//     let conn = Connection::open(database_name)?;
//     let primary_key = get_primary_key(&conn, table_name)?;

//     let query = format!("SELECT * FROM {} WHERE {} = ?", table_name, primary_key);
//     println!("{:?}",query);
//     let _  = log_tests("Executing query...", false, false, false, false);
//     println!("2");
//     let mut stmt = conn.prepare(&query)?;
//     println!("Primary key value: {}", data_id);
//     let result: Result<Vec<String>> = stmt.query_map([data_id], |row| row.get(0))?.collect();

//     println!("3");
//     Ok(result.ok())
// }

// pub fn read_data(
//     database_name: &str,
//     table_name: &str,
//     data_id: i64,
// ) -> Result<Option<String>> {
//     let conn = Connection::open(database_name)?;
//     let primary_key = get_primary_key(&conn, table_name)?;
//     println!("1");
//     let query = format!("SELECT * FROM {} WHERE {} = ?", table_name, primary_key);
//     println!("{:?}", query);
//     let _ = log_tests("Executing query...", false, false, false, false);
//     println!("2");
//     let mut stmt = conn.prepare(&query)?;
    
//     // Collect rows into a Vec<String> and handle the Result
//     let result: Result<Vec<String>> = stmt.query_map([data_id], |row| {
//         let value: i64 = row.get(0)?; // Retrieve as integer
//         Ok(value.to_string()) // Convert to String
//     })?.collect();

//     // Handle the result and join into a single String if successful
//     match result {
//         Ok(vec) => {
//             let combined_string = vec.join(", "); // Join elements with ", " or any separator
//             println!("Combined string: {:?}", combined_string);
//             Ok(Some(combined_string))
//         }
//         Err(e) => {
//             // Log error if needed, or handle it accordingly
//             eprintln!("Error retrieving data: {:?}", e);
//             Ok(None)
//         }
//     }
// }


// #[test]
// fn test_read_data() -> Result<()> {
//     let _ = log_tests("One Record Reading Test", true, false, false, false);
    
//     let row = read_data("air_quality.db", "air_quality", 740885);
//     let _ = log_tests("Asserting that row[0][data_value] == 16.4", false, false, false,  false);
//     match row {
//         Ok(Some(vec)) => {
//             // Now you can index into `vec` as it is a `Vec<String>`
//             assert_eq!(vec[5], "16.4");
//         }
//         Err(e) => {
//             // Handle the error case
//             eprintln!("Error occurred: {:?}", e);
//         }
//         Ok(None) => {
//             // Handle the case where the query returned no results (None)
//             println!("No data found.");
//         }
//     }
//     let _ = log_tests("Assert Successful", false, false, false,  false);
    
//     let _ = log_tests("One Record Reading Test Successful", true, false, false, false);
    
//     Ok(())
// }





let data: Vec<f64> = (start..=end).map(|i| i as f64).collect();

// Measure the time for get_mean
let start = Instant::now();
match get_mean(&data) {
    Ok(mean) => println!("Mean: {}", mean),
    Err(e) => println!("Error calculating mean: {}", e),
}
let duration = start.elapsed();
println!(
    "Time taken by get_mean: {} microseconds",
    duration.as_micros()
);


// fn convert_path_to_string(path: &str) -> std::io::Result<String> {
//     let path_buf = PathBuf::from(path);

//     // Convert to &Path
//     let path = path_buf.as_path();

//     // Convert to String (if valid UTF-8)
//     match path.to_str() {
//         Some(path_string) => {
//             let path_string = path_string.to_string();
//             Ok(path_string)
//         }
//         None => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Path is not valid UTF-8")),
//     }
// }



    // let path = match convert_path_to_string("../speed_test/") {
    //     Ok(path_str) => path_str,
    //     Err(e) => format!("Error: {}", e),
    // };
    // println!("{:?}",path);




    //Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        // Assuming the `users` table has an `id` and `name` column
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        Ok((id, name, age))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (id, name, age) = row?;
        println!("ID: {}, Name: {}, Age: {}", id, name, age);
    }

    Ok(())
}
