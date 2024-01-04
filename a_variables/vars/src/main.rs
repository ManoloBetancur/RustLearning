//PART 3
const STARTING_MISSILES : i32 = 20;

const  READY_AMOUNT : i32 = 6;
fn main() {

    //PART  1
    // let mut missiles = 8;
    // let ready = 2;
    // println!("Firing {} of my {} missiles...",ready, missiles);
    // missiles = missiles - ready;

    // println!("{} missiles left", missiles);

    //PART 2
    // let STARTING_MISSILES : i32 = 11;

    // let READY_AMOUNT : i32 = 3;

    // let mut missiles: i32 = STARTING_MISSILES;

    // let ready: i32 = READY_AMOUNT;

    //let (mut missiles,ready) = (STARTING_MISSILES,READY_AMOUNT);
    let (mut missiles,ready): (i32,i32) = (STARTING_MISSILES,READY_AMOUNT);


    println!("Firing {} of my {} missiles...",ready, missiles);
    missiles = missiles - ready;

    println!("{} missiles left", missiles);


}
