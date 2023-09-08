pub(crate) trait Execute {
    fn execute(&self, api: wikijs::Api);
}
