use crate::server::Server;

pub trait Plugin {
    fn init() -> Box<Self>
    where
        Self: Sized;
    fn on_enable(&self, _server: &mut Server) {}
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_version(&self) -> String;
    fn get_authors(&self) -> Vec<String>;
    fn get_homepage(&self) -> Option<String>;
    fn get_repository(&self) -> String;
    fn get_dependencies(&self) -> Vec<String>;
}
