use std::io;

fn main() {
    let mut reader = io::stdin();

    let mut sum = 0i;
    let mut result: Vec<int> = Vec::new();

    let n_groups: uint = from_str(reader.read_line()
                                       .unwrap()
                                       .as_slice()
                                       .trim()).unwrap();
    
    let groups: Vec<int> = 
            reader.read_line().unwrap().as_slice()
                  .split(' ')     
                  .map(|s| {
                    let num = from_str(s).unwrap();
                    sum = sum + num;
                    return num;
                  })  
                  .collect();           
    
    for y in range(1, sum) {
        if sum % y != 0i {
            continue;
        } 

        let mut tmp = 0i;

        for i in range(0, n_groups) {
            tmp = tmp + groups[i];
            
            if tmp > y {
                break;
            }

            if tmp == y {
                tmp = 0i;
            }
        }

        if tmp == 0i {
            result.push(y);
        }        
    }
    
    result.push(sum);
    for r in result.iter() {
      print!("{} ", *r);
    }
    println!("");
}