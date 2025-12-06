pub trait Day<T: std::fmt::Display + std::fmt::Debug> {
    fn part1(&self) -> T;
    fn part2(&self) -> T;

    fn solve(&self) {
        println!("[{}] Part 1: {:?}", Self::NAME, self.part1());
        println!("[{}] Part 2: {:?}", Self::NAME, self.part2());
    }

    const NAME: &'static str;
}
