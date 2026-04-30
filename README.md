**PT**
# Touch (Windows)
![Version](https://img.shields.io/badge/version-0.5.1-blue)

Uma implementação em Rust do comando Unix/Linux `touch` para o ecossistema Windows.

### Descrição
Ao trabalhar no terminal do Windows (PowerShell ou CMD), uma das ferramentas de que mais sinto falta é o `touch`. Embora o Windows ofereça alternativas como `type nul > ficheiro.txt`, estas não são tão simples nem tão intuitivas como escrever:
```
touch ficheiro.txt
```

Este projeto traz essa simplicidade para o Windows, permitindo criar ficheiros vazios de forma rápida e prática.

### Funcionalidades
- Criação instantânea de ficheiros vazios
- Sintaxe simples e familiar para quem vem de Linux/Unix
- Alternativa mais intuitiva aos comandos nativos do Windows

### Como instalar
1. Compilar e instalar:

Certifica-te de que tens o Rust instalado. Depois, na raiz do projeto, executa:
```
cargo install --path 
``` 
2. Configurar o PATH:
Para utilizares o comando touch em qualquer terminal, a pasta de binários do Cargo deve estar nas tuas Variáveis de Ambiente (PATH).
Normalmente, esta pasta localiza-se em:
%USERPROFILE%\.cargo\bin

`
Dica: Se instalaste o Rust recentemente, o instalador costuma adicionar este caminho automaticamente. Se o comando touch não for reconhecido após a instalação, reinicia o teu terminal.
`

### Como usar
```
touch novo_ficheiro.txt
```

**EN**
# Touch (Windows)

A Rust implementation of the Unix/Linux `touch` command for Windows.

### Description
When working in the Windows terminal, one of the tools I miss the most is `touch`. Windows has alternatives such as `type nul > file.txt`, but they are not as simple or intuitive as typing:
``` 
touch file.txt
``` 
This project brings that simplicity to Windows, making it easy to create empty files quickly and conveniently.

### Features
- Instantly creates empty files
- Simple and familiar syntax for Linux/Unix users
- A more intuitive alternative to native Windows commands

### How to install
1. Build and Install:
Make sure you have Rust installed. Then run the following command from the project root:
``` 
cargo install --path 
``` 
2. Environment Variavles (PATH):
To use the touch command from any terminal, the Cargo bin folde must be included in the Environment Variables (PATH).
It is tipically located at:
%USERPROFILE%\.cargo\bin

`
Tip: Rust's installer usually adds this path automatically. If the touch command is not recognized after installation, please restart your terminal.
`
### How to use
``` 
touch file.txt
``` 
    
