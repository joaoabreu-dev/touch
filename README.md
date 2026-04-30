# Touch (Windows)
![Version](https://img.shields.io/badge/version-1.0.0-freen)

Licensed under the MIT License. See LICENSE for details.

Uma implementação em Rust do comando Unix/Linux `touch` para o ecossistema Windows.

## Português

### Descrição
Ao trabalhar no terminal do Windows (PowerShell ou CMD), uma das ferramentas mais úteis em falta é o `touch`. Embora o Windows ofereça alternativas nativas, estas não são tão simples nem intuitivas como escrever:

```text
touch ficheiro.txt
```

Este projeto traz essa simplicidade para o Windows, permitindo criar ficheiros vazios e manipular carimbos de data/hora (`timestamps`) de forma prática.

### Funcionalidades
- Criação instantânea de ficheiros vazios.
- Atualização de tempos de acesso (`-a`) e modificação (`-m`).
- Opção para não criar novos ficheiros (`-c`) caso não existam.
- Utilização de ficheiros de referência (`-r`) para copiar timestamps.
- Validação de nomes reservados (`CON`, `PRN`, `NUL`, etc.) e caracteres inválidos no Windows.

### Como instalar

1. Compilar e instalar:

Certifica-te de que tens o Rust instalado. Na raiz do projeto, executa:

```text
cargo install --path .
```

2. Configurar o PATH:

Para utilizar o comando `touch` em qualquer terminal, a pasta de binários do Cargo deve estar nas variáveis de ambiente. Normalmente localiza-se em:

```text
%USERPROFILE%\.cargo\bin
```

Dica: Se instalaste o Rust recentemente, o instalador costuma configurar isto automaticamente. Caso o comando não seja reconhecido, reinicia o terminal.

### Exemplos de uso

```text
touch ficheiro.txt             # Cria um ficheiro novo ou atualiza timestamps
touch -c ficheiro.txt          # Atualiza apenas se o ficheiro já existir
touch -a ficheiro.txt          # Atualiza apenas o tempo de acesso
touch -m ficheiro.txt          # Atualiza apenas o tempo de modificação
touch -r ref.txt alvo.txt      # Copia os tempos de ref.txt para alvo.txt
```

---

## English

### Description
When working in a Windows terminal, the `touch` command is often missed. While Windows has native alternatives, they are not as straightforward as typing:

```text
touch file.txt
```

This project brings that simplicity to Windows, allowing the creation of empty files and the manipulation of file timestamps.

### Features
- Instant creation of empty files.
- Update access times (`-a`) and modification times (`-m`).
- Option to avoid file creation (`-c`) if the file does not exist.
- Use of reference files (`-r`) to copy timestamps from one file to another.
- Validation of Windows reserved names (`CON`, `PRN`, `NUL`, etc.) and invalid characters.

### How to install

1. Build and install:

Make sure Rust is installed. From the project root, run:

```text
cargo install --path .
```

2. Environment Variables (PATH):

To use the `touch` command from any terminal, the Cargo bin folder must be included in your `PATH`. It is typically located at:

```text
%USERPROFILE%\.cargo\bin
```

Tip: Rust's installer usually adds this path automatically. If the `touch` command is not recognized, please restart your terminal.

### Usage examples

```text
touch file.txt                 # Creates a new file or updates timestamps
touch -c file.txt              # Updates only if the file already exists
touch -a file.txt              # Updates only the access time
touch -m file.txt              # Updates only the modification time
touch -r ref.txt target.txt    # Copies timestamps from ref.txt to target.txt
```
