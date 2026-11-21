/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/
// #################################################### NOTE ###################################################################
// # Currently some bugs as it is not finished however it works, and has multi argument capabilities and some builtins already #
// #############################################################################################################################


use std::env::set_current_dir;
#[allow(unused_imports)]
use std::io::{self, BufRead ,Write};
use std::{env, fs};
use std::process::Command;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::io::ErrorKind;


fn main() {
    loop{
        // setting up shell and getting input (base default behaviour)
        let full_path_string: String = match env::var("PATH"){
            Ok(path_value) => path_value,
            Err(error) => {eprintln!("Could not read PATH: {}", error.to_string()); continue;},
        };
        default_term(); // default $ term and flushing it onto stdout

        let mut command = check_input(); // gets users command

        let command_as_vec: Vec<String> = command.split_whitespace().map(|word| String::from(word)).collect(); // getting full user input as vec to break into indexes
        
        let first_index_of_command_vec: String = match command_as_vec.first(){ // this gets index 0 and returns owned String
            Some(success) => success.to_string(),
            None => {print!(""); io::stdout().flush().unwrap(); continue},
        };
 
        let full_path_no_colon_vec: Vec<String> = full_path_string.split(':').map(String::from).collect();
        


        
        let mut cmd_to_be_exec = command.trim().split_whitespace();
        
        if let Some(program) = cmd_to_be_exec.next(){
            let args: Vec<&str> = cmd_to_be_exec.collect();
            // checking builtins
            if program == "exit" {
                std::process::exit(0);
            }
            else if program == "echo"{ // was first_index_of_command_vec
                echo_cmd(&command);
                continue;
            }
            else if program == "type"{
                type_cmd(&command_as_vec, &full_path_string);
                continue;
            }
            else if program == "pwd"{
                let path = match std::env::current_dir(){
                    Ok(success) => success,
                    Err(e) => {eprintln!("Error: {e}"); return;},
                };
                println!("{}", path.to_string_lossy());
                continue;
            }
            else if program == "cd"{
                if command_as_vec.len() == 1{
                    if let Ok(home) = std::env::var("HOME"){
                        if let Err(e) = std::env::set_current_dir(home){
                            eprintln!("cd: {}", e);
                        }
                    }
                    else{
                        eprintln!("cd: HOME env variable not set");
                    }
                }
                else if command_as_vec.len() > 2{
                    eprintln!("cd: too many arguments\nUsage: cd <DIR>");
                    continue;
                }
                else{
                    let target_dir = &command_as_vec[1];
                    match std::env::set_current_dir(target_dir) {
                        Ok(_) => (),
                        Err(e) => eprintln!("cd: {}: {}", target_dir, e),
                    };
                }
                
                continue;

                //let move_to_dir = set_current_dir(&args);
               // let move_to_dir = set_current_dir()
            }




            
            
            let mut check = false;
            
            if program.contains('/'){
                let executable_with_path = std::path::PathBuf::from(&program);
                let ran_command = Command::new(&executable_with_path).args(&args).status();
                match ran_command{
                    Ok(_) => continue,
                    Err(e) if e.kind() == io::ErrorKind::PermissionDenied =>{
                        eprintln!("Error 1st: {e}");
                        continue;
                    },
                    Err(e) if e.kind() == ErrorKind::InvalidData =>{
                        let fallback_command = Command::new("/bin/sh").arg(&executable_with_path).args(&args).status();
                        match fallback_command{
                            Ok(_) =>{continue;},
                            Err(fallback_err) => {eprintln!("Error: {}", fallback_err); continue;},
                        };
                    },
                    Err(_) => continue,
                };
            }

            for path in env::split_paths(&full_path_string){
                let executable = path.join(&program);

                if executable.is_file(){
                    let cmd_to_be_executed = Command::new(&executable).args(&args).status();
                    match cmd_to_be_executed{
                        Ok(_) => {(); check = true; break;},
                        Err(e) if e.kind() == io::ErrorKind::PermissionDenied =>{
                            eprintln!("Error: {e}");
                            check = true;
                            continue;
                        },
                        Err(e) if e.kind() == ErrorKind::InvalidData =>{
                            let fallback_command = Command::new("/bin/sh").arg(&executable).args(&args).status();
                            match fallback_command{
                                Ok(_) =>{check = true;},
                                Err(fallback_err) => {
                                    eprintln!("Error: {}", fallback_err); 
                                    check = false
                                },
                            }
                            break;

                        },
                        Err(e) =>{eprintln!("Error: {}", e); check =  false; break;}
                    }
                };

                
                
            }

            if !check{
                eprintln!("Command '{}' not found", first_index_of_command_vec);
            }
        }
        

        

    }

}




// checking user input func
fn default_term(){
    print!("$ ");
    if let Err(e) = io::stdout().flush(){
        eprintln!("Fatal error: {:?}", e);
    };
    // NOTE BELOW WAS OLD CODE BUT ILL KEEP SO TO SEE
    //.flush().unwrap(); // need this to print above
    // because stdout is line buffered, meaning rust holds what 
    // you want to print in memory and only sends it to screen when it 
    // encounters a newline (\n)
}




fn check_input() -> String {
    let mut command = String::new();

    match io::stdin().read_line(&mut command) {
        Ok(_) => {},
        Err(error) => panic!("Fatal error aborting: {:?}", error),
    };

    let mut inside_single_quotes = false;
    let mut add_space = false;
    let mut parsed_command = String::new();

    for c in command.chars() {
        if c == '\'' {
            inside_single_quotes = !inside_single_quotes;
            continue;
        }

        if c.is_whitespace() && !inside_single_quotes {
            if add_space {
                parsed_command.push(' ');
                add_space = false;
            }
            continue;
        }

        if !c.is_whitespace() || inside_single_quotes {
            add_space = true;
        }

        parsed_command.push(c);
    }

    while inside_single_quotes {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Fatal error: {:?}", e);
            break;
        }

        let mut new_command = String::new();

        match io::stdin().read_line(&mut new_command) {
            Ok(_) => (),
            Err(e) => panic!("Fatal error aborting {:?}", e),
        };

        for c in new_command.chars() {
            if c == '\'' {
                inside_single_quotes = !inside_single_quotes;
                continue;
            }

            if c.is_whitespace() && !inside_single_quotes {
                if add_space {
                    parsed_command.push(' ');
                    add_space = false;
                }
                continue;
            }

            if !c.is_whitespace() || inside_single_quotes {
                add_space = true;
            }

            parsed_command.push(c);
        }
    }

    parsed_command
}



fn not_found(input: &String){
    eprintln!("{}: command not found", input);
}




fn echo_cmd(input: &String){
    if input.starts_with("echo "){
        let args = input.strip_prefix("echo").unwrap();
        println!("{}", args);
    }
    else{
        println!();
    }
}




fn type_cmd(command_as_string_slice: &[String], full_path_string: &str){ // didnt have 2nd param before also command_as_vec was &Vec<String>
    
    for i in &command_as_string_slice[1..]{
        let mut found = false;
        if i.contains('/'){
            let cmd_path = Path::new(i);
            if cmd_path.is_file(){
                if let Ok(metadata) = cmd_path.metadata(){
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 == 0{
                        eprintln!("{}: Not executable", cmd_path.to_string_lossy());
                        continue;
                    }
                    else{
                        println!("{} is {}", i, cmd_path.to_string_lossy());
                        continue;
                    }
                }
            }            
        }
            
        

        
        match i.as_str(){
            "type" => {println!("type is a shell builtin"); found = true;},
            "exit" => {println!("exit is a shell builtin"); found = true;}
            "echo" => {println!("echo is a shell builtin"); found = true;}
            _ => {
                // for path in env::split_paths(&std::env::var("PATH").unwrap()){
                for path in env::split_paths(&full_path_string){
                    let executable = path.join(i);
                    if let Ok(metadata) = executable.metadata(){
                        let permissions = metadata.permissions();
                        if permissions.mode() & 0o111 == 0{ // performs bitwise actions (AND) to check if executable
                            //eprintln!("Not executable");
                            continue;
                        };           
                    }
                    if executable.is_file(){
                        println!("{} is {}", i, executable.display());
                        found = true;
                        break;
                    }
                } 
            },
        };
        if !found{not_found(i);}
    }
}



