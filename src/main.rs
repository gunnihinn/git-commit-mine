struct Commit {
        metadata: Vec<u8>,
        message: Vec<u8>,
}

impl Commit {
    fn length(&self) -> usize {
        return self.metadata.len() + 2 + self.message.len();
    }
}

#[cfg(test)]
mod tests {
    use Commit;
    #[test]
    fn test_length_1() {
        let c = Commit {
            metadata: vec![0, 1, 2, 3],
            message: vec![4, 5, 6, 7, 8, 9],
        };
        assert_eq!(c.length(), 12);
    }
}

fn main() {
    println!("Hello, world!");
}
