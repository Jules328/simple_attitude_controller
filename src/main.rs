// pull in supporting files
mod attitude;
mod user_interface;

use attitude::Attitude;
use user_interface::UserInput;

fn main() {
    // Initialize attitude
    let mut attitude = Attitude::new();

    // Welcome message
    user_interface::welcome_message();

    // Main loop
    loop {
        // Get user input
        if let UserInput::Input(att) = user_interface::get_user_input() {
            attitude += att; // add the input attitude to the current attitude
        } else {
            // end program on input
            println!("Quitting");
            break;
        }

        // Check what planet the satellite is pointing at
        let planet = attitude.get_planet();

        // Display Output
        println!("Pointing Towards: {planet} at {attitude}");
    }

    // Goodbye message
    user_interface::goodbye_message();
}
