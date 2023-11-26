use std::io;

fn main() {
    let mut input1 = String::new();

    println!("How many siblings do you have...");
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let nus:i32 = input1.trim().parse().expect("Not a valid input");

    if nus >= 0 {
        let mut x = 0;
        for x in 0..nus{
            let mut input2 = String::new();
            println!("Enter your sibling's name...");
            io::stdin().read_line(&mut input2).expect("Could not read input");

            let mut input3 = String::new();
            println!("Enter your sibling's age...");
            io::stdin().read_line(&mut input3).expect("Could not read input");
            let sage:i32 = input3.trim().parse().expect("Not a valid input");


          let sage = sage;
          if sage > 18 {
              let mut input4 = String::new();
             println!("Is your sibling married or not?
                i.e. true or false");
             io::stdin().read_line(&mut input4).expect("Could not read input");
             let marstate:bool = input4.trim().parse().expect("Not a valid input");

                 if marstate == false {
                     let mut input5 = String::new();
                     println!("Is your sibling a student or worker
                     Student: false
                     Worker: true");
                     io::stdin().read_line(&mut input5).expect("Could not read input");
                     let empstate:bool = input5.trim().parse().expect("Not a valid input");

                         if empstate == false {
                             let mut input6 = String::new();
                             println!("Enter the name of your sibling's university...", );
                             io::stdin().read_line(&mut input6).expect("Could not read input");
  
                             let mut input7 = String::new();
                             println!("Enter your sibling's course of study...");
                             io::stdin().read_line(&mut input7).expect("Could not read input");

                 }
                else if marstate == true{
                     let mut input8 = String::new();
                     println!("Does your sibling have any children?
                        i.e. true or false");
                     io::stdin().read_line(&mut input8).expect("Could not read input");
                     let chid:bool = input8.trim().parse().expect("Not a valid input");

                     let mut input9 = String::new();
                     println!("Where does your sibling live?");
                     io::stdin().read_line(&mut input9).expect("Could not read input");
             }    
             }
          else if sage < 18 {
            let mut waec = String::new();
            println!("Have they written WAEC?
                i.e. true or false");
            io::stdin().read_line(&mut waec).expect("Could not read input");
            let waec:bool = waec.trim().parse().expect("Not a valid input");
             
             if waec == true {
                let mut sec = String::new();
                println!("What was the name of their secondary school?");
                io::stdin().read_line(&mut sec).expect("Could not read input");
             }
             else {
                 let mut class = String::new();
                 println!("What class are they in?");
                 io::stdin().read_line(&mut class).expect("Could not read input");
             }

          }   

         }

        }
    x+=1}

    
}
