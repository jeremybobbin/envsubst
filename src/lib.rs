use std::{
    env,
    io::{
        Read,
        Write,
        BufWriter,
        BufReader,
    },
    error::Error,
};

mod peek_while;
use peek_while::*;

pub fn env_subst<W: Write, R: Read>(reader: R, writer: W) -> Result<usize, Box<dyn Error>> {
    let mut written: usize = 0;

    let mut is_escaped: bool = false;

    let mut writer = BufWriter::new(writer);
    let mut bytes = BufReader::new(reader)
        .bytes()
        .filter_map(Result::ok)
        .peekable();

    loop {
        match bytes.next() {
            Some(byte) if is_escaped => {
                written += writer.write(&[byte])?;
                is_escaped = false;
            },
            Some(b'$') => {
                let bytes = bytes.by_ref();

                let iter: Box<dyn Iterator<Item=u8>> = if let Some(b'{') = bytes.peek() {
                    bytes.next();
                    Box::new(bytes.take_while(|&b: &u8| b != b'}'))
                } else {
                    Box::new(bytes.peek_while(|&b: &u8|  b.is_ascii_alphanumeric() || b == b'_'))
                };

                let key: String = iter
                    .map(|c| c as char)
                    .collect();

                written += writer.write(env::var(key)?.as_bytes())?;
            },
            Some(b'\\') => is_escaped = true,
            Some(byte) => written += writer.write(&[byte])?,
            None => break
        };
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use crate::env_subst;

    use std::{ 
        env,
        io::{

            Cursor,
            Read,
            Write
        }
    };

    fn feed(input: &str, expected: &str) {
        let reader = Cursor::new(input);
        let mut writer: Vec<u8> = Vec::new();

        env_subst(reader, &mut writer);

        let output = String::from_utf8_lossy(&writer[..]);
        assert_eq!(output, expected);
    }

    #[test]
    fn it_works() {
        let input = "Blee blah $HOME eeell";

        let home = env::var("HOME").unwrap();
        let expected =  format!("Blee blah {} eeell", home);

        feed(input, &expected[..]);
    }


    #[test]
    fn curly_braces() {
        let input = "${HOME}";
        let expected = env::var("HOME").unwrap();
        feed(input, &expected[..]);
    }

    #[test]
    fn escapes() {
        let input = "\\$HOME";
        let expected =  "$HOME";

        feed(input, expected);
    }

    #[test]
    fn big_test() {
        let input = "\\$HOME $HOME ${HOME}";

        let home = env::var("HOME").unwrap();
        let expected =  format!("$HOME {} {}", home, home);

        feed(input, &expected[..]);
    }
}
