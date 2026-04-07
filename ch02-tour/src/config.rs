use std::env;
pub struct Config {
    pub pattern : String,
    pub filename : String,
    pub case_sensitive : bool,
}
impl Config {
    pub fn new(args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        //todo!("Parse command-line arguments")
        let  args : Vec<String> = args.collect();
        if args.is_empty(){
            return Err("Not enough arguments supplied");
        }
        if args.len() < 3{
             return Err("Not enough arguments provided!. Usage: program <pattern> <filename>");
         }

        let pattern = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            pattern,
            filename,
            case_sensitive
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 🎓 C# COMPARISON: Tests in Rust use #[test] attribute
    // In C#: [Test] or [Fact] (NUnit/xUnit)
    // In Rust: #[test]

    #[test]
    fn test_config_new_with_valid_arguments() {
        // 🎓 C# COMPARISON: Creating a list
        // C#: var args = new List<string> { "exe", "pattern", "file.txt" };
        // Rust: let args = vec![String::from("exe"), ...];
        //
        // FIXED: Correct vec! macro syntax (you had "let args : vec![...]")
        let args = vec![
            String::from("executable_name"),
            String::from("pattern"),
            String::from("filename.txt"),
        ];

        // 🎓 C# COMPARISON: Calling method with IEnumerable
        // C#: var result = Config.CreateFromArgs(args);
        // Rust: Config::new(args.into_iter())
        //
        // FIXED: Use .into_iter() to convert Vec to Iterator (not &args)
        let result = Config::new(args.into_iter());

        // 🎓 Assertions
        // C#: Assert.True(result.IsSuccess);
        // Rust: assert!(result.is_ok());
        assert!(result.is_ok(), "Config::new should succeed with valid arguments");

        let config = result.unwrap();

        // FIXED: Pattern should be "pattern", not "executable_name" (args[0] is program name)
        assert_eq!(config.pattern, "pattern");
        assert_eq!(config.filename, "filename.txt");
        // Note: case_sensitive depends on environment variable at test time
    }

    #[test]
    fn test_config_new_with_insufficient_arguments() {
        // 🎓 C# COMPARISON: Testing error cases
        // C#: Assert.Throws<ArgumentException>(() => Config.Create(args));
        // Rust: assert!(result.is_err());

        let args = vec![
            String::from("executable_name"),
            String::from("only_one_arg"),  // Missing filename!
        ];

        let result = Config::new(args.into_iter());
        assert!(result.is_err(), "Should fail with insufficient arguments");
    }

    #[test]
    fn test_config_new_with_too_many_arguments() {
        // Should work - extra arguments are ignored
        let args = vec![
            String::from("executable_name"),
            String::from("pattern"),
            String::from("filename.txt"),
            String::from("extra_arg"),
            String::from("another_extra"),
        ];

        let result = Config::new(args.into_iter());
        assert!(result.is_ok(), "Should succeed and ignore extra arguments");

        let config = result.unwrap();
        assert_eq!(config.pattern, "pattern");
        assert_eq!(config.filename, "filename.txt");
    }
}
