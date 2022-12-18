# LaTeX-Mailmerge 

A helper for converting CSV files into a LaTeX command format, so that the data can be used for mail merge letters.

## Purpose of the Program

LaTeX is a phantastic tool for composing and writing mail merges. The LaTeX document classes `letter` and `scrlttr2` provide the implementation of typesetting letters of all kind.

For writing a mail merge, it has been a common way to wrap the letter class into a created command and then call this command multiple times. In composition with the `ifthen`-package, a minimal example looks like this:

```
\documentclass[paper=a4, fontsize=12pt]{scrlttr2}
% other packages
\usepackage{ifthen}
% ...
% set letter options
\begin{document}
% create command mallmerge with two arguments: (1) last name, (2) first name
\newcommand{mailmerge}[2]{
    \begin{letter}{#2 #1}
    \opening{Dear #2,}
    This is a minimal mail example of creating multiple personalized letters.
    \closing{With Kind Regards}
    \end{letter}
}
\end{document}
% input data.tex which contains the data
\input{data}
```

Note the last line of this minimal working example: We included an other TeX-file called `data.tex` which is supposed to call the created `\mailmerge`-command. An example content would be:

```
\mailmerge{Newton}{John}
\mailmerge{James}{Mary}
\mailmerge{Smith}{Anthony}
```

This would create *three letters* with the adapted content. But where does the data come from? Mostly we would use Excel/Calc-Sheets where we have tables of customers, friends, etc. which we would like to address. In this case, it would be very complicated to manually create an adapted LaTeX command for each entry. Here `latexmailmerge` steps in. It can convert a CSV file into a TeX file with the appropriate content which then can be used to create a mail merge.

## Usage

A basic usage would look like that:

```
latexmailmerge -f1 -d "," data.csv > data.tex
```

The argument `-f` specifies the *first line* (for skipping headlines) and is zero based. The argument `-d` specifies the delimater which is most likely "," or ";". `data.csv` is the source file and `data.tex` is the file where the output will be saved.

You can see a complete list of possible options via `latexmailmerge --help`.

## Installation

Clone the repository and compile the project using Cargo:

```
git clone https://github.com/reckel-jm/latexmailmerge.git
cargo build --release
```