/////////////////////Arquivo//////////////////////////
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (bool, bool, bool),
    usuario: Usuario,
    grupo: Grupo,
}
impl Arquivo{
    fn new(nome: String, tamanho: u64, permissoes: (bool, bool, bool), usuario: Usuario, grupo: Grupo) -> Arquivo{
        Arquivo{
            nome,
            tamanho,
            permissoes: (false, true, false),
            usuario,
            grupo,
        }
    }
    fn stat(&self){
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        println!("Permissões: ({:o}/{})", self.permissoes.octal(), self.permissoes.to_string());
        println!("Uid: {}", 1000);
        println!("Gid: {}", 1000);
    }

}
/////////////////////Permissao//////////////////////////
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}
impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }
    fn octal(&self) -> u8 {
        let mut octal_value = 0;
    
        if self.leitura {
            octal_value += 4; // leitura é o bit de valor 4
        }
        if self.escrita {
            octal_value += 2; // escrita é o bit de valor 2
        }
        if self.execucao {
            octal_value += 1; // execução é o bit de valor 1
        }
    
        octal_value
    }
} 
/////////////////////Diretório//////////////////////////
struct Diretorio{
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    dono: Usuario,
}
impl Diretorio {

}
/////////////////////Usuario//////////////////////////
struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
}
impl Usuario{

}
/////////////////////Grupo//////////////////////////
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>
}

impl Grupo {

}










fn main(){

}