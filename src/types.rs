use chrono::NaiveDate;

pub trait Blog {
    fn get_title(&self) -> String;
    fn get_date_listed(&self) -> NaiveDate;
    fn get_description(&self) -> Option<String>;
    fn get_html(&self) -> String;
    fn get_full_slug(&self) -> String; // This would be `2024-03-19/my-blog`
    fn get_part_slug(&self) -> String; // This would be `my-blog`
    fn get_tags(&self) -> Vec<String>;
    fn get_table_of_contents(&self) -> Option<String>;
    fn get_keywords(&self) -> Option<Vec<String>>;
    fn get_canonicle_link(&self) -> Option<String>;
    fn get_author_name(&self) -> Option<String>; // TODO: Maybe support authors?
    fn get_author_webpage(&self) -> Option<String>; // TODO: again, support multiple authors?
    fn get_preview(&self) -> String;
    fn get_last_modified(&self) -> Option<NaiveDate>;
    fn get_priority(&self) -> Option<f64>;
}
