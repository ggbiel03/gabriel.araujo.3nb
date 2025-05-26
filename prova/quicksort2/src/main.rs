//crio uma estrutura para guardar o array
struct QuickSort {
    arr: Vec<i32>,
}

impl QuickSort {
    //faço uma função que chama o quicksort e atualiza o array
    fn sort(&mut self) {
        self.arr = Self::quick_sort2(self.arr.clone());
    }

    fn quick_sort2(mut arr: Vec<i32>) -> Vec<i32> {
        //crio uma condição (if) vendo se o vetor tem 0 ou 1 elemento, já está ordenado!
        if arr.len() <= 1 {
            return arr;
        }
        //escolho o primeiro numero como pivô
        let pivot = arr.remove(0);
        let mut menores = Vec::new();
        let mut maiores = Vec::new();
        //separo os numeros em dois vetores diferentes do o pivô
        for n in arr {
            if n <= pivot {
                menores.push(n);
            } else {
                maiores.push(n);
            }
        }
        //organizo dos menores aos maiores
        let mut ordenados = Self::quick_sort2(menores);
        ordenados.push(pivot); //coloca o numero pivô no meio
        ordenados.extend(Self::quick_sort2(maiores)); //junta os maiores
        ordenados
    }
}

fn main() {
    //cri a struct mutavel com o array fornecido
    let mut qs = QuickSort {
        arr: vec![34, 7, 23, 32, 5, 62, 31, 12, 43, 3],
    };

    //exibe o array original 
    println!("Array original: {:?}", qs.arr);

    // sorteia o array
    qs.sort();

    //exibe o array ordenado [3, 5, 7, 12, 23, 31, 32, 34, 43, 62]
    println!("Array ordenado: {:?}", qs.arr);
}

