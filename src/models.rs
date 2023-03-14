#[allow(dead_code)]

pub enum Models {
    Gpt4,
    Gpt432k,
    TextDavinci003,
    TextDavinci002,
    CodeDavinci002,
    CodeCushman001,
    Gpt35Turbo,
}

pub fn get_model(name: &str) -> Models {
    match name {
        "gpt-4" => Models::Gpt4,
        "gpt-4-32k" => Models::Gpt432k,
        "text-davinci-003" => Models::TextDavinci003,
        "text-davinci-002" => Models::TextDavinci002,
        "code-davinci-002" => Models::CodeDavinci002,
        "code-cushman-001" => Models::CodeCushman001,
        _ => Models::Gpt35Turbo,
    }
}
//TODO: update
impl Models {
    pub fn all() -> Vec<Models> {
        vec![
            Models::Gpt4,
            Models::Gpt432k,
            Models::Gpt35Turbo,
            Models::TextDavinci003,
            Models::TextDavinci002,
            Models::CodeDavinci002,
            Models::CodeCushman001,
        ]
    }

    pub fn name(&self) -> &str {
        match *self {
            Models::Gpt4 => "gpt-4",
            Models::Gpt432k => "gpt-4-32k",
            Models::Gpt35Turbo => "gpt-3.5-turbo",
            Models::TextDavinci003 => "text-davinci-003",
            Models::TextDavinci002 => "text-davinci-002",
            Models::CodeDavinci002 => "code-davinci-002",
            Models::CodeCushman001 => "code-cushman-001",
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match *self {
            Models::Gpt4 => "More capable than any GPT-3.5 model, able to do more complex tasks, and optimized for chat. Will be updated with our latest model iteration.",
            Models::Gpt432k => "More capable than any GPT-3.5 model, able to do more complex tasks, and optimized for chat. Will be updated with our latest model iteration.",
            Models::Gpt35Turbo => "Most capable GPT-3.5 model and optimized for chat at 1/10th the cost of text-davinci-003. Will be updated with our latest model iteration.",
            Models::TextDavinci003 => "Can do any language task with better quality, longer output, and consistent instruction-following than the curie, babbage, or ada models. Also supports inserting completions within text.",
            Models::TextDavinci002 => "	Similar capabilities to text-davinci-003 but trained with supervised fine-tuning instead of reinforcement learning",
            Models::CodeDavinci002 => "Optimized for code-completion tasks",
            Models::CodeCushman001 => "Cushman code generation model, version 001",
        }
    }

    #[allow(dead_code)]
    pub fn max_tokens(&self) -> i32 {
        match *self {
            Models::Gpt4 => 8192,
            Models::Gpt432k => 32768,
            Models::Gpt35Turbo => 4096,
            Models::TextDavinci003 => 4093,
            Models::TextDavinci002 => 4097,
            Models::CodeDavinci002 => 4093,
            Models::CodeCushman001 => 4093,
        }
    }

    #[allow(dead_code)]
    pub fn training_data(&self) -> &str {
        match *self {
            Models::Gpt4 => "Up to Sep 2021",
            Models::Gpt432k => "Up to Sep 2021",
            Models::Gpt35Turbo => "Up to Sep 2021",
            Models::TextDavinci003 => "Up to Sep 2021",
            Models::TextDavinci002 => "Up to Sep 2021",
            Models::CodeDavinci002 => "Up to Sep 2021",
            Models::CodeCushman001 => "Up to Sep 2021",
        }
    }
}
