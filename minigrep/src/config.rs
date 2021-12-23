use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: if let Some(t) = args.next() {
                t
            } else {
                return Err("参数不足");
            },
            filename: if let Some(t) = args.next() {
                t
            } else {
                return Err("参数不足");
            },
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename.to_string())?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    let mut i = 0;
    println!("以下为检索结果:");
    for line in results {
        i += 1;
        println!("第{}行: {}", i, line);
    }
    if i == 0 {
        println!("没有在 {} 文件中找到单词 {}", config.filename, config.query);
    } else {
        println!("一共有 {} 行记录", i);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "frog";
        let contents = "I'm nobody!Who are you?
Are you nobody,too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public,like a frog
To tell your name the livelong day
To an admiring bog!
solenovex@xxxx";
        assert_eq!(vec!["How public,like a frog"], search(query, contents));
    }
}
