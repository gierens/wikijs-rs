pub(crate) trait Execute {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn std::error::Error>>;
}
