extern crate sha1;

struct Commit {
    metadata: Vec<u8>,
    message: Vec<u8>,
    prefix: Vec<u8>,
}

impl Commit {
    fn new(metadata: &str, message: &str) -> Commit {
        Commit {
            metadata: string_to_vec(metadata),
            message: string_to_vec(message),
            prefix: Vec::new(),
        }
    }

    fn new_with_prefix(metadata: &str, message: &str, prefix: &str) -> Commit {
        Commit {
            metadata: string_to_vec(metadata),
            message: string_to_vec(message),
            prefix: string_to_vec(prefix),
        }
    }

    fn length(&self) -> usize {
        self.metadata.len() + 2 + self.message.len()
    }

    fn prefix_length(&self, nonce: u64) -> usize {
        self.length() + self.prefix.len() + 1 + base_10_length(nonce) + 1
    }

    fn sha1(&self) -> sha1::Digest {
        let mut m = sha1::Sha1::new();

        m.update(format!("commit {}\0", self.length()).as_bytes());
        m.update(self.metadata.as_slice());
        m.update(b"\n\n");
        m.update(self.message.as_slice());

        return m.digest();
    }

    fn annotate(&self, nonce: u64) -> sha1::Digest {
        let mut m = sha1::Sha1::new();

        m.update(format!("commit {}\0", self.prefix_length(nonce)).as_bytes());
        m.update(self.metadata.as_slice());
        m.update(b"\n");
        m.update(self.prefix.as_slice());
        m.update(format!(" {0}\n\n", nonce).as_bytes());
        m.update(self.message.as_slice());

        return m.digest();
    }
}

fn string_to_vec(string: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(string.as_bytes());
    return bytes;
}

fn base_10_length(n: u64) -> usize {
    // TODO: Something better
    format!("{}", n).as_bytes().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_1() {
        let c = Commit::new("fooo", "barbar");
        assert_eq!(c.length(), 12);
    }

    #[test]
    fn test_prefix_length_1() {
        let c = Commit::new_with_prefix("fooo", "barbar", "spam");
        assert_eq!(c.length(), 12);
        assert_eq!(c.prefix_length(0), c.length() + 4 + 1 + 1 + 1);
        assert_eq!(c.prefix_length(1), c.length() + 4 + 1 + 1 + 1);
        assert_eq!(c.prefix_length(9), c.length() + 4 + 1 + 1 + 1);
        assert_eq!(c.prefix_length(10), c.length() + 4 + 1 + 2 + 1);
        assert_eq!(c.prefix_length(11), c.length() + 4 + 1 + 2 + 1);
    }

    #[test]
    fn test_sha1_1() {
        let c = Commit::new(
            "tree 3a52ea9c086dae34c11faa2822d59fca1170de79
author Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526705189 +0200
committer Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526705189 +0200",
            "Calculate length of Commits\n",
        );

        let exp = "dfae4d199157e7f5c6b2f81cddb102215db12fa3";
        assert_eq!(c.sha1().to_string(), exp);
    }

    #[test]
    fn test_annotate_1() {
        let c = Commit::new_with_prefix(
            "tree 4ea62912d025c113066dab31e6135bd76277af91
parent dfae4d199157e7f5c6b2f81cddb102215db12fa3
author Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526714241 +0200
committer Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526714241 +0200",
            "Calculate sha1 of commits\n",
            "gthm-id",
        );

        let exp = "ac7569d5798d67bad1b80d8aa43245aca8b5fdec";
        assert_eq!(c.annotate(100).to_string(), exp);
    }
}

fn main() {
    println!("Hello, world!");
}
