fn main() {


    let test1 = "We need more space.";
 
    assert_eq!(trim_spaces(test1), "We need more space.");
    //trim_spaces(test1);
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    //trim_spaces(&test2);
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */

fn trim_spaces(s : &str) -> &str {
    let mut start = 0;

    
    for (index, item) in s.chars().enumerate(){
        if item != ' ' {
            start = index;
            break;
        }
        
    }

    let mut end = 0;

    for (index, item) in s.chars().rev().enumerate(){
        //println!("Reverse : {}" ,item);
        if item !=' '{
            end = s.len() - index;
            break;
        }
    }
    &s[start..end]
    
}