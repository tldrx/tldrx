use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind, stdin};
use std::path::PathBuf;

use anyhow::{Result, Context};
use crossterm::style::{Color, Stylize, Attribute, ContentStyle};
use crossterm::tty::IsTty;

use crate::config::{Config, StyledChoice};
use crate::platform::Platform;


#[derive(Debug)]
pub(crate) struct Page<'a> {
    pub file: PathBuf,
    pub kind: Kind,
    pub platform: Platform,
    pub config: &'a Config,
}

impl<'a> Page<'a> {
    #[inline]
    pub fn option_from(file: PathBuf, kind: Kind, platform: Platform, config: &'a Config) -> Option<Self> {
        if !file.is_file() {
            return None;
        }
        Some(Self {
            file,
            kind,
            platform,
            config,
        })
    }

    fn parse(&self) -> Result<Vec<Line>> {
        let file = File::open(&self.file).with_context(||
            format!("Failed to open: {}", self.file.display())
        )?;
        let mut line_iter = BufReader::new(file).lines().enumerate();

        // dorp the first line as its almost the same as what has inputed
        let first_line = line_iter.next().map(|(_, l)| l).unwrap_or(
            Err(Error::new(ErrorKind::Other, "First line must not blank!"))
        ).with_context(||
            format!("Failed to parse line [{}] for page: {}", 1, self.file.display())
        )?;

        // v2 format: drop setext headding notation line ===
        // https://github.com/tldr-pages/tldr/pull/958
        if !first_line.starts_with('#') {
            line_iter.next();
        }

        // 64 line should cover most cases.
        let mut lines: Vec<Line>= Vec::with_capacity(64);
        while let Some((num, line)) = line_iter.next() {
            let line = line.with_context(||
                format!("Failed to parse line [{}] for page: {}", num, self.file.display())
            )?;
            lines.push(Line::from(&(line)))
        }
        Ok(lines)
    }

    pub fn render(&self) -> Result<()> {
        match self.config.styled {
            StyledChoice::Auto if stdin().is_tty() => self.render_styled(),
            StyledChoice::On => self.render_styled(),
            _ => self.render_styless(),
        }
    }

    fn render_styled(&self) -> Result<()> {
        // OPTIMIZE: maybe introduce a style customize feature and put it there
        let normal_style = ContentStyle::new().with(Color::Green).attribute(Attribute::Bold);
        let token_style = ContentStyle::new().with(Color::Cyan).attribute(Attribute::Bold);
        let text_style = ContentStyle::new().with(Color::Blue).attribute(Attribute::Bold);
        let blockquote_style = ContentStyle::new().with(Color::Grey);
        let headding_style = ContentStyle::new().attribute(Attribute::Bold);
        let meta_style = ContentStyle::new().attribute(Attribute::Bold).with(
            match self.kind {
                Kind::Official => Color::Green,
                Kind::Private => Color::Red,
            }
        );

        let column = 80 - 2;
        let meta = format!("{:-^column$}", format!(" : {} :: {} : ", self.kind, self.platform));
        println!("\n  {}", meta_style.apply(meta));
        for line in self.parse()? {
            match line {
                Line::Blank => {
                    println!()
                },
                Line::Code(s) => {
                    print!("    ");
                    parse_code(&s, |segment| match segment {
                        Segment::Normal(c) => print!("{}", normal_style.apply(c)),
                        Segment::Token(c) => print!("{}", token_style.apply(c)),
                    });
                    println!();
                }
                Line::Text(s) => {
                    println!("  {}", text_style.apply(s))
                }
                Line::Headding(s) => {
                    println!("  {}", headding_style.apply(s))
                }
                Line::Blockquote(s) => {
                    println!("  {}", blockquote_style.apply(s))
                }
            }
        }
        println!();
        Ok(())
    }

    fn render_styless(&self) -> Result<()> {
        println!("  : {} :: {} :", self.kind, self.platform);
        for line in self.parse()? {
            match line {
                Line::Headding(s) => {
                    println!("  {}", s);
                }
                Line::Blockquote(s) => {
                    println!("  {}", s)
                }
                Line::Code(s) => {
                    println!("    {}", s);
                }
                Line::Text(s) => {
                    println!("  {}", s)
                }
                Line::Blank => println!(),
            }
        }
        println!();
        Ok(())
    }
}


#[derive(Debug, Clone, Copy)]
pub(crate) enum Kind {
    Official,
    Private,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Kind::Official => "official",
            Kind::Private => "private",
        };
        write!(f, "{}", text)
    }
}


#[derive(Debug)]
pub(crate) enum Line {
    Headding(String),
    Blank,
    Blockquote(String),
    Text(String),
    Code(String),
}

impl Line {
    pub fn from(line: &str) -> Self {
        match line.bytes().next() {
            None => Self::Blank,
            Some(b' ') => Self::Code(
                line.trim_start_matches(char::is_whitespace).into()
            ),
            Some(b'`') => Self::Code({
                // private pages might not strictly linted.
                let line = line.trim_end();
                line[1..(line.len() - 1)].into()
            }),
            Some(b'-') => Self::Text(
                line.trim_start_matches(|c: char| c == '-' || c.is_whitespace()).into()
            ),
            Some(b'>') => Self::Blockquote(
                line.trim_start_matches(|c: char| c == '>' || c.is_whitespace()).into()
            ),
            Some(b'#') => Self::Headding(
                line.trim_start_matches(|c: char| c == '#' || c.is_whitespace()).into()
            ),
            Some(_) => Self::Text(
                line.into()
            ),
        }
    }
}


#[derive(Debug, PartialEq)]
pub(crate) enum Segment<'a> {
    Normal(&'a str),
    Token(&'a str),
}

pub(crate) fn parse_code<'a, F>(
    code: &'a str,
    mut consumer: F
)
where
    F: FnMut(Segment<'a>) -> ()
{
    let bytes = code.as_bytes();
    let (mut i, mut start, mut open, len) = (1, 0, false, bytes.len());
    while i < len {
        if !open && bytes[i - 1] == b'{' && bytes[i] == b'{' {
            consumer(Segment::Normal(&code[start..i - 1]));
            start = i + 1;
            open = true;
        } else if open && bytes[i - 1] == b'}' && bytes[i] == b'}' {
            while i < len - 1 && bytes[i + 1] == b'}' {
                i += 1;
            }
            consumer(Segment::Token(&code[start..i - 1]));
            start = i + 1;
            open = false;
        }
        i += 1;
    }
    if start < len {
        if open {
            // no closing }} match, two step back to include {{
            start -= 2;
        }
        consumer(Segment::Normal(&code[start..len]))
    }
}




#[cfg(test)]
mod test {
    use super::{*, Segment::*};

    fn run_parse_code<'a>(code: &'a str) -> Vec<Segment<'a>> {
        let mut segments = Vec::new();
        parse_code(code, |s| segments.push(s));
        segments
    }

    #[test]
    fn test_parse_code_general() {
        assert_eq!(
            run_parse_code("git -c '{{config.key}}={{value}}' {{subcommand}}"),
            vec![
                Normal("git -c '"),
                Token("config.key"),
                Normal("="),
                Token("value"),
                Normal("' "),
                Token("subcommand"),
            ]
        );

        assert_eq!(
            run_parse_code("git pull"),
            vec![Normal("git pull")]
        );
    }

    #[test]
    fn test_parse_code_curly_brackets() {
        assert_eq!(
            run_parse_code("git commit -m {{{{message}}}}"),
            vec![
                Normal("git commit -m "),
                Token("{{message}}"),
            ]
        );

        assert_eq!(
            run_parse_code("git commit -m {{{message}}}"),
            vec![
                Normal("git commit -m "),
                Token("{message}"),
            ]
        );

        assert_eq!(
            run_parse_code("git commit -m {{message"),
            vec![
                Normal("git commit -m "),
                Normal("{{message")
            ]
        );

        assert_eq!(
            run_parse_code("git commit -m message}}"),
            vec![
                Normal("git commit -m message}}"),
            ]
        );

        assert_eq!(
            run_parse_code("git commit -m {message}"),
            vec![
                Normal("git commit -m {message}"),
            ]
        );
    }

    #[test]
    fn test_parse_code_i18n() {
        assert_eq!(
            run_parse_code("git cherry-pick {{start_commit}}~..{{end_commit}}"),
            vec![
                Normal("git cherry-pick "),
                Token("start_commit"),
                Normal("~.."),
                Token("end_commit")
            ]
        );

        assert_eq!(
            run_parse_code("git cherry-pick {{开始提交}}~..{{结束提交}}"),
            vec![
                Normal("git cherry-pick "),
                Token("开始提交"),
                Normal("~.."),
                Token("结束提交")
            ]
        );

        assert_eq!(
            run_parse_code("git cherry-pick {{শুরুর_কমিট}}~..{{শেষের_কমিট}}"),
            vec![
                Normal("git cherry-pick "),
                Token("শুরুর_কমিট"),
                Normal("~.."),
                Token("শেষের_কমিট")
            ]
        );

    }
}
