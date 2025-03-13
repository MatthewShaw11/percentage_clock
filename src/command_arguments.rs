use std::env;
use std::collections::HashMap;

pub struct CommandArgs {
    pub collection: HashMap<String,Option<String>>
}

impl CommandArgs {
    pub fn new() -> Self
    {
        let input: Vec<String> = env::args().collect();
        let mut arguments: HashMap<String,Option<String>> = HashMap::new(); 

        let mut ignore_next_arument_because_its_a_value_for_a_key = false;
        for (index,value) in input.iter().enumerate()
        {
            //println!("{index}: \"{value}\"");

            if ignore_next_arument_because_its_a_value_for_a_key
            {
                ignore_next_arument_because_its_a_value_for_a_key = false;
                continue;
            }

            let argument = value.trim();
            
            if argument.starts_with("--")
            {
                let key = argument.to_string();

                let not_last_argument = index < input.iter().count() - 1;
                if not_last_argument 
                {
                    let value = input.get(index + 1).unwrap().trim();

                    if value.starts_with("--") == false {
                        arguments.insert(key.to_string(), Option::Some(value.to_string()));
                        ignore_next_arument_because_its_a_value_for_a_key = true;
                    } 
                }
                
                if ignore_next_arument_because_its_a_value_for_a_key == false {
                    arguments.insert(key.to_string(), Option::None);
                }

            } 
            else {
                arguments.insert(argument.to_string(), Option::None);
            }
        }

        Self {
            collection: arguments 
        }
    }


    pub fn get(&self, key: &str) -> Option<Option<String>> {
        self.collection.iter().find_map(|(k,v)|
        {
            if k == key {
                Some(v.clone())
            } else {
                Option::None
            }
        })
    }

    pub fn contains(&self, key: &str) -> bool {
        let search = self.get(key);
        
        if search == None { false }
        else { true }
    }

    // pub fn count(&self) -> usize {
    //     self.collection.len()
    // }
}




