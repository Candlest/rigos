/* fn info: Output content in a canonical manner*/
pub fn info(inforation: Info, content: &str, remark: &str) {
    use colored::Colorize;
    let color_str = match inforation {
        Info::GENERATE => ("[GENERATE]".bold().blue(), remark.green()),
        Info::CLEAR => ("[CLEAR]".bold().blue(), remark.green()),
        Info::RUN => ("[RUN]".bold().blue(), remark.green()),
        Info::ERROR => ("[ERROR]".bold().red(), remark.red()),
        Info::INIT => ("[INIT]".bold().blue(), remark.green()),
    };
    println!("{}{}\t\t{}", color_str.0, content, color_str.1);
}
pub enum Info {
    GENERATE,
    RUN,
    CLEAR,
    ERROR,
    INIT
}

/* Define DIR as constant*/
pub const PUBLIC_DIR: &str = "public";
pub const SOURCE_DIR: &str = "source";
pub const STATIC_DIR: &str = "static";
pub const PAGE_DIR: &str = "page";
pub const TEMPLATE_DIR: &str = "template";

/* Thanks to web_server
 * We never use percent_encode again :-)
 */
pub fn path_local2web(local_path: &str) -> String {
    local_path[PUBLIC_DIR.len()..].to_string() /*非常方便！！！ */
}

pub fn path_root2pub(raw_path: &str) -> (String, String) {
    let path = std::path::Path::new(raw_path);
    let filename = path.file_name().unwrap();
    let result = &raw_path[PUBLIC_DIR.len()..raw_path.len() - &filename.len()]; /*非常方便！！！ */
    (
        format!("{}{}{}", PUBLIC_DIR, result, filename.to_str().unwrap()),
        format!("{}{}", PUBLIC_DIR, result),
    ) /*/css/xxx.js */
}
