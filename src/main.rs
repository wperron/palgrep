use std::{fs::File, io::BufRead, io::BufReader, io::Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];

    match args.len() {
        0 => {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            find_palindromes(std::io::stdout(), &mut [&mut handle], &[None]).unwrap()
        }
        n if n == 1 && args[0] == *"-" => {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            find_palindromes(std::io::stdout(), &mut [&mut handle], &[None]).unwrap()
        }
        _ => {
            let mut files: Vec<BufReader<File>> = vec![];
            let mut sources: Vec<Option<&str>> = vec![];

            for arg in args {
                let file = File::open(arg).unwrap();
                files.push(BufReader::new(file));
                sources.push(Some(arg.as_str()));
            }

            find_palindromes(std::io::stdout(), &mut files, &sources).unwrap()
        }
    }
}

/// Finds palindromic strings in `strs` and prints them to `w`.
fn find_palindromes(
    mut w: impl Write,
    readers: &mut [impl BufRead],
    sources: &[Option<&str>],
) -> std::io::Result<()> {
    if readers.len() != sources.len() {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }

    for (i, r) in readers.iter_mut().enumerate() {
        for (j, maybe_line) in r.lines().enumerate() {
            let line = maybe_line?;
            if is_palindrome(line.trim()) {
                match sources[i] {
                    Some(source) => writeln!(w, "{}:{}:{}", source, j, line)?,
                    None => writeln!(w, "{}:{}", j, line)?,
                }
            }
        }
    }
    Ok(())
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().into_iter().collect();
    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_find_palindromes() {
        let mut out: Vec<u8> = vec![];
        find_palindromes(
            &mut out,
            &mut [Cursor::new("foobar".as_bytes())],
            &[Some("test")],
        )
        .unwrap();
        assert_eq!(out.len(), 0);

        let mut out: Vec<u8> = vec![];
        find_palindromes(
            &mut out,
            &mut [Cursor::new("laval\nbabab\ndaaabaaad".as_bytes())],
            &[Some("test")],
        )
        .unwrap();

        let out = std::str::from_utf8(&out).unwrap().lines();
        assert_eq!(out.count(), 3);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("laval"));
        assert!(!is_palindrome("foobar"));
        assert!(is_palindrome(""));
        assert!(is_palindrome("b"));
    }
}
