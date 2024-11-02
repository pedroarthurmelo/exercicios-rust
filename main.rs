use std::io::{self, Write};

// Estruturas básicas
#[derive(Clone)]
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

#[derive(Clone)]
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}

#[derive(Clone)]
struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
}

#[derive(Clone)]
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao), // (user, group, other)
    usuario: Usuario,
    grupo: Grupo,
}

#[derive(Clone)]
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    dono: Usuario,
}

// Implementações
impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    fn octal(&self) -> u8 {
        let mut valor = 0;
        if self.leitura { valor += 4; }
        if self.escrita { valor += 2; }
        if self.execucao { valor += 1; }
        valor
    }
}

impl Arquivo {
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        let permissao_padrao = Permissao::new(false, true, false);
        let grupo = Grupo::new(String::from("default"), gid);
        let usuario = Usuario::new(String::from("default"), uid, grupo.clone());
        
        Arquivo {
            nome,
            tamanho,
            permissoes: (permissao_padrao.clone(), permissao_padrao.clone(), permissao_padrao),
            usuario,
            grupo,
        }
    }

    fn alterar_permissao(&mut self, permissao: Permissao) {
        self.permissoes.0 = permissao;
    }

    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("Permissões: {}{}{}", 
            self.permissoes.0.octal(),
            self.permissoes.1.octal(),
            self.permissoes.2.octal()
        );
        println!("Usuário: {} ({})", self.usuario.nome, self.usuario.uid);
        println!("Grupo: {} ({})", self.grupo.nome, self.grupo.gid);
    }
}

impl Diretorio {
    fn new(nome: String, permissao: (Permissao, Permissao, Permissao), dono: Usuario) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes: permissao,
            dono,
        }
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome: String) {
        self.arquivos.retain(|a| a.nome != nome);
    }

    fn listar_conteudo(&self) {
        println!("Conteúdo do diretório {}:", self.nome);
        for arquivo in &self.arquivos {
            println!("- {}", arquivo.nome);
        }
    }
}

impl Usuario {
    fn new(nome: String, uid: u16, grupo: Grupo) -> Usuario {
        Usuario {
            nome,
            uid,
            grupo,
        }
    }

    fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupo = grupo;
    }

    fn remove_grupo(&mut self, _grupo: Grupo) {
        // Implementação simplificada - apenas remove o grupo atual
        self.grupo = Grupo::new(String::from("default"), 0);
    }

    fn listar_grupos(&self) {
        println!("Grupo do usuário {}: {}", self.nome, self.grupo.nome);
    }
}

impl Grupo {
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros: Vec::new(),
        }
    }

    fn adiciona_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }

    fn remove_membro(&mut self, usuario: Usuario) {
        self.membros.retain(|u| u.nome != usuario.nome);
    }

    fn listar_membros(&self) {
        println!("Membros do grupo {}:", self.nome);
        for membro in &self.membros {
            println!("- {}", membro.nome);
        }
    }
}

// Funções auxiliares para o menu
fn ler_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn menu_principal() {
    let mut diretorio = Diretorio::new(
        String::from("root"),
        (
            Permissao::new(true, true, true),
            Permissao::new(true, false, false),
            Permissao::new(true, false, false)
        ),
        Usuario::new(
            String::from("admin"),
            0,
            Grupo::new(String::from("admin"), 0)
        )
    );

    loop {
        println!("\n=== Menu Principal ===");
        println!("1. Gerenciar Arquivos");
        println!("2. Gerenciar Usuários");
        println!("3. Gerenciar Grupos");
        println!("4. Listar conteúdo do diretório");
        println!("0. Sair");

        match ler_input("Escolha uma opção: ").parse::<u32>() {
            Ok(1) => menu_arquivos(&mut diretorio),
            Ok(2) => menu_usuarios(),
            Ok(3) => menu_grupos(),
            Ok(4) => diretorio.listar_conteudo(),
            Ok(0) => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn menu_arquivos(diretorio: &mut Diretorio) {
    loop {
        println!("\n=== Menu Arquivos ===");
        println!("1. Criar arquivo");
        println!("2. Remover arquivo");
        println!("3. Listar arquivos");
        println!("4. Ver detalhes de arquivo (stat)");
        println!("0. Voltar");

        match ler_input("Escolha uma opção: ").parse::<u32>() {
            Ok(1) => {
                let nome = ler_input("Nome do arquivo: ");
                let tamanho = ler_input("Tamanho (bytes): ").parse::<u64>().unwrap_or(0);
                let arquivo = Arquivo::new(nome, tamanho, 1000, 1000);
                diretorio.adiciona_arquivo(arquivo);
                println!("Arquivo criado com sucesso!");
            },
            Ok(2) => {
                let nome = ler_input("Nome do arquivo para remover: ");
                diretorio.remove_arquivo(nome);
                println!("Arquivo removido com sucesso!");
            },
            Ok(3) => diretorio.listar_conteudo(),
            Ok(4) => {
                let nome = ler_input("Nome do arquivo: ");
                if let Some(arquivo) = diretorio.arquivos.iter().find(|a| a.nome == nome) {
                    arquivo.stat();
                } else {
                    println!("Arquivo não encontrado!");
                }
            },
            Ok(0) => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn menu_usuarios() {
    let mut usuarios: Vec<Usuario> = Vec::new();

    loop {
        println!("\n=== Menu Usuários ===");
        println!("1. Criar usuário");
        println!("2. Listar usuários");
        println!("0. Voltar");

        match ler_input("Escolha uma opção: ").parse::<u32>() {
            Ok(1) => {
                let nome = ler_input("Nome do usuário: ");
                let uid = ler_input("UID: ").parse::<u16>().unwrap_or(1000);
                let grupo = Grupo::new(String::from("users"), 1000);
                usuarios.push(Usuario::new(nome, uid, grupo));
                println!("Usuário criado com sucesso!");
            },
            Ok(2) => {
                println!("Usuários cadastrados:");
                for usuario in &usuarios {
                    println!("- {} (UID: {})", usuario.nome, usuario.uid);
                }
            },
            Ok(0) => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn menu_grupos() {
    let mut grupos: Vec<Grupo> = Vec::new();

    loop {
        println!("\n=== Menu Grupos ===");
        println!("1. Criar grupo");
        println!("2. Listar grupos");
        println!("0. Voltar");

        match ler_input("Escolha uma opção: ").parse::<u32>() {
            Ok(1) => {
                let nome = ler_input("Nome do grupo: ");
                let gid = ler_input("GID: ").parse::<u16>().unwrap_or(1000);
                grupos.push(Grupo::new(nome, gid));
                println!("Grupo criado com sucesso!");
            },
            Ok(2) => {
                println!("Grupos cadastrados:");
                for grupo in &grupos {
                    println!("- {} (GID: {})", grupo.nome, grupo.gid);
                    grupo.listar_membros();
                }
            },
            Ok(0) => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn main() {
    println!("Sistema de Arquivos em Rust");
    menu_principal();
}