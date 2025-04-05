//use crate::BST;

// Esqueleto para implementação da BST pelos alunos
#[derive(Debug)]
struct Node {
    value: i32,               // Valor do nó
    left: Option<Box<Node>>,  // Filho esquerdo
    right: Option<Box<Node>>, // Filho direito
}

// Árvore BST
#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>, // Raiz da árvore
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }
    
    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;

        // Loop para encontrar o lugar certo para inserir
        loop {
            match current {
                // Lugar vazio encontrado, inserimos aqui
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None, // Corrigido de 'rigth' para 'right'
                    }));
                    return;
                }
                // Navegamos para a posição correta
                Some(node) => {
                    if value < node.value {
                        // Ir para esquerda
                        current = &mut node.left;
                    } else if value > node.value {
                        // Ir para direita
                        current = &mut node.right; // Corrigido de 'rigth' para 'right'
                    } else {
                        // Valor duplicado, nada a fazer
                        return;
                    }
                }
            }
        }
    }
    
    // Buscar valor
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        // Loop para navegar até encontrar ou não o valor
        while let Some(node) = current {
            if value == node.value {
                return true; // Encontrou
            } else if value < node.value {
                current = &node.left; // Vai para esquerda
            } else {
                current = &node.right; // Vai para direita
            }
        }

        false // Não encontrou
    }
}

#[cfg(test)]
mod bst_tests {
    use super::BST;

    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        assert!(!bst.search(2));

        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}

pub fn main() {
    println!("=== Árvore Binária de Busca ===");

    // BST exemplo
    let mut bst = BST::new();

    if bst.is_empty() {
        println!("A árvore está vazia.");
    }
    
    // Inserir alguns valores para retornar se é verdadeiro ou falso.
    bst.insert(10);
    bst.insert(27);
    bst.insert(23);
    bst.insert(7);

    println!("10: {}", bst.search(10));
    println!("7: {}", bst.search(7));
    println!("24: {}", bst.search(24));
}
