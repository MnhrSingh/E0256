
extern crate chrono;
use chrono::{Datelike, Timelike, Utc};
//use chrono::{DateTime, Utc};



fn main() {

/////////////////////    UTC (coordinated universal time)  FORMAT   /////////////////  

    let curr_tm = Utc::now();
   
     
    let (is_pm, hour) = curr_tm.hour12();
    println!(
        "now the UTC time is {:02}:{:02}:{:02} {}",
        hour,
        curr_tm.minute(),
        curr_tm.second(),
        if is_pm { "PM" } else { "AM" }
    );
    let (is_common_era, year) = curr_tm.year_ce();
    println!(
        "now the UTC date is {}-{:02}-{:02} {:?} \nTHE ERA OF CURRENT TIME IS....({})",
        year,
        curr_tm.month(),
        curr_tm.day(),
        curr_tm.weekday(),
       // println!(".......ERA OF THE CURRENT TIME......."),
        if is_common_era { "CE-----common era or current era" } else { "BCE----before common era" }
    );


   /* let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));

*/

 

}

