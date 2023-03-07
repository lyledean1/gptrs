# gptrs 

A OpenAI Code generation cli. 

This sends requests to the [OpenAI API](https://platform.openai.com/), parsers the output and then generates code into a file.

This is mainly a small project to help myself learn Rust and get familiar with the OpenAI API, some of the initial code was generated using the help of ChatGPT!

## Code Completion with A File and Prompt

```
gptrs completion --prompt=<PROMPT> --input=<PATH/TO/FILE> --output=</PATH/TO/OUTPUT>
```

### Example

Run the command to generate a Python file with a Fibonacci sequence as following

```
gptrs completion --prompt="generate a fibonacci sequence in Python" --output=./fibonacci.py
```

Which generates a file in Python:

<img width="343" alt="Screenshot 2023-03-06 at 21 45 50" src="https://user-images.githubusercontent.com/20296911/223241097-69448416-5457-4a77-9403-1c6ca4d70840.png">

Then run a command to refactor the code in this file and output a new file below:

```
gptrs completion --prompt="Refactor this code" --input=./fibonacci.py --output=./fibonacci_refactor.py 
```

<img width="332" alt="Screenshot 2023-03-06 at 21 46 02" src="https://user-images.githubusercontent.com/20296911/223241154-b21f7e08-a103-4148-9479-7ffa8933e257.png">

You can also set different models and temperatures 
```
gptrs completion --prompt="Write unit tests for this fibonacci function in Python" --input=./fibonacci_refactor.py --output=./fibonacci_tests.py --model=code-davinci-002 --temperature=0
```

# Setup

This assumes you have an [OpenAI Developer Account](https://platform.openai.com/)

### Generate API Token 

[See here for instructions](https://platform.openai.com/account/api-keys)

After generating the token, set the env variable 
```
export OPENAI_API_KEY={API_KEY}
```

### Install via Cargo 
```
cargo install gptrs
```
