extern crate openssl;

use openssl::rsa::{Rsa,Padding};
use openssl::sha::sha256;
use std::collections::HashMap;
use std::io::{self,Write};
use std::fs::File;

// there are the imports we use
fn main() {
//Declared global variables
   let mut password_map: HashMap<String,String>=HashMap::new();   
   let rsa = Rsa::generate(2048).unwrap();
// starting the loop for the CLI logic
   loop{
// basic user prompts
    println!("\n Save your medical records");
    println!("\n 1. Save Records");
    println!("\n 2. Retrieve Records");
    println!("\n 3. Exit");
    println!("\n Enter Choice");

    io::stdout().flush().unwrap();

let mut choice= String::new();
io::stdin().read_line(&mut choice).unwrap();

//storing user data in a variable and sharting a match function according to the user selection
    match choice.trim(){
    "1" => {
        // taking valuable data from the user
        println!("\n Enter your name");
        io::stdout().flush().unwrap();

        let mut site = String::new();
        io::stdin().read_line(&mut site).unwrap();
        let site = site.trim().to_string();

        println!("\n Enter medical datas");
        io::stdout().flush().unwrap();

        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
// encrypting the data
        let mut enc_data = vec![0; rsa.size() as usize];    
        let len = rsa.public_encrypt(&sha256(password.as_bytes()), &mut enc_data, Padding::PKCS1).unwrap();
       
        enc_data.truncate(len); 

        let encrypted_password = base64::encode(&enc_data);

        password_map.insert(site,encrypted_password); 
    },
    "2" => {
        print!("\nEnter your name: ");
        io::stdout().flush().unwrap();

        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();
//taking the name from the user
        if let Some(encrypted_password) = password_map.get(&name) {
            let encrypted_password_bytes= base64::decode(&encrypted_password).unwrap();
            let mut decrypted_data_buf = vec![0; rsa.size() as usize];
            let decrypted_data = rsa.private_decrypt(&encrypted_password_bytes, &mut decrypted_data_buf, Padding::PKCS1).unwrap();
            decrypted_data_buf.truncate(decrypted_data);

          // decrypting the data
        
           let base64_encoded = base64::encode(&decrypted_data_buf);
    println!("base64 encoded data: {}", base64_encoded);
//saving the decrypted data to a file
    let mut encrypted_file = File::create("decrypted_base64_medical_data.txt").expect("creation failed");   
    encrypted_file.write(base64_encoded.as_bytes()).expect("write failed");    
    encrypted_file.write("->".as_bytes()).expect("write failed");  
    encrypted_file.write(name.as_bytes()).expect("write failed");  
         
    println!("Medical Data Saved to files Succesfully");
        }else{
            println!("No data saved for {}",name);

        }
    },
    "3" => break,
    _ => println!("Invalid choice"),
    }  
   }



}