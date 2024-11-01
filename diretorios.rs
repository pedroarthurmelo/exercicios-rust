// Estrutura Arquivo
struct Arquivo<'a> {
    nome: &'a str,
    tamanho: u32,
}

// Implementação da estrutura Arquivo
impl<'a> Arquivo<'a> {
    // Método estático para inicializar um novo arquivo
    fn new(nome: &'a str, tamanho: u32) -> Self {
        Arquivo { nome, tamanho }
    }

    // Método para obter o nome do arquivo
    fn get_nome(&self) -> &'a str {
        self.nome
    }

    // Método para obter o tamanho do arquivo
    fn get_tamanho(&self) -> u32 {
        self.tamanho
    }

    // Método para imprimir as informações do arquivo
    fn imprimir(&self) {
        println!("Arquivo: {}, Tamanho: {} bytes", self.nome, self.tamanho);
    }
}

// Estrutura Diretorio
struct Diretorio<'a> {
    nome: &'a str,
    tamanho: u32,
    arquivos: Vec<Arquivo<'a>>,
}

// Implementação da estrutura Diretorio
impl<'a> Diretorio<'a> {
    // Método estático para inicializar um novo diretório
    fn new(nome: &'a str) -> Self {
        Diretorio {
            nome,
            tamanho: 0,
            arquivos: Vec::new(),
        }
    }

    // Método para adicionar um arquivo ao diretório
    fn adicionar_arquivo(&mut self, arquivo: Arquivo<'a>) {
        self.tamanho += arquivo.get_tamanho();
        self.arquivos.push(arquivo);
    }

    // Método para imprimir as informações do diretório e arquivos
    fn imprimir(&self) {
        println!("Diretório: {}", self.nome);
        for arquivo in &self.arquivos {
            arquivo.imprimir();
        }
        println!("Tamanho total do diretório: {} bytes\n", self.tamanho);
    }

    // Método para filtrar arquivos por tamanho máximo e extensão
    fn filtrar_arquivos(&self, max_tamanho: u32, extensao: &'a str) -> Diretorio<'a> {
        let mut novo_diretorio = Diretorio::new(self.nome);

        for arquivo in &self.arquivos {
            if arquivo.get_tamanho() < max_tamanho && arquivo.get_nome().ends_with(extensao) {
                novo_diretorio.adicionar_arquivo(Arquivo::new(arquivo.get_nome(), arquivo.get_tamanho()));
            }
        }

        novo_diretorio
    }
}

// Função principal
fn main() {
    // Criando o diretório /home e adicionando arquivos
    let mut home = Diretorio::new("/home");
    home.adicionar_arquivo(Arquivo::new("teste.json", 2000));
    home.adicionar_arquivo(Arquivo::new("imagem1.png", 1500));
    home.adicionar_arquivo(Arquivo::new("imagem2.png", 2500));
    home.adicionar_arquivo(Arquivo::new("notas.xlsx", 3000));
    home.adicionar_arquivo(Arquivo::new("data.json", 500));

    // Criando o diretório /etc e adicionando arquivos
    let mut etc = Diretorio::new("/etc");
    etc.adicionar_arquivo(Arquivo::new("passwd", 400));
    etc.adicionar_arquivo(Arquivo::new("resolv.conf", 800));
    etc.adicionar_arquivo(Arquivo::new("syslog.conf", 1200));
    etc.adicionar_arquivo(Arquivo::new("nfs.conf", 1500));
    etc.adicionar_arquivo(Arquivo::new("locate.rc", 300));
    etc.adicionar_arquivo(Arquivo::new("autofs.conf", 2000));

    // Filtrando e mostrando arquivos .png e .xlsx no diretório /home
    let filtro_home_png = home.filtrar_arquivos(5000, ".png");
    println!("Arquivos .png no diretório /home:");
    filtro_home_png.imprimir();

    let filtro_home_xlsx = home.filtrar_arquivos(5000, ".xlsx");
    println!("Arquivos .xlsx no diretório /home:");
    filtro_home_xlsx.imprimir();

    // Filtrando e mostrando arquivos .conf no diretório /etc
    let filtro_etc_conf = etc.filtrar_arquivos(5000, ".conf");
    println!("Arquivos .conf no diretório /etc:");
    filtro_etc_conf.imprimir();

    // Mostrando os diretórios /home e /etc completos
    println!("Diretório completo /home:");
    home.imprimir();

    println!("Diretório completo /etc:");
    etc.imprimir();
}
