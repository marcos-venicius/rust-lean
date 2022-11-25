pub fn new() {
    // const THREE_HOURS_IN_SECONDS = 60 * 60 * 3; // wrong
    const THREE_HOURS: u16 = 3;
    const THREE_HOURS_IN_MINUTES: u16 = THREE_HOURS * 60;
    const THREE_HOURS_IN_SECONDS: u16 = THREE_HOURS_IN_MINUTES * 60;

    println!("{THREE_HOURS}hrs, {THREE_HOURS_IN_MINUTES}m, {THREE_HOURS_IN_SECONDS}s");
}
