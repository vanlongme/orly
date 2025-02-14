use askama::Template;

use crate::models::{Author, Subject};

mod filters {
    pub fn to_id(s: &str) -> ::askama::Result<String> {
        Ok(s.replace(".", "_").replace("/", "_"))
    }
}

#[derive(Template)]
#[template(path = "chapter.xhtml", escape = "xml")]
pub struct ChapterXhtml<'a> {
    pub styles: &'a Vec<&'a String>,
    pub body: &'a str,
    pub should_support_kindle: bool,
}

#[derive(Template)]
#[template(path = "container.xml")]
pub struct ContainerXml;

#[derive(Template)]
#[template(path = "ibooks.xml")]
pub struct IbooksXml;

#[derive(Template)]
#[template(path = "navpoint.xml")]
pub struct NavPoint<'a> {
    pub id: &'a str,
    pub order: usize,
    pub label: &'a str,
    pub url: &'a str,
    pub children: Vec<NavPoint<'a>>,
}

#[derive(Template)]
#[template(path = "toc.xml")]
pub struct Toc<'a> {
    pub uid: &'a str,
    pub depth: usize,
    pub pagecount: usize,
    pub title: &'a str,
    pub author: &'a str,
    pub navpoints: &'a Vec<NavPoint<'a>>,
}

#[derive(Template)]
#[template(path = "content.xml")]
pub struct ContentOpf<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub publishers: &'a str,
    pub rights: &'a str,
    pub issued: &'a str,
    pub language: &'a str,
    pub isbn: &'a str,
    pub cover_image: &'a str,
    pub authors: &'a Vec<Author>,
    pub subjects: &'a Vec<Subject>,
    pub styles: &'a Vec<&'a String>,
    pub chapters: &'a Vec<&'a str>,
    pub images: &'a Vec<(&'a str, &'a str)>,
}
