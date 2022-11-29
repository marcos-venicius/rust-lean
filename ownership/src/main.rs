
// rust nao tem um gargage colector (coletor de lixo)
//
// a memoria eh gerenciada pelo sistema de ownership com um conjunto de regras
// que o compilador checa
//
// se uma dessas regras eh violada o programa nao compila

// Stack
//
// A stack armazena os dados de forma ordenada no modelo LIFO (Last in, First out)
//
// Todo dado armazenado na stack precisa ter um tamanho fixo conhecido.
// Dados com tamanhos desconhecidos ou que podem ser alterados devem ser alocados na heap
//
// **Inserir dados na stack nao eh considerao "alocacao"

// Heap
//
// A heap armazena os dados de forma nao ordenada
//
// Quando voce coloca dados na heap voce solicita um certo espaco
//
// o _Memory allocator_ procura um ponto vazio na heap que eh grande o suficiente
// marca como sendo usado e retorna um ponteiro (endereco da localizacao na memoria)

// Colocar dados na stack eh mais rapido do que alocar na heap
// pois o _allocator_ nunca precisara procurar um lugar para armazenar o novo dado
// esta localizacao esta sempre no topo da stack

// acessar dados da memoria heap eh mais lento que acessar da stack
// porque voce precisa seguir um ponteiro para chegar la

// Quando seu codigo chama uma funcao
// os valores passados para a funcao incluindo ponteiros para dados na heap
// e a variaveis locais da funcao sao guardadas na stack.
//
// quando a funcao retorna
// esses valores sao removidos da stack

// === Regras do ownership
//
// - Cada valor no rust tem um owner (um dono)
// - So pode haver um owner (proprietario) por vez
// - Quando o owner (proprietario) sair do escopo, o valor sera dropado

fn main() {

}

