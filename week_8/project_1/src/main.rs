use std::io;


fn main() {
    let mut num = 0;
    while num < 10{
     let aps1_2 = vec!["Intern", "Under-graduate", "Paralegal", "Placement"];
     let aps3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
     let aps6_8 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"];
     let el8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
     let el10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
     let ses = vec!["CEO", "Dean", "Partner", "Principal"];

     let mut prof = String::new();
     println!("Please enter your field of work...
         Key:
         1: Office Administrator
         2: Academic
         3: Lawyer
         4: Teacher\n\n");
     io::stdin().read_line(&mut prof).expect("Not a valid input");
     let prof:i32 = prof.trim().parse().expect("Not a specified number");

     if prof == 1 {
           let mut adm = String::new();
         println!("Please enter your position...
         Key:
         1: Intern
         2: Administrator
         3: Senior Administrator
         4: Office Manager
         5: Director
         6: CEO\n\n");
         io::stdin().read_line(&mut adm).expect("Not a valid input");
         let adm:i32 = adm.trim().parse().expect("Not a specified number");

         let mut exp1 = String::new();
         println!("How many years of experience do you have?");
         io::stdin().read_line(&mut exp1).expect("Not a valid input");
         let exp1:i32 = exp1.trim().parse().expect("Nota a specified number");

         if adm == 1 && exp1 > 0 && exp1 <= 2  
             {
             println!("Hello
             You are an {}
             You have work experience of {} years
             You are of the level APS1-2",aps1_2[0], exp1 );
             }
         if adm == 2 && exp1 >= 3 && exp1 <= 5
         {
             println!("Hello
             You are an {}
             You have work experience of {} years
             You are of the level APS3-5",aps3_5[0], exp1 );
             }
         if adm == 3 && exp1 >= 6 && exp1 <= 8
         {
             println!("Hello
             You are an {}
             You have work experience of {} years
             You are of the level APS6-8",aps6_8[0], exp1 );
             }
         if adm == 4 && exp1 >= 8 && exp1 <= 10
         {
             println!("Hello
             You are an {}
             You have work experience of {} years
             You are of the level EL1 8-10",el8_10[0], exp1 );
             } 
         if adm == 5 && exp1 >= 10 && exp1 <= 13
         {
             println!("Hello
             You are an {}
             You have work experience of {} years
             You are of the level EL10-13",el10_13[0], exp1 );
             } 
         if adm == 6 && exp1 > 13
         {
             println!("Hello
             You are an {}
             You have work experience of {} years
             You are of the level SES",ses[0], exp1 );
             }
         }
     if prof == 2 {
         let mut aca = String::new();
     println!("Please enter your position...
        Key:
        1: Under-graduate
        2: Research Assistant
        3: PhD Candidate
        4: Post-Doc Researcher
        5: Senior Lecturer
        6: Dean\n\n");
     io::stdin().read_line(&mut aca).expect("Not a valid input");
     let aca:i32 = aca.trim().parse().expect("Not a specified number");

     let mut exp2 = String::new(); 
     println!("How many years of experience do you have?");
     io::stdin().read_line(&mut exp2).expect("Not a valid input");
     let exp2:i32 = exp2.trim().parse().expect("Not a specified number");

     if aca == 1 && exp2 > 0 && exp2 <= 2 
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS1-2",aps1_2[1], exp2 );
     }
     if aca == 2 && exp2 >= 3 && exp2 <= 5
     {
     println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS3-5",aps3_5[1], exp2 );
     }
     if aca == 3 && exp2 >= 6 && exp2 <= 8
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS6-8",aps6_8[1], exp2 );
     }
     if aca == 4 && exp2 >= 8 && exp2 <= 10
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level EL1 8-10",el8_10[1], exp2 );
     }
     if aca == 5 && exp2 >= 10 && exp2 <= 13
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level EL10-13",el10_13[1], exp2 );
     } 
     if aca == 6 && exp2 > 13
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level SES",ses[1], exp2 );
     }
     }

     if prof == 3 {
          let mut law = String::new();
         println!("Please enter your position...
        Key:
        1: Paralegal
        2: Junior Associate
        3: Associate
        4: Senior Associate 1-2
        5: Senior Associate 3-4
        6: Partner\n\n");
     io::stdin().read_line(&mut law).expect("Not a valid input");
     let law:i32 = law.trim().parse().expect("Not a specified number");

     let mut exp3 = String::new();
     println!("How many years of experience do you have?");
     io::stdin().read_line(&mut exp3).expect("Not a valid input");
     let exp3:i32 = exp3.trim().parse().expect("Not a specified number");

     if law == 1 && exp3 > 0 && exp3 <= 2  
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS1-2",aps1_2[2], exp3 );
     }
     if law == 2 && exp3 >= 3 && exp3 <= 5
     {
      println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS3-5",aps3_5[2], exp3 );
     }
     if law == 3 && exp3 >= 6 && exp3 <= 8
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS6-8",aps6_8[2], exp3 );
     }
     if law == 4 && exp3 >= 8 && exp3 <= 10
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level EL1 8-10",el8_10[2], exp3 );
     }
     if law == 5 && exp3 >= 10 && exp3 <= 13
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level EL10-13",el10_13[2], exp3 );
     } 
     if law == 6 && exp3 > 13
     {
         println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level SES",ses[2], exp3 );
     }
     }

     if prof == 4 {
        let mut teacher = String::new();
     println!("Please enter your position...
        Key:
        1: Placement
        2: Classroom Teacher
        3: Senior Teacher
        4: Leading Teacher
        5: Deputy Teacher
        6: Principal\n\n");
     io::stdin().read_line(&mut teacher).expect("Not a valid input");
     let teacher:i32 = teacher.trim().parse().expect("Not a specified number");
 
     let mut exp4 = String::new();
     println!("How many years of experience do you have?");
     io::stdin().read_line(&mut exp4).expect("Not a valid input");
     let exp4:i32 = exp4.trim().parse().expect("Not a specified number");

     if teacher == 1 && exp4 > 0 && exp4 <= 2 
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS1-2",aps1_2[3], exp4 );
     }
     if teacher == 2 && exp4 >= 3 && exp4 <= 5
     {
     println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS3-5",aps3_5[3], exp4 );
     }
     if teacher == 3 && exp4 >= 6 && exp4 <= 8
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level APS6-8",aps6_8[3], exp4 );
     }
     if teacher == 4 && exp4 >= 8 && exp4 <= 10
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level EL1 8-10",el8_10[3], exp4 );
     }
     if teacher == 5 && exp4 >= 10 && exp4 <= 13
     {
        println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level EL10-13",el10_13[3], exp4 );
     } 
     if teacher == 6 && exp4 > 13
     {
         println!("Hello
            You are an {}
            You have work experience of {} years
            You are of the level SES",ses[3], exp4 );
     }
     }
     num+=1
  }
}