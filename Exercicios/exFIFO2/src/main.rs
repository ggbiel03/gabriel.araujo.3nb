fn main() {
    // Fila FIFO implementada manualmente com Vec
    let mut fila: Vec<i32> = Vec::new();

    // Adicionando elementos (push no final)
    fila.push(100);
    fila.push(200);
    fila.push(300);
    println!("Fila após adicionar: {:?}", fila);

    // Removendo elemento da frente
    if !fila.is_empty() {
        let removido = fila.remove(0);
        println!("Removido: {}", removido);
    }
    println!("Fila após remover: {:?}", fila);

    // Verificando o elemento da frente sem remover
    if let Some(frente) = fila.first() {
        println!("Elemento na frente: {}", frente);
    } else {
        println!("Fila está vazia");
    }

    // Verificando se a fila está vazia
    println!("Fila está vazia? {}", fila.is_empty());

    // Verificando o tamanho da fila
    println!("Tamanho da fila: {}", fila.len());
}
