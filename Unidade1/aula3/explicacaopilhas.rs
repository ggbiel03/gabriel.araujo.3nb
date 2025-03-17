/*Em resumo, `push` em Rust adiciona um elemento ao final de um vetor. 
Se o vetor estiver cheio, ele realoca memória, copia os elementos, e então adiciona o novo elemento.

fn main() {
    let mut vetor: Vec<i32> = Vec::new(); // Cria um vetor de inteiros (i32)

    vetor.push(10); // Adiciona 10 ao vetor
    vetor.push(20); // Adiciona 20 ao vetor
    vetor.push(30); // Adiciona 30 ao vetor

    println!("{:?}", vetor); // Imprime o vetor: [10, 20, 30]
}

```rust
fn main() {
    let mut vetor: Vec = Vec::new(); // Cria um vetor de inteiros (i32)

    vetor.push(10); // Adiciona 10 ao vetor
    vetor.push(20); // Adiciona 20 ao vetor
    vetor.push(30); // Adiciona 30 ao vetor

    println!("{:?}", vetor); // Imprime o vetor: [10, 20, 30]
}
```

**Explicando:**

*   `let mut vetor: Vec = Vec::new();`:  Cria um vetor mutável (`mut`) chamado `vetor`.  `Vec` especifica que ele armazenará inteiros de 32 bits. `Vec::new()` cria um vetor vazio.
*   `vetor.push(10);`, `vetor.push(20);`, `vetor.push(30);`: Adicionam os valores 10, 20 e 30 ao final do vetor, respectivamente. Se o vetor precisasse de mais espaço, Rust cuidaria da realocação.
*   `println!("{:?}", vetor);`: Imprime o conteúdo do vetor formatado para debug.

*/


