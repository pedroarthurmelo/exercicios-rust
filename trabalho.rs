struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: Permissao, //tupla de permissão
    usuario: Usuario,
    grupo: Grupo,
}

// Estrutura: Arquivo

// - nome: String
// - tamanho: u64
// - permissoes: Tupla de Permissao
// - usuario: Usuario
// - grupo: Grupo

// **Estrutura: Arquivo**

// - new: (nome, tamanho, uid, guid) -> Arquivo
// - alterar_permissao: (Autorreferência mutável, Permissao) -> -
// - stat: (Autorreferência) -> -

// **Estrutura Arquivo:**
// • No método "new" você deverá atribuir ao campo permissao as permissões padrão (leitura = false, escrita = true, execução = false). No restante dos campos você deverá preencher com os valores recebidos como parâmetro. 
// • No método "stat" você deverá produzir uma saída no mesmo formato da Figura 1. Lembre-se que aqui você utilizará os campos da estrutura autorreferenciada.

// **EXEMPLO DA SAÍDA** 
// Arquivo: teste
// Tamanho: 0
// Permissões: (664/rw-rw-r--)
// Uid: 1000
// Gid: 1000

// No método "alterar_permissao" você deverá alterar a permissão do arquivo para a que foi passada como parâmetro.

struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

// Estrutura: Permissao

// - leitura: bool
// - escrita: bool
// - execucao: bool


// UTILIZANDO IMPL
// **Estrutura: Permissao**

// - new: (leitura, escrita, execucao) -> Permissao
// - octal: (Autorreferência) -> u8

// **Estrutura Permissao** 
// • No método "new" você deverá atribuir aos campos leitura, escrita e execução os valores recebidos como parâmetro. 
// • No método "octal" você deverá retornar a representação octal das permissões (multiplicar pelo sua base e somar os valores).

struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: Permissao, //tupla de permissao
    dono: Usuario,
}
// Estrutura: Diretorio

// - nome: String
// - arquivos: Vec<Arquivo>
// - permissoes: Tupla de Permissao
// - dono: Usuario

// **Estrutura: Diretorio**

// - new: (nome, permissao, dono) -> Diretorio
// - adiciona_arquivo: (nome) -> -
// - remove_arquivo: (nome) -> -
// - listar_conteudo: () -> -

// **"Estrutura Diretorio"**
// • No método new você deverá inicializar os campos da estrutura Diretorio com o que foi passado como parâmetro do método. 
// • No método "adiciona_arquivo" você deve colocar um arquivo no vetor de arquivos da estrutura autorreferenciada. 
// • No método "remove_arquivo" você remover um arquivo do vetor de arquivos da estrutura autorreferenciada. 
// • No método "listar_conteudo" você deve listar todos os arquivos presentes no diretório.

struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
}

// **Estrutura: Usuario**

// - new: (nome, uid, grupo) -> Usuario
// - adiciona_grupo: (grupo) -> -
// - remove_grupo: (grupo) -> -
// - listar_grupos: () -> -

// **Estrutura Usuario** 
// • No método new você deverá inicializar os campos da estrutura "Usuario" com o que foi passado como parâmetro do método. 
// • No método "adiciona_grupo" você deve adicionar um grupo ao vetor da estrutura Grupo na autorreferencia de usuário. 
// • No método "remove_grupo" você deve remover um grupo ao vetor da estrutura Grupo na autorreferencia de usuário. 
// • No método "listar_grupos" você deve listar todos os grupos daquele usuário.

struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>
}

// **Estrutura: Grupo**

// - new: (nome, gid) -> Grupo
// - adiciona_membro: (usuario) -> -
// - remove_membro: (usuario) -> -
// - listar_membros: () -> -

// **Estrutura Grupo** 
// • No método "new" você deverá inicializar os campos da estrutura Grupo com o que foi passado como parâmetro do método. 
// • No método "adiciona_membro" você deve adicionar um grupo ao vetor da estrutura Grupo na autorreferencia de usuário. 
// • No método "remove_membro" você deve remover um grupo ao vetor da estrutura Grupo na autorreferencia de usuário. 
// • No método "listar_grupos" você deve listar todos os membros daquele grupo.

fn main(){
    println!("Olá, Mundo");
}

// deverá criar um menu que seja possível manipular (criar, editar, apagar e listar) cada uma das estruturas implementadas. Nos métodos implementados, o objetivo é você identificar em quais situações deverá utilizar autorreferências e como utilizá-las.