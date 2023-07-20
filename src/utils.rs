/* fn info: Output content in a canonical manner*/
pub fn info(inforation : Info, content: &str, remark: &str){
    use colored::Colorize;
    let color_str = match inforation{
        Info::GENERATE => ("[GENERATE]".bold().blue(), remark.green()),
        Info::CLEAR => ("[CLEAR]".bold().blue(), remark.green()),
        Info::RUN => ("[RUN]".bold().blue(), remark.green()),
        Info::ERROR => ("[ERROR]".bold().red(), remark.red()),
    };
    println!("{}\t{}\t{}", color_str.0, content, color_str.1);
}
pub enum Info{
    GENERATE,
    RUN,
    CLEAR,
    ERROR
}

/* Define DIR as constant*/
pub const PUBLIC_DIR: &str = "public";
pub const SOURCE_DIR: &str = "source";
pub const STATIC_DIR: &str = "static";
pub const PAGE_DIR: &str = "page";
pub const TEMPLATE_DIR: &str = "template";

/*PATH */
pub fn path_web2local(web_path: &str) -> String{
    let iter = percent_encoding::percent_decode(web_path.as_bytes());
    let decoded = iter.decode_utf8().unwrap();
    let path_str = format!("{}{}", PUBLIC_DIR, decoded);
    path_str
}

pub fn path_local2web(local_path: &str) -> String{
    let path = std::path::Path::new(local_path);
    let filename = path.file_name().unwrap();
    let result = &local_path[PUBLIC_DIR.len()..local_path.len() - &filename.len()]; /*非常方便！！！ */
    let filename = percent_encoding::utf8_percent_encode(filename.to_str().unwrap(), percent_encoding::NON_ALPHANUMERIC).to_string();
    format!("{}{}", result, filename)
}

pub fn path_root2pub(raw_path: &str) -> (String, String){
    let path = std::path::Path::new(raw_path);
    let filename = path.file_name().unwrap();
    let result = &raw_path[PUBLIC_DIR.len()..raw_path.len() - &filename.len()]; /*非常方便！！！ */
    (format!("{}{}{}",PUBLIC_DIR , result, filename.to_str().unwrap()), format!("{}{}",PUBLIC_DIR , result)) /*/css/xxx.js */
}