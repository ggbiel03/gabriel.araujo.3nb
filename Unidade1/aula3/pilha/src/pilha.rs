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