extern crate sha1;

struct Commit {
        metadata: Vec<u8>,
        message: Vec<u8>,
}

impl Commit {
    fn length(&self) -> usize {
        return self.metadata.len() + 2 + self.message.len();
    }

    fn sha1(&self) -> sha1::Digest {
        let mut m = sha1::Sha1::new();

        m.update(format!("commit {}\0", self.length()).as_bytes());
        m.update(self.metadata.as_slice());
        m.update(b"\n\n");
        m.update(self.message.as_slice());

        return m.digest();
    }

    fn annotate(&self, prefix: &str, nonce: u64) -> sha1::Digest {
        let mut m = sha1::Sha1::new();

        let pf = format!("\n{0} {1}", prefix, nonce);
        let pfb = pf.as_bytes();

        m.update(format!("commit {}\0", self.length() + pfb.len()).as_bytes());
        m.update(self.metadata.as_slice());
        m.update(pfb);
        m.update(b"\n\n");
        m.update(self.message.as_slice());

        return m.digest();
    }
}

fn string_to_vec(string: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(string.as_bytes());
    return bytes;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_1() {
        let c = Commit {
            metadata: vec![0, 1, 2, 3],
            message: vec![4, 5, 6, 7, 8, 9],
        };
        assert_eq!(c.length(), 12);
    }

    #[test]
    fn test_sha1_1() {
        let c = Commit {
            metadata: string_to_vec("tree 3a52ea9c086dae34c11faa2822d59fca1170de79
author Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526705189 +0200
committer Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526705189 +0200"),
            message: string_to_vec("Calculate length of Commits\n"),
        };

        let exp = "dfae4d199157e7f5c6b2f81cddb102215db12fa3";
        assert_eq!(c.sha1().to_string(), exp);
    }

    #[test]
    fn test_annotate_1() {
        let c = Commit {
            metadata: string_to_vec("tree 4ea62912d025c113066dab31e6135bd76277af91
parent dfae4d199157e7f5c6b2f81cddb102215db12fa3
author Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526714241 +0200
committer Gunnar Þór Magnússon <gunnar.magnusson@booking.com> 1526714241 +0200"),
            message: string_to_vec("Calculate sha1 of commits\n"),
        };

        let exp = "ac7569d5798d67bad1b80d8aa43245aca8b5fdec";
        assert_eq!(c.annotate("gthm-id", 100).to_string(), exp);
    }
}

fn main() {
    println!("Hello, world!");
}
