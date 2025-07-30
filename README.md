# rh

Um simples visualizador hexadecimal de linha de comando.

## Compilação

    git clone https://github.com/merces/rh.git
    cd rh
    cargo build --release

## Instalação

    cargo install --path .

## Uso

    $ rh /bin/ls
    00000000: 7F 45 4C 46 02 01 01 00 00 00 00 00 00 00 00 00   ELF            
    00000010: 03 00 3E 00 01 00 00 00 30 6D 00 00 00 00 00 00    >     0m      
    00000020: 40 00 00 00 00 00 00 00 28 24 02 00 00 00 00 00  @       ($      
    00000030: 00 00 00 00 40 00 38 00 0D 00 40 00 1F 00 1E 00      @ 8   @     
    00000040: 06 00 00 00 04 00 00 00 40 00 00 00 00 00 00 00          @       
    00000050: 40 00 00 00 00 00 00 00 40 00 00 00 00 00 00 00  @       @  
    ...

    $ echo rock | rh
    00000000: 72 6F 63 6B 0A                                   rock 
    00000005:
