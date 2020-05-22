#[derive(Debug)]
enum Platform {
    Windows,
    Linux,
    Macos,
}

impl Platform {
    fn parse(platform_args: &str) -> Platform {
        if platform_args == "windows" {
            Platform::Windows
        } else if platform_args == "linux" {
            Platform::Linux
        } else if platform_args == "macos" {
            Platform::Macos
        } else {
            panic!("unknown platform {}", platform_args);
        }
    }
}

fn main() {
    let platform_arg = "orange";
    let platform = Platform::parse(platform_arg);
    println!("Producing output for {:?}", platform)
}
