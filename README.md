# GPTshell 

GPTshell is an early prototype of a tool for developers to query and interact with ChatGPT's language capabilities. GPTshell's aim is to simplify the process of using natural language processing and increase productivity while developing.

The tool was used during the development of this shell with ChatGPT and OpenAI! Any feedback is welcome as its still an early prototype. 

Download and run `gptshell` in your terminal.

## Install via Cargo

```
cargo install gptshell
```

*Recommended way to install while still in development*

## Why use GPTshell?

Quickly build up powerful queries by loading files of code, specifying specific lines of code and exporting chat sessions for future use. Essentially reducing the feedback loop time with ChatGPT vs using a web browser where you have to navigate between different files, IDEs and copy & paste. 

## Demo
<img src="https://user-images.githubusercontent.com/20296911/224504826-7ab2c4ed-75fd-4f56-b1b6-482ec44f0606.gif" width=500 height=281>

## Commands

Run `help()` in the GPTshell for a list of these commands.

<img src="./assets/commands.png" alt="commands">

## Supported APIs

Note: Initial focus has been on the shell interactivity so currently only supports chat and completions. 

```
✅ completions
✅ chat
🚧 edits
🚧 images
🚧 moderations
🚧 audio
```

# Setup

This assumes you have an [OpenAI Developer Account](https://platform.openai.com/)

## Generate API Token 

[See here for instructions](https://platform.openai.com/account/api-keys)

After generating the token, set the env variable 
```
export OPENAI_API_KEY={API_KEY}
```
