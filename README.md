# Subtitle normalizer

A small utility CLI app that replaces all special latin characters in a UTF-8 encoded subtitle file with their ASCII counterparts.

## Prerequisites

The program is written in rust. In order to compile it you need a rust toolchain.

## Usage

The program takes one parameter and that's the path to the file you desire to strip of special characters.

```
cargo run data/subtitles.srt
```

Running the command above creates a new file `subtitles.srt.normalized`.