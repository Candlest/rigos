use std::{
    fs::{self, read_to_string},
    path,
};

use prettytable::{Table, row, Row, Cell};
use serde::{Deserialize, Serialize};
use tera::Tera;
use toml::value::Datetime;

use crate::utils::{self, get_folder_list, get_path_list, info, read_markdown, Config, Info};

#[derive(Clone)]
pub struct Builder {
    pub config: Config,
    project_root_path: String,
    posts_index: Vec<HpropertyString>,
    tags: Vec<String>,
    categroies: Vec<String>,
    pub pub_folders: Vec<String>,
}

impl Builder {
    pub fn new(project_path: String) -> Builder {
        Builder {
            config: Config::new(format!("{}/config.toml", project_path)),
            project_root_path: project_path,
            posts_index: Vec::new(),
            tags: Vec::new(),
            categroies: Vec::new(),
            pub_folders: Vec::new(),
        }
    }
    pub fn check_pub(&mut self) {
        self.pub_folders = get_folder_list(&self.config.public_dir);
        let mut table = Table::new();
        table.add_row(row!["Folder"]);
        for p in self.pub_folders.iter() {
            let pat = path::Path::new(p)
                .file_stem()
                .expect("cannot get file name")
                .to_str()
                .unwrap();
            table.add_row(row![pat]);
        }
        table.printstd();
    }
    pub fn pre_create_posts_index(&mut self) {
        let getfilelist: Vec<String> = get_path_list(&self.config.source_dir);
        for p in getfilelist.iter() {
            if p.ends_with("markdown") || p.ends_with("md") {
                // utils::info(utils::Info::GENERATE, "add", p);
                let hp: HProperty = toml::from_str(read_markdown(p).0.as_str())
                    .expect("cannot read toml of markdown");
                let hps = HpropertyString::new(hp.clone());
                for ite in hp.tags {
                    if !self.tags.contains(&ite) {
                        self.tags.push(ite);
                    }
                }
                if !self.categroies.contains(&hp.category) {
                    self.categroies.push(hp.category);
                }
                self.posts_index.push(hps);
            }
        }
        //utils::info(utils::Info::GENERATE, "add", &p);
    }
    pub fn clear(&self) {
        for folder in self.pub_folders.clone() {
            if folder != "public" {
                fs::remove_dir_all(folder).expect("clear error");
            }
        }
        let getfilelist: Vec<String> = get_path_list(&self.config.public_dir);
        for p in getfilelist.iter() {
            if !p.contains(".git"){
                fs::remove_file(p).expect("clear error");
            }
        }
    }
    pub fn generate_html(&self, item: HType, filename: String) -> Row {
        let source_file = filename.clone();
        let tera = match Tera::new(
            format!(
                "{}/{}/{}/*.html",
                self.project_root_path, self.config.template_dir, self.config.theme
            )
            .as_str(),
        ) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let mut context = tera::Context::new();

        // eprintln!(
        //     "{}/{}/{}.md",
        //     self.project_root_path, self.config.source_dir, filename
        // );
        // eprintln!(
        //     "{}/{}/{}.md",
        //     self.project_root_path, self.config.page_dir, filename
        // );
        let mut toml_source = String::new();
        let mut body_html = String::new();
        match item {
            HType::Post => {
                (toml_source, body_html) = read_markdown(
                    format!(
                        "{}/{}/{}.md",
                        self.project_root_path, self.config.source_dir, filename
                    )
                    .as_str(),
                );
            }
            HType::Page => {
                (toml_source, body_html) = read_markdown(
                    format!(
                        "{}/{}/{}.md",
                        self.project_root_path, self.config.page_dir, filename
                    )
                    .as_str(),
                )
            }
        }
        let property: HProperty =
            toml::from_str(&toml_source).expect("cannot read toml of markdown");

        let mut html_output_path = String::new();

        let htype_str = String::from(match item {
            HType::Page => {
                html_output_path = format!(
                    "{}/{}.html",
                    self.config.public_dir, property.url_name
                );
                if self.config.page_templates.contains(&filename) {
                    filename
                } else {
                    "page".to_string()
                }
            }
            HType::Post => {
                html_output_path = format!(
                    "{}/Post/{}.html",
                    self.config.public_dir, property.url_name
                );
                "post".to_string()
            }
        });

        let public_file = html_output_path.clone();

        let hps = HpropertyString::new(property);
        //let page:  = toml::from_str(&toml_info).unwrap();
        //dbg!(post);
        context.insert("body", body_html.as_str());
        context.insert("property", &hps);
        context.insert("post_index", &self.posts_index);
        context.insert("tags", &self.tags);
        context.insert("categories", &self.categroies);
        //render
        let rendered = tera
            .render(format!("{}.html", htype_str).as_str(), &context)
            .unwrap();
        //let folder = std::path::Path::new(html_output_path.as_str());
        //let _ = std::fs::create_dir_all(folder);
        std::fs::write(html_output_path, rendered).unwrap();

        row![source_file, public_file]
    }
    pub fn build_static_dir(&self) {
        /*Static DIR */
        let getfilelist: Vec<String> = get_path_list(&self.config.static_dir);
        for p in getfilelist.iter() {
            //utils::info(utils::Info::GENERATE, "copying", &p);
            let stfile_path = std::path::Path::new(p);
            let (st_file, st_file_dir) = utils::path_root2pub(&p);
            std::fs::create_dir_all(std::path::Path::new(st_file_dir.as_str())).unwrap(); //UNWRAP
            match std::fs::copy(stfile_path, std::path::Path::new(st_file.as_str())) {
                Ok(_) => (),//utils::info(utils::Info::GENERATE, "copied", &p),
                Err(_) => utils::info(utils::Info::GENERATE, "failed to copy", ""),
            }
        }

        let theme_static = format!("{}/{}/static", self.config.template_dir, self.config.theme);
        let getfilelist: Vec<String> = get_path_list(theme_static.as_str());
        for p in getfilelist.iter() {
            //utils::info(utils::Info::GENERATE, "copying", &p);
            let stfile_path = std::path::Path::new(p);
            let filename = stfile_path.file_name().expect("no file name");
            let result = &p[theme_static.len()..p.len() - &filename.len()];

            let result = format!(
                "{}/{}",
                self.config.public_dir,
                result,
            );

            std::fs::create_dir_all(std::path::Path::new(result.as_str())).unwrap(); //UNWRAP

            let result = format!("{}{}", result, filename.to_str().unwrap());

            match std::fs::copy(stfile_path, std::path::Path::new(result.as_str())) {
                Ok(_) => (),//utils::info(utils::Info::GENERATE, "copied", &result),
                Err(_) => utils::info(utils::Info::GENERATE, "failed to copy", ""),
            }
        }
    }
    pub fn build_all(&self) {
        fs::create_dir_all(format!("{}/Post", self.config.public_dir)).expect("");
        let getfilelist: Vec<String> = get_path_list(&self.config.page_dir);
        let mut table = Table::new();
        table.add_row(row!["Page", "Public"]);
        for p in getfilelist.iter() {
            if p.ends_with("markdown") || p.ends_with("md") {
                let pat = path::Path::new(p)
                    .file_stem()
                    .expect("cannot get file name")
                    .to_str()
                    .unwrap();
                
                table.add_row(self.generate_html(HType::Page, pat.to_string()));
            }
        }
        table.printstd();
        let mut table = Table::new();
        table.add_row(row!["Post", "Public"]);
        let getfilelist: Vec<String> = get_path_list(&self.config.source_dir);
        for p in getfilelist.iter() {
            if p.ends_with("markdown") || p.ends_with("md") {
                let pat = path::Path::new(p)
                    .file_stem()
                    .expect("cannot get file name")
                    .to_str()
                    .unwrap();
                table.add_row(self.generate_html(HType::Post, pat.to_string()));
            }
        }
        table.printstd();
    }
}

pub enum HType {
    Post,
    Page,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HProperty {
    pub title: String,
    pub datetime: Datetime,
    pub tags: Vec<String>,
    pub category: String,
    pub url_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HpropertyString {
    pub title: String,
    pub datetime: String,
    pub tags: Vec<String>,
    pub category: String,
    pub url_name: String,
}

impl HProperty {
    pub fn new(p: String) -> HProperty {
        toml::from_str(read_to_string(p).expect("cannot read file").as_str())
            .expect("cannot read toml of markdown")
    }
}

impl HpropertyString {
    pub fn new(hp: HProperty) -> HpropertyString {
        HpropertyString {
            title: hp.title,
            datetime: hp.datetime.to_string(),
            tags: hp.tags,
            category: hp.category,
            url_name: format!("{}.html", hp.url_name),
        }
    }
}
