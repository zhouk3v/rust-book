fn main() {
    //
    // if let is syntatic sugar for a match statement that matches only one value and ignores the rest (askin to a _ catch all that returns a unit value)
    //

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //
    // The below if let code is equivelent to the match expression above
    //
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //
    // We can include an else with an if let too, which is equivelent to the _ catch all
    //
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No max is configured");
    }
}
