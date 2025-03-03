use desafio2::queue::Queue;  // Importa o módulo Queue do projeto

// Testa a função enqueue e dequeue, verificando a ordem dos elementos que estão na fila
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();  // Cria uma nova fila vazia
        queue.enqueue(1);  // Enfileira o elemento 1
        queue.enqueue(2);  // Enfileira o elemento 2
        queue.enqueue(3);  // Enfileira o elemento 3

        // Verifica se os elementos foram removidos na ordem correta (FIFO)
        assert_eq!(queue.dequeue(), Some(1));  // O primeiro elemento a ser retirado deve ser 1
        assert_eq!(queue.dequeue(), Some(2));  // O próximo elemento a ser retirado deve ser 2
        assert_eq!(queue.dequeue(), Some(3));  // O próximo elemento a ser retirado deve ser 3
        assert_eq!(queue.dequeue(), None);  // Após remover todos os elementos, a fila deve estar vazia (None)
    }

    // Testa a função `peek`, que deve retornar o elemento da frente da fila sem removê-lo
    #[test]
    fn test_peek() {
        let mut queue = Queue::new();  // Cria uma nova fila vazia
        queue.enqueue(42);  // Enfileira o elemento 42
        assert_eq!(queue.peek(), Some(&42));  // Verifica se o primeiro elemento na fila é 42
    }

    // Testa a função `len`, que deve retornar o número de elementos na fila
    #[test]
    fn test_len() {
        let mut queue = Queue::new();  // Cria uma nova fila vazia
        assert_eq!(queue.len(), 0);  // Verifica que a fila está vazia inicialmente (tamanho 0)
        queue.enqueue(10);  // Enfileira o elemento 10
        queue.enqueue(20);  // Enfileira o elemento 20
        assert_eq!(queue.len(), 2);  // Verifica que a fila agora tem 2 elementos
    }

    // Testa a função `is_empty`, que deve verificar se a fila está vazia
    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();  // Cria uma nova fila vazia
        assert!(queue.is_empty());  // Verifica que a fila está vazia inicialmente
        queue.enqueue(5);  // Enfileira o elemento 5
        assert!(!queue.is_empty());  // Verifica que a fila não está vazia após adicionar um elemento
    }
}

