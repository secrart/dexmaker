/*
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
*/

use std::fs::write;
use std::fmt::Display;
use std::process::exit;

fn get_arg(arg: char, args: &Vec<String>) -> Result<String, String> {
    
    let mut next = false; 

    for i in args {
        if next { 
            return Ok(i.clone()); 
        }
        if i.starts_with('-') && i.ends_with(arg) {
            next = true;  
        }
    }

    if next {
        Err(format!("arg -{a} found but no parameter given! should be -{a} [parameter]", a = arg))
    } else {
        Err(format!("arg -{} not found!", arg)) 
    }
}

// kids don't do this it's bad
// I know its bad practice to just kill the process but I feel like we can trust the kernel to
// clean up the program memory
//
// // TODO: don't do this 😔
trait CleanUnwrap<T, U: Display> {
    fn peaceful_unwrap(self) -> T;
}

// peacefully smothers the program to death while printing a message right before doing so... it's
// to make the error message cleaner without the whole "PANICKED!!!!!!!!!" thing
impl CleanUnwrap<String, String> for Result<String, String> {
    
    fn peaceful_unwrap(self) -> String {
        match self {
            Ok(x) => x,
            Err(x) => {
                println!("{}", x);
                exit(0); 
            }
        } 
    }

} 

struct Recipe {

    program_name: String,
    file_name: String,
    icon_loc: String,
    show_console: bool,
    file_loc: String,

}

impl Recipe {

    fn new_with_loc(program_name: String, file_name: String, icon_loc: String, show_console: bool, file_loc: String) -> Recipe {
        Recipe {
            program_name,
            file_name,
            icon_loc,
            show_console,
            file_loc,
        }   
    }
    
    fn new(program_name: String, file_name: String, icon_loc: String, show_console: bool) -> Recipe {
        Recipe {
            program_name,
            file_name,
            icon_loc,
            show_console,
            file_loc: String::from("/usr/share/applications"),
        }   
    }

    fn cook(self) {

        let write_string = 
           format!("[Desktop Entry]\nVersion=1.0\nType=Application\nName={name}\nExec={file_name}\nIcon={icon_name}\nTerminal={show_console}", name=self.program_name, file_name=self.file_name, icon_name=self.icon_loc, show_console=self.show_console);

        match write(format!("{}/{}.{}", self.file_loc, self.file_name, "desktop"), write_string) {
            Ok(_) => (),
            Err(x) => {println!("Failed to create file {}", x)}
        }         

    }

}

fn main() {

    let arguments: Vec<String> = std::env::args().collect();

    let program_name = get_arg('n', &arguments).peaceful_unwrap(); 
    let file_name = get_arg('f', &arguments).peaceful_unwrap();
    let icon_loc = get_arg('i', &arguments).peaceful_unwrap();  
    let show_console = get_arg('s', &arguments).peaceful_unwrap();
    
    let show_console_bool = match show_console.parse::<bool>() {

        Ok(x) => x,
        Err(_) => {
            println!("show console parameter \"-s {}\" not valid!\ndefaulting to false", show_console); 
            false
        },

    };

    // an optional parameter
    let potential_file_loc = get_arg('l', &arguments); 

    let recipe = match potential_file_loc {
        Ok(x) => Recipe::new_with_loc(program_name, file_name, icon_loc, show_console_bool, x),
        Err(_) => Recipe::new(program_name, file_name, icon_loc, show_console_bool),
    };

    recipe.cook(); 

    println!("done!"); 
}
