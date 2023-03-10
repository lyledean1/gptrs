use text_colorizer::*;

pub trait Output {
    fn get_output(&self) -> String;

    fn to_file(&self, path: String) {
        std::fs::write(path, self.get_output()).expect("Unable to write file")
    }

    fn to_cli(&self) {
        println!("{}", self.get_output().green())
    }

    fn parse(&self, output_path: String) {
        if output_path != "" {
            self.to_file(output_path)
        } else {
            self.to_cli()
        }
    }
}
