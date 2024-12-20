enum CrabbyState {
    Fighting,
    Defending,
    Collecting(i32),
}

impl CrabbyState {
    fn state_represent(&self) {
        match self {
            CrabbyState::Fighting => println!("Keep Fighting!"),
            CrabbyState::Defending => println!("Good D"),
            CrabbyState::Collecting(num) => println!("We got {}", num),
            _ => println!(),
         }
    }
}

fn main() {
    let fight = CrabbyState::Fighting;
    let defend = CrabbyState::Defending;
    let collect = CrabbyState::Collecting(50);

    fight.state_represent();
    defend.state_represent();
    collect.state_represent();
}
