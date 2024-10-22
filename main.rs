fn main(){
    //ex1();
    //ex2();
    //ex4();
    //ex5();
    //ex6();
    ex7();
}
//1. Clonando e Movendo Strings: 
//Escreva um programa que recebe duas strings como entrada. A primeira string deve ser movida para uma nova variável e a segunda string deve ser clonada para uma nova variável. Mostre os valores das strings originais e das novas após o movimento e clonagem. Explique a diferença entre mover e clonar.
fn ex1(){
    let string1 = String::from("teste");
    let string2 = String::from("testando");

    let nova_string1 = string1;
    let nova_string2 = &string2;

    println!("String 1: {}", nova_string1);
    println!("String2 original: {}", string2);
    println!("String 2: {}", nova_string2);
}
//2. Função de Soma com Empréstimo Imutável
//Crie uma função soma que recebe uma slice de números inteiros como umareferência imutável. A função deve calcular e retornar a soma dos números. No programa principal, crie um vetor de inteiros e passe uma slice para a função. Após chamar a função, mostre o valor do vetor original. Dica: procure por vec!.

fn soma(num: &[i32]) -> i32 {
    let mut total = 0;

    for &num in num{
        total += num;
    }
    total
}
fn ex2(){
    let vetor = vec![1,2,3,4,5];

    let resultado = soma(&vetor);

    println!("SOMA DOS ELEMENTOS: {}", resultado);

    println!("VETOR ORIGINAL: {:?}", vetor);
}

//3. Reatribuição e Ownership: Crie um programa que tenha duas variáveis que armazenam strings. Reatribua uma das strings para a outra variável e mostre o que acontece com o valor da variável original após a reatribuição.

// fn ex3(){

//     let string1 = String::from("ola");
//     let string2: String = String::from("mundo");

//     string1 = string2;
//     println!("{}", string2);

//     println!("{}", string1);

// }

//4. Empréstimo Mutável Escreva uma função adicionar_prefixo que recebe uma string mutável e adiciona o prefixo "Prefixo: " a essa string. No programa principal, crie uma string e passe-a para a função como referência mutável. Mostre o valor da string após a modificação.

fn adicionar_prefixo(s: &mut String){
    let nova_string = String::from("Prefixo: ") + s;

    *s = nova_string;
}
fn ex4(){
    let mut string = String::from("exemplo");
    println!("Antes: {}", string);

    adicionar_prefixo(&mut string);

    println!("Depois: {}", string);
}

//5. Multiplicação de Vetores com Empréstimo Imutável Crie uma função que recebe dois vetores de números inteiros como referências imutáveis e retorna um novo vetor contendo os produtos dos elementos correspondentes. O programa principal deve criar dois vetores, passar ambos para a função, e depois imprimir o vetor resultante. Dica: procure por Vec<> e vec!


fn ex5() {
    // Criando dois vetores com números inteiros
    let vetor1 = vec![1, 2, 3, 4];
    let vetor2 = vec![5, 6, 7, 8];

    // Chamando a função que multiplica os vetores
    match multiplicar_vetores(&vetor1, &vetor2) {
        Some(resultado) => println!("Resultado da multiplicação: {:?}", resultado),
        None => println!("Os vetores devem ter o mesmo tamanho!"),
    }
}

// Função que recebe duas referências imutáveis a vetores e retorna um novo vetor
fn multiplicar_vetores(v1: &Vec<i32>, v2: &Vec<i32>) -> Option<Vec<i32>> {
    // Verificando se os vetores têm o mesmo tamanho
    if v1.len() != v2.len() {
        return None; // Retorna None se os tamanhos forem diferentes
    }

    // Criando um novo vetor para armazenar os produtos
    let mut resultado = Vec::new();

    // Usando um loop com índices para acessar os elementos
    for i in 0..v1.len() {
        // Multiplicando os elementos correspondentes e adicionando ao vetor resultado
        resultado.push(v1[i] * v2[i]);
    }

    // Retornando o vetor com os produtos dentro de Some
    Some(resultado)
}


//6. Número de Caracteres com Empréstimo Imutável Implemente uma função chamada contar_caracteres que recebe uma referência imutável para uma string e retorna o número de caracteres nela. No programa principal, crie uma string e chame a função, mostrando o resultado. Dica: procure por chars e count

fn ex6(){
    let texto = String::from("teste");

    let numero_de_caracteres = contar_caracteres(&texto);

    println!("O número de caracteres é: {}", numero_de_caracteres);

}

fn contar_caracteres(s: &String) -> usize{
    s.chars().count()
}

//7. Split de String com Empréstimo Imutável Escreva uma função que recebe uma referência imutável para uma string e retorna um vetor de palavras que são divididas por espaço. No programa principal, crie uma string, passe-a para a função e mostre o vetor resultante. Dica: procure por split_whitspace e collect.

fn ex7(){
    let texto = String::from("Ola mundo");

    let palavras = dividir_em_palavras(&texto);

    println!("As palavras são: {:?}", palavras);
}

fn dividir_em_palavras(s: &String) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn ex8(){
    
}