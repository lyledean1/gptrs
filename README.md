# gptrs

*Note: This is still WIP and needs alot more work to make this useful, built this in a few hours as a small demonstration*

This takes the [OpenAI API](https://platform.openai.com/) and uses it as a cli thats built in Rust to generate code or make suggestions.

This is mainly a small project to help myself learn Rust and get familiar with the OpenAI API, some of the initial code was generated using the help of ChatGPT!


### Code Completion with A File and Prompt


Using a file (see examples in the example folder taken from the developer documentation), and also include a prompt. The prompt will be added to the top of the file to help direct chatgpt

```
gtprs completion --file=<PATH/TO/FILE> --prompt=<PROMPT>
```

i.e in this example below, I loaded the file in /example/example.go and added a prompt for it to rewrite this code in Python

<img width="375" alt="Screenshot 2023-03-05 at 21 15 04" src="https://user-images.githubusercontent.com/20296911/222988385-8f4ec20a-221c-416b-82ef-5d6fdf75f29f.png">


### Prompt Only Code Completion

```
gtprs completion --prompt=<PROMPT>
```

<img width="937" alt="Screenshot 2023-03-05 at 20 06 48" src="https://user-images.githubusercontent.com/20296911/222983863-05122116-43af-4146-8db3-0d348d395d9e.png">

### Configurable API

Max Tokens: Depending on the model, this can vary from 1-2500, some models offer up to 8000 (codex)
```
--max-tokens=3000
```

Temperature: Effects the randomness of the model, value from 0-1 as a decimal
```
--temperature=0.7
```

Model: Selection of models to use, 'code-davinci-002' is the default, currently can add "text-davinci-003", "code-davinci-002" or "code-cushman-001".

See https://platform.openai.com/docs/models

```
--model="code-davinci-002"
```

### Chat 

*To add WIP*

# Setup

This assumes you have an [OpenAI Developer Account](https://platform.openai.com/)

### Generate API Token 

[See here for instructions](https://platform.openai.com/account/api-keys)

After generating the token, set the env variable 
```
export OPENAI_API_KEY={API_KEY}
```

### Install via Cargo 

Via local repo
```
cargo install --path=/path/to/repo
```

Via github
```
cargo install --git https://github.com/lyledean1/gptrs
```
