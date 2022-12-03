rust nao tem um gargage colector (coletor de lixo)

a memoria eh gerenciada pelo sistema de ownership com um conjunto de regras
que o compilador checa

se uma dessas regras eh violada o programa nao compila

Stack

A stack armazena os dados de forma ordenada no modelo LIFO (Last in, First out)

Todo dado armazenado na stack precisa ter um tamanho fixo conhecido.
Dados com tamanhos desconhecidos ou que podem ser alterados devem ser alocados na heap

**Inserir dados na stack nao eh considerao "alocacao"**

Heap

A heap armazena os dados de forma nao ordenada

Quando voce coloca dados na heap voce solicita um certo espaco

o _Memory allocator_ procura um ponto vazio na heap que eh grande o suficiente
marca como sendo usado e retorna um ponteiro (endereco da localizacao na memoria)

Colocar dados na stack eh mais rapido do que alocar na heap
pois o _allocator_ nunca precisara procurar um lugar para armazenar o novo dado
esta localizacao esta sempre no topo da stack

acessar dados da memoria heap eh mais lento que acessar da stack
porque voce precisa seguir um ponteiro para chegar la

Quando seu codigo chama uma funcao
os valores passados para a funcao incluindo ponteiros para dados na heap
e a variaveis locais da funcao sao guardadas na stack.

quando a funcao retorna
esses valores sao removidos da stack

## Regras do ownership

- Cada valor no rust tem um owner (um dono)
- So pode haver um owner (proprietario) por vez
- Quando o owner (proprietario) sair do escopo, o valor sera dropado

Literal string (`&str`)

No caso de uma string literal, nos sabemos o valor dela em tempo de compilacao
entao o valor fica statico no assembly final

para ver isso acontecendo execute os seguintes passos:
- escreva o seguinte porgrama
    ```rust
    fn main() {
    let s = "testing";

    println!("{s}");
    }
    ```
- agora transforme esse codigo em um assembly usando o cargo com o seguinte comando:
  cargo rustc --release -- --emit asm
- abra o arquivo em ./target/release/deps/ownership-<hash>.s
- dentro do texto procure a palavra "testing" ela estara hardcoded la mais ou menos nesse formato:
    ```asm
    .L__unnamed_4:
     .ascii  "testing"
     .size   .L__unnamed_4 7
    ```

Voce tambem o vera no executavel final:

- build a aplicacao em modo release:
    ```bash
    cargo build --release
    ```
- no linux execute o seguinte comando:
    ```bash
    strings ./target/release/ownership | grep testing
    ```
  
  voce devera ter um resultado, pois as strings literais sao salvas diretamente no executavel
  final

dessa forma voce nao pode colocar todas os valores que tem o tamanho desconhecido em tempo de
compilacao no binario final, ou mesmo strings que podem mudar de tamanho em tempo de execucao


### Alocando espaco na heap

- A memoria precisa ser requisitada pelo _Memory allocator_ em tempo de execucao
- Nos precisamos de uma maneira de retornar essa memoria para o _Memory allocator_ quando terminarmos de usar nossa `String`

## Formas como as variaveis e os dados interagem: Mover

```rs
let x = 5;
let y = x;
```
