use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    // Adicionando elementos
    fila.push_back(500);
    fila.push_back(400);
    fila.push_back(300);
    fila.push_back(200);
    fila.push_back(100);
    println!("Fila após adicionar: {:?}", fila);

    // Removendo elemento da frente
    if let Some(removido) = fila.pop_front() {
        println!("Remoção: {}", removido);
    }

    println!("Fila após remoção: {:?}", fila);

    // Verificando o elemento da frente
    if let Some(frente) = fila.front() {
        println!("Atual elemento da frente: {}", frente);
    }

    // Verificando se está vazia
    println!("A fila está vazia? {}", fila.is_empty());

    // Tamanho da fila
    println!("Quantos elementos estão na fila: {}", fila.len());

    //Mostrando os elementos restantes
    println!("Elementos restantes na fila: {:?}", fila);
   
}
