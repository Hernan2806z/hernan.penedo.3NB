// Testes Simples para Implementação de Árvore Binária de Busca (BST)
// Os alunos devem implementar a árvore para fazer estes testes passarem

#[cfg(test)]
mod bst_tests {
    // Importe sua implementação de BST aqui
    // use crate::BST;
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
struct BST {
    root: Option<Box<Node>>,
}
impl BST {
    fn new() -> Self {
        BST { root: None }
    }
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        if let Some(ref mut root) = self.root {
            Self::insert_recursively(root, new_node);
        } else {
            self.root = Some(new_node);
        }
    }
    fn insert_recursively(current: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.value < current.value {
            if let Some(ref mut left) = current.left {
                Self::insert_recursively(left, new_node);
            } else {
                current.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right) = current.right {
                Self::insert_recursively(right, new_node);
            } else {
                current.right = Some(new_node);
            }
        }
    }
    fn search(&self, value: i32) -> bool {
        Self::search_recursively(&self.root, value)
    }
    fn search_recursively(current: &Option<Box<Node>>, value: i32) -> bool {
        match current {
            Some(node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    Self::search_recursively(&node.left, value)
                } else {
                    Self::search_recursively(&node.right, value)
                }
            }
            None => false,
        }
    }
}
    
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
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}

// Esqueleto para implementação da BST pelos alunos
struct BST {
    // Defina a estrutura aqui
    // Dica: você precisará de um nó raiz
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        // Implementar
        unimplemented!()
    }
    
    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        // Implementar
        unimplemented!()
    }
    
    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        // Implementar
        unimplemented!()
    }
    
    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        // Implementar
        unimplemented!()
    }
}