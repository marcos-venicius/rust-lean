# Data types (Tipos de dados)

> Tenha em mente que rust é uma linguagem estaticamente tipada.
> O que significa que que ele precisa saber o tipo de todas as variáveis em tempo de compilação

em muitos casos o compilador pode inferir o tipo baseado no valor atribuido a variável

```rs
let name = "Marcos"; // &str
```

em casos onde muitos tipos são possíveis, como por exemplo quando convertemos uma `String` para um número utilizando o `parse` como no [guessing game](../guessing_game/src/main.rs) que criamos

```rs
// main.rs

...
let guess: u32 = match guess.trim().parse();
...
```

nós precisamos anotar o tipo da variável guess.
Se nós não informassemos o tipo `u32` o compilador nos mostraria um erro com uma mensagem mais ou menos como essa:

> _consider giving `guess` a type_

### Scalar Types

Um _Scalar Type_ representa um valor único. Rust tem 4 _Primary Scalar Types_: inteiros, números de ponto flutuante, boleanos e caracteres.

Por exemplo inteiros:

| Tamanho | com sinal (signed) | sem sinal (unsigned) |
| ------- | ------------------ | -------------------- |
| 8-bit   | i8                 | u8                   |
| 16-bit  | i16                | u16                  |
| 32-bit  | i32                | u32                  |
| 64-bit  | i64                | u64                  |
| 128-bit | i128               | u128                 |
| arch    | isize              | usize                |

> Signed numbers are stored using [two’s complement representation](https://en.wikipedia.org/wiki/Two%27s_complement)

> leia `^` com `elevado`

Cada variante com sinal (tipos que começam com `i` como `i8`, `i16`) pode guardar um número de `-(2 ^ (n - 1))` até `(2 ^ (n - 1)) - 1`, onde `n` é o número de bits do variante.

| variante | mínimo                     | máximo                     |
| -------- | -------------------------- | -------------------------- |
| `i8`     | `-(2 ^ (8 - 1)) = -128`    | `(2 ^ (8 - 1)) - 1 = 127`  |
| `i16`    | `-(2 ^ (16 - 1)) = -32768` | `(2 ^ (16 - 1)) - 1 = 127` |

Cada variante sem sinal (tipos que começam com `u` como `u8`, `u16`) pode guardar um número de `0` até `(2 ^ n) - 1`, onde `n` é o número de bits do variante

| variante | mínimo | máximo                 |
| -------- | ------ | ---------------------- |
| `u8`     | `0`    | `(2 ^ 8) - 1 = 255`    |
| `u16`    | `0`    | `(2 ^ 16) - 1 = 65535` |

> Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

* floating numbers

  ```rust
  let sum = 5 + 10;

  let difference = 95.5 - 4.3;

  let product = 4 * 30;

  let quotient = 56.7 / 32.2;
  let floored = 2 / 3; // results in 0
  let floating = 2.0 / 3.0; // will be divided 

  let remainder = 10 % 5;
  ```

* booleans

  ```rust
  let payed = true;
  let email_is_valid: bool = false;
  ```

* chars
  caracteres no rust são mais do que ASCII. Eles suportam letras acentuadas; chinesas, japonesas, e caracteres coreanos; emojis; e zero-width, spaços são `char`'s válidos 
  Unicode: de `U+0000` até `U+D7FF` e `U+E000` até `U+10FFFF`
  

# Compound Types

Os tipos compostos no rust podem agrupar multiplos valores em um tipo.
O rust tem dois tipos compostos primitivos: tuplas e listas (tuples, arrays)

### Tuplas

* não necessita que todos os valor sejam do mesmo tipo
* não precisa ser explicitamente tipada
* dois modos de acessar
    * desestruturando
        ```rust
        let color: (u8, u8, u8, f32) = (0, 255, 0, 0.5);

        // desestruturando uma tupla
        let (r, g, b, a) = color;
        ```
    * por período
        ```rust
        ...

        // periodo
        let alpha = color.3;
        ```

uma tupla sem nenhum valor tem um nome especial `unit`.
O seu valor e seu tipo são ambos `()`

veja exemplos [nesse arquivo](./src/tuples.rs);

