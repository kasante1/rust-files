


fn main() {

    let m = teststuff("a");
    m

    //     for entry in loopthing{ 
    //         let b : i32 = entry + 1;
    //         let h = b + 10;

    //         if entry % 2 == 0{
    //             if h -1 != 0{
    //                 if h + 1 != 4{
    //                     container.push(entry);
    //                 }
    //             }
    //             container.push(entry);
    //         }
    //        else{
    //         cont.push(entry);
    //     }
    // }

    // print!("{:?}\n",container);
    // print!("----------------\n");
    // print!("{:?}\n",cont);
        

}


fn teststuff(search_text:&str){
let mut container:Vec<&str> = Vec::new();

//let mut cont:Vec<&str> = Vec::new();

let loopthing = "C, an old man of 84 years whose wife had died, employed Mrs M as secretary/companion. From the beginning she occupied a position of trust, and in addition to running the house she took a confidential part in running C’s affairs. From the time of Mrs M’s employment and C’s death (January 1959 – August 1964) he gave her gifts worth £28,000 from his total assets of £40,000.

It was held by the Chancery Division that (1) All the gifts complained of were such as to satisfy the requirements to raise the presumption of undue influence, namely, that they could not be accounted for on the ground of the ordinary motives on which ordinary men act, and secondly, that the relationship between C and Mrs M involved such confidence by C in Mrs M as to place her in a position to exercise undue influence over him. (2) Mrs M failed to discharge the onus on her of establishing that the gifts were only made after ‘full, free and informed discussion’ so as to rebut the presumption of undue influence. The gifts would, therefore, be set aside.";

let text_list:Vec<&str> = loopthing.split(" ").collect();

for i in text_list{
    let h = i.replace('g', "hh");

    let k  = h.to_lowercase();

    if h.contains(search_text){
        if k.len() >= 11{
            container.push(i);
        }
        
    }
    
}
println!("{:?}",container);
}

// use walkdir::WalkDir;
// use distance::*;

// fn main() {

//     let r:&str = "we";

//     let y= "/home/asante/Documents/UG/source2/";

//     let rr = get_items(y,r);

//     print!("{:?}", rr);

// }


// fn get_items<'a>(items_vector:&'a str, search_item:&'a str)->Vec<&'a str>{

//     // let mut directory_container: Vec<DirEntry> = Vec::new();

//     let mut container:Vec<String> = Vec::new();


//     for entry in WalkDir::new(items_vector).into_iter().filter_map(|e| e.ok()) {

//        let entry =  entry.path().display().to_string();

//        container.push(entry);
//     }

//     let mut s:Vec<&str> = Vec::new();
//     s.push("hello");

//     for i in container{
        

//     let remove_slash = &i.replace("/", " ");

//     let split_filename = remove_slash.split(" ");

//     for filenames in split_filename{
//         if filenames.len() >= 3{
//             let filename = filenames.to_lowercase();

//             let similarity_index = levenshtein(search_item, &filename);

//         if similarity_index <= 2 {
//         }
//     }
//     }
// }


//     // let mut g:Vec<&str> = Vec::new();


//     // for items in items_vector{
//     //     if items == search_item{
//     //         g.push(&items);
//     //     }
//     // }
//     // return g;
// }