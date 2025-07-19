use std::fs::File;

#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use super::*;

    #[test]
    fn read_file() -> io::Result<()> {
        let mut buffer = Vec::new();
        let mut f = File::open("./data/biostats.csv")?;

        f.read_to_end(&mut buffer)?;
        for byte in buffer {
            println!("{:b}", byte)
        }
        Ok(())
    }
}
