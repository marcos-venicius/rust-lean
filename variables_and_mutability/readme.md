# Variáveis e mutabilidade

- Por padrão variáveis são imutáveis
  > Quando uma variável é imutável, uma que um valor é vinculado, ele nunca mais irá mudar

# Constantes

### Diferença entre as variáveis

- Não é permitido usar `mut` com constantes

  > Constantes não são apenas imutáveis por padrão... elas **sempre são imutáveis**

- O tipo de uma constante sempre deve ser anotado

```rs
// wrong

const x = 10;
```

```rs
// correct

const x: u32 = 10;
```

- constantes podem ser declaradas em qualquer escopo, incluindo o escopo global, que é muito útil para valores que muitas partes do código precisa usar

```rs
// example

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- a convenção de nomes no rust para constantes é: caixa alta com underlines entre as palavras

```rs
// wrong

const game_fps: u16 = 60;
```

```rs
// correct

const GAME_FPS: u16 = 60;
```