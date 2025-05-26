fn main() {
    //defino o array como mutavel (let mut) com os numeros passados
    let mut arr = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];
    //exibe o array original 
    println!("Array original: {:?}", arr);
    // sorteia o array
    arr.sort();
    //exibe o array ordenado
    println!("Array ordenado: {:?}", arr);
}

