CL-Total-RDGA
=============

O **CL-Total-RDGA** é uma implementação de um algoritmo genético para calcular a menor função de dominação romana total em um grafo. Ele utiliza heurísticas, seleção por torneio, cruzamento de dois pontos e validação de soluções para alcançar resultados otimizados.

* * *

1\. Definição: Dominação Romana Total
-------------------------------------

Uma **função de dominação romana** \\(f: V(G) → {0, 1, 2}\\) em um grafo \\(G\\) satisfaz a condição:

*   Cada vértice \\(u\\) para o qual \\(f(u) = 0\\) deve estar conectado a pelo menos um vértice \\(v\\) com \\(f(v) = 2\\).

A **dominação romana total** é uma extensão que exige que o subgrafo induzido pelos vértices com \\(f(v) > 0\\) não contenha vértices isolados.

O objetivo é minimizar o peso total \\( \\sum\_{u ∈ V(G)} f(u) \\).

* * *

2\. Como executar
-----------------

### Pré-requisitos

*   Rust (instalado via [rustup](https://rustup.rs/))
*   Dependências declaradas no `Cargo.toml`

### Compilação

1.  Clone o repositório ou copie os arquivos.
2.  Navegue até o diretório do projeto.
3.  Compile o programa:
    
        cargo build --release
    

### Uso

Para executar o programa, utilize:

    ./target/release/cl_total_rdga <file_path> <trials> [max_stagnant] [generations] [tournament_size] [crossover_prob] [pop_size]

#### Parâmetros

*   `<file_path>`: Caminho para o arquivo com a lista de arestas do grafo.
*   `<trials>`: Número de execuções independentes.
*   `[max_stagnant]` (opcional): Máximo de gerações sem melhoria (padrão: 100).
*   `[generations]` (opcional): Número total de gerações (padrão: 1000).
*   `[tournament_size]` (opcional): Tamanho do torneio na seleção (padrão: 5).
*   `[crossover_prob]` (opcional): Probabilidade de cruzamento (padrão: 0.9).
*   `[pop_size]` (opcional): Tamanho da população inicial (padrão: função baseada no tamanho do grafo).

#### Exemplo

    ./target/release/cl_total_rdga graphs/example.txt 30 200 1500 7 0.8 50

* * *

3\. Saída
---------

A saída padrão do programa inclui:

*   **graph\_name**: Nome do arquivo do grafo.
*   **graph\_order**: Número de vértices.
*   **graph\_size**: Número de arestas.
*   **fitness\_value**: Melhor valor de fitness encontrado.
*   **elapsed\_time**: Tempo total de execução (em microssegundos).

Exemplo de saída:

    graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)
    example.txt,10,15,6,543210

* * *

4\. Gerar documentação
----------------------

Você pode gerar a documentação do código com:

    cargo doc --open

Este comando abrirá a documentação gerada automaticamente pelo Rust em seu navegador padrão, detalhando todos os módulos, funções, estruturas e enums do projeto.

* * *

5\. Testes
----------

Para verificar a integridade do código e assegurar que todas as funcionalidades estão corretas, execute:

    cargo test

Os testes cobrem casos como:

*   Validação de cromossomos.
*   Seleção por torneio.
*   Operações de cruzamento.
*   Geração e validação de populações.

* * *

6\. Contato
-----------

Para dúvidas ou sugestões, entre em contato com:

*   E-mail: [seu\_email@example.com](mailto:hericsilvaho@gmail.com)
*   GitHub: [https://github.com/seu\_usuario](https://github.com/hscHeric)

* * *

