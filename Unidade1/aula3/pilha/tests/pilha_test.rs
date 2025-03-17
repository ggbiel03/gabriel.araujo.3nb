// Definição da estrutura de Pilha (Stack)
pub struct Stack<T> {
    // A implementação será adicionada após criar os testes
    elementos: Vec<T>,
    capacidade_maxima: Option<usize>,
    //<T> é usado para definir um objeto qualquer, quando não temos nada específico.
}

// A implementação será adicionada após criar os testes
impl<T> Stack<T> {
    // A implementação será adicionada após criar os testes

    //construtor para a estrutura de pilha, retorna uma nova instância dela 
    //com um vetor vazio e sem capacidade definida.
    pub fn nova() -> Self {
        Stack {
            elementos: Vec::new(),
            capacidade_maxima: None,
        }
    }

    // nessa função irei definir uma capacidade máxima por conta do usize
    // retornando Self, ou seja, uma instância da própria estrutura Pilha/Stack
    pub fn nova_com_capacidade (capacidade: usize) -> Self {
        Stack {
            elementos: Vec::with_capacity(capacidade),
            capacidade_maxima: Some(capacidade),
        }
    }

    //nessa fn eu add um elemento à pilha/stack, 
    //verificando se há espaço disponível se a pilha tiver uma capacidade máxima definida.
    //&mut self = precisa de uma referência mutável na pilha, pois irá modificar seu estado.
    pub fn empilhar(&mut self, elemento: T) -> Result<(), &'static str> {
        // Verifica se a pilha atingiu sua capacidade máxima
        if let Some(capacidade) = self.capacidade_maxima {
            if self.elementos.len() >= capacidade {
                return Err("Erro: A pilha está cheia");
            }
        }

        // adiciona o elemento à pilha
        self.elementos.push(elemento);
        Ok(()) // indica sucesso (nenhum erro ocorreu).
    }


    //serve para remover e retornar o último elemento inserido na pilha/stack(POP) 
    //seguindo o princípio LIFO (Last In, First Out).
    pub fn desempilhar(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    //nessa fn irei visualizar o elemento no topo da pilha sem removê-lo (LIFO)
    //&self = não modifica a pilha, apenas acessa os dados.
    pub fn topo(&self) -> Option<&T> {
        self.elementos.last()
    }

    //verifica se a pilha está vazia com boolean (verdadeiro ou falso)
    //&self = não modifica a pilha, apenas acessa os dados.
    pub fn esta_vazia(&self) -> bool {
        self.elementos.is_empty()
    }

    //verifica se a pilha está cheia com boolean
    //&self = não modifica a pilha, apenas acessa os dados.
    pub fn esta_cheia(&self) -> bool {
        if let Some(capacidade) = self.capacidade_maxima {
            return self.elementos.len() >= capacidade;
        }
        false
    }

    //aqui irei retornar a quantidade de elementos na minha pilha
    //&self = não modifica a pilha, apenas acessa os dados.
    pub fn tamanho(&self) -> usize {
        self.elementos.len()
    }

    // remove todos os elementos da pilha, deixando-a vazia. Modificação direta
    pub fn limpar(&mut self) {
        self.elementos.clear();
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nova_pilha_esta_vazia() {
        let pilha: Stack<i32> = Stack::nova();
        assert!(pilha.esta_vazia());
    }

    #[test]
    fn nova_pilha_com_capacidade() {
        let pilha: Stack<i32> = Stack::nova_com_capacidade(5);
        assert!(pilha.esta_vazia());
        assert_eq!(pilha.tamanho(), 0);
    }

    #[test]
    fn empilhar_aumenta_tamanho() {
        let mut pilha = Stack::nova();
        pilha.empilhar(42).unwrap();
        assert_eq!(pilha.tamanho(), 1);
        assert!(!pilha.esta_vazia());
    }

    #[test]
    fn topo_retorna_ultimo_elemento() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1).unwrap();
        pilha.empilhar(2).unwrap();
        pilha.empilhar(3).unwrap();
        assert_eq!(*pilha.topo().unwrap(), 3);
        assert_eq!(pilha.tamanho(), 3); // Verificar que o topo não remove o elemento
    }

    #[test]
    fn desempilhar_retorna_ultimo_elemento() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1).unwrap();
        pilha.empilhar(2).unwrap();
        let valor = pilha.desempilhar();
        assert_eq!(valor, Some(2));
        assert_eq!(pilha.tamanho(), 1);
    }

    #[test]
    fn desempilhar_pilha_vazia_retorna_none() {
        let mut pilha: Stack<i32> = Stack::nova();
        let valor = pilha.desempilhar();
        assert_eq!(valor, None);
    }

    #[test]
    fn topo_pilha_vazia_retorna_none() {
        let pilha: Stack<i32> = Stack::nova();
        let valor = pilha.topo();
        assert_eq!(valor, None);
    }

    #[test]
    fn pilha_limitada_rejeita_elementos_alem_da_capacidade() {
        let mut pilha = Stack::nova_com_capacidade(2);
        assert!(pilha.empilhar(1).is_ok());
        assert!(pilha.empilhar(2).is_ok());
        assert!(pilha.empilhar(3).is_err()); // Deve falhar
        assert_eq!(pilha.tamanho(), 2);
    }

    #[test]
    fn pilha_ilimitada_aceita_muitos_elementos() {
        let mut pilha = Stack::nova();
        for i in 0..1000 {
            assert!(pilha.empilhar(i).is_ok());
        }
        assert_eq!(pilha.tamanho(), 1000);
    }

    #[test]
    fn limpar_remove_todos_elementos() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1).unwrap();
        pilha.empilhar(2).unwrap();
        pilha.empilhar(3).unwrap();
        pilha.limpar();
        assert!(pilha.esta_vazia());
        assert_eq!(pilha.tamanho(), 0);
    }

    #[test]
    fn esta_cheia_retorna_correto() {
        let mut pilha = Stack::nova_com_capacidade(2);
        assert!(!pilha.esta_cheia());
        pilha.empilhar(1).unwrap();
        assert!(!pilha.esta_cheia());
        pilha.empilhar(2).unwrap();
        assert!(pilha.esta_cheia());
    }

    #[test]
    fn pilha_ilimitada_nunca_esta_cheia() {
        let mut pilha = Stack::nova();
        for i in 0..100 {
            pilha.empilhar(i).unwrap();
            assert!(!pilha.esta_cheia());
        }
    }
}
