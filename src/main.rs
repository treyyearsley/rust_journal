use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io;

//create function to handle journal entries
fn journal_entry() {
    //this will create a file in which to store the entries
    let mut data_file = File::create("journal.txt").expect("creation failed");
    let mut journal_entry = String::new();

    println!("\nEnter the date followed by your journal entry. (12/11/2023  Dear Journal...)");
    //write the entries to the txt file.
    io::stdin().read_line(&mut journal_entry).expect("Failed to read line");
    data_file.write({journal_entry}.as_bytes()).expect("write failed");
}

//function to handle reading ftom the txt file.
fn read_journal(){
        // choose which file to read from
    let mut journal_file = File::open("journal.txt").unwrap();
    //turn the file contents into a string
    let mut journal_content = String::new();
    
        // read the contents of the txt file
    journal_file.read_to_string(&mut journal_content).unwrap();
    
    println!("Journal content: {:?}", journal_content);

}

//main function will allow the user to chose to make an entry or read the file.
fn main() {
    println!("\n1: Add journal entry.");
    println!("\n2: Display journal entries.");
    println!("\nPlease enter either option 1 or option 2");
    
    loop{
        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");
    
        let option: u32 = match option.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        //option 1 will allow the user to make an entry.
        if option == 1 {
            journal_entry();
            break;
        }
        //option 2 will read the txt file.
        else {
        read_journal();
        break;
        }
    }

}

