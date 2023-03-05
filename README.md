# gptrs

*Note: This is still WIP and needs alot more work to make this useful, built this in a few hours as a small demonstration*

This takes the [OpenAI API](https://platform.openai.com/) and uses it as a cli thats built in Rust to generate code or make suggestions.

This is mainly a small project to help myself learn Rust and get familiar with the OpenAI API, some of the initial code was generated using the help of ChatGPT!


### Code Completion

Using a prompt 

```
gtprs completion --prompt=<PROMPT>
```

<img width="937" alt="Screenshot 2023-03-05 at 20 06 48" src="https://user-images.githubusercontent.com/20296911/222983863-05122116-43af-4146-8db3-0d348d395d9e.png">

Using a file (see examples in the example folder taken from the developer documentation)

```
gtprs completion --file=<PATH/TO/FILE>
```

<img width="587" alt="Screenshot 2023-03-05 at 20 07 42" src="https://user-images.githubusercontent.com/20296911/222983816-e304e80a-147c-4f46-b662-1e2994f7e7ea.png">


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
