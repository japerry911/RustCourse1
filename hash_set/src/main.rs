use std::collections::HashSet;

fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("vega");

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("Added");
    } else {
        println!("Not Added");
    }

    greeks.insert("kappa");

    if !greeks.contains("kappa") {
        println!("We don't have kappa");
    } else {
        println!("We have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("removed");
    } else {
        println!("not removed");
    }

    let _1_5:HashSet<_> = (1..=5).collect();
    let _6_10:HashSet<_> = (6..=10).collect();
    let _1_10:HashSet<_> = (1..=10).collect();
    let _2_8:HashSet<_> = (2..=8).collect();

    println!("is {:?} a subset of {:?} => {}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    println!("is NOT {:?} a subset of {:?} => {}", _2_8, _1_10, _1_5.is_disjoint(&_6_10));
    println!("items in either {:?} or {:?} are {:?}", _2_8, _6_10, _2_8.union(&_6_10));
}
