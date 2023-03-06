# gptrs a OpenAI Code Generation cli

This takes the [OpenAI API](https://platform.openai.com/) and uses it as a cli to generate code into a file.

This is mainly a small project to help myself learn Rust and get familiar with the OpenAI API, some of the initial code was generated using the help of ChatGPT!

## Code Completion with A File and Prompt

```
gptrs completion --prompt=<PROMPT> --file=<PATH/TO/FILE> --output=</PATH/TO/OUTPUT>
```

### Example

I ran the command to generate a Python file with a fibonacci sequence as following

```
gptrs completion --prompt="generate a fibonacci sequence in Python" --output=./fibonacci.py
```

Which generated this file in Python:

<img width="343" alt="Screenshot 2023-03-06 at 21 45 50" src="https://user-images.githubusercontent.com/20296911/223241097-69448416-5457-4a77-9403-1c6ca4d70840.png">

I then ran a command to refactor the code in this file and output a new file below:

<img width="332" alt="Screenshot 2023-03-06 at 21 46 02" src="https://user-images.githubusercontent.com/20296911/223241154-b21f7e08-a103-4148-9479-7ffa8933e257.png">

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
