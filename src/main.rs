use std::time::Instant;

/// Busca sequencial simples - percorre todo o vetor
fn busca_sequencial_simples(
    vetor: &[String],
    alvo: &str,
) -> (Vec<usize>, usize) {
    let mut operacoes = 0;
    let mut posicoes = Vec::new();

    for i in 0..vetor.len() {
        operacoes += 1;

        if vetor[i] == alvo {
            posicoes.push(i);
        }
    }

    (posicoes, operacoes)
}

/// Busca sequencial com interrupção antecipada
fn busca_sequencial_interrompida(
    vetor: &[String],
    alvo: &str,
) -> (Vec<usize>, usize) {
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;

        if vetor[i] == alvo {
            return (vec![i], operacoes);
        }
    }

    (Vec::new(), operacoes)
}

/// Gera um vetor com valores de 1 até n
fn gerar_vetor(tamanho: usize) -> Vec<String> {
    (1..=tamanho)
        .map(|n| n.to_string())
        .collect()
}

/// Executa experimento comparativo entre os dois algoritmos
fn executar_experimento(tamanho: usize, alvo: &str) {
    let vetor = gerar_vetor(tamanho);

    println!("\n{}", "=".repeat(60));
    println!("Tamanho do vetor: {}", tamanho);
    println!("Elemento procurado: {}", alvo);
    println!("{}", "-".repeat(60));

    // Busca Sequencial Simples
    let inicio = Instant::now();
    let (posicoes1, ops1) =
        busca_sequencial_simples(&vetor, alvo);
    let tempo1 = inicio.elapsed();

    println!("\n📌 BUSCA SEQUENCIAL SIMPLES:");
    println!(" Ocorrências: {}", posicoes1.len());
    println!(" Posições encontradas: {:?}", posicoes1);
    println!(" Iterações: {}", ops1);
    println!(" Tempo: {:?}", tempo1);

    // Busca com Interrupção
    let inicio = Instant::now();
    let (posicoes2, ops2) =
        busca_sequencial_interrompida(&vetor, alvo);
    let tempo2 = inicio.elapsed();

    println!("\n📌 BUSCA SEQUENCIAL COM INTERRUPÇÃO:");
    println!(" Ocorrências encontradas: {}", posicoes2.len());
    println!(" Primeira posição: {:?}", posicoes2);
    println!(" Iterações: {}", ops2);
    println!(" Tempo: {:?}", tempo2);

    // Análise Comparativa
    println!("\n📊 ANÁLISE COMPARATIVA:");
    println!(
        " Diferença de operações: {}",
        ops1.saturating_sub(ops2)
    );

    if tempo1 > tempo2 {
        println!(" Algoritmo com interrupção foi mais rápido");
    } else if tempo2 > tempo1 {
        println!(" Algoritmo simples foi mais rápido");
    } else {
        println!(" Tempos praticamente iguais");
    }

    println!("{}", "=".repeat(60));
}

fn main() {
    println!("\n🔬 EXPERIMENTO: COMPARAÇÃO DE ALGORITMOS DE BUSCA\n");

    // Cenário 1: Elemento no início do vetor
    println!( "\n🎯 CENÁRIO 1: Elemento no início (melhor caso para interrupção)" );
    executar_experimento(1_000, "5");
    executar_experimento(100_000, "5");
    executar_experimento(1_000_000, "5");

    // Cenário 2: Elemento no meio do vetor
    println!("\n\n🎯 CENÁRIO 2: Elemento no meio do vetor");
    executar_experimento(1_000, "500");
    executar_experimento(100_000, "50000");
    executar_experimento(1_000_000, "500000");

    // Cenário 3: Elemento no final do vetor (pior caso)
    println!("\n\n🎯 CENÁRIO 3: Elemento no final (pior caso)");
    executar_experimento(1_000, "1000");
    executar_experimento(100_000, "100000");
    executar_experimento(1_000_000, "1000000");

    // Cenário 4: Elemento não existe
    println!("\n\n🎯 CENÁRIO 4: Elemento não existe no vetor");
    executar_experimento(1_000, "-1");
    executar_experimento(100_000, "-1");
    executar_experimento(1_000_000, "-1");

    println!("\n✅ Experimento concluído!");
}