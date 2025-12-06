use advent_of_code_2025::Day;

mod days;

fn main() {
    let day_one = days::one::DayOne::new();
    day_one.solve();

    let day_two = days::two::DayTwo::new();
    day_two.solve();

    let day_three = days::three::DayThree::new();
    day_three.solve();

    let day_four = days::four::DayFour::new();
    day_four.solve();
}
