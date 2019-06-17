use std::{
    env,
    io::{
        Read,
        Write,
        BufWriter,
        BufReader,
        stdin,
        stdout
    },
    error::Error,
};

mod peek_while;
use peek_while::*;

pub fn env_subst<W: Write, R: Read>(reader: R, writer: W) -> Result<usize, Box<dyn Error>> {
    let mut written: usize = 0;

    let mut writer = BufWriter::new(writer);
    let mut bytes = BufReader::new(reader)
        .bytes()
        .filter_map(Result::ok)
        .peekable();

    while let Some(byte) = bytes.next() {
        if byte == b'$' {
            let key: String = bytes.by_ref()
                .peek_while(|&b| b.is_ascii_alphanumeric() || b == b'_')
                .map(|c| c as char)
                .collect();

            written += writer.write(env::var(key)?.as_bytes())?;
        } else {
            written += writer.write(&[byte])?;
        }
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

    #[test]
    fn it_works() {
        let reader: Vec<u8> = "Blee blah $HOME eeell".into();
        let mut writer: Vec<u8> = Vec::new();

        let home = env::var("HOME").unwrap();
        let expected: Vec<u8> =  format!("Blee blah {} eeell", home).into();
        
        env_subst(&reader[..], &mut writer);
        
        let output = writer; 

        assert_eq!(output, expected);
    }
}
