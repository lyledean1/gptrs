#[allow(dead_code)]

pub enum Models {
    TextDavinci003,
    CodeDavinci002,
    CodeCushman001,
    Gpt35Turbo,
}

//TODO: update
impl Models {
    pub fn name(&self) -> &str {
        match *self {
            Models::Gpt35Turbo => "gpt-3.5-turbo",
            Models::TextDavinci003 => "text-davinci-003",
            Models::CodeDavinci002 => "code-davinci-002",
            Models::CodeCushman001 => "code-cushman-003",
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match *self {
            Models::Gpt35Turbo => "Most capable GPT-3.5 model and optimized for chat at 1/10th the cost of text-davinci-003. Will be updated with our latest model iteration.",
            Models::TextDavinci003 => "Can do any language task with better quality, longer output, and consistent instruction-following than the curie, babbage, or ada models. Also supports inserting completions within text.",
            Models::CodeDavinci002 => "DaVinci code generation model, version 002",
            Models::CodeCushman001 => "Cushman code generation model, version 001",
        }
    }

    #[allow(dead_code)]
    pub fn max_tokens(&self) -> i32 {
        match *self {
            Models::Gpt35Turbo => 4096,
            Models::TextDavinci003 => 4093,
            Models::CodeDavinci002 => 4093,
            Models::CodeCushman001 => 4093,
        }
    }
    
    #[allow(dead_code)]
    pub fn training_data(&self) -> &str {
        match *self {
            Models::Gpt35Turbo => "Up to Sep 2021",
            Models::TextDavinci003 => "Up to Sep 2021",
            Models::CodeDavinci002 => "Up to Sep 2021",
            Models::CodeCushman001 => "Up to Sep 2021",
        }
    }
}
